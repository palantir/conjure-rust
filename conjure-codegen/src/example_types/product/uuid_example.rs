use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Copy)]
pub struct UuidExample {
    uuid: conjure_object::Uuid,
}
impl UuidExample {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(uuid: conjure_object::Uuid) -> UuidExample {
        UuidExample { uuid: uuid }
    }
    /// Returns a new builder.
    #[inline]
    pub fn builder() -> Builder {
        Default::default()
    }
    #[inline]
    pub fn uuid(&self) -> conjure_object::Uuid {
        self.uuid
    }
}
///A builder for the `UuidExample` type.
#[derive(Debug, Clone, Default)]
pub struct Builder {
    uuid: Option<conjure_object::Uuid>,
}
impl Builder {
    ///
    /// Required.
    #[inline]
    pub fn uuid(&mut self, uuid: conjure_object::Uuid) -> &mut Self {
        self.uuid = Some(uuid);
        self
    }
    /// Constructs a new instance of the type.
    ///
    /// # Panics
    ///
    /// Panics if a required field was not set.
    #[inline]
    pub fn build(&self) -> UuidExample {
        UuidExample {
            uuid: self.uuid.clone().expect("field uuid was not set"),
        }
    }
}
impl From<UuidExample> for Builder {
    #[inline]
    fn from(_v: UuidExample) -> Builder {
        Builder { uuid: Some(_v.uuid) }
    }
}
impl ser::Serialize for UuidExample {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let size = 1usize;
        let mut s = s.serialize_struct("UuidExample", size)?;
        s.serialize_field("uuid", &self.uuid)?;
        s.end()
    }
}
impl<'de> de::Deserialize<'de> for UuidExample {
    fn deserialize<D>(d: D) -> Result<UuidExample, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct("UuidExample", &["uuid"], Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = UuidExample;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<UuidExample, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut uuid = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::Uuid => uuid = Some(map_.next_value()?),
                Field_::Unknown_ => {
                    map_.next_value::<de::IgnoredAny>()?;
                }
            }
        }
        let uuid = match uuid {
            Some(v) => v,
            None => return Err(de::Error::missing_field("uuid")),
        };
        Ok(UuidExample { uuid })
    }
}
enum Field_ {
    Uuid,
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
            "uuid" => Field_::Uuid,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
