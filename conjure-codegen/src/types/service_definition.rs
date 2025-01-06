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
pub struct ServiceDefinition {
    #[builder(custom(type = super::TypeName, convert = Box::new))]
    #[serde(rename = "serviceName")]
    service_name: Box<super::TypeName>,
    #[builder(default, list(item(type = super::EndpointDefinition)))]
    #[serde(rename = "endpoints", skip_serializing_if = "Vec::is_empty", default)]
    endpoints: Vec<super::EndpointDefinition>,
    #[builder(default, into)]
    #[serde(rename = "docs", skip_serializing_if = "Option::is_none", default)]
    docs: Option<super::Documentation>,
}
impl ServiceDefinition {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(service_name: super::TypeName) -> Self {
        Self::builder().service_name(service_name).build()
    }
    #[inline]
    pub fn service_name(&self) -> &super::TypeName {
        &*self.service_name
    }
    #[inline]
    pub fn endpoints(&self) -> &[super::EndpointDefinition] {
        &*self.endpoints
    }
    #[inline]
    pub fn docs(&self) -> Option<&super::Documentation> {
        self.docs.as_ref().map(|o| &*o)
    }
}
