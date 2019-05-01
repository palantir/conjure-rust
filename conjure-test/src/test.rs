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
use conjure_http::client::{Client, RequestBody, VisitRequestBody, VisitResponse, WriteBody};
use conjure_http::{PathParams, QueryParams};
use conjure_object::serde::de::DeserializeOwned;
use conjure_object::serde::Serialize;
use conjure_object::{BearerToken, ResourceIdentifier};
use conjure_serde::json;
use http::{HeaderMap, Method};
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

#[derive(Debug, PartialEq)]
enum TestBody {
    Empty,
    Json(String),
    Streaming(Vec<u8>),
}

struct TestClient {
    method: Method,
    path: &'static str,
    path_params: PathParams,
    query_params: QueryParams,
    headers: HeaderMap,
    body: TestBody,
    response: TestBody,
}

impl TestClient {
    fn new(method: Method, path: &'static str) -> TestClient {
        TestClient {
            method,
            path,
            path_params: PathParams::new(),
            query_params: QueryParams::new(),
            headers: HeaderMap::new(),
            body: TestBody::Empty,
            response: TestBody::Empty,
        }
    }

    fn path_param(mut self, key: &str, value: &str) -> TestClient {
        self.path_params.insert(key, value);
        self
    }

    fn query_param(mut self, key: &str, value: &str) -> TestClient {
        self.query_params.insert(key, value);
        self
    }

    fn header(mut self, key: &'static str, value: &str) -> TestClient {
        self.headers.insert(key, value.parse().unwrap());
        self
    }

    fn body(mut self, body: TestBody) -> TestClient {
        self.body = body;
        self
    }

    fn response(mut self, response: TestBody) -> TestClient {
        self.response = response;
        self
    }
}

impl Client for TestClient {
    type ResponseBody = Vec<u8>;

    fn request<'a, T, U>(
        &self,
        method: Method,
        path: &'static str,
        path_params: PathParams,
        query_params: QueryParams,
        headers: HeaderMap,
        body: T,
        response_visitor: U,
    ) -> Result<U::Output, Error>
    where
        T: RequestBody<'a>,
        U: VisitResponse<Vec<u8>>,
    {
        assert_eq!(method, self.method);
        assert_eq!(path, self.path);
        assert_eq!(path_params, self.path_params);
        assert_eq!(query_params, self.query_params);
        assert_eq!(headers, self.headers);
        let body = body.accept(TestBodyVisitor).unwrap();
        assert_eq!(body, self.body);

        match &self.response {
            TestBody::Empty => response_visitor.visit_empty(),
            TestBody::Json(json) => response_visitor
                .visit_serializable(&mut json::ClientDeserializer::from_slice(json.as_bytes())),
            TestBody::Streaming(buf) => response_visitor.visit_binary(buf.clone()),
        }
    }
}

struct TestBodyVisitor;

impl<'a> VisitRequestBody<'a> for TestBodyVisitor {
    type Output = TestBody;

    fn visit_empty(self) -> Result<TestBody, Error> {
        Ok(TestBody::Empty)
    }

    fn visit_serializable<T>(self, body: T) -> Result<TestBody, Error>
    where
        T: Serialize + 'a,
    {
        let body = json::to_string(&body).unwrap();
        Ok(TestBody::Json(body))
    }

    fn visit_binary<T>(self, mut body: T) -> Result<TestBody, Error>
    where
        T: WriteBody + 'a,
    {
        let mut buf = vec![];
        body.write_body(&mut buf).unwrap();
        Ok(TestBody::Streaming(buf))
    }
}

#[test]
fn query_params() {
    let client = TestClient::new(Method::GET, "/test/queryParams")
        .query_param("normal", "hello world")
        .query_param("custom", "10")
        .query_param("list", "1")
        .query_param("list", "2")
        .query_param("set", "true");

    let mut set = BTreeSet::new();
    set.insert(true);
    TestServiceClient::new(client)
        .query_params("hello world", Some(10), &[1, 2], &set)
        .unwrap();

    let client = TestClient::new(Method::GET, "/test/queryParams").query_param("normal", "foo");

    TestServiceClient::new(client)
        .query_params("foo", None, &[], &BTreeSet::new())
        .unwrap();
}

