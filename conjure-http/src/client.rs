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

pub trait SerializeRequest<'a, T, W> {
    fn content_type(value: &T) -> HeaderValue;

    fn content_length(value: &T) -> Option<u64> {
        let _value = value;
        None
    }

    fn serialize(value: T) -> RequestBody<'a, W>;
}

pub enum DefaultRequestSerializer {}

impl<'a, T, W> SerializeRequest<'a, T, W> for DefaultRequestSerializer
where
    T: Serialize,
{
    fn content_type(_: &T) -> HeaderValue {
        APPLICATION_JSON
    }

    fn serialize(value: T) -> RequestBody<'a, W> {
        let buf = json::to_vec(&value).unwrap();
        RequestBody::Fixed(Bytes::from(buf))
    }
}

pub trait DeserializeResponse<T, R> {
    fn accept() -> Option<HeaderValue>;

    fn deserialize(response: Response<R>) -> Result<T, Error>;
}

pub enum DefaultResponseDeserializer {}

impl<T, R> DeserializeResponse<T, R> for DefaultResponseDeserializer
where
    T: DeserializeOwned,
    R: Iterator<Item = Result<Bytes, Error>>,
{
    fn accept() -> Option<HeaderValue> {
        Some(APPLICATION_JSON)
    }

    fn deserialize(response: Response<R>) -> Result<T, Error> {
        private::decode_serializable_response(response)
    }
}

pub trait EncodeHeader<T> {
    fn encode(value: T) -> Result<Vec<HeaderValue>, Error>;
}

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

pub trait EncodeParam<T> {
    fn encode(value: T) -> Vec<String>;
}

pub enum DisplayParamEncoder {}

impl<T> EncodeParam<T> for DisplayParamEncoder
where
    T: Display,
{
    fn encode(value: T) -> Vec<String> {
        vec![value.to_string()]
    }
}

pub enum DisplaySeqParamEncoder {}

impl<T, U> EncodeParam<T> for DisplaySeqParamEncoder
where
    T: IntoIterator<Item = U>,
    U: Display,
{
    fn encode(value: T) -> Vec<String> {
        value.into_iter().map(|v| v.to_string()).collect()
    }
}
