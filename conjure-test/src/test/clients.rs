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
use conjure_http::client::{
    AsyncClient, AsyncRequestBody, AsyncVisitRequestBody, AsyncWriteBody, Client, RequestBody,
    VisitRequestBody, VisitResponse, WriteBody,
};
use conjure_http::{PathParams, QueryParams};
use conjure_object::serde::Serialize;
use conjure_object::{BearerToken, ResourceIdentifier};
use conjure_serde::json;
use futures::executor;
use http::{HeaderMap, Method};
use std::collections::{BTreeMap, BTreeSet};
use std::future::Future;
use std::pin::Pin;

use crate::types::*;

#[derive(Debug, PartialEq)]
enum TestBody<T = Vec<u8>> {
    Empty,
    Json(String),
    Streaming(T),
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

impl<'b> Client for &'b TestClient {
    type BinaryWriter = Vec<u8>;
    type BinaryBody = Vec<u8>;

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
        T: RequestBody<'a, Vec<u8>>,
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

impl<'b> AsyncClient for &'b TestClient {
    type BinaryWriter = Vec<u8>;
    type BinaryBody = Vec<u8>;

    fn request<'a, T, U>(
        &'a self,
        method: Method,
        path: &'static str,
        path_params: PathParams,
        query_params: QueryParams,
        headers: HeaderMap,
        body: T,
        response_visitor: U,
    ) -> Pin<Box<dyn Future<Output = Result<U::Output, Error>> + Send + 'a>>
    where
        T: AsyncRequestBody<'a, Self::BinaryWriter> + Send + 'a,
        U: VisitResponse<Self::BinaryBody> + Send + 'a,
    {
        let f = async move {
            assert_eq!(method, self.method);
            assert_eq!(method, self.method);
            assert_eq!(path, self.path);
            assert_eq!(path_params, self.path_params);
            assert_eq!(query_params, self.query_params);
            assert_eq!(headers, self.headers);
            let body = match body.accept(TestBodyVisitor) {
                TestBody::Empty => TestBody::Empty,
                TestBody::Json(b) => TestBody::Json(b),
                TestBody::Streaming(mut writer) => {
                    let mut buf = vec![];
                    writer.as_mut().write_body(Pin::new(&mut buf)).await?;
                    TestBody::Streaming(buf)
                }
            };
            assert_eq!(body, self.body);

            match &self.response {
                TestBody::Empty => response_visitor.visit_empty(),
                TestBody::Json(json) => response_visitor
                    .visit_serializable(&mut json::ClientDeserializer::from_slice(json.as_bytes())),
                TestBody::Streaming(buf) => response_visitor.visit_binary(buf.clone()),
            }
        };

        Box::pin(f)
    }
}

struct TestBodyVisitor;

impl<'a> VisitRequestBody<'a, Vec<u8>> for TestBodyVisitor {
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
        T: WriteBody<Vec<u8>> + 'a,
    {
        let mut buf = vec![];
        body.write_body(&mut buf).unwrap();
        TestBody::Streaming(buf)
    }
}

impl<'a> AsyncVisitRequestBody<'a, Vec<u8>> for TestBodyVisitor {
    type Output = TestBody<Pin<Box<dyn AsyncWriteBody<Vec<u8>> + Sync + Send + 'a>>>;

    fn visit_empty(self) -> Self::Output {
        TestBody::Empty
    }

    fn visit_serializable<T>(self, body: T) -> Self::Output
    where
        T: Serialize + 'a,
    {
        let body = json::to_string(&body).unwrap();
        TestBody::Json(body)
    }

