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
use async_trait::async_trait;
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
use std::io::Write;
use std::pin::Pin;

#[allow(missing_docs)]
#[deprecated(note = "renamed to RequestBody", since = "3.5.0")]
pub type Body<'a, T> = RequestBody<'a, T>;

#[allow(missing_docs)]
#[deprecated(note = "renamed to AsyncRequestBody", since = "3.5.0")]
pub type AsyncBody<'a, T> = AsyncRequestBody<'a, T>;

/// A trait implemented by generated blocking client interfaces for a Conjure service.
pub trait Service<C> {
    /// Creates a new service wrapping an HTTP client.
    fn new(client: C) -> Self;
}

/// A trait implemented by generated async client interfaces for a Conjure service.
pub trait AsyncService<C> {
    /// Creates a new service wrapping an async HTTP client.
    fn new(client: C) -> Self;
}

/// Conjure-specific metadata about an endpoint.
///
/// This is included as an extension in all `Request`s passed to blocking and async Conjure clients.
#[derive(Clone)]
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
    Streaming(&'a mut dyn WriteBody<W>),
}

/// The body of an async Conjure request.
pub enum AsyncRequestBody<'a, W> {
    /// No body.
    Empty,
    /// A body already buffered in memory.
    Fixed(Bytes),
    /// A streaming body.
    Streaming(Pin<&'a mut (dyn AsyncWriteBody<W> + Send)>),
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
///
/// This trait can most easily be implemented with the [async-trait crate](https://docs.rs/async-trait).
#[async_trait]
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
    async fn send(
        &self,
        req: Request<AsyncRequestBody<'_, Self::BodyWriter>>,
    ) -> Result<Response<Self::ResponseBody>, Error>;
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
/// This trait can most easily be implemented with the [async-trait crate](https://docs.rs/async-trait).
///
/// # Examples
///
/// ```ignore
/// use async_trait::async_trait;
/// use conjure_error::Error;
/// use conjure_http::client::AsyncWriteBody;
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
///     async fn write_body(self: Pin<&mut Self>, mut w: Pin<&mut W>) -> Result<(), Error> {
///         w.write_all(b"hello world").await.map_err(Error::internal_safe)
///     }
///
///     async fn reset(self: Pin<&mut Self>) -> bool
///     where
///         W: 'async_trait,
///     {
///         true
///     }
/// }
/// ```
#[async_trait]
pub trait AsyncWriteBody<W> {
    /// Writes the body out, in its entirety.
    ///
    /// Behavior is unspecified if this method is called twice without a successful call to `reset` in between.
    async fn write_body(self: Pin<&mut Self>, w: Pin<&mut W>) -> Result<(), Error>;

    /// Attempts to reset the body so that it can be written out again.
    ///
    /// Returns `true` if successful. Behavior is unspecified if this is not called after a call to `write_body`.
    async fn reset(self: Pin<&mut Self>) -> bool
    where
        W: 'async_trait;
}

/// A trait implemented by request body serializers used by custom Conjure client trait
/// implementations.
pub trait SerializeRequest<'a, T, W> {
    /// Returns the body's content type.
    fn content_type(value: &T) -> HeaderValue;

    /// Returns the body's length, if known.
    ///
    /// Empty and fixed size bodies will have their content length filled in automatically.
    ///
    /// The default implementation returns `None`.
    fn content_length(value: &T) -> Option<u64> {
        let _value = value;
        None
    }

    /// Serializes the body.
    fn serialize(value: T) -> Result<RequestBody<'a, W>, Error>;
}

/// A trait implemented by request body serializers used by custom async Conjure client trait
/// implementations.
pub trait AsyncSerializeRequest<'a, T, W> {
    /// Returns the body's content type.
    fn content_type(value: &T) -> HeaderValue;

    /// Returns the body's length, if known.
    ///
    /// Empty and fixed size bodies will have their content length filled in automatically.
    ///
    /// The default implementation returns `None`.
    fn content_length(value: &T) -> Option<u64> {
        let _value = value;
        None
    }

    /// Serializes the body.
    fn serialize(value: T) -> Result<AsyncRequestBody<'a, W>, Error>;
}

/// A body serializer which acts like a Conjure-generated client would.
pub enum ConjureRequestSerializer {}

impl<'a, T, W> SerializeRequest<'a, T, W> for ConjureRequestSerializer
where
    T: Serialize,
{
    fn content_type(_: &T) -> HeaderValue {
        APPLICATION_JSON
    }

    fn serialize(value: T) -> Result<RequestBody<'a, W>, Error> {
        let body = json::to_vec(&value).map_err(Error::internal)?;
        Ok(RequestBody::Fixed(body.into()))
    }
}

