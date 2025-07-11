// Copyright 2019 Palantir Technologies, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Runtime support for Conjure error types.
//!
//! In a networked service, the error objects that are propagated through its codebase are responsible for two things:
//!
//! * Collecting useful information that a developer can use to diagnose whatever problem caused the error.
//! * Controlling how the error is presented to the client.
//!
//! Services implemented using Conjure's frameworks use the [`Error`] type defined in this crate as the single error
//! type throughout the codebase. [`Error`]s store:
//!
//! * Developer facing:
//!   * [`Error::cause`] - the underlying cause of the error. This can be a type implementing the Rust
//!     [`std::error::Error`] trait, or just a [`str`] or [`String`] containing a description of what happened. When
//!     an [`Error`] is logged, the cause (and its chain of sources via [`std::error::Error::source`]) are included as a
//!     parameter. The log-safety of that cause information is identified by the choice of constructor of the [`Error`].
//!   * [`Error::safe_params`] and [`Error::unsafe_params`] - key-value pairs that can be added to the error to provide
//!     context. When an [`Error`] is logged, these are included in the service log's parameters. When a service
//!     [`Error`] is created, all of the parameters of its associated Conjure error are automatically included as
//!     params, with [`ErrorType::safe_args`] used to partition the parameters between safe and unsafe. Additional
//!     params can be added via [`Error::with_safe_param`] and [`Error::with_unsafe_param`].
//!   * [`Error::backtraces`] - a sequence of backtraces to annotate the error with the state of the function call
//!     stack. A backtrace is automatically taken when the [`Error`] is created, and additional backtraces can be added
//!     with the [`Error::with_backtrace`] method. This can be used when, for example, an [`Error`] transfers from one
//!     thread to another. When an [`Error`] is logged, its backtraces will be included in the stacktrace field.
//! * Client facing:
//!   * [`Error::kind`] - how the error should be reported to the client. There are currently three kinds:
//!       * [`ErrorKind::Service`] - a standard service error. These are constructed from a type implementing
//!         [`Serialize`] and [`ErrorType`]. The value is expected to serialize as a struct, with the struct's fields
//!         being the parameters of the error. Errors defined in Conjure APIs will generate types implementing these
//!         traits. This will generate an HTTP response following the [Conjure wire spec]. Service errors are created
//!         with the [`Error::service`], [`Error::service_safe`], [`Error::internal`], and [`Error::internal_safe`]
//!         functions.
//!       * [`ErrorKind::Throttle`] - an indication that the client is making too many requests and should throttle
//!         itself. This will generate a `429 Too Many Requests` HTTP response. Throttle errors are created with the
//!         [`Error::throttle`], [`Error::throttle_safe`], [`Error::throttle_for`], and [`Error::throttle_for_safe`]
//!         functions.
//!       * [`ErrorKind::Unavailable`] - an indication that the server is unable to handle the request. This will
//!         generate a `503 Service Unavailable` HTTP response. Unavailable errors are created with the
//!         [`Error::unavailable`] and [`Error::unavailable_safe`] functions.
//!
//! [Conjure wire spec]: https://github.com/palantir/conjure/blob/master/docs/spec/wire.md#34-conjure-errors
//!
//! ## Examples
//!
//! Mapping a [`std::error::Error`] returned by a stdlib API into a generic internal service error:
//!
//! ```rust,no_run
//! use conjure_error::Error;
//! use std::fs::File;
//!
//! # fn foo() -> Result<(), Error> {
//! let file = File::open("var/data/database.csv").map_err(Error::internal_safe)?;
//! # Ok(()) }
//! ```
//!
//! Doing the same, but including the filename as an extra parameter:
//!
//! ```rust,no_run
//! use conjure_error::Error;
//! use std::fs::File;
//!
//! # fn foo() -> Result<(), Error> {
//! let filename = "var/data/database.csv";
//! let file = File::open(filename).map_err(|e| {
//!     Error::internal_safe(e).with_safe_param("filename", filename)
//! })?;
//! # Ok(()) }
//! ```
//!
//! Returning a specific Conjure error when there is no existing error cause:
//!
//! ```yaml
//! types:
//!   definitions:
//!     errors:
//!       ObjectNotFound:
//!         namespace: MyService
//!         code: INVALID_ARGUMENT
//!         safe-args:
//!           objectRid: rid
//! ```
//!
//! ```rust,ignore
//! use conjure_error::Error;
//! use my_service_api::errors::ObjectNotFound;
//!
//! if !object_was_found {
//!     return Err(Error::service_safe("failed to find object", ObjectNotFound::new(object_rid)));
//! }
//! ```
#![warn(clippy::all, missing_docs)]

