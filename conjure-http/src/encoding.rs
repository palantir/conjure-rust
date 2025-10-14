// Copyright 2025 Palantir Technologies, Inc.
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
//! Encoding APIs for serializable bodies.

use erased_serde::{Deserializer, Serializer};
use http::HeaderValue;

/// An encoding of HTTP bodies.
pub trait Encoding {
    /// The encoding's MIME type.
    fn content_type(&self) -> HeaderValue;

    /// Returns state which will serialize the response body into the provided buffer.
    fn serializer<'a>(&self, w: &'a mut Vec<u8>) -> Box<dyn SerializerState<'a> + 'a>;

    /// Returns state which will deserialize the request body from the provided buffer.
    fn deserializer<'a>(&self, buf: &'a [u8]) -> Box<dyn DeserializerState<'a> + 'a>;
}

/// An intermediate state between an [`Encoding`] and [`Serializer`].
///
/// This only exists due to the specifics of [`erased_serde`]'s implementation.
pub trait SerializerState<'a> {
    /// Returns the state's internal serializer.
    fn serializer<'b, 'c>(&'b mut self) -> Box<dyn Serializer + 'c>
    where
        'a: 'c,
        'b: 'c;
}

/// An intermediate state between an [`Encoding`] and [`Deserializer`].
///
/// This only exists due to the specifics of [`erased_serde`]'s implementation.
pub trait DeserializerState<'de> {
    /// Returns the state's internal deserializer.
    fn deserializer<'a>(&'a mut self) -> Box<dyn Deserializer<'de> + 'a>;
}
