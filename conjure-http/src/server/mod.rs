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
use bytes::Bytes;
use conjure_error::{Error, InvalidArgument};
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
use std::pin::Pin;
use std::str;
use std::str::FromStr;
use std::sync::Arc;

use crate::private::{self, SERIALIZABLE_REQUEST_SIZE_LIMIT};
pub use crate::server::encoding::*;
#[doc(inline)]
pub use crate::server::runtime::ConjureRuntime;

pub mod conjure;
mod encoding;
pub mod runtime;

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
    fn handle(
        &self,
        req: Request<I>,
        response_extensions: &mut Extensions,
    ) -> impl Future<Output = Result<Response<AsyncResponseBody<O>>, Error>> + Send;
}

impl<T, I, O> AsyncEndpoint<I, O> for Box<T>
where
    T: ?Sized + AsyncEndpoint<I, O>,
{
    fn handle(
        &self,
        req: Request<I>,
        response_extensions: &mut Extensions,
    ) -> impl Future<Output = Result<Response<AsyncResponseBody<O>>, Error>> + Send {
        (**self).handle(req, response_extensions)
    }
}

// An internal object-safe version of AsyncEndpoint used to implement BoxAsyncEndpoint
trait AsyncEndpointEraser<I, O>: EndpointMetadata {
    #[allow(clippy::type_complexity)]
    fn handle<'a>(
        &'a self,
        req: Request<I>,
        response_extensions: &'a mut Extensions,
    ) -> Pin<Box<dyn Future<Output = Result<Response<AsyncResponseBody<O>>, Error>> + Send + 'a>>
    where
        I: 'a,
        O: 'a;
}

impl<T, I, O> AsyncEndpointEraser<I, O> for T
where
    T: AsyncEndpoint<I, O>,
{
    fn handle<'a>(
        &'a self,
        req: Request<I>,
        response_extensions: &'a mut Extensions,
    ) -> Pin<Box<dyn Future<Output = Result<Response<AsyncResponseBody<O>>, Error>> + Send + 'a>>
    where
        I: 'a,
        O: 'a,
    {
        Box::pin(self.handle(req, response_extensions))
    }
}

/// A boxed [`AsyncEndpoint`] trait object.
pub struct BoxAsyncEndpoint<'a, I, O> {
    inner: Box<dyn AsyncEndpointEraser<I, O> + 'a + Sync + Send>,
}

impl<'a, I, O> BoxAsyncEndpoint<'a, I, O> {
    /// Creates a new `BoxAsyncEndpoint`.
    pub fn new<T>(v: T) -> Self
    where
        T: AsyncEndpoint<I, O> + Sync + Send + 'a,
    {
        BoxAsyncEndpoint { inner: Box::new(v) }
    }
}

impl<I, O> EndpointMetadata for BoxAsyncEndpoint<'_, I, O> {
    fn method(&self) -> Method {
        self.inner.method()
    }

    fn path(&self) -> &[PathSegment] {
        self.inner.path()
    }

    fn template(&self) -> &str {
        self.inner.template()
    }

    fn service_name(&self) -> &str {
        self.inner.service_name()
    }

    fn name(&self) -> &str {
        self.inner.name()
    }

    fn deprecated(&self) -> Option<&str> {
        self.inner.deprecated()
    }
}

impl<I, O> AsyncEndpoint<I, O> for BoxAsyncEndpoint<'_, I, O>
where
    I: Send,
{
    async fn handle(
        &self,
        req: Request<I>,
        response_extensions: &mut Extensions,
    ) -> Result<Response<AsyncResponseBody<O>>, Error> {
        self.inner.handle(req, response_extensions).await
    }
}

/// A nonblocking local HTTP endpoint.
pub trait LocalAsyncEndpoint<I, O>: EndpointMetadata {
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
    ) -> impl Future<Output = Result<Response<LocalAsyncResponseBody<O>>, Error>>;
}

impl<T, I, O> LocalAsyncEndpoint<I, O> for Box<T>
where
    T: ?Sized + LocalAsyncEndpoint<I, O>,
{
    fn handle(
        &self,
        req: Request<I>,
        response_extensions: &mut Extensions,
    ) -> impl Future<Output = Result<Response<LocalAsyncResponseBody<O>>, Error>> {
        (**self).handle(req, response_extensions)
    }
}

