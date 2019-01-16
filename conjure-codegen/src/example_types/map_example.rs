use conjure_object::serde::ser::SerializeMap as SerializeMap_;
use conjure_object::serde::{de, ser};
use std::fmt;
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct MapExample {
    items: std::collections::BTreeMap<String, String>,
}
impl MapExample {
    #[doc = r" Returns a new builder."]
    #[inline]
    pub fn builder() -> Builder {
        Default::default()
    }
    #[inline]
    pub fn items(&self) -> &std::collections::BTreeMap<String, String> {
        &self.items
    }
}
#[derive(Debug, Clone, Default)]
pub struct Builder {
    items: std::collections::BTreeMap<String, String>,
}
impl Builder {
    pub fn items<T>(&mut self, items: T) -> &mut Self
    where
        T: IntoIterator<Item = (String, String)>,
    {
        self.items = items.into_iter().collect();
        self
    }
    pub fn extend_items<T>(&mut self, items: T) -> &mut Self
    where
        T: IntoIterator<Item = (String, String)>,
    {
        self.items.extend(items);
        self
    }
    pub fn insert_items<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.items.insert(key.into(), value.into());
        self
    }
    #[doc = r" Constructs a new instance of the type."]
    #[doc = r""]
    #[doc = r" # Panics"]
    #[doc = r""]
    #[doc = r" Panics if a required field was not set."]
    #[inline]
    pub fn build(&self) -> MapExample {
        MapExample {
            items: self.items.clone(),
        }
    }
}
impl From<MapExample> for Builder {
    #[inline]
    fn from(_v: MapExample) -> Builder {
        Builder { items: _v.items }
    }
}
impl ser::Serialize for MapExample {
    fn serialize<S_>(&self, s: S_) -> Result<S_::Ok, S_::Error>
    where
        S_: ser::Serializer,
    {
        let mut size = 0usize;
        let skip_items = self.items.is_empty();
        if !skip_items {
            size += 1;
        }
        let mut map = s.serialize_map(Some(size))?;
        if !skip_items {
            map.serialize_entry(&"items", &self.items)?;
        }
        map.end()
    }
}
impl<'de> de::Deserialize<'de> for MapExample {
    fn deserialize<D_>(d: D_) -> Result<MapExample, D_::Error>
    where
        D_: de::Deserializer<'de>,
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
    fn visit_map<A_>(self, mut map_: A_) -> Result<MapExample, A_::Error>
    where
        A_: de::MapAccess<'de>,
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
            "items" => Field_::Items,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
