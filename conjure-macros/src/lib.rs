// Copyright 2022 Palantir Technologies, Inc.
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
//! Macros exposed by conjure-http.
//!
//! Do not consume directly.
// Copyright 2022 Palantir Technologies, Inc.
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
//! Macros exposed by conjure-http.
//!
//! Do not consume directly.
#![warn(missing_docs)]

use proc_macro::TokenStream;
use syn::{Error, ItemTrait, TraitItem};

mod client;
mod endpoints;
mod path;

/// Creates a Conjure client type implementing the annotated trait.
///
/// For a trait named `MyService`, the macro will create a type named `MyServiceClient` which
/// implements the Conjure `Client` and `MyService` traits.
///
/// The attribute has several parameters:
///
/// * `name` - The value of the `service` field in the `Endpoint` extension. Defaults to the trait's
///     name.
/// * `version` - The value of the `version` field in the `Endpoint` extension. Defaults to
///     `Some(env!("CARGO_PKG_VERSION"))`.
///
/// # Parameters
///
/// The trait can optionally be declared generic over the request body and response writer types by
/// using the `#[request_writer]` and `#[response_body]` annotations on the type parameters.
///
/// # Endpoints
///
/// Each method corresponds to a separate HTTP endpoint, and is expected to take `&self` and return
/// `Result<T, Error>`. Each must be annotated with `#[endpoint]`, which has several
/// parameters:
///
/// * `method` - The HTTP method (e.g. `GET`). Required.
/// * `path` - The HTTP path template. Path parameters should be identified by `{name}` and must
///     make up an entire path component. Required.
/// * `name` - The value of the `name` field in the `Endpoint` extension. Defaults to the method's
///     name.
/// * `accept` - A type implementing `DeserializeResponse` which will be used to create the return
///     value. Defaults to returning `()`.
///
/// Each method argument must have an annotation describing the type of parameter. One of:
///
/// * `#[path]` - A path parameter. The path template must contain a parameter component matching
///     the argument name.
///
///     Parameters:
///     * `encoder` - A type implementing `EncodeParam` which will be used to encode the value into
///         a string. Defaults to `DisplayParamEncoder`.
/// * `#[query]` - A query parameter.
///
///     Parameters:
///     * `name` - The string used as the key in the encoded URI. Required.
///     * `encoder` - A type implementing `EncodeParam` which will be used to encode the value into
///         a string. Defaults to `DisplayParamEncoder`.
/// * `#[auth]` - A `BearerToken` used to authenticate the request. A method may only have at most
///     one auth parameter.
///
///     Parameters:
///     * `cookie_name` - The name of the cookie used if the token is to be passed via a `Cookie`
///         header. If unset, it will be passed via an `Authorization` header instead.
/// * `#[header]` - A header.
///
///     Parameters:
///     * `name` - The header name. Required.
///     * `encoder` - A type implementing `EncodeHeader` which will be used to encode the value
///         into a header. Defaults to `DisplayHeaderEncoder`.
/// * `#[body]` - The request body. A method may only have at most one body parameter.
///
///     Parameters:
///     * `serializer` - A type implementing `SerializeRequest` which will be used to serialize the
///         value into a body. Defaults to `ConjureRequestSerializer`.
///
/// # Async
///
/// Both blocking and async clients are supported. For technical reasons, async method definitions
/// will be rewritten by the macro to require the returned future be `Send`.
///
/// # Examples
///
/// ```rust
/// use conjure_error::Error;
/// use conjure_http::{conjure_client, endpoint};
/// use conjure_http::client::{
///     AsyncClient, AsyncService, Client, ConjureResponseDeserializer, DeserializeResponse,
///     DisplaySeqEncoder, RequestBody, SerializeRequest, Service, WriteBody,
/// };
/// use conjure_object::BearerToken;
/// use http::Response;
/// use http::header::HeaderValue;
/// use std::io::Write;
///
/// #[conjure_client]
/// trait MyService {
///     #[endpoint(method = GET, path = "/yaks/{yak_id}", accept = ConjureResponseDeserializer)]
///     fn get_yak(&self, #[auth] auth: &BearerToken, #[path] yak_id: i32) -> Result<String, Error>;
///
///     #[endpoint(method = POST, path = "/yaks")]
///     fn create_yak(
///         &self,
///         #[auth] auth_token: &BearerToken,
///         #[query(name = "parentName", encoder = DisplaySeqEncoder)] parent_id: Option<&str>,
///         #[body] yak: &str,
///     ) -> Result<(), Error>;
/// }
///
/// fn do_work(client: impl Client, auth: &BearerToken) -> Result<(), Error> {
///     let client = MyServiceClient::new(client);
///     client.create_yak(auth, None, "my cool yak")?;
///
///     Ok(())
/// }
///
/// #[conjure_client]
/// trait MyServiceAsync {
///     #[endpoint(method = GET, path = "/yaks/{yak_id}", accept = ConjureResponseDeserializer)]
///     async fn get_yak(
///         &self,
///         #[auth] auth: &BearerToken,
///         #[path] yak_id: i32,
///     ) -> Result<String, Error>;
///
///     #[endpoint(method = POST, path = "/yaks")]
///     async fn create_yak(
///         &self,
///         #[auth] auth_token: &BearerToken,
///         #[query(name = "parentName", encoder = DisplaySeqEncoder)] parent_id: Option<&str>,
///         #[body] yak: &str,
///     ) -> Result<(), Error>;
/// }
///
/// async fn do_work_async<C>(client: C, auth: &BearerToken) -> Result<(), Error>
/// where
///     C: AsyncClient + Sync + Send,
///     C::ResponseBody: 'static + Send,
/// {
///     let client = MyServiceAsyncClient::new(client);
///     client.create_yak(auth, None, "my cool yak").await?;
///
///     Ok(())
/// }
///
/// #[conjure_client]
/// trait MyStreamingService<#[response_body] I, #[request_writer] O>
/// where
///     O: Write,
/// {
///     #[endpoint(method = POST, path = "/streamData")]
///     fn upload_stream(
///         &self,
///         #[body(serializer = StreamingRequestSerializer)] body: StreamingRequest,
///     ) -> Result<(), Error>;
///
///     #[endpoint(method = GET, path = "/streamData", accept = StreamingResponseDeserializer)]
///     fn download_stream(&self) -> Result<I, Error>;
/// }
///
/// struct StreamingRequest;
///
/// impl<W> WriteBody<W> for StreamingRequest
/// where
///     W: Write,
/// {
///     fn write_body(&mut self, w: &mut W) -> Result<(), Error> {
///         // ...
///         Ok(())
///     }
///
///     fn reset(&mut self) -> bool {
///         true
///     }
/// }
///
/// enum StreamingRequestSerializer {}
///
/// impl<W> SerializeRequest<'static, StreamingRequest, W> for StreamingRequestSerializer
/// where
///     W: Write,
/// {
///     fn content_type(_: &StreamingRequest) -> HeaderValue {
///         HeaderValue::from_static("text/plain")
///     }
///
///     fn serialize(value: StreamingRequest) -> Result<RequestBody<'static, W>, Error> {
///         Ok(RequestBody::Streaming(Box::new(value)))
///     }
/// }
///
/// enum StreamingResponseDeserializer {}
///
/// impl<R> DeserializeResponse<R, R> for StreamingResponseDeserializer {
///     fn accept() -> Option<HeaderValue> {
///         None
///     }
///
///     fn deserialize(response: Response<R>) -> Result<R, Error> {
///         Ok(response.into_body())
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn conjure_client(attr: TokenStream, item: TokenStream) -> TokenStream {
    client::generate(attr, item)
}

