use conjure_types::serde::ser::SerializeMap as SerializeMap_;
use conjure_types::serde::{de, ser};
use std::fmt;
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct EnumValueDefinition {
    value: String,
    docs: Option<super::Documentation>,
}
impl EnumValueDefinition {
    #[doc = r" Returns a new builder."]
    #[inline]
    pub fn builder() -> Builder {
        Default::default()
    }
    #[inline]
    pub fn value(&self) -> &str {
        &*self.value
    }
    #[inline]
    pub fn docs(&self) -> Option<&super::Documentation> {
        self.docs.as_ref().map(|o| &*o)
    }
}
#[derive(Debug, Clone, Default)]
pub struct Builder {
    value: Option<String>,
    docs: Option<super::Documentation>,
}
impl Builder {
    pub fn value<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.value = Some(value.into());
        self
    }
    pub fn docs<T>(&mut self, docs: T) -> &mut Self
    where
        T: Into<Option<super::Documentation>>,
    {
        self.docs = docs.into();
        self
    }
    #[doc = r" Constructs a new instance of the type."]
    #[doc = r""]
    #[doc = r" # Panics"]
    #[doc = r""]
    #[doc = r" Panics if a required field was not set."]
    #[inline]
    pub fn build(&self) -> EnumValueDefinition {
        EnumValueDefinition {
            value: self.value.clone().expect("field value was not set"),
            docs: self.docs.clone(),
        }
    }
}
impl From<EnumValueDefinition> for Builder {
    #[inline]
    fn from(_v: EnumValueDefinition) -> Builder {
        Builder {
            value: Some(_v.value),
            docs: _v.docs,
        }
    }
}
impl ser::Serialize for EnumValueDefinition {
    fn serialize<S_>(&self, s: S_) -> Result<S_::Ok, S_::Error>
    where
        S_: ser::Serializer,
    {
        let mut size = 1usize;
        let skip_docs = self.docs.is_none();
        if !skip_docs {
            size += 1;
        }
        let mut map = s.serialize_map(Some(size))?;
        map.serialize_entry(&"value", &self.value)?;
        if !skip_docs {
            map.serialize_entry(&"docs", &self.docs)?;
        }
        map.end()
    }
}
impl<'de> de::Deserialize<'de> for EnumValueDefinition {
    fn deserialize<D_>(d: D_) -> Result<EnumValueDefinition, D_::Error>
    where
        D_: de::Deserializer<'de>,
    {
        d.deserialize_struct("EnumValueDefinition", &["value", "docs"], Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = EnumValueDefinition;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A_>(self, mut map_: A_) -> Result<EnumValueDefinition, A_::Error>
    where
        A_: de::MapAccess<'de>,
    {
        let mut value = None;
        let mut docs = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::Value => value = Some(map_.next_value()?),
                Field_::Docs => docs = Some(map_.next_value()?),
                Field_::Unknown_ => {
                    map_.next_value::<de::IgnoredAny>()?;
                }
            }
        }
        let value = match value {
            Some(v) => v,
            None => return Err(de::Error::missing_field("value")),
        };
        let docs = match docs {
            Some(v) => v,
            None => Default::default(),
        };
        Ok(EnumValueDefinition { value, docs })
    }
}
enum Field_ {
    Value,
    Docs,
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
            "value" => Field_::Value,
            "docs" => Field_::Docs,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
