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
pub struct ConjureDefinition {
    #[serde(rename = "version")]
    version: i32,
    #[builder(default, list(item(type = super::ErrorDefinition)))]
    #[serde(rename = "errors", skip_serializing_if = "Vec :: is_empty", default)]
    errors: Vec<super::ErrorDefinition>,
    #[builder(default, list(item(type = super::TypeDefinition)))]
    #[serde(rename = "types", skip_serializing_if = "Vec :: is_empty", default)]
    types: Vec<super::TypeDefinition>,
    #[builder(default, list(item(type = super::ServiceDefinition)))]
    #[serde(rename = "services", skip_serializing_if = "Vec :: is_empty", default)]
    services: Vec<super::ServiceDefinition>,
    #[builder(
        default,
        map(
            key(type = String, into),
            value(
                custom(
                    type = impl
                    conjure_object::serde::Serialize,
                    convert = |v|conjure_object::Any::new(
                        v
                    ).expect("value failed to serialize")
                )
            )
        )
    )]
    #[serde(
        rename = "extensions",
        skip_serializing_if = "std :: collections :: BTreeMap :: is_empty",
        default
    )]
    extensions: std::collections::BTreeMap<String, conjure_object::Any>,
}
impl ConjureDefinition {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(version: i32) -> Self {
        Self::builder().version(version).build()
    }
    #[inline]
    pub fn version(&self) -> i32 {
        self.version
    }
    #[inline]
    pub fn errors(&self) -> &[super::ErrorDefinition] {
        &*self.errors
    }
    #[inline]
    pub fn types(&self) -> &[super::TypeDefinition] {
        &*self.types
    }
    #[inline]
    pub fn services(&self) -> &[super::ServiceDefinition] {
        &*self.services
    }
    #[inline]
    pub fn extensions(
        &self,
    ) -> &std::collections::BTreeMap<String, conjure_object::Any> {
        &self.extensions
    }
}
