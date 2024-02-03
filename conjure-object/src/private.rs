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
pub use educe::Educe;
use ordered_float::OrderedFloat;
use serde::de::{self, IntoDeserializer};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::{fmt, mem};

pub trait DoubleOps {
    fn cmp(&self, other: &Self) -> Ordering;

    fn eq(&self, other: &Self) -> bool;

    fn hash<H>(&self, hasher: &mut H)
    where
        H: Hasher;
}

impl DoubleOps for f64 {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        OrderedFloat(*self).cmp(&OrderedFloat(*other))
    }

    #[inline]
    fn eq(&self, other: &Self) -> bool {
        OrderedFloat(*self) == OrderedFloat(*other)
    }

    #[inline]
    fn hash<H>(&self, hasher: &mut H)
    where
        H: Hasher,
    {
        OrderedFloat(*self).hash(hasher)
    }
}

impl<T> DoubleOps for Option<T>
where
    T: DoubleOps,
{
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Some(a), Some(b)) => a.cmp(b),
            (Some(_), None) => Ordering::Greater,
            (None, Some(_)) => Ordering::Less,
            (None, None) => Ordering::Equal,
        }
    }

    #[inline]
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Some(a), Some(b)) => a.eq(b),
            (Some(_), None) | (None, Some(_)) => false,
            (None, None) => true,
        }
    }

    #[inline]
    fn hash<H>(&self, hasher: &mut H)
    where
        H: Hasher,
    {
        mem::discriminant(self).hash(hasher);
        if let Some(v) = self {
            v.hash(hasher);
        }
    }
}

impl<T> DoubleOps for Vec<T>
where
    T: DoubleOps,
{
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        let l = usize::min(self.len(), other.len());

        let lhs = &self[..l];
        let rhs = &other[..l];

        for i in 0..l {
            match lhs[i].cmp(&rhs[i]) {
                Ordering::Equal => {}
                v => return v,
            }
        }

        self.len().cmp(&other.len())
    }

    #[inline]
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        for i in 0..self.len() {
            if !self[i].eq(&other[i]) {
                return false;
            }
        }

        true
    }

    #[inline]
    fn hash<H>(&self, hasher: &mut H)
    where
        H: Hasher,
    {
        self.len().hash(hasher);
        for v in self {
            v.hash(hasher);
        }
    }
}

pub fn valid_enum_variant(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    s.as_bytes()
        .iter()
        .all(|b| matches!(b, b'A'..=b'Z' | b'0'..=b'9' | b'_'))
}

pub enum UnionField_<T> {
    Type,
    Value(T),
}

impl<'de, T> de::Deserialize<'de> for UnionField_<T>
where
    T: de::Deserialize<'de>,
{
    fn deserialize<D>(d: D) -> Result<UnionField_<T>, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_str(UnionFieldVisitor(PhantomData))
    }
}

struct UnionFieldVisitor<T>(PhantomData<T>);

impl<'de, T> de::Visitor<'de> for UnionFieldVisitor<T>
where
    T: de::Deserialize<'de>,
{
    type Value = UnionField_<T>;

    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("string")
    }

    fn visit_str<E>(self, value: &str) -> Result<UnionField_<T>, E>
    where
        E: de::Error,
    {
        match value {
            "type" => Ok(UnionField_::Type),
            value => T::deserialize(value.into_deserializer()).map(UnionField_::Value),
        }
    }
}

pub struct UnionTypeField_;

impl<'de> de::Deserialize<'de> for UnionTypeField_ {
    fn deserialize<D>(d: D) -> Result<UnionTypeField_, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_str(UnionTypeFieldVisitor)
    }
}

struct UnionTypeFieldVisitor;

impl<'de> de::Visitor<'de> for UnionTypeFieldVisitor {
    type Value = UnionTypeField_;

    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("type field")
    }

    fn visit_str<E>(self, value: &str) -> Result<UnionTypeField_, E>
    where
        E: de::Error,
    {
        match value {
            "type" => Ok(UnionTypeField_),
            value => Err(E::invalid_value(de::Unexpected::Str(value), &self)),
        }
    }
}
