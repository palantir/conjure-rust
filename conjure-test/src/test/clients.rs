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

use conjure_error::Error;
use conjure_http::client::{Client, RequestBody, VisitRequestBody, VisitResponse, WriteBody};
use conjure_http::{PathParams, QueryParams};
use conjure_object::serde::Serialize;
use conjure_object::{BearerToken, ResourceIdentifier};
use conjure_serde::json;
use http::{HeaderMap, Method};
use std::collections::{BTreeMap, BTreeSet};

use crate::types::*;

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
        let body = body.accept(TestBodyVisitor);
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

    fn visit_empty(self) -> TestBody {
        TestBody::Empty
    }

    fn visit_serializable<T>(self, body: T) -> TestBody
    where
        T: Serialize + 'a,
    {
        let body = json::to_string(&body).unwrap();
        TestBody::Json(body)
    }

    fn visit_binary<T>(self, mut body: T) -> TestBody
    where
        T: WriteBody + 'a,
    {
        let mut buf = vec![];
        body.write_body(&mut buf).unwrap();
        TestBody::Streaming(buf)
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
