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
#![allow(clippy::disallowed_names)]

use crate::test::RemoteBody;
use crate::types::*;
use async_trait::async_trait;
use conjure_error::Error;
use conjure_http::server::{
    AsyncResponseBody, AsyncService, AsyncWriteBody, ConjureResponseSerializer, DeserializeRequest,
    FromStrOptionDecoder, FromStrSeqDecoder, RequestContext, ResponseBody, SerializeResponse,
    Service, WriteBody,
};
use conjure_http::{PathParams, SafeParams};
use conjure_macros::{conjure_endpoints, endpoint};
use conjure_object::{BearerToken, ResourceIdentifier};
use futures::executor;
use http::{Extensions, HeaderMap, Request, Response, Uri};
use mockall::mock;
use mockall::predicate::eq;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;
use std::pin::Pin;

macro_rules! test_service_handler {
    ($(
        fn $fn_name:ident(&self $(, $arg_name:ident : $arg_type:ty)*) -> Result<$ret_type:ty, Error>;
    )*) => {
        struct TestServiceHandler {
            $(
                $fn_name: Option<Box<dyn Fn($($arg_type),*) -> Result<$ret_type, Error> + Sync + Send>>,
            )*
        }

        impl TestServiceHandler {
            fn new() -> TestServiceHandler {
                TestServiceHandler {
                    $($fn_name: None,)*
                }
            }

            $(
                #[allow(dead_code)]
                fn $fn_name<F>(mut self, f: F) -> TestServiceHandler
                where
                    F: Fn($($arg_type),*) -> Result<$ret_type, Error> + 'static + Sync + Send,
                {
                    self.$fn_name = Some(Box::new(f));
                    self
                }
            )*
        }

        impl TestService<RemoteBody, Vec<u8>> for TestServiceHandler {
            type StreamingResponseBody = StreamingBody;
            type OptionalStreamingResponseBody = StreamingBody;
            type StreamingAliasResponseBody = StreamingBody;
            type OptionalStreamingAliasResponseBody = StreamingBody;

            $(
                fn $fn_name(&self $(, $arg_name: $arg_type)*) -> Result<$ret_type, Error> {
                    self.$fn_name.as_ref().unwrap()($($arg_name),*)
                }
            )*
        }

        #[async_trait]
        impl AsyncTestService<RemoteBody, Vec<u8>> for TestServiceHandler {
            type StreamingResponseBody = StreamingBody;
            type OptionalStreamingResponseBody = StreamingBody;
            type StreamingAliasResponseBody = StreamingBody;
            type OptionalStreamingAliasResponseBody = StreamingBody;

            $(
                async fn $fn_name(
                    &self
                    $(, $arg_name: $arg_type)*
                ) -> Result<$ret_type, Error> {
                    self.$fn_name.as_ref().unwrap()($($arg_name),*)
                }
            )*
        }
    }
}

