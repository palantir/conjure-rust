use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use conjure_object::serde::{de, ser};
use std::fmt;
#[doc = "A generic `TIMEOUT` error."]
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Copy)]
pub struct Timeout {}
impl Timeout {
    #[doc = r" Constructs a new instance of the type."]
    #[inline]
    pub fn new() -> Timeout {
        Timeout {}
    }
    #[doc = r" Returns a new builder."]
    #[inline]
    pub fn builder() -> Builder {
        Default::default()
    }
}
#[derive(Debug, Clone, Default)]
pub struct Builder {}
impl Builder {
    #[doc = r" Constructs a new instance of the type."]
    #[doc = r""]
    #[doc = r" # Panics"]
    #[doc = r""]
    #[doc = r" Panics if a required field was not set."]
    #[inline]
    pub fn build(&self) -> Timeout {
        Timeout {}
    }
}
impl From<Timeout> for Builder {
    #[inline]
    fn from(_v: Timeout) -> Builder {
        Builder {}
    }
}
impl ser::Serialize for Timeout {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let size = 0usize;
        let s = s.serialize_struct("Timeout", size)?;
        s.end()
    }
}
impl<'de> de::Deserialize<'de> for Timeout {
    fn deserialize<D>(d: D) -> Result<Timeout, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct("Timeout", &[], Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = Timeout;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<Timeout, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::Unknown_ => {
                    map_.next_value::<de::IgnoredAny>()?;
                }
            }
        }
        Ok(Timeout {})
    }
}
enum Field_ {
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
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
impl conjure_error::ErrorType for Timeout {
    #[inline]
    fn code(&self) -> conjure_error::ErrorCode {
        conjure_error::ErrorCode::Timeout
    }
    #[inline]
    fn name(&self) -> &str {
        "Default:Timeout"
    }
    #[inline]
    fn instance_id(&self) -> Option<conjure_object::Uuid> {
        None
    }
    #[inline]
    fn safe_args(&self) -> &'static [&'static str] {
        &[]
    }
}
