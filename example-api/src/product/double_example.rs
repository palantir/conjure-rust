use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, PartialEq, PartialOrd, Copy)]
pub struct DoubleExample {
    double_value: f64,
}
impl DoubleExample {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(double_value: f64) -> DoubleExample {
        DoubleExample {
            double_value: double_value,
        }
    }
    /// Returns a new builder.
    #[inline]
    pub fn builder() -> Builder {
        Default::default()
    }
    #[inline]
    pub fn double_value(&self) -> f64 {
        self.double_value
    }
}
///A builder for the `DoubleExample` type.
#[derive(Debug, Clone, Default)]
pub struct Builder {
    double_value: Option<f64>,
}
impl Builder {
    ///
    /// Required.
    #[inline]
    pub fn double_value(&mut self, double_value: f64) -> &mut Self {
        self.double_value = Some(double_value);
        self
    }
    /// Constructs a new instance of the type.
    ///
    /// # Panics
    ///
    /// Panics if a required field was not set.
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
