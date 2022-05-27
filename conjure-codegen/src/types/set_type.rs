use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct SetType {
    item_type: Box<super::Type>,
}
impl SetType {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(item_type: super::Type) -> SetType {
        SetType {
            item_type: Box::new(item_type),
        }
    }
    /// Returns a new builder.
    #[inline]
    pub fn builder() -> Builder {
        Default::default()
    }
    #[inline]
    pub fn item_type(&self) -> &super::Type {
        &*self.item_type
    }
}
///A builder for the `SetType` type.
#[derive(Debug, Clone, Default)]
pub struct Builder {
    item_type: Option<Box<super::Type>>,
}
impl Builder {
    ///
    /// Required.
    #[inline]
    pub fn item_type(&mut self, item_type: super::Type) -> &mut Self {
        self.item_type = Some(Box::new(item_type));
        self
    }
    /// Constructs a new instance of the type.
    ///
    /// # Panics
    ///
    /// Panics if a required field was not set.
    #[inline]
    pub fn build(&self) -> SetType {
        SetType {
            item_type: self.item_type.clone().expect("field item_type was not set"),
        }
    }
}
impl From<SetType> for Builder {
    #[inline]
    fn from(_v: SetType) -> Builder {
        Builder {
            item_type: Some(_v.item_type),
        }
    }
}
impl ser::Serialize for SetType {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let size = 1usize;
        let mut s = s.serialize_struct("SetType", size)?;
        s.serialize_field("itemType", &self.item_type)?;
        s.end()
    }
}
impl<'de> de::Deserialize<'de> for SetType {
    fn deserialize<D>(d: D) -> Result<SetType, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct("SetType", &["itemType"], Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = SetType;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<SetType, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut item_type = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::ItemType => item_type = Some(map_.next_value()?),
                Field_::Unknown_ => {
                    map_.next_value::<de::IgnoredAny>()?;
                }
            }
        }
        let item_type = match item_type {
            Some(v) => v,
            None => return Err(de::Error::missing_field("itemType")),
        };
        Ok(SetType { item_type })
    }
}
enum Field_ {
    ItemType,
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
            "itemType" => Field_::ItemType,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
