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
use conjure_error::{Error, ErrorCode, ErrorType};
use conjure_http::client::{Body, Client};
use conjure_object::serde::de::DeserializeOwned;
use conjure_object::serde::Serialize;
use http::{Request, Response, StatusCode};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;

use crate::types::*;

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

    let e = conjure_serde::json::client_from_str::<TestUnion>(json)
        .err()
        .unwrap();
    assert!(e.is_data());
}

#[test]
fn optional_field_constructor() {
    let builder = OptionalConstructorFields::builder()
        .list(vec![1, 2])
        .string("hi".to_string())
        .integer(3)
        .build();
    let constructor = OptionalConstructorFields::new(vec![1, 2], "hi", 3);
    assert_eq!(builder, constructor);

    let builder = OptionalConstructorFields2::builder()
        .object(TestObject::new(0))
        .build();
    let constructor = OptionalConstructorFields2::new(TestObject::new(0));
    assert_eq!(builder, constructor);
}

// just make sure that things end up in the right modules
#[test]
fn subpackage() {
    SuperpackageObject::new(foo::SubpackageObject::new(IntegerAlias(1)));
    bar::baz::OtherSubpackageObject::new(foo::SubpackageObject::new(IntegerAlias(1)));
}

#[test]
fn error_serialization() {
    let error = SimpleError::new("hello", 15, false);

    assert_eq!(error.code(), ErrorCode::Internal);
    assert_eq!(error.name(), "Test:SimpleError");
    assert_eq!(error.safe_args(), &["bar", "foo"]);

    let encoded = conjure_error::encode(&error);

    assert_eq!(*encoded.error_code(), ErrorCode::Internal);
    assert_eq!(encoded.error_name(), "Test:SimpleError");

    let mut params = BTreeMap::new();
    params.insert("foo".to_string(), "hello".to_string());
    params.insert("bar".to_string(), "15".to_string());
    params.insert("unsafeFoo".to_string(), "false".to_string());
    assert_eq!(*encoded.parameters(), params);
}

struct TestClient<F>(F);

impl<F> TestClient<F>
where
    F: Fn(Request<Body>) -> Result<Response<&'static [u8]>, Error>,
{
    fn new(f: F) -> TestClient<F> {
        TestClient(f)
    }
}

impl<F> Client for TestClient<F>
where
    F: Fn(Request<Body>) -> Result<Response<&'static [u8]>, Error>,
{
    type ResponseBody = &'static [u8];

    fn request(&self, request: Request<Body>) -> Result<Response<&'static [u8]>, Error> {
        (self.0)(request)
    }
}

#[test]
fn all_optional_query_params() {
    let client = TestServiceClient::new(TestClient::new(|req| {
        assert_eq!(
            req.uri(),
            "/test/allOptionalQueryParams?bar=hi&bar=there&baz=2"
        );
        Ok(Response::builder()
            .status(StatusCode::NO_CONTENT)
            .body(&[][..])
            .unwrap())
    }));

    let mut set = BTreeSet::new();
    set.insert(2);
    client
        .all_optional_query_params(None, &["hi".to_string(), "there".to_string()], &set)
        .unwrap();

    let client = TestServiceClient::new(TestClient::new(|req| {
        assert_eq!(req.uri(), "/test/allOptionalQueryParams");
        Ok(Response::builder()
            .status(StatusCode::NO_CONTENT)
            .body(&[][..])
            .unwrap())
    }));

    client
        .all_optional_query_params(None, &[], &BTreeSet::new())
        .unwrap();
}

#[test]
fn partially_optional_query_params() {
    let client = TestServiceClient::new(TestClient::new(|req| {
        assert_eq!(req.uri(), "/test/partiallyOptionalQueryParams?bar=hi");
        Ok(Response::builder()
            .status(StatusCode::NO_CONTENT)
            .body(&[][..])
            .unwrap())
    }));

    client.partially_optional_query_params(None, "hi").unwrap();

    let client = TestServiceClient::new(TestClient::new(|req| {
        assert_eq!(req.uri(), "/test/partiallyOptionalQueryParams?bar=hi&foo=2");
        Ok(Response::builder()
            .status(StatusCode::NO_CONTENT)
            .body(&[][..])
            .unwrap())
    }));

    client
        .partially_optional_query_params(Some(2), "hi")
        .unwrap();
}