/// Creates a Conjure service type wrapping types implementing the annotated trait.
///
/// For a trait named `MyService`, the macro will create a type named `MyServiceEndpoints` which
/// implements the conjure `Service` trait.
///
/// The attribute has a parameter:
///
/// * `name` - The value returned from the `EndpointMetadata::service_name` method. Defaults to the
///     trait name.
///
/// # Parameters
///
/// The trait can optionally be declared generic over the request body and response writer types by
/// using the `#[request_body]` and `#[response_writer]` annotations on the type parameters.
///
/// # Endpoints
///
/// Each method corresponds to a separate HTTP endpoint, and is expected to take `&self` and return
/// `Result<T, Error>`. Each must be annotated with `#[endpoint]`, which has several parameters:
///
/// * `method` - The HTTP method (e.g. `GET`). Required.
/// * `path` - The HTTP path template. Path parameters should be identified by `{name}` and must
///     make up an entire path component. Required.
/// * `name` - The value returned from the `EndpointMetadata::name` method. Defaults to the method
///     name.
/// * `produces` - A type implementing `SerializeResponse` which will be used to convert the value
///     returned by the method into a response. Defaults to `EmptyResponseSerializer`.
///
/// Each method argument must have an annotation describing the type of parameter. One of:
///
/// * `#[path]` - A path parameter. The path template mut contain a parameter component matching
///     the argument name.
///
///     Parameters:
///     * `decoder` - A type implementing `DecodeParam` which will be used to decode the value.
///         Defaults to `FromStrDecoder`.
///     * `safe` - If set, the parameter will be added to the `SafeParams` response extension.
/// * `#[query]` - A query parameter.
///
///     Parameters:
///     * `name` - The string used as the key in the encoded URI. Required.
///     * `decoder` - A type implementing `DecodeParam` which will be used to decode the value.
///         Defaults to `FromStrDecoder`.
///     * `safe` - If set, the parameter will be added to the `SafeParams` response extension.
/// * `#[auth]` - A `BearerToken` used to authenticate the request.
///
///     Parameters:
///     * `cookie_name` - The name of the cookie if the token is to be parsed from a `Cookie`
///         header. If unset, it will be parsed from an `Authorization` header instead.
/// * `#[header]` - A header parameter.
///
///     Parameters:
///     * `name` - The header name. Required.
///     * `decoder` - A type implementing `DecodeHeader` which will be used to decode the value.
///         Defaults to `FromStrDecoder`.
///     * `safe` - If set, the parameter will be added to the `SafeParams` response extension.
/// * `#[body]` - The request body.
///
///     Parameters:
///     * `deserializer` - A type implementing `DeserializeRequest` which will be used to
///         deserialize the request body into a value. Defaults to `StdRequestDeserializer`.
///     * `safe` - If set, the parameter will be added to the `SafeParams` response extension.
/// * `#[context]` - A `RequestContext` which provides lower level access to the request.
///
/// # Async
///
/// Both blocking and async services are supported. For technical reasons, async method definitions
/// will be rewritten by the macro to require the returned future be `Send`.
///
/// # Examples
///
/// ```rust
/// use conjure_error::Error;
/// use conjure_http::{conjure_endpoints, endpoint};
/// use conjure_http::server::{
///     ConjureRuntime, DeserializeRequest, FromStrOptionDecoder, ResponseBody, SerializeResponse,
///     StdResponseSerializer, WriteBody,
/// };
/// use conjure_object::BearerToken;
/// use http::Response;
/// use http::header::{CONTENT_TYPE, HeaderMap, HeaderValue};
/// use std::io::Write;
///
/// #[conjure_endpoints]
/// trait MyService {
///     #[endpoint(method = GET, path = "/yaks/{yak_id}", produces = StdResponseSerializer)]
///     fn get_yak(
///         &self,
///         #[auth] auth: BearerToken,
///         #[path(safe)] yak_id: i32,
///     ) -> Result<String, Error>;
///
///     #[endpoint(method = POST, path = "/yaks")]
///     fn create_yak(
///         &self,
///         #[auth] auth: BearerToken,
///         #[query(name = "parentName", decoder = FromStrOptionDecoder)] parent_id: Option<String>,
///         #[body] yak: String,
///     ) -> Result<(), Error>;
/// }
///
/// #[conjure_endpoints]
/// trait AsyncMyService {
///     #[endpoint(method = GET, path = "/yaks/{yak_id}", produces = StdResponseSerializer)]
///     async fn get_yak(
///         &self,
///         #[auth] auth: BearerToken,
///         #[path(safe)] yak_id: i32,
///     ) -> Result<String, Error>;
///
///     #[endpoint(method = POST, path = "/yaks")]
///     async fn create_yak(
///         &self,
///         #[auth] auth: BearerToken,
///         #[query(name = "parentName", decoder = FromStrOptionDecoder)] parent_id: Option<String>,
///         #[body] yak: String,
///     ) -> Result<(), Error>;
/// }
///
/// #[conjure_endpoints]
/// trait MyStreamingService<#[request_body] I, #[response_writer] O>
/// where
///     O: Write,
/// {
///     #[endpoint(method = POST, path = "/streamData")]
///     fn receive_stream(
///         &self,
///         #[body(deserializer = StreamingRequestDeserializer)] body: I,
///     )  -> Result<(), Error>;
///
///     #[endpoint(method = GET, path = "/streamData", produces = StreamingResponseSerializer)]
///     fn stream_response(&self) -> Result<StreamingResponse, Error>;
/// }
///
/// struct StreamingRequestDeserializer;
///
/// impl<I> DeserializeRequest<I, I> for StreamingRequestDeserializer {
///     fn deserialize(
///         _runtime: &ConjureRuntime,
///         _headers: &HeaderMap,
///         body: I,
///     ) -> Result<I, Error> {
///         Ok(body)
///     }
/// }
///
/// struct StreamingResponse;
///
/// impl<O> WriteBody<O> for StreamingResponse
/// where
///     O: Write,
/// {
///     fn write_body(self: Box<Self>, w: &mut O) -> Result<(), Error> {
///         // ...
///         Ok(())
///     }
/// }
///
/// struct StreamingResponseSerializer;
///
/// impl<O> SerializeResponse<StreamingResponse, O> for StreamingResponseSerializer
/// where
///     O: Write,
/// {
///     fn serialize(
///         _runtime: &ConjureRuntime,
///         _request_headers: &HeaderMap,
///         body: StreamingResponse,
///     ) -> Result<Response<ResponseBody<O>>, Error> {
///         let mut response = Response::new(ResponseBody::Streaming(Box::new(body)));
///         response.headers_mut().insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
///         Ok(response)
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn conjure_endpoints(attr: TokenStream, item: TokenStream) -> TokenStream {
    endpoints::generate(attr, item)
}

