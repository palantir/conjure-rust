use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use conjure_object::serde::{de, ser};
use std::fmt;
#[doc = "Invalid Conjure service definition."]
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct InvalidServiceDefinition {
    service_name: String,
    service_def: conjure_object::Any,
}
impl InvalidServiceDefinition {
    #[doc = r" Constructs a new instance of the type."]
    #[inline]
    pub fn new<T, U>(service_name: T, service_def: U) -> InvalidServiceDefinition
    where
        T: Into<String>,
        U: conjure_object::serde::Serialize,
    {
        InvalidServiceDefinition {
            service_name: service_name.into(),
            service_def: conjure_object::serde_value::to_value(service_def)
                .expect("value failed to serialize"),
        }
    }
    #[doc = r" Returns a new builder."]
    #[inline]
    pub fn builder() -> Builder {
        Default::default()
    }
    #[doc = "Name of the invalid service definition."]
    #[inline]
    pub fn service_name(&self) -> &str {
        &*self.service_name
    }
    #[doc = "Details of the invalid service definition."]
    #[inline]
    pub fn service_def(&self) -> &conjure_object::Any {
        &self.service_def
    }
}
#[doc = "A builder for the `InvalidServiceDefinition` type."]
#[derive(Debug, Clone, Default)]
pub struct Builder {
    service_name: Option<String>,
    service_def: Option<conjure_object::Any>,
}
impl Builder {
    #[doc = "Name of the invalid service definition."]
    #[doc = r""]
    #[doc = r" Required."]
    pub fn service_name<T>(&mut self, service_name: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.service_name = Some(service_name.into());
        self
    }
    #[doc = "Details of the invalid service definition."]
    #[doc = r""]
    #[doc = r" Required."]
    pub fn service_def<T>(&mut self, service_def: T) -> &mut Self
    where
        T: conjure_object::serde::Serialize,
    {
        self.service_def = Some(
            conjure_object::serde_value::to_value(service_def).expect("value failed to serialize"),
        );
        self
    }
    #[doc = r" Constructs a new instance of the type."]
    #[doc = r""]
    #[doc = r" # Panics"]
    #[doc = r""]
    #[doc = r" Panics if a required field was not set."]
    #[inline]
    pub fn build(&self) -> InvalidServiceDefinition {
        InvalidServiceDefinition {
            service_name: self
                .service_name
                .clone()
                .expect("field service_name was not set"),
            service_def: self
                .service_def
                .clone()
                .expect("field service_def was not set"),
        }
    }
}
impl From<InvalidServiceDefinition> for Builder {
    #[inline]
    fn from(_v: InvalidServiceDefinition) -> Builder {
        Builder {
            service_name: Some(_v.service_name),
            service_def: Some(_v.service_def),
        }
    }
}
impl ser::Serialize for InvalidServiceDefinition {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let size = 2usize;
        let mut s = s.serialize_struct("InvalidServiceDefinition", size)?;
        s.serialize_field("serviceName", &self.service_name)?;
        s.serialize_field("serviceDef", &self.service_def)?;
        s.end()
    }
}
impl<'de> de::Deserialize<'de> for InvalidServiceDefinition {
    fn deserialize<D>(d: D) -> Result<InvalidServiceDefinition, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct(
            "InvalidServiceDefinition",
            &["serviceName", "serviceDef"],
            Visitor_,
        )
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = InvalidServiceDefinition;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<InvalidServiceDefinition, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut service_name = None;
        let mut service_def = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::ServiceName => service_name = Some(map_.next_value()?),
                Field_::ServiceDef => service_def = Some(map_.next_value()?),
                Field_::Unknown_ => {
                    map_.next_value::<de::IgnoredAny>()?;
                }
            }
        }
        let service_name = match service_name {
            Some(v) => v,
            None => return Err(de::Error::missing_field("serviceName")),
        };
        let service_def = match service_def {
            Some(v) => v,
            None => return Err(de::Error::missing_field("serviceDef")),
        };
        Ok(InvalidServiceDefinition {
            service_name,
            service_def,
        })
    }
}
enum Field_ {
    ServiceName,
    ServiceDef,
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
            "serviceDef" => Field_::ServiceDef,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
impl conjure_error::ErrorType for InvalidServiceDefinition {
    #[inline]
    fn code(&self) -> conjure_error::ErrorCode {
        conjure_error::ErrorCode::InvalidArgument
    }
    #[inline]
    fn name(&self) -> &str {
        "Conjure:InvalidServiceDefinition"
    }
    #[inline]
    fn instance_id(&self) -> Option<conjure_object::Uuid> {
        None
    }
    #[inline]
    fn safe_args(&self) -> &'static [&'static str] {
        &["serviceName"]
    }
}
