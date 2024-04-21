use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BinaryExample {
    binary: conjure_object::Bytes,
}
impl BinaryExample {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new<T>(binary: T) -> BinaryExample
    where
        T: Into<conjure_object::Bytes>,
    {
        BinaryExample {
            binary: binary.into(),
        }
    }
    /// Returns a new builder.
    #[inline]
    pub fn builder() -> BuilderStage0 {
        Default::default()
    }
    #[inline]
    pub fn binary(&self) -> &conjure_object::Bytes {
        &self.binary
    }
}
impl Default for BuilderStage0 {
    #[inline]
    fn default() -> Self {
        BuilderStage0 {}
    }
}
impl From<BinaryExample> for BuilderStage1 {
    #[inline]
    fn from(value: BinaryExample) -> Self {
        BuilderStage1 {
            binary: value.binary,
        }
    }
}
///The stage 0 builder for the [`BinaryExample`] type
#[derive(Debug, Clone)]
pub struct BuilderStage0 {}
impl BuilderStage0 {
    #[inline]
    pub fn binary<T>(self, binary: T) -> BuilderStage1
    where
        T: Into<conjure_object::Bytes>,
    {
        BuilderStage1 {
            binary: binary.into(),
        }
    }
}
///The stage 1 builder for the [`BinaryExample`] type
#[derive(Debug, Clone)]
pub struct BuilderStage1 {
    binary: conjure_object::Bytes,
}
impl BuilderStage1 {
    #[inline]
    pub fn binary<T>(mut self, binary: T) -> Self
    where
        T: Into<conjure_object::Bytes>,
    {
        self.binary = binary.into();
        self
    }
    /// Consumes the builder, constructing a new instance of the type.
    #[inline]
    pub fn build(self) -> BinaryExample {
        BinaryExample {
            binary: self.binary,
        }
    }
}
impl ser::Serialize for BinaryExample {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let size = 1usize;
        let mut s = s.serialize_struct("BinaryExample", size)?;
        s.serialize_field("binary", &self.binary)?;
        s.end()
    }
}
impl<'de> de::Deserialize<'de> for BinaryExample {
    fn deserialize<D>(d: D) -> Result<BinaryExample, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct("BinaryExample", &["binary"], Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = BinaryExample;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<BinaryExample, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut binary = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::Binary => binary = Some(map_.next_value()?),
                Field_::Unknown_ => {
                    map_.next_value::<de::IgnoredAny>()?;
                }
            }
        }
        let binary = match binary {
            Some(v) => v,
            None => return Err(de::Error::missing_field("binary")),
        };
        Ok(BinaryExample { binary })
    }
}
enum Field_ {
    Binary,
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
            "binary" => Field_::Binary,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
