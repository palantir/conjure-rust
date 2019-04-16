use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use conjure_object::serde::{de, ser};
use std::fmt;
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct AnyExample {
    any: conjure_object::Value,
}
impl AnyExample {
    #[doc = r" Constructs a new instance of the type."]
    #[inline]
    pub fn new<T>(any: T) -> AnyExample
    where
        T: conjure_object::serde::Serialize,
    {
        AnyExample {
            any: conjure_object::serde_value::to_value(any).expect("value failed to serialize"),
        }
    }
    #[doc = r" Returns a new builder."]
    #[inline]
    pub fn builder() -> Builder {
        Default::default()
    }
    #[inline]
    pub fn any(&self) -> &conjure_object::Value {
        &self.any
    }
}
#[derive(Debug, Clone, Default)]
pub struct Builder {
    any: Option<conjure_object::Value>,
}
impl Builder {
    #[doc = r""]
    #[doc = r" Required."]
    pub fn any<T>(&mut self, any: T) -> &mut Self
    where
        T: conjure_object::serde::Serialize,
    {
        self.any =
            Some(conjure_object::serde_value::to_value(any).expect("value failed to serialize"));
        self
    }
    #[doc = r" Constructs a new instance of the type."]
    #[doc = r""]
    #[doc = r" # Panics"]
    #[doc = r""]
    #[doc = r" Panics if a required field was not set."]
    #[inline]
    pub fn build(&self) -> AnyExample {
        AnyExample {
            any: self.any.clone().expect("field any was not set"),
        }
    }
}
impl From<AnyExample> for Builder {
    #[inline]
    fn from(_v: AnyExample) -> Builder {
        Builder { any: Some(_v.any) }
    }
}
impl ser::Serialize for AnyExample {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut s = s.serialize_struct("AnyExample", 1usize)?;
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
