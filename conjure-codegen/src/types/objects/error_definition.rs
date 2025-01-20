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
pub struct ErrorDefinition {
    #[builder(custom(type = super::TypeName, convert = Box::new))]
    #[serde(rename = "errorName")]
    error_name: Box<super::TypeName>,
    #[builder(default, into)]
    #[serde(rename = "docs", skip_serializing_if = "Option::is_none", default)]
    docs: Option<super::Documentation>,
    #[serde(rename = "namespace")]
    namespace: super::ErrorNamespace,
    #[serde(rename = "code")]
    code: super::ErrorCode,
    #[builder(default, list(item(type = super::FieldDefinition)))]
    #[serde(rename = "safeArgs", skip_serializing_if = "Vec::is_empty", default)]
    safe_args: Vec<super::FieldDefinition>,
    #[builder(default, list(item(type = super::FieldDefinition)))]
    #[serde(rename = "unsafeArgs", skip_serializing_if = "Vec::is_empty", default)]
    unsafe_args: Vec<super::FieldDefinition>,
}
impl ErrorDefinition {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(
        error_name: super::TypeName,
        namespace: super::ErrorNamespace,
        code: super::ErrorCode,
    ) -> Self {
        Self::builder().error_name(error_name).namespace(namespace).code(code).build()
    }
    #[inline]
    pub fn error_name(&self) -> &super::TypeName {
        &*self.error_name
    }
    #[inline]
    pub fn docs(&self) -> Option<&super::Documentation> {
        self.docs.as_ref().map(|o| &*o)
    }
    #[inline]
    pub fn namespace(&self) -> &super::ErrorNamespace {
        &self.namespace
    }
    #[inline]
    pub fn code(&self) -> &super::ErrorCode {
        &self.code
    }
    #[inline]
    pub fn safe_args(&self) -> &[super::FieldDefinition] {
        &*self.safe_args
    }
    #[inline]
    pub fn unsafe_args(&self) -> &[super::FieldDefinition] {
        &*self.unsafe_args
    }
}
