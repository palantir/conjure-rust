use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[conjure_object::private::staged_builder::staged_builder]
#[builder(crate = conjure_object::private::staged_builder, update, inline)]
pub struct EnumFieldExample {
    #[builder()]
    enum_: super::EnumExample,
}
impl EnumFieldExample {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(enum_: super::EnumExample) -> Self {
        Self::builder().enum_(enum_).build()
    }
    #[inline]
    pub fn enum_(&self) -> &super::EnumExample {
        &self.enum_
    }
}
impl ser::Serialize for EnumFieldExample {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let size = 1usize;
        let mut s = s.serialize_struct("EnumFieldExample", size)?;
        s.serialize_field("enum", &self.enum_)?;
        s.end()
    }
}
impl<'de> de::Deserialize<'de> for EnumFieldExample {
    fn deserialize<D>(d: D) -> Result<EnumFieldExample, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct("EnumFieldExample", &["enum"], Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = EnumFieldExample;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<EnumFieldExample, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut enum_ = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::Enum => enum_ = Some(map_.next_value()?),
                Field_::Unknown_ => {
                    map_.next_value::<de::IgnoredAny>()?;
                }
            }
        }
        let enum_ = match enum_ {
            Some(v) => v,
            None => return Err(de::Error::missing_field("enum")),
        };
        Ok(EnumFieldExample { enum_ })
    }
}
enum Field_ {
    Enum,
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
            "enum" => Field_::Enum,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
