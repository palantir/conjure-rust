use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
///A generic `FAILED_PRECONDITION` error.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
pub struct FailedPrecondition {}
impl FailedPrecondition {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new() -> FailedPrecondition {
        FailedPrecondition {}
    }
    /// Returns a new builder.
    #[inline]
    pub fn builder() -> BuilderStage0 {
        Default::default()
    }
}
impl Default for BuilderStage0 {
    #[inline]
    fn default() -> Self {
        BuilderStage0 {}
    }
}
impl From<FailedPrecondition> for BuilderStage0 {
    #[inline]
    fn from(_: FailedPrecondition) -> Self {
        BuilderStage0 {}
    }
}
///The stage 0 builder for the [`FailedPrecondition`] type
#[derive(Debug, Clone)]
pub struct BuilderStage0 {}
impl BuilderStage0 {
    /// Consumes the builder, constructing a new instance of the type.
    #[inline]
    pub fn build(self) -> FailedPrecondition {
        FailedPrecondition {}
    }
}
impl ser::Serialize for FailedPrecondition {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let size = 0usize;
        let s = s.serialize_struct("FailedPrecondition", size)?;
        s.end()
    }
}
impl<'de> de::Deserialize<'de> for FailedPrecondition {
    fn deserialize<D>(d: D) -> Result<FailedPrecondition, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct("FailedPrecondition", &[], Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = FailedPrecondition;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<FailedPrecondition, A::Error>
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
        Ok(FailedPrecondition {})
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
impl conjure_error::ErrorType for FailedPrecondition {
    #[inline]
    fn code(&self) -> conjure_error::ErrorCode {
        conjure_error::ErrorCode::FailedPrecondition
    }
    #[inline]
    fn name(&self) -> &str {
        "Default:FailedPrecondition"
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
