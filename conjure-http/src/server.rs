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

//! The Conjure HTTP server API.
use crate::private::{self, APPLICATION_JSON, SERIALIZABLE_REQUEST_SIZE_LIMIT};
use async_trait::async_trait;
use bytes::Bytes;
use conjure_error::{Error, InvalidArgument};
use conjure_serde::json;
use futures_core::Stream;
use http::header::CONTENT_TYPE;
use http::{
    request, Extensions, HeaderMap, HeaderValue, Method, Request, Response, StatusCode, Uri,
};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::borrow::Cow;
use std::error;
use std::future::Future;
use std::io::Write;
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::ops::Deref;
use std::pin::Pin;
use std::str;
use std::str::FromStr;

/// Metadata about an HTTP endpoint.
pub trait EndpointMetadata {
    /// The endpoint's HTTP method.
    fn method(&self) -> Method;

    /// The endpoint's parsed HTTP URI path.
    ///
    /// Each value in the slice represents one segment of the URI.
    fn path(&self) -> &[PathSegment];

    /// The endpoint's raw HTTP URI template.
    ///
    /// Use the [`Self::path()`] method for routing rather than parsing this string.
    fn template(&self) -> &str;

    /// The name of the service defining this endpoint.
    fn service_name(&self) -> &str;

    /// The name of the endpoint.
    fn name(&self) -> &str;

    /// If the endpoint is deprecated, returns the deprecation documentation.
    fn deprecated(&self) -> Option<&str>;
}

impl<T> EndpointMetadata for Box<T>
where
    T: ?Sized + EndpointMetadata,
{
    fn method(&self) -> Method {
        (**self).method()
    }

    fn path(&self) -> &[PathSegment] {
        (**self).path()
    }

    fn template(&self) -> &str {
        (**self).template()
    }

    fn service_name(&self) -> &str {
        (**self).service_name()
    }

    fn name(&self) -> &str {
        (**self).name()
    }

    fn deprecated(&self) -> Option<&str> {
        (**self).deprecated()
    }
}

/// A blocking HTTP endpoint.
pub trait Endpoint<I, O>: EndpointMetadata {
    /// Handles a request to the endpoint.
    ///
    /// If the endpoint has path parameters, callers must include a
    /// [`PathParams`](crate::PathParams) extension in the request containing the extracted
    /// parameters from the URI. The implementation is reponsible for all other request handling,
    /// including parsing query parameters, header parameters, and the request body.
    ///
    /// The `response_extensions` will be added to the extensions of the response produced by the
    /// endpoint, even if an error is returned.
    fn handle(
        &self,
        req: Request<I>,
        response_extensions: &mut Extensions,
    ) -> Result<Response<ResponseBody<O>>, Error>;
}

impl<T, I, O> Endpoint<I, O> for Box<T>
where
    T: ?Sized + Endpoint<I, O>,
{
    fn handle(
        &self,
        req: Request<I>,
        response_extensions: &mut Extensions,
    ) -> Result<Response<ResponseBody<O>>, Error> {
        (**self).handle(req, response_extensions)
    }
}

/// A nonblocking HTTP endpoint.
#[async_trait]
pub trait AsyncEndpoint<I, O>: EndpointMetadata {
    /// Handles a request to the endpoint.
    ///
    /// If the endpoint has path parameters, callers must include a
    /// [`PathParams`](crate::PathParams) extension in the request containing the extracted
    /// parameters from the URI. The implementation is reponsible for all other request handling,
    /// including parsing query parameters, header parameters, and the request body.
    ///
    /// The `response_extensions` will be added to the extensions of the response produced by the
    /// endpoint, even if an error is returned.
    async fn handle(
        &self,
        req: Request<I>,
        response_extensions: &mut Extensions,
    ) -> Result<Response<AsyncResponseBody<O>>, Error>
    where
        I: 'async_trait;
}

