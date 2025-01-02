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
pub struct EndpointDefinition {
    #[serde(rename = "endpointName")]
    endpoint_name: super::EndpointName,
    #[serde(rename = "httpMethod")]
    http_method: super::HttpMethod,
    #[serde(rename = "httpPath")]
    http_path: super::HttpPath,
    #[builder(
        default,
        custom(
            type = impl
            Into<Option<super::AuthType>>,
            convert = |v|v.into().map(Box::new)
        )
    )]
    #[serde(rename = "auth", skip_serializing_if = "Option :: is_none", default)]
    auth: Option<Box<super::AuthType>>,
    #[builder(default, list(item(type = super::ArgumentDefinition)))]
    #[serde(rename = "args", skip_serializing_if = "Vec :: is_empty", default)]
    args: Vec<super::ArgumentDefinition>,
    #[builder(
        default,
        custom(
            type = impl
            Into<Option<super::Type>>,
            convert = |v|v.into().map(Box::new)
        )
    )]
    #[serde(rename = "returns", skip_serializing_if = "Option :: is_none", default)]
    returns: Option<Box<super::Type>>,
    #[builder(default, into)]
    #[serde(rename = "docs", skip_serializing_if = "Option :: is_none", default)]
    docs: Option<super::Documentation>,
    #[builder(default, into)]
    #[serde(rename = "deprecated", skip_serializing_if = "Option :: is_none", default)]
    deprecated: Option<super::Documentation>,
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
impl EndpointDefinition {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(
        endpoint_name: super::EndpointName,
        http_method: super::HttpMethod,
        http_path: super::HttpPath,
    ) -> Self {
        Self::builder()
            .endpoint_name(endpoint_name)
            .http_method(http_method)
            .http_path(http_path)
            .build()
    }
    #[inline]
    pub fn endpoint_name(&self) -> &super::EndpointName {
        &self.endpoint_name
    }
    #[inline]
    pub fn http_method(&self) -> &super::HttpMethod {
        &self.http_method
    }
    #[inline]
    pub fn http_path(&self) -> &super::HttpPath {
        &self.http_path
    }
    #[inline]
    pub fn auth(&self) -> Option<&super::AuthType> {
        self.auth.as_ref().map(|o| &**o)
    }
    #[inline]
    pub fn args(&self) -> &[super::ArgumentDefinition] {
        &*self.args
    }
    #[inline]
    pub fn returns(&self) -> Option<&super::Type> {
        self.returns.as_ref().map(|o| &**o)
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
    pub fn markers(&self) -> &[super::Type] {
        &*self.markers
    }
    #[inline]
    pub fn tags(&self) -> &std::collections::BTreeSet<String> {
        &self.tags
    }
}
