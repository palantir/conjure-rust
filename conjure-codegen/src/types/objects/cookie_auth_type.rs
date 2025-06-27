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
pub struct CookieAuthType {
    #[builder(into)]
    #[serde(rename = "cookieName")]
    cookie_name: String,
}
impl CookieAuthType {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(cookie_name: impl Into<String>) -> Self {
        Self::builder().cookie_name(cookie_name).build()
    }
    #[inline]
    pub fn cookie_name(&self) -> &str {
        &*self.cookie_name
    }
}
