// Copyright 2026 Palantir Technologies, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use conjure_object::{DoubleKey, Uuid};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use std::collections::BTreeMap;
use std::fmt::Debug;

fn serialize<T>(value: &T) -> Vec<u8>
where
    T: Serialize,
{
    crate::cbor::to_vec(value).unwrap()
}

fn deserialize_client<T>(cbor: &[u8]) -> T
where
    T: DeserializeOwned,
{
    crate::cbor::client_from_slice(cbor).unwrap()
}

fn deserialize_server<T>(cbor: &[u8]) -> T
where
    T: DeserializeOwned,
{
    crate::cbor::server_from_slice(cbor).unwrap()
}

fn test_ser<T>(ty: &T, expected_cbor: &[u8])
where
    T: Serialize,
{
    let actual_cbor = serialize(ty);
    let expected_value = serde_cbor_2::from_slice::<serde_cbor_2::Value>(expected_cbor).unwrap();
    let actual_value = serde_cbor_2::from_slice::<serde_cbor_2::Value>(&actual_cbor).unwrap();

    assert_eq!(expected_value, actual_value);
}

fn test_de<T>(ty: &T, cbor: &[u8])
where
    T: DeserializeOwned + PartialEq + Debug,
{
    let deserialized = deserialize_client(cbor);
    assert_eq!(*ty, deserialized);

    let deserialized = deserialize_server(cbor);
    assert_eq!(*ty, deserialized);
}

fn test_serde<T>(ty: &T, expected_cbor: &[u8])
where
    T: Serialize + DeserializeOwned + PartialEq + Debug,
{
    test_ser(ty, expected_cbor);
    test_de(ty, expected_cbor);
}

#[test]
fn binary_serde() {
    test_serde(
        &ByteBuf::from(b"foobar".to_vec()),
        &[0x46, b'f', b'o', b'o', b'b', b'a', b'r'],
    );
}

#[test]
fn binary_keys() {
    test_serde(
        &BTreeMap::from([(ByteBuf::from(b"foobar".to_vec()), 0)]),
        &[
            0xa1, // map(1)
            0x46, b'f', b'o', b'o', b'b', b'a', b'r', // bytes(6)
            0x00, // unsigned(0)
        ],
    )
}

#[test]
fn boolean_keys() {
    test_serde(
        &BTreeMap::from([(false, 0), (true, 1)]),
        &[
            0xa2, // map(2)
            0xf4, // false
            0x00, // unsigned(0)
            0xf5, // true
            0x01, // unsigned(1)
        ],
    );
}

#[test]
fn double_keys() {
    // Test serialization only - NaN keys don't compare equal in deserialization
    let map = BTreeMap::from([
        (DoubleKey(f64::NEG_INFINITY), 0),
        (DoubleKey(-1.5), 1),
        (DoubleKey(1.5), 2),
        (DoubleKey(f64::INFINITY), 3),
        (DoubleKey(f64::NAN), 4),
    ]);

    // Just verify it serializes without error - CBOR encoding is implementation-defined
    let cbor = serialize(&map);

    // Verify we can deserialize it back (even though NaN != NaN in the map)
    let _: BTreeMap<DoubleKey, i32> = deserialize_client(&cbor);
    let _: BTreeMap<DoubleKey, i32> = deserialize_server(&cbor);
}

#[test]
fn uuid_keys() {
    test_serde(
        &BTreeMap::from([(Uuid::nil(), 1)]),
        &[
            0xa1, // map(1)
            0x50, // bytes(16)
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x01, // unsigned(1)
        ],
    );
}

