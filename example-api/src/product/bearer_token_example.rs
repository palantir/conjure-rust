use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[conjure_object::private::staged_builder::staged_builder]
#[builder(crate = conjure_object::private::staged_builder, update, inline)]
pub struct BearerTokenExample {
    bearer_token_value: conjure_object::BearerToken,
}
impl BearerTokenExample {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(bearer_token_value: conjure_object::BearerToken) -> Self {
        Self::builder().bearer_token_value(bearer_token_value).build()
    }
    #[inline]
    pub fn bearer_token_value(&self) -> &conjure_object::BearerToken {
        &self.bearer_token_value
    }
}
impl ser::Serialize for BearerTokenExample {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let size = 1usize;
        let mut s = s.serialize_struct("BearerTokenExample", size)?;
        s.serialize_field("bearerTokenValue", &self.bearer_token_value)?;
        s.end()
    }
}
impl<'de> de::Deserialize<'de> for BearerTokenExample {
    fn deserialize<D>(d: D) -> Result<BearerTokenExample, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct("BearerTokenExample", &["bearerTokenValue"], Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = BearerTokenExample;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<BearerTokenExample, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut bearer_token_value = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::BearerTokenValue => bearer_token_value = Some(map_.next_value()?),
                Field_::Unknown_ => {
                    map_.next_value::<de::IgnoredAny>()?;
                }
            }
        }
        let bearer_token_value = match bearer_token_value {
            Some(v) => v,
            None => return Err(de::Error::missing_field("bearerTokenValue")),
        };
        Ok(BearerTokenExample {
            bearer_token_value,
        })
    }
}
enum Field_ {
    BearerTokenValue,
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
            "bearerTokenValue" => Field_::BearerTokenValue,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
