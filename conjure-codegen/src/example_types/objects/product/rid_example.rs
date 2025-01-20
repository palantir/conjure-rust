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
pub struct RidExample {
    #[serde(rename = "ridValue")]
    rid_value: conjure_object::ResourceIdentifier,
}
impl RidExample {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(rid_value: conjure_object::ResourceIdentifier) -> Self {
        Self::builder().rid_value(rid_value).build()
    }
    #[inline]
    pub fn rid_value(&self) -> &conjure_object::ResourceIdentifier {
        &self.rid_value
    }
}
