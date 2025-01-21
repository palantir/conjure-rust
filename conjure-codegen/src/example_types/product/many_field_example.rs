#[derive(
    Debug,
    Clone,
    conjure_object::serde::Serialize,
    conjure_object::serde::Deserialize,
    conjure_object::private::Educe
)]
#[serde(crate = "conjure_object::serde")]
#[educe(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[conjure_object::private::staged_builder::staged_builder]
#[builder(crate = conjure_object::private::staged_builder, update, inline)]
pub struct ManyFieldExample {
    #[builder(into)]
    #[serde(rename = "string")]
    string: String,
    #[serde(rename = "integer")]
    integer: i32,
    #[serde(rename = "doubleValue")]
    #[educe(
        PartialEq(method(conjure_object::private::DoubleOps::eq)),
        Ord(method(conjure_object::private::DoubleOps::cmp)),
        Hash(method(conjure_object::private::DoubleOps::hash)),
    )]
    double_value: f64,
    #[builder(default, into)]
    #[serde(rename = "optionalItem", default)]
    optional_item: Option<String>,
    #[builder(default, list(item(type = String, into)))]
    #[serde(rename = "items", default)]
    items: Vec<String>,
    #[builder(default, set(item(type = String, into)))]
    #[serde(rename = "set", default)]
    set: std::collections::BTreeSet<String>,
    #[builder(default, map(key(type = String, into), value(type = String, into)))]
    #[serde(rename = "map", default)]
    map: std::collections::BTreeMap<String, String>,
    #[serde(rename = "alias")]
    alias: super::StringAliasExample,
}
impl ManyFieldExample {
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
