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
#![allow(clippy::blacklisted_name)]

use crate::test::RemoteBody;
use async_trait::async_trait;
use conjure_error::Error;
use conjure_http::{PathParams, SafeParams};
use conjure_object::{BearerToken, ResourceIdentifier};
use http::{HeaderMap, Request, Uri};
use std::collections::{BTreeMap, BTreeSet};
use std::pin::Pin;

use crate::types::*;
use conjure_http::server::{
    AsyncResponseBody, AsyncService, AsyncWriteBody, ResponseBody, Service, WriteBody,
};
use futures::executor;
use serde::Serialize;

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
        safe_header: String,
        unsafe_header: String
    ) -> Result<(), Error>;

    fn deprecated(&self) -> Result<(), Error>;
}

impl TestServiceHandler {
    fn call(self) -> Call {
        Call {
            service: TestServiceService::new(self),
            uri: Uri::default(),
            path_params: PathParams::new(),
            headers: HeaderMap::new(),
            body: vec![],
            safe_params: SafeParams::new(),
            response: TestBody::Empty,
        }
    }
}

struct Call {
    service: TestServiceService<TestServiceHandler>,
    uri: Uri,
    path_params: PathParams,
    headers: HeaderMap,
    body: Vec<u8>,
    safe_params: SafeParams,
    response: TestBody,
}

impl Call {
    fn uri(&mut self, uri: &str) -> &mut Call {
        self.uri = uri.parse().unwrap();
        self
    }

    fn path_param(&mut self, key: &str, value: &str) -> &mut Call {
        self.path_params.insert(key, value);
        self
    }

    fn header(&mut self, key: &'static str, value: &str) -> &mut Call {
        self.headers.insert(key, value.parse().unwrap());
        self
    }

    fn body(&mut self, body: &[u8]) -> &mut Call {
        self.body = body.to_vec();
        self
    }

    fn response(&mut self, response: TestBody) -> &mut Call {
        self.response = response;
        self
    }

    fn safe_param<T>(&mut self, name: &'static str, value: T) -> &mut Call
    where
        T: Serialize,
    {
        self.safe_params.insert(name, &value);
        self
    }

    fn send(&self, name: &str) {
        self.send_sync(name);
        executor::block_on(self.send_async(name));
    }

    fn send_sync(&self, name: &str) {
        let endpoint = Service::endpoints(&self.service)
            .into_iter()
            .find(|e| e.name() == name)
            .unwrap();

        let mut request = Request::new(RemoteBody(self.body.clone()));
        *request.uri_mut() = self.uri.clone();
        *request.headers_mut() = self.headers.clone();
        request.extensions_mut().insert(self.path_params.clone());

        let response = endpoint.handle(request);
        assert_eq!(
            &self.safe_params,
            response.extensions().get::<SafeParams>().unwrap()
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

    async fn send_async(&self, name: &str) {
        let endpoint = AsyncService::endpoints(&self.service)
            .into_iter()
            .find(|e| e.name() == name)
            .unwrap();

        let mut request = Request::new(RemoteBody(self.body.clone()));
        *request.uri_mut() = self.uri.clone();
        *request.headers_mut() = self.headers.clone();
        request.extensions_mut().insert(self.path_params.clone());

        let response = endpoint.handle(request).await;
        assert_eq!(
            &self.safe_params,
            response.extensions().get::<SafeParams>().unwrap()
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
        .path_param("foo", "hello world")
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
