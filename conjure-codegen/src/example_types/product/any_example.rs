use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[conjure_object::private::staged_builder::staged_builder]
#[builder(crate = conjure_object::private::staged_builder, update, inline)]
pub struct AnyExample {
    #[builder(
        custom(
            type = impl
            conjure_object::serde::Serialize,
            convert = |v|conjure_object::Any::new(v).expect("value failed to serialize")
        )
    )]
    any: conjure_object::Any,
}
impl AnyExample {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(any: impl conjure_object::serde::Serialize) -> Self {
        Self::builder().any(any).build()
    }
    #[inline]
    pub fn any(&self) -> &conjure_object::Any {
        &self.any
    }
}
impl ser::Serialize for AnyExample {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let size = 1usize;
        let mut s = s.serialize_struct("AnyExample", size)?;
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
