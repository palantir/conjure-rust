/// Invalid Conjure service definition.
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
pub struct InvalidServiceDefinition {
    #[builder(into)]
    #[serde(rename = "serviceName")]
    service_name: String,
    #[builder(
        custom(
            type = impl
            conjure_object::serde::Serialize,
            convert = |v|conjure_object::Any::new(v).expect("value failed to serialize")
        )
    )]
    #[serde(rename = "serviceDef")]
    service_def: conjure_object::Any,
}
impl InvalidServiceDefinition {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(
        service_name: impl Into<String>,
        service_def: impl conjure_object::serde::Serialize,
    ) -> Self {
        Self::builder().service_name(service_name).service_def(service_def).build()
    }
    /// Name of the invalid service definition.
    #[inline]
    pub fn service_name(&self) -> &str {
        &*self.service_name
    }
    /// Details of the invalid service definition.
    #[inline]
    pub fn service_def(&self) -> &conjure_object::Any {
        &self.service_def
    }
}
impl conjure_error::ErrorType for InvalidServiceDefinition {
    #[inline]
    fn code(&self) -> conjure_error::ErrorCode {
        conjure_error::ErrorCode::InvalidArgument
    }
    #[inline]
    fn name(&self) -> &str {
        "Conjure:InvalidServiceDefinition"
    }
    #[inline]
    fn instance_id(&self) -> Option<conjure_object::Uuid> {
        None
    }
    #[inline]
    fn safe_args(&self) -> &'static [&'static str] {
        &["serviceName"]
    }
}
