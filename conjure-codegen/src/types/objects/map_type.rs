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
pub struct MapType {
    #[builder(custom(type = super::Type, convert = Box::new))]
    #[serde(rename = "keyType")]
    key_type: Box<super::Type>,
    #[builder(custom(type = super::Type, convert = Box::new))]
    #[serde(rename = "valueType")]
    value_type: Box<super::Type>,
}
impl MapType {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(key_type: super::Type, value_type: super::Type) -> Self {
        Self::builder().key_type(key_type).value_type(value_type).build()
    }
    #[inline]
    pub fn key_type(&self) -> &super::Type {
        &*self.key_type
    }
    #[inline]
    pub fn value_type(&self) -> &super::Type {
        &*self.value_type
    }
}
