use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[conjure_object::private::staged_builder::staged_builder]
#[builder(crate = conjure_object::private::staged_builder, update, inline)]
pub struct ServiceDefinition {
    #[builder(custom(type = super::TypeName, convert = Box::new))]
    service_name: Box<super::TypeName>,
    #[builder(default, list(item(type = super::EndpointDefinition)))]
    endpoints: Vec<super::EndpointDefinition>,
    #[builder(default, into)]
    docs: Option<super::Documentation>,
}
impl ServiceDefinition {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(service_name: super::TypeName) -> Self {
        Self::builder().service_name(service_name).build()
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
