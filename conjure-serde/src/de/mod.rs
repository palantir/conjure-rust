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
use serde::de::{self, DeserializeSeed, EnumAccess, MapAccess, SeqAccess, VariantAccess, Visitor};
use serde::Deserializer;
use std::marker::PhantomData;

pub mod delegating_deserializer;
pub mod delegating_visitor;
pub mod null_collections_behavior;
pub mod unknown_fields_behavior;
pub mod wrapping_deserializer;

macro_rules! impl_deserialize_body {
    ($inner:ty, $behavior:ty) => {
        type Error = <$inner as de::Deserializer<'de>>::Error;

        impl_deserialize_body! {
            @delegate
            $behavior,
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
            V: de::Visitor<'de>
        {
            $crate::de::Override::<_, $behavior>::new(&mut self.0).deserialize_unit_struct(name, visitor)
        }

        fn deserialize_newtype_struct<V>(
            self,
            name: &'static str,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>
        {
            $crate::de::Override::<_, $behavior>::new(&mut self.0).deserialize_newtype_struct(name, visitor)
        }

        fn deserialize_tuple<V>(
            self,
            len: usize,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>
        {
            $crate::de::Override::<_, $behavior>::new(&mut self.0).deserialize_tuple(len, visitor)
        }

        fn deserialize_tuple_struct<V>(
            self,
            name: &'static str,
            len: usize,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>
        {
            $crate::de::Override::<_, $behavior>::new(&mut self.0).deserialize_tuple_struct(name, len, visitor)
        }

        fn deserialize_struct<V>(
            self,
            name: &'static str,
            fields: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>
        {
            $crate::de::Override::<_, $behavior>::new(&mut self.0).deserialize_struct(name, fields, visitor)
        }

        fn deserialize_enum<V>(
            self,
            name: &'static str,
            variants: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>
        {
            $crate::de::Override::<_, $behavior>::new(&mut self.0).deserialize_enum(name, variants, visitor)
        }
    };
    (@delegate $behavior:ty, $($method:ident,)*) => {
        $(
            fn $method<V>(self, visitor: V) -> Result<V::Value, Self::Error>
            where
                V: de::Visitor<'de>
            {
                $crate::de::Override::<_, $behavior>::new(&mut self.0).$method(visitor)
            }
        )*
    }
}

pub trait Behavior {
    type KeyBehavior: Behavior;

    fn deserialize_bool<'de, D, V>(de: D, visitor: V) -> Result<V::Value, D::Error>
    where
        D: Deserializer<'de>,
        V: Visitor<'de>,
    {
        de.deserialize_bool(visitor)
    }

    fn deserialize_f32<'de, D, V>(de: D, visitor: V) -> Result<V::Value, D::Error>
    where
        D: Deserializer<'de>,
        V: Visitor<'de>,
    {
        de.deserialize_f32(visitor)
    }

    fn deserialize_f64<'de, D, V>(de: D, visitor: V) -> Result<V::Value, D::Error>
    where
        D: Deserializer<'de>,
        V: Visitor<'de>,
    {
        de.deserialize_f64(visitor)
    }

    fn deserialize_bytes<'de, D, V>(de: D, visitor: V) -> Result<V::Value, D::Error>
    where
        D: Deserializer<'de>,
        V: Visitor<'de>,
    {
        de.deserialize_bytes(visitor)
    }

    fn deserialize_byte_buf<'de, D, V>(de: D, visitor: V) -> Result<V::Value, D::Error>
    where
        D: Deserializer<'de>,
        V: Visitor<'de>,
    {
        de.deserialize_byte_buf(visitor)
    }

    fn deserialize_seq<'de, D, V>(de: D, visitor: V) -> Result<V::Value, D::Error>
    where
        D: Deserializer<'de>,
        V: Visitor<'de>,
    {
        de.deserialize_seq(visitor)
    }

    fn deserialize_map<'de, D, V>(de: D, visitor: V) -> Result<V::Value, D::Error>
    where
        D: Deserializer<'de>,
        V: Visitor<'de>,
    {
        de.deserialize_map(visitor)
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
        de.deserialize_struct(name, fields, visitor)
    }
}

pub struct Override<T, B> {
    inner: T,
    _p: PhantomData<B>,
}

impl<T, B> Override<T, B> {
    pub fn new(inner: T) -> Self {
        Override {
            inner,
            _p: PhantomData,
        }
    }
}

macro_rules! delegate_deserialize {
    ($($method:ident,)*) => {
        $(
            fn $method<V>(self, visitor: V) -> Result<V::Value, T::Error>
            where
                V: Visitor<'de>
            {
                self.inner.$method(Override::<_, B>::new(visitor))
            }
        )*
    }
}

macro_rules! behavior {
    ($($method:ident,)*) => {
        $(
            fn $method<V>(self, visitor: V) -> Result<V::Value, T::Error>
            where
                V: Visitor<'de>
            {
                B::$method(self.inner, Override::<_, B>::new(visitor))
            }
        )*
    }
}

impl<'de, T, B> Deserializer<'de> for Override<T, B>
where
    T: Deserializer<'de>,
    B: Behavior,
{
    type Error = T::Error;

    delegate_deserialize!(
        deserialize_any,
        deserialize_i8,
        deserialize_i16,
        deserialize_i32,
        deserialize_i64,
        deserialize_i128,
        deserialize_u8,
        deserialize_u16,
        deserialize_u32,
        deserialize_u64,
        deserialize_u128,
        deserialize_char,
        deserialize_str,
        deserialize_string,
        deserialize_option,
        deserialize_unit,
        deserialize_identifier,
        deserialize_ignored_any,
    );

    behavior! {
        deserialize_bool,
        deserialize_f32,
        deserialize_f64,
        deserialize_bytes,
        deserialize_byte_buf,
        deserialize_seq,
        deserialize_map,
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.inner
            .deserialize_unit_struct(name, Override::<_, B>::new(visitor))
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.inner
            .deserialize_newtype_struct(name, Override::<_, B>::new(visitor))
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.inner
            .deserialize_tuple(len, Override::<_, B>::new(visitor))
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
        self.inner
            .deserialize_tuple_struct(name, len, Override::<_, B>::new(visitor))
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
        B::deserialize_struct(self.inner, name, fields, Override::<_, B>::new(visitor))
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
        self.inner
            .deserialize_enum(name, variants, Override::<_, B>::new(visitor))
    }

    fn is_human_readable(&self) -> bool {
        self.inner.is_human_readable()
    }
}

macro_rules! delegate_visit {
    ($($method:ident = $ty:ty,)*) => {
        $(
            fn $method<E>(self, v: $ty) -> Result<V::Value, E>
            where
                E: de::Error,
            {
                self.inner.$method(v)
            }
        )*
    };
}

impl<'de, V, B> Visitor<'de> for Override<V, B>
where
    V: Visitor<'de>,
    B: Behavior,
{
    type Value = V::Value;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.inner.expecting(formatter)
    }

    delegate_visit!(
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
    );

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.inner.visit_none()
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        self.inner.visit_some(Override::<_, B>::new(deserializer))
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.inner.visit_unit()
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        self.inner
            .visit_newtype_struct(Override::<_, B>::new(deserializer))
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        self.inner.visit_seq(Override::<_, B>::new(seq))
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        self.inner.visit_map(Override::<_, B>::new(map))
    }

    fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
    where
        A: EnumAccess<'de>,
    {
        self.inner.visit_enum(Override::<_, B>::new(data))
    }
}

impl<'de, A, B> SeqAccess<'de> for Override<A, B>
where
    A: SeqAccess<'de>,
    B: Behavior,
{
    type Error = A::Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        self.inner.next_element_seed(Override::<_, B>::new(seed))
    }

    fn size_hint(&self) -> Option<usize> {
        self.inner.size_hint()
    }
}

impl<'de, A, B> MapAccess<'de> for Override<A, B>
where
    A: MapAccess<'de>,
    B: Behavior,
{
    type Error = A::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        self.inner
            .next_key_seed(Override::<_, B::KeyBehavior>::new(seed))
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        self.inner.next_value_seed(Override::<_, B>::new(seed))
    }

    fn size_hint(&self) -> Option<usize> {
        self.inner.size_hint()
    }
}

impl<'de, A, B> EnumAccess<'de> for Override<A, B>
where
    A: EnumAccess<'de>,
    B: Behavior,
{
    type Error = A::Error;

    type Variant = Override<A::Variant, B>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        self.inner
            .variant_seed(Override::<_, B>::new(seed))
            .map(|(value, variant)| (value, Override::new(variant)))
    }
}

impl<'de, A, B> VariantAccess<'de> for Override<A, B>
where
    A: VariantAccess<'de>,
    B: Behavior,
{
    type Error = A::Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        self.inner.unit_variant()
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        self.inner.newtype_variant_seed(Override::<_, B>::new(seed))
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.inner
            .tuple_variant(len, Override::<_, B>::new(visitor))
    }

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.inner
            .struct_variant(fields, Override::<_, B>::new(visitor))
    }
}

impl<'de, T, B> DeserializeSeed<'de> for Override<T, B>
where
    T: DeserializeSeed<'de>,
    B: Behavior,
{
    type Value = T::Value;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        self.inner.deserialize(Override::<_, B>::new(deserializer))
    }
}
