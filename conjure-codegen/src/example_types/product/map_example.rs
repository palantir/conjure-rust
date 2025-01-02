#[derive(
    Debug,
    Clone,
    conjure_object::serde::Serialize,
    conjure_object::serde::Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash
)]
#[serde(crate = "conjure_object::serde")]
#[conjure_object::private::staged_builder::staged_builder]
#[builder(crate = conjure_object::private::staged_builder, update, inline)]
pub struct MapExample {
    #[builder(default, map(key(type = String, into), value(type = String, into)))]
    #[serde(
        rename = "items",
        skip_serializing_if = "std :: collections :: BTreeMap :: is_empty",
        default
    )]
    items: std::collections::BTreeMap<String, String>,
}
impl MapExample {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new() -> Self {
        Self::builder().build()
    }
    #[inline]
    pub fn items(&self) -> &std::collections::BTreeMap<String, String> {
        &self.items
    }
}
