use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MapType {
    key_type: Box<super::Type>,
    value_type: Box<super::Type>,
}
impl MapType {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(key_type: super::Type, value_type: super::Type) -> MapType {
        MapType {
            key_type: Box::new(key_type),
            value_type: Box::new(value_type),
        }
    }
    /// Returns a new builder.
    #[inline]
    pub fn builder() -> BuilderStage0 {
        Default::default()
    }
    #[inline]
    pub fn key_type(&self) -> &super::Type {
        &*self.key_type
    }
    #[inline]
    pub fn value_type(&self) -> &super::Type {
        &*self.value_type
    }
}
impl Default for BuilderStage0 {
    #[inline]
    fn default() -> Self {
        BuilderStage0 {}
    }
}
impl From<MapType> for BuilderStage2 {
    #[inline]
    fn from(value: MapType) -> Self {
        BuilderStage2 {
            key_type: value.key_type,
            value_type: value.value_type,
        }
    }
}
///The stage 0 builder for the [`MapType`] type
#[derive(Debug, Clone)]
pub struct BuilderStage0 {}
impl BuilderStage0 {
    #[inline]
    pub fn key_type(self, key_type: super::Type) -> BuilderStage1 {
        BuilderStage1 {
            key_type: Box::new(key_type),
        }
    }
}
///The stage 1 builder for the [`MapType`] type
#[derive(Debug, Clone)]
pub struct BuilderStage1 {
    key_type: Box<super::Type>,
}
impl BuilderStage1 {
    #[inline]
    pub fn value_type(self, value_type: super::Type) -> BuilderStage2 {
        BuilderStage2 {
            key_type: self.key_type,
            value_type: Box::new(value_type),
        }
    }
}
///The stage 2 builder for the [`MapType`] type
#[derive(Debug, Clone)]
pub struct BuilderStage2 {
    key_type: Box<super::Type>,
    value_type: Box<super::Type>,
}
impl BuilderStage2 {
    #[inline]
    pub fn key_type(mut self, key_type: super::Type) -> Self {
        self.key_type = Box::new(key_type);
        self
    }
    #[inline]
    pub fn value_type(mut self, value_type: super::Type) -> Self {
        self.value_type = Box::new(value_type);
        self
    }
    /// Consumes the builder, constructing a new instance of the type.
    #[inline]
    pub fn build(self) -> MapType {
        MapType {
            key_type: self.key_type,
            value_type: self.value_type,
        }
    }
}
impl ser::Serialize for MapType {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let size = 2usize;
        let mut s = s.serialize_struct("MapType", size)?;
        s.serialize_field("keyType", &self.key_type)?;
        s.serialize_field("valueType", &self.value_type)?;
        s.end()
    }
}
impl<'de> de::Deserialize<'de> for MapType {
    fn deserialize<D>(d: D) -> Result<MapType, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct("MapType", &["keyType", "valueType"], Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = MapType;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<MapType, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut key_type = None;
        let mut value_type = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::KeyType => key_type = Some(map_.next_value()?),
                Field_::ValueType => value_type = Some(map_.next_value()?),
                Field_::Unknown_ => {
                    map_.next_value::<de::IgnoredAny>()?;
                }
            }
        }
        let key_type = match key_type {
            Some(v) => v,
            None => return Err(de::Error::missing_field("keyType")),
        };
        let value_type = match value_type {
            Some(v) => v,
            None => return Err(de::Error::missing_field("valueType")),
        };
        Ok(MapType { key_type, value_type })
    }
}
enum Field_ {
    KeyType,
    ValueType,
    Unknown_,
}
impl<'de> de::Deserialize<'de> for Field_ {
    fn deserialize<D>(d: D) -> Result<Field_, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_str(FieldVisitor_)
    }
}
struct FieldVisitor_;
impl<'de> de::Visitor<'de> for FieldVisitor_ {
    type Value = Field_;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("string")
    }
    fn visit_str<E>(self, value: &str) -> Result<Field_, E>
    where
        E: de::Error,
    {
        let v = match value {
            "keyType" => Field_::KeyType,
            "valueType" => Field_::ValueType,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
