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
#![warn(clippy::all)]

extern crate self as conjure_error;

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

pub trait ErrorType {
    fn code(&self) -> ErrorCode;

    fn name(&self) -> &str;

    fn safe_arg(&self, name: &str) -> bool;
}

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
        .parameters(parameters)
        .build()
}