// An internal object-safe version of LocalAsyncEndpoint used to implement BoxLocalAsyncEndpoint
trait LocalAsyncEndpointEraser<I, O>: EndpointMetadata {
    #[allow(clippy::type_complexity)]
    fn handle<'a>(
        &'a self,
        req: Request<I>,
        response_extensions: &'a mut Extensions,
    ) -> Pin<Box<dyn Future<Output = Result<Response<LocalAsyncResponseBody<O>>, Error>> + 'a>>
    where
        I: 'a,
        O: 'a;
}

impl<T, I, O> LocalAsyncEndpointEraser<I, O> for T
where
    T: LocalAsyncEndpoint<I, O>,
{
    fn handle<'a>(
        &'a self,
        req: Request<I>,
        response_extensions: &'a mut Extensions,
    ) -> Pin<Box<dyn Future<Output = Result<Response<LocalAsyncResponseBody<O>>, Error>> + 'a>>
    where
        I: 'a,
        O: 'a,
    {
        Box::pin(self.handle(req, response_extensions))
    }
}

/// A boxed [`LocalAsyncEndpoint`] trait object.
pub struct BoxLocalAsyncEndpoint<'a, I, O> {
    inner: Box<dyn LocalAsyncEndpointEraser<I, O> + 'a>,
}

impl<'a, I, O> BoxLocalAsyncEndpoint<'a, I, O> {
    /// Creates a new `BoxLocalAsyncEndpoint`.
    pub fn new<T>(v: T) -> Self
    where
        T: LocalAsyncEndpoint<I, O> + 'a,
    {
        BoxLocalAsyncEndpoint { inner: Box::new(v) }
    }
}

impl<I, O> EndpointMetadata for BoxLocalAsyncEndpoint<'_, I, O> {
    fn method(&self) -> Method {
        self.inner.method()
    }

    fn path(&self) -> &[PathSegment] {
        self.inner.path()
    }

    fn template(&self) -> &str {
        self.inner.template()
    }

    fn service_name(&self) -> &str {
        self.inner.service_name()
    }

    fn name(&self) -> &str {
        self.inner.name()
    }

    fn deprecated(&self) -> Option<&str> {
        self.inner.deprecated()
    }
}

