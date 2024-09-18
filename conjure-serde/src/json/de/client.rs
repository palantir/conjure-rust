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
use crate::de::delegating_visitor::{DelegatingVisitor, Visitor2};
use crate::de::Behavior;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use serde::de::{self, Visitor};
use serde_json::de::{IoRead, Read, SliceRead, StrRead};
use serde_json::Error;
use std::fmt;
use std::io;

/// Deserializes a value from a reader of JSON data.
pub fn client_from_reader<R, T>(reader: R) -> Result<T, Error>
where
    R: io::Read,
    T: de::DeserializeOwned,
{
    let mut de = ClientDeserializer::from_reader(reader);
    let value = T::deserialize(&mut de)?;
    de.end()?;
    Ok(value)
}

/// Deserializes a value from a string of JSON data.
pub fn client_from_str<'a, T>(s: &'a str) -> Result<T, Error>
where
    T: de::Deserialize<'a>,
{
    let mut de = ClientDeserializer::from_str(s);
    let value = T::deserialize(&mut de)?;
    de.end()?;
    Ok(value)
}

/// Deserializes a value from a slice of JSON data.
pub fn client_from_slice<'a, T>(s: &'a [u8]) -> Result<T, Error>
where
    T: de::Deserialize<'a>,
{
    let mut de = ClientDeserializer::from_slice(s);
    let value = T::deserialize(&mut de)?;
    de.end()?;
    Ok(value)
}

/// A serde JSON deserializer appropriate for use by Conjure clients.
pub struct ClientDeserializer<R>(serde_json::Deserializer<R>);

impl<R> ClientDeserializer<IoRead<R>>
where
    R: io::Read,
{
    /// Creates a Conjure JSON client deserializer from an `io::Read`.
    pub fn from_reader(reader: R) -> ClientDeserializer<IoRead<R>> {
        ClientDeserializer(serde_json::Deserializer::from_reader(reader))
    }
}

