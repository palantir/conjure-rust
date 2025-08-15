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
pub struct ExternalReference {
    #[builder(custom(type = super::TypeName, convert = Box::new))]
    #[serde(rename = "externalReference")]
    external_reference: Box<super::TypeName>,
    #[builder(custom(type = super::Type, convert = Box::new))]
    #[serde(rename = "fallback")]
    fallback: Box<super::Type>,
}
impl ExternalReference {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(external_reference: super::TypeName, fallback: super::Type) -> Self {
        Self::builder().external_reference(external_reference).fallback(fallback).build()
    }
    /// An identifier for a non-Conjure type which is already defined in a different language (e.g. Java).
    #[inline]
    pub fn external_reference(&self) -> &super::TypeName {
        &*self.external_reference
    }
    /// Other language generators may use the provided fallback if the non-Conjure type is not available. The ANY PrimitiveType is permissible for all external types, but a more specific definition is preferable.
    #[inline]
    pub fn fallback(&self) -> &super::Type {
        &*self.fallback
    }
}
