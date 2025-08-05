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
pub struct QueryParameterType {
    #[serde(rename = "paramId")]
    param_id: super::ParameterId,
}
impl QueryParameterType {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(param_id: super::ParameterId) -> Self {
        Self::builder().param_id(param_id).build()
    }
    #[inline]
    pub fn param_id(&self) -> &super::ParameterId {
        &self.param_id
    }
}
