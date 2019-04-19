use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use conjure_object::serde::{de, ser};
use std::fmt;
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct OptionalType {
    item_type: Box<super::Type>,
}
impl OptionalType {
    #[doc = r" Constructs a new instance of the type."]
    #[inline]
    pub fn new(item_type: super::Type) -> OptionalType {
        OptionalType {
            item_type: Box::new(item_type),
        }
    }
    #[doc = r" Returns a new builder."]
    #[inline]
    pub fn builder() -> Builder {
        Default::default()
    }
    #[inline]
    pub fn item_type(&self) -> &super::Type {
        &*self.item_type
    }
}
#[doc = "A builder for the `OptionalType` type."]
#[derive(Debug, Clone, Default)]
pub struct Builder {
    item_type: Option<Box<super::Type>>,
}
impl Builder {
    #[doc = r""]
    #[doc = r" Required."]
    #[inline]
    pub fn item_type(&mut self, item_type: super::Type) -> &mut Self {
        self.item_type = Some(Box::new(item_type));
        self
    }
    #[doc = r" Constructs a new instance of the type."]
    #[doc = r""]
    #[doc = r" # Panics"]
    #[doc = r""]
    #[doc = r" Panics if a required field was not set."]
    #[inline]
    pub fn build(&self) -> OptionalType {
        OptionalType {
            item_type: self.item_type.clone().expect("field item_type was not set"),
        }
    }
}
impl From<OptionalType> for Builder {
    #[inline]
    fn from(_v: OptionalType) -> Builder {
        Builder {
            item_type: Some(_v.item_type),
        }
    }
}
impl ser::Serialize for OptionalType {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let size = 1usize;
        let mut s = s.serialize_struct("OptionalType", size)?;
        s.serialize_field("itemType", &self.item_type)?;
        s.end()
    }
}
impl<'de> de::Deserialize<'de> for OptionalType {
    fn deserialize<D>(d: D) -> Result<OptionalType, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct("OptionalType", &["itemType"], Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = OptionalType;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<OptionalType, A::Error>
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
        Ok(OptionalType { item_type })
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
