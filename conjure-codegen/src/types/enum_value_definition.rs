use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EnumValueDefinition {
    value: String,
    docs: Option<super::Documentation>,
    deprecated: Option<super::Documentation>,
}
impl EnumValueDefinition {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new<T>(
        value: T,
        docs: super::Documentation,
        deprecated: super::Documentation,
    ) -> EnumValueDefinition
    where
        T: Into<String>,
    {
        EnumValueDefinition {
            value: value.into(),
            docs: Some(docs),
            deprecated: Some(deprecated),
        }
    }
    /// Returns a new builder.
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
    #[inline]
    pub fn deprecated(&self) -> Option<&super::Documentation> {
        self.deprecated.as_ref().map(|o| &*o)
    }
}
///A builder for the `EnumValueDefinition` type.
#[derive(Debug, Clone, Default)]
pub struct Builder {
    value: Option<String>,
    docs: Option<super::Documentation>,
    deprecated: Option<super::Documentation>,
}
impl Builder {
    ///
    /// Required.
    #[inline]
    pub fn value<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.value = Some(value.into());
        self
    }
    #[inline]
    pub fn docs<T>(&mut self, docs: T) -> &mut Self
    where
        T: Into<Option<super::Documentation>>,
    {
        self.docs = docs.into();
        self
    }
    #[inline]
    pub fn deprecated<T>(&mut self, deprecated: T) -> &mut Self
    where
        T: Into<Option<super::Documentation>>,
    {
        self.deprecated = deprecated.into();
        self
    }
    /// Constructs a new instance of the type.
    ///
    /// # Panics
    ///
    /// Panics if a required field was not set.
    #[inline]
    pub fn build(&self) -> EnumValueDefinition {
        EnumValueDefinition {
            value: self.value.clone().expect("field value was not set"),
            docs: self.docs.clone(),
            deprecated: self.deprecated.clone(),
        }
    }
}
impl From<EnumValueDefinition> for Builder {
    #[inline]
    fn from(_v: EnumValueDefinition) -> Builder {
        Builder {
            value: Some(_v.value),
            docs: _v.docs,
            deprecated: _v.deprecated,
        }
    }
}
impl ser::Serialize for EnumValueDefinition {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut size = 1usize;
        let skip_docs = self.docs.is_none();
        if !skip_docs {
            size += 1;
        }
        let skip_deprecated = self.deprecated.is_none();
        if !skip_deprecated {
            size += 1;
        }
        let mut s = s.serialize_struct("EnumValueDefinition", size)?;
        s.serialize_field("value", &self.value)?;
        if skip_docs {
            s.skip_field("docs")?;
        } else {
            s.serialize_field("docs", &self.docs)?;
        }
        if skip_deprecated {
            s.skip_field("deprecated")?;
        } else {
            s.serialize_field("deprecated", &self.deprecated)?;
        }
        s.end()
    }
}
impl<'de> de::Deserialize<'de> for EnumValueDefinition {
    fn deserialize<D>(d: D) -> Result<EnumValueDefinition, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct(
            "EnumValueDefinition",
            &["value", "docs", "deprecated"],
            Visitor_,
        )
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = EnumValueDefinition;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<EnumValueDefinition, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut value = None;
        let mut docs = None;
        let mut deprecated = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::Value => value = Some(map_.next_value()?),
                Field_::Docs => docs = Some(map_.next_value()?),
                Field_::Deprecated => deprecated = Some(map_.next_value()?),
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
        let deprecated = match deprecated {
            Some(v) => v,
            None => Default::default(),
        };
        Ok(EnumValueDefinition {
            value,
            docs,
            deprecated,
        })
    }
}
enum Field_ {
    Value,
    Docs,
    Deprecated,
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
            "value" => Field_::Value,
            "docs" => Field_::Docs,
            "deprecated" => Field_::Deprecated,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
