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
use serde::de::{
    DeserializeSeed, EnumAccess, Error as _, MapAccess, SeqAccess, Unexpected, VariantAccess,
    Visitor,
};
use serde::{de, forward_to_deserialize_any, Deserialize, Deserializer};
use std::collections::{btree_map, BTreeMap};
use std::f32;
use std::f64;
use std::fmt;
use std::vec;

impl<'de> Deserialize<'de> for Any {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(AnyVisitor)
    }
}

struct AnyVisitor;

impl<'de> Visitor<'de> for AnyVisitor {
    type Value = Any;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("any JSON value")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Any(Inner::Bool(v)))
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Any(Inner::I8(v)))
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Any(Inner::I16(v)))
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Any(Inner::I32(v)))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Any(Inner::I64(v)))
    }

    fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Any(Inner::I128(v)))
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Any(Inner::U8(v)))
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Any(Inner::U16(v)))
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Any(Inner::U32(v)))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Any(Inner::U64(v)))
    }

    fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Any(Inner::U128(v)))
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Any(Inner::F32(OrderedFloat(v))))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Any(Inner::F64(OrderedFloat(v))))
    }

    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Any(Inner::Char(v)))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Any(Inner::String(v.to_string())))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Any(Inner::String(v)))
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Any(Inner::Bytes(v.to_vec())))
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Any(Inner::Bytes(v)))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Any(Inner::Null))
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        Any::deserialize(deserializer)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Any(Inner::Null))
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut out = vec![];

        while let Some(value) = seq.next_element()? {
            out.push(value);
        }

        Ok(Any(Inner::Seq(out)))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut out = BTreeMap::new();

        while let Some((key, value)) = map.next_entry()? {
            out.insert(key, value);
        }

        Ok(Any(Inner::Map(out)))
    }
}

impl<'de> Deserializer<'de> for Any {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Inner::Null => visitor.visit_unit(),
            Inner::Bool(v) => visitor.visit_bool(v),
            Inner::I8(v) => visitor.visit_i8(v),
            Inner::I16(v) => visitor.visit_i16(v),
            Inner::I32(v) => visitor.visit_i32(v),
            Inner::I64(v) => visitor.visit_i64(v),
            Inner::I128(v) => visitor.visit_i128(v),
            Inner::U8(v) => visitor.visit_u8(v),
            Inner::U16(v) => visitor.visit_u16(v),
            Inner::U32(v) => visitor.visit_u32(v),
            Inner::U64(v) => visitor.visit_u64(v),
            Inner::U128(v) => visitor.visit_u128(v),
            Inner::F32(v) => visitor.visit_f32(v.0),
            Inner::F64(v) => visitor.visit_f64(v.0),
            Inner::Char(v) => visitor.visit_char(v),
            Inner::String(v) => visitor.visit_string(v),
            Inner::Bytes(v) => visitor.visit_byte_buf(v),
            Inner::Seq(v) => visitor.visit_seq(SeqDeserializer(v.into_iter())),
            Inner::Map(v) => visitor.visit_map(MapDeserializer {
                it: v.into_iter(),
                value: None,
            }),
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match &self.0 {
            Inner::String(v) if v == "NaN" => visitor.visit_f32(f32::NAN),
            Inner::String(v) if v == "Infinity" => visitor.visit_f32(f32::INFINITY),
            Inner::String(v) if v == "-Infinity" => visitor.visit_f32(f32::NEG_INFINITY),
            _ => self.deserialize_any(visitor),
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match &self.0 {
            Inner::String(v) if v == "NaN" => visitor.visit_f64(f64::NAN),
            Inner::String(v) if v == "Infinity" => visitor.visit_f64(f64::INFINITY),
            Inner::String(v) if v == "-Infinity" => visitor.visit_f64(f64::NEG_INFINITY),
            _ => self.deserialize_any(visitor),
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match &self.0 {
            Inner::String(v) => match base64::decode(v) {
                Ok(buf) => visitor.visit_byte_buf(buf),
                Err(_) => self.deserialize_any(visitor),
            },
            _ => self.deserialize_any(visitor),
        }
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Inner::Null => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_enum<V>(
        self,
        _: &'static str,
        _: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let (variant, value) = match self.0 {
            Inner::Map(value) => {
                let mut iter = value.into_iter();
                let (variant, value) = match iter.next() {
                    Some(v) => v,
                    None => {
                        return Err(Error::invalid_value(
                            Unexpected::Map,
                            &"map with a single key",
                        ))
                    }
                };
                if iter.next().is_some() {
                    return Err(Error::invalid_value(
                        Unexpected::Map,
                        &"map with a single key",
                    ));
                }
                (variant, Some(value))
            }
            Inner::String(variant) => (Any(Inner::String(variant)), None),
            _ => return self.deserialize_any(visitor),
        };

        visitor.visit_enum(EnumDeserializer { variant, value })
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 char str string unit unit_struct newtype_struct seq tuple tuple_struct map
        struct identifier ignored_any
    }
}

struct SeqDeserializer(vec::IntoIter<Any>);

impl<'de> SeqAccess<'de> for SeqDeserializer {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.0.next() {
            Some(value) => seed.deserialize(value).map(Some),
            None => Ok(None),
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.0.len())
    }
}

struct MapDeserializer {
    it: btree_map::IntoIter<Any, Any>,
    value: Option<Any>,
}

impl<'de> MapAccess<'de> for MapDeserializer {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        let (key, value) = match self.it.next() {
            Some((key, value)) => (key, value),
            None => return Ok(None),
        };

