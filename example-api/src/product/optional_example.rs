use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OptionalExample {
    item: Option<String>,
}
impl OptionalExample {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new<T>(item: T) -> OptionalExample
    where
        T: Into<String>,
    {
        OptionalExample {
            item: Some(item.into()),
        }
    }
    /// Returns a new builder.
    #[inline]
    pub fn builder() -> BuilderStage0 {
        Default::default()
    }
    #[inline]
    pub fn item(&self) -> Option<&str> {
        self.item.as_ref().map(|o| &**o)
    }
}
impl Default for BuilderStage0 {
    #[inline]
    fn default() -> Self {
        BuilderStage0 {
            item: Default::default(),
        }
    }
}
impl From<OptionalExample> for BuilderStage0 {
    #[inline]
    fn from(value: OptionalExample) -> Self {
        BuilderStage0 { item: value.item }
    }
}
///The stage 0 builder for the [`OptionalExample`] type
#[derive(Debug, Clone)]
pub struct BuilderStage0 {
    item: Option<String>,
}
impl BuilderStage0 {
    #[inline]
    pub fn item<T>(mut self, item: T) -> Self
    where
        T: Into<Option<String>>,
    {
        self.item = item.into();
        self
    }
    /// Consumes the builder, constructing a new instance of the type.
    #[inline]
    pub fn build(self) -> OptionalExample {
        OptionalExample { item: self.item }
    }
}
impl ser::Serialize for OptionalExample {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut size = 0usize;
        let skip_item = self.item.is_none();
        if !skip_item {
            size += 1;
        }
        let mut s = s.serialize_struct("OptionalExample", size)?;
        if skip_item {
            s.skip_field("item")?;
        } else {
            s.serialize_field("item", &self.item)?;
        }
        s.end()
    }
}
impl<'de> de::Deserialize<'de> for OptionalExample {
    fn deserialize<D>(d: D) -> Result<OptionalExample, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct("OptionalExample", &["item"], Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = OptionalExample;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<OptionalExample, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut item = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::Item => item = Some(map_.next_value()?),
                Field_::Unknown_ => {
                    map_.next_value::<de::IgnoredAny>()?;
                }
            }
        }
        let item = match item {
            Some(v) => v,
            None => Default::default(),
        };
        Ok(OptionalExample { item })
    }
}
enum Field_ {
    Item,
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
            "item" => Field_::Item,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
