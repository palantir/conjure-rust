use conjure_object::serde::ser::SerializeMap as SerializeMap_;
use conjure_object::serde::{de, ser};
use std::fmt;
#[derive(Debug, Clone, PartialEq, PartialOrd, Copy)]
pub struct DoubleExample {
    double_value: f64,
}
impl DoubleExample {
    #[doc = r" Returns a new builder."]
    #[inline]
    pub fn builder() -> Builder {
        Default::default()
    }
    #[inline]
    pub fn double_value(&self) -> f64 {
        self.double_value
    }
}
#[derive(Debug, Clone, Default)]
pub struct Builder {
    double_value: Option<f64>,
}
impl Builder {
    #[doc = r""]
    #[doc = r" Required."]
    #[inline]
    pub fn double_value(&mut self, double_value: f64) -> &mut Self {
        self.double_value = Some(double_value);
        self
    }
    #[doc = r" Constructs a new instance of the type."]
    #[doc = r""]
    #[doc = r" # Panics"]
    #[doc = r""]
    #[doc = r" Panics if a required field was not set."]
    #[inline]
    pub fn build(&self) -> DoubleExample {
        DoubleExample {
            double_value: self
                .double_value
                .clone()
                .expect("field double_value was not set"),
        }
    }
}
impl From<DoubleExample> for Builder {
    #[inline]
    fn from(_v: DoubleExample) -> Builder {
        Builder {
            double_value: Some(_v.double_value),
        }
    }
}
impl ser::Serialize for DoubleExample {
    fn serialize<S_>(&self, s: S_) -> Result<S_::Ok, S_::Error>
    where
        S_: ser::Serializer,
    {
        let size = 1usize;
        let mut map = s.serialize_map(Some(size))?;
        map.serialize_entry(&"doubleValue", &self.double_value)?;
        map.end()
    }
}
impl<'de> de::Deserialize<'de> for DoubleExample {
    fn deserialize<D_>(d: D_) -> Result<DoubleExample, D_::Error>
    where
        D_: de::Deserializer<'de>,
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
    fn visit_map<A_>(self, mut map_: A_) -> Result<DoubleExample, A_::Error>
    where
        A_: de::MapAccess<'de>,
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
            "doubleValue" => Field_::DoubleValue,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
