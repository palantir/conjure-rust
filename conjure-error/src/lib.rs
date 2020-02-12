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
//! Conjure errors are represented by a struct implementing the `ErrorType` trait. The struct's fields are the error's
//! parameters, and the trait implementation stores the remainder of the error's information.
#![doc(html_root_url = "https://docs.rs/conjure-error/0.6")]
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

impl ErrorType for SerializableError {
    #[inline]
    fn code(&self) -> ErrorCode {
        self.error_code().clone()
    }

    #[inline]
    fn name(&self) -> &str {
        self.error_name()
    }

    #[inline]
    fn instance_id(&self) -> Option<Uuid> {
        Some(self.error_instance_id())
    }

    #[inline]
    fn safe_args(&self) -> &'static [&'static str] {
        &[]
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
    let mut builder = SerializableError::builder();

    let parameters = error
        .serialize(ParametersSerializer)
        .expect("failed to serialize error parameters");

    for (key, value) in parameters {
        if let Ok(value) = StringSeed.deserialize(value) {
            builder.insert_parameters(key, value);
        }
    }

    builder
        .error_code(error.code())
        .error_name(error.name())
        .error_instance_id(error.instance_id().unwrap_or_else(Uuid::new_v4))
        .build()
}