impl<T, I, O> AsyncEndpoint<I, O> for Box<T>
where
    T: ?Sized + AsyncEndpoint<I, O>,
{
    #[allow(clippy::type_complexity)]
    fn handle<'life0, 'life1, 'async_trait>(
        &'life0 self,
        req: Request<I>,
        response_extensions: &'life1 mut Extensions,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<Response<AsyncResponseBody<O>>, Error>>
                + Send
                + 'async_trait,
        >,
    >
    where
        I: 'async_trait,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        (**self).handle(req, response_extensions)
    }
}

/// One segment of an endpoint URI template.
#[derive(Debug, Clone)]
pub enum PathSegment {
    /// A literal string.
    Literal(Cow<'static, str>),

    /// A parameter.
    Parameter {
        /// The name of the parameter.
        name: Cow<'static, str>,

        /// The regex pattern used to match the pattern.
        regex: Option<Cow<'static, str>>,
    },
}

/// The response body returned from a blocking endpoint.
pub enum ResponseBody<O> {
    /// An empty body.
    Empty,
    /// A body buffered in memory.
    Fixed(Bytes),
    /// A streaming body.
    Streaming(Box<dyn WriteBody<O>>),
}

/// The response body returned from an async endpoint.
pub enum AsyncResponseBody<O> {
    /// An empty body.
    Empty,
    /// A body buffered in memory.
    Fixed(Bytes),
    /// A streaming body.
    Streaming(Box<dyn AsyncWriteBody<O> + Send>),
}

/// A blocking Conjure service.
pub trait Service<I, O> {
    /// Returns the endpoints in the service.
    fn endpoints(&self) -> Vec<Box<dyn Endpoint<I, O> + Sync + Send>>;
}

/// An async Conjure service.
pub trait AsyncService<I, O> {
    /// Returns the endpoints in the service.
    fn endpoints(&self) -> Vec<Box<dyn AsyncEndpoint<I, O> + Sync + Send>>;
}

/// A trait implemented by streaming bodies.
pub trait WriteBody<W> {
    /// Writes the body out, in its entirety.
    // This should not be limited to `Box<Self>`, but it otherwise can't be used as a trait object currently :(
    fn write_body(self: Box<Self>, w: &mut W) -> Result<(), Error>;
}

impl<W> WriteBody<W> for Vec<u8>
where
    W: Write,
{
    fn write_body(self: Box<Self>, w: &mut W) -> Result<(), Error> {
        w.write_all(&self).map_err(Error::internal_safe)
    }
}

/// A trait implemented by asynchronous streaming bodies.
///
/// This trait can most easily be implemented with the [async-trait crate](https://docs.rs/async-trait).
///
/// # Examples
///
/// ```ignore
/// use async_trait::async_trait;
/// use conjure_error::Error;
/// use conjure_http::server::AsyncWriteBody;
/// use std::pin::Pin;
/// use tokio_io::{AsyncWrite, AsyncWriteExt};
///
/// pub struct SimpleBodyWriter;
///
/// #[async_trait]
/// impl<W> AsyncWriteBody<W> for SimpleBodyWriter
/// where
///     W: AsyncWrite + Send,
/// {
///     async fn write_body(self, mut w: Pin<&mut W>) -> Result<(), Error> {
///         w.write_all(b"hello world").await.map_err(Error::internal_safe)
///     }
/// }
/// ```
#[async_trait]
pub trait AsyncWriteBody<W> {
    /// Writes the body out, in its entirety.
    // This should not be limited to `Box<Self>`, but it otherwise can't be used as a trait object currently :(
    async fn write_body(self: Box<Self>, w: Pin<&mut W>) -> Result<(), Error>;
}

/// An object containing extra low-level contextual information about a request.
///
/// Conjure service endpoints declared with the `server-request-context` tag will be passed a
/// `RequestContext` in the generated trait.
pub struct RequestContext<'a> {
    request_parts: MaybeBorrowed<'a, request::Parts>,
    response_extensions: &'a mut Extensions,
}

