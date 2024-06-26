use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[conjure_object::private::staged_builder::staged_builder]
#[builder(crate = conjure_object::private::staged_builder, update, inline)]
pub struct HeaderParameterType {
    param_id: super::ParameterId,
}
impl HeaderParameterType {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(param_id: super::ParameterId) -> Self {
        Self::builder().param_id(param_id).build()
    }
    #[inline]
    pub fn param_id(&self) -> &super::ParameterId {
        &self.param_id
    }
}
impl ser::Serialize for HeaderParameterType {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let size = 1usize;
        let mut s = s.serialize_struct("HeaderParameterType", size)?;
        s.serialize_field("paramId", &self.param_id)?;
        s.end()
    }
}
impl<'de> de::Deserialize<'de> for HeaderParameterType {
    fn deserialize<D>(d: D) -> Result<HeaderParameterType, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct("HeaderParameterType", &["paramId"], Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = HeaderParameterType;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<HeaderParameterType, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut param_id = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::ParamId => param_id = Some(map_.next_value()?),
                Field_::Unknown_ => {
                    map_.next_value::<de::IgnoredAny>()?;
                }
            }
        }
        let param_id = match param_id {
            Some(v) => v,
            None => return Err(de::Error::missing_field("paramId")),
        };
        Ok(HeaderParameterType { param_id })
    }
}
enum Field_ {
    ParamId,
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
            "paramId" => Field_::ParamId,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
