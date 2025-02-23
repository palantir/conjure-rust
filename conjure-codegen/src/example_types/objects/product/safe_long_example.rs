#[derive(
    Debug,
    Clone,
    conjure_object::serde::Serialize,
    conjure_object::serde::Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Copy
)]
#[serde(crate = "conjure_object::serde")]
#[conjure_object::private::staged_builder::staged_builder]
#[builder(crate = conjure_object::private::staged_builder, update, inline)]
pub struct SafeLongExample {
    #[serde(rename = "safeLongValue")]
    safe_long_value: conjure_object::SafeLong,
}
impl SafeLongExample {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(safe_long_value: conjure_object::SafeLong) -> Self {
        Self::builder().safe_long_value(safe_long_value).build()
    }
    #[inline]
    pub fn safe_long_value(&self) -> conjure_object::SafeLong {
        self.safe_long_value
    }
}
