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
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use std::f64;
use std::fmt::Debug;

fn serialize<T>(value: &T) -> String
where
    T: Serialize,
{
    let mut buf = vec![];
    value
        .serialize(crate::Serializer::new(&mut serde_json::Serializer::new(
            &mut buf,
        )))
        .unwrap();
    String::from_utf8(buf).unwrap()
}

fn deserialize_client<T>(json: &str) -> T
where
    T: DeserializeOwned,
{
    T::deserialize(crate::ClientDeserializer::new(
        &mut serde_json::Deserializer::from_str(json),
    ))
    .unwrap()
}

fn deserialize_server<T>(json: &str) -> T
where
    T: DeserializeOwned,
{
    T::deserialize(crate::ServerDeserializer::new(
        &mut serde_json::Deserializer::from_str(json),
    ))
    .unwrap()
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

    let deserialized = deserialize_client(json);
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

    let e = Foo::deserialize(crate::ServerDeserializer::new(
        &mut serde_json::Deserializer::from_str(json),
    ))
    .err()
    .unwrap();

    assert!(e.is_data());
    assert!(e.to_string().contains("foo"));
    assert!(e.to_string().contains("bogus"));
}