/// A no-op attribute macro required due to technical limitations of Rust's macro system.
#[proc_macro_attribute]
pub fn endpoint(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

struct Errors(Vec<Error>);

impl Errors {
    fn new() -> Self {
        Errors(vec![])
    }

    fn push(&mut self, error: Error) {
        self.0.push(error);
    }

    fn build(mut self) -> Result<(), Error> {
        let Some(mut error) = self.0.pop() else {
            return Ok(());
        };
        for other in self.0 {
            error.combine(other);
        }
        Err(error)
    }
}

#[derive(Copy, Clone)]
enum Asyncness {
    Sync,
    Async,
}

impl Asyncness {
    fn resolve(trait_: &ItemTrait) -> Result<Self, Error> {
        let mut it = trait_.items.iter().filter_map(|t| match t {
            TraitItem::Fn(f) => Some(f),
            _ => None,
        });

        let Some(first) = it.next() else {
            return Ok(Asyncness::Sync);
        };

        let is_async = first.sig.asyncness.is_some();

        let mut errors = Errors::new();

        for f in it {
            if f.sig.asyncness.is_some() != is_async {
                errors.push(Error::new_spanned(
                    f,
                    "all methods must either be sync or async",
                ));
            }
        }

        errors.build()?;
        let asyncness = if is_async {
            Asyncness::Async
        } else {
            Asyncness::Sync
        };
        Ok(asyncness)
    }
}
