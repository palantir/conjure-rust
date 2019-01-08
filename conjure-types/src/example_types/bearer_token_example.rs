use crate::serde::ser::SerializeMap as SerializeMap_;
use crate::serde::{de, ser};
use std::fmt;
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct BearerTokenExample {
    bearer_token_value: crate::BearerToken,
}
impl BearerTokenExample {
    #[doc = r" Returns a new builder."]
    #[inline]
    pub fn builder() -> Builder {
        Default::default()
    }
    #[inline]
    pub fn bearer_token_value(&self) -> &crate::BearerToken {
        &self.bearer_token_value
    }
}
#[derive(Debug, Clone, Default)]
pub struct Builder {
    bearer_token_value: Option<crate::BearerToken>,
}
impl Builder {
    #[inline]
    pub fn bearer_token_value(&mut self, bearer_token_value: crate::BearerToken) -> &mut Self {
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
    fn serialize<S_>(&self, s: S_) -> Result<S_::Ok, S_::Error>
    where
        S_: ser::Serializer,
    {
        let size = 1usize;
        let mut map = s.serialize_map(Some(size))?;
        map.serialize_entry(&"bearerTokenValue", &self.bearer_token_value)?;
        map.end()
    }
}
impl<'de> de::Deserialize<'de> for BearerTokenExample {
    fn deserialize<D_>(d: D_) -> Result<BearerTokenExample, D_::Error>
    where
        D_: de::Deserializer<'de>,
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
    fn visit_map<A_>(self, mut map_: A_) -> Result<BearerTokenExample, A_::Error>
    where
        A_: de::MapAccess<'de>,
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
    fn deserialize<D_>(d: D_) -> Result<Field_, D_::Error>
    where
        D_: de::Deserializer<'de>,
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
    fn visit_str<E_>(self, value: &str) -> Result<Field_, E_>
    where
        E_: de::Error,
    {
        let v = match value {
            "bearerTokenValue" => Field_::BearerTokenValue,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