#[test]
fn path_params() {
    let client = TestClient::new(Method::GET, "/test/pathParams/{foo}/{bar}/raw/{baz}")
        .path_param("foo", "hello world")
        .path_param("bar", "false")
        .path_param("baz", "ri.conjure.main.test.foo");

    TestServiceClient::new(client)
        .path_params(
            "hello world",
            false,
            &ResourceIdentifier::new("ri.conjure.main.test.foo").unwrap(),
        )
        .unwrap();
}

#[test]
fn headers() {
    let client =
        TestClient::new(Method::GET, "/test/headers").header("Some-Custom-Header", "hello world");

    TestServiceClient::new(client)
        .headers("hello world", None)
        .unwrap();
    let client = TestClient::new(Method::GET, "/test/headers")
        .header("Some-Custom-Header", "hello world")
        .header("Some-Optional-Header", "2");

    TestServiceClient::new(client)
        .headers("hello world", Some(2))
        .unwrap();
}

#[test]
fn empty_request() {
    let client = TestClient::new(Method::POST, "/test/emptyRequest");

    TestServiceClient::new(client).empty_request().unwrap();
}

#[test]
fn json_request() {
    let client = TestClient::new(Method::POST, "/test/jsonRequest")
        .body(TestBody::Json(r#""hello world""#.to_string()));

    TestServiceClient::new(client)
        .json_request("hello world")
        .unwrap();
}

#[test]
fn optional_json_request() {
    let client = TestClient::new(Method::POST, "/test/optionalJsonRequest")
        .body(TestBody::Json(r#""hello world""#.to_string()));

    TestServiceClient::new(client)
        .optional_json_request(Some("hello world"))
        .unwrap();

    let client = TestClient::new(Method::POST, "/test/optionalJsonRequest")
        .body(TestBody::Json("null".to_string()));

    TestServiceClient::new(client)
        .optional_json_request(None)
        .unwrap();
}

#[test]
fn streaming_request() {
    let client = TestClient::new(Method::POST, "/test/streamingRequest")
        .body(TestBody::Streaming(vec![0, 1, 2, 3]));

    TestServiceClient::new(client)
        .streaming_request(&[0, 1, 2, 3][..])
        .unwrap();
}

#[test]
fn streaming_alias_request() {
    let client = TestClient::new(Method::POST, "/test/streamingAliasRequest")
        .body(TestBody::Streaming(vec![0, 1, 2, 3]));

    TestServiceClient::new(client)
        .streaming_alias_request(&[0, 1, 2, 3][..])
        .unwrap();
}

#[test]
fn json_response() {
    let client = TestClient::new(Method::GET, "/test/jsonResponse")
        .response(TestBody::Json(r#""hello world""#.to_string()));

    let s = TestServiceClient::new(client).json_response().unwrap();
    assert_eq!(s, "hello world");
}

#[test]
fn optional_json_response() {
    let client = TestClient::new(Method::GET, "/test/optionalJsonResponse")
        .response(TestBody::Json(r#""hello world""#.to_string()));

    let s = TestServiceClient::new(client)
        .optional_json_response()
        .unwrap();
    assert_eq!(s, Some("hello world".to_string()));

    let client = TestClient::new(Method::GET, "/test/optionalJsonResponse");

    let s = TestServiceClient::new(client)
        .optional_json_response()
        .unwrap();
    assert_eq!(s, None);
}

#[test]
fn list_json_response() {
    let client = TestClient::new(Method::GET, "/test/listJsonResponse");

    let s = TestServiceClient::new(client).list_json_response().unwrap();
    assert_eq!(s, Vec::<String>::new());

    let client = TestClient::new(Method::GET, "/test/listJsonResponse")
        .response(TestBody::Json(r#"["hello"]"#.to_string()));

    let s = TestServiceClient::new(client).list_json_response().unwrap();
    assert_eq!(s, vec!["hello".to_string()]);
}

#[test]
fn set_json_response() {
    let client = TestClient::new(Method::GET, "/test/setJsonResponse");

    let s = TestServiceClient::new(client).set_json_response().unwrap();
    assert_eq!(s, BTreeSet::new());

    let client = TestClient::new(Method::GET, "/test/setJsonResponse")
        .response(TestBody::Json(r#"["hello"]"#.to_string()));

    let s = TestServiceClient::new(client).set_json_response().unwrap();
    let mut set = BTreeSet::new();
    set.insert("hello".to_string());
    assert_eq!(s, set);
}

#[test]
fn map_json_response() {
    let client = TestClient::new(Method::GET, "/test/mapJsonResponse");

    let s = TestServiceClient::new(client).map_json_response().unwrap();
    assert_eq!(s, BTreeMap::new());

    let client = TestClient::new(Method::GET, "/test/mapJsonResponse")
        .response(TestBody::Json(r#"{"hello": "world"}"#.to_string()));

    let s = TestServiceClient::new(client).map_json_response().unwrap();
    let mut map = BTreeMap::new();
    map.insert("hello".to_string(), "world".to_string());
    assert_eq!(s, map);
}

#[test]
fn streaming_response() {
    let client = TestClient::new(Method::GET, "/test/streamingResponse")
        .response(TestBody::Streaming(b"foobar".to_vec()));

    let r = TestServiceClient::new(client).streaming_response().unwrap();
    assert_eq!(r, b"foobar".to_vec());
}

#[test]
fn optional_streaming_response() {
    let client = TestClient::new(Method::GET, "/test/optionalStreamingResponse")
        .response(TestBody::Streaming(b"foobar".to_vec()));

    let r = TestServiceClient::new(client)
        .optional_streaming_response()
        .unwrap();
    assert_eq!(r, Some(b"foobar".to_vec()));

    let client = TestClient::new(Method::GET, "/test/optionalStreamingResponse");

    let r = TestServiceClient::new(client)
        .optional_streaming_response()
        .unwrap();
    assert_eq!(r, None);
}

#[test]
fn streaming_alias_response() {
    let client = TestClient::new(Method::GET, "/test/streamingAliasResponse")
        .response(TestBody::Streaming(b"foobar".to_vec()));

    let r = TestServiceClient::new(client)
        .streaming_alias_response()
        .unwrap();
    assert_eq!(r, b"foobar".to_vec());
}

#[test]
fn optional_streaming_alias_response() {
    let client = TestClient::new(Method::GET, "/test/optionalStreamingAliasResponse")
        .response(TestBody::Streaming(b"foobar".to_vec()));

    let r = TestServiceClient::new(client)
        .optional_streaming_alias_response()
        .unwrap();
    assert_eq!(r, Some(b"foobar".to_vec()));

    let client = TestClient::new(Method::GET, "/test/optionalStreamingAliasResponse");

    let r = TestServiceClient::new(client)
        .optional_streaming_alias_response()
        .unwrap();
    assert_eq!(r, None);
}

#[test]
fn header_auth() {
    let client =
        TestClient::new(Method::GET, "/test/headerAuth").header("Authorization", "Bearer fizzbuzz");

    TestServiceClient::new(client)
        .header_auth(&BearerToken::new("fizzbuzz").unwrap())
        .unwrap();
}

#[test]
fn cookie_auth() {
    let client =
        TestClient::new(Method::GET, "/test/cookieAuth").header("Cookie", "foobar=fizzbuzz");

    TestServiceClient::new(client)
        .cookie_auth(&BearerToken::new("fizzbuzz").unwrap())
        .unwrap();
}
