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
pub struct FieldDefinition {
    #[serde(rename = "fieldName")]
    field_name: super::FieldName,
    #[builder(custom(type = super::Type, convert = Box::new))]
    #[serde(rename = "type")]
    type_: Box<super::Type>,
    #[builder(default, into)]
    #[serde(rename = "docs", skip_serializing_if = "Option :: is_none", default)]
    docs: Option<super::Documentation>,
    #[builder(default, into)]
    #[serde(rename = "deprecated", skip_serializing_if = "Option :: is_none", default)]
    deprecated: Option<super::Documentation>,
    #[builder(default, into)]
    #[serde(rename = "safety", skip_serializing_if = "Option :: is_none", default)]
    safety: Option<super::LogSafety>,
}
impl FieldDefinition {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(field_name: super::FieldName, type_: super::Type) -> Self {
        Self::builder().field_name(field_name).type_(type_).build()
    }
    #[inline]
    pub fn field_name(&self) -> &super::FieldName {
        &self.field_name
    }
    #[inline]
    pub fn type_(&self) -> &super::Type {
        &*self.type_
    }
    #[inline]
    pub fn docs(&self) -> Option<&super::Documentation> {
        self.docs.as_ref().map(|o| &*o)
    }
    #[inline]
    pub fn deprecated(&self) -> Option<&super::Documentation> {
        self.deprecated.as_ref().map(|o| &*o)
    }
    #[inline]
    pub fn safety(&self) -> Option<&super::LogSafety> {
        self.safety.as_ref().map(|o| &*o)
    }
}
