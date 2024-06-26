use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, conjure_object::private::Educe, Copy)]
#[educe(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[conjure_object::private::staged_builder::staged_builder]
#[builder(crate = conjure_object::private::staged_builder, update, inline)]
pub struct DoubleExample {
    #[educe(
        PartialEq(method(conjure_object::private::DoubleOps::eq)),
        Ord(method(conjure_object::private::DoubleOps::cmp)),
        Hash(method(conjure_object::private::DoubleOps::hash)),
    )]
    double_value: f64,
}
impl DoubleExample {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(double_value: f64) -> Self {
        Self::builder().double_value(double_value).build()
    }
    #[inline]
    pub fn double_value(&self) -> f64 {
        self.double_value
    }
}
impl ser::Serialize for DoubleExample {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let size = 1usize;
        let mut s = s.serialize_struct("DoubleExample", size)?;
        s.serialize_field("doubleValue", &self.double_value)?;
        s.end()
    }
}
impl<'de> de::Deserialize<'de> for DoubleExample {
    fn deserialize<D>(d: D) -> Result<DoubleExample, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct("DoubleExample", &["doubleValue"], Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = DoubleExample;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<DoubleExample, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut double_value = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::DoubleValue => double_value = Some(map_.next_value()?),
                Field_::Unknown_ => {
                    map_.next_value::<de::IgnoredAny>()?;
                }
            }
        }
        let double_value = match double_value {
            Some(v) => v,
            None => return Err(de::Error::missing_field("doubleValue")),
        };
        Ok(DoubleExample { double_value })
    }
}
enum Field_ {
    DoubleValue,
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
            "doubleValue" => Field_::DoubleValue,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
