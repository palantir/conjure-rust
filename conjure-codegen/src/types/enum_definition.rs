use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct EnumDefinition {
    type_name: Box<super::TypeName>,
    values: Vec<super::EnumValueDefinition>,
    docs: Option<super::Documentation>,
}
impl EnumDefinition {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new<T>(
        type_name: super::TypeName,
        values: T,
        docs: super::Documentation,
    ) -> EnumDefinition
    where
        T: IntoIterator<Item = super::EnumValueDefinition>,
    {
        EnumDefinition {
            type_name: Box::new(type_name),
            values: values.into_iter().collect(),
            docs: Some(docs),
        }
    }
    /// Returns a new builder.
    #[inline]
    pub fn builder() -> Builder {
        Default::default()
    }
    #[inline]
    pub fn type_name(&self) -> &super::TypeName {
        &*self.type_name
    }
    #[inline]
    pub fn values(&self) -> &[super::EnumValueDefinition] {
        &*self.values
    }
    #[inline]
    pub fn docs(&self) -> Option<&super::Documentation> {
        self.docs.as_ref().map(|o| &*o)
    }
}
///A builder for the `EnumDefinition` type.
#[derive(Debug, Clone, Default)]
pub struct Builder {
    type_name: Option<Box<super::TypeName>>,
    values: Vec<super::EnumValueDefinition>,
    docs: Option<super::Documentation>,
}
impl Builder {
    ///
    /// Required.
    #[inline]
    pub fn type_name(&mut self, type_name: super::TypeName) -> &mut Self {
        self.type_name = Some(Box::new(type_name));
        self
    }
    #[inline]
    pub fn values<T>(&mut self, values: T) -> &mut Self
    where
        T: IntoIterator<Item = super::EnumValueDefinition>,
    {
        self.values = values.into_iter().collect();
        self
    }
    #[inline]
    pub fn extend_values<T>(&mut self, values: T) -> &mut Self
    where
        T: IntoIterator<Item = super::EnumValueDefinition>,
    {
        self.values.extend(values);
        self
    }
    #[inline]
    pub fn push_values(&mut self, value: super::EnumValueDefinition) -> &mut Self {
        self.values.push(value);
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
    /// Constructs a new instance of the type.
    ///
    /// # Panics
    ///
    /// Panics if a required field was not set.
    #[inline]
    pub fn build(&self) -> EnumDefinition {
        EnumDefinition {
            type_name: self.type_name.clone().expect("field type_name was not set"),
            values: self.values.clone(),
            docs: self.docs.clone(),
        }
    }
}
impl From<EnumDefinition> for Builder {
    #[inline]
    fn from(_v: EnumDefinition) -> Builder {
        Builder {
            type_name: Some(_v.type_name),
            values: _v.values,
            docs: _v.docs,
        }
    }
}
impl ser::Serialize for EnumDefinition {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut size = 1usize;
        let skip_values = self.values.is_empty();
        if !skip_values {
            size += 1;
        }
        let skip_docs = self.docs.is_none();
        if !skip_docs {
            size += 1;
        }
        let mut s = s.serialize_struct("EnumDefinition", size)?;
        s.serialize_field("typeName", &self.type_name)?;
        if skip_values {
            s.skip_field("values")?;
        } else {
            s.serialize_field("values", &self.values)?;
        }
        if skip_docs {
            s.skip_field("docs")?;
        } else {
            s.serialize_field("docs", &self.docs)?;
        }
        s.end()
    }
}
impl<'de> de::Deserialize<'de> for EnumDefinition {
    fn deserialize<D>(d: D) -> Result<EnumDefinition, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct("EnumDefinition", &["typeName", "values", "docs"], Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = EnumDefinition;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<EnumDefinition, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut type_name = None;
        let mut values = None;
        let mut docs = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::TypeName => type_name = Some(map_.next_value()?),
                Field_::Values => values = Some(map_.next_value()?),
                Field_::Docs => docs = Some(map_.next_value()?),
                Field_::Unknown_ => {
                    map_.next_value::<de::IgnoredAny>()?;
                }
            }
        }
        let type_name = match type_name {
            Some(v) => v,
            None => return Err(de::Error::missing_field("typeName")),
        };
        let values = match values {
            Some(v) => v,
            None => Default::default(),
        };
        let docs = match docs {
            Some(v) => v,
            None => Default::default(),
        };
        Ok(EnumDefinition {
            type_name,
            values,
            docs,
        })
    }
}
enum Field_ {
    TypeName,
    Values,
    Docs,
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
            "typeName" => Field_::TypeName,
            "values" => Field_::Values,
            "docs" => Field_::Docs,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
