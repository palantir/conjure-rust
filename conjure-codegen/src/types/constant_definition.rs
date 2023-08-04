use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstantDefinition {
    type_name: Box<super::TypeName>,
    value: String,
    const_type: super::PrimitiveType,
    docs: Option<super::Documentation>
}
impl ConstantDefinition {
    // Constructs a new instance of the type.
    #[inline]
    pub fn new<T>(
        type_name: super::TypeName,
        value: String,
        const_type: super::PrimitiveType,
        docs: super::Documentation,
    ) -> ConstantDefinition
    where
        T: IntoIterator<Item = super::FieldDefinition>,
    {
        ConstantDefinition {
            type_name: Box::new(type_name),
            value,
            const_type,
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
    pub fn const_type(&self) -> &super::PrimitiveType {
        &self.const_type
    }
    #[inline]
    pub fn value(&self) -> &String {
        &self.value
    }
    #[inline]
    pub fn docs(&self) -> Option<&super::Documentation> {
        self.docs.as_ref().map(|o| &*o)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Builder {
    type_name: Option<Box<super::TypeName>>,
    value: Option<String>,
    const_type: Option<super::PrimitiveType>,
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
    pub fn value(&mut self, value: String) -> &mut Self {
        self.value = Some(value);
        self
    }
    #[inline]
    pub fn const_type(&mut self, const_type: super::PrimitiveType) -> &mut Self {
        self.const_type = Some(const_type);
        self
    }
    #[inline]
    pub fn docs(&mut self, docs: super::Documentation) -> &mut Self {
        self.docs = Some(docs);
        self
    }
    #[inline]
    pub fn build(&self) -> ConstantDefinition {
        ConstantDefinition {
            type_name: self.type_name.clone().expect("field type_name is missing"),
            value: self.value.clone().expect("field value is missing"),
            const_type: self.const_type.clone().expect("field const_type is missing"),
            docs: self.docs.clone(),
        }
    }
}
impl From<ConstantDefinition> for Builder {
    #[inline]
    fn from(_v: ConstantDefinition) -> Builder {
        Builder {
            type_name: Some(_v.type_name),
            value: Some(_v.value),
            const_type: Some(_v.const_type),
            docs: _v.docs,
        }
    }
}
impl ser::Serialize for ConstantDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut size = 2usize;
        let skip_docs = self.docs.is_none();
        if !skip_docs {
            size += 1;
        }
        let mut map = serializer.serialize_struct("ConstantDefinition", size)?;
        map.serialize_field("typeName", &self.type_name)?;
        map.serialize_field("value", &self.value)?;
        map.serialize_field("type", &self.const_type)?;
        if let Some(ref s) = self.docs {
            map.serialize_field("docs", s)?;
        }
        map.end()
    }
}

impl<'de> de::Deserialize<'de> for ConstantDefinition  {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: de::Deserializer<'de> {
        deserializer.deserialize_struct("ConstantDefinition", &["typeName", "value", "constType", "docs"], Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = ConstantDefinition;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<ConstantDefinition, A::Error>
    where A: de::MapAccess<'de> {
        let mut type_name = None;
        let mut value = None;
        let mut const_type = None;
        let mut docs = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::TypeName => type_name = Some(map_.next_value()?),
                Field_::Value => value = Some(map_.next_value()?),
                Field_::ConstType => const_type = Some(map_.next_value()?),
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
        let value = match value {
            Some(v) => v,
            None => return Err(de::Error::missing_field("value")),
        };
        let const_type = match const_type {
            Some(v) => v,
            None => return Err(de::Error::missing_field("constType")),
        };
        let docs = match docs {
            Some(v) => v,
            None => Default::default(),
        };
        Ok(ConstantDefinition {
            type_name,
            value,
            const_type,
            docs,
        })
    }
}
enum Field_ {
    TypeName,
    Value,
    ConstType,
    Docs,
    Unknown_
}
impl<'de> de::Deserialize<'de> for Field_ {
    fn deserialize<D>(d: D) -> Result<Field_, D::Error>
    where
        D: de::Deserializer<'de>, { d.deserialize_str(FieldVisitor_ )}
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
            "value" => Field_::Value,
            "type" => Field_::ConstType,
            "docs" => Field_::Docs,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}