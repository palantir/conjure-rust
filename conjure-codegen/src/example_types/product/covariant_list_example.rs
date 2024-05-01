use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[conjure_object::private::staged_builder::staged_builder]
#[builder(crate = conjure_object::private::staged_builder, update, inline)]
pub struct CovariantListExample {
    #[builder(
        default,
        list(
            item(
                custom(
                    type = impl
                    conjure_object::serde::Serialize,
                    convert = |v|conjure_object::Any::new(
                        v
                    ).expect("value failed to serialize")
                )
            )
        )
    )]
    items: Vec<conjure_object::Any>,
    #[builder(default, list(item(type = String, into)))]
    external_items: Vec<String>,
}
impl CovariantListExample {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new() -> Self {
        Self::builder().build()
    }
    #[inline]
    pub fn items(&self) -> &[conjure_object::Any] {
        &*self.items
    }
    #[inline]
    pub fn external_items(&self) -> &[String] {
        &*self.external_items
    }
}
impl ser::Serialize for CovariantListExample {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut size = 0usize;
        let skip_items = self.items.is_empty();
        if !skip_items {
            size += 1;
        }
        let skip_external_items = self.external_items.is_empty();
        if !skip_external_items {
            size += 1;
        }
        let mut s = s.serialize_struct("CovariantListExample", size)?;
        if skip_items {
            s.skip_field("items")?;
        } else {
            s.serialize_field("items", &self.items)?;
        }
        if skip_external_items {
            s.skip_field("externalItems")?;
        } else {
            s.serialize_field("externalItems", &self.external_items)?;
        }
        s.end()
    }
}
impl<'de> de::Deserialize<'de> for CovariantListExample {
    fn deserialize<D>(d: D) -> Result<CovariantListExample, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct(
            "CovariantListExample",
            &["items", "externalItems"],
            Visitor_,
        )
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = CovariantListExample;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<CovariantListExample, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut items = None;
        let mut external_items = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::Items => items = Some(map_.next_value()?),
                Field_::ExternalItems => external_items = Some(map_.next_value()?),
                Field_::Unknown_ => {
                    map_.next_value::<de::IgnoredAny>()?;
                }
            }
        }
        let items = match items {
            Some(v) => v,
            None => Default::default(),
        };
        let external_items = match external_items {
            Some(v) => v,
            None => Default::default(),
        };
        Ok(CovariantListExample {
            items,
            external_items,
        })
    }
}
enum Field_ {
    Items,
    ExternalItems,
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
            "externalItems" => Field_::ExternalItems,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
