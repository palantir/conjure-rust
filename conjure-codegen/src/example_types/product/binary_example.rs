use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[conjure_object::private::staged_builder::staged_builder]
#[builder(crate = conjure_object::private::staged_builder, update, inline)]
pub struct BinaryExample {
    #[builder(into)]
    binary: conjure_object::Bytes,
}
impl BinaryExample {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(binary: impl Into<conjure_object::Bytes>) -> Self {
        Self::builder().binary(binary).build()
    }
    #[inline]
    pub fn binary(&self) -> &conjure_object::Bytes {
        &self.binary
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
