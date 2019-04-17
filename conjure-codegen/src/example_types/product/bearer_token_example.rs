use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use conjure_object::serde::{de, ser};
use std::fmt;
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct BearerTokenExample {
    bearer_token_value: conjure_object::BearerToken,
}
impl BearerTokenExample {
    #[doc = r" Constructs a new instance of the type."]
    #[inline]
    pub fn new(bearer_token_value: conjure_object::BearerToken) -> BearerTokenExample {
        BearerTokenExample {
            bearer_token_value: bearer_token_value,
        }
    }
    #[doc = r" Returns a new builder."]
    #[inline]
    pub fn builder() -> Builder {
        Default::default()
    }
    #[inline]
    pub fn bearer_token_value(&self) -> &conjure_object::BearerToken {
        &self.bearer_token_value
    }
}
#[derive(Debug, Clone, Default)]
pub struct Builder {
    bearer_token_value: Option<conjure_object::BearerToken>,
}
impl Builder {
    #[doc = r""]
    #[doc = r" Required."]
    #[inline]
    pub fn bearer_token_value(
        &mut self,
        bearer_token_value: conjure_object::BearerToken,
    ) -> &mut Self {
        self.bearer_token_value = Some(bearer_token_value);
        self
    }
    #[doc = r" Constructs a new instance of the type."]
    #[doc = r""]
    #[doc = r" # Panics"]
    #[doc = r""]
    #[doc = r" Panics if a required field was not set."]
    #[inline]
    pub fn build(&self) -> BearerTokenExample {
        BearerTokenExample {
            bearer_token_value: self
                .bearer_token_value
                .clone()
                .expect("field bearer_token_value was not set"),
        }
    }
}
impl From<BearerTokenExample> for Builder {
    #[inline]
    fn from(_v: BearerTokenExample) -> Builder {
        Builder {
            bearer_token_value: Some(_v.bearer_token_value),
        }
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
        Ok(BearerTokenExample { bearer_token_value })
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
