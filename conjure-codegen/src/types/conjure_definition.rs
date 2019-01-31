use conjure_object::serde::ser::SerializeMap as SerializeMap_;
use conjure_object::serde::{de, ser};
use std::fmt;
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ConjureDefinition {
    version: i32,
    errors: Vec<super::ErrorDefinition>,
    types: Vec<super::TypeDefinition>,
    services: Vec<super::ServiceDefinition>,
}
impl ConjureDefinition {
    #[doc = r" Returns a new builder."]
    #[inline]
    pub fn builder() -> Builder {
        Default::default()
    }
    #[inline]
    pub fn version(&self) -> i32 {
        self.version
    }
    #[inline]
    pub fn errors(&self) -> &[super::ErrorDefinition] {
        &*self.errors
    }
    #[inline]
    pub fn types(&self) -> &[super::TypeDefinition] {
        &*self.types
    }
    #[inline]
    pub fn services(&self) -> &[super::ServiceDefinition] {
        &*self.services
    }
}
#[derive(Debug, Clone, Default)]
pub struct Builder {
    version: Option<i32>,
    errors: Vec<super::ErrorDefinition>,
    types: Vec<super::TypeDefinition>,
    services: Vec<super::ServiceDefinition>,
}
impl Builder {
    #[doc = r""]
    #[doc = r" Required."]
    #[inline]
    pub fn version(&mut self, version: i32) -> &mut Self {
        self.version = Some(version);
        self
    }
    pub fn errors<T>(&mut self, errors: T) -> &mut Self
    where
        T: IntoIterator<Item = super::ErrorDefinition>,
    {
        self.errors = errors.into_iter().collect();
        self
    }
    pub fn extend_errors<T>(&mut self, errors: T) -> &mut Self
    where
        T: IntoIterator<Item = super::ErrorDefinition>,
    {
        self.errors.extend(errors);
        self
    }
    pub fn push_errors(&mut self, value: super::ErrorDefinition) -> &mut Self {
        self.errors.push(value);
        self
    }
    pub fn types<T>(&mut self, types: T) -> &mut Self
    where
        T: IntoIterator<Item = super::TypeDefinition>,
    {
        self.types = types.into_iter().collect();
        self
    }
    pub fn extend_types<T>(&mut self, types: T) -> &mut Self
    where
        T: IntoIterator<Item = super::TypeDefinition>,
    {
        self.types.extend(types);
        self
    }
    pub fn push_types(&mut self, value: super::TypeDefinition) -> &mut Self {
        self.types.push(value);
        self
    }
    pub fn services<T>(&mut self, services: T) -> &mut Self
    where
        T: IntoIterator<Item = super::ServiceDefinition>,
    {
        self.services = services.into_iter().collect();
        self
    }
    pub fn extend_services<T>(&mut self, services: T) -> &mut Self
    where
        T: IntoIterator<Item = super::ServiceDefinition>,
    {
        self.services.extend(services);
        self
    }
    pub fn push_services(&mut self, value: super::ServiceDefinition) -> &mut Self {
        self.services.push(value);
        self
    }
    #[doc = r" Constructs a new instance of the type."]
    #[doc = r""]
    #[doc = r" # Panics"]
    #[doc = r""]
    #[doc = r" Panics if a required field was not set."]
    #[inline]
    pub fn build(&self) -> ConjureDefinition {
        ConjureDefinition {
            version: self.version.clone().expect("field version was not set"),
            errors: self.errors.clone(),
            types: self.types.clone(),
            services: self.services.clone(),
        }
    }
}
impl From<ConjureDefinition> for Builder {
    #[inline]
    fn from(_v: ConjureDefinition) -> Builder {
        Builder {
            version: Some(_v.version),
            errors: _v.errors,
            types: _v.types,
            services: _v.services,
        }
    }
}
impl ser::Serialize for ConjureDefinition {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut size = 1usize;
        let skip_errors = self.errors.is_empty();
        if !skip_errors {
            size += 1;
        }
        let skip_types = self.types.is_empty();
        if !skip_types {
            size += 1;
        }
        let skip_services = self.services.is_empty();
        if !skip_services {
            size += 1;
        }
        let mut map = s.serialize_map(Some(size))?;
        map.serialize_entry(&"version", &self.version)?;
        if !skip_errors {
            map.serialize_entry(&"errors", &self.errors)?;
        }
        if !skip_types {
            map.serialize_entry(&"types", &self.types)?;
        }
        if !skip_services {
            map.serialize_entry(&"services", &self.services)?;
        }
        map.end()
    }
}
impl<'de> de::Deserialize<'de> for ConjureDefinition {
    fn deserialize<D>(d: D) -> Result<ConjureDefinition, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct(
            "ConjureDefinition",
            &["version", "errors", "types", "services"],
            Visitor_,
        )
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = ConjureDefinition;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<ConjureDefinition, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut version = None;
        let mut errors = None;
        let mut types = None;
        let mut services = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::Version => version = Some(map_.next_value()?),
                Field_::Errors => errors = Some(map_.next_value()?),
                Field_::Types => types = Some(map_.next_value()?),
                Field_::Services => services = Some(map_.next_value()?),
                Field_::Unknown_ => {
                    map_.next_value::<de::IgnoredAny>()?;
                }
            }
        }
        let version = match version {
            Some(v) => v,
            None => return Err(de::Error::missing_field("version")),
        };
        let errors = match errors {
            Some(v) => v,
            None => Default::default(),
        };
        let types = match types {
            Some(v) => v,
            None => Default::default(),
        };
        let services = match services {
            Some(v) => v,
            None => Default::default(),
        };
        Ok(ConjureDefinition {
            version,
            errors,
            types,
            services,
        })
    }
}
enum Field_ {
    Version,
    Errors,
    Types,
    Services,
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
            "version" => Field_::Version,
            "errors" => Field_::Errors,
            "types" => Field_::Types,
            "services" => Field_::Services,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
