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
use crate::SafeParams;
use async_trait::async_trait;
use bytes::Bytes;
use conjure_error::Error;
use http::{Method, Request, Response};
use std::borrow::Cow;
use std::future::Future;
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
    /// If the endpoint has path parameters, callers must include a [`PathParams`](crate::PathParams) extension in the
    /// request containing the extracted parameters from the URI. The implementation is reponsible for all other request
    /// handling, including parsing query parameters, header parameters, and the request body.
    ///
    /// The [`SafeParams`] argument can be used to register which request parameters are safe to include as parameters
    /// in the request log.
    fn handle(
        &self,
        safe_params: &mut SafeParams,
        req: Request<I>,
    ) -> Result<Response<ResponseBody<O>>, Error>;
}

impl<T, I, O> Endpoint<I, O> for Box<T>
where
    T: ?Sized + Endpoint<I, O>,
{
    fn handle(
        &self,
        safe_params: &mut SafeParams,
        req: Request<I>,
    ) -> Result<Response<ResponseBody<O>>, Error> {
        (**self).handle(safe_params, req)
    }
}

/// A nonblocking HTTP endpoint.
#[async_trait]
pub trait AsyncEndpoint<I, O>: EndpointMetadata {
    /// Handles a request to the endpoint.
    ///
    /// If the endpoint has path parameters, callers must include a [`PathParams`](crate::PathParams) extension in the
    /// request containing the extracted parameters from the URI. The implementation is reponsible for all other request
    /// handling, including parsing query parameters, header parameters, and the request body.
    ///
    /// The [`SafeParams`] argument can be used to register which request parameters are safe to include as parameters
    /// in the request log.
    async fn handle(
        &self,
        safe_params: &mut SafeParams,
        req: Request<I>,
    ) -> Result<Response<AsyncResponseBody<O>>, Error>
    where
        I: 'async_trait;
}

impl<T, I, O> AsyncEndpoint<I, O> for Box<T>
where
    T: ?Sized + AsyncEndpoint<I, O>,
{
    fn handle<'life0, 'life1, 'async_trait>(
        &'life0 self,
        safe_params: &'life1 mut SafeParams,
        req: Request<I>,
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
        (**self).handle(safe_params, req)
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
