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
pub struct SetExample {
    #[builder(default, set(item(type = String, into)))]
    #[serde(
        rename = "items",
        skip_serializing_if = "std :: collections :: BTreeSet :: is_empty",
        default
    )]
    items: std::collections::BTreeSet<String>,
}
impl SetExample {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new() -> Self {
        Self::builder().build()
    }
    #[inline]
    pub fn items(&self) -> &std::collections::BTreeSet<String> {
        &self.items
    }
}
