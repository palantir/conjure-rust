use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
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
    pub fn builder() -> Builder {
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
///A builder for the `MapType` type.
#[derive(Debug, Clone, Default)]
pub struct Builder {
    key_type: Option<Box<super::Type>>,
    value_type: Option<Box<super::Type>>,
}
impl Builder {
    ///
    /// Required.
    #[inline]
    pub fn key_type(&mut self, key_type: super::Type) -> &mut Self {
        self.key_type = Some(Box::new(key_type));
        self
    }
    ///
    /// Required.
    #[inline]
    pub fn value_type(&mut self, value_type: super::Type) -> &mut Self {
        self.value_type = Some(Box::new(value_type));
        self
    }
    /// Constructs a new instance of the type.
    ///
    /// # Panics
    ///
    /// Panics if a required field was not set.
    #[inline]
    pub fn build(&self) -> MapType {
        MapType {
            key_type: self.key_type.clone().expect("field key_type was not set"),
            value_type: self.value_type.clone().expect("field value_type was not set"),
        }
    }
}
impl From<MapType> for Builder {
    #[inline]
    fn from(_v: MapType) -> Builder {
        Builder {
            key_type: Some(_v.key_type),
            value_type: Some(_v.value_type),
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
