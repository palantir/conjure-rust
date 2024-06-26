use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[conjure_object::private::staged_builder::staged_builder]
#[builder(crate = conjure_object::private::staged_builder, update, inline)]
pub struct EndpointDefinition {
    endpoint_name: super::EndpointName,
    http_method: super::HttpMethod,
    http_path: super::HttpPath,
    #[builder(
        default,
        custom(
            type = impl
            Into<Option<super::AuthType>>,
            convert = |v|v.into().map(Box::new)
        )
    )]
    auth: Option<Box<super::AuthType>>,
    #[builder(default, list(item(type = super::ArgumentDefinition)))]
    args: Vec<super::ArgumentDefinition>,
    #[builder(
        default,
        custom(
            type = impl
            Into<Option<super::Type>>,
            convert = |v|v.into().map(Box::new)
        )
    )]
    returns: Option<Box<super::Type>>,
    #[builder(default, into)]
    docs: Option<super::Documentation>,
    #[builder(default, into)]
    deprecated: Option<super::Documentation>,
    #[builder(default, list(item(type = super::Type)))]
    markers: Vec<super::Type>,
    #[builder(default, set(item(type = String, into)))]
    tags: std::collections::BTreeSet<String>,
}
impl EndpointDefinition {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(
        endpoint_name: super::EndpointName,
        http_method: super::HttpMethod,
        http_path: super::HttpPath,
    ) -> Self {
        Self::builder()
            .endpoint_name(endpoint_name)
            .http_method(http_method)
            .http_path(http_path)
            .build()
    }
    #[inline]
    pub fn endpoint_name(&self) -> &super::EndpointName {
        &self.endpoint_name
    }
    #[inline]
    pub fn http_method(&self) -> &super::HttpMethod {
        &self.http_method
    }
    #[inline]
    pub fn http_path(&self) -> &super::HttpPath {
        &self.http_path
    }
    #[inline]
    pub fn auth(&self) -> Option<&super::AuthType> {
        self.auth.as_ref().map(|o| &**o)
    }
    #[inline]
    pub fn args(&self) -> &[super::ArgumentDefinition] {
        &*self.args
    }
    #[inline]
    pub fn returns(&self) -> Option<&super::Type> {
        self.returns.as_ref().map(|o| &**o)
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
    pub fn markers(&self) -> &[super::Type] {
        &*self.markers
    }
    #[inline]
    pub fn tags(&self) -> &std::collections::BTreeSet<String> {
        &self.tags
    }
}
impl ser::Serialize for EndpointDefinition {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut size = 3usize;
        let skip_auth = self.auth.is_none();
        if !skip_auth {
            size += 1;
        }
        let skip_args = self.args.is_empty();
        if !skip_args {
            size += 1;
        }
        let skip_returns = self.returns.is_none();
        if !skip_returns {
            size += 1;
        }
        let skip_docs = self.docs.is_none();
        if !skip_docs {
            size += 1;
        }
        let skip_deprecated = self.deprecated.is_none();
        if !skip_deprecated {
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
        let mut s = s.serialize_struct("EndpointDefinition", size)?;
        s.serialize_field("endpointName", &self.endpoint_name)?;
        s.serialize_field("httpMethod", &self.http_method)?;
        s.serialize_field("httpPath", &self.http_path)?;
        if skip_auth {
            s.skip_field("auth")?;
        } else {
            s.serialize_field("auth", &self.auth)?;
        }
        if skip_args {
            s.skip_field("args")?;
        } else {
            s.serialize_field("args", &self.args)?;
        }
        if skip_returns {
            s.skip_field("returns")?;
        } else {
            s.serialize_field("returns", &self.returns)?;
        }
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
impl<'de> de::Deserialize<'de> for EndpointDefinition {
    fn deserialize<D>(d: D) -> Result<EndpointDefinition, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct(
            "EndpointDefinition",
            &[
                "endpointName",
                "httpMethod",
                "httpPath",
                "auth",
                "args",
                "returns",
                "docs",
                "deprecated",
                "markers",
                "tags",
            ],
            Visitor_,
        )
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = EndpointDefinition;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<EndpointDefinition, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut endpoint_name = None;
        let mut http_method = None;
        let mut http_path = None;
        let mut auth = None;
        let mut args = None;
        let mut returns = None;
        let mut docs = None;
        let mut deprecated = None;
        let mut markers = None;
        let mut tags = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::EndpointName => endpoint_name = Some(map_.next_value()?),
                Field_::HttpMethod => http_method = Some(map_.next_value()?),
                Field_::HttpPath => http_path = Some(map_.next_value()?),
                Field_::Auth => auth = Some(map_.next_value()?),
                Field_::Args => args = Some(map_.next_value()?),
                Field_::Returns => returns = Some(map_.next_value()?),
                Field_::Docs => docs = Some(map_.next_value()?),
                Field_::Deprecated => deprecated = Some(map_.next_value()?),
                Field_::Markers => markers = Some(map_.next_value()?),
                Field_::Tags => tags = Some(map_.next_value()?),
                Field_::Unknown_ => {
                    map_.next_value::<de::IgnoredAny>()?;
                }
            }
        }
        let endpoint_name = match endpoint_name {
            Some(v) => v,
            None => return Err(de::Error::missing_field("endpointName")),
        };
        let http_method = match http_method {
            Some(v) => v,
            None => return Err(de::Error::missing_field("httpMethod")),
        };
        let http_path = match http_path {
            Some(v) => v,
            None => return Err(de::Error::missing_field("httpPath")),
        };
        let auth = match auth {
            Some(v) => v,
            None => Default::default(),
        };
        let args = match args {
            Some(v) => v,
            None => Default::default(),
        };
        let returns = match returns {
            Some(v) => v,
            None => Default::default(),
        };
        let docs = match docs {
            Some(v) => v,
            None => Default::default(),
        };
        let deprecated = match deprecated {
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
        Ok(EndpointDefinition {
            endpoint_name,
            http_method,
            http_path,
            auth,
            args,
            returns,
            docs,
            deprecated,
            markers,
            tags,
        })
    }
}
enum Field_ {
    EndpointName,
    HttpMethod,
    HttpPath,
    Auth,
    Args,
    Returns,
    Docs,
    Deprecated,
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
            "endpointName" => Field_::EndpointName,
            "httpMethod" => Field_::HttpMethod,
            "httpPath" => Field_::HttpPath,
            "auth" => Field_::Auth,
            "args" => Field_::Args,
            "returns" => Field_::Returns,
            "docs" => Field_::Docs,
            "deprecated" => Field_::Deprecated,
            "markers" => Field_::Markers,
            "tags" => Field_::Tags,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
