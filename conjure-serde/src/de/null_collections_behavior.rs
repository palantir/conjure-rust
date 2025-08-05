use crate::de::delegating_visitor::{DelegatingVisitor, Visitor2};
use crate::de::Behavior;
use serde::de;
use std::marker::PhantomData;

pub struct NullCollectionsBehavior<B> {
    _p: PhantomData<B>,
}

impl<B> Behavior for NullCollectionsBehavior<B>
where
    B: Behavior,
{
    type KeyBehavior = NullCollectionsBehavior<B::KeyBehavior>;

    fn deserialize_bool<'de, D, V>(de: D, visitor: V) -> Result<V::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
        V: de::Visitor<'de>,
    {
        B::deserialize_bool(de, visitor)
    }

    fn deserialize_f32<'de, D, V>(de: D, visitor: V) -> Result<V::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
        V: de::Visitor<'de>,
    {
        B::deserialize_f32(de, visitor)
    }

    fn deserialize_f64<'de, D, V>(de: D, visitor: V) -> Result<V::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
        V: de::Visitor<'de>,
    {
        B::deserialize_f64(de, visitor)
    }

    fn deserialize_bytes<'de, D, V>(de: D, visitor: V) -> Result<V::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
        V: de::Visitor<'de>,
    {
        B::deserialize_bytes(de, visitor)
    }

    fn deserialize_byte_buf<'de, D, V>(de: D, visitor: V) -> Result<V::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
        V: de::Visitor<'de>,
    {
        B::deserialize_byte_buf(de, visitor)
    }

    fn deserialize_seq<'de, D, V>(de: D, visitor: V) -> Result<V::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
        V: de::Visitor<'de>,
    {
        de.deserialize_any(DelegatingVisitor::new(EmptySeqVisitor, visitor))
    }

    fn deserialize_map<'de, D, V>(de: D, visitor: V) -> Result<V::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
        V: de::Visitor<'de>,
    {
        de.deserialize_any(DelegatingVisitor::new(EmptyMapVisitor, visitor))
    }

    fn deserialize_struct<'de, D, V>(
        de: D,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
        V: de::Visitor<'de>,
    {
        B::deserialize_struct(de, name, fields, visitor)
    }
}

struct EmptySeqVisitor;

impl<'de, V> Visitor2<'de, V> for EmptySeqVisitor
where
    V: de::Visitor<'de>,
{
    fn visit_unit<E>(self, visitor: V) -> Result<V::Value, E>
    where
        E: de::Error,
    {
        visitor.visit_seq(EmptySeqAccess { _p: PhantomData })
    }
}

struct EmptySeqAccess<E> {
    _p: PhantomData<E>,
}

impl<'de, E> de::SeqAccess<'de> for EmptySeqAccess<E>
where
    E: de::Error,
{
    type Error = E;

    fn next_element_seed<T>(&mut self, _: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        Ok(None)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(0)
    }
}

struct EmptyMapVisitor;

impl<'de, V> Visitor2<'de, V> for EmptyMapVisitor
where
    V: de::Visitor<'de>,
{
    fn visit_unit<E>(self, visitor: V) -> Result<V::Value, E>
    where
        E: de::Error,
    {
        visitor.visit_map(EmptyMapAccess { _p: PhantomData })
    }
}

struct EmptyMapAccess<E> {
    _p: PhantomData<E>,
}

impl<'de, E> de::MapAccess<'de> for EmptyMapAccess<E>
where
    E: de::Error,
{
    type Error = E;

    fn next_key_seed<K>(&mut self, _: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        Ok(None)
    }

    fn next_value_seed<V>(&mut self, _: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        unreachable!()
    }

    fn size_hint(&self) -> Option<usize> {
        Some(0)
    }
}
