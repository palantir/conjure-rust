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

//! The Conjure HTTP client API.

use crate::private::{self, APPLICATION_JSON};
use bytes::Bytes;
use conjure_error::Error;
use conjure_serde::json;
use futures_core::Stream;
use http::header::CONTENT_TYPE;
use http::{HeaderValue, Request, Response};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::convert::TryFrom;
use std::fmt::Display;
use std::future::Future;
use std::io::Write;
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::Arc;

pub mod conjure;

#[allow(missing_docs)]
#[deprecated(note = "renamed to RequestBody", since = "3.5.0")]
pub type Body<'a, T> = RequestBody<'a, T>;

#[allow(missing_docs)]
#[deprecated(note = "renamed to AsyncRequestBody", since = "3.5.0")]
pub type AsyncBody<'a, T> = AsyncRequestBody<'a, T>;

/// A trait implemented by generated blocking client interfaces for a Conjure service.
pub trait Service<C> {
    /// Creates a new service wrapping an HTTP client.
    fn new(client: C, runtime: &Arc<ConjureRuntime>) -> Self;
}

/// A trait implemented by generated async client interfaces for a Conjure service.
pub trait AsyncService<C> {
    /// Creates a new service wrapping an async HTTP client.
    fn new(client: C, runtime: &Arc<ConjureRuntime>) -> Self;
}

/// Conjure-specific metadata about an endpoint.
///
/// This is included as an extension in all `Request`s passed to blocking and async Conjure clients.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Endpoint {
    service: &'static str,
    version: Option<&'static str>,
    name: &'static str,
    path: &'static str,
}

impl Endpoint {
    /// Creates a new `Endpoint`.
    #[inline]
    pub fn new(
        service: &'static str,
        version: Option<&'static str>,
        name: &'static str,
        path: &'static str,
    ) -> Self {
        Endpoint {
            service,
            version,
            name,
            path,
        }
    }

    /// Returns the name of the service the endpoint is part of.
    #[inline]
    pub fn service(&self) -> &'static str {
        self.service
    }

    /// Returns the version of the Conjure definition defining the service, if known.
    #[inline]
    pub fn version(&self) -> Option<&'static str> {
        self.version
    }

    /// Returns the name of the endpoint.
    #[inline]
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// Returns the templated URI path of the endpoint.
    #[inline]
    pub fn path(&self) -> &'static str {
        self.path
    }
}

/// The body of a blocking Conjure request.
pub enum RequestBody<'a, W> {
    /// No body.
    Empty,
    /// A body already buffered in memory.
    Fixed(Bytes),
    /// A streaming body.
    Streaming(Box<dyn WriteBody<W> + 'a>),
}

/// The body of an async Conjure request.
pub enum AsyncRequestBody<'a, W> {
    /// No body.
    Empty,
    /// A body already buffered in memory.
    Fixed(Bytes),
    /// A streaming body.
    Streaming(BoxAsyncWriteBody<'a, W>),
}

/// A trait implemented by HTTP client implementations.
pub trait Client {
    /// The client's binary request write type.
    type BodyWriter;
    /// The client's binary response body type.
    type ResponseBody: Iterator<Item = Result<Bytes, Error>>;

    /// Makes an HTTP request.
    ///
    /// The request's URI will be in absolute-form and it will always contain an `Endpoint` object in its extensions.
    ///
    /// A response must only be returned if it has a 2xx status code. The client is responsible for handling all other
    /// status codes (for example, converting a 5xx response into a service error). The client is also responsible for
    /// decoding the response body if necessary.
    fn send(
        &self,
        req: Request<RequestBody<'_, Self::BodyWriter>>,
    ) -> Result<Response<Self::ResponseBody>, Error>;
}

/// A trait implemented by async HTTP client implementations.
pub trait AsyncClient {
    /// The client's binary request body write type.
    type BodyWriter;
    /// The client's binary response body type.
    type ResponseBody: Stream<Item = Result<Bytes, Error>>;

    /// Makes an HTTP request.
    ///
    /// The client is responsible for assembling the request URI. It is provided with the path template, unencoded path
    /// parameters, unencoded query parameters, header parameters, and request body.
    ///
    /// A response must only be returned if it has a 2xx status code. The client is responsible for handling all other
    /// status codes (for example, converting a 5xx response into a service error). The client is also responsible for
    /// decoding the response body if necessary.
    fn send(
        &self,
        req: Request<AsyncRequestBody<'_, Self::BodyWriter>>,
    ) -> impl Future<Output = Result<Response<Self::ResponseBody>, Error>> + Send;
}

