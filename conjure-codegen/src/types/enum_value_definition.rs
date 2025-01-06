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
pub struct EnumValueDefinition {
    #[builder(into)]
    #[serde(rename = "value")]
    value: String,
    #[builder(default, into)]
    #[serde(rename = "docs", skip_serializing_if = "Option::is_none", default)]
    docs: Option<super::Documentation>,
    #[builder(default, into)]
    #[serde(rename = "deprecated", skip_serializing_if = "Option::is_none", default)]
    deprecated: Option<super::Documentation>,
}
impl EnumValueDefinition {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(value: impl Into<String>) -> Self {
        Self::builder().value(value).build()
    }
    #[inline]
    pub fn value(&self) -> &str {
        &*self.value
    }
    #[inline]
    pub fn docs(&self) -> Option<&super::Documentation> {
        self.docs.as_ref().map(|o| &*o)
    }
    #[inline]
    pub fn deprecated(&self) -> Option<&super::Documentation> {
        self.deprecated.as_ref().map(|o| &*o)
    }
}
