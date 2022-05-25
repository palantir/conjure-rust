use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
///Invalid Conjure type definition.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct InvalidTypeDefinition {
    type_name: String,
    type_def: conjure_object::Any,
}
impl InvalidTypeDefinition {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new<T, U>(type_name: T, type_def: U) -> InvalidTypeDefinition
    where
        T: Into<String>,
        U: conjure_object::serde::Serialize,
    {
        InvalidTypeDefinition {
            type_name: type_name.into(),
            type_def: conjure_object::Any::new(type_def)
                .expect("value failed to serialize"),
        }
    }
    /// Returns a new builder.
    #[inline]
    pub fn builder() -> Builder {
        Default::default()
    }
    #[inline]
    pub fn type_name(&self) -> &str {
        &*self.type_name
    }
    #[inline]
    pub fn type_def(&self) -> &conjure_object::Any {
        &self.type_def
    }
}
///A builder for the `InvalidTypeDefinition` type.
#[derive(Debug, Clone, Default)]
pub struct Builder {
    type_name: Option<String>,
    type_def: Option<conjure_object::Any>,
}
impl Builder {
    ///
    /// Required.
    #[inline]
    pub fn type_name<T>(&mut self, type_name: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.type_name = Some(type_name.into());
        self
    }
    ///
    /// Required.
    #[inline]
    pub fn type_def<T>(&mut self, type_def: T) -> &mut Self
    where
        T: conjure_object::serde::Serialize,
    {
        self
            .type_def = Some(
            conjure_object::Any::new(type_def).expect("value failed to serialize"),
        );
        self
    }
    /// Constructs a new instance of the type.
    ///
    /// # Panics
    ///
    /// Panics if a required field was not set.
    #[inline]
    pub fn build(&self) -> InvalidTypeDefinition {
        InvalidTypeDefinition {
            type_name: self.type_name.clone().expect("field type_name was not set"),
            type_def: self.type_def.clone().expect("field type_def was not set"),
        }
    }
}
impl From<InvalidTypeDefinition> for Builder {
    #[inline]
    fn from(_v: InvalidTypeDefinition) -> Builder {
        Builder {
            type_name: Some(_v.type_name),
            type_def: Some(_v.type_def),
        }
    }
}
impl ser::Serialize for InvalidTypeDefinition {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let size = 2usize;
        let mut s = s.serialize_struct("InvalidTypeDefinition", size)?;
        s.serialize_field("typeName", &self.type_name)?;
        s.serialize_field("typeDef", &self.type_def)?;
        s.end()
    }
}
impl<'de> de::Deserialize<'de> for InvalidTypeDefinition {
    fn deserialize<D>(d: D) -> Result<InvalidTypeDefinition, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct("InvalidTypeDefinition", &["typeName", "typeDef"], Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = InvalidTypeDefinition;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<InvalidTypeDefinition, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut type_name = None;
        let mut type_def = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::TypeName => type_name = Some(map_.next_value()?),
                Field_::TypeDef => type_def = Some(map_.next_value()?),
                Field_::Unknown_ => {
                    map_.next_value::<de::IgnoredAny>()?;
                }
            }
        }
        let type_name = match type_name {
            Some(v) => v,
            None => return Err(de::Error::missing_field("typeName")),
        };
        let type_def = match type_def {
            Some(v) => v,
            None => return Err(de::Error::missing_field("typeDef")),
        };
        Ok(InvalidTypeDefinition {
            type_name,
            type_def,
        })
    }
}
enum Field_ {
    TypeName,
    TypeDef,
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
            "typeName" => Field_::TypeName,
            "typeDef" => Field_::TypeDef,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
impl conjure_error::ErrorType for InvalidTypeDefinition {
    #[inline]
    fn code(&self) -> conjure_error::ErrorCode {
        conjure_error::ErrorCode::InvalidArgument
    }
    #[inline]
    fn name(&self) -> &str {
        "Conjure:InvalidTypeDefinition"
    }
    #[inline]
    fn instance_id(&self) -> Option<conjure_object::Uuid> {
        None
    }
    #[inline]
    fn safe_args(&self) -> &'static [&'static str] {
        &["typeName"]
    }
}
