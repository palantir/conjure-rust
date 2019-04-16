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
#![warn(clippy::all)]

extern crate self as conjure_error;

use conjure_object::Value;
use serde::Serialize;

use crate::ser::ParametersSerializer;

mod ser;
#[allow(clippy::all)]
mod types;

pub use crate::types::*;

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

    /// Returns a sorted slice of the names of the error's safe parameters.
    fn safe_args(&self) -> &'static [&'static str];
}

/// Encodes a Conjure error into its serialized form.
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
        if let Some(value) = value_string(&value) {
            builder.insert_parameters(key, value);
        }
    }

    builder
        .error_code(error.code())
        .error_name(error.name())
        .build()
}

fn value_string(value: &Value) -> Option<String> {
    match value {
        Value::Bool(v) => Some(v.to_string()),
        Value::U8(v) => Some(v.to_string()),
        Value::U16(v) => Some(v.to_string()),
        Value::U32(v) => Some(v.to_string()),
        Value::U64(v) => Some(v.to_string()),
        Value::I8(v) => Some(v.to_string()),
        Value::I16(v) => Some(v.to_string()),
        Value::I32(v) => Some(v.to_string()),
        Value::I64(v) => Some(v.to_string()),
        Value::F32(v) => Some(v.to_string()),
        Value::F64(v) => Some(v.to_string()),
        Value::Char(v) => Some(v.to_string()),
        Value::String(v) => Some(v.to_string()),
        Value::Unit => None,
        Value::Option(Some(v)) => value_string(v),
        Value::Option(None) => None,
        Value::Newtype(v) => value_string(v),
        Value::Seq(_) | Value::Map(_) | Value::Bytes(_) => None,
    }
}
