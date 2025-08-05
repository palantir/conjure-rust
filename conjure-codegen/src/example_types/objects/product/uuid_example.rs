#[derive(
    Debug,
    Clone,
    conjure_object::serde::Serialize,
    conjure_object::serde::Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Copy
)]
#[serde(crate = "conjure_object::serde")]
#[conjure_object::private::staged_builder::staged_builder]
#[builder(crate = conjure_object::private::staged_builder, update, inline)]
pub struct UuidExample {
    #[serde(rename = "uuid")]
    uuid: conjure_object::Uuid,
}
impl UuidExample {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(uuid: conjure_object::Uuid) -> Self {
        Self::builder().uuid(uuid).build()
    }
    #[inline]
    pub fn uuid(&self) -> conjure_object::Uuid {
        self.uuid
    }
}
