use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[conjure_object::private::staged_builder::staged_builder]
#[builder(crate = conjure_object::private::staged_builder, update, inline)]
pub struct MapType {
    #[builder(custom(type = super::Type, convert = Box::new))]
    key_type: Box<super::Type>,
    #[builder(custom(type = super::Type, convert = Box::new))]
    value_type: Box<super::Type>,
}
impl MapType {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(key_type: super::Type, value_type: super::Type) -> Self {
        Self::builder().key_type(key_type).value_type(value_type).build()
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
