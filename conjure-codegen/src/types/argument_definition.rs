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
pub struct ArgumentDefinition {
    #[serde(rename = "argName")]
    arg_name: super::ArgumentName,
    #[builder(custom(type = super::Type, convert = Box::new))]
    #[serde(rename = "type")]
    type_: Box<super::Type>,
    #[builder(custom(type = super::ParameterType, convert = Box::new))]
    #[serde(rename = "paramType")]
    param_type: Box<super::ParameterType>,
    #[builder(default, into)]
    #[serde(rename = "safety", skip_serializing_if = "Option :: is_none", default)]
    safety: Option<super::LogSafety>,
    #[builder(default, into)]
    #[serde(rename = "docs", skip_serializing_if = "Option :: is_none", default)]
    docs: Option<super::Documentation>,
    #[builder(default, list(item(type = super::Type)))]
    #[serde(rename = "markers", skip_serializing_if = "Vec :: is_empty", default)]
    markers: Vec<super::Type>,
    #[builder(default, set(item(type = String, into)))]
    #[serde(
        rename = "tags",
        skip_serializing_if = "std :: collections :: BTreeSet :: is_empty",
        default
    )]
    tags: std::collections::BTreeSet<String>,
}
impl ArgumentDefinition {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(
        arg_name: super::ArgumentName,
        type_: super::Type,
        param_type: super::ParameterType,
    ) -> Self {
        Self::builder().arg_name(arg_name).type_(type_).param_type(param_type).build()
    }
    #[inline]
    pub fn arg_name(&self) -> &super::ArgumentName {
        &self.arg_name
    }
    #[inline]
    pub fn type_(&self) -> &super::Type {
        &*self.type_
    }
    #[inline]
    pub fn param_type(&self) -> &super::ParameterType {
        &*self.param_type
    }
    #[inline]
    pub fn safety(&self) -> Option<&super::LogSafety> {
        self.safety.as_ref().map(|o| &*o)
    }
    #[inline]
    pub fn docs(&self) -> Option<&super::Documentation> {
        self.docs.as_ref().map(|o| &*o)
    }
    #[inline]
    pub fn markers(&self) -> &[super::Type] {
        &*self.markers
    }
    #[inline]
    pub fn tags(&self) -> &std::collections::BTreeSet<String> {
        &self.tags
    }
}
