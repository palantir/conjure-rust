use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AnyMapExample {
    items: std::collections::BTreeMap<String, conjure_object::Any>,
}
impl AnyMapExample {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new<T>(items: T) -> AnyMapExample
    where
        T: IntoIterator<Item = (String, conjure_object::Any)>,
    {
        AnyMapExample {
            items: items.into_iter().collect(),
        }
    }
    /// Returns a new builder.
    #[inline]
    pub fn builder() -> BuilderStage0 {
        Default::default()
    }
    #[inline]
    pub fn items(&self) -> &std::collections::BTreeMap<String, conjure_object::Any> {
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
impl From<AnyMapExample> for BuilderStage0 {
    #[inline]
    fn from(value: AnyMapExample) -> Self {
        BuilderStage0 {
            items: value.items,
        }
    }
}
///The stage 0 builder for the [`AnyMapExample`] type
#[derive(Debug, Clone)]
pub struct BuilderStage0 {
    items: std::collections::BTreeMap<String, conjure_object::Any>,
}
impl BuilderStage0 {
    #[inline]
    pub fn items<T>(mut self, items: T) -> Self
    where
        T: IntoIterator<Item = (String, conjure_object::Any)>,
    {
        self.items = items.into_iter().collect();
        self
    }
    #[inline]
    pub fn extend_items<T>(mut self, items: T) -> Self
    where
        T: IntoIterator<Item = (String, conjure_object::Any)>,
    {
        self.items.extend(items);
        self
    }
    #[inline]
    pub fn insert_items<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: conjure_object::serde::Serialize,
    {
        self.items
            .insert(
                key.into(),
                conjure_object::Any::new(value).expect("value failed to serialize"),
            );
        self
    }
    /// Consumes the builder, constructing a new instance of the type.
    #[inline]
    pub fn build(self) -> AnyMapExample {
        AnyMapExample { items: self.items }
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
