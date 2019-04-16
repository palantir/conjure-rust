use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use conjure_object::serde::{de, ser};
use std::fmt;
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ListExample {
    items: Vec<String>,
    primitive_items: Vec<i32>,
    double_items: Vec<f64>,
}
impl ListExample {
    #[doc = r" Constructs a new instance of the type."]
    #[inline]
    pub fn new<T, U, V>(items: T, primitive_items: U, double_items: V) -> ListExample
    where
        T: IntoIterator<Item = String>,
        U: IntoIterator<Item = i32>,
        V: IntoIterator<Item = f64>,
    {
        ListExample {
            items: items.into_iter().collect(),
            primitive_items: primitive_items.into_iter().collect(),
            double_items: double_items.into_iter().collect(),
        }
    }
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
    pub fn push_items<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.items.push(value.into());
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
    pub fn push_primitive_items(&mut self, value: i32) -> &mut Self {
        self.primitive_items.push(value);
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
    pub fn push_double_items(&mut self, value: f64) -> &mut Self {
        self.double_items.push(value);
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
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut s = s.serialize_struct("ListExample", 3usize)?;
        if self.items.is_empty() {
            s.skip_field("items")?;
        } else {
            s.serialize_field("items", &self.items)?;
        }
        if self.primitive_items.is_empty() {
            s.skip_field("primitiveItems")?;
        } else {
            s.serialize_field("primitiveItems", &self.primitive_items)?;
        }
        if self.double_items.is_empty() {
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
