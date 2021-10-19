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
use serde::de::Visitor;
use serde::Deserializer;

macro_rules! delegate {
    ($($method:ident,)*) => {
        $(
            fn $method<V>(self, deserializer: D, visitor: V) -> Result<V::Value, D::Error>
            where
                V: Visitor<'de>,
            {
                deserializer.$method(visitor)
            }
        )*
    };
}

pub trait Deserializer2<'de, D>
where
    Self: Sized,
    D: Deserializer<'de>,
{
    delegate! {
        deserialize_any,
        deserialize_bool,
        deserialize_i8,
        deserialize_i16,
        deserialize_i32,
        deserialize_i64,
        deserialize_u8,
        deserialize_u16,
        deserialize_u32,
        deserialize_u64,
        deserialize_f32,
        deserialize_f64,
        deserialize_char,
        deserialize_str,
        deserialize_string,
        deserialize_bytes,
        deserialize_byte_buf,
        deserialize_option,
        deserialize_unit,
        deserialize_seq,
        deserialize_map,
        deserialize_identifier,
        deserialize_ignored_any,
        deserialize_i128,
        deserialize_u128,
    }

    fn deserialize_unit_struct<V>(
        self,
        deserializer: D,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, D::Error>
    where
        V: Visitor<'de>,
    {
        deserializer.deserialize_unit_struct(name, visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        deserializer: D,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, D::Error>
    where
        V: Visitor<'de>,
    {
        deserializer.deserialize_newtype_struct(name, visitor)
    }

    fn deserialize_tuple<V>(
        self,
        deserializer: D,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, D::Error>
    where
        V: Visitor<'de>,
    {
        deserializer.deserialize_tuple(len, visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        deserializer: D,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, D::Error>
    where
        V: Visitor<'de>,
    {
        deserializer.deserialize_tuple_struct(name, len, visitor)
    }

    fn deserialize_struct<V>(
        self,
        deserializer: D,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, D::Error>
    where
        V: Visitor<'de>,
    {
        deserializer.deserialize_struct(name, fields, visitor)
    }

    fn deserialize_enum<V>(
        self,
        deserializer: D,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, D::Error>
    where
        V: Visitor<'de>,
    {
        deserializer.deserialize_enum(name, variants, visitor)
    }

    fn is_human_readable(&self, deserializer: &D) -> bool {
        deserializer.is_human_readable()
    }
}

pub struct DelegatingDeserializer<T, D> {
    custom: T,
    inner: D,
}

impl<T, D> DelegatingDeserializer<T, D> {
    pub fn new(custom: T, inner: D) -> Self {
        DelegatingDeserializer { custom, inner }
    }
}

macro_rules! delegate_impl {
    ($($method:ident,)*) => {
        $(
            fn $method<V>(self, visitor: V) -> Result<V::Value, Self::Error>
            where
                V: Visitor<'de>,
            {
                self.custom.$method(self.inner, visitor)
            }
        )*
    }
}

impl<'de, T, D> Deserializer<'de> for DelegatingDeserializer<T, D>
where
    T: Deserializer2<'de, D>,
    D: Deserializer<'de>,
{
    type Error = D::Error;

    delegate_impl! {
        deserialize_any,
        deserialize_bool,
        deserialize_i8,
        deserialize_i16,
        deserialize_i32,
        deserialize_i64,
        deserialize_u8,
        deserialize_u16,
        deserialize_u32,
        deserialize_u64,
        deserialize_f32,
        deserialize_f64,
        deserialize_char,
        deserialize_str,
        deserialize_string,
        deserialize_bytes,
        deserialize_byte_buf,
        deserialize_option,
        deserialize_unit,
        deserialize_seq,
        deserialize_map,
        deserialize_identifier,
        deserialize_ignored_any,
        deserialize_i128,
        deserialize_u128,
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.custom
            .deserialize_unit_struct(self.inner, name, visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.custom
            .deserialize_newtype_struct(self.inner, name, visitor)
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.custom.deserialize_tuple(self.inner, len, visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.custom
            .deserialize_tuple_struct(self.inner, name, len, visitor)
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.custom
            .deserialize_struct(self.inner, name, fields, visitor)
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.custom
            .deserialize_enum(self.inner, name, variants, visitor)
    }

    fn is_human_readable(&self) -> bool {
        self.custom.is_human_readable(&self.inner)
    }
}
