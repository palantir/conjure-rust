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
pub struct CreateDatasetRequest {
    #[builder(into)]
    #[serde(rename = "fileSystemId")]
    file_system_id: String,
    #[builder(into)]
    #[serde(rename = "path")]
    path: String,
}
impl CreateDatasetRequest {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(file_system_id: impl Into<String>, path: impl Into<String>) -> Self {
        Self::builder().file_system_id(file_system_id).path(path).build()
    }
    #[inline]
    pub fn file_system_id(&self) -> &str {
        &*self.file_system_id
    }
    #[inline]
    pub fn path(&self) -> &str {
        &*self.path
    }
}
