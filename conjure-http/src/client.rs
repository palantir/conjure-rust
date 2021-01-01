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

use crate::{PathParams, QueryParams};
use async_trait::async_trait;
use conjure_error::Error;
use http::{HeaderMap, Method};
use serde::{Deserializer, Serialize};
use std::error;
use std::future::Future;
use std::io::Write;
use std::pin::Pin;

/// A trait implemented by generated blocking client interfaces for a Conjure service.
pub trait Service<C> {
    /// The name of the service.
    const NAME: &'static str;

    /// The version of the Conjure definition defining the service, if known.
    const VERSION: Option<&'static str>;

    /// Creates a new service wrapping an HTTP client.
    fn new(client: C) -> Self;
}

/// A trait implemented by HTTP client implementations.
pub trait Client {
    /// The client's binary request body writer type.
    type BinaryWriter;
    /// The client's binary response body type.
    type BinaryBody;

    /// Makes an HTTP request.
    ///
    /// The client is responsible for assembling the request URI. It is provided with the path template, unencoded path
    /// parameters, unencoded query parameters, header parameters, and request body.
    ///
    /// A response must only be returned if it has a 2xx status code. The client is responsible for handling all other
    /// status codes (for example, converting a 5xx response into a service error). The client is also responsible for
    /// decoding the response body if necessary.
    #[allow(clippy::too_many_arguments)]
    fn request<'a, T, U>(
        &self,
        method: Method,
        path: &'static str,
        path_params: PathParams,
        query_params: QueryParams,
        headers: HeaderMap,
        body: T,
        response_visitor: U,
    ) -> Result<U::Output, Error>
    where
        T: RequestBody<'a, Self::BinaryWriter>,
        U: VisitResponse<Self::BinaryBody>;
}

/// A trait implemented by generated async client interfaces for a Conjure service.
pub trait AsyncService<C> {
    /// The name of the service.
    const NAME: &'static str;

    /// The version of the Conjure definition defining the service, if known.
    const VERSION: Option<&'static str>;

    /// Creates a new service wrapping an async HTTP client.
    fn new(client: C) -> Self;
}

/// A trait implemented by async HTTP client implementations.
pub trait AsyncClient {
    /// The client's binary request body writer type.
    type BinaryWriter;
    /// The client's binary response body type.
    type BinaryBody;

    /// Makes an async HTTP request.
    ///
    /// The client is responsible for assembling the request URI. It is provided with the path template, unencoded path
    /// parameters, unencoded query parameters, header parameters, and request body.
    ///
    /// A response must only be returned if it has a 2xx status code. The client is responsible for handling all other
    /// status codes (for example, converting a 5xx response into a service error). The client is also responsible for
    /// decoding the response body if necessary.
    #[allow(clippy::too_many_arguments)]
    fn request<'a, T, U>(
        &'a self,
        method: Method,
        path: &'static str,
        path_params: PathParams,
        query_params: QueryParams,
        headers: HeaderMap,
        body: T,
        response_visitor: U,
    ) -> Pin<Box<dyn Future<Output = Result<U::Output, Error>> + Send + 'a>>
    where
        T: AsyncRequestBody<'a, Self::BinaryWriter> + Send + 'a,
        U: VisitResponse<Self::BinaryBody> + Send + 'a;
}

/// A trait implemented by request bodies.
pub trait RequestBody<'a, W> {
    /// Accepts a visitor, calling the correct method corresponding to this body type.
    fn accept<V>(self, visitor: V) -> V::Output
    where
        V: VisitRequestBody<'a, W>;
}

/// A trait implemented by async request bodies.
pub trait AsyncRequestBody<'a, W> {
    /// Accepts a visitor, calling the correct method corresponding to this body type.
    fn accept<V>(self, visitor: V) -> V::Output
    where
        V: AsyncVisitRequestBody<'a, W>;
}

/// A visitor over request body formats.
pub trait VisitRequestBody<'a, W> {
    /// The output type returned by visit methods.
    type Output;

    /// Visits an empty body.
    fn visit_empty(self) -> Self::Output;

    /// Visits a serializable body.
    fn visit_serializable<T>(self, body: T) -> Self::Output
    where
        T: Serialize + 'a;

    /// Visits a streaming, binary body.
    fn visit_binary<T>(self, body: T) -> Self::Output
    where
        T: WriteBody<W> + 'a;
}

/// A visitor over async request body formats.
pub trait AsyncVisitRequestBody<'a, W> {
    /// The output type returned by visit methods.
    type Output;

    /// Visits an empty body.
    fn visit_empty(self) -> Self::Output;

    /// Visits a serializable body.
    fn visit_serializable<T>(self, body: T) -> Self::Output
    where
        T: Serialize + 'a;

    /// Visits a streaming, binary body.
    fn visit_binary<T>(self, body: T) -> Self::Output
    where
        T: AsyncWriteBody<W> + Sync + Send + 'a;
}

/// A visitor over HTTP responses.
pub trait VisitResponse<T>: Sized {
    /// The type produced by the visitor.
    type Output;

    /// Returns the type of response the visitor accepts.
    ///
    /// This is used to create the HTTP `Accept` header.
    fn accept(&self) -> Accept;

    /// Visits an empty response.
    fn visit_empty(self) -> Result<Self::Output, Error> {
        Err(Error::internal_safe("unexpected empty response"))
    }

    /// Visits a serializable response.
    fn visit_serializable<'de, D>(self, deserializer: D) -> Result<Self::Output, Error>
    where
        D: Deserializer<'de>,
        D::Error: Into<Box<dyn error::Error + Sync + Send>>,
    {
        let _ = deserializer;
        Err(Error::internal_safe("unexpected serializable response"))
    }

    /// Visits a streaming binary response.
    fn visit_binary(self, body: T) -> Result<Self::Output, Error> {
        let _ = body;
        Err(Error::internal_safe("unexpected binary response"))
    }
}

/// The type of response expected by a visitor.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Accept {
    /// An empty response.
    Empty,
    /// A serializable response.
    Serializable,
    /// A binary response.
    Binary,
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
