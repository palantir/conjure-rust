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
use base64::display::Base64Display;
use serde::ser;
use serde_json::ser::{CompactFormatter, Formatter};
use std::f32;
use std::f64;
use std::fmt;
use std::io::Write;

/// A serde JSON serializer compatible with the Conjure specification.
///
/// In contrast to serde_json, the f32 and f64 types are serialized as the strings `"Infinity"`, `"-Infinity"`, and
/// `"NaN"` when appropriate, and bytes are serialized as base64 encoded strings.
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

impl<'a, W, F> ser::Serializer for &'a mut Serializer<W, F>
where
    W: Write,
    F: Formatter,
{
    type Ok = <&'a mut serde_json::Serializer<W, F> as ser::Serializer>::Ok;
    type Error = <&'a mut serde_json::Serializer<W, F> as ser::Serializer>::Error;
    type SerializeSeq =
        SerializeSeq<<&'a mut serde_json::Serializer<W, F> as ser::Serializer>::SerializeSeq>;
    type SerializeTuple =
        SerializeTuple<<&'a mut serde_json::Serializer<W, F> as ser::Serializer>::SerializeTuple>;
    type SerializeTupleStruct = SerializeTupleStruct<
        <&'a mut serde_json::Serializer<W, F> as ser::Serializer>::SerializeTupleStruct,
    >;
    type SerializeTupleVariant = SerializeTupleVariant<
        <&'a mut serde_json::Serializer<W, F> as ser::Serializer>::SerializeTupleVariant,
    >;
    type SerializeMap =
        SerializeMap<<&'a mut serde_json::Serializer<W, F> as ser::Serializer>::SerializeMap>;
    type SerializeStruct =
        SerializeStruct<<&'a mut serde_json::Serializer<W, F> as ser::Serializer>::SerializeStruct>;
    type SerializeStructVariant = SerializeStructVariant<
        <&'a mut serde_json::Serializer<W, F> as ser::Serializer>::SerializeStructVariant,
    >;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_bool(v)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_i8(v)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_i16(v)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_i32(v)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_i64(v)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_u8(v)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_u16(v)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_u32(v)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_u64(v)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        if v.is_nan() {
            self.0.serialize_str("NaN")
        } else if v == f32::INFINITY {
            self.0.serialize_str("Infinity")
        } else if v == f32::NEG_INFINITY {
            self.0.serialize_str("-Infinity")
        } else {
            self.0.serialize_f32(v)
        }
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        if v.is_nan() {
            self.0.serialize_str("NaN")
        } else if v == f64::INFINITY {
            self.0.serialize_str("Infinity")
        } else if v == f64::NEG_INFINITY {
            self.0.serialize_str("-Infinity")
        } else {
            self.0.serialize_f64(v)
        }
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_char(v)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_str(v)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.0
            .collect_str(&Base64Display::with_config(v, base64::STANDARD))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_none()
    }

    fn serialize_some<U>(self, value: &U) -> Result<Self::Ok, Self::Error>
    where
        U: ?Sized + ser::Serialize,
    {
        self.0.serialize_some(&Serialize(value))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_unit()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_unit_struct(name)
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_unit_variant(name, variant_index, variant)
    }

    fn serialize_newtype_struct<U>(
        self,
        name: &'static str,
        value: &U,
    ) -> Result<Self::Ok, Self::Error>
    where
        U: ?Sized + ser::Serialize,
    {
        self.0.serialize_newtype_struct(name, &Serialize(value))
    }

    fn serialize_newtype_variant<U>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &U,
    ) -> Result<Self::Ok, Self::Error>
    where
        U: ?Sized + ser::Serialize,
    {
        self.0
            .serialize_newtype_variant(name, variant_index, variant, &Serialize(value))
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.0.serialize_seq(len).map(SerializeSeq)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.0.serialize_tuple(len).map(SerializeTuple)
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.0
            .serialize_tuple_struct(name, len)
            .map(SerializeTupleStruct)
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.0
            .serialize_tuple_variant(name, variant_index, variant, len)
            .map(SerializeTupleVariant)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.0.serialize_map(len).map(SerializeMap)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.0.serialize_struct(name, len).map(SerializeStruct)
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.0
            .serialize_struct_variant(name, variant_index, variant, len)
            .map(SerializeStructVariant)
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_i128(v)
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        self.0.serialize_u128(v)
    }

    fn collect_str<U>(self, value: &U) -> Result<Self::Ok, Self::Error>
    where
        U: ?Sized + fmt::Display,
    {
        self.0.collect_str(value)
    }

    // we can't delegate this due to the signature, but luckily we know the answer
    fn is_human_readable(&self) -> bool {
        true
    }
}

struct WrapSerializer<T>(T);

