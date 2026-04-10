// Copyright 2026 Palantir Technologies, Inc.
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
use crate::de::null_collections_behavior::NullCollectionsBehavior;
use crate::de::Behavior;
use serde::de::{self, Visitor};
use serde::Deserializer;
use serde_cbor_2::de::{IoRead, SliceRead};
use serde_cbor_2::Error;
use std::io;

/// Deserializes a value from a reader of CBOR data.
pub fn client_from_reader<R, T>(reader: R) -> Result<T, Error>
where
    R: io::Read,
    T: de::DeserializeOwned,
{
    let mut de = ClientDeserializer::from_reader(reader);
    T::deserialize(&mut de)
}

/// Deserializes a value from a slice of CBOR data.
pub fn client_from_slice<'a, T>(s: &'a [u8]) -> Result<T, Error>
where
    T: de::Deserialize<'a>,
{
    let mut de = ClientDeserializer::from_slice(s);
    T::deserialize(&mut de)
}

/// A serde CBOR deserializer appropriate for use by Conjure clients.
pub struct ClientDeserializer<R>(serde_cbor_2::Deserializer<R>);

impl<R> ClientDeserializer<IoRead<R>>
where
    R: io::Read,
{
    /// Creates a Conjure CBOR client deserializer from an `io::Read`.
    pub fn from_reader(reader: R) -> Self {
        ClientDeserializer(serde_cbor_2::Deserializer::from_reader(reader))
    }
}

impl<'a> ClientDeserializer<SliceRead<'a>> {
    /// Creates a Conjure CBOR client deserializer from a `&[u8]`.
    pub fn from_slice(bytes: &'a [u8]) -> Self {
        ClientDeserializer(serde_cbor_2::Deserializer::from_slice(bytes))
    }
}

impl<'a, 'de, R> de::Deserializer<'de> for &'a mut ClientDeserializer<R>
where
    R: serde_cbor_2::de::Read<'de>,
{
    impl_deserialize_body!(
        &'a mut serde_cbor_2::Deserializer<R>,
        NullCollectionsBehavior<ValueBehavior>
    );

    fn is_human_readable(&self) -> bool {
        false
    }
}

pub enum ValueBehavior {}

impl Behavior for ValueBehavior {
    type KeyBehavior = KeyBehavior;

    fn deserialize_byte_buf<'de, D, V>(de: D, visitor: V) -> Result<V::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
        V: de::Visitor<'de>,
    {
        // CBOR byte strings (major type 2) are the natural encoding for byte buffers
        de.deserialize_byte_buf(visitor)
    }
}

pub enum KeyBehavior {}

impl Behavior for KeyBehavior {
    type KeyBehavior = Self;

    fn deserialize_bytes<'de, D, V>(de: D, visitor: V) -> Result<V::Value, D::Error>
    where
        D: Deserializer<'de>,
        V: Visitor<'de>,
    {
        de.deserialize_any(DelegatingVisitor::new(UuidKeyVisitor, visitor))
    }
}

/// Visitor that accepts UUID keys as either strings (Java format) or bytes (Rust format)
struct UuidKeyVisitor;

impl<'de, V> Visitor2<'de, V> for UuidKeyVisitor
where
    V: Visitor<'de>,
{
    // Forward byte arrays as-is (original Rust binary format)
    fn visit_bytes<E>(self, visitor: V, v: &[u8]) -> Result<V::Value, E>
    where
        E: de::Error,
    {
        visitor.visit_bytes(v)
    }

    fn visit_borrowed_bytes<E>(self, visitor: V, v: &'de [u8]) -> Result<V::Value, E>
    where
        E: de::Error,
    {
        visitor.visit_borrowed_bytes(v)
    }

    fn visit_byte_buf<E>(self, visitor: V, v: Vec<u8>) -> Result<V::Value, E>
    where
        E: de::Error,
    {
        visitor.visit_byte_buf(v)
    }

    // Parse string format UUIDs (Java format) and convert to bytes
    fn visit_str<E>(self, visitor: V, v: &str) -> Result<V::Value, E>
    where
        E: de::Error,
    {
        match conjure_object::Uuid::parse_str(v) {
            Ok(uuid) => visitor.visit_byte_buf(uuid.as_bytes().to_vec()),
            Err(_) => Err(E::custom(format!("invalid UUID string: {}", v))),
        }
    }

    fn visit_borrowed_str<E>(self, visitor: V, v: &'de str) -> Result<V::Value, E>
    where
        E: de::Error,
    {
        match conjure_object::Uuid::parse_str(v) {
            Ok(uuid) => visitor.visit_byte_buf(uuid.as_bytes().to_vec()),
            Err(_) => Err(E::custom(format!("invalid UUID string: {}", v))),
        }
    }

    fn visit_string<E>(self, visitor: V, v: String) -> Result<V::Value, E>
    where
        E: de::Error,
    {
        match conjure_object::Uuid::parse_str(&v) {
            Ok(uuid) => visitor.visit_byte_buf(uuid.as_bytes().to_vec()),
            Err(_) => Err(E::custom(format!("invalid UUID string: {}", v))),
        }
    }
}
