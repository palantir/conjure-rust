use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use conjure_object::serde::{de, ser};
use std::fmt;
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct AnyMapExample {
    items: std::collections::BTreeMap<String, conjure_object::Any>,
}
impl AnyMapExample {
    #[doc = r" Constructs a new instance of the type."]
    #[inline]
    pub fn new<T>(items: T) -> AnyMapExample
    where
        T: IntoIterator<Item = (String, conjure_object::Any)>,
    {
        AnyMapExample {
            items: items.into_iter().collect(),
        }
    }
    #[doc = r" Returns a new builder."]
    #[inline]
    pub fn builder() -> Builder {
        Default::default()
    }
    #[inline]
    pub fn items(&self) -> &std::collections::BTreeMap<String, conjure_object::Any> {
        &self.items
    }
}
#[doc = "A builder for the `AnyMapExample` type."]
#[derive(Debug, Clone, Default)]
pub struct Builder {
    items: std::collections::BTreeMap<String, conjure_object::Any>,
}
impl Builder {
    pub fn items<T>(&mut self, items: T) -> &mut Self
    where
        T: IntoIterator<Item = (String, conjure_object::Any)>,
    {
        self.items = items.into_iter().collect();
        self
    }
    pub fn extend_items<T>(&mut self, items: T) -> &mut Self
    where
        T: IntoIterator<Item = (String, conjure_object::Any)>,
    {
        self.items.extend(items);
        self
    }
    pub fn insert_items<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: Into<String>,
        V: conjure_object::serde::Serialize,
    {
        self.items.insert(
            key.into(),
            conjure_object::serde_value::to_value(value).expect("value failed to serialize"),
        );
        self
    }
    #[doc = r" Constructs a new instance of the type."]
    #[doc = r""]
    #[doc = r" # Panics"]
    #[doc = r""]
    #[doc = r" Panics if a required field was not set."]
    #[inline]
    pub fn build(&self) -> AnyMapExample {
        AnyMapExample {
            items: self.items.clone(),
        }
    }
}
impl From<AnyMapExample> for Builder {
    #[inline]
    fn from(_v: AnyMapExample) -> Builder {
        Builder { items: _v.items }
    }
}
impl ser::Serialize for AnyMapExample {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut size = 0usize;
        let skip_items = self.items.is_empty();
        if !skip_items {
            size += 1;
        }
        let mut s = s.serialize_struct("AnyMapExample", size)?;
        if skip_items {
            s.skip_field("items")?;
        } else {
            s.serialize_field("items", &self.items)?;
        }
        s.end()
    }
}
impl<'de> de::Deserialize<'de> for AnyMapExample {
    fn deserialize<D>(d: D) -> Result<AnyMapExample, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct("AnyMapExample", &["items"], Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = AnyMapExample;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<AnyMapExample, A::Error>
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
        Ok(AnyMapExample { items })
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
