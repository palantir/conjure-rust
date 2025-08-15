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
pub struct SetType {
    #[builder(custom(type = super::Type, convert = Box::new))]
    #[serde(rename = "itemType")]
    item_type: Box<super::Type>,
}
impl SetType {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(item_type: super::Type) -> Self {
        Self::builder().item_type(item_type).build()
    }
    #[inline]
    pub fn item_type(&self) -> &super::Type {
        &*self.item_type
    }
}
