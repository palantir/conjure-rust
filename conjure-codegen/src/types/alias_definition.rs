use conjure_types::serde::ser::SerializeMap as SerializeMap_;
use conjure_types::serde::{de, ser};
use std::fmt;
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct AliasDefinition {
    type_name: Box<super::TypeName>,
    alias: Box<super::Type>,
    docs: Option<super::Documentation>,
}
impl AliasDefinition {
    #[doc = r" Returns a new builder."]
    #[inline]
    pub fn builder() -> Builder {
        Default::default()
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
}
#[derive(Debug, Clone, Default)]
pub struct Builder {
    type_name: Option<Box<super::TypeName>>,
    alias: Option<Box<super::Type>>,
    docs: Option<super::Documentation>,
}
impl Builder {
    #[inline]
    pub fn type_name(&mut self, type_name: super::TypeName) -> &mut Self {
        self.type_name = Some(Box::new(type_name));
        self
    }
    #[inline]
    pub fn alias(&mut self, alias: super::Type) -> &mut Self {
        self.alias = Some(Box::new(alias));
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
    pub fn build(&self) -> AliasDefinition {
        AliasDefinition {
            type_name: self.type_name.clone().expect("field type_name was not set"),
            alias: self.alias.clone().expect("field alias was not set"),
            docs: self.docs.clone(),
        }
    }
}
impl From<AliasDefinition> for Builder {
    #[inline]
    fn from(_v: AliasDefinition) -> Builder {
        Builder {
            type_name: Some(_v.type_name),
            alias: Some(_v.alias),
            docs: _v.docs,
        }
    }
}
impl ser::Serialize for AliasDefinition {
    fn serialize<S_>(&self, s: S_) -> Result<S_::Ok, S_::Error>
    where
        S_: ser::Serializer,
    {
        let mut size = 2usize;
        let skip_docs = self.docs.is_none();
        if !skip_docs {
            size += 1;
        }
        let mut map = s.serialize_map(Some(size))?;
        map.serialize_entry(&"typeName", &self.type_name)?;
        map.serialize_entry(&"alias", &self.alias)?;
        if !skip_docs {
            map.serialize_entry(&"docs", &self.docs)?;
        }
        map.end()
    }
}
impl<'de> de::Deserialize<'de> for AliasDefinition {
    fn deserialize<D_>(d: D_) -> Result<AliasDefinition, D_::Error>
    where
        D_: de::Deserializer<'de>,
    {
        d.deserialize_struct("AliasDefinition", &["typeName", "alias", "docs"], Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = AliasDefinition;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A_>(self, mut map_: A_) -> Result<AliasDefinition, A_::Error>
    where
        A_: de::MapAccess<'de>,
    {
        let mut type_name = None;
        let mut alias = None;
        let mut docs = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::TypeName => type_name = Some(map_.next_value()?),
                Field_::Alias => alias = Some(map_.next_value()?),
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
        let alias = match alias {
            Some(v) => v,
            None => return Err(de::Error::missing_field("alias")),
        };
        let docs = match docs {
            Some(v) => v,
            None => Default::default(),
        };
        Ok(AliasDefinition {
            type_name,
            alias,
            docs,
        })
    }
}
enum Field_ {
    TypeName,
    Alias,
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
            "typeName" => Field_::TypeName,
            "alias" => Field_::Alias,
            "docs" => Field_::Docs,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
