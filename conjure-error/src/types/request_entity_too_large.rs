use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
///A generic `REQUEST_ENTITY_TOO_LARGE` error.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Copy)]
pub struct RequestEntityTooLarge {}
impl RequestEntityTooLarge {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new() -> RequestEntityTooLarge {
        RequestEntityTooLarge {}
    }
    /// Returns a new builder.
    #[inline]
    pub fn builder() -> Builder {
        Default::default()
    }
}
///A builder for the `RequestEntityTooLarge` type.
#[derive(Debug, Clone, Default)]
pub struct Builder {}
impl Builder {
    /// Constructs a new instance of the type.
    ///
    /// # Panics
    ///
    /// Panics if a required field was not set.
    #[inline]
    pub fn build(&self) -> RequestEntityTooLarge {
        RequestEntityTooLarge {}
    }
}
impl From<RequestEntityTooLarge> for Builder {
    #[inline]
    fn from(_v: RequestEntityTooLarge) -> Builder {
        Builder {}
    }
}
impl ser::Serialize for RequestEntityTooLarge {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let size = 0usize;
        let s = s.serialize_struct("RequestEntityTooLarge", size)?;
        s.end()
    }
}
impl<'de> de::Deserialize<'de> for RequestEntityTooLarge {
    fn deserialize<D>(d: D) -> Result<RequestEntityTooLarge, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct("RequestEntityTooLarge", &[], Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = RequestEntityTooLarge;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<RequestEntityTooLarge, A::Error>
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
        Ok(RequestEntityTooLarge {})
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
impl conjure_error::ErrorType for RequestEntityTooLarge {
    #[inline]
    fn code(&self) -> conjure_error::ErrorCode {
        conjure_error::ErrorCode::RequestEntityTooLarge
    }
    #[inline]
    fn name(&self) -> &str {
        "Default:RequestEntityTooLarge"
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
