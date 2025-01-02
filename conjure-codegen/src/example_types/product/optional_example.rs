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
pub struct OptionalExample {
    #[builder(default, into)]
    #[serde(rename = "item", skip_serializing_if = "Option :: is_none", default)]
    item: Option<String>,
}
impl OptionalExample {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new() -> Self {
        Self::builder().build()
    }
    #[inline]
    pub fn item(&self) -> Option<&str> {
        self.item.as_ref().map(|o| &**o)
    }
}
