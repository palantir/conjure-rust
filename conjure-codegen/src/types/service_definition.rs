use conjure_object::serde::ser::SerializeMap as SerializeMap_;
use conjure_object::serde::{de, ser};
use std::fmt;
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ServiceDefinition {
    service_name: Box<super::TypeName>,
    endpoints: Vec<super::EndpointDefinition>,
    docs: Option<super::Documentation>,
}
impl ServiceDefinition {
    #[doc = r" Returns a new builder."]
    #[inline]
    pub fn builder() -> Builder {
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
#[derive(Debug, Clone, Default)]
pub struct Builder {
    service_name: Option<Box<super::TypeName>>,
    endpoints: Vec<super::EndpointDefinition>,
    docs: Option<super::Documentation>,
}
impl Builder {
    #[doc = r""]
    #[doc = r" Required."]
    #[inline]
    pub fn service_name(&mut self, service_name: super::TypeName) -> &mut Self {
        self.service_name = Some(Box::new(service_name));
        self
    }
    pub fn endpoints<T>(&mut self, endpoints: T) -> &mut Self
    where
        T: IntoIterator<Item = super::EndpointDefinition>,
    {
        self.endpoints = endpoints.into_iter().collect();
        self
    }
    pub fn extend_endpoints<T>(&mut self, endpoints: T) -> &mut Self
    where
        T: IntoIterator<Item = super::EndpointDefinition>,
    {
        self.endpoints.extend(endpoints);
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
    pub fn build(&self) -> ServiceDefinition {
        ServiceDefinition {
            service_name: self
                .service_name
                .clone()
                .expect("field service_name was not set"),
            endpoints: self.endpoints.clone(),
            docs: self.docs.clone(),
        }
    }
}
impl From<ServiceDefinition> for Builder {
    #[inline]
    fn from(_v: ServiceDefinition) -> Builder {
        Builder {
            service_name: Some(_v.service_name),
            endpoints: _v.endpoints,
            docs: _v.docs,
        }
    }
}
impl ser::Serialize for ServiceDefinition {
    fn serialize<S_>(&self, s: S_) -> Result<S_::Ok, S_::Error>
    where
        S_: ser::Serializer,
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
        let mut map = s.serialize_map(Some(size))?;
        map.serialize_entry(&"serviceName", &self.service_name)?;
        if !skip_endpoints {
            map.serialize_entry(&"endpoints", &self.endpoints)?;
        }
        if !skip_docs {
            map.serialize_entry(&"docs", &self.docs)?;
        }
        map.end()
    }
}
impl<'de> de::Deserialize<'de> for ServiceDefinition {
    fn deserialize<D_>(d: D_) -> Result<ServiceDefinition, D_::Error>
    where
        D_: de::Deserializer<'de>,
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
    fn visit_map<A_>(self, mut map_: A_) -> Result<ServiceDefinition, A_::Error>
    where
        A_: de::MapAccess<'de>,
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
            "serviceName" => Field_::ServiceName,
            "endpoints" => Field_::Endpoints,
            "docs" => Field_::Docs,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
