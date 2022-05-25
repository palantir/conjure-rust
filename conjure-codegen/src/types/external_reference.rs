use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ExternalReference {
    external_reference: Box<super::TypeName>,
    fallback: Box<super::Type>,
}
impl ExternalReference {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(
        external_reference: super::TypeName,
        fallback: super::Type,
    ) -> ExternalReference {
        ExternalReference {
            external_reference: Box::new(external_reference),
            fallback: Box::new(fallback),
        }
    }
    /// Returns a new builder.
    #[inline]
    pub fn builder() -> Builder {
        Default::default()
    }
    ///An identifier for a non-Conjure type which is already defined in a different language (e.g. Java).
    #[inline]
    pub fn external_reference(&self) -> &super::TypeName {
        &*self.external_reference
    }
    ///Other language generators may use the provided fallback if the non-Conjure type is not available. The ANY PrimitiveType is permissible for all external types, but a more specific definition is preferable.
    #[inline]
    pub fn fallback(&self) -> &super::Type {
        &*self.fallback
    }
}
///A builder for the `ExternalReference` type.
#[derive(Debug, Clone, Default)]
pub struct Builder {
    external_reference: Option<Box<super::TypeName>>,
    fallback: Option<Box<super::Type>>,
}
impl Builder {
    ///An identifier for a non-Conjure type which is already defined in a different language (e.g. Java).
    ///
    /// Required.
    #[inline]
    pub fn external_reference(
        &mut self,
        external_reference: super::TypeName,
    ) -> &mut Self {
        self.external_reference = Some(Box::new(external_reference));
        self
    }
    ///Other language generators may use the provided fallback if the non-Conjure type is not available. The ANY PrimitiveType is permissible for all external types, but a more specific definition is preferable.
    ///
    /// Required.
    #[inline]
    pub fn fallback(&mut self, fallback: super::Type) -> &mut Self {
        self.fallback = Some(Box::new(fallback));
        self
    }
    /// Constructs a new instance of the type.
    ///
    /// # Panics
    ///
    /// Panics if a required field was not set.
    #[inline]
    pub fn build(&self) -> ExternalReference {
        ExternalReference {
            external_reference: self
                .external_reference
                .clone()
                .expect("field external_reference was not set"),
            fallback: self.fallback.clone().expect("field fallback was not set"),
        }
    }
}
impl From<ExternalReference> for Builder {
    #[inline]
    fn from(_v: ExternalReference) -> Builder {
        Builder {
            external_reference: Some(_v.external_reference),
            fallback: Some(_v.fallback),
        }
    }
}
impl ser::Serialize for ExternalReference {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let size = 2usize;
        let mut s = s.serialize_struct("ExternalReference", size)?;
        s.serialize_field("externalReference", &self.external_reference)?;
        s.serialize_field("fallback", &self.fallback)?;
        s.end()
    }
}
impl<'de> de::Deserialize<'de> for ExternalReference {
    fn deserialize<D>(d: D) -> Result<ExternalReference, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct(
            "ExternalReference",
            &["externalReference", "fallback"],
            Visitor_,
        )
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = ExternalReference;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<ExternalReference, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut external_reference = None;
        let mut fallback = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::ExternalReference => {
                    external_reference = Some(map_.next_value()?);
                }
                Field_::Fallback => fallback = Some(map_.next_value()?),
                Field_::Unknown_ => {
                    map_.next_value::<de::IgnoredAny>()?;
                }
            }
        }
        let external_reference = match external_reference {
            Some(v) => v,
            None => return Err(de::Error::missing_field("externalReference")),
        };
        let fallback = match fallback {
            Some(v) => v,
            None => return Err(de::Error::missing_field("fallback")),
        };
        Ok(ExternalReference {
            external_reference,
            fallback,
        })
    }
}
enum Field_ {
    ExternalReference,
    Fallback,
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
            "externalReference" => Field_::ExternalReference,
            "fallback" => Field_::Fallback,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