/// A type providing server logic that is configured at runtime.
pub struct ConjureRuntime(());

impl ConjureRuntime {
    /// Creates a new runtime with default settings.
    pub fn new() -> Self {
        ConjureRuntime(())
    }
}

impl Default for ConjureRuntime {
    fn default() -> Self {
        Self::new()
    }
}

/// A trait implemented by streaming bodies.
pub trait WriteBody<W> {
    /// Writes the body out, in its entirety.
    ///
    /// Behavior is unspecified if this method is called twice without a successful call to `reset` in between.
    fn write_body(&mut self, w: &mut W) -> Result<(), Error>;

    /// Attempts to reset the body so that it can be written out again.
    ///
    /// Returns `true` if successful. Behavior is unspecified if this is not called after a call to `write_body`.
    fn reset(&mut self) -> bool;
}

impl<W> WriteBody<W> for &[u8]
where
    W: Write,
{
    fn write_body(&mut self, w: &mut W) -> Result<(), Error> {
        w.write_all(self).map_err(Error::internal_safe)
    }

    fn reset(&mut self) -> bool {
        true
    }
}

/// A trait implemented by async streaming bodies.
///
/// # Examples
///
/// ```ignore
/// use conjure_error::Error;
/// use conjure_http::client::AsyncWriteBody;
/// use std::pin::Pin;
/// use tokio_io::{AsyncWrite, AsyncWriteExt};
///
/// pub struct SimpleBodyWriter;
///
/// impl<W> AsyncWriteBody<W> for SimpleBodyWriter
/// where
///     W: AsyncWrite + Send,
/// {
///     async fn write_body(self: Pin<&mut Self>, mut w: Pin<&mut W>) -> Result<(), Error> {
///         w.write_all(b"hello world").await.map_err(Error::internal_safe)
///     }
///
///     async fn reset(self: Pin<&mut Self>) -> bool {
///         true
///     }
/// }
/// ```
pub trait AsyncWriteBody<W> {
    /// Writes the body out, in its entirety.
    ///
    /// Behavior is unspecified if this method is called twice without a successful call to `reset` in between.
    fn write_body(
        self: Pin<&mut Self>,
        w: Pin<&mut W>,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Attempts to reset the body so that it can be written out again.
    ///
    /// Returns `true` if successful. Behavior is unspecified if this is not called after a call to `write_body`.
    fn reset(self: Pin<&mut Self>) -> impl Future<Output = bool> + Send;
}

// An internal object-safe version of AsyncWriteBody used to implement BoxAsyncWriteBody.
trait AsyncWriteBodyEraser<W> {
    fn write_body<'a>(
        self: Pin<&'a mut Self>,
        w: Pin<&'a mut W>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>;

    fn reset<'a>(self: Pin<&'a mut Self>) -> Pin<Box<dyn Future<Output = bool> + Send + 'a>>
    where
        W: 'a;
}

impl<T, W> AsyncWriteBodyEraser<W> for T
where
    T: AsyncWriteBody<W> + ?Sized,
{
    fn write_body<'a>(
        self: Pin<&'a mut Self>,
        w: Pin<&'a mut W>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>> {
        Box::pin(self.write_body(w))
    }

    fn reset<'a>(self: Pin<&'a mut Self>) -> Pin<Box<dyn Future<Output = bool> + Send + 'a>>
    where
        W: 'a,
    {
        Box::pin(self.reset())
    }
}

/// A boxed [`AsyncWriteBody`] trait object.
pub struct BoxAsyncWriteBody<'a, W> {
    inner: Pin<Box<dyn AsyncWriteBodyEraser<W> + Send + 'a>>,
}

impl<'a, W> BoxAsyncWriteBody<'a, W> {
    /// Creates a new `BoxAsyncWriteBody`.
    pub fn new<T>(v: T) -> Self
    where
        T: AsyncWriteBody<W> + Send + 'a,
    {
        BoxAsyncWriteBody { inner: Box::pin(v) }
    }
}

impl<W> AsyncWriteBody<W> for BoxAsyncWriteBody<'_, W>
where
    W: Send,
{
    async fn write_body(mut self: Pin<&mut Self>, w: Pin<&mut W>) -> Result<(), Error> {
        self.inner.as_mut().write_body(w).await
    }

    async fn reset(mut self: Pin<&mut Self>) -> bool {
        self.inner.as_mut().reset().await
    }
}

