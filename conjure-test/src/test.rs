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
use conjure_types::serde::de::DeserializeOwned;
use conjure_types::serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;

use crate::types::*;

fn serialize<T>(value: &T) -> String
where
    T: Serialize,
{
    let mut buf = vec![];
    value
        .serialize(&mut conjure_serde::json::Serializer::new(&mut buf))
        .unwrap();
    String::from_utf8(buf).unwrap()
}

fn deserialize<T>(json: &str) -> T
where
    T: DeserializeOwned,
{
    let mut de = conjure_serde::json::ClientDeserializer::from_str(json);
    let v = T::deserialize(&mut de).unwrap();
    de.end().unwrap();
    v
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

#[test]
fn enums() {
    test_serde(&TestEnum::One, r#""ONE""#);
    assert_eq!(TestEnum::One.as_str(), "ONE");

    let bogus = deserialize::<TestEnum>(r#""BOGUS""#);
    match &bogus {
        TestEnum::Unknown(u) => assert_eq!(&**u, "BOGUS"),
        _ => panic!(),
    }
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
fn union_trailing_fields() {
    let json = r#"
    {
        "type": "integer",
        "integer": 10,
        "foo": 1
    }
    "#;

    let e = TestUnion::deserialize(&mut conjure_serde::json::ClientDeserializer::from_str(json))
        .err()
        .unwrap();

    assert!(e.is_data());
}
