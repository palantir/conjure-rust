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
pub struct StringExample {
    #[builder(into)]
    #[serde(rename = "string")]
    string: String,
}
impl StringExample {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(string: impl Into<String>) -> Self {
        Self::builder().string(string).build()
    }
    #[inline]
    pub fn string(&self) -> &str {
        &*self.string
    }
}