test_service_handler! {
    fn query_params(
        &self,
        normal: String,
        optional: Option<i32>,
        list: Vec<i32>,
        set: BTreeSet<bool>
    ) -> Result<(), Error>;

    fn alias_query_params(
        &self,
        optional: OptionalAliasAlias,
        list: ListAliasAlias,
        set: SetAliasAlias
    ) -> Result<(), Error>;

    fn path_params(&self, foo: String, bar: bool, baz: ResourceIdentifier) -> Result<(), Error>;

    fn headers(&self, foo: String, bar: Option<i32>) -> Result<(), Error>;

    fn alias_headers(&self, bar: OptionalAliasAlias) -> Result<(), Error>;

    fn empty_request(&self) -> Result<(), Error>;

    fn json_request(&self, body: String) -> Result<(), Error>;

    fn optional_json_request(&self, body: Option<String>) -> Result<(), Error>;

    fn optional_alias_request(&self, body: OptionalAlias) -> Result<(), Error>;

    fn streaming_request(&self, body: RemoteBody) -> Result<(), Error>;

    fn streaming_alias_request(&self, body: RemoteBody) -> Result<(), Error>;

    fn json_response(&self) -> Result<String, Error>;

    fn optional_json_response(&self) -> Result<Option<String>, Error>;

    fn list_json_response(&self) -> Result<Vec<String>, Error>;

    fn set_json_response(&self) -> Result<BTreeSet<String>, Error>;

    fn map_json_response(&self) -> Result<BTreeMap<String, String>, Error>;

    fn streaming_response(&self) -> Result<StreamingBody, Error>;

    fn optional_streaming_response(&self) -> Result<Option<StreamingBody>, Error>;

    fn streaming_alias_response(&self) -> Result<StreamingBody, Error>;

    fn optional_streaming_alias_response(&self) -> Result<Option<StreamingBody>, Error>;

    fn header_auth(&self, auth: BearerToken) -> Result<(), Error>;

    fn cookie_auth(&self, auth: BearerToken) -> Result<(), Error>;

    fn safe_params(
        &self,
        safe_path: String,
        unsafe_path: String,
        safe_query: String,
        unsafe_query: String,
        safe_header: SafeStringAlias,
        unsafe_header: UnsafeStringAlias
    ) -> Result<(), Error>;

    fn deprecated(&self) -> Result<(), Error>;

    fn context(&self, arg: Option<String>, request_context: RequestContext<'_>) -> Result<(), Error>;

    fn context_no_args(&self, request_context: RequestContext<'_>) -> Result<(), Error>;
}

impl TestServiceHandler {
    fn call(self) -> Call<TestServiceEndpoints<TestServiceHandler>> {
        Call::new(TestServiceEndpoints::new(self))
    }
}

struct Call<T> {
    service: T,
    uri: Uri,
    path_params: PathParams,
    headers: HeaderMap,
    body: Vec<u8>,
    safe_params: SafeParams,
    response: TestBody,
}

impl<T> Call<T> {
    fn new(service: T) -> Self {
        Call {
            service,
            uri: Uri::default(),
            path_params: PathParams::new(),
            headers: HeaderMap::new(),
            body: vec![],
            safe_params: SafeParams::new(),
            response: TestBody::Empty,
        }
    }

    fn uri(&mut self, uri: &str) -> &mut Self {
        self.uri = uri.parse().unwrap();
        self
    }

    fn path_param(&mut self, key: &str, value: &str) -> &mut Self {
        self.path_params.insert(key, value);
        self
    }

    fn header(&mut self, key: &'static str, value: &str) -> &mut Self {
        self.headers.insert(key, value.parse().unwrap());
        self
    }

    fn body(&mut self, body: &[u8]) -> &mut Self {
        self.body = body.to_vec();
        self
    }

    fn response(&mut self, response: TestBody) -> &mut Self {
        self.response = response;
        self
    }

    fn safe_param<V>(&mut self, name: &'static str, value: V) -> &mut Self
    where
        V: Serialize,
    {
        self.safe_params.insert(name, &value);
        self
    }
}

impl<T> Call<T>
where
    T: Service<RemoteBody, Vec<u8>> + AsyncService<RemoteBody, Vec<u8>>,
{
    fn send(&self, name: &str) {
        self.send_sync(name);
        executor::block_on(self.send_async(name));
    }
}

impl<T> Call<T>
where
    T: Service<RemoteBody, Vec<u8>>,
{
    fn send_sync(&self, name: &str) {
        let endpoint = Service::endpoints(&self.service)
            .into_iter()
            .find(|e| e.name() == name)
            .unwrap();

        let mut request = Request::new(RemoteBody(self.body.clone()));
        *request.uri_mut() = self.uri.clone();
        *request.headers_mut() = self.headers.clone();
        request.extensions_mut().insert(self.path_params.clone());

        let mut extensions = Extensions::new();
        let response = endpoint.handle(request, &mut extensions).unwrap();
        assert_eq!(
            self.safe_params,
            extensions.remove::<SafeParams>().unwrap_or_default()
        );
        let body = match response.into_body() {
            ResponseBody::Empty => TestBody::Empty,
            ResponseBody::Fixed(bytes) => {
                TestBody::Json(String::from_utf8(bytes.to_vec()).unwrap())
            }
            ResponseBody::Streaming(body) => {
                let mut buf = vec![];
                body.write_body(&mut buf).unwrap();
                TestBody::Streaming(buf)
            }
        };
        assert_eq!(self.response, body);
    }
}

impl<T> Call<T>
where
    T: AsyncService<RemoteBody, Vec<u8>>,
{
    async fn send_async(&self, name: &str) {
        let endpoint = AsyncService::endpoints(&self.service)
            .into_iter()
            .find(|e| e.name() == name)
            .unwrap();

        let mut request = Request::new(RemoteBody(self.body.clone()));
        *request.uri_mut() = self.uri.clone();
        *request.headers_mut() = self.headers.clone();
        request.extensions_mut().insert(self.path_params.clone());

        let mut extensions = Extensions::new();
        let response = endpoint.handle(request, &mut extensions).await.unwrap();
        assert_eq!(
            self.safe_params,
            extensions.remove::<SafeParams>().unwrap_or_default()
        );
        let body = match response.into_body() {
            AsyncResponseBody::Empty => TestBody::Empty,
            AsyncResponseBody::Fixed(bytes) => {
                TestBody::Json(String::from_utf8(bytes.to_vec()).unwrap())
            }
            AsyncResponseBody::Streaming(body) => {
                let mut buf = vec![];
                body.write_body(Pin::new(&mut buf)).await.unwrap();
                TestBody::Streaming(buf)
            }
        };
        assert_eq!(self.response, body);
    }
}

#[derive(PartialEq, Debug, Clone)]
struct StreamingBody(Vec<u8>);

impl WriteBody<Vec<u8>> for StreamingBody {
    fn write_body(self: Box<Self>, w: &mut Vec<u8>) -> Result<(), Error> {
        w.extend_from_slice(&self.0);
        Ok(())
    }
}

#[async_trait]
impl AsyncWriteBody<Vec<u8>> for StreamingBody {
    async fn write_body(self: Box<Self>, mut w: Pin<&mut Vec<u8>>) -> Result<(), Error> {
        w.extend_from_slice(&self.0);
        Ok(())
    }
}

#[derive(PartialEq, Debug, Clone)]
enum TestBody {
    Empty,
    Json(String),
    Streaming(Vec<u8>),
}

#[test]
fn query_params() {
    TestServiceHandler::new()
        .query_params(|normal, optional, list, set| {
            assert_eq!(normal, "hello world");
            assert_eq!(optional, Some(2));
            assert_eq!(list, vec![1, 2]);
            let mut expected = BTreeSet::new();
            expected.insert(false);
            assert_eq!(set, expected);
            Ok(())
        })
        .call()
        .uri("/test/queryParams?normal=hello%20world&custom=2&list=1&list=2&set=false")
        .send("queryParams");

    TestServiceHandler::new()
        .query_params(|normal, optional, list, set| {
            assert_eq!(normal, "hello world");
            assert_eq!(optional, None);
            assert_eq!(list, Vec::<i32>::new());
            assert_eq!(set, BTreeSet::new());
            Ok(())
        })
        .call()
        .uri("/test/queryParams?normal=hello%20world")
        .send("queryParams");
}

#[test]
fn alias_query_params() {
    TestServiceHandler::new()
        .alias_query_params(|optional, list, set| {
            assert_eq!(optional, OptionalAliasAlias(OptionalAlias(Some(2))));
            assert_eq!(list, ListAliasAlias(ListAlias(vec![1, 2])));
            let mut expected = BTreeSet::new();
            expected.insert(3);
            assert_eq!(set, SetAliasAlias(SetAlias(expected)));
            Ok(())
        })
        .call()
        .uri("/test/aliasQueryParams?optional=2&list=1&list=2&set=3")
        .send("aliasQueryParams");

    TestServiceHandler::new()
        .alias_query_params(|optional, list, set| {
            assert_eq!(optional, OptionalAliasAlias(OptionalAlias(None)));
            assert_eq!(list, ListAliasAlias(ListAlias(vec![])));
            assert_eq!(set, SetAliasAlias(SetAlias(BTreeSet::new())));
            Ok(())
        })
        .call()
        .send("aliasQueryParams");
}

#[test]
fn path_params() {
    TestServiceHandler::new()
        .path_params(|foo, bar, baz| {
            assert_eq!(foo, "hello world");
            assert!(bar);
            assert_eq!(
                baz,
                ResourceIdentifier::new("ri.conjure.main.test.foo").unwrap()
            );
            Ok(())
        })
        .call()
        .path_param("foo", "hello%20world")
        .path_param("bar", "true")
        .path_param("baz", "ri.conjure.main.test.foo")
        .send("pathParams");
}

#[test]
fn headers() {
    TestServiceHandler::new()
        .headers(|foo, bar| {
            assert_eq!(foo, "hello world");
            assert_eq!(bar, Some(2));
            Ok(())
        })
        .call()
        .header("Some-Custom-Header", "hello world")
        .header("Some-Optional-Header", "2")
        .send("headers");

    TestServiceHandler::new()
        .headers(|foo, bar| {
            assert_eq!(foo, "hello world");
            assert_eq!(bar, None);
            Ok(())
        })
        .call()
        .header("Some-Custom-Header", "hello world")
        .send("headers");
}

#[test]
fn alias_headers() {
    TestServiceHandler::new()
        .alias_headers(|bar| {
            assert_eq!(bar, OptionalAliasAlias(OptionalAlias(Some(2))));
            Ok(())
        })
        .call()
        .header("Some-Optional-Header", "2")
        .send("aliasHeaders");

    TestServiceHandler::new()
        .alias_headers(|bar| {
            assert_eq!(bar, OptionalAliasAlias(OptionalAlias(None)));
            Ok(())
        })
        .call()
        .send("aliasHeaders");
}

#[test]
fn empty_request() {
    TestServiceHandler::new()
        .empty_request(|| Ok(()))
        .call()
        .send("emptyRequest");
}

#[test]
fn json_request() {
    TestServiceHandler::new()
        .json_request(|body| {
            assert_eq!(body, "hello world");
            Ok(())
        })
        .call()
        .body(br#""hello world""#)
        .header("Content-Type", "application/json")
        .send("jsonRequest");
}

#[test]
fn optional_json_request() {
    TestServiceHandler::new()
        .optional_json_request(|body| {
            assert_eq!(body, Some("hello world".to_string()));
            Ok(())
        })
        .call()
        .header("Content-Type", "application/json")
        .body(br#""hello world""#)
        .send("optionalJsonRequest");

    TestServiceHandler::new()
        .optional_json_request(|body| {
            assert_eq!(body, None);
            Ok(())
        })
        .call()
        .header("Content-Type", "application/json")
        .body(b"null")
        .send("optionalJsonRequest");

    TestServiceHandler::new()
        .optional_json_request(|body| {
            assert_eq!(body, None);
            Ok(())
        })
        .call()
        .send("optionalJsonRequest");
}

#[test]
fn optional_alias_request() {
    TestServiceHandler::new()
        .optional_alias_request(|body| {
            assert_eq!(body, OptionalAlias(Some(5)));
            Ok(())
        })
        .call()
        .header("Content-Type", "application/json")
        .body(b"5")
        .send("optionalAliasRequest");

    TestServiceHandler::new()
        .optional_alias_request(|body| {
            assert_eq!(body, OptionalAlias(None));
            Ok(())
        })
        .call()
        .header("Content-Type", "application/json")
        .body(b"null")
        .send("optionalAliasRequest");

    TestServiceHandler::new()
        .optional_alias_request(|body| {
            assert_eq!(body, OptionalAlias(None));
            Ok(())
        })
        .call()
        .send("optionalAliasRequest");
}

#[test]
fn streaming_request() {
    TestServiceHandler::new()
        .streaming_request(|body| {
            assert_eq!(body.0, [1, 2, 3, 4]);
            Ok(())
        })
        .call()
        .header("Content-Type", "application/octet-stream")
        .body(&[1, 2, 3, 4])
        .send("streamingRequest");
}

#[test]
fn streaming_alias_request() {
    TestServiceHandler::new()
        .streaming_alias_request(|body| {
            assert_eq!(body.0, [1, 2, 3, 4]);
            Ok(())
        })
        .call()
        .header("Content-Type", "application/octet-stream")
        .body(&[1, 2, 3, 4])
        .send("streamingAliasRequest");
}

#[test]
fn json_response() {
    TestServiceHandler::new()
        .json_response(|| Ok("hello world".to_string()))
        .call()
        .response(TestBody::Json(r#""hello world""#.to_string()))
        .send("jsonResponse");
}

#[test]
fn optional_json_response() {
    TestServiceHandler::new()
        .optional_json_response(|| Ok(Some("hello world".to_string())))
        .call()
        .response(TestBody::Json(r#""hello world""#.to_string()))
        .send("optionalJsonResponse");

    TestServiceHandler::new()
        .optional_json_response(|| Ok(None))
        .call()
        .send("optionalJsonResponse");
}

#[test]
fn list_json_response() {
    TestServiceHandler::new()
        .list_json_response(|| Ok(vec!["hello".to_string(), "world".to_string()]))
        .call()
        .response(TestBody::Json(r#"["hello","world"]"#.to_string()))
        .send("listJsonResponse");

    TestServiceHandler::new()
        .list_json_response(|| Ok(vec![]))
        .call()
        .send("listJsonResponse");
}

#[test]
fn set_json_response() {
    TestServiceHandler::new()
        .set_json_response(|| {
            let mut set = BTreeSet::new();
            set.insert("hello".to_string());
            set.insert("world".to_string());
            Ok(set)
        })
        .call()
        .response(TestBody::Json(r#"["hello","world"]"#.to_string()))
        .send("setJsonResponse");

    TestServiceHandler::new()
        .set_json_response(|| Ok(BTreeSet::new()))
        .call()
        .send("setJsonResponse");
}

#[test]
fn map_json_response() {
    TestServiceHandler::new()
        .map_json_response(|| {
            let mut map = BTreeMap::new();
            map.insert("hello".to_string(), "world".to_string());
            Ok(map)
        })
        .call()
        .response(TestBody::Json(r#"{"hello":"world"}"#.to_string()))
        .send("mapJsonResponse");

    TestServiceHandler::new()
        .map_json_response(|| Ok(BTreeMap::new()))
        .call()
        .send("mapJsonResponse");
}

#[test]
fn streaming_response() {
    TestServiceHandler::new()
        .streaming_response(|| Ok(StreamingBody(vec![1, 2, 3, 4])))
        .call()
        .response(TestBody::Streaming(vec![1, 2, 3, 4]))
        .send("streamingResponse");
}

#[test]
fn optional_streaming_response() {
    TestServiceHandler::new()
        .optional_streaming_response(|| Ok(Some(StreamingBody(vec![1, 2, 3, 4]))))
        .call()
        .response(TestBody::Streaming(vec![1, 2, 3, 4]))
        .send("optionalStreamingResponse");

    TestServiceHandler::new()
        .optional_streaming_response(|| Ok(None))
        .call()
        .send("optionalStreamingResponse");
}

#[test]
fn streaming_alias_response() {
    TestServiceHandler::new()
        .streaming_alias_response(|| Ok(StreamingBody(vec![1, 2, 3, 4])))
        .call()
        .response(TestBody::Streaming(vec![1, 2, 3, 4]))
        .send("streamingAliasResponse");
}

#[test]
fn optional_streaming_alias_response() {
    TestServiceHandler::new()
        .optional_streaming_alias_response(|| Ok(Some(StreamingBody(vec![1, 2, 3, 4]))))
        .call()
        .response(TestBody::Streaming(vec![1, 2, 3, 4]))
        .send("optionalStreamingAliasResponse");

    TestServiceHandler::new()
        .optional_streaming_alias_response(|| Ok(None))
        .call()
        .send("optionalStreamingAliasResponse");
}

#[test]
fn header_auth() {
    TestServiceHandler::new()
        .header_auth(|auth| {
            assert_eq!(auth, BearerToken::new("foobar").unwrap());
            Ok(())
        })
        .call()
        .header("Authorization", "Bearer foobar")
        .send("headerAuth");
}

#[test]
fn cookie_auth() {
    TestServiceHandler::new()
        .cookie_auth(|auth| {
            assert_eq!(auth, BearerToken::new("fizzbuzz").unwrap());
            Ok(())
        })
        .call()
        .header("Cookie", "foobar=fizzbuzz")
        .send("cookieAuth");
}

#[test]
fn safe_params() {
    TestServiceHandler::new()
        .safe_params(|_, _, _, _, _, _| Ok(()))
        .call()
        .uri("/test/safeParams/foo/bar?safeQueryId=a&unsafeQueryId=b")
        .path_param("safePath", "foo")
        .path_param("unsafePath", "bar")
        .header("Safe-Header", "biz")
        .header("Unsafe-Header", "buz")
        .safe_param("safePath", "foo")
        .safe_param("safeQuery", "a")
        .safe_param("safeHeader", "biz")
        .send("safeParams");
}

#[test]
fn context() {
    TestServiceHandler::new()
        .context(|_, context| {
            assert_eq!(context.request_headers().get("TestHeader").unwrap(), "foo");
            Ok(())
        })
        .call()
        .uri("/test/context")
        .header("TestHeader", "foo")
        .send("context");
}

macro_rules! custom_service {
    ($(
        $(#[$meta:meta])*
        fn $fn_name:ident(&self $(, $(#[$arg_meta:meta])* $arg_name:ident : $arg_type:ty)*) -> Result<$ret_type:ty, Error>;
    )*) => {
        #[conjure_endpoints]
        trait CustomService {
            $(
                $(#[$meta])*
                fn $fn_name(&self $(, $(#[$arg_meta])* $arg_name : $arg_type)*) -> Result<$ret_type, Error>;
            )*
        }

        struct CustomServiceHandler {
            $(
                $fn_name: Option<Box<dyn Fn($($arg_type),*) -> Result<$ret_type, Error> + Sync + Send>>,
            )*
        }

        impl CustomServiceHandler {
            fn new() -> Self {
                Self {
                    $($fn_name: None,)*
                }
            }

            $(
                fn $fn_name<F>(mut self, f: F) -> Self
                where
                    F: Fn($($arg_type),*) -> Result<$ret_type, Error> + 'static + Sync + Send,
                {
                    self.$fn_name = Some(Box::new(f));
                    self
                }
            )*
        }

        impl CustomService for CustomServiceHandler {
            $(
                fn $fn_name(&self $(, $arg_name: $arg_type)*) -> Result<$ret_type, Error> {
                    self.$fn_name.as_ref().unwrap()($($arg_name),*)
                }
            )*
        }
    };
}

impl CustomServiceHandler {
    fn call(self) -> Call<CustomServiceEndpoints<Self>> {
        Call::new(CustomServiceEndpoints::new(self))
    }
}

custom_service! {
    #[endpoint(method = GET, path = "/test/queryParams")]
    fn query_params(
        &self,
        #[query(name = "normal")] normal: String,
        #[query(name = "list", decoder = FromStrSeqDecoder<_>)] list: Vec<i32>
    ) -> Result<(), Error>;

    #[endpoint(method = GET, path = "/test/pathParams/{foo}/raw")]
    fn path_params(&self, #[path] foo: String) -> Result<(), Error>;

    #[endpoint(method = GET, path = "/test/headers")]
    fn headers(
        &self,
        #[header(name = "Some-Custom-Header")] custom_header: String,
        #[header(name = "Some-Optional-Header", decoder = FromStrOptionDecoder)]
        optional_header: Option<i32>
    ) -> Result<(), Error>;

    #[endpoint(method = POST, path = "/test/jsonRequest")]
    fn json_request(&self, #[body] body: String) -> Result<(), Error>;

    #[endpoint(method = GET, path = "/test/jsonResponse", produces = ConjureResponseSerializer)]
    fn json_response(&self) -> Result<String, Error>;

    #[endpoint(method = GET, path = "/test/authHeader")]
    fn auth_header(&self, #[auth] auth: BearerToken) -> Result<(), Error>;

    #[endpoint(method = GET, path = "/test/cookieHeader")]
    fn cookie_header(&self, #[auth(cookie_name = "foobar")] auth: BearerToken) -> Result<(), Error>;

    #[endpoint(method = POST, path = "/test/safeParams/{safe_path}/{unsafe_path}")]
    #[allow(clippy::too_many_arguments)]
    fn safe_params(
        &self,
        #[path(safe)] safe_path: String,
        #[path] unsafe_path: String,
        #[query(safe, name = "safeQuery")] safe_query: String,
        #[query(name = "unsafeQuery")]  unsafe_query: String,
        #[header(safe, name = "Safe-Header")] safe_header: String,
        #[header(name = "Unsafe-Header")] unsafe_header: String,
        #[body(safe)] body: String
    ) -> Result<(), Error>;
}

#[test]
fn custom_query_params() {
    CustomServiceHandler::new()
        .query_params(|normal, list| {
            assert_eq!(normal, "hello world");
            assert_eq!(list, vec![1, 2]);
            Ok(())
        })
        .call()
        .uri("/test/queryParams?normal=hello%20world&list=1&list=2")
        .send_sync("query_params");

    CustomServiceHandler::new()
        .query_params(|normal, list| {
            assert_eq!(normal, "foo");
            assert_eq!(list, Vec::<i32>::new());
            Ok(())
        })
        .call()
        .uri("/test/queryParams?normal=foo")
        .send_sync("query_params");
}

#[test]
fn custom_path_params() {
    CustomServiceHandler::new()
        .path_params(|foo| {
            assert_eq!(foo, "hello world");
            Ok(())
        })
        .call()
        .path_param("foo", "hello%20world")
        .send_sync("path_params");
}

#[test]
fn custom_headers() {
    CustomServiceHandler::new()
        .headers(|custom_header, optional_header| {
            assert_eq!(custom_header, "hello world");
            assert_eq!(optional_header, Some(2));
            Ok(())
        })
        .call()
        .header("Some-Custom-Header", "hello world")
        .header("Some-Optional-Header", "2")
        .send_sync("headers");

    CustomServiceHandler::new()
        .headers(|custom_header, optional_header| {
            assert_eq!(custom_header, "hello world");
            assert_eq!(optional_header, None);
            Ok(())
        })
        .call()
        .header("Some-Custom-Header", "hello world")
        .send_sync("headers");
}

#[test]
fn custom_json_request() {
    CustomServiceHandler::new()
        .json_request(|body| {
            assert_eq!(body, "hello world");
            Ok(())
        })
        .call()
        .header("Content-Type", "application/json")
        .body(br#""hello world""#)
        .send_sync("json_request");
}

#[test]
fn custom_json_response() {
    CustomServiceHandler::new()
        .json_response(|| Ok("hello world".to_string()))
        .call()
        .header("Accept", "application/json")
        .response(TestBody::Json(r#""hello world""#.to_string()))
        .send_sync("json_response");
}

#[test]
fn custom_auth_header() {
    CustomServiceHandler::new()
        .auth_header(|auth| {
            assert_eq!(auth, BearerToken::new("foobar").unwrap());
            Ok(())
        })
        .call()
        .header("Authorization", "Bearer foobar")
        .send_sync("auth_header");
}

#[test]
fn custom_cookie_header() {
    CustomServiceHandler::new()
        .cookie_header(|auth| {
            assert_eq!(auth, BearerToken::new("fizzbuzz").unwrap());
            Ok(())
        })
        .call()
        .header("Cookie", "foobar=fizzbuzz")
        .send_sync("cookie_header");
}

#[test]
fn custom_safe_params() {
    CustomServiceHandler::new()
        .safe_params(|_, _, _, _, _, _, _| Ok(()))
        .call()
        .uri("/test/safeParams?safeQuery=safe%20query%20value&unsafeQuery=unsafe%20query%20value")
        .path_param("safe_path", "safe path value")
        .path_param("unsafe_path", "unsafe path value")
        .header("Safe-Header", "safe header value")
        .header("Unsafe-Header", "unsafe header value")
        .header("Content-Type", "application/json")
        .body(br#""safe body value""#)
        .safe_param("safe_path", "safe path value")
        .safe_param("safe_query", "safe query value")
        .safe_param("safe_header", "safe header value")
        .safe_param("body", "safe body value")
        .send_sync("safe_params");
}

#[conjure_endpoints]
trait CustomStreamingService<#[request_body] I, #[response_writer] O>
where
    O: Write,
{
    #[endpoint(method = POST, path = "/test/stremaingRequest")]
    fn streaming_request(
        &self,
        #[body(deserializer = RawRequestDeserializer)] body: I,
    ) -> Result<(), Error>;

    #[endpoint(method = GET, path = "/test/streamingReponse", produces = RawResponseSerializer)]
    fn streaming_response(&self) -> Result<TestBodyWriter, Error>;
}

// We can't annotate the trait with #[mockall] due to annoying interactions with #[conjure_endpoints]
mock! {
    CustomStreamingService<I, O> {}

    impl<I, O> CustomStreamingService<I, O> for CustomStreamingService<I, O>
    where
        O: Write
    {
        fn streaming_request(
            &self,
            body: I,
        ) -> Result<(), Error>;

        fn streaming_response(&self) -> Result<TestBodyWriter, Error>;
    }
}

enum RawRequestDeserializer {}

impl<I> DeserializeRequest<I, I> for RawRequestDeserializer {
    fn deserialize(_: &HeaderMap, body: I) -> Result<I, Error> {
        Ok(body)
    }
}

enum RawResponseSerializer {}

impl<T, O> SerializeResponse<T, O> for RawResponseSerializer
where
    T: WriteBody<O> + 'static + Send,
{
    fn serialize(_: &HeaderMap, value: T) -> Result<Response<ResponseBody<O>>, Error> {
        Ok(Response::new(ResponseBody::Streaming(Box::new(value))))
    }
}

struct TestBodyWriter;

impl<O> WriteBody<O> for TestBodyWriter
where
    O: Write,
{
    fn write_body(self: Box<Self>, w: &mut O) -> Result<(), Error> {
        w.write_all(b"hello world").map_err(Error::internal_safe)
    }
}

#[test]
fn custom_streaming_request() {
    let mut mock = MockCustomStreamingService::new();
    mock.expect_streaming_request()
        .with(eq(RemoteBody(b"hello world".to_vec())))
        .returning(|_| Ok(()));

    Call::new(CustomStreamingServiceEndpoints::new(mock))
        .body(b"hello world")
        .send_sync("streaming_request");
}

#[test]
fn custom_streaming_response() {
    let mut mock = MockCustomStreamingService::new();
    mock.expect_streaming_response()
        .returning(|| Ok(TestBodyWriter));

    Call::new(CustomStreamingServiceEndpoints::new(mock))
        .response(TestBody::Streaming(b"hello world".to_vec()))
        .send_sync("streaming_response");
}
