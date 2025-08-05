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
use crate::types::clients::*;
use conjure_error::Error;
use conjure_http::client::{
    AsyncClient, AsyncRequestBody, AsyncService, AsyncWriteBody, Client,
    ConjureResponseDeserializer, DeserializeResponse, DisplaySeqEncoder, Endpoint,
    LocalAsyncClient, LocalAsyncRequestBody, LocalAsyncWriteBody, RequestBody, SerializeRequest,
    Service, WriteBody,
};
use conjure_macros::{conjure_client, endpoint};
use conjure_object::{BearerToken, ResourceIdentifier};
use futures::executor;
use http::header::CONTENT_TYPE;
use http::{HeaderMap, HeaderValue, Method, Request, Response, StatusCode};
use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;
use std::marker::PhantomData;
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
    endpoint: Option<Endpoint>,
    body: TestBody,
    response: TestBody,
}

impl TestClient {
    fn new(method: Method, path: &'static str) -> TestClient {
        TestClient {
            method,
            path,
            headers: HeaderMap::new(),
            endpoint: None,
            body: TestBody::Empty,
            response: TestBody::Empty,
        }
    }

    fn header(mut self, key: &'static str, value: &str) -> TestClient {
        self.headers.insert(key, value.parse().unwrap());
        self
    }

