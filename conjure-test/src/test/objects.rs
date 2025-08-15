// Copyright 2019 Palantir Technologies, Inc.
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

use crate::types::objects::*;
use bytes::Bytes;
use conjure_object::Any;
use conjure_object::DoubleKey;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use std::f64;
use std::fmt::Debug;

fn serialize<T>(value: &T) -> String
where
    T: Serialize,
{
    conjure_serde::json::to_string(value).unwrap()
}

fn deserialize<T>(json: &str) -> T
where
    T: DeserializeOwned,
{
    conjure_serde::json::client_from_str(json).unwrap()
}

fn deserialize_server<T>(json: &str) -> T
where
    T: DeserializeOwned,
{
    conjure_serde::json::server_from_str(json).unwrap()
}

fn test_ser<T>(ty: &T, expected_json: &str)
where
    T: Serialize,
{
    let actual_json = serialize(ty);
    let expected_value = serde_json::from_str::<serde_json::Value>(expected_json).unwrap();
    let actual_value = serde_json::from_str::<serde_json::Value>(&actual_json).unwrap();
    assert_eq!(expected_value, actual_value);

    let actual_any = Any::new(ty).unwrap();
    let any_json = serialize(&actual_any);
    let expected_value = serde_json::from_str::<serde_json::Value>(expected_json).unwrap();
    let actual_value = serde_json::from_str::<serde_json::Value>(&any_json).unwrap();
    assert_eq!(expected_value, actual_value);
}

