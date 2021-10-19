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
use serde::de::{EnumAccess, Error, MapAccess, SeqAccess, Visitor};
use serde::Deserializer;
use std::fmt;

macro_rules! delegate {
    ($($method:ident = $ty:ty,)*) => {
        $(
            fn $method<E>(self, visitor: V, v: $ty) -> Result<V::Value, E>
            where
                E: Error,
            {
                visitor.$method(v)
            }
        )*
    };
}

pub trait Visitor2<'de, V>
where
    Self: Sized,
    V: Visitor<'de>,
{
    fn expecting(&self, visitor: &V, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        visitor.expecting(formatter)
    }

    delegate! {
        visit_bool = bool,
        visit_i8 = i8,
        visit_i16 = i16,
        visit_i32 = i32,
        visit_i64 = i64,
        visit_i128 = i128,
        visit_u8 = u8,
        visit_u16 = u16,
        visit_u32 = u32,
        visit_u64 = u64,
        visit_u128 = u128,
        visit_f32 = f32,
        visit_f64 = f64,
        visit_char = char,
        visit_str = &str,
        visit_borrowed_str = &'de str,
        visit_string = String,
        visit_bytes = &[u8],
        visit_borrowed_bytes = &'de [u8],
        visit_byte_buf = Vec<u8>,
    }

    fn visit_none<E>(self, visitor: V) -> Result<V::Value, E>
    where
        E: Error,
    {
        visitor.visit_none()
    }

    fn visit_some<D>(self, visitor: V, deserializer: D) -> Result<V::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        visitor.visit_some(deserializer)
    }

    fn visit_unit<E>(self, visitor: V) -> Result<V::Value, E>
    where
        E: Error,
    {
        visitor.visit_unit()
    }

    fn visit_newtype_struct<D>(self, visitor: V, deserializer: D) -> Result<V::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        visitor.visit_newtype_struct(deserializer)
    }

    fn visit_seq<A>(self, visitor: V, seq: A) -> Result<V::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        visitor.visit_seq(seq)
    }

    fn visit_map<A>(self, visitor: V, map: A) -> Result<V::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        visitor.visit_map(map)
    }

    fn visit_enum<A>(self, visitor: V, data: A) -> Result<V::Value, A::Error>
    where
        A: EnumAccess<'de>,
    {
        visitor.visit_enum(data)
    }
}

pub struct DelegatingVisitor<T, V> {
    custom: T,
    inner: V,
}

impl<T, V> DelegatingVisitor<T, V> {
    pub fn new(custom: T, inner: V) -> Self {
        DelegatingVisitor { custom, inner }
    }
}

macro_rules! delegate_impl {
    ($($method:ident = $ty:ty,)*) => {
        $(
            fn $method<E>(self, v: $ty) -> Result<Self::Value, E>
            where
                E: Error,
            {
                self.custom.$method(self.inner, v)
            }
        )*
    };
}

impl<'de, T, V> Visitor<'de> for DelegatingVisitor<T, V>
where
    T: Visitor2<'de, V>,
    V: Visitor<'de>,
{
    type Value = V::Value;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.custom.expecting(&self.inner, formatter)
    }

    delegate_impl! {
        visit_bool = bool,
        visit_i8 = i8,
        visit_i16 = i16,
        visit_i32 = i32,
        visit_i64 = i64,
        visit_i128 = i128,
        visit_u8 = u8,
        visit_u16 = u16,
        visit_u32 = u32,
        visit_u64 = u64,
        visit_u128 = u128,
        visit_f32 = f32,
        visit_f64 = f64,
        visit_char = char,
        visit_str = &str,
        visit_borrowed_str = &'de str,
        visit_string = String,
        visit_bytes = &[u8],
        visit_borrowed_bytes = &'de [u8],
        visit_byte_buf = Vec<u8>,
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.custom.visit_none(self.inner)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        self.custom.visit_some(self.inner, deserializer)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.custom.visit_unit(self.inner)
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        self.custom.visit_newtype_struct(self.inner, deserializer)
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        self.custom.visit_seq(self.inner, seq)
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        self.custom.visit_map(self.inner, map)
    }

    fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
    where
        A: EnumAccess<'de>,
    {
        self.custom.visit_enum(self.inner, data)
    }
}
