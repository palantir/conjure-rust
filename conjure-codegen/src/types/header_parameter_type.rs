use conjure_types::serde::ser::SerializeMap as SerializeMap_;
use conjure_types::serde::{de, ser};
use std::fmt;
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct HeaderParameterType {
    param_id: super::ParameterId,
}
impl HeaderParameterType {
    #[doc = r" Returns a new builder."]
    #[inline]
    pub fn builder() -> Builder {
        Default::default()
    }
    #[inline]
    pub fn param_id(&self) -> &super::ParameterId {
        &self.param_id
    }
}
#[derive(Debug, Clone, Default)]
pub struct Builder {
    param_id: Option<super::ParameterId>,
}
impl Builder {
    #[inline]
    pub fn param_id(&mut self, param_id: super::ParameterId) -> &mut Self {
        self.param_id = Some(param_id);
        self
    }
    #[doc = r" Constructs a new instance of the type."]
    #[doc = r""]
    #[doc = r" # Panics"]
    #[doc = r""]
    #[doc = r" Panics if a required field was not set."]
    #[inline]
    pub fn build(&self) -> HeaderParameterType {
        HeaderParameterType {
            param_id: self.param_id.clone().expect("field param_id was not set"),
        }
    }
}
impl From<HeaderParameterType> for Builder {
    #[inline]
    fn from(_v: HeaderParameterType) -> Builder {
        Builder {
            param_id: Some(_v.param_id),
        }
    }
}
impl ser::Serialize for HeaderParameterType {
    fn serialize<S_>(&self, s: S_) -> Result<S_::Ok, S_::Error>
    where
        S_: ser::Serializer,
    {
        let size = 1usize;
        let mut map = s.serialize_map(Some(size))?;
        map.serialize_entry(&"paramId", &self.param_id)?;
        map.end()
    }
}
impl<'de> de::Deserialize<'de> for HeaderParameterType {
    fn deserialize<D_>(d: D_) -> Result<HeaderParameterType, D_::Error>
    where
        D_: de::Deserializer<'de>,
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
    fn visit_map<A_>(self, mut map_: A_) -> Result<HeaderParameterType, A_::Error>
    where
        A_: de::MapAccess<'de>,
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
    fn deserialize<D_>(d: D_) -> Result<Field_, D_::Error>
    where
        D_: de::Deserializer<'de>,
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
    fn visit_str<E_>(self, value: &str) -> Result<Field_, E_>
    where
        E_: de::Error,
    {
        let v = match value {
            "paramId" => Field_::ParamId,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