impl<'a> RequestContext<'a> {
    // This is public API but not exposed in docs since it should only be called by generated code.
    #[doc(hidden)]
    #[inline]
    // FIXME remove in favor of borrowed constructor
    pub fn new(request_parts: request::Parts, response_extensions: &'a mut Extensions) -> Self {
        RequestContext {
            request_parts: MaybeBorrowed::Owned(request_parts),
            response_extensions,
        }
    }

    #[doc(hidden)]
    #[inline]
    pub fn new2(
        request_parts: &'a request::Parts,
        response_extensions: &'a mut Extensions,
    ) -> Self {
        RequestContext {
            request_parts: MaybeBorrowed::Borrowed(request_parts),
            response_extensions,
        }
    }

    /// Returns the request's URI.
    #[inline]
    pub fn request_uri(&self) -> &Uri {
        &self.request_parts.uri
    }

    /// Returns a shared reference to the request's headers.
    #[inline]
    pub fn request_headers(&self) -> &HeaderMap {
        &self.request_parts.headers
    }

    /// Returns a shared reference to the request's extensions.
    #[inline]
    pub fn request_extensions(&self) -> &Extensions {
        &self.request_parts.extensions
    }

    /// Returns a shared reference to extensions that will be added to the response.
    #[inline]
    pub fn response_extensions(&self) -> &Extensions {
        self.response_extensions
    }

    /// Returns a mutable reference to extensions that will be added to the response.
    #[inline]
    pub fn response_extensions_mut(&mut self) -> &mut Extensions {
        self.response_extensions
    }
}

enum MaybeBorrowed<'a, T> {
    Borrowed(&'a T),
    Owned(T),
}

impl<T> Deref for MaybeBorrowed<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        match self {
            MaybeBorrowed::Borrowed(v) => v,
            MaybeBorrowed::Owned(v) => v,
        }
    }
}

/// A trait implemented by request body deserializers used by custom Conjure server trait
/// implementations.
pub trait DeserializeRequest<T, R> {
    /// Deserializes the request body.
    fn deserialize(headers: &HeaderMap, body: R) -> Result<T, Error>;
}

/// A trait implemented by response deserializers used by custom async Conjure server trait
/// implementations.
#[async_trait]
pub trait AsyncDeserializeRequest<T, R> {
    /// Deserializes the request body.
    async fn deserialize(headers: &HeaderMap, body: R) -> Result<T, Error>
    where
        R: 'async_trait;
}

/// A request deserializer which acts as a Conjure-generated endpoint would.
pub enum ConjureRequestDeserializer {}

impl ConjureRequestDeserializer {
    fn check_content_type(headers: &HeaderMap) -> Result<(), Error> {
        if headers.get(CONTENT_TYPE) != Some(&APPLICATION_JSON) {
            return Err(Error::service_safe(
                "invalid request Content-Type",
                InvalidArgument::new(),
            ));
        }

        Ok(())
    }
}

impl<T, R> DeserializeRequest<T, R> for ConjureRequestDeserializer
where
    T: DeserializeOwned,
    R: Iterator<Item = Result<Bytes, Error>>,
{
    fn deserialize(headers: &HeaderMap, body: R) -> Result<T, Error> {
        Self::check_content_type(headers)?;
        let buf = private::read_body(body, Some(SERIALIZABLE_REQUEST_SIZE_LIMIT))?;
        json::server_from_slice(&buf).map_err(|e| Error::service(e, InvalidArgument::new()))
    }
}

#[async_trait]
impl<T, R> AsyncDeserializeRequest<T, R> for ConjureRequestDeserializer
where
    T: DeserializeOwned,
    R: Stream<Item = Result<Bytes, Error>> + Send,
{
    async fn deserialize(headers: &HeaderMap, body: R) -> Result<T, Error>
    where
        R: 'async_trait,
    {
        Self::check_content_type(headers)?;
        let buf = private::async_read_body(body, Some(SERIALIZABLE_REQUEST_SIZE_LIMIT)).await?;
        json::server_from_slice(&buf).map_err(|e| Error::service(e, InvalidArgument::new()))
    }
}

