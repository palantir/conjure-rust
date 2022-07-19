use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
pub struct HeaderAuthType {}
impl HeaderAuthType {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new() -> HeaderAuthType {
        HeaderAuthType {}
    }
    /// Returns a new builder.
    #[inline]
    pub fn builder() -> Builder {
        Default::default()
    }
}
///A builder for the `HeaderAuthType` type.
#[derive(Debug, Clone, Default)]
pub struct Builder {}
impl Builder {
    /// Constructs a new instance of the type.
    ///
    /// # Panics
    ///
    /// Panics if a required field was not set.
    #[inline]
    pub fn build(&self) -> HeaderAuthType {
        HeaderAuthType {}
    }
}
impl From<HeaderAuthType> for Builder {
    #[inline]
    fn from(_v: HeaderAuthType) -> Builder {
        Builder {}
    }
}
impl ser::Serialize for HeaderAuthType {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let size = 0usize;
        let s = s.serialize_struct("HeaderAuthType", size)?;
        s.end()
    }
}
impl<'de> de::Deserialize<'de> for HeaderAuthType {
    fn deserialize<D>(d: D) -> Result<HeaderAuthType, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct("HeaderAuthType", &[], Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = HeaderAuthType;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<HeaderAuthType, A::Error>
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
        Ok(HeaderAuthType {})
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
