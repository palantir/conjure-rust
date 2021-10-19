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

pub trait WrapVisitor<'de> {
    fn wrap_visitor<D, V>(self, delegate: D, visitor: V) -> Result<V::Value, D::Error>
    where
        D: Delegate<'de>,
        V: Visitor<'de>;
}

pub trait Delegate<'de> {
    type Error;

    fn delegate<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>;
}

pub struct WrappingDeserializer<W, D> {
    wrapper: W,
    inner: D,
}

impl<W, D> WrappingDeserializer<W, D> {
    pub fn new(wrapper: W, inner: D) -> Self {
        WrappingDeserializer { wrapper, inner }
    }
}

macro_rules! delegator {
    ($delegator:ident<$t:ident> = $e:expr) => {
        struct $delegator<$t>($t);

        impl<'de, $t> Delegate<'de> for $delegator<T>
        where
            $t: Deserializer<'de>,
        {
            type Error = T::Error;

            fn delegate<V>(self, visitor: V) -> Result<V::Value, Self::Error>
            where
                V: Visitor<'de>,
            {
                $e(self.0, visitor)
            }
        }
    };
}

macro_rules! delegate {
    ($($method:ident,)*) => {
        $(
            fn $method<V>(self, visitor: V) -> Result<V::Value, Self::Error>
            where
                V: Visitor<'de>,
            {
                delegator!(Delegator<T> = T::$method);
                self.wrapper.wrap_visitor(Delegator(self.inner), visitor)
            }
        )*
    }
}

impl<'de, W, D> Deserializer<'de> for WrappingDeserializer<W, D>
where
    W: WrapVisitor<'de>,
    D: Deserializer<'de>,
{
    type Error = D::Error;

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
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        struct Delegator<T> {
            name: &'static str,
            inner: T,
        }

        impl<'de, T> Delegate<'de> for Delegator<T>
        where
            T: Deserializer<'de>,
        {
            type Error = T::Error;

            fn delegate<V>(self, visitor: V) -> Result<V::Value, Self::Error>
            where
                V: Visitor<'de>,
            {
                self.inner.deserialize_unit_struct(self.name, visitor)
            }
        }

        self.wrapper.wrap_visitor(
            Delegator {
                name,
                inner: self.inner,
            },
            visitor,
        )
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        struct Delegator<T> {
            name: &'static str,
            inner: T,
        }

        impl<'de, T> Delegate<'de> for Delegator<T>
        where
            T: Deserializer<'de>,
        {
            type Error = T::Error;

            fn delegate<V>(self, visitor: V) -> Result<V::Value, Self::Error>
            where
                V: Visitor<'de>,
            {
                self.inner.deserialize_newtype_struct(self.name, visitor)
            }
        }

        self.wrapper.wrap_visitor(
            Delegator {
                name,
                inner: self.inner,
            },
            visitor,
        )
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        struct Delegator<T> {
            len: usize,
            inner: T,
        }

        impl<'de, T> Delegate<'de> for Delegator<T>
        where
            T: Deserializer<'de>,
        {
            type Error = T::Error;

            fn delegate<V>(self, visitor: V) -> Result<V::Value, Self::Error>
            where
                V: Visitor<'de>,
            {
                self.inner.deserialize_tuple(self.len, visitor)
            }
        }

        self.wrapper.wrap_visitor(
            Delegator {
                len,
                inner: self.inner,
            },
            visitor,
        )
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
        struct Delegator<T> {
            name: &'static str,
            len: usize,
            inner: T,
        }

        impl<'de, T> Delegate<'de> for Delegator<T>
        where
            T: Deserializer<'de>,
        {
            type Error = T::Error;

            fn delegate<V>(self, visitor: V) -> Result<V::Value, Self::Error>
            where
                V: Visitor<'de>,
            {
                self.inner
                    .deserialize_tuple_struct(self.name, self.len, visitor)
            }
        }

        self.wrapper.wrap_visitor(
            Delegator {
                name,
                len,
                inner: self.inner,
            },
            visitor,
        )
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
        struct Delegator<T> {
            name: &'static str,
            fields: &'static [&'static str],
            inner: T,
        }

        impl<'de, T> Delegate<'de> for Delegator<T>
        where
            T: Deserializer<'de>,
        {
            type Error = T::Error;

            fn delegate<V>(self, visitor: V) -> Result<V::Value, Self::Error>
            where
                V: Visitor<'de>,
            {
                self.inner
                    .deserialize_struct(self.name, self.fields, visitor)
            }
        }

        self.wrapper.wrap_visitor(
            Delegator {
                name,
                fields,
                inner: self.inner,
            },
            visitor,
        )
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
        struct Delegator<T> {
            name: &'static str,
            variants: &'static [&'static str],
            inner: T,
        }

        impl<'de, T> Delegate<'de> for Delegator<T>
        where
            T: Deserializer<'de>,
        {
            type Error = T::Error;

            fn delegate<V>(self, visitor: V) -> Result<V::Value, Self::Error>
            where
                V: Visitor<'de>,
            {
                self.inner
                    .deserialize_enum(self.name, self.variants, visitor)
            }
        }

        self.wrapper.wrap_visitor(
            Delegator {
                name,
                variants,
                inner: self.inner,
            },
            visitor,
        )
    }

    fn is_human_readable(&self) -> bool {
        self.inner.is_human_readable()
    }
}