impl<'a, T, W> AsyncSerializeRequest<'a, T, W> for ConjureRequestSerializer
where
    T: Serialize,
{
    fn content_type(_: &T) -> HeaderValue {
        APPLICATION_JSON
    }

    fn serialize(value: T) -> Result<AsyncRequestBody<'a, W>, Error> {
        let buf = json::to_vec(&value).map_err(Error::internal)?;
        Ok(AsyncRequestBody::Fixed(Bytes::from(buf)))
    }
}

/// A trait implemented by response deserializers used by custom Conjure client trait
/// implementations.
pub trait DeserializeResponse<T, R> {
    /// Returns the value of the `Accept` header to be included in the request.
    fn accept() -> Option<HeaderValue>;

    /// Deserializes the response.
    fn deserialize(response: Response<R>) -> Result<T, Error>;
}

/// A trait implemented by response deserializers used by custom async Conjure client trait
/// implementations.
#[async_trait]
pub trait AsyncDeserializeResponse<T, R> {
    /// Returns the value of the `Accept` header to be included in the request.
    fn accept() -> Option<HeaderValue>;

    /// Deserializes the response.
    async fn deserialize(response: Response<R>) -> Result<T, Error>;
}

/// A response deserializer which acts like a Conjure-generated client would.
pub enum ConjureResponseDeserializer {}

impl<T, R> DeserializeResponse<T, R> for ConjureResponseDeserializer
where
    T: DeserializeOwned,
    R: Iterator<Item = Result<Bytes, Error>>,
{
    fn accept() -> Option<HeaderValue> {
        Some(APPLICATION_JSON)
    }

    fn deserialize(response: Response<R>) -> Result<T, Error> {
        if response.headers().get(CONTENT_TYPE) != Some(&APPLICATION_JSON) {
            return Err(Error::internal_safe("invalid response Content-Type"));
        }
        let buf = private::read_body(response.into_body(), None)?;
        json::client_from_slice(&buf).map_err(Error::internal)
    }
}

#[async_trait]
impl<T, R> AsyncDeserializeResponse<T, R> for ConjureResponseDeserializer
where
    T: DeserializeOwned,
    R: Stream<Item = Result<Bytes, Error>> + 'static + Send,
{
    fn accept() -> Option<HeaderValue> {
        Some(APPLICATION_JSON)
    }

    async fn deserialize(response: Response<R>) -> Result<T, Error> {
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
    fn encode(value: T) -> Result<Vec<HeaderValue>, Error>;
}

/// A header encoder which converts values via their `Display` implementation.
pub enum DisplayHeaderEncoder {}

impl<T> EncodeHeader<T> for DisplayHeaderEncoder
where
    T: Display,
{
    fn encode(value: T) -> Result<Vec<HeaderValue>, Error> {
        HeaderValue::try_from(value.to_string())
            .map_err(Error::internal_safe)
            .map(|v| vec![v])
    }
}

/// A header encoder which converts a sequence of values via their individual `Display`
/// implementations.
pub enum DisplaySeqHeaderEncoder {}

impl<T, U> EncodeHeader<T> for DisplaySeqHeaderEncoder
where
    T: IntoIterator<Item = U>,
    U: Display,
{
    fn encode(value: T) -> Result<Vec<HeaderValue>, Error> {
        value
            .into_iter()
            .map(|v| HeaderValue::try_from(v.to_string()).map_err(Error::internal_safe))
            .collect()
    }
}

/// A trait implemented by URL parameter encoders used by custom Conjure client trait
/// implementations.
pub trait EncodeParam<T> {
    /// Encodes the value into a sequence of parameters.
    ///
    /// When used with a path parameter, each returned string will be a separate path component.
    /// When used with a query parameter, each returned string will be the value of a separate query
    /// entry.
    fn encode(value: T) -> Result<Vec<String>, Error>;
}

/// A param encoder which converts values via their `Display` implementations.
pub enum DisplayParamEncoder {}

impl<T> EncodeParam<T> for DisplayParamEncoder
where
    T: Display,
{
    fn encode(value: T) -> Result<Vec<String>, Error> {
        Ok(vec![value.to_string()])
    }
}

/// A param encoder which converts a sequence of values via their individual `Display`
/// implementations.
pub enum DisplaySeqParamEncoder {}

impl<T, U> EncodeParam<T> for DisplaySeqParamEncoder
where
    T: IntoIterator<Item = U>,
    U: Display,
{
    fn encode(value: T) -> Result<Vec<String>, Error> {
        Ok(value.into_iter().map(|v| v.to_string()).collect())
    }
}
