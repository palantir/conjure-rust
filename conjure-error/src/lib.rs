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
#![warn(clippy::all, missing_docs)]

extern crate self as conjure_error;

use conjure_object::Uuid;
use serde::Serialize;

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
    fn code() -> ErrorCode;

    /// Returns the error's name.
    ///
    /// The name must be formatted like `NamespaceName:ErrorName`.
    fn name() -> &'static str;

    /// Returns a sorted slice of the names of the error's safe parameters.
    fn safe_args() -> &'static [&'static str];
}

impl<T> ErrorType for &T
where
    T: ?Sized + ErrorType,
{
    #[inline]
    fn code() -> ErrorCode {
        T::code()
    }

    #[inline]
    fn name() -> &'static str {
        T::name()
    }

    #[inline]
    fn safe_args() -> &'static [&'static str] {
        T::safe_args()
    }
}

/// Encodes a Conjure error into its serialized form.
///
/// The error's instance ID will be randomly generated.
///
/// # Panics
///
/// Panics if the error type does not serialize as a struct.
pub fn encode<T>(error: &T) -> SerializableError
where
    T: ErrorType + Serialize,
{
    let mut builder = SerializableError::builder()
        .error_code(T::code())
        .error_name(T::name())
        .error_instance_id(Uuid::new_v4());

    let parameters = error
        .serialize(ParametersSerializer)
        .expect("failed to serialize error parameters");

    for (key, value) in parameters {
        if let Ok(value) = StringSeed.deserialize(value) {
            builder = builder.insert_parameters(key, value);
        }
    }

    builder.build()
}
