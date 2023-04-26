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

use crate::test::RemoteBody;
use crate::types::*;
use async_trait::async_trait;
use bytes::{Bytes, BytesMut};
use conjure_error::Error;
use conjure_http::client::{
    AsyncClient, AsyncRequestBody, AsyncService, AsyncWriteBody, Client, DeserializeResponse,
    DisplaySeqParamEncoder, JsonResponseDeserializer, RequestBody, SerializeRequest, Service,
    WriteBody,
};
use conjure_macros::{endpoint, service};
use conjure_object::{BearerToken, ResourceIdentifier};
use futures::executor;
use http::header::CONTENT_TYPE;
use http::{HeaderMap, HeaderValue, Method, Request, Response, StatusCode};
use std::collections::{BTreeMap, BTreeSet};
use std::pin::Pin;

struct StreamingBody<'a>(&'a [u8]);

impl WriteBody<Vec<u8>> for StreamingBody<'_> {
    fn write_body(&mut self, w: &mut Vec<u8>) -> Result<(), Error> {
        w.extend_from_slice(self.0);
        Ok(())
    }

    fn reset(&mut self) -> bool {
        true
    }
}

#[async_trait]
impl AsyncWriteBody<Vec<u8>> for StreamingBody<'_> {
    async fn write_body(self: Pin<&mut Self>, mut w: Pin<&mut Vec<u8>>) -> Result<(), Error> {
        w.extend_from_slice(self.0);
        Ok(())
    }

    async fn reset(self: Pin<&mut Self>) -> bool {
        true
    }
}

#[derive(Debug, PartialEq)]
enum TestBody<T = Vec<u8>> {
    Empty,
    Json(String),
    Streaming(T),
}

struct TestClient {
    method: Method,
    path: &'static str,
    headers: HeaderMap,
    body: TestBody,
    response: TestBody,
}

impl TestClient {
    fn new(method: Method, path: &'static str) -> TestClient {
        TestClient {
            method,
            path,
            headers: HeaderMap::new(),
            body: TestBody::Empty,
            response: TestBody::Empty,
        }
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
    type BodyWriter = Vec<u8>;
    type ResponseBody = RemoteBody;

    fn send(
        &self,
        req: Request<RequestBody<'_, Self::BodyWriter>>,
    ) -> Result<Response<Self::ResponseBody>, Error> {
        assert_eq!(*req.method(), self.method);
        assert_eq!(*req.uri(), self.path);
        assert_eq!(*req.headers(), self.headers);

        let body = match req.into_body() {
            RequestBody::Empty => TestBody::Empty,
            RequestBody::Fixed(body) => TestBody::Json(String::from_utf8(body.to_vec()).unwrap()),
            RequestBody::Streaming(body) => {
                let mut buf = vec![];
                body.write_body(&mut buf).unwrap();
                TestBody::Streaming(buf)
            }
        };
        assert_eq!(body, self.body);

        match &self.response {
            TestBody::Empty => Ok(Response::builder()
                .status(StatusCode::NO_CONTENT)
                .body(RemoteBody(vec![]))
                .unwrap()),
            TestBody::Json(json) => Ok(Response::builder()
                .status(StatusCode::OK)
                .header(CONTENT_TYPE, "application/json")
                .body(RemoteBody(json.as_bytes().to_vec()))
                .unwrap()),
            TestBody::Streaming(buf) => Ok(Response::builder()
                .status(StatusCode::OK)
                .header(CONTENT_TYPE, "application/octet-stream")
                .body(RemoteBody(buf.clone()))
                .unwrap()),
        }
    }
}

#[async_trait]
impl AsyncClient for &'_ TestClient {
    type BodyWriter = Vec<u8>;
    type ResponseBody = RemoteBody;

