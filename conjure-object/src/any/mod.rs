// Copyright 2020 Palantir Technologies, Inc.
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
//! The Conjure `any` type.

use crate::any::ser::AnySerializer;
use ordered_float::OrderedFloat;
use serde::de::{DeserializeOwned, Unexpected};
use serde::Serialize;
use std::collections::BTreeMap;
use std::error;
use std::f64;
use std::fmt;

mod de;
mod ser;

/// An error serializing to or from an `Any` value.
#[derive(Debug)]
pub struct Error(String);

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(&self.0)
    }
}

impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display,
    {
        Error(msg.to_string())
    }
}

impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display,
    {
        Error(msg.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Inner {
    Null,
    Bool(bool),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    F32(OrderedFloat<f32>),
    F64(OrderedFloat<f64>),
    Char(char),
    String(String),
    Bytes(Vec<u8>),
    Seq(Vec<Any>),
    Map(BTreeMap<Any, Any>),
}

/// A representation of an arbitrary serializable value, corresponding to the Conjure `any` type.
///
/// The type is designed to be a lossless representation of a Conjure JSON value and follows Conjure's specifications
/// regarding various edge cases such as base64 encoded binary values and non-finite floats. Its internal structure is
/// opaque. Values can be converted to and from it with the `Any::new` and `Any::deserialize_into` methods, and it can
/// be deserialized to and from JSON via its `Serialize` and `Deserialize` implementations.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Any(Inner);

impl Any {
    /// Converts a value into an `Any`.
    ///
    /// # Errors
    ///
    /// Returns an error if the value's serialize implementation returns an error or it does not serialize to a
    /// JSON-compatible representation.
    pub fn new<T>(value: T) -> Result<Any, Error>
    where
        T: Serialize,
    {
        value.serialize(AnySerializer)
    }

    /// Converts the `Any` into a typed value.
    ///
    /// This is simply a convenience function using `Any`'s `Deserializer` implementation.
    ///
    /// # Errors
    ///
    /// Returns an error if the type cannot be deserialized from this `Any`.
    pub fn deserialize_into<T>(self) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        T::deserialize(self)
    }

    fn unexpected(&self) -> Unexpected<'_> {
        match &self.0 {
            Inner::Null => Unexpected::Unit,
            Inner::Bool(v) => Unexpected::Bool(*v),
            Inner::I8(v) => Unexpected::Signed(i64::from(*v)),
            Inner::I16(v) => Unexpected::Signed(i64::from(*v)),
            Inner::I32(v) => Unexpected::Signed(i64::from(*v)),
            Inner::I64(v) => Unexpected::Signed(*v),
            Inner::I128(v) => Unexpected::Signed(*v as i64),
            Inner::U8(v) => Unexpected::Unsigned(u64::from(*v)),
            Inner::U16(v) => Unexpected::Unsigned(u64::from(*v)),
            Inner::U32(v) => Unexpected::Unsigned(u64::from(*v)),
            Inner::U64(v) => Unexpected::Unsigned(*v),
            Inner::U128(v) => Unexpected::Unsigned(*v as u64),
            Inner::F32(v) => Unexpected::Float(v.0 as f64),
            Inner::F64(v) => Unexpected::Float(v.0),
            Inner::Char(v) => Unexpected::Char(*v),
            Inner::String(v) => Unexpected::Str(v),
            Inner::Bytes(v) => Unexpected::Bytes(v),
            Inner::Seq(_) => Unexpected::Seq,
            Inner::Map(_) => Unexpected::Map,
        }
    }
}