impl<T> ser::Serializer for WrapSerializer<T>
where
    T: ser::Serializer,
{
    type Ok = T::Ok;
    type Error = T::Error;
    type SerializeSeq = SerializeSeq<T::SerializeSeq>;
    type SerializeTuple = SerializeTuple<T::SerializeTuple>;
    type SerializeTupleStruct = SerializeTupleStruct<T::SerializeTupleStruct>;
    type SerializeTupleVariant = SerializeTupleVariant<T::SerializeTupleVariant>;
    type SerializeMap = SerializeMap<T::SerializeMap>;
    type SerializeStruct = SerializeStruct<T::SerializeStruct>;
    type SerializeStructVariant = SerializeStructVariant<T::SerializeStructVariant>;

    fn serialize_bool(self, v: bool) -> Result<T::Ok, T::Error> {
        self.0.serialize_bool(v)
    }

    fn serialize_i8(self, v: i8) -> Result<T::Ok, T::Error> {
        self.0.serialize_i8(v)
    }

    fn serialize_i16(self, v: i16) -> Result<T::Ok, T::Error> {
        self.0.serialize_i16(v)
    }

    fn serialize_i32(self, v: i32) -> Result<T::Ok, T::Error> {
        self.0.serialize_i32(v)
    }

    fn serialize_i64(self, v: i64) -> Result<T::Ok, T::Error> {
        self.0.serialize_i64(v)
    }

    fn serialize_u8(self, v: u8) -> Result<T::Ok, T::Error> {
        self.0.serialize_u8(v)
    }

    fn serialize_u16(self, v: u16) -> Result<T::Ok, T::Error> {
        self.0.serialize_u16(v)
    }

    fn serialize_u32(self, v: u32) -> Result<T::Ok, T::Error> {
        self.0.serialize_u32(v)
    }

    fn serialize_u64(self, v: u64) -> Result<T::Ok, T::Error> {
        self.0.serialize_u64(v)
    }

    fn serialize_f32(self, v: f32) -> Result<T::Ok, T::Error> {
        if v.is_nan() {
            self.0.serialize_str("NaN")
        } else if v == f32::INFINITY {
            self.0.serialize_str("Infinity")
        } else if v == f32::NEG_INFINITY {
            self.0.serialize_str("-Infinity")
        } else {
            self.0.serialize_f32(v)
        }
    }

    fn serialize_f64(self, v: f64) -> Result<T::Ok, T::Error> {
        if v.is_nan() {
            self.0.serialize_str("NaN")
        } else if v == f64::INFINITY {
            self.0.serialize_str("Infinity")
        } else if v == f64::NEG_INFINITY {
            self.0.serialize_str("-Infinity")
        } else {
            self.0.serialize_f64(v)
        }
    }

    fn serialize_char(self, v: char) -> Result<T::Ok, T::Error> {
        self.0.serialize_char(v)
    }

    fn serialize_str(self, v: &str) -> Result<T::Ok, T::Error> {
        self.0.serialize_str(v)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<T::Ok, T::Error> {
        self.0
            .collect_str(&Base64Display::with_config(v, base64::STANDARD))
    }

    fn serialize_none(self) -> Result<T::Ok, T::Error> {
        self.0.serialize_none()
    }

    fn serialize_some<U>(self, value: &U) -> Result<T::Ok, T::Error>
    where
        U: ?Sized + ser::Serialize,
    {
        self.0.serialize_some(&Serialize(value))
    }

    fn serialize_unit(self) -> Result<T::Ok, T::Error> {
        self.0.serialize_unit()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<T::Ok, T::Error> {
        self.0.serialize_unit_struct(name)
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<T::Ok, T::Error> {
        self.0.serialize_unit_variant(name, variant_index, variant)
    }

    fn serialize_newtype_struct<U>(self, name: &'static str, value: &U) -> Result<T::Ok, T::Error>
    where
        U: ?Sized + ser::Serialize,
    {
        self.0.serialize_newtype_struct(name, &Serialize(value))
    }

    fn serialize_newtype_variant<U>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &U,
    ) -> Result<T::Ok, T::Error>
    where
        U: ?Sized + ser::Serialize,
    {
        self.0
            .serialize_newtype_variant(name, variant_index, variant, &Serialize(value))
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<SerializeSeq<T::SerializeSeq>, T::Error> {
        self.0.serialize_seq(len).map(SerializeSeq)
    }

    fn serialize_tuple(self, len: usize) -> Result<SerializeTuple<T::SerializeTuple>, T::Error> {
        self.0.serialize_tuple(len).map(SerializeTuple)
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<SerializeTupleStruct<T::SerializeTupleStruct>, T::Error> {
        self.0
            .serialize_tuple_struct(name, len)
            .map(SerializeTupleStruct)
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<SerializeTupleVariant<T::SerializeTupleVariant>, T::Error> {
        self.0
            .serialize_tuple_variant(name, variant_index, variant, len)
            .map(SerializeTupleVariant)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<SerializeMap<T::SerializeMap>, T::Error> {
        self.0.serialize_map(len).map(SerializeMap)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<SerializeStruct<T::SerializeStruct>, T::Error> {
        self.0.serialize_struct(name, len).map(SerializeStruct)
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<SerializeStructVariant<T::SerializeStructVariant>, T::Error> {
        self.0
            .serialize_struct_variant(name, variant_index, variant, len)
            .map(SerializeStructVariant)
    }

    fn serialize_i128(self, v: i128) -> Result<T::Ok, T::Error> {
        self.0.serialize_i128(v)
    }

    fn serialize_u128(self, v: u128) -> Result<T::Ok, T::Error> {
        self.0.serialize_u128(v)
    }

    fn collect_str<U>(self, value: &U) -> Result<T::Ok, T::Error>
    where
        U: ?Sized + fmt::Display,
    {
        self.0.collect_str(value)
    }

    fn is_human_readable(&self) -> bool {
        self.0.is_human_readable()
    }
}

struct Serialize<'a, T: ?Sized + 'a>(&'a T);

impl<'a, T> ser::Serialize for Serialize<'a, T>
where
    T: ?Sized + ser::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self.0.serialize(WrapSerializer(serializer))
    }
}

pub struct SerializeSeq<T>(T);

impl<T> ser::SerializeSeq for SerializeSeq<T>
where
    T: ser::SerializeSeq,
{
    type Ok = T::Ok;
    type Error = T::Error;

    fn serialize_element<U>(&mut self, value: &U) -> Result<(), T::Error>
    where
        U: ?Sized + ser::Serialize,
    {
        self.0.serialize_element(&Serialize(value))
    }

    fn end(self) -> Result<T::Ok, T::Error> {
        self.0.end()
    }
}

pub struct SerializeTuple<T>(T);

impl<T> ser::SerializeTuple for SerializeTuple<T>
where
    T: ser::SerializeTuple,
{
    type Ok = T::Ok;
    type Error = T::Error;

    fn serialize_element<U>(&mut self, value: &U) -> Result<(), T::Error>
    where
        U: ?Sized + ser::Serialize,
    {
        self.0.serialize_element(&Serialize(value))
    }

    fn end(self) -> Result<T::Ok, T::Error> {
        self.0.end()
    }
}

pub struct SerializeTupleStruct<T>(T);

impl<T> ser::SerializeTupleStruct for SerializeTupleStruct<T>
where
    T: ser::SerializeTupleStruct,
{
    type Ok = T::Ok;
    type Error = T::Error;

    fn serialize_field<U>(&mut self, value: &U) -> Result<(), T::Error>
    where
        U: ?Sized + ser::Serialize,
    {
        self.0.serialize_field(&Serialize(value))
    }

    fn end(self) -> Result<T::Ok, T::Error> {
        self.0.end()
    }
}

pub struct SerializeTupleVariant<T>(T);

impl<T> ser::SerializeTupleVariant for SerializeTupleVariant<T>
where
    T: ser::SerializeTupleVariant,
{
    type Ok = T::Ok;
    type Error = T::Error;

    fn serialize_field<U>(&mut self, value: &U) -> Result<(), T::Error>
    where
        U: ?Sized + ser::Serialize,
    {
        self.0.serialize_field(&Serialize(value))
    }

    fn end(self) -> Result<T::Ok, T::Error> {
        self.0.end()
    }
}

pub struct SerializeMap<T>(T);

impl<T> ser::SerializeMap for SerializeMap<T>
where
    T: ser::SerializeMap,
{
    type Ok = T::Ok;
    type Error = T::Error;

    fn serialize_key<U>(&mut self, key: &U) -> Result<(), T::Error>
    where
        U: ?Sized + ser::Serialize,
    {
        self.0.serialize_key(&Serialize(key))
    }

    fn serialize_value<U>(&mut self, value: &U) -> Result<(), T::Error>
    where
        U: ?Sized + ser::Serialize,
    {
        self.0.serialize_value(&Serialize(value))
    }

    fn end(self) -> Result<T::Ok, T::Error> {
        self.0.end()
    }

    fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<(), T::Error>
    where
        K: ?Sized + ser::Serialize,
        V: ?Sized + ser::Serialize,
    {
        self.0.serialize_entry(&Serialize(key), &Serialize(value))
    }
}

pub struct SerializeStruct<T>(T);

impl<T> ser::SerializeStruct for SerializeStruct<T>
where
    T: ser::SerializeStruct,
{
    type Ok = T::Ok;
    type Error = T::Error;

    fn serialize_field<U>(&mut self, key: &'static str, value: &U) -> Result<(), T::Error>
    where
        U: ?Sized + ser::Serialize,
    {
        self.0.serialize_field(key, &Serialize(value))
    }

    fn end(self) -> Result<T::Ok, T::Error> {
        self.0.end()
    }

    fn skip_field(&mut self, key: &'static str) -> Result<(), T::Error> {
        self.0.skip_field(key)
    }
}

pub struct SerializeStructVariant<T>(T);

impl<T> ser::SerializeStructVariant for SerializeStructVariant<T>
where
    T: ser::SerializeStructVariant,
{
    type Ok = T::Ok;
    type Error = T::Error;

    fn serialize_field<U>(&mut self, key: &'static str, value: &U) -> Result<(), T::Error>
    where
        U: ?Sized + ser::Serialize,
    {
        self.0.serialize_field(key, &Serialize(value))
    }

    fn end(self) -> Result<T::Ok, T::Error> {
        self.0.end()
    }

    fn skip_field(&mut self, key: &'static str) -> Result<(), T::Error> {
        self.0.skip_field(key)
    }
}
