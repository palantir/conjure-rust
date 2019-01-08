use conjure::serde::ser::SerializeMap as SerializeMap_;
use conjure::serde::{de, ser};
use std::fmt;
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ArgumentDefinition {
    arg_name: super::ArgumentName,
    type_: Box<super::Type>,
    param_type: Box<super::ParameterType>,
    docs: Option<super::Documentation>,
    markers: Vec<super::Type>,
}
impl ArgumentDefinition {
    #[doc = r" Returns a new builder."]
    #[inline]
    pub fn builder() -> Builder {
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
    pub fn docs(&self) -> Option<&super::Documentation> {
        self.docs.as_ref().map(|o| &*o)
    }
    #[inline]
    pub fn markers(&self) -> &[super::Type] {
        &*self.markers
    }
}
#[derive(Debug, Clone, Default)]
pub struct Builder {
    arg_name: Option<super::ArgumentName>,
    type_: Option<Box<super::Type>>,
    param_type: Option<Box<super::ParameterType>>,
    docs: Option<super::Documentation>,
    markers: Vec<super::Type>,
}
impl Builder {
    #[inline]
    pub fn arg_name(&mut self, arg_name: super::ArgumentName) -> &mut Self {
        self.arg_name = Some(arg_name);
        self
    }
    #[inline]
    pub fn type_(&mut self, type_: super::Type) -> &mut Self {
        self.type_ = Some(Box::new(type_));
        self
    }
    #[inline]
    pub fn param_type(&mut self, param_type: super::ParameterType) -> &mut Self {
        self.param_type = Some(Box::new(param_type));
        self
    }
    pub fn docs<T>(&mut self, docs: T) -> &mut Self
    where
        T: Into<Option<super::Documentation>>,
    {
        self.docs = docs.into();
        self
    }
    pub fn markers<T>(&mut self, markers: T) -> &mut Self
    where
        T: IntoIterator<Item = super::Type>,
    {
        self.markers = markers.into_iter().collect();
        self
    }
    pub fn extend_markers<T>(&mut self, markers: T) -> &mut Self
    where
        T: IntoIterator<Item = super::Type>,
    {
        self.markers.extend(markers);
        self
    }
    #[doc = r" Constructs a new instance of the type."]
    #[doc = r""]
    #[doc = r" # Panics"]
    #[doc = r""]
    #[doc = r" Panics if a required field was not set."]
    #[inline]
    pub fn build(&self) -> ArgumentDefinition {
        ArgumentDefinition {
            arg_name: self.arg_name.clone().expect("field arg_name was not set"),
            type_: self.type_.clone().expect("field type_ was not set"),
            param_type: self
                .param_type
                .clone()
                .expect("field param_type was not set"),
            docs: self.docs.clone(),
            markers: self.markers.clone(),
        }
    }
}
impl From<ArgumentDefinition> for Builder {
    #[inline]
    fn from(_v: ArgumentDefinition) -> Builder {
        Builder {
            arg_name: Some(_v.arg_name),
            type_: Some(_v.type_),
            param_type: Some(_v.param_type),
            docs: _v.docs,
            markers: _v.markers,
        }
    }
}
impl ser::Serialize for ArgumentDefinition {
    fn serialize<S_>(&self, s: S_) -> Result<S_::Ok, S_::Error>
    where
        S_: ser::Serializer,
    {
        let mut size = 3usize;
        let skip_docs = self.docs.is_none();
        if !skip_docs {
            size += 1;
        }
        let skip_markers = self.markers.is_empty();
        if !skip_markers {
            size += 1;
        }
        let mut map = s.serialize_map(Some(size))?;
        map.serialize_entry(&"argName", &self.arg_name)?;
        map.serialize_entry(&"type", &self.type_)?;
        map.serialize_entry(&"paramType", &self.param_type)?;
        if !skip_docs {
            map.serialize_entry(&"docs", &self.docs)?;
        }
        if !skip_markers {
            map.serialize_entry(&"markers", &self.markers)?;
        }
        map.end()
    }
}
impl<'de> de::Deserialize<'de> for ArgumentDefinition {
    fn deserialize<D_>(d: D_) -> Result<ArgumentDefinition, D_::Error>
    where
        D_: de::Deserializer<'de>,
    {
        d.deserialize_struct(
            "ArgumentDefinition",
            &["argName", "type", "paramType", "docs", "markers"],
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
    fn visit_map<A_>(self, mut map_: A_) -> Result<ArgumentDefinition, A_::Error>
    where
        A_: de::MapAccess<'de>,
    {
        let mut arg_name = None;
        let mut type_ = None;
        let mut param_type = None;
        let mut docs = None;
        let mut markers = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::ArgName => arg_name = Some(map_.next_value()?),
                Field_::Type => type_ = Some(map_.next_value()?),
                Field_::ParamType => param_type = Some(map_.next_value()?),
                Field_::Docs => docs = Some(map_.next_value()?),
                Field_::Markers => markers = Some(map_.next_value()?),
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
        let docs = match docs {
            Some(v) => v,
            None => Default::default(),
        };
        let markers = match markers {
            Some(v) => v,
            None => Default::default(),
        };
        Ok(ArgumentDefinition {
            arg_name,
            type_,
            param_type,
            docs,
            markers,
        })
    }
}
enum Field_ {
    ArgName,
    Type,
    ParamType,
    Docs,
    Markers,
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
            "argName" => Field_::ArgName,
            "type" => Field_::Type,
            "paramType" => Field_::ParamType,
            "docs" => Field_::Docs,
            "markers" => Field_::Markers,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
