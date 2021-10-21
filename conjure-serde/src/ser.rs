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
use serde::ser::{
    SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
    SerializeTupleStruct, SerializeTupleVariant,
};
use serde::{Serialize, Serializer};
use std::marker::PhantomData;

macro_rules! impl_serialize_body {
    ($inner:ty, $behavior:ty) => {
        type Ok = <$inner as ser::Serializer>::Ok;

        type Error = <$inner as ser::Serializer>::Error;

        type SerializeSeq = Override<
            <$inner as ser::Serializer>::SerializeSeq,
            ValueBehavior,
        >;

        type SerializeTuple = Override<
            <$inner as ser::Serializer>::SerializeTuple,
            ValueBehavior,
        >;

        type SerializeTupleStruct = Override<
            <$inner as ser::Serializer>::SerializeTupleStruct,
            ValueBehavior,
        >;

        type SerializeTupleVariant = Override<
            <$inner as ser::Serializer>::SerializeTupleVariant,
            ValueBehavior,
        >;

        type SerializeMap = Override<
            <$inner as ser::Serializer>::SerializeMap,
            ValueBehavior,
        >;

        type SerializeStruct = Override<
            <$inner as ser::Serializer>::SerializeStruct,
            ValueBehavior,
        >;

        type SerializeStructVariant = Override<
            <$inner as ser::Serializer>::SerializeStructVariant,
            ValueBehavior,
        >;

        impl_serialize_body! {
            @delegate
            $behavior,
            serialize_bool = bool,
            serialize_i8 = i8,
            serialize_i16 = i16,
            serialize_i32 = i32,
            serialize_i64 = i64,
            serialize_i128 = i128,
            serialize_u8 = u8,
            serialize_u16 = u16,
            serialize_u32 = u32,
            serialize_u64 = u64,
            serialize_u128 = u128,
            serialize_f32 = f32,
            serialize_f64 = f64,
            serialize_char = char,
            serialize_str = &str,
            serialize_bytes = &[u8],
        }

        fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
            $crate::ser::Override::<_, $behavior>::new(&mut self.0).serialize_none()
        }

        fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
        where
            T: ?Sized + serde::Serialize,
        {
            $crate::ser::Override::<_, $behavior>::new(&mut self.0).serialize_some(value)
        }

        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
            $crate::ser::Override::<_, $behavior>::new(&mut self.0).serialize_unit()
        }

        fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
            $crate::ser::Override::<_, $behavior>::new(&mut self.0).serialize_unit_struct(name)
        }

        fn serialize_unit_variant(
            self,
            name: &'static str,
            variant_index: u32,
            variant: &'static str,
        ) -> Result<Self::Ok, Self::Error> {
            $crate::ser::Override::<_, $behavior>::new(&mut self.0)
                .serialize_unit_variant(name, variant_index, variant)
        }

        fn serialize_newtype_struct<T>(
            self,
            name: &'static str,
            value: &T,
        ) -> Result<Self::Ok, Self::Error>
        where
            T: ?Sized + serde::Serialize,
        {
            $crate::ser::Override::<_, $behavior>::new(&mut self.0).serialize_newtype_struct(name, value)
        }

        fn serialize_newtype_variant<T>(
            self,
            name: &'static str,
            variant_index: u32,
            variant: &'static str,
            value: &T,
        ) -> Result<Self::Ok, Self::Error>
        where
            T: ?Sized + serde::Serialize,
        {
            $crate::ser::Override::<_, $behavior>::new(&mut self.0)
                .serialize_newtype_variant(name, variant_index, variant, value)
        }

        fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
            $crate::ser::Override::<_, $behavior>::new(&mut self.0).serialize_seq(len)
        }

        fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
            $crate::ser::Override::<_, $behavior>::new(&mut self.0).serialize_tuple(len)
        }

        fn serialize_tuple_struct(
            self,
            name: &'static str,
            len: usize,
        ) -> Result<Self::SerializeTupleStruct, Self::Error> {
            $crate::ser::Override::<_, $behavior>::new(&mut self.0).serialize_tuple_struct(name, len)
        }

        fn serialize_tuple_variant(
            self,
            name: &'static str,
            variant_index: u32,
            variant: &'static str,
            len: usize,
        ) -> Result<Self::SerializeTupleVariant, Self::Error> {
            $crate::ser::Override::<_, $behavior>::new(&mut self.0)
                .serialize_tuple_variant(name, variant_index, variant, len)
        }

        fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
            $crate::ser::Override::<_, $behavior>::new(&mut self.0).serialize_map(len)
        }

        fn serialize_struct(
            self,
            name: &'static str,
            len: usize,
        ) -> Result<Self::SerializeStruct, Self::Error> {
            $crate::ser::Override::<_, $behavior>::new(&mut self.0).serialize_struct(name, len)
        }

        fn serialize_struct_variant(
            self,
            name: &'static str,
            variant_index: u32,
            variant: &'static str,
            len: usize,
        ) -> Result<Self::SerializeStructVariant, Self::Error> {
            $crate::ser::Override::<_, $behavior>::new(&mut self.0)
                .serialize_struct_variant(name, variant_index, variant, len)
        }
    };
    (@delegate $behavior:ty, $($name:ident = $t:ty,)*) => {
        $(
            fn $name(self, v: $t) -> Result<Self::Ok, Self::Error> {
                $crate::ser::Override::<_, $behavior>::new(&mut self.0).$name(v)
            }
        )*
    }
}

pub trait Behavior {
    type KeyBehavior: Behavior;

