#[derive(
    Debug,
    Clone,
    conjure_object::serde::Serialize,
    conjure_object::serde::Deserialize,
    conjure_object::private::DeriveWith
)]
#[serde(crate = "conjure_object::serde")]
#[derive_with(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[conjure_object::private::staged_builder::staged_builder]
#[builder(crate = conjure_object::private::staged_builder, update, inline)]
pub struct ListExample {
    #[builder(default, list(item(type = String, into)))]
    #[serde(rename = "items", skip_serializing_if = "Vec::is_empty", default)]
    items: Vec<String>,
    #[builder(default, list(item(type = i32)))]
    #[serde(rename = "primitiveItems", skip_serializing_if = "Vec::is_empty", default)]
    primitive_items: Vec<i32>,
    #[builder(default, list(item(type = f64)))]
    #[serde(rename = "doubleItems", skip_serializing_if = "Vec::is_empty", default)]
    #[derive_with(with = conjure_object::private::DoubleWrapper)]
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