        self.value = Some(value);
        seed.deserialize(KeyDeserializer(key)).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        match self.value.take() {
            Some(value) => seed.deserialize(value),
            None => Err(Error::custom("value is missing")),
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.it.len())
    }
}

macro_rules! deserialize_parse {
    ($method:ident => $visit:ident) => {
        fn $method<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            if let Inner::String(s) = &(self.0).0 {
                if let Ok(v) = s.parse() {
                    return visitor.$visit(v);
                }
            }

            self.0.$method(visitor)
        }
    };
}

macro_rules! deserialize_delegate {
    ($method:ident) => {
        fn $method<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.0.$method(visitor)
        }
    };
}

struct KeyDeserializer(Any);

impl<'de> Deserializer<'de> for KeyDeserializer {
    type Error = Error;

    deserialize_delegate!(deserialize_any);

    deserialize_parse!(deserialize_bool => visit_bool);
    deserialize_parse!(deserialize_i8 => visit_i8);
    deserialize_parse!(deserialize_i16 => visit_i16);
    deserialize_parse!(deserialize_i32 => visit_i32);
    deserialize_parse!(deserialize_i64 => visit_i64);
    deserialize_parse!(deserialize_i128 => visit_i128);
    deserialize_parse!(deserialize_u8 => visit_u8);
    deserialize_parse!(deserialize_u16 => visit_u16);
    deserialize_parse!(deserialize_u32 => visit_u32);
    deserialize_parse!(deserialize_u64 => visit_u64);
    deserialize_parse!(deserialize_u128 => visit_u128);
    deserialize_parse!(deserialize_f32 => visit_f32);
    deserialize_parse!(deserialize_f64 => visit_f64);

    deserialize_delegate!(deserialize_char);
    deserialize_delegate!(deserialize_str);
    deserialize_delegate!(deserialize_string);
    deserialize_delegate!(deserialize_bytes);
    deserialize_delegate!(deserialize_byte_buf);

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match &(self.0).0 {
            Inner::Null => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    deserialize_delegate!(deserialize_unit);

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.deserialize_unit_struct(name, visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    deserialize_delegate!(deserialize_seq);

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.deserialize_tuple(len, visitor)
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
        self.0.deserialize_tuple_struct(name, len, visitor)
    }

    deserialize_delegate!(deserialize_map);

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.deserialize_struct(name, fields, visitor)
    }

    fn deserialize_enum<V>(
        self,
        _: &'static str,
        _: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_enum(self)
    }

    deserialize_delegate!(deserialize_identifier);
    deserialize_delegate!(deserialize_ignored_any);
}

impl<'de> EnumAccess<'de> for KeyDeserializer {
    type Error = Error;
    type Variant = UnitVariantDeserializer;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let value = seed.deserialize(self)?;
        Ok((value, UnitVariantDeserializer))
    }
}

struct UnitVariantDeserializer;

impl<'de> VariantAccess<'de> for UnitVariantDeserializer {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(
        self,
        _: T,
    ) -> Result<<T as DeserializeSeed<'de>>::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        Err(Error::invalid_value(
            Unexpected::UnitVariant,
            &"newtype variant",
        ))
    }

    fn tuple_variant<V>(self, _: usize, _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::invalid_value(
            Unexpected::UnitVariant,
            &"tuple variant",
        ))
    }

    fn struct_variant<V>(self, _: &'static [&'static str], _: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::invalid_value(
            Unexpected::UnitVariant,
            &"struct variant",
        ))
    }
}

struct EnumDeserializer {
    variant: Any,
    value: Option<Any>,
}

impl<'de> EnumAccess<'de> for EnumDeserializer {
    type Error = Error;
    type Variant = VariantDeserializer;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let variant = seed.deserialize(self.variant)?;
        Ok((variant, VariantDeserializer(self.value)))
    }
}

struct VariantDeserializer(Option<Any>);

impl<'de> VariantAccess<'de> for VariantDeserializer {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        match self.0 {
            Some(value) => Deserialize::deserialize(value),
            None => Ok(()),
        }
    }

    fn newtype_variant_seed<T>(
        self,
        seed: T,
    ) -> Result<<T as DeserializeSeed<'de>>::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.0 {
            Some(value) => seed.deserialize(value),
            None => Err(Error::invalid_value(
                Unexpected::UnitVariant,
                &"newtype variant",
            )),
        }
    }

    fn tuple_variant<V>(
        self,
        _: usize,
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Some(Any(Inner::Seq(value))) => visitor.visit_seq(SeqDeserializer(value.into_iter())),
            Some(v) => Err(Error::invalid_value(v.unexpected(), &"tuple variant")),
            None => Err(Error::invalid_value(
                Unexpected::UnitVariant,
                &"tuple variant",
            )),
        }
    }

    fn struct_variant<V>(
        self,
        _: &'static [&'static str],
        visitor: V,
    ) -> Result<<V as Visitor<'de>>::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Some(Any(Inner::Map(value))) => visitor.visit_map(MapDeserializer {
                it: value.into_iter(),
                value: None,
            }),
            Some(v) => Err(Error::invalid_value(v.unexpected(), &"struct variant")),
            None => Err(Error::invalid_value(
                Unexpected::UnitVariant,
                &"struct variant",
            )),
        }
    }
}
