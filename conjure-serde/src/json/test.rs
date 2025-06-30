// Copyright 2018 Palantir Technologies, Inc.
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
use conjure_object::DoubleKey;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use std::collections::{BTreeMap, BTreeSet};
use std::f64;
use std::fmt::Debug;

fn serialize<T>(value: &T) -> String
where
    T: Serialize,
{
    crate::json::to_string(value).unwrap()
}

fn deserialize_client<T>(json: &str) -> T
where
    T: DeserializeOwned,
{
    crate::json::client_from_str(json).unwrap()
}

fn deserialize_server<T>(json: &str) -> T
where
    T: DeserializeOwned,
{
    crate::json::server_from_str(json).unwrap()
}

fn test_ser<T>(ty: &T, expected_json: &str)
where
    T: Serialize,
{
    let actual_json = serialize(ty);
    let expected_value = serde_json::from_str::<serde_json::Value>(expected_json).unwrap();
    let actual_value = serde_json::from_str::<serde_json::Value>(&actual_json).unwrap();

    assert_eq!(expected_value, actual_value);
}

fn test_de<T>(ty: &T, json: &str)
where
    T: DeserializeOwned + PartialEq + Debug,
{
    let deserialized = deserialize_client(json);
    assert_eq!(*ty, deserialized);

    let deserialized = deserialize_server(json);
    assert_eq!(*ty, deserialized);
}

fn test_serde<T>(ty: &T, expected_json: &str)
where
    T: Serialize + DeserializeOwned + PartialEq + Debug,
{
    test_ser(ty, expected_json);
    test_de(ty, expected_json);
}

#[test]
fn binary_serde() {
    test_serde(&ByteBuf::from(b"foobar".to_vec()), r#""Zm9vYmFy""#);
}

#[test]
fn boolean_keys() {
    test_serde(
        &BTreeMap::from([(false, 0), (true, 1)]),
        r#"{"false":0,"true":1}"#,
    );
}

#[allow(clippy::float_cmp)]
fn test_doubles(value: f64, string: &str) {
    let json = format!(r#""{}""#, string);
    test_ser(&value, &json);

    let deserialized = deserialize_client::<f64>(&json);
    assert!((value.is_nan() && deserialized.is_nan()) || value == deserialized);

    let deserialized = deserialize_server::<f64>(&json);
    assert!((value.is_nan() && deserialized.is_nan()) || value == deserialized);
}

#[test]
fn nonfinite_doubles() {
    test_doubles(f64::INFINITY, "Infinity");
    test_doubles(f64::NEG_INFINITY, "-Infinity");
    test_doubles(f64::NAN, "NaN");
}

#[test]
fn double_keys() {
    test_serde(
        &BTreeMap::from([
            (DoubleKey(f64::NEG_INFINITY), 0),
            (DoubleKey(-1.5), 1),
            (DoubleKey(1.5), 2),
            (DoubleKey(f64::INFINITY), 3),
            (DoubleKey(f64::NAN), 4),
        ]),
        r#"
        {
            "-Infinity": 0,
            "-1.5": 1,
            "1.5": 2,
            "Infinity": 3,
            "NaN": 4
        }
        "#,
    )
}

#[derive(Deserialize, Debug, PartialEq)]
struct Foo {
    foo: i32,
}

#[test]
fn client_unknown_fields() {
    let deserialized = deserialize_client::<Foo>(
        r#"
        {
            "foo": 1,
            "bogus": "hello"
        }
        "#,
    );
    assert_eq!(Foo { foo: 1 }, deserialized);
}

#[test]
fn server_unknown_fields() {
    let json = r#"
    {
        "foo": 1,
        "bogus": "hello"
    }
    "#;

    let e = Foo::deserialize(&mut crate::json::ServerDeserializer::from_str(json))
        .err()
        .unwrap();

    assert!(e.is_data());
    assert!(e.to_string().contains("foo"));
    assert!(e.to_string().contains("bogus"));

    let mut r = json.as_bytes();
    let e = Foo::deserialize(&mut crate::json::ServerDeserializer::from_reader(&mut r))
        .err()
        .unwrap();

    assert!(e.is_data());
    assert!(e.to_string().contains("foo"));
    assert!(e.to_string().contains("bogus"));
}

#[derive(Deserialize, Debug, PartialEq)]
struct Collections {
    list: Vec<u32>,
    set: BTreeSet<u32>,
    map: BTreeMap<String, u32>,
}

#[test]
fn null_collections() {
    let json = r#"
    {
        "list": null,
        "set": null,
        "map": null
    }
    "#;

    let expected = Collections {
        list: vec![],
        set: BTreeSet::new(),
        map: BTreeMap::new(),
    };

    let actual =
        Collections::deserialize(&mut crate::json::ServerDeserializer::from_str(json)).unwrap();
    assert_eq!(expected, actual);

    let actual =
        Collections::deserialize(&mut crate::json::ClientDeserializer::from_str(json)).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn binary_seq() {
    let json = r#"
    [
        "Zm9vYmFy"
    ]
    "#;

    let expected = vec![ByteBuf::from("foobar")];

    let actual =
        Vec::<ByteBuf>::deserialize(&mut crate::json::ServerDeserializer::from_str(json)).unwrap();
    assert_eq!(expected, actual);

    let actual =
        Vec::<ByteBuf>::deserialize(&mut crate::json::ClientDeserializer::from_str(json)).unwrap();
    assert_eq!(expected, actual);
}
