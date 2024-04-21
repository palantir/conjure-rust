use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MapExample {
    items: std::collections::BTreeMap<String, String>,
}
impl MapExample {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new<T>(items: T) -> MapExample
    where
        T: IntoIterator<Item = (String, String)>,
    {
        MapExample {
            items: items.into_iter().collect(),
        }
    }
    /// Returns a new builder.
    #[inline]
    pub fn builder() -> BuilderStage0 {
        Default::default()
    }
    #[inline]
    pub fn items(&self) -> &std::collections::BTreeMap<String, String> {
        &self.items
    }
}
impl Default for BuilderStage0 {
    #[inline]
    fn default() -> Self {
        BuilderStage0 {
            items: Default::default(),
        }
    }
}
impl From<MapExample> for BuilderStage0 {
    #[inline]
    fn from(value: MapExample) -> Self {
        BuilderStage0 {
            items: value.items,
        }
    }
}
///The stage 0 builder for the [`MapExample`] type
#[derive(Debug, Clone)]
pub struct BuilderStage0 {
    items: std::collections::BTreeMap<String, String>,
}
impl BuilderStage0 {
    #[inline]
    pub fn items<T>(mut self, items: T) -> Self
    where
        T: IntoIterator<Item = (String, String)>,
    {
        self.items = items.into_iter().collect();
        self
    }
    #[inline]
    pub fn extend_items<T>(mut self, items: T) -> Self
    where
        T: IntoIterator<Item = (String, String)>,
    {
        self.items.extend(items);
        self
    }
    #[inline]
    pub fn insert_items<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.items.insert(key.into(), value.into());
        self
    }
    /// Consumes the builder, constructing a new instance of the type.
    #[inline]
    pub fn build(self) -> MapExample {
        MapExample { items: self.items }
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