impl<I, O> LocalAsyncEndpoint<I, O> for BoxLocalAsyncEndpoint<'_, I, O> {
    async fn handle(
        &self,
        req: Request<I>,
        response_extensions: &mut Extensions,
    ) -> Result<Response<LocalAsyncResponseBody<O>>, Error> {
        self.inner.handle(req, response_extensions).await
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
    Streaming(BoxAsyncWriteBody<'static, O>),
}

/// The response body returned from a local async endpoint.
pub enum LocalAsyncResponseBody<O> {
    /// An empty body.
    Empty,
    /// A body buffered in memory.
    Fixed(Bytes),
    /// A streaming body.
    Streaming(BoxLocalAsyncWriteBody<'static, O>),
}

/// A blocking Conjure service.
pub trait Service<I, O> {
    /// Returns the endpoints in the service.
    fn endpoints(
        &self,
        runtime: &Arc<ConjureRuntime>,
    ) -> Vec<Box<dyn Endpoint<I, O> + Sync + Send>>;
}

/// An async Conjure service.
pub trait AsyncService<I, O> {
    /// Returns the endpoints in the service.
    fn endpoints(&self, runtime: &Arc<ConjureRuntime>) -> Vec<BoxAsyncEndpoint<'static, I, O>>;
}

/// A local async Conjure service.
pub trait LocalAsyncService<I, O> {
    /// Returns the endpoints in the service.
    fn endpoints(&self, runtime: &Arc<ConjureRuntime>)
        -> Vec<BoxLocalAsyncEndpoint<'static, I, O>>;
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
/// # Examples
///
/// ```ignore
/// use conjure_error::Error;
/// use conjure_http::server::AsyncWriteBody;
/// use std::pin::Pin;
/// use tokio_io::{AsyncWrite, AsyncWriteExt};
///
/// pub struct SimpleBodyWriter;
///
/// impl<W> AsyncWriteBody<W> for SimpleBodyWriter
/// where
///     W: AsyncWrite + Send,
/// {
///     async fn write_body(self, mut w: Pin<&mut W>) -> Result<(), Error> {
///         w.write_all(b"hello world").await.map_err(Error::internal_safe)
///     }
/// }
/// ```
pub trait AsyncWriteBody<W> {
    /// Writes the body out, in its entirety.
    fn write_body(self, w: Pin<&mut W>) -> impl Future<Output = Result<(), Error>> + Send;
}

// An internal object-safe version of AsyncWriteBody used to implement BoxAsyncWriteBody
trait AsyncWriteBodyEraser<W> {
    fn write_body<'a>(
        self: Box<Self>,
        w: Pin<&'a mut W>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>
    where
        Self: 'a;
}

impl<T, W> AsyncWriteBodyEraser<W> for T
where
    T: AsyncWriteBody<W>,
{
    fn write_body<'a>(
        self: Box<Self>,
        w: Pin<&'a mut W>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>
    where
        Self: 'a,
    {
        Box::pin((*self).write_body(w))
    }
}

/// A boxed [`AsyncWriteBody`] trait object.
pub struct BoxAsyncWriteBody<'a, W> {
    inner: Box<dyn AsyncWriteBodyEraser<W> + Send + 'a>,
}

impl<'a, W> BoxAsyncWriteBody<'a, W> {
    /// Creates a new `BoxAsyncWriteBody`.
    pub fn new<T>(v: T) -> Self
    where
        T: AsyncWriteBody<W> + Send + 'a,
    {
        BoxAsyncWriteBody { inner: Box::new(v) }
    }
}

impl<W> AsyncWriteBody<W> for BoxAsyncWriteBody<'_, W>
where
    W: Send,
{
    async fn write_body(self, w: Pin<&mut W>) -> Result<(), Error>
    where
        W: Send,
    {
        self.inner.write_body(w).await
    }
}

/// A trait implemented by local asynchronous streaming bodies.
///
/// # Examples
///
/// ```ignore
/// use conjure_error::Error;
/// use conjure_http::server::LocalAsyncWriteBody;
/// use std::pin::Pin;
/// use tokio_io::{AsyncWrite, AsyncWriteExt};
///
/// pub struct SimpleBodyWriter;
///
/// impl<W> LocalAsyncWriteBody<W> for SimpleBodyWriter
/// where
///     W: AsyncWrite,
/// {
///     async fn write_body(self, mut w: Pin<&mut W>) -> Result<(), Error> {
///         w.write_all(b"hello world").await.map_err(Error::internal_safe)
///     }
/// }
/// ```
pub trait LocalAsyncWriteBody<W> {
    /// Writes the body out, in its entirety.
    fn write_body(self, w: Pin<&mut W>) -> impl Future<Output = Result<(), Error>>;
}

// An internal object-safe version of LocalAsyncWriteBody used to implement BoxLocalAsyncWriteBody
trait LocalAsyncWriteBodyEraser<W> {
    fn write_body<'a>(
        self: Box<Self>,
        w: Pin<&'a mut W>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + 'a>>
    where
        Self: 'a;
}

impl<T, W> LocalAsyncWriteBodyEraser<W> for T
where
    T: LocalAsyncWriteBody<W>,
{
    fn write_body<'a>(
        self: Box<Self>,
        w: Pin<&'a mut W>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + 'a>>
    where
        Self: 'a,
    {
        Box::pin((*self).write_body(w))
    }
}

/// A boxed [`LocalAsyncWriteBody`] trait object.
pub struct BoxLocalAsyncWriteBody<'a, W> {
    inner: Box<dyn LocalAsyncWriteBodyEraser<W> + 'a>,
}

impl<'a, W> BoxLocalAsyncWriteBody<'a, W> {
    /// Creates a new `BoxLocalAsyncWriteBody`.
    pub fn new<T>(v: T) -> Self
    where
        T: LocalAsyncWriteBody<W> + 'a,
    {
        BoxLocalAsyncWriteBody { inner: Box::new(v) }
    }
}

impl<W> LocalAsyncWriteBody<W> for BoxLocalAsyncWriteBody<'_, W> {
    async fn write_body(self, w: Pin<&mut W>) -> Result<(), Error> {
        self.inner.write_body(w).await
    }
}

