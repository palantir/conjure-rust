use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RidExample {
    rid_value: conjure_object::ResourceIdentifier,
}
impl RidExample {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(rid_value: conjure_object::ResourceIdentifier) -> RidExample {
        RidExample { rid_value: rid_value }
    }
    /// Returns a new builder.
    #[inline]
    pub fn builder() -> Builder {
        Default::default()
    }
    #[inline]
    pub fn rid_value(&self) -> &conjure_object::ResourceIdentifier {
        &self.rid_value
    }
}
///A builder for the `RidExample` type.
#[derive(Debug, Clone, Default)]
pub struct Builder {
    rid_value: Option<conjure_object::ResourceIdentifier>,
}
impl Builder {
    ///
    /// Required.
    #[inline]
    pub fn rid_value(
        &mut self,
        rid_value: conjure_object::ResourceIdentifier,
    ) -> &mut Self {
        self.rid_value = Some(rid_value);
        self
    }
    /// Constructs a new instance of the type.
    ///
    /// # Panics
    ///
    /// Panics if a required field was not set.
    #[inline]
    pub fn build(&self) -> RidExample {
        RidExample {
            rid_value: self.rid_value.clone().expect("field rid_value was not set"),
        }
    }
}
impl From<RidExample> for Builder {
    #[inline]
    fn from(_v: RidExample) -> Builder {
        Builder {
            rid_value: Some(_v.rid_value),
        }
    }
}
impl ser::Serialize for RidExample {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let size = 1usize;
        let mut s = s.serialize_struct("RidExample", size)?;
        s.serialize_field("ridValue", &self.rid_value)?;
        s.end()
    }
}
impl<'de> de::Deserialize<'de> for RidExample {
    fn deserialize<D>(d: D) -> Result<RidExample, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct("RidExample", &["ridValue"], Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = RidExample;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<RidExample, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut rid_value = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::RidValue => rid_value = Some(map_.next_value()?),
                Field_::Unknown_ => {
                    map_.next_value::<de::IgnoredAny>()?;
                }
            }
        }
        let rid_value = match rid_value {
            Some(v) => v,
            None => return Err(de::Error::missing_field("ridValue")),
        };
        Ok(RidExample { rid_value })
    }
}
enum Field_ {
    RidValue,
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
            "ridValue" => Field_::RidValue,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
