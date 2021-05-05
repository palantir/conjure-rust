use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use conjure_object::serde::{de, ser};
use std::fmt;
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ConjureDefinition {
    version: i32,
    errors: Vec<super::ErrorDefinition>,
    types: Vec<super::TypeDefinition>,
    services: Vec<super::ServiceDefinition>,
    extensions: std::collections::BTreeMap<String, conjure_object::Any>,
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
    #[inline]
    pub fn extensions(&self) -> &std::collections::BTreeMap<String, conjure_object::Any> {
        &self.extensions
    }
}
#[doc = "A builder for the `ConjureDefinition` type."]
#[derive(Debug, Clone, Default)]
pub struct Builder {
    version: Option<i32>,
    errors: Vec<super::ErrorDefinition>,
    types: Vec<super::TypeDefinition>,
    services: Vec<super::ServiceDefinition>,
    extensions: std::collections::BTreeMap<String, conjure_object::Any>,
}
impl Builder {
    #[doc = r""]
    #[doc = r" Required."]
    #[inline]
    pub fn version(&mut self, version: i32) -> &mut Self {
        self.version = Some(version);
        self
    }
    #[inline]
    pub fn errors<T>(&mut self, errors: T) -> &mut Self
    where
        T: IntoIterator<Item = super::ErrorDefinition>,
    {
        self.errors = errors.into_iter().collect();
        self
    }
    #[inline]
    pub fn extend_errors<T>(&mut self, errors: T) -> &mut Self
    where
        T: IntoIterator<Item = super::ErrorDefinition>,
    {
        self.errors.extend(errors);
        self
    }
    #[inline]
    pub fn push_errors(&mut self, value: super::ErrorDefinition) -> &mut Self {
        self.errors.push(value);
        self
    }
    #[inline]
    pub fn types<T>(&mut self, types: T) -> &mut Self
    where
        T: IntoIterator<Item = super::TypeDefinition>,
    {
        self.types = types.into_iter().collect();
        self
    }
    #[inline]
    pub fn extend_types<T>(&mut self, types: T) -> &mut Self
    where
        T: IntoIterator<Item = super::TypeDefinition>,
    {
        self.types.extend(types);
        self
    }
    #[inline]
    pub fn push_types(&mut self, value: super::TypeDefinition) -> &mut Self {
        self.types.push(value);
        self
    }
    #[inline]
    pub fn services<T>(&mut self, services: T) -> &mut Self
    where
        T: IntoIterator<Item = super::ServiceDefinition>,
    {
        self.services = services.into_iter().collect();
        self
    }
    #[inline]
    pub fn extend_services<T>(&mut self, services: T) -> &mut Self
    where
        T: IntoIterator<Item = super::ServiceDefinition>,
    {
        self.services.extend(services);
        self
    }
    #[inline]
    pub fn push_services(&mut self, value: super::ServiceDefinition) -> &mut Self {
        self.services.push(value);
        self
    }
    #[inline]
    pub fn extensions<T>(&mut self, extensions: T) -> &mut Self
    where
        T: IntoIterator<Item = (String, conjure_object::Any)>,
    {
        self.extensions = extensions.into_iter().collect();
        self
    }
    #[inline]
    pub fn extend_extensions<T>(&mut self, extensions: T) -> &mut Self
    where
        T: IntoIterator<Item = (String, conjure_object::Any)>,
    {
        self.extensions.extend(extensions);
        self
    }
    #[inline]
    pub fn insert_extensions<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: Into<String>,
        V: conjure_object::serde::Serialize,
    {
        self.extensions.insert(
            key.into(),
            conjure_object::Any::new(value).expect("value failed to serialize"),
        );
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
            extensions: self.extensions.clone(),
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
            extensions: _v.extensions,
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
        let skip_extensions = self.extensions.is_empty();
        if !skip_extensions {
            size += 1;
        }
        let mut s = s.serialize_struct("ConjureDefinition", size)?;
        s.serialize_field("version", &self.version)?;
        if skip_errors {
            s.skip_field("errors")?;
        } else {
            s.serialize_field("errors", &self.errors)?;
        }
        if skip_types {
            s.skip_field("types")?;
        } else {
            s.serialize_field("types", &self.types)?;
        }
        if skip_services {
            s.skip_field("services")?;
        } else {
            s.serialize_field("services", &self.services)?;
        }
        if skip_extensions {
            s.skip_field("extensions")?;
        } else {
            s.serialize_field("extensions", &self.extensions)?;
        }
        s.end()
    }
}
impl<'de> de::Deserialize<'de> for ConjureDefinition {
    fn deserialize<D>(d: D) -> Result<ConjureDefinition, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct(
            "ConjureDefinition",
            &["version", "errors", "types", "services", "extensions"],
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
        let mut extensions = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::Version => version = Some(map_.next_value()?),
                Field_::Errors => errors = Some(map_.next_value()?),
                Field_::Types => types = Some(map_.next_value()?),
                Field_::Services => services = Some(map_.next_value()?),
                Field_::Extensions => extensions = Some(map_.next_value()?),
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
        let extensions = match extensions {
            Some(v) => v,
            None => Default::default(),
        };
        Ok(ConjureDefinition {
            version,
            errors,
            types,
            services,
            extensions,
        })
    }
}
enum Field_ {
    Version,
    Errors,
    Types,
    Services,
    Extensions,
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
            "extensions" => Field_::Extensions,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
