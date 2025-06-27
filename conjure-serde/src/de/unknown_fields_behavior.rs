// Copyright 2021 Palantir Technologies, Inc.
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
use super::wrapping_deserializer::Delegate;
use crate::de::delegating_deserializer::{DelegatingDeserializer, Deserializer2};
use crate::de::delegating_visitor::{DelegatingVisitor, Visitor2};
use crate::de::wrapping_deserializer::{WrapVisitor, WrappingDeserializer};
use crate::de::Behavior;
use serde::de::{DeserializeSeed, Error, MapAccess, Visitor};
use serde::Deserializer;
use std::borrow::Cow;
use std::marker::PhantomData;

pub struct UnknownFieldsBehavior<B> {
    _p: PhantomData<B>,
}

impl<B> Behavior for UnknownFieldsBehavior<B>
where
    B: Behavior,
{
    type KeyBehavior = UnknownFieldsBehavior<B::KeyBehavior>;

    fn deserialize_bool<'de, D, V>(de: D, visitor: V) -> Result<V::Value, D::Error>
    where
        D: Deserializer<'de>,
        V: Visitor<'de>,
    {
        B::deserialize_bool(de, visitor)
    }

    fn deserialize_f32<'de, D, V>(de: D, visitor: V) -> Result<V::Value, D::Error>
    where
        D: Deserializer<'de>,
        V: Visitor<'de>,
    {
        B::deserialize_f32(de, visitor)
    }

    fn deserialize_f64<'de, D, V>(de: D, visitor: V) -> Result<V::Value, D::Error>
    where
        D: Deserializer<'de>,
        V: Visitor<'de>,
    {
        B::deserialize_f64(de, visitor)
    }

    fn deserialize_bytes<'de, D, V>(de: D, visitor: V) -> Result<V::Value, D::Error>
    where
        D: Deserializer<'de>,
        V: Visitor<'de>,
    {
        B::deserialize_bytes(de, visitor)
    }

    fn deserialize_byte_buf<'de, D, V>(de: D, visitor: V) -> Result<V::Value, D::Error>
    where
        D: Deserializer<'de>,
        V: Visitor<'de>,
    {
        B::deserialize_byte_buf(de, visitor)
    }

    fn deserialize_struct<'de, D, V>(
        de: D,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, D::Error>
    where
        D: Deserializer<'de>,
        V: Visitor<'de>,
    {
        B::deserialize_struct(
            de,
            name,
            fields,
            DelegatingVisitor::new(StructVisitor { fields }, visitor),
        )
    }
}

struct StructVisitor {
    fields: &'static [&'static str],
}

impl<'de, V> Visitor2<'de, V> for StructVisitor
where
    V: Visitor<'de>,
{
    fn visit_map<A>(self, visitor: V, map: A) -> Result<V::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        visitor.visit_map(StructMapAccess {
            map,
            fields: self.fields,
            key: None,
        })
    }
}

struct StructMapAccess<'de, T> {
    map: T,
    fields: &'static [&'static str],
    key: Option<Cow<'de, str>>,
}

impl<'de, T> MapAccess<'de> for StructMapAccess<'de, T>
where
    T: MapAccess<'de>,
{
    type Error = T::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, T::Error>
    where
        K: DeserializeSeed<'de>,
    {
        self.key = None;
        self.map.next_key_seed(KeyDeserializeSeed {
            seed,
            key: &mut self.key,
        })
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, T::Error>
    where
        V: DeserializeSeed<'de>,
    {
        self.map.next_value_seed(ValueDeserializeSeed {
            seed,
            fields: self.fields,
            key: &self.key,
        })
    }

    fn size_hint(&self) -> Option<usize> {
        self.map.size_hint()
    }
}

struct KeyDeserializeSeed<'de, 'a, T> {
    seed: T,
    key: &'a mut Option<Cow<'de, str>>,
}

impl<'de, T> DeserializeSeed<'de> for KeyDeserializeSeed<'de, '_, T>
where
    T: DeserializeSeed<'de>,
{
    type Value = T::Value;

    fn deserialize<D>(self, deserializer: D) -> Result<T::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        self.seed.deserialize(WrappingDeserializer::new(
            KeyWrapper { key: self.key },
            deserializer,
        ))
    }
}

struct KeyWrapper<'de, 'a> {
    key: &'a mut Option<Cow<'de, str>>,
}

impl<'de> WrapVisitor<'de> for KeyWrapper<'de, '_> {
    fn wrap_visitor<D, V>(self, delegate: D, visitor: V) -> Result<V::Value, D::Error>
    where
        D: Delegate<'de>,
        V: Visitor<'de>,
    {
        delegate.delegate(DelegatingVisitor::new(
            KeyVisitor { key: self.key },
            visitor,
        ))
    }
}

struct KeyVisitor<'de, 'a> {
    key: &'a mut Option<Cow<'de, str>>,
}

impl<'de, V> Visitor2<'de, V> for KeyVisitor<'de, '_>
where
    V: Visitor<'de>,
{
    fn visit_str<E>(self, visitor: V, value: &str) -> Result<V::Value, E>
    where
        E: Error,
    {
        *self.key = Some(Cow::Owned(value.to_string()));
        visitor.visit_str(value)
    }

    fn visit_borrowed_str<E>(self, visitor: V, value: &'de str) -> Result<V::Value, E>
    where
        E: Error,
    {
        *self.key = Some(Cow::Borrowed(value));
        visitor.visit_borrowed_str(value)
    }

    fn visit_string<E>(self, visitor: V, value: String) -> Result<V::Value, E>
    where
        E: Error,
    {
        *self.key = Some(Cow::Owned(value.to_string()));
        visitor.visit_string(value)
    }
}

struct ValueDeserializeSeed<'de, 'a, T> {
    seed: T,
    fields: &'static [&'static str],
    key: &'a Option<Cow<'de, str>>,
}

impl<'de, T> DeserializeSeed<'de> for ValueDeserializeSeed<'de, '_, T>
where
    T: DeserializeSeed<'de>,
{
    type Value = T::Value;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        self.seed.deserialize(DelegatingDeserializer::new(
            ValueDeserializer {
                fields: self.fields,
                key: self.key,
            },
            deserializer,
        ))
    }
}

struct ValueDeserializer<'de, 'a> {
    fields: &'static [&'static str],
    key: &'a Option<Cow<'de, str>>,
}

impl<'de, D> Deserializer2<'de, D> for ValueDeserializer<'de, '_>
where
    D: Deserializer<'de>,
{
    fn deserialize_ignored_any<V>(self, _deserializer: D, _visitor: V) -> Result<V::Value, D::Error>
    where
        V: Visitor<'de>,
    {
        let key = match self.key {
            Some(key) => &**key,
            None => "<unknown>",
        };

        Err(Error::unknown_field(key, self.fields))
    }
}