/// A trait implemented by response serializers used by custom Conjure server trait implementations.
pub trait SerializeResponse<T, W> {
    /// Serializes the response.
    fn serialize(request_headers: &HeaderMap, value: T)
        -> Result<Response<ResponseBody<W>>, Error>;
}

/// A trait implemented by response serializers used by custom async Conjure server trait
/// implementations.
pub trait AsyncSerializeResponse<T, W> {
    /// Serializes the response.
    fn serialize(
        request_headers: &HeaderMap,
        value: T,
    ) -> Result<Response<AsyncResponseBody<W>>, Error>;
}

/// A serializer which encodes `()` as an empty body and status code of `204 No Content`.
pub enum EmptyResponseSerializer {}

impl EmptyResponseSerializer {
    fn serialize_inner<T>(body: T) -> Result<Response<T>, Error> {
        let mut response = Response::new(body);
        *response.status_mut() = StatusCode::NO_CONTENT;
        Ok(response)
    }
}

impl<W> SerializeResponse<(), W> for EmptyResponseSerializer {
    fn serialize(_: &HeaderMap, _: ()) -> Result<Response<ResponseBody<W>>, Error> {
        Self::serialize_inner(ResponseBody::Empty)
    }
}

impl<W> AsyncSerializeResponse<(), W> for EmptyResponseSerializer {
    fn serialize(_: &HeaderMap, _: ()) -> Result<Response<AsyncResponseBody<W>>, Error> {
        Self::serialize_inner(AsyncResponseBody::Empty)
    }
}

/// A serializer which acts like a Conjure-generated client would.
pub enum ConjureResponseSerializer {}

impl ConjureResponseSerializer {
    fn serialize_inner<T, B>(
        value: T,
        make_body: impl FnOnce(Bytes) -> B,
    ) -> Result<Response<B>, Error>
    where
        T: Serialize,
    {
        let body = json::to_vec(&value).map_err(Error::internal)?;

        let mut response = Response::new(make_body(body.into()));
        response
            .headers_mut()
            .insert(CONTENT_TYPE, APPLICATION_JSON);

        Ok(response)
    }
}

impl<T, W> SerializeResponse<T, W> for ConjureResponseSerializer
where
    T: Serialize,
{
    fn serialize(
        _request_headers: &HeaderMap,
        value: T,
    ) -> Result<Response<ResponseBody<W>>, Error> {
        Self::serialize_inner(value, ResponseBody::Fixed)
    }
}

impl<T, W> AsyncSerializeResponse<T, W> for ConjureResponseSerializer
where
    T: Serialize,
{
    fn serialize(
        _request_headers: &HeaderMap,
        value: T,
    ) -> Result<Response<AsyncResponseBody<W>>, Error> {
        Self::serialize_inner(value, AsyncResponseBody::Fixed)
    }
}

/// A trait implemented by header decoders used by custom Conjure server trait implementations.
pub trait DecodeHeader<T> {
    /// Decodes the value from headers.
    fn decode<'a, I>(headers: I) -> Result<T, Error>
    where
        I: IntoIterator<Item = &'a HeaderValue>;
}

/// A trait implemented by URL parameter decoders used by custom Conjure server trait
/// implementations.
pub trait DecodeParam<T> {
    /// Decodes the value from the sequence of values.
    ///
    /// The values have already been percent-decoded.
    fn decode<I>(params: I) -> Result<T, Error>
    where
        I: IntoIterator,
        I::Item: AsRef<str>;
}

/// A decoder which converts a single value using its [`FromStr`] implementation.
pub enum FromStrDecoder {}