/// A trait implemented by request body serializers used by custom Conjure client trait
/// implementations.
pub trait SerializeRequest<'a, T, W> {
    /// Returns the body's content type.
    fn content_type(runtime: &ConjureRuntime, value: &T) -> HeaderValue;

    /// Returns the body's length, if known.
    ///
    /// Empty and fixed size bodies will have their content length filled in automatically.
    ///
    /// The default implementation returns `None`.
    fn content_length(runtime: &ConjureRuntime, value: &T) -> Option<u64> {
        let _runtime = runtime;
        let _value = value;
        None
    }

    /// Serializes the body.
    fn serialize(runtime: &ConjureRuntime, value: T) -> Result<RequestBody<'a, W>, Error>;
}

/// A trait implemented by request body serializers used by custom async Conjure client trait
/// implementations.
pub trait AsyncSerializeRequest<'a, T, W> {
    /// Returns the body's content type.
    fn content_type(runtime: &ConjureRuntime, value: &T) -> HeaderValue;

    /// Returns the body's length, if known.
    ///
    /// Empty and fixed size bodies will have their content length filled in automatically.
    ///
    /// The default implementation returns `None`.
    fn content_length(runtime: &ConjureRuntime, value: &T) -> Option<u64> {
        let _runtime = runtime;
        let _value = value;
        None
    }

    /// Serializes the body.
    fn serialize(runtime: &ConjureRuntime, value: T) -> Result<AsyncRequestBody<'a, W>, Error>;
}

/// A body serializer for standard request types.
pub enum StdRequestSerializer {}

impl<'a, T, W> SerializeRequest<'a, T, W> for StdRequestSerializer
where
    T: Serialize,
{
    fn content_type(_: &ConjureRuntime, _: &T) -> HeaderValue {
        APPLICATION_JSON
    }

    fn serialize(_: &ConjureRuntime, value: T) -> Result<RequestBody<'a, W>, Error> {
        let body = json::to_vec(&value).map_err(Error::internal)?;
        Ok(RequestBody::Fixed(body.into()))
    }
}

impl<'a, T, W> AsyncSerializeRequest<'a, T, W> for StdRequestSerializer
where
    T: Serialize,
{
    fn content_type(_: &ConjureRuntime, _: &T) -> HeaderValue {
        APPLICATION_JSON
    }

    fn serialize(_: &ConjureRuntime, value: T) -> Result<AsyncRequestBody<'a, W>, Error> {
        let buf = json::to_vec(&value).map_err(Error::internal)?;
        Ok(AsyncRequestBody::Fixed(Bytes::from(buf)))
    }
}

/// A trait implemented by response deserializers used by custom Conjure client trait
/// implementations.
pub trait DeserializeResponse<T, R> {
    /// Returns the value of the `Accept` header to be included in the request.
    fn accept(runtime: &ConjureRuntime) -> Option<HeaderValue>;

    /// Deserializes the response.
    fn deserialize(runtime: &ConjureRuntime, response: Response<R>) -> Result<T, Error>;
}

/// A trait implemented by response deserializers used by custom async Conjure client trait
/// implementations.
pub trait AsyncDeserializeResponse<T, R> {
    /// Returns the value of the `Accept` header to be included in the request.
    fn accept(runtime: &ConjureRuntime) -> Option<HeaderValue>;

    /// Deserializes the response.
    fn deserialize(
        runtime: &ConjureRuntime,
        response: Response<R>,
    ) -> impl Future<Output = Result<T, Error>> + Send;
}

/// A response deserializer which ignores the response and returns `()`.
pub enum UnitResponseDeserializer {}

impl<R> DeserializeResponse<(), R> for UnitResponseDeserializer {
    fn accept(_: &ConjureRuntime) -> Option<HeaderValue> {
        None
    }

    fn deserialize(_: &ConjureRuntime, _: Response<R>) -> Result<(), Error> {
        Ok(())
    }
}

impl<R> AsyncDeserializeResponse<(), R> for UnitResponseDeserializer
where
    R: Send,
{
    fn accept(_: &ConjureRuntime) -> Option<HeaderValue> {
        None
    }

    async fn deserialize(_: &ConjureRuntime, _: Response<R>) -> Result<(), Error> {
        Ok(())
    }
}

/// A response deserializer for standard body types.
pub enum StdResponseDeserializer {}

