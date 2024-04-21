use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ServiceDefinition {
    service_name: Box<super::TypeName>,
    endpoints: Vec<super::EndpointDefinition>,
    docs: Option<super::Documentation>,
}
impl ServiceDefinition {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new<T>(
        service_name: super::TypeName,
        endpoints: T,
        docs: super::Documentation,
    ) -> ServiceDefinition
    where
        T: IntoIterator<Item = super::EndpointDefinition>,
    {
        ServiceDefinition {
            service_name: Box::new(service_name),
            endpoints: endpoints.into_iter().collect(),
            docs: Some(docs),
        }
    }
    /// Returns a new builder.
    #[inline]
    pub fn builder() -> BuilderStage0 {
        Default::default()
    }
    #[inline]
    pub fn service_name(&self) -> &super::TypeName {
        &*self.service_name
    }
    #[inline]
    pub fn endpoints(&self) -> &[super::EndpointDefinition] {
        &*self.endpoints
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
impl From<ServiceDefinition> for BuilderStage1 {
    #[inline]
    fn from(value: ServiceDefinition) -> Self {
        BuilderStage1 {
            service_name: value.service_name,
            endpoints: value.endpoints,
            docs: value.docs,
        }
    }
}
///The stage 0 builder for the [`ServiceDefinition`] type
#[derive(Debug, Clone)]
pub struct BuilderStage0 {}
impl BuilderStage0 {
    #[inline]
    pub fn service_name(self, service_name: super::TypeName) -> BuilderStage1 {
        BuilderStage1 {
            service_name: Box::new(service_name),
            endpoints: Default::default(),
            docs: Default::default(),
        }
    }
}
///The stage 1 builder for the [`ServiceDefinition`] type
#[derive(Debug, Clone)]
pub struct BuilderStage1 {
    service_name: Box<super::TypeName>,
    endpoints: Vec<super::EndpointDefinition>,
    docs: Option<super::Documentation>,
}
impl BuilderStage1 {
    #[inline]
    pub fn service_name(mut self, service_name: super::TypeName) -> Self {
        self.service_name = Box::new(service_name);
        self
    }
    #[inline]
    pub fn endpoints<T>(mut self, endpoints: T) -> Self
    where
        T: IntoIterator<Item = super::EndpointDefinition>,
    {
        self.endpoints = endpoints.into_iter().collect();
        self
    }
    #[inline]
    pub fn extend_endpoints<T>(mut self, endpoints: T) -> Self
    where
        T: IntoIterator<Item = super::EndpointDefinition>,
    {
        self.endpoints.extend(endpoints);
        self
    }
    #[inline]
    pub fn push_endpoints(mut self, value: super::EndpointDefinition) -> Self {
        self.endpoints.push(value);
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
    pub fn build(self) -> ServiceDefinition {
        ServiceDefinition {
            service_name: self.service_name,
            endpoints: self.endpoints,
            docs: self.docs,
        }
    }
}
impl ser::Serialize for ServiceDefinition {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut size = 1usize;
        let skip_endpoints = self.endpoints.is_empty();
        if !skip_endpoints {
            size += 1;
        }
        let skip_docs = self.docs.is_none();
        if !skip_docs {
            size += 1;
        }
        let mut s = s.serialize_struct("ServiceDefinition", size)?;
        s.serialize_field("serviceName", &self.service_name)?;
        if skip_endpoints {
            s.skip_field("endpoints")?;
        } else {
            s.serialize_field("endpoints", &self.endpoints)?;
        }
        if skip_docs {
            s.skip_field("docs")?;
        } else {
            s.serialize_field("docs", &self.docs)?;
        }
        s.end()
    }
}
impl<'de> de::Deserialize<'de> for ServiceDefinition {
    fn deserialize<D>(d: D) -> Result<ServiceDefinition, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct(
            "ServiceDefinition",
            &["serviceName", "endpoints", "docs"],
            Visitor_,
        )
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = ServiceDefinition;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<ServiceDefinition, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut service_name = None;
        let mut endpoints = None;
        let mut docs = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::ServiceName => service_name = Some(map_.next_value()?),
                Field_::Endpoints => endpoints = Some(map_.next_value()?),
                Field_::Docs => docs = Some(map_.next_value()?),
                Field_::Unknown_ => {
                    map_.next_value::<de::IgnoredAny>()?;
                }
            }
        }
        let service_name = match service_name {
            Some(v) => v,
            None => return Err(de::Error::missing_field("serviceName")),
        };
        let endpoints = match endpoints {
            Some(v) => v,
            None => Default::default(),
        };
        let docs = match docs {
            Some(v) => v,
            None => Default::default(),
        };
        Ok(ServiceDefinition {
            service_name,
            endpoints,
            docs,
        })
    }
}
enum Field_ {
    ServiceName,
    Endpoints,
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
            "serviceName" => Field_::ServiceName,
            "endpoints" => Field_::Endpoints,
            "docs" => Field_::Docs,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
