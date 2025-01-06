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
pub struct BinaryExample {
    #[builder(into)]
    #[serde(rename = "binary")]
    binary: conjure_object::Bytes,
}
impl BinaryExample {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(binary: impl Into<conjure_object::Bytes>) -> Self {
        Self::builder().binary(binary).build()
    }
    #[inline]
    pub fn binary(&self) -> &conjure_object::Bytes {
        &self.binary
    }
}
