use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
#[conjure_object::private::staged_builder::staged_builder]
#[builder(crate = conjure_object::private::staged_builder, update, inline)]
pub struct IntegerExample {
    #[builder()]
    integer: i32,
}
impl IntegerExample {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(integer: i32) -> Self {
        Self::builder().integer(integer).build()
    }
    #[inline]
    pub fn integer(&self) -> i32 {
        self.integer
    }
}
impl ser::Serialize for IntegerExample {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let size = 1usize;
        let mut s = s.serialize_struct("IntegerExample", size)?;
        s.serialize_field("integer", &self.integer)?;
        s.end()
    }
}
impl<'de> de::Deserialize<'de> for IntegerExample {
    fn deserialize<D>(d: D) -> Result<IntegerExample, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct("IntegerExample", &["integer"], Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = IntegerExample;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<IntegerExample, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut integer = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::Integer => integer = Some(map_.next_value()?),
                Field_::Unknown_ => {
                    map_.next_value::<de::IgnoredAny>()?;
                }
            }
        }
        let integer = match integer {
            Some(v) => v,
            None => return Err(de::Error::missing_field("integer")),
        };
        Ok(IntegerExample { integer })
    }
}
enum Field_ {
    Integer,
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
            "integer" => Field_::Integer,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