impl<T> DecodeHeader<T> for FromStrDecoder
where
    T: FromStr,
    T::Err: Into<Box<dyn error::Error + Sync + Send>>,
{
    fn decode<'a, I>(headers: I) -> Result<T, Error>
    where
        I: IntoIterator<Item = &'a HeaderValue>,
    {
        only_item(headers)?
            .to_str()
            .map_err(|e| Error::service(e, InvalidArgument::new()))?
            .parse()
            .map_err(|e| Error::service(e, InvalidArgument::new()))
    }
}

impl<T> DecodeParam<T> for FromStrDecoder
where
    T: FromStr,
    T::Err: Into<Box<dyn error::Error + Sync + Send>>,
{
    fn decode<'a, I>(params: I) -> Result<T, Error>
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        only_item(params)?
            .as_ref()
            .parse()
            .map_err(|e| Error::service(e, InvalidArgument::new()))
    }
}

/// A decoder which converts an optional value using its [`FromStr`] implementation.
pub enum FromStrOptionDecoder {}

impl<T> DecodeHeader<Option<T>> for FromStrOptionDecoder
where
    T: FromStr,
    T::Err: Into<Box<dyn error::Error + Sync + Send>>,
{
    fn decode<'a, I>(headers: I) -> Result<Option<T>, Error>
    where
        I: IntoIterator<Item = &'a HeaderValue>,
    {
        let Some(header) = optional_item(headers)? else { return Ok(None) };
        let value = header
            .to_str()
            .map_err(|e| Error::service(e, InvalidArgument::new()))?
            .parse()
            .map_err(|e| Error::service(e, InvalidArgument::new()))?;
        Ok(Some(value))
    }
}

impl<T> DecodeParam<Option<T>> for FromStrOptionDecoder
where
    T: FromStr,
    T::Err: Into<Box<dyn error::Error + Sync + Send>>,
{
    fn decode<I>(params: I) -> Result<Option<T>, Error>
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        let Some(param) = optional_item(params)? else { return Ok(None) };
        let value = param
            .as_ref()
            .parse()
            .map_err(|e| Error::service(e, InvalidArgument::new()))?;
        Ok(Some(value))
    }
}

fn optional_item<I>(it: I) -> Result<Option<I::Item>, Error>
where
    I: IntoIterator,
{
    let mut it = it.into_iter();
    let Some(item) = it.next() else { return Ok(None) };

    let remaining = it.count();
    if remaining > 0 {
        return Err(
            Error::service_safe("expected at most 1 parameter", InvalidArgument::new())
                .with_safe_param("actual", remaining + 1),
        );
    }

    Ok(Some(item))
}

/// A decoder which converts a sequence of values via its [`FromStr`] implementation into a
/// collection via a [`FromIterator`] implementation.
pub struct FromStrSeqDecoder<U> {
    _p: PhantomData<U>,
}

impl<T, U> DecodeParam<T> for FromStrSeqDecoder<U>
where
    T: FromIterator<U>,
    U: FromStr,
    U::Err: Into<Box<dyn error::Error + Sync + Send>>,
{
    fn decode<I>(params: I) -> Result<T, Error>
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        params
            .into_iter()
            .map(|s| {
                s.as_ref()
                    .parse()
                    .map_err(|e| Error::service(e, InvalidArgument::new()))
            })
            .collect()
    }
}

fn only_item<I>(it: I) -> Result<I::Item, Error>
where
    I: IntoIterator,
{
    let mut it = it.into_iter();
    let Some(item) = it.next() else {
        return Err(Error::service_safe(
            "expected exactly 1 parameter",
            InvalidArgument::new(),
        ).with_safe_param("actual", 0));
    };

    let remaining = it.count();
    if remaining > 0 {
        return Err(
            Error::service_safe("expected exactly 1 parameter", InvalidArgument::new())
                .with_safe_param("actual", remaining + 1),
        );
    }

    Ok(item)
}
