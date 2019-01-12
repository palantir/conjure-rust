use conjure_types::serde::ser::SerializeMap as SerializeMap_;
use conjure_types::serde::{de, ser};
use std::fmt;
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct StringExample {
    string: String,
}
impl StringExample {
    #[doc = r" Returns a new builder."]
    #[inline]
    pub fn builder() -> Builder {
        Default::default()
    }
    #[inline]
    pub fn string(&self) -> &str {
        &*self.string
    }
}
#[derive(Debug, Clone, Default)]
pub struct Builder {
    string: Option<String>,
}
impl Builder {
    #[doc = r""]
    #[doc = r" Required."]
    pub fn string<T>(&mut self, string: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.string = Some(string.into());
        self
    }
    #[doc = r" Constructs a new instance of the type."]
    #[doc = r""]
    #[doc = r" # Panics"]
    #[doc = r""]
    #[doc = r" Panics if a required field was not set."]
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
        Builder {
            string: Some(_v.string),
        }
    }
}
impl ser::Serialize for StringExample {
    fn serialize<S_>(&self, s: S_) -> Result<S_::Ok, S_::Error>
    where
        S_: ser::Serializer,
    {
        let size = 1usize;
        let mut map = s.serialize_map(Some(size))?;
        map.serialize_entry(&"string", &self.string)?;
        map.end()
    }
}
impl<'de> de::Deserialize<'de> for StringExample {
    fn deserialize<D_>(d: D_) -> Result<StringExample, D_::Error>
    where
        D_: de::Deserializer<'de>,
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
    fn visit_map<A_>(self, mut map_: A_) -> Result<StringExample, A_::Error>
    where
        A_: de::MapAccess<'de>,
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
    fn deserialize<D_>(d: D_) -> Result<Field_, D_::Error>
    where
        D_: de::Deserializer<'de>,
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
    fn visit_str<E_>(self, value: &str) -> Result<Field_, E_>
    where
        E_: de::Error,
    {
        let v = match value {
            "string" => Field_::String,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
