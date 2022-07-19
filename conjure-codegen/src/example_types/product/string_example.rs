use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StringExample {
    string: String,
}
impl StringExample {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new<T>(string: T) -> StringExample
    where
        T: Into<String>,
    {
        StringExample {
            string: string.into(),
        }
    }
    /// Returns a new builder.
    #[inline]
    pub fn builder() -> Builder {
        Default::default()
    }
    #[inline]
    pub fn string(&self) -> &str {
        &*self.string
    }
}
///A builder for the `StringExample` type.
#[derive(Debug, Clone, Default)]
pub struct Builder {
    string: Option<String>,
}
impl Builder {
    ///
    /// Required.
    #[inline]
    pub fn string<T>(&mut self, string: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.string = Some(string.into());
        self
    }
    /// Constructs a new instance of the type.
    ///
    /// # Panics
    ///
    /// Panics if a required field was not set.
    #[inline]
    pub fn build(&self) -> StringExample {
        StringExample {
            string: self.string.clone().expect("field string was not set"),
        }
    }
}
impl From<StringExample> for Builder {
    #[inline]
    fn from(_v: StringExample) -> Builder {
        Builder { string: Some(_v.string) }
    }
}
impl ser::Serialize for StringExample {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let size = 1usize;
        let mut s = s.serialize_struct("StringExample", size)?;
        s.serialize_field("string", &self.string)?;
        s.end()
    }
}
impl<'de> de::Deserialize<'de> for StringExample {
    fn deserialize<D>(d: D) -> Result<StringExample, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct("StringExample", &["string"], Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = StringExample;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<StringExample, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut string = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::String => string = Some(map_.next_value()?),
                Field_::Unknown_ => {
                    map_.next_value::<de::IgnoredAny>()?;
                }
            }
        }
        let string = match string {
            Some(v) => v,
            None => return Err(de::Error::missing_field("string")),
        };
        Ok(StringExample { string })
    }
}
enum Field_ {
    String,
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
            "string" => Field_::String,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