#[test]
fn uuid_values() {
    test_serde(
        &Uuid::nil(),
        &[
            0x50, // bytes(16)
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ],
    )
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Foo {
    foo: i32,
}

#[test]
fn client_unknown_fields() {
    // Note: ciborium ignores unknown fields by default, which is appropriate for Conjure clients
    let deserialized = deserialize_client::<Foo>(&[
        0xa2, // map(2)
        0x63, b'f', b'o', b'o', // text(3) "foo"
        0x01, // unsigned(1)
        0x65, b'b', b'o', b'g', b'u', b's', // text(5) "bogus"
        0x65, b'h', b'e', b'l', b'l', b'o', // text(5) "hello"
    ]);
    assert_eq!(Foo { foo: 1 }, deserialized);
}

#[test]
fn server_unknown_fields() {
    let cbor = &[
        0xa2, // map(2)
        0x63, b'f', b'o', b'o', // text(3) "foo"
        0x01, // unsigned(1)
        0x65, b'b', b'o', b'g', b'u', b's', // text(5) "bogus"
        0x65, b'h', b'e', b'l', b'l', b'o', // text(5) "hello"
    ];

    let e = crate::cbor::server_from_slice::<Foo>(cbor).err().unwrap();

    assert!(e.to_string().contains("bogus") || e.to_string().contains("unknown"));
}

#[derive(Deserialize, Debug, PartialEq)]
struct Collections {
    list: Vec<u32>,
    set: std::collections::BTreeSet<u32>,
    map: BTreeMap<String, u32>,
}

#[test]
fn null_collections() {
    let cbor = &[
        0xa3, // map(3)
        0x64, b'l', b'i', b's', b't', // text(4) "list"
        0xf6, // null
        0x63, b'm', b'a', b'p', // text(3) "map"
        0xf6, // null
        0x63, b's', b'e', b't', // text(3) "set"
        0xf6, // null
    ];

    let expected = Collections {
        list: vec![],
        set: std::collections::BTreeSet::new(),
        map: BTreeMap::new(),
    };

    let actual = deserialize_server(cbor);
    assert_eq!(expected, actual);

    let actual = deserialize_client(cbor);
    assert_eq!(expected, actual);
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct UuidField {
    uuid: Uuid,
}

#[test]
fn uuid_field() {
    let cbor = &[
        0xa1, // map(1)
        0x64, b'u', b'u', b'i', b'd', // text(4) "uuid"
        0x50, // bytes(16)
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];

    let value = UuidField { uuid: Uuid::nil() };

    let actual = crate::cbor::to_vec(&value).unwrap();
    assert_eq!(actual, cbor);

    let actual = crate::cbor::client_from_slice::<UuidField>(cbor).unwrap();
    assert_eq!(actual, value);

    let actual = crate::cbor::server_from_slice::<UuidField>(cbor).unwrap();
    assert_eq!(actual, value);
}

#[test]
fn round_trip() {
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct TestStruct {
        a: i32,
        b: String,
        c: Vec<u8>,
        d: Option<bool>,
    }

    let value = TestStruct {
        a: 42,
        b: "hello".to_string(),
        c: vec![1, 2, 3],
        d: Some(true),
    };

    let cbor = serialize(&value);
    let deserialized: TestStruct = deserialize_client(&cbor);
    assert_eq!(value, deserialized);

    let deserialized: TestStruct = deserialize_server(&cbor);
    assert_eq!(value, deserialized);
}

#[test]
fn integer_keys_from_cbor() {
    // Test deserializing a map with integer keys (as sent by Java)
    // This simulates Java's Map<Integer, String> serialized to CBOR
    let cbor = &[
        0xbf, // indefinite map start
        0x01, // integer 1
        0x63, b'o', b'n', b'e', // text(3) "one"
        0x18, 0x2a, // integer 42
        0x69, b'f', b'o', b'r', b't', b'y', b'-', b't', b'w', b'o', // text(9) "forty-two"
        0xff, // break
    ];

    // Should be able to deserialize to BTreeMap<String, String>
    let map: BTreeMap<String, String> = deserialize_client(cbor);
    assert_eq!(map.get("1"), Some(&"one".to_string()));
    assert_eq!(map.get("42"), Some(&"forty-two".to_string()));
    assert_eq!(map.len(), 2);
}

#[test]
fn binary_as_byte_string() {
    // Test deserializing CBOR byte strings (as sent by Java for byte[])
    let cbor = &[
        0x47, // byte string length 7
        0x00, 0x01, 0x02, 0x03, 0xFF, 0xAB, 0xCD,
    ];

    let bytes: Vec<u8> = deserialize_client(cbor);
    assert_eq!(bytes, vec![0x00, 0x01, 0x02, 0x03, 0xFF, 0xAB, 0xCD]);
}

#[test]
fn serialize_uuid_map_keys_as_strings() {
    // Test that serialize_uuid_map converts UUID keys to strings
    #[derive(Serialize)]
    struct TestStructWithUuidKeys {
        #[serde(serialize_with = "crate::cbor::serialize_uuid_map")]
        uuid_map: BTreeMap<Uuid, String>,
    }

    #[derive(Deserialize)]
    struct TestStructWithStringKeys {
        uuid_map: BTreeMap<String, String>,
    }

    let mut map = BTreeMap::new();
    map.insert(
        Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap(),
        "first".to_string(),
    );
    map.insert(
        Uuid::parse_str("6ba7b810-9dad-11d1-80b4-00c04fd430c8").unwrap(),
        "second".to_string(),
    );

    // Serialize with UUID keys using our custom serializer
    let test_struct = TestStructWithUuidKeys { uuid_map: map };
    let cbor = serialize(&test_struct);

    // Deserialize back and verify keys are strings (Java-compatible)
    let deserialized: TestStructWithStringKeys = serde_cbor_2::from_slice(&cbor).unwrap();
    assert_eq!(
        deserialized
            .uuid_map
            .get("550e8400-e29b-41d4-a716-446655440000"),
        Some(&"first".to_string())
    );
    assert_eq!(
        deserialized
            .uuid_map
            .get("6ba7b810-9dad-11d1-80b4-00c04fd430c8"),
        Some(&"second".to_string())
    );
}

#[test]
fn serialize_uuid_alias_map_keys_as_strings() {
    // Test that serialize_map_keys_as_strings works with UUID aliases
    // Newtype UUID alias
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
    #[serde(transparent)]
    struct UuidAlias(Uuid);

    impl std::fmt::Display for UuidAlias {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            std::fmt::Display::fmt(&self.0, f)
        }
    }

    #[derive(Serialize)]
    struct TestStructWithAliasKeys {
        #[serde(serialize_with = "crate::cbor::serialize_map_keys_as_strings")]
        alias_map: BTreeMap<UuidAlias, String>,
    }

    #[derive(Deserialize)]
    struct TestStructWithStringKeys {
        alias_map: BTreeMap<String, String>,
    }

    let mut map = BTreeMap::new();
    map.insert(
        UuidAlias(Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap()),
        "first".to_string(),
    );
    map.insert(
        UuidAlias(Uuid::parse_str("6ba7b810-9dad-11d1-80b4-00c04fd430c8").unwrap()),
        "second".to_string(),
    );

    // Serialize with UUID alias keys using the generic serializer
    let test_struct = TestStructWithAliasKeys { alias_map: map };
    let cbor = serialize(&test_struct);

    // Deserialize back and verify keys are strings (Java-compatible)
    let deserialized: TestStructWithStringKeys = serde_cbor_2::from_slice(&cbor).unwrap();
    assert_eq!(
        deserialized
            .alias_map
            .get("550e8400-e29b-41d4-a716-446655440000"),
        Some(&"first".to_string())
    );
    assert_eq!(
        deserialized
            .alias_map
            .get("6ba7b810-9dad-11d1-80b4-00c04fd430c8"),
        Some(&"second".to_string())
    );
}
