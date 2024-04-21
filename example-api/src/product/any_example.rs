use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AnyExample {
    any: conjure_object::Any,
}
impl AnyExample {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new<T>(any: T) -> AnyExample
    where
        T: conjure_object::serde::Serialize,
    {
        AnyExample {
            any: conjure_object::Any::new(any).expect("value failed to serialize"),
        }
    }
    /// Returns a new builder.
    #[inline]
    pub fn builder() -> BuilderStage0 {
        Default::default()
    }
    #[inline]
    pub fn any(&self) -> &conjure_object::Any {
        &self.any
    }
}
impl Default for BuilderStage0 {
    #[inline]
    fn default() -> Self {
        BuilderStage0 {}
    }
}
impl From<AnyExample> for BuilderStage1 {
    #[inline]
    fn from(value: AnyExample) -> Self {
        BuilderStage1 { any: value.any }
    }
}
///The stage 0 builder for the [`AnyExample`] type
#[derive(Debug, Clone)]
pub struct BuilderStage0 {}
impl BuilderStage0 {
    #[inline]
    pub fn any<T>(self, any: T) -> BuilderStage1
    where
        T: conjure_object::serde::Serialize,
    {
        BuilderStage1 {
            any: conjure_object::Any::new(any).expect("value failed to serialize"),
        }
    }
}
///The stage 1 builder for the [`AnyExample`] type
#[derive(Debug, Clone)]
pub struct BuilderStage1 {
    any: conjure_object::Any,
}
impl BuilderStage1 {
    #[inline]
    pub fn any<T>(mut self, any: T) -> Self
    where
        T: conjure_object::serde::Serialize,
    {
        self.any = conjure_object::Any::new(any).expect("value failed to serialize");
        self
    }
    /// Consumes the builder, constructing a new instance of the type.
    #[inline]
    pub fn build(self) -> AnyExample {
        AnyExample { any: self.any }
    }
}
impl ser::Serialize for AnyExample {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let size = 1usize;
        let mut s = s.serialize_struct("AnyExample", size)?;
        s.serialize_field("any", &self.any)?;
        s.end()
    }
}
impl<'de> de::Deserialize<'de> for AnyExample {
    fn deserialize<D>(d: D) -> Result<AnyExample, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct("AnyExample", &["any"], Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = AnyExample;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<AnyExample, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut any = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::Any => any = Some(map_.next_value()?),
                Field_::Unknown_ => {
                    map_.next_value::<de::IgnoredAny>()?;
                }
            }
        }
        let any = match any {
            Some(v) => v,
            None => return Err(de::Error::missing_field("any")),
        };
        Ok(AnyExample { any })
    }
}
enum Field_ {
    Any,
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
            "any" => Field_::Any,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
