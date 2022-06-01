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
use crate::any::{Any, Error, Inner};
use ordered_float::OrderedFloat;
use serde::ser::{
    Error as _, SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant,
    SerializeTuple, SerializeTupleStruct, SerializeTupleVariant,
};
use serde::{Serialize, Serializer};
use std::collections::BTreeMap;
use std::f64;
use std::fmt;

impl Serialize for Any {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self.0 {
            Inner::Null => serializer.serialize_unit(),
            Inner::Bool(v) => serializer.serialize_bool(*v),
            Inner::I8(v) => serializer.serialize_i8(*v),
            Inner::I16(v) => serializer.serialize_i16(*v),
            Inner::I32(v) => serializer.serialize_i32(*v),
            Inner::I64(v) => serializer.serialize_i64(*v),
            Inner::I128(v) => serializer.serialize_i128(*v),
            Inner::U8(v) => serializer.serialize_u8(*v),
            Inner::U16(v) => serializer.serialize_u16(*v),
            Inner::U32(v) => serializer.serialize_u32(*v),
            Inner::U64(v) => serializer.serialize_u64(*v),
            Inner::U128(v) => serializer.serialize_u128(*v),
            Inner::F32(v) => serializer.serialize_f32(v.0),
            Inner::F64(v) => serializer.serialize_f64(v.0),
            Inner::Char(v) => serializer.serialize_char(*v),
            Inner::String(v) => serializer.serialize_str(v),
            Inner::Bytes(v) => serializer.serialize_bytes(v),
            Inner::Seq(v) => v.serialize(serializer),
            Inner::Map(v) => v.serialize(serializer),
        }
    }
}

pub(crate) struct AnySerializer;

impl Serializer for AnySerializer {
    type Ok = Any;
    type Error = Error;
    type SerializeSeq = SeqSerializer;
    type SerializeTuple = SeqSerializer;
    type SerializeTupleStruct = SeqSerializer;
    type SerializeTupleVariant = TupleVariantSerializer;
    type SerializeMap = MapSerializer;
    type SerializeStruct = MapSerializer;
    type SerializeStructVariant = StructVariantSerializer;

    #[inline]
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(Any(Inner::Bool(v)))
    }

    #[inline]
    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(Any(Inner::I8(v)))
    }

    #[inline]
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(Any(Inner::I16(v)))
    }

    #[inline]
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(Any(Inner::I32(v)))
    }

    #[inline]
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(Any(Inner::I64(v)))
    }

    #[inline]
    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        Ok(Any(Inner::I128(v)))
    }

    #[inline]
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(Any(Inner::U8(v)))
    }

    #[inline]
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(Any(Inner::U16(v)))
    }

    #[inline]
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(Any(Inner::U32(v)))
    }

    #[inline]
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(Any(Inner::U64(v)))
    }

    #[inline]
    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        Ok(Any(Inner::U128(v)))
    }

    #[inline]
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(Any(Inner::F32(OrderedFloat(v))))
    }

    #[inline]
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(Any(Inner::F64(OrderedFloat(v))))
    }

    #[inline]
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(Any(Inner::String(v.to_string())))
    }

    #[inline]
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(Any(Inner::String(v.to_string())))
    }

    #[inline]
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(Any(Inner::Bytes(v.to_vec())))
    }

    #[inline]
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(Any(Inner::Null))
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(Any(Inner::Null))
    }

    #[inline]
    fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let mut map = BTreeMap::new();
        let value = value.serialize(self)?;
        map.insert(Any(Inner::String(variant.to_string())), value);
        Ok(Any(Inner::Map(map)))
    }

    #[inline]
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(SeqSerializer(Vec::with_capacity(len.unwrap_or(0))))
    }

    #[inline]
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    #[inline]
    fn serialize_tuple_struct(
        self,
        _: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(Some(len))
    }

    #[inline]
    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(TupleVariantSerializer {
            variant,
            vec: Vec::with_capacity(len),
        })
    }

    #[inline]
    fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(MapSerializer {
            map: BTreeMap::new(),
            key: None,
        })
    }

    #[inline]
    fn serialize_struct(
        self,
        _: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.serialize_map(Some(len))
    }

    #[inline]
    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(StructVariantSerializer {
            variant,
            map: BTreeMap::new(),
        })
    }

    fn collect_str<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + fmt::Display,
    {
        Ok(Any(Inner::String(value.to_string())))
    }
}

pub(crate) struct SeqSerializer(Vec<Any>);

impl SerializeSeq for SeqSerializer {
    type Ok = Any;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let value = Any::new(value)?;
        self.0.push(value);
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Any(Inner::Seq(self.0)))
    }
}

impl SerializeTuple for SeqSerializer {
    type Ok = Any;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        SerializeSeq::serialize_element(self, value)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        SerializeSeq::end(self)
    }
}

impl SerializeTupleStruct for SeqSerializer {
    type Ok = Any;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        SerializeSeq::serialize_element(self, value)
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        SerializeSeq::end(self)
    }
}

pub(crate) struct TupleVariantSerializer {
    variant: &'static str,
    vec: Vec<Any>,
}

impl SerializeTupleVariant for TupleVariantSerializer {
    type Ok = Any;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let value = Any::new(value)?;
        self.vec.push(value);
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        let mut out = BTreeMap::new();
        out.insert(
            Any(Inner::String(self.variant.to_string())),
            Any(Inner::Seq(self.vec)),
        );
        Ok(Any(Inner::Map(out)))
    }
}

pub(crate) struct MapSerializer {
    map: BTreeMap<Any, Any>,
    key: Option<Any>,
}

impl SerializeMap for MapSerializer {
    type Ok = Any;
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let key = key.serialize(AnySerializer)?;
        self.key = Some(key);
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let key = match self.key.take() {
            Some(key) => key,
            None => return Err(Error::custom("key missing")),
        };

        let value = value.serialize(AnySerializer)?;
        self.map.insert(key, value);

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Any(Inner::Map(self.map)))
    }
}

impl SerializeStruct for MapSerializer {
    type Ok = Any;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        SerializeMap::serialize_entry(self, key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        SerializeMap::end(self)
    }
}

pub(crate) struct StructVariantSerializer {
    variant: &'static str,
    map: BTreeMap<Any, Any>,
}

impl SerializeStructVariant for StructVariantSerializer {
    type Ok = Any;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let key = key.serialize(AnySerializer)?;
        let value = value.serialize(AnySerializer)?;
        self.map.insert(key, value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let mut out = BTreeMap::new();
        out.insert(
            Any(Inner::String(self.variant.to_string())),
            Any(Inner::Map(self.map)),
        );
        Ok(Any(Inner::Map(out)))
    }
}
