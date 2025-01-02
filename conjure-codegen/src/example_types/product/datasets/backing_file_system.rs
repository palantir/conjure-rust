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
pub struct BackingFileSystem {
    #[builder(into)]
    #[serde(rename = "fileSystemId")]
    file_system_id: String,
    #[builder(into)]
    #[serde(rename = "baseUri")]
    base_uri: String,
    #[builder(default, map(key(type = String, into), value(type = String, into)))]
    #[serde(
        rename = "configuration",
        skip_serializing_if = "std :: collections :: BTreeMap :: is_empty",
        default
    )]
    configuration: std::collections::BTreeMap<String, String>,
}
impl BackingFileSystem {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(file_system_id: impl Into<String>, base_uri: impl Into<String>) -> Self {
        Self::builder().file_system_id(file_system_id).base_uri(base_uri).build()
    }
    ///The name by which this file system is identified.
    #[inline]
    pub fn file_system_id(&self) -> &str {
        &*self.file_system_id
    }
    #[inline]
    pub fn base_uri(&self) -> &str {
        &*self.base_uri
    }
    #[inline]
    pub fn configuration(&self) -> &std::collections::BTreeMap<String, String> {
        &self.configuration
    }
}
