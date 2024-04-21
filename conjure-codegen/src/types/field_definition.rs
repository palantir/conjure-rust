use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FieldDefinition {
    field_name: super::FieldName,
    type_: Box<super::Type>,
    docs: Option<super::Documentation>,
    deprecated: Option<super::Documentation>,
    safety: Option<super::LogSafety>,
}
impl FieldDefinition {
    /// Returns a new builder.
    #[inline]
    pub fn builder() -> BuilderStage0 {
        Default::default()
    }
    #[inline]
    pub fn field_name(&self) -> &super::FieldName {
        &self.field_name
    }
    #[inline]
    pub fn type_(&self) -> &super::Type {
        &*self.type_
    }
    #[inline]
    pub fn docs(&self) -> Option<&super::Documentation> {
        self.docs.as_ref().map(|o| &*o)
    }
    #[inline]
    pub fn deprecated(&self) -> Option<&super::Documentation> {
        self.deprecated.as_ref().map(|o| &*o)
    }
    #[inline]
    pub fn safety(&self) -> Option<&super::LogSafety> {
        self.safety.as_ref().map(|o| &*o)
    }
}
impl Default for BuilderStage0 {
    #[inline]
    fn default() -> Self {
        BuilderStage0 {}
    }
}
impl From<FieldDefinition> for BuilderStage2 {
    #[inline]
    fn from(value: FieldDefinition) -> Self {
        BuilderStage2 {
            field_name: value.field_name,
            type_: value.type_,
            docs: value.docs,
            deprecated: value.deprecated,
            safety: value.safety,
        }
    }
}
///The stage 0 builder for the [`FieldDefinition`] type
#[derive(Debug, Clone)]
pub struct BuilderStage0 {}
impl BuilderStage0 {
    #[inline]
    pub fn field_name(self, field_name: super::FieldName) -> BuilderStage1 {
        BuilderStage1 {
            field_name: field_name,
        }
    }
}
///The stage 1 builder for the [`FieldDefinition`] type
#[derive(Debug, Clone)]
pub struct BuilderStage1 {
    field_name: super::FieldName,
}
impl BuilderStage1 {
    #[inline]
    pub fn type_(self, type_: super::Type) -> BuilderStage2 {
        BuilderStage2 {
            field_name: self.field_name,
            type_: Box::new(type_),
            docs: Default::default(),
            deprecated: Default::default(),
            safety: Default::default(),
        }
    }
}
///The stage 2 builder for the [`FieldDefinition`] type
#[derive(Debug, Clone)]
pub struct BuilderStage2 {
    field_name: super::FieldName,
    type_: Box<super::Type>,
    docs: Option<super::Documentation>,
    deprecated: Option<super::Documentation>,
    safety: Option<super::LogSafety>,
}
impl BuilderStage2 {
    #[inline]
    pub fn field_name(mut self, field_name: super::FieldName) -> Self {
        self.field_name = field_name;
        self
    }
    #[inline]
    pub fn type_(mut self, type_: super::Type) -> Self {
        self.type_ = Box::new(type_);
        self
    }
    #[inline]
    pub fn docs<T>(mut self, docs: T) -> Self
    where
        T: Into<Option<super::Documentation>>,
    {
        self.docs = docs.into();
        self
    }
    #[inline]
    pub fn deprecated<T>(mut self, deprecated: T) -> Self
    where
        T: Into<Option<super::Documentation>>,
    {
        self.deprecated = deprecated.into();
        self
    }
    #[inline]
    pub fn safety<T>(mut self, safety: T) -> Self
    where
        T: Into<Option<super::LogSafety>>,
    {
        self.safety = safety.into();
        self
    }
    /// Consumes the builder, constructing a new instance of the type.
    #[inline]
    pub fn build(self) -> FieldDefinition {
        FieldDefinition {
            field_name: self.field_name,
            type_: self.type_,
            docs: self.docs,
            deprecated: self.deprecated,
            safety: self.safety,
        }
    }
}
impl ser::Serialize for FieldDefinition {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut size = 2usize;
        let skip_docs = self.docs.is_none();
        if !skip_docs {
            size += 1;
        }
        let skip_deprecated = self.deprecated.is_none();
        if !skip_deprecated {
            size += 1;
        }
        let skip_safety = self.safety.is_none();
        if !skip_safety {
            size += 1;
        }
        let mut s = s.serialize_struct("FieldDefinition", size)?;
        s.serialize_field("fieldName", &self.field_name)?;
        s.serialize_field("type", &self.type_)?;
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
        if skip_safety {
            s.skip_field("safety")?;
        } else {
            s.serialize_field("safety", &self.safety)?;
        }
        s.end()
    }
}
impl<'de> de::Deserialize<'de> for FieldDefinition {
    fn deserialize<D>(d: D) -> Result<FieldDefinition, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct(
            "FieldDefinition",
            &["fieldName", "type", "docs", "deprecated", "safety"],
            Visitor_,
        )
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = FieldDefinition;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<FieldDefinition, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut field_name = None;
        let mut type_ = None;
        let mut docs = None;
        let mut deprecated = None;
        let mut safety = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::FieldName => field_name = Some(map_.next_value()?),
                Field_::Type => type_ = Some(map_.next_value()?),
                Field_::Docs => docs = Some(map_.next_value()?),
                Field_::Deprecated => deprecated = Some(map_.next_value()?),
                Field_::Safety => safety = Some(map_.next_value()?),
                Field_::Unknown_ => {
                    map_.next_value::<de::IgnoredAny>()?;
                }
            }
        }
        let field_name = match field_name {
            Some(v) => v,
            None => return Err(de::Error::missing_field("fieldName")),
        };
        let type_ = match type_ {
            Some(v) => v,
            None => return Err(de::Error::missing_field("type")),
        };
        let docs = match docs {
            Some(v) => v,
            None => Default::default(),
        };
        let deprecated = match deprecated {
            Some(v) => v,
            None => Default::default(),
        };
        let safety = match safety {
            Some(v) => v,
            None => Default::default(),
        };
        Ok(FieldDefinition {
            field_name,
            type_,
            docs,
            deprecated,
            safety,
        })
    }
}
enum Field_ {
    FieldName,
    Type,
    Docs,
    Deprecated,
    Safety,
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
            "fieldName" => Field_::FieldName,
            "type" => Field_::Type,
            "docs" => Field_::Docs,
            "deprecated" => Field_::Deprecated,
            "safety" => Field_::Safety,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
