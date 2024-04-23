use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, conjure_object::private::Educe)]
#[educe(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[conjure_object::private::staged_builder::staged_builder]
#[builder(crate = conjure_object::private::staged_builder, update, inline)]
pub struct ListExample {
    #[builder(default, list(item(type = String, into)))]
    items: Vec<String>,
    #[builder(default, list(item(type = i32)))]
    primitive_items: Vec<i32>,
    #[builder(default, list(item(type = f64)))]
    #[educe(
        PartialEq(method(conjure_object::private::DoubleOps::eq)),
        Ord(method(conjure_object::private::DoubleOps::cmp)),
        Hash(method(conjure_object::private::DoubleOps::hash)),
    )]
    double_items: Vec<f64>,
}
impl ListExample {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new() -> Self {
        Self::builder().build()
    }
    #[inline]
    pub fn items(&self) -> &[String] {
        &*self.items
    }
    #[inline]
    pub fn primitive_items(&self) -> &[i32] {
        &*self.primitive_items
    }
    #[inline]
    pub fn double_items(&self) -> &[f64] {
        &*self.double_items
    }
}
impl ser::Serialize for ListExample {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut size = 0usize;
        let skip_items = self.items.is_empty();
        if !skip_items {
            size += 1;
        }
        let skip_primitive_items = self.primitive_items.is_empty();
        if !skip_primitive_items {
            size += 1;
        }
        let skip_double_items = self.double_items.is_empty();
        if !skip_double_items {
            size += 1;
        }
        let mut s = s.serialize_struct("ListExample", size)?;
        if skip_items {
            s.skip_field("items")?;
        } else {
            s.serialize_field("items", &self.items)?;
        }
        if skip_primitive_items {
            s.skip_field("primitiveItems")?;
        } else {
            s.serialize_field("primitiveItems", &self.primitive_items)?;
        }
        if skip_double_items {
            s.skip_field("doubleItems")?;
        } else {
            s.serialize_field("doubleItems", &self.double_items)?;
        }
        s.end()
    }
}
impl<'de> de::Deserialize<'de> for ListExample {
    fn deserialize<D>(d: D) -> Result<ListExample, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct(
            "ListExample",
            &["items", "primitiveItems", "doubleItems"],
            Visitor_,
        )
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = ListExample;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<ListExample, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut items = None;
        let mut primitive_items = None;
        let mut double_items = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::Items => items = Some(map_.next_value()?),
                Field_::PrimitiveItems => primitive_items = Some(map_.next_value()?),
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
        let primitive_items = match primitive_items {
            Some(v) => v,
            None => Default::default(),
        };
        let double_items = match double_items {
            Some(v) => v,
            None => Default::default(),
        };
        Ok(ListExample {
            items,
            primitive_items,
            double_items,
        })
    }
}
enum Field_ {
    Items,
    PrimitiveItems,
    DoubleItems,
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
            "primitiveItems" => Field_::PrimitiveItems,
            "doubleItems" => Field_::DoubleItems,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
