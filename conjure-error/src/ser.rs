// Copyright 2019 Palantir Technologies, Inc.
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
use conjure_object::any::{Any, Error};
use serde::de::{self, DeserializeSeed, Deserializer, Visitor};
use serde::ser::{Error as _, Impossible, Serialize, SerializeStruct, Serializer};
use std::fmt;

pub struct ParametersSerializer;

fn unexpected<T>() -> Result<T, Error> {
    Err(Error::custom("expected struct"))
}

macro_rules! unexpected {
    ($($func:ident = $t:ty,)*) => {
        $(
            fn $func(self, _: $t) -> Result<Self::Ok, Self::Error> {
                unexpected()
            }
        )*
    }
}

impl Serializer for ParametersSerializer {
    type Ok = Vec<(String, Any)>;
    type Error = Error;
    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = StructSerializer;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    unexpected! {
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
        unexpected()
    }

    fn serialize_some<T>(self, _: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        unexpected()
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        unexpected()
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> {
        unexpected()
    }

    fn serialize_unit_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        unexpected()
    }

    fn serialize_newtype_struct<T>(self, _: &'static str, _: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        unexpected()
    }

    fn serialize_newtype_variant<T>(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        unexpected()
    }

    fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        unexpected()
    }

    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> {
        unexpected()
    }

    fn serialize_tuple_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        unexpected()
    }

    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        unexpected()
    }

    fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        unexpected()
    }

    #[inline]
    fn serialize_struct(
        self,
        _: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(StructSerializer {
            entries: Vec::with_capacity(len),
        })
    }

    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        unexpected()
    }
}

pub struct StructSerializer {
    entries: Vec<(String, Any)>,
}

impl SerializeStruct for StructSerializer {
    type Ok = Vec<(String, Any)>;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        let key = key.to_string();
        let value = Any::new(value)?;
        self.entries.push((key, value));
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Vec<(String, Any)>, Error> {
        Ok(self.entries)
    }
}

pub struct StringSeed;

impl<'de> DeserializeSeed<'de> for StringSeed {
    type Value = String;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(StringVisitor)
    }
}

struct StringVisitor;

impl Visitor<'_> for StringVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a scalar value")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v.to_string())
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v.to_string())
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v.to_string())
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v.to_string())
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v.to_string())
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v)
    }
}
