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
pub struct CovariantOptionalExample {
    #[builder(default, into)]
    #[serde(rename = "item", default)]
    item: Option<conjure_object::Any>,
}
impl CovariantOptionalExample {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new() -> Self {
        Self::builder().build()
    }
    #[inline]
    pub fn item(&self) -> Option<&conjure_object::Any> {
        self.item.as_ref().map(|o| &*o)
    }
}
