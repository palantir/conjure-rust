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
pub struct Dataset {
    #[builder(into)]
    #[serde(rename = "fileSystemId")]
    file_system_id: String,
    #[serde(rename = "rid")]
    rid: conjure_object::ResourceIdentifier,
}
impl Dataset {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(
        file_system_id: impl Into<String>,
        rid: conjure_object::ResourceIdentifier,
    ) -> Self {
        Self::builder().file_system_id(file_system_id).rid(rid).build()
    }
    #[inline]
    pub fn file_system_id(&self) -> &str {
        &*self.file_system_id
    }
    /// Uniquely identifies this dataset.
    #[inline]
    pub fn rid(&self) -> &conjure_object::ResourceIdentifier {
        &self.rid
    }
}
