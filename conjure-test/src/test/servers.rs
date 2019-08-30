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

use conjure_error::Error;
use conjure_http::server::{
    HeaderParameter, Parameter, ParameterType, PathParameter, QueryParameter, RequestBody,
    Resource, VisitRequestBody, VisitResponse, WriteBody,
};
use conjure_http::{PathParams, QueryParams};
use conjure_object::{BearerToken, ResourceIdentifier};
use conjure_serde::json::{self, ServerDeserializer};
use http::{HeaderMap, Method};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

use crate::types::*;

macro_rules! test_service_handler {
    ($(
        fn $fn_name:ident(&self $(, $arg_name:ident : $arg_type:ty)*) -> Result<$ret_type:ty, Error>;
    )*) => {
        struct TestServiceHandler {
            $(
                $fn_name: Option<Box<dyn Fn($($arg_type),*) -> Result<$ret_type, Error>>>,
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
                    F: Fn($($arg_type),*) -> Result<$ret_type, Error> + 'static,
                {
                    self.$fn_name = Some(Box::new(f));
                    self
                }
            )*
        }

        impl TestService<Vec<u8>, Vec<u8>> for TestServiceHandler {
            type StreamingResponseBody = Vec<u8>;
            type OptionalStreamingResponseBody = Vec<u8>;
            type StreamingAliasResponseBody = Vec<u8>;
            type OptionalStreamingAliasResponseBody = Vec<u8>;

            $(
                fn $fn_name(&self $(, $arg_name: $arg_type)*) -> Result<$ret_type, Error> {
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

    fn streaming_request(&self, body: Vec<u8>) -> Result<(), Error>;

    fn streaming_alias_request(&self, body: Vec<u8>) -> Result<(), Error>;

    fn json_response(&self) -> Result<String, Error>;

    fn optional_json_response(&self) -> Result<Option<String>, Error>;

    fn list_json_response(&self) -> Result<Vec<String>, Error>;

    fn set_json_response(&self) -> Result<BTreeSet<String>, Error>;

    fn map_json_response(&self) -> Result<BTreeMap<String, String>, Error>;

    fn streaming_response(&self) -> Result<Vec<u8>, Error>;

    fn optional_streaming_response(&self) -> Result<Option<Vec<u8>>, Error>;

    fn streaming_alias_response(&self) -> Result<Vec<u8>, Error>;

    fn optional_streaming_alias_response(&self) -> Result<Option<Vec<u8>>, Error>;

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
            resource: TestServiceResource::new(self),
            path_params: PathParams::new(),
            query_params: QueryParams::new(),
            headers: HeaderMap::new(),
            body: TestBody::Empty,
            response: TestBody::Empty,
        }
    }
}

struct Call {
    resource: TestServiceResource<TestServiceHandler>,
    path_params: PathParams,
    query_params: QueryParams,
    headers: HeaderMap,
    body: TestBody,
    response: TestBody,
}

impl Call {
    fn path_param(&mut self, key: &str, value: &str) -> &mut Call {
        self.path_params.insert(key, value);
        self
    }

    fn query_param(&mut self, key: &str, value: &str) -> &mut Call {
        self.query_params.insert(key, value);
        self
    }

    fn header(&mut self, key: &'static str, value: &str) -> &mut Call {
        self.headers.insert(key, value.parse().unwrap());
        self
    }

    fn body(&mut self, body: TestBody) -> &mut Call {
        self.body = body;
        self
    }

    fn response(&mut self, response: TestBody) -> &mut Call {
        self.response = response;
        self
    }

    fn send(&self, name: &str) {
        let endpoint = TestServiceResource::<TestServiceHandler>::endpoints()
            .into_iter()
            .find(|e| e.name() == name)
            .unwrap();

        let response = endpoint.handler()(
            &self.resource,
            &self.path_params,
            &self.query_params,
            &self.headers,
            self.body.clone(),
            TestResponseVisitor,
        )
        .unwrap();
        assert_eq!(response, self.response);
    }
}

#[derive(PartialEq, Debug, Clone)]
enum TestBody {
    Empty,
    Json(String),
    Streaming(Vec<u8>),
}

impl RequestBody for TestBody {
    type BinaryBody = Vec<u8>;

    fn accept<V>(self, visitor: V) -> Result<V::Output, Error>
    where
        V: VisitRequestBody<Vec<u8>>,
    {
        match self {
            TestBody::Empty => visitor.visit_empty(),
            TestBody::Json(s) => {
                let mut deserializer = ServerDeserializer::from_str(&s);
                let r = visitor.visit_serializable(&mut deserializer);
                deserializer.end().unwrap();
                r
            }
            TestBody::Streaming(s) => visitor.visit_binary(s),
        }
    }
}

struct TestResponseVisitor;

impl VisitResponse for TestResponseVisitor {
    type BinaryWriter = Vec<u8>;

    type Output = TestBody;

    fn visit_empty(self) -> Result<TestBody, Error> {
        Ok(TestBody::Empty)
    }

    fn visit_serializable<T>(self, body: T) -> Result<TestBody, Error>
    where
        T: Serialize + 'static,
    {
        let body = json::to_string(&body).unwrap();
        Ok(TestBody::Json(body))
    }

    fn visit_binary<T>(self, body: T) -> Result<TestBody, Error>
    where
        T: WriteBody<Vec<u8>> + 'static,
    {
        let mut buf = vec![];
        body.write_body(&mut buf).unwrap();
        Ok(TestBody::Streaming(buf))
    }
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
        .query_param("normal", "hello world")
        .query_param("custom", "2")
        .query_param("list", "1")
        .query_param("list", "2")
        .query_param("set", "false")
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
        .query_param("normal", "hello world")
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
        .query_param("optional", "2")
        .query_param("list", "1")
        .query_param("list", "2")
        .query_param("set", "3")
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
            assert_eq!(bar, true);
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
        .body(TestBody::Json(r#""hello world""#.to_string()))
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
        .body(TestBody::Json(r#""hello world""#.to_string()))
        .send("optionalJsonRequest");

    TestServiceHandler::new()
        .optional_json_request(|body| {
            assert_eq!(body, None);
            Ok(())
        })
        .call()
        .body(TestBody::Json("null".to_string()))
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
            assert_eq!(body, vec![1, 2, 3, 4]);
            Ok(())
        })
        .call()
        .body(TestBody::Streaming(vec![1, 2, 3, 4]))
        .send("streamingRequest");
}

#[test]
fn streaming_alias_request() {
    TestServiceHandler::new()
        .streaming_alias_request(|body| {
            assert_eq!(body, vec![1, 2, 3, 4]);
            Ok(())
        })
        .call()
        .body(TestBody::Streaming(vec![1, 2, 3, 4]))
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
        .streaming_response(|| Ok(vec![1, 2, 3, 4]))
        .call()
        .response(TestBody::Streaming(vec![1, 2, 3, 4]))
        .send("streamingResponse");
}

#[test]
fn optional_streaming_response() {
    TestServiceHandler::new()
        .optional_streaming_response(|| Ok(Some(vec![1, 2, 3, 4])))
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
        .streaming_alias_response(|| Ok(vec![1, 2, 3, 4]))
        .call()
        .response(TestBody::Streaming(vec![1, 2, 3, 4]))
        .send("streamingAliasResponse");
}

#[test]
fn optional_streaming_alias_response() {
    TestServiceHandler::new()
        .optional_streaming_alias_response(|| Ok(Some(vec![1, 2, 3, 4])))
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
fn endpoint() {
    let endpoint =
        TestServiceResource::<TestServiceHandler>::endpoints::<TestBody, TestResponseVisitor>()
            .into_iter()
            .find(|e| e.name() == "safeParams")
            .unwrap();

    assert_eq!(endpoint.method(), &Method::GET);
    assert_eq!(endpoint.path(), "/test/safeParams/{safePath}/{unsafePath}");
    assert!(!endpoint.deprecated());

    let expected_params = &[
        Parameter::new("safePath", ParameterType::Path(PathParameter::new())).with_safe(true),
        Parameter::new("unsafePath", ParameterType::Path(PathParameter::new())),
        Parameter::new(
            "safeQuery",
            ParameterType::Query(QueryParameter::new("safeQueryId")),
        )
        .with_safe(true),
        Parameter::new(
            "unsafeQuery",
            ParameterType::Query(QueryParameter::new("unsafeQueryId")),
        ),
        Parameter::new(
            "safeHeader",
            ParameterType::Header(HeaderParameter::new("Safe-Header")),
        )
        .with_safe(true),
        Parameter::new(
            "unsafeHeader",
            ParameterType::Header(HeaderParameter::new("Unsafe-Header")),
        ),
    ];

    assert_eq!(endpoint.parameters(), expected_params);
}

#[test]
fn deprecated_endpoint() {
    let endpoint =
        TestServiceResource::<TestServiceHandler>::endpoints::<TestBody, TestResponseVisitor>()
            .into_iter()
            .find(|e| e.name() == "deprecated")
            .unwrap();

    assert!(endpoint.deprecated());
}
