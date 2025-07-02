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
pub struct EnumFieldExample {
    #[serde(rename = "enum")]
    enum_: super::EnumExample,
}
impl EnumFieldExample {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(enum_: super::EnumExample) -> Self {
        Self::builder().enum_(enum_).build()
    }
    #[inline]
    pub fn enum_(&self) -> &super::EnumExample {
        &self.enum_
    }
}
