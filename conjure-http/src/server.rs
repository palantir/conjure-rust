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
use async_trait::async_trait;
use bytes::Bytes;
use conjure_error::Error;
use http::{Method, Request, Response};
use std::borrow::Cow;
use std::io::Write;
use std::pin::Pin;

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

/// A blocking HTTP endpoint.
pub trait Endpoint<I, O>: EndpointMetadata {
    /// Handles a request to the endpoint.
    ///
    /// If the endpoint has path parameters, callers must include a [`PathParams`](crate::PathParams) extension in the
    /// request containing the extracted parameters from the URI. The implementation is reponsible for all other request
    /// handling, including parsing query parameters, header parameters, and the request body.
    ///
    /// The returned response may include a [`SafeParams`](crate::SafeParams) extension containing request parameters
    /// which are safe to log. If the response was due to an error, it must also include the [`Error`] itself as an
    /// extension.
    fn handle(&self, req: Request<I>) -> Response<ResponseBody<O>>;
}

/// A nonblocking HTTP endpoint.
#[async_trait]
pub trait AsyncEndpoint<I, O>: EndpointMetadata {
    /// Handles a request to the endpoint.
    ///
    /// Callers must include a [`PathParams`](crate::PathParams) extension in the request containing the extracted
    /// parameters from the URI. The implementation is reponsible for all other request handling, including parsing
    /// query parameters, header parameters, and the request body.
    ///
    /// The returned response may include a [`SafeParams`](crate::SafeParams) extension containing request parameters
    /// which are safe to log. If the response was due to an error, it must also include the [`Error`] itself as an
    /// extension.
    async fn handle(&self, req: Request<I>) -> Response<AsyncResponseBody<O>>
    where
        I: 'async_trait;
}

/// One segment of an endpoint URI template.
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
