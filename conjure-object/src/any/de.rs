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
use ordered_float::NotNan;
use serde::de::{
    DeserializeSeed, EnumAccess, Error as _, IntoDeserializer, MapAccess, SeqAccess, Unexpected,
    VariantAccess, Visitor,
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

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v >= 0 {
            Ok(Any(Inner::PositiveInt(v as u64)))
        } else {
            Ok(Any(Inner::NegativeInt(v)))
        }
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Any(Inner::PositiveInt(v)))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v.is_nan() {
            Ok(Any(Inner::String("NaN".to_string())))
        } else if v == f64::INFINITY {
            Ok(Any(Inner::String("Infinity".to_string())))
        } else if v == f64::NEG_INFINITY {
            Ok(Any(Inner::String("-Infinity".to_string())))
        } else {
            Ok(Any(Inner::Float(NotNan::new(v).unwrap())))
        }
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

        Ok(Any(Inner::Array(out)))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut out = BTreeMap::new();

        while let Some((key, value)) = map.next_entry()? {
            out.insert(key, value);
        }

        Ok(Any(Inner::Object(out)))
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
            Inner::Float(v) => visitor.visit_f64(*v),
            Inner::PositiveInt(v) => visitor.visit_u64(v),
            Inner::NegativeInt(v) => visitor.visit_i64(v),
            Inner::String(v) => visitor.visit_string(v),
            Inner::Array(v) => visitor.visit_seq(SeqDeserializer(v.into_iter())),
            Inner::Object(v) => visitor.visit_map(MapDeserializer {
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
            Inner::Object(value) => {
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
            Inner::String(variant) => (variant, None),
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
    it: btree_map::IntoIter<String, Any>,
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

macro_rules! deserialize_integer_key {
    ($method:ident => $visit:ident) => {
        fn $method<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            match self.0.parse() {
                Ok(v) => visitor.$visit(v),
                Err(_) => visitor.visit_string(self.0),
            }
        }
    };
}

struct KeyDeserializer(String);

impl<'de> Deserializer<'de> for KeyDeserializer {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_string(self.0)
    }

    deserialize_integer_key!(deserialize_bool => visit_bool);
    deserialize_integer_key!(deserialize_i8 => visit_i8);
    deserialize_integer_key!(deserialize_i16 => visit_i16);
    deserialize_integer_key!(deserialize_i32 => visit_i32);
    deserialize_integer_key!(deserialize_i64 => visit_i64);
    deserialize_integer_key!(deserialize_u8 => visit_u8);
    deserialize_integer_key!(deserialize_u16 => visit_u16);
    deserialize_integer_key!(deserialize_u32 => visit_u32);
    deserialize_integer_key!(deserialize_u64 => visit_u64);

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match &*self.0 {
            "NaN" => visitor.visit_f32(f32::NAN),
            "Infinity" => visitor.visit_f32(f32::INFINITY),
            "-Infinity" => visitor.visit_f32(f32::NEG_INFINITY),
            _ => match self.0.parse() {
                Ok(v) => visitor.visit_f32(v),
                Err(_) => visitor.visit_string(self.0),
            },
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match &*self.0 {
            "NaN" => visitor.visit_f64(f64::NAN),
            "Infinity" => visitor.visit_f64(f64::INFINITY),
            "-Infinity" => visitor.visit_f64(f64::NEG_INFINITY),
            _ => match self.0.parse() {
                Ok(v) => visitor.visit_f64(v),
                Err(_) => visitor.visit_string(self.0),
            },
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match base64::decode(&self.0) {
            Ok(buf) => visitor.visit_byte_buf(buf),
            Err(_) => Err(Error::invalid_value(
                Unexpected::Str(&self.0),
                &"base64 bytes",
            )),
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
        visitor.visit_some(self)
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

    forward_to_deserialize_any! {
        char str string unit unit_struct seq tuple tuple_struct map struct identifier ignored_any
    }
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
    variant: String,
    value: Option<Any>,
}

impl<'de> EnumAccess<'de> for EnumDeserializer {
    type Error = Error;
    type Variant = VariantDeserializer;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let variant = seed.deserialize(self.variant.into_deserializer())?;
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
            Some(Any(Inner::Array(value))) => visitor.visit_seq(SeqDeserializer(value.into_iter())),
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
            Some(Any(Inner::Object(value))) => visitor.visit_map(MapDeserializer {
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
