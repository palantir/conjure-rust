use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
///Invalid Conjure type definition.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[conjure_object::private::staged_builder::staged_builder]
#[builder(crate = conjure_object::private::staged_builder, update, inline)]
pub struct InvalidTypeDefinition {
    #[builder(into)]
    type_name: String,
    #[builder(
        custom(
            type = impl
            conjure_object::serde::Serialize,
            convert = |v|conjure_object::Any::new(v).expect("value failed to serialize")
        )
    )]
    type_def: conjure_object::Any,
}
impl InvalidTypeDefinition {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(
        type_name: impl Into<String>,
        type_def: impl conjure_object::serde::Serialize,
    ) -> Self {
        Self::builder().type_name(type_name).type_def(type_def).build()
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
