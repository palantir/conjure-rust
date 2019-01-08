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
use conjure::serde::de::DeserializeOwned;
use conjure::serde::{Deserialize, Serialize};
use conjure::ByteBuf;
use std::collections::{BTreeMap, BTreeSet};
use std::f64;
use std::fmt::Debug;

use crate::types::*;

fn serialize<T>(value: &T) -> String
where
    T: Serialize,
{
    let mut buf = vec![];
    value
        .serialize(conjure::Serializer::new(&mut serde_json::Serializer::new(
            &mut buf,
        )))
        .unwrap();
    String::from_utf8(buf).unwrap()
}

fn deserialize<T>(json: &str) -> T
where
    T: DeserializeOwned,
{
    T::deserialize(conjure::ClientDeserializer::new(
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
    let deserialized = deserialize(json);
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
    let object = TestBinary::builder()
        .primitive("primitive")
        .optional(ByteBuf::from("optional"))
        .list(Some(ByteBuf::from("list")))
        .set(Some(ByteBuf::from("set")))
        .map(Some((ByteBuf::from("key"), ByteBuf::from("value"))))
        .alias(BinaryAlias(ByteBuf::from("alias")))
        .build();

    test_serde(
        &object,
        r#"
        {
            "primitive": "cHJpbWl0aXZl",
            "optional": "b3B0aW9uYWw=",
            "list": ["bGlzdA=="],
            "set": ["c2V0"],
            "map": {"a2V5": "dmFsdWU="},
            "alias": "YWxpYXM="
        }
        "#,
    );
}

#[test]
fn empty_fields() {
    let object = EmptyFields::builder().build();

    test_serde(&object, "{}");
    test_de(
        &object,
        r#"
        {
            "optional": null,
            "list": [],
            "set": [],
            "map": {}
        }
        "#,
    );
}

#[allow(clippy::float_cmp)]
fn test_doubles(value: f64, string: &str) {
    let json = format!(r#""{}""#, string);
    test_ser(&value, &json);

    let deserialized = deserialize::<f64>(&json);
    assert!((value.is_nan() && deserialized.is_nan()) || value == deserialized);
}

#[test]
fn nonfinite_doubles() {
    test_doubles(f64::INFINITY, "Infinity");
    test_doubles(f64::NEG_INFINITY, "-Infinity");
    test_doubles(f64::NAN, "NaN");
}

#[test]
fn enums() {
    test_serde(&TestEnum::ONE, r#""ONE""#);
    assert_eq!(TestEnum::ONE.as_str(), "ONE");

    let bogus = deserialize::<TestEnum>(r#""BOGUS""#);
    test_ser(&bogus, r#""BOGUS""#);
}

#[test]
fn unions() {
    test_serde(
        &TestUnion::Integer(15),
        r#"{"type": "integer", "integer": 15}"#,
    );
    test_serde(
        &TestUnion::String("hi".to_string()),
        r#"{"type": "string", "string": "hi"}"#,
    );
    test_serde(
        &TestUnion::Object(TestObject::builder().foo(1).build()),
        r#"{"type": "object", "object": {"foo": 1}}"#,
    );

    let unknown_json = r#"{"type": "double", "double": 14.3}"#;
    let unknown_value = deserialize::<TestUnion>(unknown_json);
    match &unknown_value {
        TestUnion::Unknown(v) => assert_eq!(v.type_(), "double"),
        _ => panic!("invalid variant"),
    }
    test_ser(&unknown_value, unknown_json);
}

#[test]
fn transparent_aliases() {
    test_serde(
        &TransparentAliases::builder()
            .optional_alias(OptionalAlias(None))
            .list_alias(ListAlias(vec![]))
            .set_alias(SetAlias(BTreeSet::new()))
            .map_alias(MapAlias(BTreeMap::new()))
            .object_alias(ObjectAlias(TestObject::builder().foo(1).build()))
            .union_alias(UnionAlias(TestUnion::Integer(1)))
            .build(),
        r#"
        {
            "objectAlias": {"foo": 1},
            "unionAlias": {"type": "integer", "integer": 1}
        }
        "#,
    );

    test_serde(
        &TransparentAliases::builder()
            .optional_of_alias(IntegerAlias(1))
            .optional_alias(OptionalAlias(Some(1)))
            .list_alias(ListAlias(vec![1]))
            .set_alias(SetAlias(Some(1).into_iter().collect()))
            .map_alias(MapAlias(Some((1, 1)).into_iter().collect()))
            .object_alias(ObjectAlias(TestObject::builder().foo(1).build()))
            .optional_of_object_alias(ObjectAlias(TestObject::builder().foo(1).build()))
            .union_alias(UnionAlias(TestUnion::Integer(1)))
            .optional_of_union_alias(UnionAlias(TestUnion::Integer(1)))
            .optional_object_alias(OptionalObjectAlias(Some(
                TestObject::builder().foo(1).build(),
            )))
            .build(),
        r#"
        {
            "optionalOfAlias": 1,
            "optionalAlias": 1,
            "listAlias": [1],
            "setAlias": [1],
            "mapAlias": {"1": 1},
            "objectAlias": {"foo": 1},
            "optionalOfObjectAlias": {"foo": 1},
            "unionAlias": {"type": "integer", "integer": 1},
            "optionalOfUnionAlias": {"type": "integer", "integer": 1},
            "optionalObjectAlias": {"foo": 1}
        }
        "#,
    )
}

#[test]
fn client_unknown_fields() {
    test_de(
        &TestObject::builder().foo(1).build(),
        r#"
        {
            "foo": 1,
            "bogus": "hi there"
        }
        "#,
    );
}

#[test]
fn server_unknown_fields() {
    let json = r#"
    {
        "foo": 1,
        "bogus": "hi there"
    }
    "#;

    let e = TestObject::deserialize(conjure::ServerDeserializer::new(
        &mut serde_json::Deserializer::from_str(json),
    ))
    .err()
    .unwrap();

    assert!(e.is_data());
    assert!(e.to_string().contains("foo"));
    assert!(e.to_string().contains("bogus"));
}

#[test]
fn union_trailing_fields() {
    let json = r#"
    {
        "type": "integer",
        "integer": 10,
        "foo": 1
    }
    "#;

    let e = TestUnion::deserialize(conjure::ClientDeserializer::new(
        &mut serde_json::Deserializer::from_str(json),
    ))
    .err()
    .unwrap();

    assert!(e.is_data());
}
