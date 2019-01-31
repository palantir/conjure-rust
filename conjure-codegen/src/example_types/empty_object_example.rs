use conjure_object::serde::ser::SerializeMap as SerializeMap_;
use conjure_object::serde::{de, ser};
use std::fmt;
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Copy)]
pub struct EmptyObjectExample {}
impl EmptyObjectExample {
    #[doc = r" Constructs a new instance of the type."]
    #[inline]
    pub fn new() -> EmptyObjectExample {
        EmptyObjectExample::builder().build()
    }
    #[doc = r" Returns a new builder."]
    #[inline]
    pub fn builder() -> Builder {
        Default::default()
    }
}
#[derive(Debug, Clone, Default)]
pub struct Builder {}
impl Builder {
    #[doc = r" Constructs a new instance of the type."]
    #[doc = r""]
    #[doc = r" # Panics"]
    #[doc = r""]
    #[doc = r" Panics if a required field was not set."]
    #[inline]
    pub fn build(&self) -> EmptyObjectExample {
        EmptyObjectExample {}
    }
}
impl From<EmptyObjectExample> for Builder {
    #[inline]
    fn from(_v: EmptyObjectExample) -> Builder {
        Builder {}
    }
}
impl ser::Serialize for EmptyObjectExample {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let size = 0usize;
        let map = s.serialize_map(Some(size))?;
        map.end()
    }
}
impl<'de> de::Deserialize<'de> for EmptyObjectExample {
    fn deserialize<D>(d: D) -> Result<EmptyObjectExample, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct("EmptyObjectExample", &[], Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = EmptyObjectExample;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<EmptyObjectExample, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::Unknown_ => {
                    map_.next_value::<de::IgnoredAny>()?;
                }
            }
        }
        Ok(EmptyObjectExample {})
    }
}
enum Field_ {
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
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
