use crate::serde::ser::SerializeMap as SerializeMap_;
use crate::serde::{de, ser};
use std::fmt;
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SetExample {
    items: std::collections::BTreeSet<String>,
    double_items: std::collections::BTreeSet<f64>,
}
impl SetExample {
    #[doc = r" Returns a new builder."]
    #[inline]
    pub fn builder() -> Builder {
        Default::default()
    }
    #[inline]
    pub fn items(&self) -> &std::collections::BTreeSet<String> {
        &self.items
    }
    #[inline]
    pub fn double_items(&self) -> &std::collections::BTreeSet<f64> {
        &self.double_items
    }
}
#[derive(Debug, Clone, Default)]
pub struct Builder {
    items: std::collections::BTreeSet<String>,
    double_items: std::collections::BTreeSet<f64>,
}
impl Builder {
    pub fn items<T>(&mut self, items: T) -> &mut Self
    where
        T: IntoIterator<Item = String>,
    {
        self.items = items.into_iter().collect();
        self
    }
    pub fn extend_items<T>(&mut self, items: T) -> &mut Self
    where
        T: IntoIterator<Item = String>,
    {
        self.items.extend(items);
        self
    }
    pub fn double_items<T>(&mut self, double_items: T) -> &mut Self
    where
        T: IntoIterator<Item = f64>,
    {
        self.double_items = double_items.into_iter().collect();
        self
    }
    pub fn extend_double_items<T>(&mut self, double_items: T) -> &mut Self
    where
        T: IntoIterator<Item = f64>,
    {
        self.double_items.extend(double_items);
        self
    }
    #[doc = r" Constructs a new instance of the type."]
    #[doc = r""]
    #[doc = r" # Panics"]
    #[doc = r""]
    #[doc = r" Panics if a required field was not set."]
    #[inline]
    pub fn build(&self) -> SetExample {
        SetExample {
            items: self.items.clone(),
            double_items: self.double_items.clone(),
        }
    }
}
impl From<SetExample> for Builder {
    #[inline]
    fn from(_v: SetExample) -> Builder {
        Builder {
            items: _v.items,
            double_items: _v.double_items,
        }
    }
}
impl ser::Serialize for SetExample {
    fn serialize<S_>(&self, s: S_) -> Result<S_::Ok, S_::Error>
    where
        S_: ser::Serializer,
    {
        let mut size = 0usize;
        let skip_items = self.items.is_empty();
        if !skip_items {
            size += 1;
        }
        let skip_double_items = self.double_items.is_empty();
        if !skip_double_items {
            size += 1;
        }
        let mut map = s.serialize_map(Some(size))?;
        if !skip_items {
            map.serialize_entry(&"items", &self.items)?;
        }
        if !skip_double_items {
            map.serialize_entry(&"doubleItems", &self.double_items)?;
        }
        map.end()
    }
}
impl<'de> de::Deserialize<'de> for SetExample {
    fn deserialize<D_>(d: D_) -> Result<SetExample, D_::Error>
    where
        D_: de::Deserializer<'de>,
    {
        d.deserialize_struct("SetExample", &["items", "doubleItems"], Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = SetExample;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A_>(self, mut map_: A_) -> Result<SetExample, A_::Error>
    where
        A_: de::MapAccess<'de>,
    {
        let mut items = None;
        let mut double_items = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::Items => items = Some(map_.next_value()?),
                Field_::DoubleItems => double_items = Some(map_.next_value()?),
                Field_::Unknown_ => {
                    map_.next_value::<de::IgnoredAny>()?;
                }
            }
        }
        let items = match items {
            Some(v) => v,
            None => Default::default(),
        };
        let double_items = match double_items {
            Some(v) => v,
            None => Default::default(),
        };
        Ok(SetExample {
            items,
            double_items,
        })
    }
}
enum Field_ {
    Items,
    DoubleItems,
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
            "doubleItems" => Field_::DoubleItems,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
