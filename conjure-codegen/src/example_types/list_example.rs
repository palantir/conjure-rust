use conjure_object::serde::ser::SerializeMap as SerializeMap_;
use conjure_object::serde::{de, ser};
use std::fmt;
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ListExample {
    items: Vec<String>,
    primitive_items: Vec<i32>,
    double_items: Vec<f64>,
}
impl ListExample {
    #[doc = r" Returns a new builder."]
    #[inline]
    pub fn builder() -> Builder {
        Default::default()
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
#[derive(Debug, Clone, Default)]
pub struct Builder {
    items: Vec<String>,
    primitive_items: Vec<i32>,
    double_items: Vec<f64>,
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
    pub fn primitive_items<T>(&mut self, primitive_items: T) -> &mut Self
    where
        T: IntoIterator<Item = i32>,
    {
        self.primitive_items = primitive_items.into_iter().collect();
        self
    }
    pub fn extend_primitive_items<T>(&mut self, primitive_items: T) -> &mut Self
    where
        T: IntoIterator<Item = i32>,
    {
        self.primitive_items.extend(primitive_items);
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
    pub fn build(&self) -> ListExample {
        ListExample {
            items: self.items.clone(),
            primitive_items: self.primitive_items.clone(),
            double_items: self.double_items.clone(),
        }
    }
}
impl From<ListExample> for Builder {
    #[inline]
    fn from(_v: ListExample) -> Builder {
        Builder {
            items: _v.items,
            primitive_items: _v.primitive_items,
            double_items: _v.double_items,
        }
    }
}
impl ser::Serialize for ListExample {
    fn serialize<S_>(&self, s: S_) -> Result<S_::Ok, S_::Error>
    where
        S_: ser::Serializer,
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
        let mut map = s.serialize_map(Some(size))?;
        if !skip_items {
            map.serialize_entry(&"items", &self.items)?;
        }
        if !skip_primitive_items {
            map.serialize_entry(&"primitiveItems", &self.primitive_items)?;
        }
        if !skip_double_items {
            map.serialize_entry(&"doubleItems", &self.double_items)?;
        }
        map.end()
    }
}
impl<'de> de::Deserialize<'de> for ListExample {
    fn deserialize<D_>(d: D_) -> Result<ListExample, D_::Error>
    where
        D_: de::Deserializer<'de>,
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
    fn visit_map<A_>(self, mut map_: A_) -> Result<ListExample, A_::Error>
    where
        A_: de::MapAccess<'de>,
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
            "primitiveItems" => Field_::PrimitiveItems,
            "doubleItems" => Field_::DoubleItems,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
