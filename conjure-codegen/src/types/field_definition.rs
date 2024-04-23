use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[conjure_object::private::staged_builder::staged_builder]
#[builder(crate = conjure_object::private::staged_builder, update, inline)]
pub struct FieldDefinition {
    #[builder()]
    field_name: super::FieldName,
    #[builder(custom(type = super::Type, convert = Box::new))]
    type_: Box<super::Type>,
    #[builder(default, into)]
    docs: Option<super::Documentation>,
    #[builder(default, into)]
    deprecated: Option<super::Documentation>,
    #[builder(default, into)]
    safety: Option<super::LogSafety>,
}
impl FieldDefinition {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(field_name: super::FieldName, type_: super::Type) -> Self {
        Self::builder().field_name(field_name).type_(type_).build()
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