    fn endpoint(mut self, endpoint: Endpoint) -> TestClient {
        self.endpoint = Some(endpoint);
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

impl Client for &TestClient {
    type BodyWriter = Vec<u8>;
    type ResponseBody = RemoteBody;

    fn send(
        &self,
        req: Request<RequestBody<'_, Self::BodyWriter>>,
    ) -> Result<Response<Self::ResponseBody>, Error> {
        assert_eq!(*req.method(), self.method);
        assert_eq!(*req.uri(), self.path);
        assert_eq!(*req.headers(), self.headers);

        if let Some(endpoint) = &self.endpoint {
            assert_eq!(endpoint, req.extensions().get::<Endpoint>().unwrap());
        }

        let body = match req.into_body() {
            RequestBody::Empty => TestBody::Empty,
            RequestBody::Fixed(body) => TestBody::Json(String::from_utf8(body.to_vec()).unwrap()),
            RequestBody::Streaming(mut body) => {
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

        if let Some(endpoint) = &self.endpoint {
            assert_eq!(endpoint, req.extensions().get::<Endpoint>().unwrap());
        }

        let body = match req.into_body() {
            AsyncRequestBody::Empty => TestBody::Empty,
            AsyncRequestBody::Fixed(body) => {
                TestBody::Json(String::from_utf8(body.to_vec()).unwrap())
            }
            AsyncRequestBody::Streaming(mut writer) => {
                let mut buf = vec![];
                Pin::new(&mut writer).write_body(Pin::new(&mut buf)).await?;
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

impl LocalAsyncClient for &'_ TestClient {
    type BodyWriter = Vec<u8>;
    type ResponseBody = RemoteBody;

    async fn send(
        &self,
        req: Request<LocalAsyncRequestBody<'_, Self::BodyWriter>>,
    ) -> Result<Response<Self::ResponseBody>, Error> {
        assert_eq!(*req.method(), self.method);
        assert_eq!(*req.uri(), self.path);
        assert_eq!(*req.headers(), self.headers);

        if let Some(endpoint) = &self.endpoint {
            assert_eq!(endpoint, req.extensions().get::<Endpoint>().unwrap());
        }

        let body = match req.into_body() {
            LocalAsyncRequestBody::Empty => TestBody::Empty,
            LocalAsyncRequestBody::Fixed(body) => {
                TestBody::Json(String::from_utf8(body.to_vec()).unwrap())
            }
            LocalAsyncRequestBody::Streaming(mut writer) => {
                let mut buf = vec![];
                Pin::new(&mut writer).write_body(Pin::new(&mut buf)).await?;
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

macro_rules! check_custom {
    ($client:ident, $call:expr) => {
        check_custom!($client, $call, ());
    };
    ($client:ident, $call:expr, $expected_response:expr) => {{
        let raw_client = $client;
        let $client = CustomServiceClient::new(&raw_client);
        let response = $call.unwrap();
        assert_eq!(response, $expected_response);

        let $client = CustomServiceAsyncClient::new(&raw_client);
        let response = executor::block_on($call).unwrap();
        assert_eq!(response, $expected_response);
    }};
}

#[conjure_client]
trait CustomService {
    #[endpoint(method = GET, path = "/test/queryParams")]
    fn query_param(
        &self,
        #[query(name = "normal")] normal: &str,
        #[query(name = "list", encoder = DisplaySeqEncoder)] list: &[i32],
    ) -> Result<(), Error>;

    #[endpoint(method = GET, path = "/test/pathParams/{foo}/{baz}/raw/{multi}")]
    fn path_param(
        &self,
        #[path] foo: &str,
        #[path(name = "baz")] bar: i32,
        #[path(encoder = DisplaySeqEncoder)] multi: &[&str],
    ) -> Result<(), Error>;

    #[endpoint(method = GET, path = "/test/headers")]
    fn headers(
        &self,
        #[header(name = "Some-Custom-Header")] custom_header: &str,
        #[header(name = "Some-Optional-Header", encoder = DisplaySeqEncoder)]
        optional_header: Option<i32>,
    ) -> Result<(), Error>;

    #[endpoint(method = POST, path = "/test/jsonRequest")]
    fn json_request(&self, #[body] body: &str) -> Result<(), Error>;

    #[endpoint(method = GET, path = "/test/jsonResponse", accept = ConjureResponseDeserializer)]
    fn json_response(&self) -> Result<String, Error>;

    #[endpoint(method = GET, path = "/test/authHeader")]
    fn auth_header(&self, #[auth] auth: &BearerToken) -> Result<(), Error>;

    #[endpoint(method = GET, path = "/test/cookieHeader")]
    fn cookie_header(
        &self,
        #[auth(cookie_name = "foobar")] auth: &BearerToken,
    ) -> Result<(), Error>;
}

#[conjure_client]
trait CustomServiceAsync {
    #[endpoint(method = GET, path = "/test/queryParams")]
    async fn query_param(
        &self,
        #[query(name = "normal")] normal: &str,
        #[query(name = "list", encoder = DisplaySeqEncoder)] list: &[i32],
    ) -> Result<(), Error>;

    #[endpoint(method = GET, path = "/test/pathParams/{foo}/{baz}/raw/{multi}")]
    async fn path_param(
        &self,
        #[path] foo: &str,
        #[path(name = "baz")] bar: i32,
        #[path(encoder = DisplaySeqEncoder)] multi: &[&str],
    ) -> Result<(), Error>;

    #[endpoint(method = GET, path = "/test/headers")]
    async fn headers(
        &self,
        #[header(name = "Some-Custom-Header")] custom_header: &str,
        #[header(name = "Some-Optional-Header", encoder = DisplaySeqEncoder)]
        optional_header: Option<i32>,
    ) -> Result<(), Error>;

    #[endpoint(method = POST, path = "/test/jsonRequest")]
    async fn json_request(&self, #[body] body: &str) -> Result<(), Error>;

    #[endpoint(method = GET, path = "/test/jsonResponse", accept = ConjureResponseDeserializer)]
    async fn json_response(&self) -> Result<String, Error>;

    #[endpoint(method = GET, path = "/test/authHeader")]
    async fn auth_header(&self, #[auth] auth: &BearerToken) -> Result<(), Error>;

    #[endpoint(method = GET, path = "/test/cookieHeader")]
    async fn cookie_header(
        &self,
        #[auth(cookie_name = "foobar")] auth: &BearerToken,
    ) -> Result<(), Error>;
}

#[test]
fn custom_query_params() {
    let client = TestClient::new(
        Method::GET,
        "/test/queryParams?normal=hello%20world&list=1&list=2",
    );
    check_custom!(client, client.query_param("hello world", &[1, 2]));

    let client = TestClient::new(Method::GET, "/test/queryParams?normal=foo");
    check_custom!(client, client.query_param("foo", &[]));
}

#[test]
fn custom_path_params() {
    let client = TestClient::new(
        Method::GET,
        "/test/pathParams/hello%20world/42/raw/foo/bar%2Fbaz",
    );

    check_custom!(
        client,
        client.path_param("hello world", 42, &["foo", "bar/baz"])
    );
}

#[test]
fn custom_headers() {
    let client =
        TestClient::new(Method::GET, "/test/headers").header("Some-Custom-Header", "hello world");
    check_custom!(client, client.headers("hello world", None));

    let client = TestClient::new(Method::GET, "/test/headers")
        .header("Some-Custom-Header", "hello world")
        .header("Some-Optional-Header", "2");
    check_custom!(client, client.headers("hello world", Some(2)));
}

#[test]
fn custom_json_request() {
    let client = TestClient::new(Method::POST, "/test/jsonRequest")
        .header("Content-Type", "application/json")
        .body(TestBody::Json(r#""hello world""#.to_string()));
    check_custom!(client, client.json_request("hello world"));
}

#[test]
fn custom_json_repsonse() {
    let client = TestClient::new(Method::GET, "/test/jsonResponse")
        .header("Accept", "application/json")
        .response(TestBody::Json(r#""hello world""#.to_string()));
    check_custom!(client, client.json_response(), "hello world");
}

#[test]
fn custom_auth() {
    let client =
        TestClient::new(Method::GET, "/test/authHeader").header("Authorization", "Bearer foobar");
    check_custom!(
        client,
        client.auth_header(&BearerToken::new("foobar").unwrap())
    );

    let client =
        TestClient::new(Method::GET, "/test/cookieHeader").header("Cookie", "foobar=fizzbuzz");
    check_custom!(
        client,
        client.cookie_header(&BearerToken::new("fizzbuzz").unwrap())
    );
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

#[conjure_client]
trait CustomStreamingService<#[response_body] I, #[request_writer] O>
where
    O: Write,
{
    #[endpoint(method = POST, path = "/test/streamingRequest")]
    fn streaming_request<B>(
        &self,
        #[body(serializer = RawRequestSerializer)] body: B,
    ) -> Result<(), Error>
    where
        B: WriteBody<O>;

    #[endpoint(method = GET, path = "/test/streamingResponse", accept = RawResponseDeserializer)]
    fn streaming_response(&self) -> Result<I, Error>;
}

struct RawRequest;

impl<W> WriteBody<W> for RawRequest
where
    W: Write,
{
    fn write_body(&mut self, w: &mut W) -> Result<(), Error> {
        w.write_all(b"hello world").map_err(Error::internal_safe)
    }

    fn reset(&mut self) -> bool {
        true
    }
}

enum RawRequestSerializer {}

impl<'a, T, W> SerializeRequest<'a, T, W> for RawRequestSerializer
where
    T: WriteBody<W> + 'a,
{
    fn content_type(_: &T) -> HeaderValue {
        HeaderValue::from_static("text/plain")
    }

    fn serialize(value: T) -> Result<RequestBody<'a, W>, Error> {
        Ok(RequestBody::Streaming(Box::new(value)))
    }
}

enum RawResponseDeserializer {}

impl<R> DeserializeResponse<R, R> for RawResponseDeserializer {
    fn accept() -> Option<HeaderValue> {
        None
    }

    fn deserialize(response: Response<R>) -> Result<R, Error> {
        Ok(response.into_body())
    }
}

#[test]
fn custom_streaming_request() {
    let client = TestClient::new(Method::POST, "/test/streamingRequest")
        .header("Content-Type", "text/plain")
        .body(TestBody::Streaming(b"hello world".to_vec()));

    CustomStreamingServiceClient::new(&client)
        .streaming_request(RawRequest)
        .unwrap();
}

#[test]
fn custom_streaming_response() {
    let client = TestClient::new(Method::GET, "/test/streamingResponse");

    CustomStreamingServiceClient::new(&client)
        .streaming_response()
        .unwrap();
}

#[conjure_client(name = "name", version = Some("version"))]
trait CustomConfig {
    #[endpoint(method = GET, path = "/path", name = "endpoint")]
    fn foo(&self) -> Result<(), Error>;
}

#[test]
fn custom_endpoint_config() {
    let client = TestClient::new(Method::GET, "/path")
        .endpoint(Endpoint::new("name", Some("version"), "endpoint", "/path"))
        .body(TestBody::Empty);

    CustomConfigClient::new(&client).foo().unwrap();
}

#[conjure_client(name = "name", version = None)]
trait UnversionedCustomConfig {
    #[endpoint(method = GET, path = "/path", name = "endpoint")]
    fn foo(&self) -> Result<(), Error>;
}

#[test]
fn unversioned_custom_endpoint_config() {
    let client = TestClient::new(Method::GET, "/path")
        .endpoint(Endpoint::new("name", None, "endpoint", "/path"))
        .body(TestBody::Empty);

    UnversionedCustomConfigClient::new(&client).foo().unwrap();
}

#[conjure_client(local)]
trait TestLocal {
    #[endpoint(method = GET, path = "/path")]
    async fn foo(&self) -> Result<(), Error>;
}

struct NonSend<T> {
    inner: T,
    _p: PhantomData<*mut ()>,
}

impl<T> NonSend<T> {
    fn new(inner: T) -> Self {
        NonSend {
            inner,
            _p: PhantomData,
        }
    }
}

impl<T> LocalAsyncClient for NonSend<T>
where
    T: LocalAsyncClient,
{
    type BodyWriter = T::BodyWriter;

    type ResponseBody = T::ResponseBody;

    async fn send(
        &self,
        req: Request<LocalAsyncRequestBody<'_, Self::BodyWriter>>,
    ) -> Result<Response<Self::ResponseBody>, Error> {
        self.inner.send(req).await
    }
}

#[test]
fn test_local() {
    let client = TestClient::new(Method::GET, "/path").body(TestBody::Empty);

    executor::block_on(TestLocalClient::new(NonSend::new(&client)).foo()).unwrap();
}