impl<T, R> DeserializeResponse<T, R> for StdResponseDeserializer
where
    T: DeserializeOwned,
    R: Iterator<Item = Result<Bytes, Error>>,
{
    fn accept(_: &ConjureRuntime) -> Option<HeaderValue> {
        Some(APPLICATION_JSON)
    }

    fn deserialize(_: &ConjureRuntime, response: Response<R>) -> Result<T, Error> {
        if response.headers().get(CONTENT_TYPE) != Some(&APPLICATION_JSON) {
            return Err(Error::internal_safe("invalid response Content-Type"));
        }
        let buf = private::read_body(response.into_body(), None)?;
        json::client_from_slice(&buf).map_err(Error::internal)
    }
}

impl<T, R> AsyncDeserializeResponse<T, R> for StdResponseDeserializer
where
    T: DeserializeOwned,
    R: Stream<Item = Result<Bytes, Error>> + Send,
{
    fn accept(_: &ConjureRuntime) -> Option<HeaderValue> {
        Some(APPLICATION_JSON)
    }

    async fn deserialize(_: &ConjureRuntime, response: Response<R>) -> Result<T, Error> {
        if response.headers().get(CONTENT_TYPE) != Some(&APPLICATION_JSON) {
            return Err(Error::internal_safe("invalid response Content-Type"));
        }
        let buf = private::async_read_body(response.into_body(), None).await?;
        json::client_from_slice(&buf).map_err(Error::internal)
    }
}

/// A trait implemented by header encoders used by custom Conjure client trait implementations.
pub trait EncodeHeader<T> {
    /// Encodes the value into headers.
    ///
    /// In almost all cases a single `HeaderValue` should be returned.
    fn encode(runtime: &ConjureRuntime, value: T) -> Result<Vec<HeaderValue>, Error>;
}

/// A trait implemented by URL parameter encoders used by custom Conjure client trait
/// implementations.
pub trait EncodeParam<T> {
    /// Encodes the value into a sequence of parameters.
    ///
    /// When used with a path parameter, each returned string will be a separate path component.
    /// When used with a query parameter, each returned string will be the value of a separate query
    /// entry.
    fn encode(runtime: &ConjureRuntime, value: T) -> Result<Vec<String>, Error>;
}

/// An encoder which converts values via their `Display` implementation.
pub enum DisplayEncoder {}

impl<T> EncodeHeader<T> for DisplayEncoder
where
    T: Display,
{
    fn encode(_: &ConjureRuntime, value: T) -> Result<Vec<HeaderValue>, Error> {
        HeaderValue::try_from(value.to_string())
            .map_err(Error::internal_safe)
            .map(|v| vec![v])
    }
}

impl<T> EncodeParam<T> for DisplayEncoder
where
    T: Display,
{
    fn encode(_: &ConjureRuntime, value: T) -> Result<Vec<String>, Error> {
        Ok(vec![value.to_string()])
    }
}

/// An encoder which converts a sequence of values via their individual `Display`
/// implementations.
pub enum DisplaySeqEncoder {}

impl<T, U> EncodeHeader<T> for DisplaySeqEncoder
where
    T: IntoIterator<Item = U>,
    U: Display,
{
    fn encode(_: &ConjureRuntime, value: T) -> Result<Vec<HeaderValue>, Error> {
        value
            .into_iter()
            .map(|v| HeaderValue::try_from(v.to_string()).map_err(Error::internal_safe))
            .collect()
    }
}

impl<T, U> EncodeParam<T> for DisplaySeqEncoder
where
    T: IntoIterator<Item = U>,
    U: Display,
{
    fn encode(_: &ConjureRuntime, value: T) -> Result<Vec<String>, Error> {
        Ok(value.into_iter().map(|v| v.to_string()).collect())
    }
}

/// An encoder which delegates to another with [`AsRef::as_ref`].
pub struct AsRefEncoder<D, U> {
    _p: PhantomData<(D, U)>,
}

impl<T, D, U> EncodeHeader<T> for AsRefEncoder<D, U>
where
    T: AsRef<U>,
    for<'a> D: EncodeHeader<&'a U>,
{
    fn encode(runtime: &ConjureRuntime, value: T) -> Result<Vec<HeaderValue>, Error> {
        D::encode(runtime, value.as_ref())
    }
}

impl<T, D, U> EncodeParam<T> for AsRefEncoder<D, U>
where
    T: AsRef<U>,
    for<'a> D: EncodeParam<&'a U>,
{
    fn encode(runtime: &ConjureRuntime, value: T) -> Result<Vec<String>, Error> {
        D::encode(runtime, value.as_ref())
    }
}
