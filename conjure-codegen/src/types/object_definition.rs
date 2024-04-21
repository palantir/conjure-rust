use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObjectDefinition {
    type_name: Box<super::TypeName>,
    fields: Vec<super::FieldDefinition>,
    docs: Option<super::Documentation>,
}
impl ObjectDefinition {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new<T>(
        type_name: super::TypeName,
        fields: T,
        docs: super::Documentation,
    ) -> ObjectDefinition
    where
        T: IntoIterator<Item = super::FieldDefinition>,
    {
        ObjectDefinition {
            type_name: Box::new(type_name),
            fields: fields.into_iter().collect(),
            docs: Some(docs),
        }
    }
    /// Returns a new builder.
    #[inline]
    pub fn builder() -> BuilderStage0 {
        Default::default()
    }
    #[inline]
    pub fn type_name(&self) -> &super::TypeName {
        &*self.type_name
    }
    #[inline]
    pub fn fields(&self) -> &[super::FieldDefinition] {
        &*self.fields
    }
    #[inline]
    pub fn docs(&self) -> Option<&super::Documentation> {
        self.docs.as_ref().map(|o| &*o)
    }
}
impl Default for BuilderStage0 {
    #[inline]
    fn default() -> Self {
        BuilderStage0 {}
    }
}
impl From<ObjectDefinition> for BuilderStage1 {
    #[inline]
    fn from(value: ObjectDefinition) -> Self {
        BuilderStage1 {
            type_name: value.type_name,
            fields: value.fields,
            docs: value.docs,
        }
    }
}
///The stage 0 builder for the [`ObjectDefinition`] type
#[derive(Debug, Clone)]
pub struct BuilderStage0 {}
impl BuilderStage0 {
    #[inline]
    pub fn type_name(self, type_name: super::TypeName) -> BuilderStage1 {
        BuilderStage1 {
            type_name: Box::new(type_name),
            fields: Default::default(),
            docs: Default::default(),
        }
    }
}
///The stage 1 builder for the [`ObjectDefinition`] type
#[derive(Debug, Clone)]
pub struct BuilderStage1 {
    type_name: Box<super::TypeName>,
    fields: Vec<super::FieldDefinition>,
    docs: Option<super::Documentation>,
}
impl BuilderStage1 {
    #[inline]
    pub fn type_name(mut self, type_name: super::TypeName) -> Self {
        self.type_name = Box::new(type_name);
        self
    }
    #[inline]
    pub fn fields<T>(mut self, fields: T) -> Self
    where
        T: IntoIterator<Item = super::FieldDefinition>,
    {
        self.fields = fields.into_iter().collect();
        self
    }
    #[inline]
    pub fn extend_fields<T>(mut self, fields: T) -> Self
    where
        T: IntoIterator<Item = super::FieldDefinition>,
    {
        self.fields.extend(fields);
        self
    }
    #[inline]
    pub fn push_fields(mut self, value: super::FieldDefinition) -> Self {
        self.fields.push(value);
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
    /// Consumes the builder, constructing a new instance of the type.
    #[inline]
    pub fn build(self) -> ObjectDefinition {
        ObjectDefinition {
            type_name: self.type_name,
            fields: self.fields,
            docs: self.docs,
        }
    }
}
impl ser::Serialize for ObjectDefinition {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut size = 1usize;
        let skip_fields = self.fields.is_empty();
        if !skip_fields {
            size += 1;
        }
        let skip_docs = self.docs.is_none();
        if !skip_docs {
            size += 1;
        }
        let mut s = s.serialize_struct("ObjectDefinition", size)?;
        s.serialize_field("typeName", &self.type_name)?;
        if skip_fields {
            s.skip_field("fields")?;
        } else {
            s.serialize_field("fields", &self.fields)?;
        }
        if skip_docs {
            s.skip_field("docs")?;
        } else {
            s.serialize_field("docs", &self.docs)?;
        }
        s.end()
    }
}
impl<'de> de::Deserialize<'de> for ObjectDefinition {
    fn deserialize<D>(d: D) -> Result<ObjectDefinition, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct(
            "ObjectDefinition",
            &["typeName", "fields", "docs"],
            Visitor_,
        )
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = ObjectDefinition;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<ObjectDefinition, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut type_name = None;
        let mut fields = None;
        let mut docs = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::TypeName => type_name = Some(map_.next_value()?),
                Field_::Fields => fields = Some(map_.next_value()?),
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
        let fields = match fields {
            Some(v) => v,
            None => Default::default(),
        };
        let docs = match docs {
            Some(v) => v,
            None => Default::default(),
        };
        Ok(ObjectDefinition {
            type_name,
            fields,
            docs,
        })
    }
}
enum Field_ {
    TypeName,
    Fields,
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
            "fields" => Field_::Fields,
            "docs" => Field_::Docs,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
