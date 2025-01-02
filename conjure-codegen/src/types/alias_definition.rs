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
pub struct AliasDefinition {
    #[builder(custom(type = super::TypeName, convert = Box::new))]
    #[serde(rename = "typeName")]
    type_name: Box<super::TypeName>,
    #[builder(custom(type = super::Type, convert = Box::new))]
    #[serde(rename = "alias")]
    alias: Box<super::Type>,
    #[builder(default, into)]
    #[serde(rename = "docs", skip_serializing_if = "Option :: is_none", default)]
    docs: Option<super::Documentation>,
    #[builder(default, into)]
    #[serde(rename = "safety", skip_serializing_if = "Option :: is_none", default)]
    safety: Option<super::LogSafety>,
}
impl AliasDefinition {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(type_name: super::TypeName, alias: super::Type) -> Self {
        Self::builder().type_name(type_name).alias(alias).build()
    }
    #[inline]
    pub fn type_name(&self) -> &super::TypeName {
        &*self.type_name
    }
    #[inline]
    pub fn alias(&self) -> &super::Type {
        &*self.alias
    }
    #[inline]
    pub fn docs(&self) -> Option<&super::Documentation> {
        self.docs.as_ref().map(|o| &*o)
    }
    #[inline]
    pub fn safety(&self) -> Option<&super::LogSafety> {
        self.safety.as_ref().map(|o| &*o)
    }
}
