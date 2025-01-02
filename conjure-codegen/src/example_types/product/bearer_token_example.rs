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
pub struct BearerTokenExample {
    #[serde(rename = "bearerTokenValue")]
    bearer_token_value: conjure_object::BearerToken,
}
impl BearerTokenExample {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(bearer_token_value: conjure_object::BearerToken) -> Self {
        Self::builder().bearer_token_value(bearer_token_value).build()
    }
    #[inline]
    pub fn bearer_token_value(&self) -> &conjure_object::BearerToken {
        &self.bearer_token_value
    }
}
