use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnionDefinition {
    type_name: Box<super::TypeName>,
    union_: Vec<super::FieldDefinition>,
    docs: Option<super::Documentation>,
}
impl UnionDefinition {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new<T>(
        type_name: super::TypeName,
        union_: T,
        docs: super::Documentation,
    ) -> UnionDefinition
    where
        T: IntoIterator<Item = super::FieldDefinition>,
    {
        UnionDefinition {
            type_name: Box::new(type_name),
            union_: union_.into_iter().collect(),
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
    pub fn union_(&self) -> &[super::FieldDefinition] {
        &*self.union_
    }
    #[inline]
    pub fn docs(&self) -> Option<&super::Documentation> {
        self.docs.as_ref().map(|o| &*o)
    }
}
///A builder for the `UnionDefinition` type.
#[derive(Debug, Clone, Default)]
pub struct Builder {
    type_name: Option<Box<super::TypeName>>,
    union_: Vec<super::FieldDefinition>,
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
    pub fn union_<T>(&mut self, union_: T) -> &mut Self
    where
        T: IntoIterator<Item = super::FieldDefinition>,
    {
        self.union_ = union_.into_iter().collect();
        self
    }
    #[inline]
    pub fn extend_union_<T>(&mut self, union_: T) -> &mut Self
    where
        T: IntoIterator<Item = super::FieldDefinition>,
    {
        self.union_.extend(union_);
        self
    }
    #[inline]
    pub fn push_union_(&mut self, value: super::FieldDefinition) -> &mut Self {
        self.union_.push(value);
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
    pub fn build(&self) -> UnionDefinition {
        UnionDefinition {
            type_name: self.type_name.clone().expect("field type_name was not set"),
            union_: self.union_.clone(),
            docs: self.docs.clone(),
        }
    }
}
impl From<UnionDefinition> for Builder {
    #[inline]
    fn from(_v: UnionDefinition) -> Builder {
        Builder {
            type_name: Some(_v.type_name),
            union_: _v.union_,
            docs: _v.docs,
        }
    }
}
impl ser::Serialize for UnionDefinition {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut size = 1usize;
        let skip_union_ = self.union_.is_empty();
        if !skip_union_ {
            size += 1;
        }
        let skip_docs = self.docs.is_none();
        if !skip_docs {
            size += 1;
        }
        let mut s = s.serialize_struct("UnionDefinition", size)?;
        s.serialize_field("typeName", &self.type_name)?;
        if skip_union_ {
            s.skip_field("union")?;
        } else {
            s.serialize_field("union", &self.union_)?;
        }
        if skip_docs {
            s.skip_field("docs")?;
        } else {
            s.serialize_field("docs", &self.docs)?;
        }
        s.end()
    }
}
impl<'de> de::Deserialize<'de> for UnionDefinition {
    fn deserialize<D>(d: D) -> Result<UnionDefinition, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct("UnionDefinition", &["typeName", "union", "docs"], Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = UnionDefinition;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<UnionDefinition, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut type_name = None;
        let mut union_ = None;
        let mut docs = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::TypeName => type_name = Some(map_.next_value()?),
                Field_::Union => union_ = Some(map_.next_value()?),
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
        let union_ = match union_ {
            Some(v) => v,
            None => Default::default(),
        };
        let docs = match docs {
            Some(v) => v,
            None => Default::default(),
        };
        Ok(UnionDefinition {
            type_name,
            union_,
            docs,
        })
    }
}
enum Field_ {
    TypeName,
    Union,
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
            "union" => Field_::Union,
            "docs" => Field_::Docs,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
