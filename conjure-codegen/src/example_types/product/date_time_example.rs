use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use conjure_object::serde::{de, ser};
use std::fmt;
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Copy)]
pub struct DateTimeExample {
    datetime: conjure_object::DateTime<conjure_object::Utc>,
}
impl DateTimeExample {
    #[doc = r" Constructs a new instance of the type."]
    #[inline]
    pub fn new(datetime: conjure_object::DateTime<conjure_object::Utc>) -> DateTimeExample {
        DateTimeExample { datetime: datetime }
    }
    #[doc = r" Returns a new builder."]
    #[inline]
    pub fn builder() -> Builder {
        Default::default()
    }
    #[inline]
    pub fn datetime(&self) -> conjure_object::DateTime<conjure_object::Utc> {
        self.datetime
    }
}
#[derive(Debug, Clone, Default)]
pub struct Builder {
    datetime: Option<conjure_object::DateTime<conjure_object::Utc>>,
}
impl Builder {
    #[doc = r""]
    #[doc = r" Required."]
    #[inline]
    pub fn datetime(
        &mut self,
        datetime: conjure_object::DateTime<conjure_object::Utc>,
    ) -> &mut Self {
        self.datetime = Some(datetime);
        self
    }
    #[doc = r" Constructs a new instance of the type."]
    #[doc = r""]
    #[doc = r" # Panics"]
    #[doc = r""]
    #[doc = r" Panics if a required field was not set."]
    #[inline]
    pub fn build(&self) -> DateTimeExample {
        DateTimeExample {
            datetime: self.datetime.clone().expect("field datetime was not set"),
        }
    }
}
impl From<DateTimeExample> for Builder {
    #[inline]
    fn from(_v: DateTimeExample) -> Builder {
        Builder {
            datetime: Some(_v.datetime),
        }
    }
}
impl ser::Serialize for DateTimeExample {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut s = s.serialize_struct("DateTimeExample", 1usize)?;
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
