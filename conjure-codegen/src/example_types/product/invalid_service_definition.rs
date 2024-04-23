use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
///Invalid Conjure service definition.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[conjure_object::private::staged_builder::staged_builder]
#[builder(crate = conjure_object::private::staged_builder, update, inline)]
pub struct InvalidServiceDefinition {
    #[builder(into)]
    service_name: String,
    #[builder(
        custom(
            type = impl
            conjure_object::serde::Serialize,
            convert = |v|conjure_object::Any::new(v).expect("value failed to serialize")
        )
    )]
    service_def: conjure_object::Any,
}
impl InvalidServiceDefinition {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(
        service_name: impl Into<String>,
        service_def: impl conjure_object::serde::Serialize,
    ) -> Self {
        Self::builder().service_name(service_name).service_def(service_def).build()
    }
    ///Name of the invalid service definition.
    #[inline]
    pub fn service_name(&self) -> &str {
        &*self.service_name
    }
    ///Details of the invalid service definition.
    #[inline]
    pub fn service_def(&self) -> &conjure_object::Any {
        &self.service_def
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