    async fn send(
        &self,
        req: Request<AsyncRequestBody<'_, Self::BodyWriter>>,
    ) -> Result<Response<Self::ResponseBody>, Error> {
        assert_eq!(*req.method(), self.method);
        assert_eq!(*req.uri(), self.path);
        assert_eq!(*req.headers(), self.headers);

        let body = match req.into_body() {
            AsyncRequestBody::Empty => TestBody::Empty,
            AsyncRequestBody::Fixed(body) => {
                TestBody::Json(String::from_utf8(body.to_vec()).unwrap())
            }
            AsyncRequestBody::Streaming(mut writer) => {
                let mut buf = vec![];
                writer.as_mut().write_body(Pin::new(&mut buf)).await?;
                TestBody::Streaming(buf)
            }
        };
        assert_eq!(body, self.body);

        match &self.response {
            TestBody::Empty => Ok(Response::builder()
                .status(StatusCode::NO_CONTENT)
                .body(RemoteBody(vec![]))
                .unwrap()),
            TestBody::Json(json) => Ok(Response::builder()
                .status(StatusCode::OK)
                .header(CONTENT_TYPE, "application/json")
                .body(RemoteBody(json.as_bytes().to_vec()))
                .unwrap()),
            TestBody::Streaming(buf) => Ok(Response::builder()
                .status(StatusCode::OK)
                .header(CONTENT_TYPE, "application/octet-stream")
                .body(RemoteBody(buf.clone()))
                .unwrap()),
        }
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
    let client = TestClient::new(
        Method::GET,
        "/test/queryParams?normal=hello%20world&custom=10&list=1&list=2&set=true",
    )
    .header("Accept", "application/json");
    let mut set = BTreeSet::new();
    set.insert(true);
    check!(
        client,
        client.query_params("hello world", Some(10), &[1, 2], &set)
    );

    let client = TestClient::new(Method::GET, "/test/queryParams?normal=foo")
        .header("Accept", "application/json");
    check!(
        client,
        client.query_params("foo", None, &[], &BTreeSet::new())
    );
}

#[test]
fn path_params() {
    let client = TestClient::new(
        Method::GET,
        "/test/pathParams/hello%20world/false/raw/ri.conjure.main.test.foo",
    )
    .header("Accept", "application/json");

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
    let client = TestClient::new(Method::GET, "/test/headers")
        .header("Some-Custom-Header", "hello world")
        .header("Accept", "application/json");
    check!(client, client.headers("hello world", None));

    let client = TestClient::new(Method::GET, "/test/headers")
        .header("Some-Custom-Header", "hello world")
        .header("Some-Optional-Header", "2")
        .header("Accept", "application/json");
    check!(client, client.headers("hello world", Some(2)));
}

#[test]
fn empty_request() {
    let client =
        TestClient::new(Method::POST, "/test/emptyRequest").header("Accept", "application/json");
    check!(client, client.empty_request());
}

#[test]
fn unexpected_json_response() {
    let client = TestClient::new(Method::POST, "/test/emptyRequest")
        .header("Accept", "application/json")
        .response(TestBody::Json(r#""hello world""#.to_string()));
    check!(client, client.empty_request());
}

#[test]
fn json_request() {
    let client = TestClient::new(Method::POST, "/test/jsonRequest")
        .header("Content-Type", "application/json")
        .header("Content-Length", "13")
        .header("Accept", "application/json")
        .body(TestBody::Json(r#""hello world""#.to_string()));
    check!(client, client.json_request("hello world"));
}

#[test]
fn optional_json_request() {
    let client = TestClient::new(Method::POST, "/test/optionalJsonRequest")
        .header("Content-Type", "application/json")
        .header("Content-Length", "13")
        .header("Accept", "application/json")
        .body(TestBody::Json(r#""hello world""#.to_string()));
    check!(client, client.optional_json_request(Some("hello world")));

    let client = TestClient::new(Method::POST, "/test/optionalJsonRequest")
        .header("Content-Type", "application/json")
        .header("Content-Length", "4")
        .header("Accept", "application/json")
        .body(TestBody::Json("null".to_string()));
    check!(client, client.optional_json_request(None));
}

#[test]
fn streaming_request() {
    let client = TestClient::new(Method::POST, "/test/streamingRequest")
        .header("Content-Type", "application/octet-stream")
        .header("Accept", "application/json")
        .body(TestBody::Streaming(vec![0, 1, 2, 3]));
    check!(
        client,
        client.streaming_request(StreamingBody(&[0, 1, 2, 3][..]))
    );
}

#[test]
fn streaming_alias_request() {
    let client = TestClient::new(Method::POST, "/test/streamingAliasRequest")
        .header("Content-Type", "application/octet-stream")
        .header("Accept", "application/json")
        .body(TestBody::Streaming(vec![0, 1, 2, 3]));
    check!(
        client,
        client.streaming_alias_request(StreamingBody(&[0, 1, 2, 3][..]))
    );
}

#[test]
fn json_response() {
    let client = TestClient::new(Method::GET, "/test/jsonResponse")
        .header("Accept", "application/json")
        .response(TestBody::Json(r#""hello world""#.to_string()));
    check!(client, client.json_response(), "hello world");
}

#[test]
fn optional_json_response() {
    let client = TestClient::new(Method::GET, "/test/optionalJsonResponse")
        .header("Accept", "application/json")
        .response(TestBody::Json(r#""hello world""#.to_string()));
    check!(
        client,
        client.optional_json_response(),
        Some("hello world".to_string())
    );

    let client = TestClient::new(Method::GET, "/test/optionalJsonResponse")
        .header("Accept", "application/json");
    check!(client, client.optional_json_response(), None);
}

#[test]
fn list_json_response() {
    let client =
        TestClient::new(Method::GET, "/test/listJsonResponse").header("Accept", "application/json");
    check!(client, client.list_json_response(), Vec::<String>::new());

    let client = TestClient::new(Method::GET, "/test/listJsonResponse")
        .header("Accept", "application/json")
        .response(TestBody::Json(r#"["hello"]"#.to_string()));
    check!(
        client,
        client.list_json_response(),
        vec!["hello".to_string()]
    );
}

#[test]
fn set_json_response() {
    let client =
        TestClient::new(Method::GET, "/test/setJsonResponse").header("Accept", "application/json");
    check!(client, client.set_json_response(), BTreeSet::new());

    let client = TestClient::new(Method::GET, "/test/setJsonResponse")
        .header("Accept", "application/json")
        .response(TestBody::Json(r#"["hello"]"#.to_string()));
    let mut set = BTreeSet::new();
    set.insert("hello".to_string());
    check!(client, client.set_json_response(), set);
}

#[test]
fn map_json_response() {
    let client =
        TestClient::new(Method::GET, "/test/mapJsonResponse").header("Accept", "application/json");
    check!(client, client.map_json_response(), BTreeMap::new());

    let client = TestClient::new(Method::GET, "/test/mapJsonResponse")
        .header("Accept", "application/json")
        .response(TestBody::Json(r#"{"hello": "world"}"#.to_string()));
    let mut map = BTreeMap::new();
    map.insert("hello".to_string(), "world".to_string());
    check!(client, client.map_json_response(), map);
}

#[test]
fn streaming_response() {
    let client = TestClient::new(Method::GET, "/test/streamingResponse")
        .header("Accept", "application/octet-stream")
        .response(TestBody::Streaming(b"foobar".to_vec()));
    check!(
        client,
        client.streaming_response(),
        RemoteBody(b"foobar".to_vec())
    );
}

#[test]
fn optional_streaming_response() {
    let client = TestClient::new(Method::GET, "/test/optionalStreamingResponse")
        .header("Accept", "application/octet-stream")
        .response(TestBody::Streaming(b"foobar".to_vec()));
    check!(
        client,
        client.optional_streaming_response(),
        Some(RemoteBody(b"foobar".to_vec()))
    );

    let client = TestClient::new(Method::GET, "/test/optionalStreamingResponse")
        .header("Accept", "application/octet-stream");
    check!(client, client.optional_streaming_response(), None);
}

#[test]
fn streaming_alias_response() {
    let client = TestClient::new(Method::GET, "/test/streamingAliasResponse")
        .header("Accept", "application/octet-stream")
        .response(TestBody::Streaming(b"foobar".to_vec()));
    check!(
        client,
        client.streaming_alias_response(),
        RemoteBody(b"foobar".to_vec())
    );
}

#[test]
fn optional_streaming_alias_response() {
    let client = TestClient::new(Method::GET, "/test/optionalStreamingAliasResponse")
        .header("Accept", "application/octet-stream")
        .response(TestBody::Streaming(b"foobar".to_vec()));
    check!(
        client,
        client.optional_streaming_alias_response(),
        Some(RemoteBody(b"foobar".to_vec()))
    );

    let client = TestClient::new(Method::GET, "/test/optionalStreamingAliasResponse")
        .header("Accept", "application/octet-stream");
    check!(client, client.optional_streaming_alias_response(), None);
}

#[test]
fn header_auth() {
    let client = TestClient::new(Method::GET, "/test/headerAuth")
        .header("Authorization", "Bearer fizzbuzz")
        .header("Accept", "application/json");
    check!(
        client,
        client.header_auth(&BearerToken::new("fizzbuzz").unwrap())
    );
}

#[test]
fn cookie_auth() {
    let client = TestClient::new(Method::GET, "/test/cookieAuth")
        .header("Cookie", "foobar=fizzbuzz")
        .header("Accept", "application/json");
    check!(
        client,
        client.cookie_auth(&BearerToken::new("fizzbuzz").unwrap())
    );
}

#[test]
fn custom_client() {
    #[service]
    trait CustomService {
        #[endpoint(method = POST, path = "/foo")]
        fn post(&self) -> Result<(), Error>;

        #[endpoint(method = POST, path = "/foo")]
        fn post_with_body(&self, #[body] body: &str) -> Result<(), Error>;

        #[endpoint(method = POST, path = "/foo")]
        fn post_plain_text(
            &self,
            #[auth] auth: &BearerToken,
            #[body(serializer = PlainTextRequestSerializer)] body: &str,
        ) -> Result<(), Error>;

        #[endpoint(method = GET, path = "/foo", accept = PlainTextResponseDeserializer)]
        fn get_plain_text(
            &self,
            #[auth(cookie_name = "foobar")] auth: &BearerToken,
        ) -> Result<String, Error>;

        #[endpoint(method = GET, path = "/foo", accept = JsonResponseDeserializer)]
        fn get_json(&self) -> Result<String, Error>;

        #[endpoint(method = GET, path = "/foo")]
        fn header_param(
            &self,
            #[header(name = "Test-Header")] test_header: &str,
        ) -> Result<(), Error>;

        #[endpoint(method = GET, path = "/foo")]
        fn query_param(&self, #[query(name = "queryParam")] query_param: bool)
            -> Result<(), Error>;

        #[endpoint(method = GET, path = "/foo")]
        fn query_params(
            &self,
            #[query(name = "queryParam", encoder = DisplaySeqParamEncoder)] query_params: &[bool],
        ) -> Result<(), Error>;

        #[endpoint(method = GET, path = "/foo/{bar}")]
        fn path_params(&self, #[path] bar: &str) -> Result<(), Error>;
    }
}

enum PlainTextRequestSerializer {}

impl<'a, W> SerializeRequest<'a, &str, W> for PlainTextRequestSerializer {
    fn content_type(_: &&str) -> HeaderValue {
        HeaderValue::from_static("text/plain")
    }

    fn serialize(value: &str) -> Result<RequestBody<'a, W>, Error> {
        Ok(RequestBody::Fixed(BytesMut::from(value).freeze()))
    }
}

enum PlainTextResponseDeserializer {}

impl<R> DeserializeResponse<String, R> for PlainTextResponseDeserializer
where
    R: Iterator<Item = Result<Bytes, Error>>,
{
    fn accept() -> Option<HeaderValue> {
        Some(HeaderValue::from_static("text/plain"))
    }

    fn deserialize(response: Response<R>) -> Result<String, Error> {
        let mut buf = vec![];
        for chunk in response.into_body() {
            buf.extend_from_slice(&chunk?);
        }

        String::from_utf8(buf).map_err(Error::internal_safe)
    }
}