/// An object containing extra low-level contextual information about a request.
///
/// Conjure service endpoints declared with the `server-request-context` tag will be passed a
/// `RequestContext` in the generated trait.
pub struct RequestContext<'a> {
    request_parts: &'a request::Parts,
    response_extensions: &'a mut Extensions,
}

impl<'a> RequestContext<'a> {
    #[doc(hidden)]
    #[inline]
    pub fn new(request_parts: &'a request::Parts, response_extensions: &'a mut Extensions) -> Self {
        RequestContext {
            request_parts,
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

/// A marker value to opt into legacy error serialization.
///
/// If present in the response extensions of a request, server implementations should use
/// [`conjure_error::stringify_parameters`] to convert all error parameters to their legacy
/// stringified format.
#[derive(Copy, Clone, Debug)]
pub struct UseLegacyErrorSerialization;

/// A trait implemented by request body deserializers used by custom Conjure server trait
/// implementations.
pub trait DeserializeRequest<T, R> {
    /// Deserializes the request body.
    fn deserialize(runtime: &ConjureRuntime, headers: &HeaderMap, body: R) -> Result<T, Error>;
}

/// A trait implemented by response deserializers used by custom async Conjure server trait
/// implementations.
pub trait AsyncDeserializeRequest<T, R> {
    /// Deserializes the request body.
    fn deserialize(
        runtime: &ConjureRuntime,
        headers: &HeaderMap,
        body: R,
    ) -> impl Future<Output = Result<T, Error>> + Send;
}

/// A trait implemented by response deserializers used by custom local async Conjure server trait
/// implementations.
pub trait LocalAsyncDeserializeRequest<T, R> {
    /// Deserializes the request body.
    fn deserialize(
        runtime: &ConjureRuntime,
        headers: &HeaderMap,
        body: R,
    ) -> impl Future<Output = Result<T, Error>>;
}

/// A request deserializer for standard body types.
///
/// It is parameterized by the maximum number of bytes that will be read from the request body
/// before an error is returned. The limit defaults to 50 MiB.
pub enum StdRequestDeserializer<const N: usize = { SERIALIZABLE_REQUEST_SIZE_LIMIT }> {}

impl<const N: usize> StdRequestDeserializer<N> {
    async fn deserialize_inner<T, R>(
        runtime: &ConjureRuntime,
        headers: &HeaderMap,
        body: R,
    ) -> Result<T, Error>
    where
        T: DeserializeOwned,
        R: Stream<Item = Result<Bytes, Error>>,
    {
        let encoding = runtime.request_body_encoding(headers)?;
        let buf = private::async_read_body(body, Some(N)).await?;
        let v = T::deserialize(encoding.deserializer(&buf).deserializer())
            .map_err(|e| Error::service(e, InvalidArgument::new()))?;
        Ok(v)
    }
}

impl<const N: usize, T, R> DeserializeRequest<T, R> for StdRequestDeserializer<N>
where
    T: DeserializeOwned,
    R: Iterator<Item = Result<Bytes, Error>>,
{
    fn deserialize(runtime: &ConjureRuntime, headers: &HeaderMap, body: R) -> Result<T, Error> {
        let encoding = runtime.request_body_encoding(headers)?;
        let buf = private::read_body(body, Some(N))?;
        let v = T::deserialize(encoding.deserializer(&buf).deserializer())
            .map_err(|e| Error::service(e, InvalidArgument::new()))?;
        Ok(v)
    }
}

impl<const N: usize, T, R> AsyncDeserializeRequest<T, R> for StdRequestDeserializer<N>
where
    T: DeserializeOwned,
    R: Stream<Item = Result<Bytes, Error>> + Send,
{
    async fn deserialize(
        runtime: &ConjureRuntime,
        headers: &HeaderMap,
        body: R,
    ) -> Result<T, Error> {
        Self::deserialize_inner(runtime, headers, body).await
    }
}

impl<const N: usize, T, R> LocalAsyncDeserializeRequest<T, R> for StdRequestDeserializer<N>
where
    T: DeserializeOwned,
    R: Stream<Item = Result<Bytes, Error>>,
{
    async fn deserialize(
        runtime: &ConjureRuntime,
        headers: &HeaderMap,
        body: R,
    ) -> Result<T, Error> {
        Self::deserialize_inner(runtime, headers, body).await
    }
}

/// A request deserializer which maps the output of another with [`From::from`].
pub struct FromRequestDeserializer<D, U> {
    _p: PhantomData<(D, U)>,
}

impl<T, R, D, U> DeserializeRequest<T, R> for FromRequestDeserializer<D, U>
where
    T: From<U>,
    D: DeserializeRequest<U, R>,
{
    fn deserialize(runtime: &ConjureRuntime, headers: &HeaderMap, body: R) -> Result<T, Error> {
        D::deserialize(runtime, headers, body).map(From::from)
    }
}

impl<T, R, D, U> AsyncDeserializeRequest<T, R> for FromRequestDeserializer<D, U>
where
    T: From<U>,
    D: AsyncDeserializeRequest<U, R>,
    R: Send,
{
    async fn deserialize(
        runtime: &ConjureRuntime,
        headers: &HeaderMap,
        body: R,
    ) -> Result<T, Error> {
        D::deserialize(runtime, headers, body).await.map(From::from)
    }
}

impl<T, R, D, U> LocalAsyncDeserializeRequest<T, R> for FromRequestDeserializer<D, U>
where
    T: From<U>,
    D: LocalAsyncDeserializeRequest<U, R>,
{
    async fn deserialize(
        runtime: &ConjureRuntime,
        headers: &HeaderMap,
        body: R,
    ) -> Result<T, Error> {
        D::deserialize(runtime, headers, body).await.map(From::from)
    }
}

/// A trait implemented by response serializers used by custom Conjure server trait implementations.
pub trait SerializeResponse<T, W> {
    /// Serializes the response.
    fn serialize(
        runtime: &ConjureRuntime,
        request_headers: &HeaderMap,
        value: T,
    ) -> Result<Response<ResponseBody<W>>, Error>;
}

/// A trait implemented by response serializers used by custom async Conjure server trait
/// implementations.
pub trait AsyncSerializeResponse<T, W> {
    /// Serializes the response.
    fn serialize(
        runtime: &ConjureRuntime,
        request_headers: &HeaderMap,
        value: T,
    ) -> Result<Response<AsyncResponseBody<W>>, Error>;
}

/// A trait implemented by response serializers used by custom local async Conjure server trait
/// implementations.
pub trait LocalAsyncSerializeResponse<T, W> {
    /// Serializes the response.
    fn serialize(
        runtime: &ConjureRuntime,
        request_headers: &HeaderMap,
        value: T,
    ) -> Result<Response<LocalAsyncResponseBody<W>>, Error>;
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
    fn serialize(
        _: &ConjureRuntime,
        _: &HeaderMap,
        _: (),
    ) -> Result<Response<ResponseBody<W>>, Error> {
        Self::serialize_inner(ResponseBody::Empty)
    }
}

impl<W> AsyncSerializeResponse<(), W> for EmptyResponseSerializer {
    fn serialize(
        _: &ConjureRuntime,
        _: &HeaderMap,
        _: (),
    ) -> Result<Response<AsyncResponseBody<W>>, Error> {
        Self::serialize_inner(AsyncResponseBody::Empty)
    }
}

impl<W> LocalAsyncSerializeResponse<(), W> for EmptyResponseSerializer {
    fn serialize(
        _: &ConjureRuntime,
        _: &HeaderMap,
        _: (),
    ) -> Result<Response<LocalAsyncResponseBody<W>>, Error> {
        Self::serialize_inner(LocalAsyncResponseBody::Empty)
    }
}

/// A body serializer for standard response types.
pub enum StdResponseSerializer {}

impl StdResponseSerializer {
    fn serialize_inner<B>(
        runtime: &ConjureRuntime,
        request_headers: &HeaderMap,
        value: &dyn erased_serde::Serialize,
        make_body: impl FnOnce(Bytes) -> B,
    ) -> Result<Response<B>, Error> {
        let encoding = runtime.response_body_encoding(request_headers)?;

        let mut body = vec![];
        value
            .erased_serialize(&mut *encoding.serializer(&mut body).serializer())
            .map_err(Error::internal)?;

        let mut response = Response::new(make_body(body.into()));
        response
            .headers_mut()
            .insert(CONTENT_TYPE, encoding.content_type());

        Ok(response)
    }
}

impl<T, W> SerializeResponse<T, W> for StdResponseSerializer
where
    T: Serialize,
{
    fn serialize(
        runtime: &ConjureRuntime,
        request_headers: &HeaderMap,
        value: T,
    ) -> Result<Response<ResponseBody<W>>, Error> {
        Self::serialize_inner(runtime, request_headers, &value, ResponseBody::Fixed)
    }
}

impl<T, W> AsyncSerializeResponse<T, W> for StdResponseSerializer
where
    T: Serialize,
{
    fn serialize(
        runtime: &ConjureRuntime,
        request_headers: &HeaderMap,
        value: T,
    ) -> Result<Response<AsyncResponseBody<W>>, Error> {
        Self::serialize_inner(runtime, request_headers, &value, AsyncResponseBody::Fixed)
    }
}

impl<T, W> LocalAsyncSerializeResponse<T, W> for StdResponseSerializer
where
    T: Serialize,
{
    fn serialize(
        runtime: &ConjureRuntime,
        request_headers: &HeaderMap,
        value: T,
    ) -> Result<Response<LocalAsyncResponseBody<W>>, Error> {
        Self::serialize_inner(
            runtime,
            request_headers,
            &value,
            LocalAsyncResponseBody::Fixed,
        )
    }
}

/// A trait implemented by header decoders used by custom Conjure server trait implementations.
pub trait DecodeHeader<T> {
    /// Decodes the value from headers.
    fn decode<'a, I>(runtime: &ConjureRuntime, headers: I) -> Result<T, Error>
    where
        I: IntoIterator<Item = &'a HeaderValue>;
}

/// A trait implemented by URL parameter decoders used by custom Conjure server trait
/// implementations.
pub trait DecodeParam<T> {
    /// Decodes the value from the sequence of values.
    ///
    /// The values have already been percent-decoded.
    fn decode<I>(runtime: &ConjureRuntime, params: I) -> Result<T, Error>
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
    fn decode<'a, I>(_: &ConjureRuntime, headers: I) -> Result<T, Error>
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
    fn decode<I>(_: &ConjureRuntime, params: I) -> Result<T, Error>
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
    fn decode<'a, I>(_: &ConjureRuntime, headers: I) -> Result<Option<T>, Error>
    where
        I: IntoIterator<Item = &'a HeaderValue>,
    {
        let Some(header) = optional_item(headers)? else {
            return Ok(None);
        };
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
    fn decode<I>(_: &ConjureRuntime, params: I) -> Result<Option<T>, Error>
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        let Some(param) = optional_item(params)? else {
            return Ok(None);
        };
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
    let Some(item) = it.next() else {
        return Ok(None);
    };

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
    fn decode<I>(_: &ConjureRuntime, params: I) -> Result<T, Error>
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
        return Err(
            Error::service_safe("expected exactly 1 parameter", InvalidArgument::new())
                .with_safe_param("actual", 0),
        );
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

/// A decoder which maps the output of another with [`From::from`].
pub struct FromDecoder<D, U> {
    _p: PhantomData<(D, U)>,
}

impl<T, D, U> DecodeParam<T> for FromDecoder<D, U>
where
    T: From<U>,
    D: DecodeParam<U>,
{
    fn decode<I>(runtime: &ConjureRuntime, params: I) -> Result<T, Error>
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        D::decode(runtime, params).map(T::from)
    }
}

impl<T, D, U> DecodeHeader<T> for FromDecoder<D, U>
where
    T: From<U>,
    D: DecodeHeader<U>,
{
    fn decode<'a, I>(runtime: &ConjureRuntime, headers: I) -> Result<T, Error>
    where
        I: IntoIterator<Item = &'a HeaderValue>,
    {
        D::decode(runtime, headers).map(T::from)
    }
}