extern crate self as conjure_error;

use conjure_object::Uuid;
use serde::{Serialize, Serializer};

use crate::ser::{ParametersSerializer, StringSeed};

pub use crate::error::*;
pub use crate::types::*;
use serde::de::DeserializeSeed;

mod error;
mod ser;
#[allow(clippy::all, missing_docs)]
#[rustfmt::skip]
mod types;

impl ErrorCode {
    /// Returns the HTTP status code associated with the error code.
    #[inline]
    pub fn status_code(&self) -> u16 {
        match self {
            ErrorCode::PermissionDenied => 403,
            ErrorCode::InvalidArgument => 400,
            ErrorCode::NotFound => 404,
            ErrorCode::Conflict => 409,
            ErrorCode::RequestEntityTooLarge => 413,
            ErrorCode::FailedPrecondition => 500,
            ErrorCode::Internal => 500,
            ErrorCode::Timeout => 500,
            ErrorCode::CustomClient => 400,
            ErrorCode::CustomServer => 500,
        }
    }
}

/// A trait implemented by Conjure error types.
pub trait ErrorType {
    /// Returns the error's code.
    fn code(&self) -> ErrorCode;

    /// Returns the error's name.
    ///
    /// The name must be formatted like `NamespaceName:ErrorName`.
    fn name(&self) -> &str;

    /// Returns the error's instance ID, if it stores one.
    ///
    /// Conjure-generated error types return `None`, but other implementations like those for `SerializableError`
    /// and `WithInstanceId` return a value.
    fn instance_id(&self) -> Option<Uuid>;

    /// Returns a sorted slice of the names of the error's safe parameters.
    fn safe_args(&self) -> &'static [&'static str];

    /// Wraps the error in another that overrides its instance ID.
    #[inline]
    fn with_instance_id(self, instance_id: Uuid) -> WithInstanceId<Self>
    where
        Self: Sized,
    {
        WithInstanceId {
            error: self,
            instance_id,
        }
    }
}

impl<T> ErrorType for &T
where
    T: ?Sized + ErrorType,
{
    #[inline]
    fn code(&self) -> ErrorCode {
        (**self).code()
    }

    #[inline]
    fn name(&self) -> &str {
        (**self).name()
    }

    #[inline]
    fn instance_id(&self) -> Option<Uuid> {
        (**self).instance_id()
    }

    #[inline]
    fn safe_args(&self) -> &'static [&'static str] {
        (**self).safe_args()
    }
}

/// An `ErrorType` which wraps another and overrides its instance ID.
pub struct WithInstanceId<T> {
    error: T,
    instance_id: Uuid,
}

impl<T> ErrorType for WithInstanceId<T>
where
    T: ErrorType,
{
    fn code(&self) -> ErrorCode {
        self.error.code()
    }

    fn name(&self) -> &str {
        self.error.name()
    }

    fn instance_id(&self) -> Option<Uuid> {
        Some(self.instance_id)
    }

    fn safe_args(&self) -> &'static [&'static str] {
        self.error.safe_args()
    }
}

impl<T> Serialize for WithInstanceId<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.error.serialize(s)
    }
}

/// Encodes a Conjure error into its serialized form.
///
/// The error's instance ID will be randomly generated if not provided by the error.
///
/// # Panics
///
/// Panics if the error type does not serialize as a struct.
pub fn encode<T>(error: &T) -> SerializableError
where
    T: ErrorType + Serialize,
{
    let parameters = error
        .serialize(ParametersSerializer)
        .expect("failed to serialize error parameters");

    SerializableError::builder()
        .error_code(error.code())
        .error_name(error.name())
        .error_instance_id(error.instance_id().unwrap_or_else(Uuid::new_v4))
        .parameters(parameters)
        .build()
}

/// Re-serializes the parameters of a [`SerializableError`] in the legacy stringified format.
///
/// Scalar parameters will be converted to their string representations and composite parameters
/// will be dropped.
pub fn stringify_parameters(error: SerializableError) -> SerializableError {
    let mut stringified_parameters = vec![];

    for (key, value) in error.parameters() {
        if let Ok(value) = StringSeed.deserialize(value.clone()) {
            stringified_parameters.push((key.clone(), value));
        }
    }

    serializable_error::Builder::from(error)
        .parameters(stringified_parameters)
        .build()
}
