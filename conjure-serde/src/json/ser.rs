// Copyright 2018 Palantir Technologies, Inc.
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
use crate::ser::Behavior;
use base64::display::Base64Display;
use base64::engine::general_purpose::STANDARD;
use serde::ser;
use serde_json::ser::{CompactFormatter, Formatter, PrettyFormatter};
use serde_json::Error;
use std::f32;
use std::f64;
use std::io::Write;

/// Serializes a value as JSON into a byte buffer.
pub fn to_vec<T>(value: &T) -> Result<Vec<u8>, Error>
where
    T: ?Sized + ser::Serialize,
{
    let mut buf = Vec::with_capacity(128);
    value.serialize(&mut Serializer::new(&mut buf))?;
    Ok(buf)
}

/// Serializes a value as JSON into a string.
pub fn to_string<T>(value: &T) -> Result<String, Error>
where
    T: ?Sized + ser::Serialize,
{
    let vec = to_vec(value)?;
    // JSON is always valid UTF8
    unsafe { Ok(String::from_utf8_unchecked(vec)) }
}

/// Serializes a value as JSON into a writer.
pub fn to_writer<W, T>(writer: W, value: &T) -> Result<(), Error>
where
    W: Write,
    T: ?Sized + ser::Serialize,
{
    value.serialize(&mut Serializer::new(writer))
}

/// A serde JSON serializer compatible with the Conjure specification.
pub struct Serializer<W, F = CompactFormatter>(serde_json::Serializer<W, F>);

impl<W> Serializer<W>
where
    W: Write,
{
    /// Creates a new Conjure JSON serializer.
    pub fn new(writer: W) -> Serializer<W> {
        Serializer(serde_json::Serializer::new(writer))
    }
}

impl<'a, W> Serializer<W, PrettyFormatter<'a>>
where
    W: Write,
{
    /// Creates a new Conjure pretty JSON serializer.
    pub fn pretty(writer: W) -> Serializer<W, PrettyFormatter<'a>> {
        Serializer(serde_json::Serializer::pretty(writer))
    }
}

impl<W, F> Serializer<W, F>
where
    W: Write,
    F: Formatter,
{
    /// Creates a new Conjure JSON serializer with a custom formatter.
    pub fn with_formatter(writer: W, formatter: F) -> Serializer<W, F> {
        Serializer(serde_json::Serializer::with_formatter(writer, formatter))
    }

    /// Returns the inner writer.
    pub fn into_inner(self) -> W {
        self.0.into_inner()
    }
}

impl<'a, W, F> ser::Serializer for &'a mut Serializer<W, F>
where
    W: Write,
    F: Formatter,
{
    impl_serialize_body!(&'a mut serde_json::Serializer<W, F>, ValueBehavior);

    // we can't delegate this due to the signature, but luckily we know the answer
    fn is_human_readable(&self) -> bool {
        true
    }
}

pub enum ValueBehavior {}

impl Behavior for ValueBehavior {
    type KeyBehavior = KeyBehavior;

    fn serialize_f32<S>(ser: S, v: f32) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if v.is_nan() {
            ser.serialize_str("NaN")
        } else if v == f32::INFINITY {
            ser.serialize_str("Infinity")
        } else if v == f32::NEG_INFINITY {
            ser.serialize_str("-Infinity")
        } else {
            ser.serialize_f32(v)
        }
    }

    fn serialize_f64<S>(ser: S, v: f64) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if v.is_nan() {
            ser.serialize_str("NaN")
        } else if v == f64::INFINITY {
            ser.serialize_str("Infinity")
        } else if v == f64::NEG_INFINITY {
            ser.serialize_str("-Infinity")
        } else {
            ser.serialize_f64(v)
        }
    }

    fn serialize_bytes<S>(ser: S, v: &[u8]) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        ser.collect_str(&Base64Display::new(v, &STANDARD))
    }
}

pub enum KeyBehavior {}

impl Behavior for KeyBehavior {
    type KeyBehavior = Self;

    fn serialize_bool<S>(ser: S, v: bool) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if v {
            ser.serialize_str("true")
        } else {
            ser.serialize_str("false")
        }
    }

    fn serialize_f32<S>(ser: S, v: f32) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if v.is_nan() {
            ser.serialize_str("NaN")
        } else if v == f32::INFINITY {
            ser.serialize_str("Infinity")
        } else if v == f32::NEG_INFINITY {
            ser.serialize_str("-Infinity")
        } else {
            ser.collect_str(&v)
        }
    }

    fn serialize_f64<S>(ser: S, v: f64) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if v.is_nan() {
            ser.serialize_str("NaN")
        } else if v == f64::INFINITY {
            ser.serialize_str("Infinity")
        } else if v == f64::NEG_INFINITY {
            ser.serialize_str("-Infinity")
        } else {
            ser.collect_str(&v)
        }
    }

    fn serialize_bytes<S>(ser: S, v: &[u8]) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        ser.collect_str(&Base64Display::new(v, &STANDARD))
    }
}
