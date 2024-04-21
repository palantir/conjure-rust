use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArgumentDefinition {
    arg_name: super::ArgumentName,
    type_: Box<super::Type>,
    param_type: Box<super::ParameterType>,
    safety: Option<super::LogSafety>,
    docs: Option<super::Documentation>,
    markers: Vec<super::Type>,
    tags: std::collections::BTreeSet<String>,
}
impl ArgumentDefinition {
    /// Returns a new builder.
    #[inline]
    pub fn builder() -> BuilderStage0 {
        Default::default()
    }
    #[inline]
    pub fn arg_name(&self) -> &super::ArgumentName {
        &self.arg_name
    }
    #[inline]
    pub fn type_(&self) -> &super::Type {
        &*self.type_
    }
    #[inline]
    pub fn param_type(&self) -> &super::ParameterType {
        &*self.param_type
    }
    #[inline]
    pub fn safety(&self) -> Option<&super::LogSafety> {
        self.safety.as_ref().map(|o| &*o)
    }
    #[inline]
    pub fn docs(&self) -> Option<&super::Documentation> {
        self.docs.as_ref().map(|o| &*o)
    }
    #[inline]
    pub fn markers(&self) -> &[super::Type] {
        &*self.markers
    }
    #[inline]
    pub fn tags(&self) -> &std::collections::BTreeSet<String> {
        &self.tags
    }
}
impl Default for BuilderStage0 {
    #[inline]
    fn default() -> Self {
        BuilderStage0 {}
    }
}
impl From<ArgumentDefinition> for BuilderStage3 {
    #[inline]
    fn from(value: ArgumentDefinition) -> Self {
        BuilderStage3 {
            arg_name: value.arg_name,
            type_: value.type_,
            param_type: value.param_type,
            safety: value.safety,
            docs: value.docs,
            markers: value.markers,
            tags: value.tags,
        }
    }
}
///The stage 0 builder for the [`ArgumentDefinition`] type
#[derive(Debug, Clone)]
pub struct BuilderStage0 {}
impl BuilderStage0 {
    #[inline]
    pub fn arg_name(self, arg_name: super::ArgumentName) -> BuilderStage1 {
        BuilderStage1 {
            arg_name: arg_name,
        }
    }
}
///The stage 1 builder for the [`ArgumentDefinition`] type
#[derive(Debug, Clone)]
pub struct BuilderStage1 {
    arg_name: super::ArgumentName,
}
impl BuilderStage1 {
    #[inline]
    pub fn type_(self, type_: super::Type) -> BuilderStage2 {
        BuilderStage2 {
            arg_name: self.arg_name,
            type_: Box::new(type_),
        }
    }
}
///The stage 2 builder for the [`ArgumentDefinition`] type
#[derive(Debug, Clone)]
pub struct BuilderStage2 {
    arg_name: super::ArgumentName,
    type_: Box<super::Type>,
}
impl BuilderStage2 {
    #[inline]
    pub fn param_type(self, param_type: super::ParameterType) -> BuilderStage3 {
        BuilderStage3 {
            arg_name: self.arg_name,
            type_: self.type_,
            param_type: Box::new(param_type),
            safety: Default::default(),
            docs: Default::default(),
            markers: Default::default(),
            tags: Default::default(),
        }
    }
}
///The stage 3 builder for the [`ArgumentDefinition`] type
#[derive(Debug, Clone)]
pub struct BuilderStage3 {
    arg_name: super::ArgumentName,
    type_: Box<super::Type>,
    param_type: Box<super::ParameterType>,
    safety: Option<super::LogSafety>,
    docs: Option<super::Documentation>,
    markers: Vec<super::Type>,
    tags: std::collections::BTreeSet<String>,
}
impl BuilderStage3 {
    #[inline]
    pub fn arg_name(mut self, arg_name: super::ArgumentName) -> Self {
        self.arg_name = arg_name;
        self
    }
    #[inline]
    pub fn type_(mut self, type_: super::Type) -> Self {
        self.type_ = Box::new(type_);
        self
    }
    #[inline]
    pub fn param_type(mut self, param_type: super::ParameterType) -> Self {
        self.param_type = Box::new(param_type);
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
    #[inline]
    pub fn docs<T>(mut self, docs: T) -> Self
    where
        T: Into<Option<super::Documentation>>,
    {
        self.docs = docs.into();
        self
    }
    #[inline]
    pub fn markers<T>(mut self, markers: T) -> Self
    where
        T: IntoIterator<Item = super::Type>,
    {
        self.markers = markers.into_iter().collect();
        self
    }
    #[inline]
    pub fn extend_markers<T>(mut self, markers: T) -> Self
    where
        T: IntoIterator<Item = super::Type>,
    {
        self.markers.extend(markers);
        self
    }
    #[inline]
    pub fn push_markers(mut self, value: super::Type) -> Self {
        self.markers.push(value);
        self
    }
    #[inline]
    pub fn tags<T>(mut self, tags: T) -> Self
    where
        T: IntoIterator<Item = String>,
    {
        self.tags = tags.into_iter().collect();
        self
    }
    #[inline]
    pub fn extend_tags<T>(mut self, tags: T) -> Self
    where
        T: IntoIterator<Item = String>,
    {
        self.tags.extend(tags);
        self
    }
    #[inline]
    pub fn insert_tags<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.tags.insert(value.into());
        self
    }
    /// Consumes the builder, constructing a new instance of the type.
    #[inline]
    pub fn build(self) -> ArgumentDefinition {
        ArgumentDefinition {
            arg_name: self.arg_name,
            type_: self.type_,
            param_type: self.param_type,
            safety: self.safety,
            docs: self.docs,
            markers: self.markers,
            tags: self.tags,
        }
    }
}
impl ser::Serialize for ArgumentDefinition {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut size = 3usize;
        let skip_safety = self.safety.is_none();
        if !skip_safety {
            size += 1;
        }
        let skip_docs = self.docs.is_none();
        if !skip_docs {
            size += 1;
        }
        let skip_markers = self.markers.is_empty();
        if !skip_markers {
            size += 1;
        }
        let skip_tags = self.tags.is_empty();
        if !skip_tags {
            size += 1;
        }
        let mut s = s.serialize_struct("ArgumentDefinition", size)?;
        s.serialize_field("argName", &self.arg_name)?;
        s.serialize_field("type", &self.type_)?;
        s.serialize_field("paramType", &self.param_type)?;
        if skip_safety {
            s.skip_field("safety")?;
        } else {
            s.serialize_field("safety", &self.safety)?;
        }
        if skip_docs {
            s.skip_field("docs")?;
        } else {
            s.serialize_field("docs", &self.docs)?;
        }
        if skip_markers {
            s.skip_field("markers")?;
        } else {
            s.serialize_field("markers", &self.markers)?;
        }
        if skip_tags {
            s.skip_field("tags")?;
        } else {
            s.serialize_field("tags", &self.tags)?;
        }
        s.end()
    }
}
impl<'de> de::Deserialize<'de> for ArgumentDefinition {
    fn deserialize<D>(d: D) -> Result<ArgumentDefinition, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct(
            "ArgumentDefinition",
            &["argName", "type", "paramType", "safety", "docs", "markers", "tags"],
            Visitor_,
        )
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = ArgumentDefinition;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<ArgumentDefinition, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut arg_name = None;
        let mut type_ = None;
        let mut param_type = None;
        let mut safety = None;
        let mut docs = None;
        let mut markers = None;
        let mut tags = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::ArgName => arg_name = Some(map_.next_value()?),
                Field_::Type => type_ = Some(map_.next_value()?),
                Field_::ParamType => param_type = Some(map_.next_value()?),
                Field_::Safety => safety = Some(map_.next_value()?),
                Field_::Docs => docs = Some(map_.next_value()?),
                Field_::Markers => markers = Some(map_.next_value()?),
                Field_::Tags => tags = Some(map_.next_value()?),
                Field_::Unknown_ => {
                    map_.next_value::<de::IgnoredAny>()?;
                }
            }
        }
        let arg_name = match arg_name {
            Some(v) => v,
            None => return Err(de::Error::missing_field("argName")),
        };
        let type_ = match type_ {
            Some(v) => v,
            None => return Err(de::Error::missing_field("type")),
        };
        let param_type = match param_type {
            Some(v) => v,
            None => return Err(de::Error::missing_field("paramType")),
        };
        let safety = match safety {
            Some(v) => v,
            None => Default::default(),
        };
        let docs = match docs {
            Some(v) => v,
            None => Default::default(),
        };
        let markers = match markers {
            Some(v) => v,
            None => Default::default(),
        };
        let tags = match tags {
            Some(v) => v,
            None => Default::default(),
        };
        Ok(ArgumentDefinition {
            arg_name,
            type_,
            param_type,
            safety,
            docs,
            markers,
            tags,
        })
    }
}
enum Field_ {
    ArgName,
    Type,
    ParamType,
    Safety,
    Docs,
    Markers,
    Tags,
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
            "argName" => Field_::ArgName,
            "type" => Field_::Type,
            "paramType" => Field_::ParamType,
            "safety" => Field_::Safety,
            "docs" => Field_::Docs,
            "markers" => Field_::Markers,
            "tags" => Field_::Tags,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
