use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ManyFieldExample {
    string: String,
    integer: i32,
    double_value: f64,
    optional_item: Option<String>,
    items: Vec<String>,
    set: std::collections::BTreeSet<String>,
    map: std::collections::BTreeMap<String, String>,
    alias: super::StringAliasExample,
}
impl ManyFieldExample {
    /// Returns a new builder.
    #[inline]
    pub fn builder() -> Builder {
        Default::default()
    }
    ///docs for string field
    #[inline]
    pub fn string(&self) -> &str {
        &*self.string
    }
    ///docs for integer field
    #[inline]
    pub fn integer(&self) -> i32 {
        self.integer
    }
    ///docs for doubleValue field
    #[inline]
    pub fn double_value(&self) -> f64 {
        self.double_value
    }
    ///docs for optionalItem field
    #[inline]
    pub fn optional_item(&self) -> Option<&str> {
        self.optional_item.as_ref().map(|o| &**o)
    }
    ///docs for items field
    #[inline]
    pub fn items(&self) -> &[String] {
        &*self.items
    }
    ///docs for set field
    #[inline]
    pub fn set(&self) -> &std::collections::BTreeSet<String> {
        &self.set
    }
    ///docs for map field
    #[inline]
    pub fn map(&self) -> &std::collections::BTreeMap<String, String> {
        &self.map
    }
    ///docs for alias field
    #[inline]
    pub fn alias(&self) -> &super::StringAliasExample {
        &self.alias
    }
}
///A builder for the `ManyFieldExample` type.
#[derive(Debug, Clone, Default)]
pub struct Builder {
    string: Option<String>,
    integer: Option<i32>,
    double_value: Option<f64>,
    optional_item: Option<String>,
    items: Vec<String>,
    set: std::collections::BTreeSet<String>,
    map: std::collections::BTreeMap<String, String>,
    alias: Option<super::StringAliasExample>,
}
impl Builder {
    ///docs for string field
    ///
    /// Required.
    #[inline]
    pub fn string<T>(&mut self, string: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.string = Some(string.into());
        self
    }
    ///docs for integer field
    ///
    /// Required.
    #[inline]
    pub fn integer(&mut self, integer: i32) -> &mut Self {
        self.integer = Some(integer);
        self
    }
    ///docs for doubleValue field
    ///
    /// Required.
    #[inline]
    pub fn double_value(&mut self, double_value: f64) -> &mut Self {
        self.double_value = Some(double_value);
        self
    }
    ///docs for optionalItem field
    #[inline]
    pub fn optional_item<T>(&mut self, optional_item: T) -> &mut Self
    where
        T: Into<Option<String>>,
    {
        self.optional_item = optional_item.into();
        self
    }
    ///docs for items field
    #[inline]
    pub fn items<T>(&mut self, items: T) -> &mut Self
    where
        T: IntoIterator<Item = String>,
    {
        self.items = items.into_iter().collect();
        self
    }
    ///docs for items field
    #[inline]
    pub fn extend_items<T>(&mut self, items: T) -> &mut Self
    where
        T: IntoIterator<Item = String>,
    {
        self.items.extend(items);
        self
    }
    ///docs for items field
    #[inline]
    pub fn push_items<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.items.push(value.into());
        self
    }
    ///docs for set field
    #[inline]
    pub fn set<T>(&mut self, set: T) -> &mut Self
    where
        T: IntoIterator<Item = String>,
    {
        self.set = set.into_iter().collect();
        self
    }
    ///docs for set field
    #[inline]
    pub fn extend_set<T>(&mut self, set: T) -> &mut Self
    where
        T: IntoIterator<Item = String>,
    {
        self.set.extend(set);
        self
    }
    ///docs for set field
    #[inline]
    pub fn insert_set<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.set.insert(value.into());
        self
    }
    ///docs for map field
    #[inline]
    pub fn map<T>(&mut self, map: T) -> &mut Self
    where
        T: IntoIterator<Item = (String, String)>,
    {
        self.map = map.into_iter().collect();
        self
    }
    ///docs for map field
    #[inline]
    pub fn extend_map<T>(&mut self, map: T) -> &mut Self
    where
        T: IntoIterator<Item = (String, String)>,
    {
        self.map.extend(map);
        self
    }
    ///docs for map field
    #[inline]
    pub fn insert_map<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.map.insert(key.into(), value.into());
        self
    }
    ///docs for alias field
    ///
    /// Required.
    #[inline]
    pub fn alias(&mut self, alias: super::StringAliasExample) -> &mut Self {
        self.alias = Some(alias);
        self
    }
    /// Constructs a new instance of the type.
    ///
    /// # Panics
    ///
    /// Panics if a required field was not set.
    #[inline]
    pub fn build(&self) -> ManyFieldExample {
        ManyFieldExample {
            string: self.string.clone().expect("field string was not set"),
            integer: self.integer.clone().expect("field integer was not set"),
            double_value: self
                .double_value
                .clone()
                .expect("field double_value was not set"),
            optional_item: self.optional_item.clone(),
            items: self.items.clone(),
            set: self.set.clone(),
            map: self.map.clone(),
            alias: self.alias.clone().expect("field alias was not set"),
        }
    }
}
impl From<ManyFieldExample> for Builder {
    #[inline]
    fn from(_v: ManyFieldExample) -> Builder {
        Builder {
            string: Some(_v.string),
            integer: Some(_v.integer),
            double_value: Some(_v.double_value),
            optional_item: _v.optional_item,
            items: _v.items,
            set: _v.set,
            map: _v.map,
            alias: Some(_v.alias),
        }
    }
}
impl ser::Serialize for ManyFieldExample {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut size = 4usize;
        let skip_optional_item = self.optional_item.is_none();
        if !skip_optional_item {
            size += 1;
        }
        let skip_items = self.items.is_empty();
        if !skip_items {
            size += 1;
        }
        let skip_set = self.set.is_empty();
        if !skip_set {
            size += 1;
        }
        let skip_map = self.map.is_empty();
        if !skip_map {
            size += 1;
        }
        let mut s = s.serialize_struct("ManyFieldExample", size)?;
        s.serialize_field("string", &self.string)?;
        s.serialize_field("integer", &self.integer)?;
        s.serialize_field("doubleValue", &self.double_value)?;
        if skip_optional_item {
            s.skip_field("optionalItem")?;
        } else {
            s.serialize_field("optionalItem", &self.optional_item)?;
        }
        if skip_items {
            s.skip_field("items")?;
        } else {
            s.serialize_field("items", &self.items)?;
        }
        if skip_set {
            s.skip_field("set")?;
        } else {
            s.serialize_field("set", &self.set)?;
        }
        if skip_map {
            s.skip_field("map")?;
        } else {
            s.serialize_field("map", &self.map)?;
        }
        s.serialize_field("alias", &self.alias)?;
        s.end()
    }
}
impl<'de> de::Deserialize<'de> for ManyFieldExample {
    fn deserialize<D>(d: D) -> Result<ManyFieldExample, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct(
            "ManyFieldExample",
            &[
                "string",
                "integer",
                "doubleValue",
                "optionalItem",
                "items",
                "set",
                "map",
                "alias",
            ],
            Visitor_,
        )
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = ManyFieldExample;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<ManyFieldExample, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut string = None;
        let mut integer = None;
        let mut double_value = None;
        let mut optional_item = None;
        let mut items = None;
        let mut set = None;
        let mut map = None;
        let mut alias = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::String => string = Some(map_.next_value()?),
                Field_::Integer => integer = Some(map_.next_value()?),
                Field_::DoubleValue => double_value = Some(map_.next_value()?),
                Field_::OptionalItem => optional_item = Some(map_.next_value()?),
                Field_::Items => items = Some(map_.next_value()?),
                Field_::Set => set = Some(map_.next_value()?),
                Field_::Map => map = Some(map_.next_value()?),
                Field_::Alias => alias = Some(map_.next_value()?),
                Field_::Unknown_ => {
                    map_.next_value::<de::IgnoredAny>()?;
                }
            }
        }
        let string = match string {
            Some(v) => v,
            None => return Err(de::Error::missing_field("string")),
        };
        let integer = match integer {
            Some(v) => v,
            None => return Err(de::Error::missing_field("integer")),
        };
        let double_value = match double_value {
            Some(v) => v,
            None => return Err(de::Error::missing_field("doubleValue")),
        };
        let optional_item = match optional_item {
            Some(v) => v,
            None => Default::default(),
        };
        let items = match items {
            Some(v) => v,
            None => Default::default(),
        };
        let set = match set {
            Some(v) => v,
            None => Default::default(),
        };
        let map = match map {
            Some(v) => v,
            None => Default::default(),
        };
        let alias = match alias {
            Some(v) => v,
            None => return Err(de::Error::missing_field("alias")),
        };
        Ok(ManyFieldExample {
            string,
            integer,
            double_value,
            optional_item,
            items,
            set,
            map,
            alias,
        })
    }
}
enum Field_ {
    String,
    Integer,
    DoubleValue,
    OptionalItem,
    Items,
    Set,
    Map,
    Alias,
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
            "string" => Field_::String,
            "integer" => Field_::Integer,
            "doubleValue" => Field_::DoubleValue,
            "optionalItem" => Field_::OptionalItem,
            "items" => Field_::Items,
            "set" => Field_::Set,
            "map" => Field_::Map,
            "alias" => Field_::Alias,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
