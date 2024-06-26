use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[conjure_object::private::staged_builder::staged_builder]
#[builder(crate = conjure_object::private::staged_builder, update, inline)]
pub struct MapExample {
    #[builder(default, map(key(type = String, into), value(type = String, into)))]
    items: std::collections::BTreeMap<String, String>,
}
impl MapExample {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new() -> Self {
        Self::builder().build()
    }
    #[inline]
    pub fn items(&self) -> &std::collections::BTreeMap<String, String> {
        &self.items
    }
}
impl ser::Serialize for MapExample {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut size = 0usize;
        let skip_items = self.items.is_empty();
        if !skip_items {
            size += 1;
        }
        let mut s = s.serialize_struct("MapExample", size)?;
        if skip_items {
            s.skip_field("items")?;
        } else {
            s.serialize_field("items", &self.items)?;
        }
        s.end()
    }
}
impl<'de> de::Deserialize<'de> for MapExample {
    fn deserialize<D>(d: D) -> Result<MapExample, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct("MapExample", &["items"], Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = MapExample;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<MapExample, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut items = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::Items => items = Some(map_.next_value()?),
                Field_::Unknown_ => {
                    map_.next_value::<de::IgnoredAny>()?;
                }
            }
        }
        let items = match items {
            Some(v) => v,
            None => Default::default(),
        };
        Ok(MapExample { items })
    }
}
enum Field_ {
    Items,
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
            "items" => Field_::Items,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
