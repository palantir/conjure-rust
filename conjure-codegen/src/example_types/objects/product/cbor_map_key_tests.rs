/// Test maps with various key types for CBOR Java compatibility
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
    #[builder(default, map(key(type = String, into), value(type = String, into)))]
    #[serde(
        rename = "stringKeyMap",
        skip_serializing_if = "std::collections::BTreeMap::is_empty",
        default
    )]
    string_key_map: std::collections::BTreeMap<String, String>,
    #[builder(
        default,
        map(key(type = conjure_object::Uuid), value(type = String, into))
    )]
    #[serde(
        rename = "uuidKeyMap",
        serialize_with = "conjure_serde::cbor::serialize_map_keys_as_strings",
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
    /// String keys are the standard case
    #[inline]
    pub fn string_key_map(&self) -> &std::collections::BTreeMap<String, String> {
        &self.string_key_map
    }
    /// UUID keys get stringified for Java compatibility
    #[inline]
    pub fn uuid_key_map(
        &self,
    ) -> &std::collections::BTreeMap<conjure_object::Uuid, String> {
        &self.uuid_key_map
    }
}
