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
pub struct CborMapKeyTests {
    #[builder(default, map(key(type = String), value(type = String)))]
    #[serde(
        rename = "stringKeyMap",
        skip_serializing_if = "std::collections::BTreeMap::is_empty",
        default
    )]
    string_key_map: std::collections::BTreeMap<String, String>,
    #[builder(default, map(key(type = conjure_object::Uuid), value(type = String)))]
    #[serde(
        rename = "uuidKeyMap",
        serialize_with = "conjure_serde::cbor::serialize_uuid_map",
        skip_serializing_if = "std::collections::BTreeMap::is_empty",
        default
    )]
    uuid_key_map: std::collections::BTreeMap<conjure_object::Uuid, String>,
}
impl CborMapKeyTests {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new() -> Self {
        Self::builder().build()
    }
    #[inline]
    pub fn string_key_map(&self) -> &std::collections::BTreeMap<String, String> {
        &self.string_key_map
    }
    #[inline]
    pub fn uuid_key_map(&self) -> &std::collections::BTreeMap<conjure_object::Uuid, String> {
        &self.uuid_key_map
    }
}
impl Default for CborMapKeyTests {
    fn default() -> Self {
        Self::new()
    }
}