fn test_de<T>(ty: &T, json: &str)
where
    T: DeserializeOwned + PartialEq + Debug,
{
    let deserialized = deserialize(json);
    assert_eq!(*ty, deserialized);
    let deserialized = deserialize_server(json);
    assert_eq!(*ty, deserialized);

    let deserialized = deserialize::<Any>(json).deserialize_into().unwrap();
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
fn null_fields() {
    let object = EmptyFields::builder().build();

    test_de(
        &object,
        r#"
        {
            "optional": null,
            "list": null,
            "set": null,
            "map": null
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

    assert!(conjure_serde::json::client_from_str::<TestEnum>(r#""""#).is_err());
    assert!(conjure_serde::json::client_from_str::<TestEnum>(r#""lowercase""#).is_err());
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
        #[allow(deprecated)]
        &TestUnion::Object(TestObject::builder().foo(1).build()),
        r#"{"type": "object", "object": {"foo": 1}}"#,
    );

    let unknown_json = r#"{"type": "foobar", "foobar": 14.3}"#;
    let unknown_value = deserialize::<TestUnion>(unknown_json);
    match &unknown_value {
        TestUnion::Unknown(v) => assert_eq!(v.type_(), "foobar"),
        _ => panic!("invalid variant"),
    }
    test_ser(&unknown_value, unknown_json);
}

#[test]
fn transparent_aliases() {
    test_serde(
        #[allow(deprecated)]
        &TransparentAliases::builder()
            .object_alias(ObjectAlias(TestObject::builder().foo(1).build()))
            .union_alias(UnionAlias(TestUnion::Integer(1)))
            .optional_alias(OptionalAlias(None))
            .list_alias(ListAlias(vec![]))
            .set_alias(SetAlias(BTreeSet::new()))
            .map_alias(MapAlias(BTreeMap::new()))
            .build(),
        r#"
        {
            "objectAlias": {"foo": 1},
            "unionAlias": {"type": "integer", "integer": 1}
        }
        "#,
    );

    test_serde(
        #[allow(deprecated)]
        &TransparentAliases::builder()
            .object_alias(ObjectAlias(TestObject::builder().foo(1).build()))
            .union_alias(UnionAlias(TestUnion::Integer(1)))
            .optional_of_alias(IntegerAlias(1))
            .optional_alias(OptionalAlias(Some(1)))
            .list_alias(ListAlias(vec![1]))
            .set_alias(SetAlias(Some(1).into_iter().collect()))
            .map_alias(MapAlias(Some((1, 1)).into_iter().collect()))
            .optional_of_object_alias(ObjectAlias(TestObject::builder().foo(1).build()))
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

    let e = conjure_serde::json::client_from_str::<TestUnion>(json)
        .err()
        .unwrap();
    assert!(e.is_data());
}

// just make sure that things end up in the right modules
#[test]
fn subpackage() {
    SuperpackageObject::new(foo::SubpackageObject::new(IntegerAlias(1)));
    bar::baz::OtherSubpackageObject::new(foo::SubpackageObject::new(IntegerAlias(1)));
}

#[test]
fn binary() {
    let json = r#"
    {
        "binary": "aGVsbG8gd29ybGQ=",
        "double": "Infinity"
    }
    "#;
    let value = CustomValueHandling::new(b"hello world".to_vec(), f64::INFINITY);
    test_serde(&value, json);
}

#[test]
fn optional_binary_field() {
    let json = r#"
    {
        "binary": "aGVsbG8gd29ybGQ="
    }
    "#;
    let value = OptionalBinaryField::builder()
        .binary(Bytes::from_static(b"hello world"))
        .build();
    test_serde(&value, json);

    let json = "{}";
    let value = OptionalBinaryField::builder().build();
    test_serde(&value, json);
}

#[test]
fn staged_all_required_fields() {
    let json = r#"
    {
        "integer": 1,
        "double": 1.5,
        "string": "hello world"
    }
    "#;
    let value = AllRequiredFields::builder()
        .integer(1)
        .double(1.5)
        .string("hello world")
        .build();
    test_serde(&value, json);
}

#[test]
fn staged_all_optional_fields() {
    let json = r#"
    {
        "optionalString": "hello world",
        "map": {
            "foo": "bar",
            "fizz": "buzz"
        },
        "list": [
            "a",
            "b"
        ],
        "set": [
            "1",
            "2"
        ]
    }
    "#;
    let value = AllOptionalFields::builder()
        .optional_string("hello world".to_string())
        .insert_map("foo", "bar")
        .insert_map("fizz", "buzz")
        .push_list("a")
        .push_list("b")
        .insert_set("1")
        .insert_set("2")
        .build();
    test_serde(&value, json);
}

#[test]
fn staged_mixed_fields() {
    let json = r#"
    {
        "integer": 1,
        "map": {
            "a": "b",
            "c": "d"
        },
        "string": "hello world"
    }
    "#;
    let value = MixedFields::builder()
        .integer(1)
        .string("hello world")
        .insert_map("a", "b")
        .insert_map("c", "d")
        .build();
    test_serde(&value, json);
}

#[test]
fn staged_update_with_from() {
    let json = r#"
    {
        "integer": 1,
        "double": 1.5,
        "string": "foobar"
    }
    "#;
    let original = AllRequiredFields::builder()
        .integer(1)
        .double(1.5)
        .string("hello world")
        .build();
    let updated = all_required_fields::Builder::from(original)
        .string("foobar")
        .build();
    test_serde(&updated, json);
}

#[test]
fn double_keys() {
    let json = r#"
    {
        "doubleMap": {
            "1.5": 1,
            "Infinity": 2,
            "NaN": 3
        },
        "doubleSet": [
            1.5,
            "Infinity",
            "NaN"
        ],
        "aliasMap": {
            "1.5": 1,
            "Infinity": 2,
            "NaN": 3
        },
        "aliasSet": [
            1.5,
            "Infinity",
            "NaN"
        ]
    }
    "#;
    let value = DoubleKeys::builder()
        .insert_double_map(DoubleKey(1.5), 1)
        .insert_double_map(DoubleKey(f64::INFINITY), 2)
        .insert_double_map(DoubleKey(f64::NAN), 3)
        .insert_double_set(DoubleKey(1.5))
        .insert_double_set(DoubleKey(f64::INFINITY))
        .insert_double_set(DoubleKey(f64::NAN))
        .insert_alias_map(DoubleAlias(1.5), 1)
        .insert_alias_map(DoubleAlias(f64::INFINITY), 2)
        .insert_alias_map(DoubleAlias(f64::NAN), 3)
        .insert_alias_set(DoubleAlias(1.5))
        .insert_alias_set(DoubleAlias(f64::INFINITY))
        .insert_alias_set(DoubleAlias(f64::NAN))
        .build();
    test_serde(&value, json);
}

#[test]
fn boolean_keys() {
    let json = r#"
    {
        "booleanMap": {
            "false": 0,
            "true": 1
        },
        "booleanSet": [
            false,
            true
        ],
        "aliasMap": {
            "false": 0,
            "true": 1
        },
        "aliasSet": [
            false,
            true
        ]
    }
    "#;
    let value = BooleanKeys::builder()
        .insert_boolean_map(false, 0)
        .insert_boolean_map(true, 1)
        .insert_boolean_set(false)
        .insert_boolean_set(true)
        .insert_alias_map(BooleanAlias(false), 0)
        .insert_alias_map(BooleanAlias(true), 1)
        .insert_alias_set(BooleanAlias(false))
        .insert_alias_set(BooleanAlias(true))
        .build();
    test_serde(&value, json);
}

#[test]
fn nested_maps() {
    let json = r#"
    {
        "maps": {
            "a": {
                "hello": "world"
            }
        }
    }
    "#;
    let value = NestedMap::builder()
        .insert_maps(
            "a".to_string(),
            BTreeMap::from([("hello".to_string(), "world".to_string())]),
        )
        .build();
    test_serde(&value, json);
}

#[test]
fn set_of_objects_with_doubles() {
    let json = r#"
    {
        "set": [
            {
                "integer": 1,
                "double": 1.5,
                "string": "hi"
            },
            {
                "integer": 2,
                "double": "NaN",
                "string": "hello"
            }
        ]
    }
    "#;
    let value = SetOfObjectsWithDoubles::builder()
        .insert_set(
            AllRequiredFields::builder()
                .integer(1)
                .double(1.5)
                .string("hi")
                .build(),
        )
        .insert_set(
            AllRequiredFields::builder()
                .integer(2)
                .double(f64::NAN)
                .string("hello")
                .build(),
        )
        .build();
    test_serde(&value, json);
}

#[test]
fn map_double_values() {
    let json = r#"
    {
        "raw": {
            "foo": 1.0
        },
        "optional": {
            "foo": 1.0,
            "bar": null
        },
        "list": {
            "foo": [
                1.0
            ]
        }
    }
    "#;
    let value = MapDoubleValues::builder()
        .insert_raw("foo", 1.)
        .insert_optional("foo", Some(1.))
        .insert_optional("bar", None)
        .insert_list("foo", vec![1.])
        .build();
    test_serde(&value, json);
}

#[test]
fn double_alias_fromiter() {
    let actual = [(DoubleKey(1.0), "1".to_string())]
        .into_iter()
        .collect::<MapDoubleAlias>();
    assert_eq!(
        actual,
        MapDoubleAlias(BTreeMap::from([(DoubleKey(1.0), "1".to_string())])),
    );

    let actual = [DoubleKey(1.0)].into_iter().collect::<SetDoubleAlias>();
    assert_eq!(actual, SetDoubleAlias(BTreeSet::from([DoubleKey(1.0)])));
}
