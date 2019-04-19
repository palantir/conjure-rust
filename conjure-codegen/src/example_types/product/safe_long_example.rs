use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use conjure_object::serde::{de, ser};
use std::fmt;
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Copy)]
pub struct SafeLongExample {
    safe_long_value: conjure_object::SafeLong,
}
impl SafeLongExample {
    #[doc = r" Constructs a new instance of the type."]
    #[inline]
    pub fn new(safe_long_value: conjure_object::SafeLong) -> SafeLongExample {
        SafeLongExample {
            safe_long_value: safe_long_value,
        }
    }
    #[doc = r" Returns a new builder."]
    #[inline]
    pub fn builder() -> Builder {
        Default::default()
    }
    #[inline]
    pub fn safe_long_value(&self) -> conjure_object::SafeLong {
        self.safe_long_value
    }
}
#[doc = "A builder for the `SafeLongExample` type."]
#[derive(Debug, Clone, Default)]
pub struct Builder {
    safe_long_value: Option<conjure_object::SafeLong>,
}
impl Builder {
    #[doc = r""]
    #[doc = r" Required."]
    #[inline]
    pub fn safe_long_value(&mut self, safe_long_value: conjure_object::SafeLong) -> &mut Self {
        self.safe_long_value = Some(safe_long_value);
        self
    }
    #[doc = r" Constructs a new instance of the type."]
    #[doc = r""]
    #[doc = r" # Panics"]
    #[doc = r""]
    #[doc = r" Panics if a required field was not set."]
    #[inline]
    pub fn build(&self) -> SafeLongExample {
        SafeLongExample {
            safe_long_value: self
                .safe_long_value
                .clone()
                .expect("field safe_long_value was not set"),
        }
    }
}
impl From<SafeLongExample> for Builder {
    #[inline]
    fn from(_v: SafeLongExample) -> Builder {
        Builder {
            safe_long_value: Some(_v.safe_long_value),
        }
    }
}
impl ser::Serialize for SafeLongExample {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let size = 1usize;
        let mut s = s.serialize_struct("SafeLongExample", size)?;
        s.serialize_field("safeLongValue", &self.safe_long_value)?;
        s.end()
    }
}
impl<'de> de::Deserialize<'de> for SafeLongExample {
    fn deserialize<D>(d: D) -> Result<SafeLongExample, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct("SafeLongExample", &["safeLongValue"], Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = SafeLongExample;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<SafeLongExample, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut safe_long_value = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::SafeLongValue => safe_long_value = Some(map_.next_value()?),
                Field_::Unknown_ => {
                    map_.next_value::<de::IgnoredAny>()?;
                }
            }
        }
        let safe_long_value = match safe_long_value {
            Some(v) => v,
            None => return Err(de::Error::missing_field("safeLongValue")),
        };
        Ok(SafeLongExample { safe_long_value })
    }
}
enum Field_ {
    SafeLongValue,
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
            "safeLongValue" => Field_::SafeLongValue,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
