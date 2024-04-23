use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
#[conjure_object::private::staged_builder::staged_builder]
#[builder(crate = conjure_object::private::staged_builder, update, inline)]
pub struct DateTimeExample {
    #[builder()]
    datetime: conjure_object::DateTime<conjure_object::Utc>,
}
impl DateTimeExample {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(datetime: conjure_object::DateTime<conjure_object::Utc>) -> Self {
        Self::builder().datetime(datetime).build()
    }
    #[inline]
    pub fn datetime(&self) -> conjure_object::DateTime<conjure_object::Utc> {
        self.datetime
    }
}
impl ser::Serialize for DateTimeExample {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let size = 1usize;
        let mut s = s.serialize_struct("DateTimeExample", size)?;
        s.serialize_field("datetime", &self.datetime)?;
        s.end()
    }
}
impl<'de> de::Deserialize<'de> for DateTimeExample {
    fn deserialize<D>(d: D) -> Result<DateTimeExample, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct("DateTimeExample", &["datetime"], Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = DateTimeExample;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<DateTimeExample, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut datetime = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::Datetime => datetime = Some(map_.next_value()?),
                Field_::Unknown_ => {
                    map_.next_value::<de::IgnoredAny>()?;
                }
            }
        }
        let datetime = match datetime {
            Some(v) => v,
            None => return Err(de::Error::missing_field("datetime")),
        };
        Ok(DateTimeExample { datetime })
    }
}
enum Field_ {
    Datetime,
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
            "datetime" => Field_::Datetime,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
