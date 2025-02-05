///The JSON-serializable representation of an error.
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
pub struct SerializableError {
    #[serde(rename = "errorCode")]
    error_code: super::ErrorCode,
    #[builder(into)]
    #[serde(rename = "errorName")]
    error_name: String,
    #[serde(rename = "errorInstanceId")]
    error_instance_id: conjure_object::Uuid,
    #[builder(default, map(key(type = String, into), value(type = String, into)))]
    #[serde(
        rename = "parameters",
        skip_serializing_if = "std::collections::BTreeMap::is_empty",
        default
    )]
    parameters: std::collections::BTreeMap<String, String>,
}
impl SerializableError {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(
        error_code: super::ErrorCode,
        error_name: impl Into<String>,
        error_instance_id: conjure_object::Uuid,
    ) -> Self {
        Self::builder()
            .error_code(error_code)
            .error_name(error_name)
            .error_instance_id(error_instance_id)
            .build()
    }
    ///The broad category of the error.
    ///
    ///When transmitted over HTTP, this determines the response's status code.
    #[inline]
    pub fn error_code(&self) -> &super::ErrorCode {
        &self.error_code
    }
    ///The error's name.
    ///
    ///The name is made up of a namespace and more specific error name, separated by a `:`.
    #[inline]
    pub fn error_name(&self) -> &str {
        &*self.error_name
    }
    ///A unique identifier for this error instance.
    ///
    ///This can be used to correlate reporting about the error as it transfers between components of a
    ///distributed system.
    #[inline]
    pub fn error_instance_id(&self) -> conjure_object::Uuid {
        self.error_instance_id
    }
    ///Parameters providing more information about the error.
    #[inline]
    pub fn parameters(&self) -> &std::collections::BTreeMap<String, String> {
        &self.parameters
    }
}
