use conjure_types::serde::ser::SerializeMap as SerializeMap_;
use conjure_types::serde::{de, ser};
use std::fmt;
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Copy)]
pub struct IntegerExample {
    integer: i32,
}
impl IntegerExample {
    #[doc = r" Returns a new builder."]
    #[inline]
    pub fn builder() -> Builder {
        Default::default()
    }
    #[inline]
    pub fn integer(&self) -> i32 {
        self.integer
    }
}
#[derive(Debug, Clone, Default)]
pub struct Builder {
    integer: Option<i32>,
}
impl Builder {
    #[doc = r""]
    #[doc = r" Required."]
    #[inline]
    pub fn integer(&mut self, integer: i32) -> &mut Self {
        self.integer = Some(integer);
        self
    }
    #[doc = r" Constructs a new instance of the type."]
    #[doc = r""]
    #[doc = r" # Panics"]
    #[doc = r""]
    #[doc = r" Panics if a required field was not set."]
    #[inline]
    pub fn build(&self) -> IntegerExample {
        IntegerExample {
            integer: self.integer.clone().expect("field integer was not set"),
        }
    }
}
impl From<IntegerExample> for Builder {
    #[inline]
    fn from(_v: IntegerExample) -> Builder {
        Builder {
            integer: Some(_v.integer),
        }
    }
}
impl ser::Serialize for IntegerExample {
    fn serialize<S_>(&self, s: S_) -> Result<S_::Ok, S_::Error>
    where
        S_: ser::Serializer,
    {
        let size = 1usize;
        let mut map = s.serialize_map(Some(size))?;
        map.serialize_entry(&"integer", &self.integer)?;
        map.end()
    }
}
impl<'de> de::Deserialize<'de> for IntegerExample {
    fn deserialize<D_>(d: D_) -> Result<IntegerExample, D_::Error>
    where
        D_: de::Deserializer<'de>,
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
    fn visit_map<A_>(self, mut map_: A_) -> Result<IntegerExample, A_::Error>
    where
        A_: de::MapAccess<'de>,
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
            "integer" => Field_::Integer,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