    fn serialize_bool<S>(ser: S, v: bool) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ser.serialize_bool(v)
    }

    fn serialize_f32<S>(ser: S, v: f32) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ser.serialize_f32(v)
    }

    fn serialize_f64<S>(ser: S, v: f64) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ser.serialize_f64(v)
    }

    fn serialize_bytes<S>(ser: S, v: &[u8]) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ser.serialize_bytes(v)
    }
}

pub struct Override<S, B> {
    inner: S,
    _p: PhantomData<B>,
}

impl<S, B> Override<S, B> {
    pub fn new(inner: S) -> Self {
        Override {
            inner,
            _p: PhantomData,
        }
    }
}

macro_rules! delegate {
    ($($name:ident = $t:ty,)*) => {
        $(
            fn $name(self, v: $t) -> Result<Self::Ok, Self::Error> {
                self.inner.$name(v)
            }
        )*
    }
}

macro_rules! behavior {
    ($($name:ident = $t:ty,)*) => {
        $(
            fn $name(self, v: $t) -> Result<Self::Ok, Self::Error> {
                B::$name(self.inner, v)
            }
        )*
    }
}

impl<S, B> Serializer for Override<S, B>
where
    S: Serializer,
    B: Behavior,
{
    type Ok = S::Ok;

    type Error = S::Error;

    type SerializeSeq = Override<S::SerializeSeq, B>;

    type SerializeTuple = Override<S::SerializeTuple, B>;

    type SerializeTupleStruct = Override<S::SerializeTupleStruct, B>;

    type SerializeTupleVariant = Override<S::SerializeTupleVariant, B>;

    type SerializeMap = Override<S::SerializeMap, B>;

    type SerializeStruct = Override<S::SerializeStruct, B>;

    type SerializeStructVariant = Override<S::SerializeStructVariant, B>;

    delegate! {
        serialize_i8 = i8,
        serialize_i16 = i16,
        serialize_i32 = i32,
        serialize_i64 = i64,
        serialize_i128 = i128,
        serialize_u8 = u8,
        serialize_u16 = u16,
        serialize_u32 = u32,
        serialize_u64 = u64,
        serialize_u128 = u128,
        serialize_char = char,
        serialize_str = &str,
    }

    behavior! {
        serialize_bool = bool,
        serialize_f32 = f32,
        serialize_f64 = f64,
        serialize_bytes = &[u8],
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_none()
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.inner.serialize_some(&Override::<_, B>::new(value))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_unit()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.inner.serialize_unit_struct(name)
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.inner
            .serialize_unit_variant(name, variant_index, variant)
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.inner
            .serialize_newtype_struct(name, &Override::<_, B>::new(value))
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.inner.serialize_newtype_variant(
            name,
            variant_index,
            variant,
            &Override::<_, B>::new(value),
        )
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.inner.serialize_seq(len).map(Override::<_, B>::new)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.inner.serialize_tuple(len).map(Override::<_, B>::new)
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.inner
            .serialize_tuple_struct(name, len)
            .map(Override::<_, B>::new)
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.inner
            .serialize_tuple_variant(name, variant_index, variant, len)
            .map(Override::<_, B>::new)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.inner.serialize_map(len).map(Override::<_, B>::new)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.inner
            .serialize_struct(name, len)
            .map(Override::<_, B>::new)
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.inner
            .serialize_struct_variant(name, variant_index, variant, len)
            .map(Override::<_, B>::new)
    }
}

impl<T, B> Serialize for Override<T, B>
where
    T: Serialize,
    B: Behavior,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.inner.serialize(Override::<_, B>::new(serializer))
    }
}

impl<S, B> SerializeSeq for Override<S, B>
where
    S: SerializeSeq,
    B: Behavior,
{
    type Ok = S::Ok;

    type Error = S::Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.inner.serialize_element(&Override::<_, B>::new(value))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.inner.end()
    }
}

impl<S, B> SerializeTuple for Override<S, B>
where
    S: SerializeTuple,
    B: Behavior,
{
    type Ok = S::Ok;

    type Error = S::Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.inner.serialize_element(&Override::<_, B>::new(value))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.inner.end()
    }
}

impl<S, B> SerializeTupleStruct for Override<S, B>
where
    S: SerializeTupleStruct,
    B: Behavior,
{
    type Ok = S::Ok;

    type Error = S::Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.inner.serialize_field(&Override::<_, B>::new(value))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.inner.end()
    }
}

impl<S, B> SerializeTupleVariant for Override<S, B>
where
    S: SerializeTupleVariant,
    B: Behavior,
{
    type Ok = S::Ok;

    type Error = S::Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.inner.serialize_field(&Override::<_, B>::new(value))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.inner.end()
    }
}

impl<S, B> SerializeMap for Override<S, B>
where
    S: SerializeMap,
    B: Behavior,
{
    type Ok = S::Ok;

    type Error = S::Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.inner
            .serialize_key(&Override::<_, B::KeyBehavior>::new(key))
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.inner.serialize_value(&Override::<_, B>::new(value))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.inner.end()
    }
}

impl<S, B> SerializeStruct for Override<S, B>
where
    S: SerializeStruct,
    B: Behavior,
{
    type Ok = S::Ok;

    type Error = S::Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.inner
            .serialize_field(key, &Override::<_, B>::new(value))
    }

    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> {
        self.inner.skip_field(key)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.inner.end()
    }
}

impl<S, B> SerializeStructVariant for Override<S, B>
where
    S: SerializeStructVariant,
    B: Behavior,
{
    type Ok = S::Ok;

    type Error = S::Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.inner
            .serialize_field(key, &Override::<_, B>::new(value))
    }

    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> {
        self.inner.skip_field(key)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.inner.end()
    }
}
