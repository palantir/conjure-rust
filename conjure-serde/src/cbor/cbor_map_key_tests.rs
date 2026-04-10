#[cfg(test)]
#[test]
fn test_cbor_map_key_bidirectional() {
    use conjure_codegen::example_types::objects::product::CborMapKeyTests;
    use conjure_object::Uuid;
    use std::collections::BTreeMap;
    let mut string_map = BTreeMap::new();
    string_map.insert("key1".to_string(), "value1".to_string());
    string_map.insert("key2".to_string(), "value2".to_string());

    let mut uuid_map = BTreeMap::new();
    uuid_map.insert(
        Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap(),
        "first".to_string(),
    );
    uuid_map.insert(
        Uuid::parse_str("6ba7b810-9dad-11d1-80b4-00c04fd430c8").unwrap(),
        "second".to_string(),
    );

    let original = CborMapKeyTests::builder()
        .string_key_map(string_map.clone())
        .uuid_key_map(uuid_map.clone())
        .build();

    // Serialize to CBOR
    let cbor_bytes = crate::cbor::to_vec(&original).expect("Failed to serialize to CBOR");

    // Deserialize from CBOR (client mode)
    let deserialized: CborMapKeyTests =
        crate::cbor::client_from_slice(&cbor_bytes).expect("Failed to deserialize from CBOR");

    // Verify bidirectional serialization works
    assert_eq!(original.string_key_map(), deserialized.string_key_map());
    assert_eq!(original.uuid_key_map(), deserialized.uuid_key_map());

    // Verify UUID keys were serialized as strings (not binary)
    // We do this by checking that we can deserialize with string keys
    #[derive(serde::Deserialize)]
    struct StringKeyVersion {
        #[serde(rename = "stringKeyMap")]
        string_key_map: BTreeMap<String, String>,
        #[serde(rename = "uuidKeyMap")]
        uuid_key_map: BTreeMap<String, String>,
    }

    let string_version: StringKeyVersion =
        serde_cbor_2::from_slice(&cbor_bytes).expect("UUID keys should be strings");

    assert_eq!(
        string_version.string_key_map.get("key1"),
        Some(&"value1".to_string())
    );
    assert_eq!(
        string_version
            .uuid_key_map
            .get("550e8400-e29b-41d4-a716-446655440000"),
        Some(&"first".to_string())
    );
    assert_eq!(
        string_version
            .uuid_key_map
            .get("6ba7b810-9dad-11d1-80b4-00c04fd430c8"),
        Some(&"second".to_string())
    );
}