    fn visit_binary<T>(self, body: T) -> Self::Output
    where
        T: AsyncWriteBody<Vec<u8>> + Sync + Send + 'a,
    {
        TestBody::Streaming(Box::pin(body))
    }
}

macro_rules! check {
    ($client:ident, $call:expr) => {
        check!($client, $call, ());
    };
    ($client:ident, $call:expr, $expected_response:expr) => {{
        let raw_client = $client;
        let $client = TestServiceClient::new(&raw_client);
        let response = $call.unwrap();
        assert_eq!(response, $expected_response);

        let $client = TestServiceAsyncClient::new(&raw_client);
        let response = executor::block_on($call).unwrap();
        assert_eq!(response, $expected_response);
    }};
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
    check!(
        client,
        client.query_params("hello world", Some(10), &[1, 2], &set)
    );

    let client = TestClient::new(Method::GET, "/test/queryParams").query_param("normal", "foo");
    check!(
        client,
        client.query_params("foo", None, &[], &BTreeSet::new())
    );
}

#[test]
fn path_params() {
    let client = TestClient::new(Method::GET, "/test/pathParams/{foo}/{bar}/raw/{baz}")
        .path_param("foo", "hello world")
        .path_param("bar", "false")
        .path_param("baz", "ri.conjure.main.test.foo");

    check!(
        client,
        client.path_params(
            "hello world",
            false,
            &ResourceIdentifier::new("ri.conjure.main.test.foo").unwrap(),
        )
    );
}

#[test]
fn headers() {
    let client =
        TestClient::new(Method::GET, "/test/headers").header("Some-Custom-Header", "hello world");
    check!(client, client.headers("hello world", None));

    let client = TestClient::new(Method::GET, "/test/headers")
        .header("Some-Custom-Header", "hello world")
        .header("Some-Optional-Header", "2");
    check!(client, client.headers("hello world", Some(2)));
}

#[test]
fn empty_request() {
    let client = TestClient::new(Method::POST, "/test/emptyRequest");
    check!(client, client.empty_request());
}

#[test]
fn json_request() {
    let client = TestClient::new(Method::POST, "/test/jsonRequest")
        .body(TestBody::Json(r#""hello world""#.to_string()));
    check!(client, client.json_request("hello world"));
}

#[test]
fn optional_json_request() {
    let client = TestClient::new(Method::POST, "/test/optionalJsonRequest")
        .body(TestBody::Json(r#""hello world""#.to_string()));
    check!(client, client.optional_json_request(Some("hello world")));

    let client = TestClient::new(Method::POST, "/test/optionalJsonRequest")
        .body(TestBody::Json("null".to_string()));
    check!(client, client.optional_json_request(None));
}

#[test]
fn streaming_request() {
    let client = TestClient::new(Method::POST, "/test/streamingRequest")
        .body(TestBody::Streaming(vec![0, 1, 2, 3]));
    check!(client, client.streaming_request(&[0, 1, 2, 3][..]));
}

#[test]
fn streaming_alias_request() {
    let client = TestClient::new(Method::POST, "/test/streamingAliasRequest")
        .body(TestBody::Streaming(vec![0, 1, 2, 3]));
    check!(client, client.streaming_alias_request(&[0, 1, 2, 3][..]))
}

#[test]
fn json_response() {
    let client = TestClient::new(Method::GET, "/test/jsonResponse")
        .response(TestBody::Json(r#""hello world""#.to_string()));
    check!(client, client.json_response(), "hello world");
}

#[test]
fn optional_json_response() {
    let client = TestClient::new(Method::GET, "/test/optionalJsonResponse")
        .response(TestBody::Json(r#""hello world""#.to_string()));
    check!(
        client,
        client.optional_json_response(),
        Some("hello world".to_string())
    );

    let client = TestClient::new(Method::GET, "/test/optionalJsonResponse");
    check!(client, client.optional_json_response(), None);
}

#[test]
fn list_json_response() {
    let client = TestClient::new(Method::GET, "/test/listJsonResponse");
    check!(client, client.list_json_response(), Vec::<String>::new());

    let client = TestClient::new(Method::GET, "/test/listJsonResponse")
        .response(TestBody::Json(r#"["hello"]"#.to_string()));
    check!(
        client,
        client.list_json_response(),
        vec!["hello".to_string()]
    );
}

#[test]
fn set_json_response() {
    let client = TestClient::new(Method::GET, "/test/setJsonResponse");
    check!(client, client.set_json_response(), BTreeSet::new());

    let client = TestClient::new(Method::GET, "/test/setJsonResponse")
        .response(TestBody::Json(r#"["hello"]"#.to_string()));
    let mut set = BTreeSet::new();
    set.insert("hello".to_string());
    check!(client, client.set_json_response(), set);
}

#[test]
fn map_json_response() {
    let client = TestClient::new(Method::GET, "/test/mapJsonResponse");
    check!(client, client.map_json_response(), BTreeMap::new());

    let client = TestClient::new(Method::GET, "/test/mapJsonResponse")
        .response(TestBody::Json(r#"{"hello": "world"}"#.to_string()));
    let mut map = BTreeMap::new();
    map.insert("hello".to_string(), "world".to_string());
    check!(client, client.map_json_response(), map);
}

#[test]
fn streaming_response() {
    let client = TestClient::new(Method::GET, "/test/streamingResponse")
        .response(TestBody::Streaming(b"foobar".to_vec()));
    check!(client, client.streaming_response(), b"foobar".to_vec());
}

#[test]
fn optional_streaming_response() {
    let client = TestClient::new(Method::GET, "/test/optionalStreamingResponse")
        .response(TestBody::Streaming(b"foobar".to_vec()));
    check!(
        client,
        client.optional_streaming_response(),
        Some(b"foobar".to_vec())
    );

    let client = TestClient::new(Method::GET, "/test/optionalStreamingResponse");
    check!(client, client.optional_streaming_response(), None);
}

#[test]
fn streaming_alias_response() {
    let client = TestClient::new(Method::GET, "/test/streamingAliasResponse")
        .response(TestBody::Streaming(b"foobar".to_vec()));
    check!(
        client,
        client.streaming_alias_response(),
        b"foobar".to_vec()
    );
}

#[test]
fn optional_streaming_alias_response() {
    let client = TestClient::new(Method::GET, "/test/optionalStreamingAliasResponse")
        .response(TestBody::Streaming(b"foobar".to_vec()));
    check!(
        client,
        client.optional_streaming_alias_response(),
        Some(b"foobar".to_vec())
    );

    let client = TestClient::new(Method::GET, "/test/optionalStreamingAliasResponse");
    check!(client, client.optional_streaming_alias_response(), None);
}

#[test]
fn header_auth() {
    let client =
        TestClient::new(Method::GET, "/test/headerAuth").header("Authorization", "Bearer fizzbuzz");
    check!(
        client,
        client.header_auth(&BearerToken::new("fizzbuzz").unwrap())
    );
}

#[test]
fn cookie_auth() {
    let client =
        TestClient::new(Method::GET, "/test/cookieAuth").header("Cookie", "foobar=fizzbuzz");
    check!(
        client,
        client.cookie_auth(&BearerToken::new("fizzbuzz").unwrap())
    );
}
