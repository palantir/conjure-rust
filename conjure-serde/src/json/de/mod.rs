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
use serde::de;
use std::f32;
use std::f64;
use std::fmt;

pub mod client;
pub mod server;

macro_rules! delegate_visit {
    ($($method:ident = $ty:ty,)*) => {
        $(
            fn $method<E>(self, v: $ty) -> Result<T::Value, E>
            where
                E: de::Error,
            {
                (self.0).$method(v)
            }
        )*
    };
}

macro_rules! float_visitor {
    ($name:ident, $method:ident, $module:ident) => {
        struct $name<T>(T);

        impl<'de, T> de::Visitor<'de> for $name<T>
        where
            T: de::Visitor<'de>,
        {
            type Value = T::Value;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                self.0.expecting(formatter)
            }

            delegate_visit!(
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
            );

            fn visit_str<E>(self, v: &str) -> Result<T::Value, E>
            where
                E: de::Error,
            {
                match v {
                    "NaN" => (self.0).$method($module::NAN),
                    "Infinity" => (self.0).$method($module::INFINITY),
                    "-Infinity" => (self.0).$method($module::NEG_INFINITY),
                    _ => self.0.visit_str(v),
                }
            }
        }
    };
}

float_visitor!(F32Visitor, visit_f32, f32);
float_visitor!(F64Visitor, visit_f64, f64);

struct ByteBufVisitor<T>(T);

impl<'de, T> de::Visitor<'de> for ByteBufVisitor<T>
where
    T: de::Visitor<'de>,
{
    type Value = T::Value;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a base64 string")
    }

    fn visit_str<E>(self, v: &str) -> Result<T::Value, E>
    where
        E: de::Error,
    {
        match base64::decode(v) {
            Ok(v) => self.0.visit_byte_buf(v),
            Err(_) => Err(E::invalid_value(de::Unexpected::Str(v), &self)),
        }
    }
}