impl<'a> ClientDeserializer<SliceRead<'a>> {
    /// Creates a Conjure JSON client deserializer from a `&[u8]`.
    pub fn from_slice(bytes: &'a [u8]) -> ClientDeserializer<SliceRead<'a>> {
        ClientDeserializer(serde_json::Deserializer::from_slice(bytes))
    }
}

impl<'a> ClientDeserializer<StrRead<'a>> {
    /// Creates a Conjure JSON client deserializer from a `&str`.
    #[allow(clippy::should_implement_trait)] // match serde_json's API
    pub fn from_str(s: &'a str) -> ClientDeserializer<StrRead<'a>> {
        ClientDeserializer(serde_json::Deserializer::from_str(s))
    }
}

impl<'de, R> ClientDeserializer<R>
where
    R: Read<'de>,
{
    /// Validates that the input stream is at the end or that it only has trailing whitespace.
    pub fn end(&mut self) -> Result<(), Error> {
        self.0.end()
    }
}

impl<'a, 'de, R> de::Deserializer<'de> for &'a mut ClientDeserializer<R>
where
    R: Read<'de>,
{
    impl_deserialize_body!(&'a mut serde_json::Deserializer<R>, ValueBehavior);

    // we can't delegate this due to the signature, but luckily we know the answer
    fn is_human_readable(&self) -> bool {
        true
    }
}

pub enum ValueBehavior {}

impl Behavior for ValueBehavior {
    type KeyBehavior = KeyBehavior;

    fn deserialize_f32<'de, D, V>(de: D, visitor: V) -> Result<V::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
        V: de::Visitor<'de>,
    {
        de.deserialize_any(DelegatingVisitor::new(F32Visitor, visitor))
    }

    fn deserialize_f64<'de, D, V>(de: D, visitor: V) -> Result<V::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
        V: de::Visitor<'de>,
    {
        de.deserialize_any(DelegatingVisitor::new(F64Visitor, visitor))
    }

    fn deserialize_bytes<'de, D, V>(de: D, visitor: V) -> Result<V::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
        V: de::Visitor<'de>,
    {
        de.deserialize_str(ByteBufVisitor(visitor))
    }

    fn deserialize_byte_buf<'de, D, V>(de: D, visitor: V) -> Result<V::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
        V: de::Visitor<'de>,
    {
        de.deserialize_str(ByteBufVisitor(visitor))
    }
}

pub enum KeyBehavior {}

impl Behavior for KeyBehavior {
    type KeyBehavior = Self;

    fn deserialize_bool<'de, D, V>(de: D, visitor: V) -> Result<V::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
        V: Visitor<'de>,
    {
        de.deserialize_str(BoolKeyVisitor(visitor))
    }

    fn deserialize_f32<'de, D, V>(de: D, visitor: V) -> Result<V::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
        V: de::Visitor<'de>,
    {
        de.deserialize_str(F32KeyVisitor(visitor))
    }

    fn deserialize_f64<'de, D, V>(de: D, visitor: V) -> Result<V::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
        V: de::Visitor<'de>,
    {
        de.deserialize_str(F64KeyVisitor(visitor))
    }

    fn deserialize_bytes<'de, D, V>(de: D, visitor: V) -> Result<V::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
        V: de::Visitor<'de>,
    {
        de.deserialize_str(ByteBufVisitor(visitor))
    }

    fn deserialize_byte_buf<'de, D, V>(de: D, visitor: V) -> Result<V::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
        V: de::Visitor<'de>,
    {
        de.deserialize_str(ByteBufVisitor(visitor))
    }

    // We don't need this for JSON, but it allows the Smile logic to work with this KeyBehavior
    fn is_human_readable<'de, D>(_: &D) -> bool
    where
        D: de::Deserializer<'de>,
    {
        true
    }
}

macro_rules! float_visitor {
    ($name:ident, $method:ident, $module:ident) => {
        struct $name;

        impl<'de, V> Visitor2<'de, V> for $name
        where
            V: Visitor<'de>,
        {
            fn visit_str<E>(self, visitor: V, v: &str) -> Result<V::Value, E>
            where
                E: de::Error,
            {
                match v {
                    "NaN" => visitor.$method($module::NAN),
                    "Infinity" => visitor.$method($module::INFINITY),
                    "-Infinity" => visitor.$method($module::NEG_INFINITY),
                    _ => visitor.visit_str(v),
                }
            }

            fn visit_borrowed_str<E>(self, visitor: V, v: &'de str) -> Result<V::Value, E>
            where
                E: de::Error,
            {
                self.visit_str(visitor, v)
            }

            fn visit_string<E>(self, visitor: V, v: String) -> Result<V::Value, E>
            where
                E: de::Error,
            {
                self.visit_str(visitor, &v)
            }
        }
    };
}

float_visitor!(F32Visitor, visit_f32, f32);
float_visitor!(F64Visitor, visit_f64, f64);

// NB: not using DelegatingVisitor for these since we specifically expect a string
macro_rules! float_key_visitor {
    ($name:ident, $method:ident, $module:ident) => {
        struct $name<T>(T);

        impl<'de, T> de::Visitor<'de> for $name<T>
        where
            T: de::Visitor<'de>,
        {
            type Value = T::Value;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a number")
            }

            fn visit_str<E>(self, v: &str) -> Result<T::Value, E>
            where
                E: de::Error,
            {
                match v {
                    "NaN" => (self.0).$method($module::NAN),
                    "Infinity" => (self.0).$method($module::INFINITY),
                    "-Infinity" => (self.0).$method($module::NEG_INFINITY),
                    v => match v.parse() {
                        Ok(v) => (self.0).$method(v),
                        Err(_) => Err(de::Error::invalid_value(de::Unexpected::Str(v), &self)),
                    },
                }
            }
        }
    };
}

float_key_visitor!(F32KeyVisitor, visit_f32, f32);
float_key_visitor!(F64KeyVisitor, visit_f64, f64);

struct ByteBufVisitor<T>(T);

impl<'de, T> de::Visitor<'de> for ByteBufVisitor<T>
where
    T: de::Visitor<'de>,
{
    type Value = T::Value;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a base64 string")
    }

    fn visit_str<E>(self, v: &str) -> Result<T::Value, E>
    where
        E: de::Error,
    {
        match STANDARD.decode(v) {
            Ok(v) => self.0.visit_byte_buf(v),
            Err(_) => Err(E::invalid_value(de::Unexpected::Str(v), &self)),
        }
    }
}

struct BoolKeyVisitor<T>(T);

impl<'de, T> Visitor<'de> for BoolKeyVisitor<T>
where
    T: Visitor<'de>,
{
    type Value = T::Value;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a boolean")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match v.parse() {
            Ok(v) => self.0.visit_bool(v),
            Err(_) => Err(E::invalid_value(de::Unexpected::Str(v), &self)),
        }
    }
}
