use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
#[conjure_object::private::staged_builder::staged_builder]
#[builder(crate = conjure_object::private::staged_builder, update, inline)]
pub struct BooleanExample {
    coin: bool,
}
impl BooleanExample {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(coin: bool) -> Self {
        Self::builder().coin(coin).build()
    }
    #[inline]
    pub fn coin(&self) -> bool {
        self.coin
    }
}
impl ser::Serialize for BooleanExample {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let size = 1usize;
        let mut s = s.serialize_struct("BooleanExample", size)?;
        s.serialize_field("coin", &self.coin)?;
        s.end()
    }
}
impl<'de> de::Deserialize<'de> for BooleanExample {
    fn deserialize<D>(d: D) -> Result<BooleanExample, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct("BooleanExample", &["coin"], Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = BooleanExample;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<BooleanExample, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut coin = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::Coin => coin = Some(map_.next_value()?),
                Field_::Unknown_ => {
                    map_.next_value::<de::IgnoredAny>()?;
                }
            }
        }
        let coin = match coin {
            Some(v) => v,
            None => return Err(de::Error::missing_field("coin")),
        };
        Ok(BooleanExample { coin })
    }
}
enum Field_ {
    Coin,
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
            "coin" => Field_::Coin,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
