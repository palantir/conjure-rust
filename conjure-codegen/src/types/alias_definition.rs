use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[conjure_object::private::staged_builder::staged_builder]
#[builder(crate = conjure_object::private::staged_builder, update, inline)]
pub struct AliasDefinition {
    #[builder(custom(type = super::TypeName, convert = Box::new))]
    type_name: Box<super::TypeName>,
    #[builder(custom(type = super::Type, convert = Box::new))]
    alias: Box<super::Type>,
    #[builder(default, into)]
    docs: Option<super::Documentation>,
    #[builder(default, into)]
    safety: Option<super::LogSafety>,
}
impl AliasDefinition {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(type_name: super::TypeName, alias: super::Type) -> Self {
        Self::builder().type_name(type_name).alias(alias).build()
    }
    #[inline]
    pub fn type_name(&self) -> &super::TypeName {
        &*self.type_name
    }
    #[inline]
    pub fn alias(&self) -> &super::Type {
        &*self.alias
    }
    #[inline]
    pub fn docs(&self) -> Option<&super::Documentation> {
        self.docs.as_ref().map(|o| &*o)
    }
    #[inline]
    pub fn safety(&self) -> Option<&super::LogSafety> {
        self.safety.as_ref().map(|o| &*o)
    }
}
impl ser::Serialize for AliasDefinition {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut size = 2usize;
        let skip_docs = self.docs.is_none();
        if !skip_docs {
            size += 1;
        }
        let skip_safety = self.safety.is_none();
        if !skip_safety {
            size += 1;
        }
        let mut s = s.serialize_struct("AliasDefinition", size)?;
        s.serialize_field("typeName", &self.type_name)?;
        s.serialize_field("alias", &self.alias)?;
        if skip_docs {
            s.skip_field("docs")?;
        } else {
            s.serialize_field("docs", &self.docs)?;
        }
        if skip_safety {
            s.skip_field("safety")?;
        } else {
            s.serialize_field("safety", &self.safety)?;
        }
        s.end()
    }
}
impl<'de> de::Deserialize<'de> for AliasDefinition {
    fn deserialize<D>(d: D) -> Result<AliasDefinition, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct(
            "AliasDefinition",
            &["typeName", "alias", "docs", "safety"],
            Visitor_,
        )
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = AliasDefinition;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<AliasDefinition, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut type_name = None;
        let mut alias = None;
        let mut docs = None;
        let mut safety = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::TypeName => type_name = Some(map_.next_value()?),
                Field_::Alias => alias = Some(map_.next_value()?),
                Field_::Docs => docs = Some(map_.next_value()?),
                Field_::Safety => safety = Some(map_.next_value()?),
                Field_::Unknown_ => {
                    map_.next_value::<de::IgnoredAny>()?;
                }
            }
        }
        let type_name = match type_name {
            Some(v) => v,
            None => return Err(de::Error::missing_field("typeName")),
        };
        let alias = match alias {
            Some(v) => v,
            None => return Err(de::Error::missing_field("alias")),
        };
        let docs = match docs {
            Some(v) => v,
            None => Default::default(),
        };
        let safety = match safety {
            Some(v) => v,
            None => Default::default(),
        };
        Ok(AliasDefinition {
            type_name,
            alias,
            docs,
            safety,
        })
    }
}
enum Field_ {
    TypeName,
    Alias,
    Docs,
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
            "typeName" => Field_::TypeName,
            "alias" => Field_::Alias,
            "docs" => Field_::Docs,
            "safety" => Field_::Safety,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
