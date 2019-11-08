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
use crate::{PathParams, QueryParams};
use async_trait::async_trait;
use conjure_error::{Error, InvalidArgument};
use http::{HeaderMap, Method};
use serde::{Deserializer, Serialize};
use std::error;
use std::future::Future;
use std::io::Write;
use std::pin::Pin;

/// A trait implemented by synchronous endpoint handlers.
pub trait Handler<T, B, R>
where
    B: RequestBody,
    R: VisitResponse,
{
    /// Handles a synchronous request.
    fn handle(
        &self,
        service: &T,
        path_params: &PathParams,
        query_params: &QueryParams,
        headers: &HeaderMap,
        body: B,
        response_visitor: R,
    ) -> Result<R::Output, Error>;
}

/// A trait implemented by asynchronous endpoint handlers.
pub trait AsyncHandler<T, B, R>
where
    T: Sync + Send,
    B: RequestBody + Send,
    B::BinaryBody: Send,
    R: AsyncVisitResponse + Send,
{
    /// Handles an asynchronous request.
    fn handle<'a>(
        &self,
        service: &'a T,
        path_params: &'a PathParams,
        query_params: &'a QueryParams,
        headers: &'a HeaderMap,
        body: B,
        response_visitor: R,
    ) -> Pin<Box<dyn Future<Output = Result<R::Output, Error>> + Send + 'a>>
    where
        T: 'a,
        B: 'a,
        R: 'a;
}

/// A parameter of an endpoint.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Parameter {
    name: &'static str,
    type_: ParameterType,
    safe: bool,
}

impl Parameter {
    /// Creates a new parameter.
    #[inline]
    pub const fn new(name: &'static str, type_: ParameterType) -> Parameter {
        Parameter {
            name,
            type_,
            safe: false,
        }
    }

    /// Sets the safety of the parameter.
    #[inline]
    pub const fn with_safe(mut self, safe: bool) -> Parameter {
        self.safe = safe;
        self
    }

    /// Returns the name of the parameter.
    #[inline]
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// Returns the type of the parameter.
    #[inline]
    pub fn type_(&self) -> ParameterType {
        self.type_
    }

    /// Returns true if the parameter is safe for logging.
    #[inline]
    pub fn safe(&self) -> bool {
        self.safe
    }
}

/// The specific type of a parameter.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ParameterType {
    /// A path parameter.
    Path(PathParameter),
    /// A query parameter.
    Query(QueryParameter),
    /// A header parameter.
    Header(HeaderParameter),
}

/// A path parameter.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct PathParameter(());

impl PathParameter {
    /// Creates a new path parameter.
    #[inline]
    pub const fn new() -> PathParameter {
        PathParameter(())
    }
}

/// A query parameter.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct QueryParameter {
    key: &'static str,
}

impl QueryParameter {
    /// Creates a new query parameter.
    #[inline]
    pub const fn new(key: &'static str) -> QueryParameter {
        QueryParameter { key }
    }

    /// Returns the key corresponding to this parameter in a URI's query.
    #[inline]
    pub fn key(&self) -> &'static str {
        self.key
    }
}

/// A header parameter.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct HeaderParameter {
    header: &'static str,
}

impl HeaderParameter {
    /// Creates a new header parameter.
    #[inline]
    pub const fn new(header: &'static str) -> HeaderParameter {
        HeaderParameter { header }
    }

    /// Returns the header corresponding to this parameter in an HTTP request.
    #[inline]
    pub fn header(&self) -> &'static str {
        self.header
    }
}

/// Information about an endpoint of a resource.
pub struct Metadata {
    name: &'static str,
    method: Method,
    path: &'static str,
    parameters: &'static [Parameter],
    deprecated: bool,
}

impl Metadata {
    /// Creates a new metadata object.
    #[inline]
    pub const fn new(
        name: &'static str,
        method: Method,
        path: &'static str,
        parameters: &'static [Parameter],
        deprecated: bool,
    ) -> Metadata {
        Metadata {
            name,
            method,
            path,
            parameters,
            deprecated,
        }
    }

    /// Returns the endpoint's name.
    #[inline]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    /// Returns the endpoint's HTTP method.
    #[inline]
    pub const fn method(&self) -> &Method {
        &self.method
    }

    /// Returns the endpoint's HTTP path template.
    #[inline]
    pub const fn path(&self) -> &'static str {
        self.path
    }

    /// Returns the endpoint's parameters.
    #[inline]
    pub const fn parameters(&self) -> &'static [Parameter] {
        self.parameters
    }

    /// Returns if the endpoint is deprecated.
    #[inline]
    pub const fn deprecated(&self) -> bool {
        self.deprecated
    }
}

/// A synchronous HTTP endpoint.
pub struct Endpoint<T, B, R>
where
    T: 'static,
    B: RequestBody + 'static,
    R: VisitResponse + 'static,
{
    /// Information about the endpoint.
    pub metadata: Metadata,
    /// The handler for the endpoint.
    pub handler: &'static (dyn Handler<T, B, R> + Sync + Send),
}

/// An asynchronous HTTP endpoint.
pub struct AsyncEndpoint<T, B, R>
where
    T: 'static,
    B: RequestBody + 'static,
    R: AsyncVisitResponse + 'static,
{
    /// Information about the endpoint.
    pub metadata: Metadata,
    /// The handler for the endpoint.
    pub handler: &'static (dyn AsyncHandler<T, B, R> + Sync + Send),
}

/// An HTTP resource.
///
/// The server-half of a Conjure service implements this trait.
pub trait Resource<I, O>: Sized {
    /// The resource's name.
    const NAME: &'static str;

    /// Returns the resource's HTTP endpoints.
    // FIXME ideally this would be a &'static [Endpoint] once const fns become more powerful
    fn endpoints<B, R>() -> Vec<Endpoint<Self, B, R>>
    where
        B: RequestBody<BinaryBody = I>,
        R: VisitResponse<BinaryWriter = O>;
}

/// An asynchronous HTTP resource.
///
/// The server-half of a Conjure service implements this trait.
pub trait AsyncResource<I, O>: Sized + Sync + Send {
    /// The resource's name.
    const NAME: &'static str;

    /// Returns the resource's HTTP endpoints.
    // FIXME ideally this would be a &'static [Endpoint] once const fns become more powerful
    fn endpoints<B, R>() -> Vec<AsyncEndpoint<Self, B, R>>
    where
        B: RequestBody<BinaryBody = I> + Send,
        B::BinaryBody: Send,
        R: AsyncVisitResponse<BinaryWriter = O> + Send;
}

/// An HTTP request body.
pub trait RequestBody {
    /// The binary body type.
    type BinaryBody;

    /// Accepts a visitor, calling the correct method corresponding to this body type.
    fn accept<V>(self, visitor: V) -> Result<V::Output, Error>
    where
        V: VisitRequestBody<Self::BinaryBody>;
}

/// A visitor over request body formats.
pub trait VisitRequestBody<T>: Sized {
    /// The output type returned by visit methods.
    type Output;

    /// Visits an empty body.
    ///
    /// The default implementation returns an error.
    fn visit_empty(self) -> Result<Self::Output, Error> {
        Err(Error::service_safe(
            "unexpected empty request body",
            InvalidArgument::new(),
        ))
    }

    /// Visits a serializable body.
    ///
    /// The default implementation returns an error.
    fn visit_serializable<'de, D>(self, deserializer: D) -> Result<Self::Output, Error>
    where
        D: Deserializer<'de>,
        D::Error: Into<Box<dyn error::Error + Sync + Send>>,
    {
        let _ = deserializer;
        Err(Error::service_safe(
            "unexpected serializable request body",
            InvalidArgument::new(),
        ))
    }

    /// Visits a streaming binary body.
    ///
    /// The default implementation returns an error.
    fn visit_binary(self, body: T) -> Result<Self::Output, Error> {
        let _ = body;
        Err(Error::service_safe(
            "unexpected binary request body",
            InvalidArgument::new(),
        ))
    }
}

/// An HTTP response.
pub trait Response<W> {
    /// Accepts a visitor, calling the correct method corresponding to the response type.
    fn accept<V>(self, visitor: V) -> Result<V::Output, Error>
    where
        V: VisitResponse<BinaryWriter = W>;
}

/// An asynchronous HTTP response.
pub trait AsyncResponse<W> {
    /// Accepts a visitor, calling the correct method corresponding to the response type.
    fn accept<V>(self, visitor: V) -> Result<V::Output, Error>
    where
        V: AsyncVisitResponse<BinaryWriter = W>;
}

/// A visitor over response body formats.
pub trait VisitResponse {
    /// The server's binary response body writer type.
    type BinaryWriter;

    /// The output type returned by visit methods.
    type Output;

    /// Visits an empty body.
    fn visit_empty(self) -> Result<Self::Output, Error>;

    /// Visits a serializable body.
    fn visit_serializable<T>(self, body: T) -> Result<Self::Output, Error>
    where
        T: Serialize + 'static;

    /// Visits a streaming binary body.
    fn visit_binary<T>(self, body: T) -> Result<Self::Output, Error>
    where
        T: WriteBody<Self::BinaryWriter> + 'static;
}

/// A visitor over asynchronous response body formats.
pub trait AsyncVisitResponse {
    /// The server's binary response body writer type.
    type BinaryWriter;

    /// The output type returned by visit methods.
    type Output;

    /// Visits an empty body.
    fn visit_empty(self) -> Result<Self::Output, Error>;

    /// Visits a serializable body.
    fn visit_serializable<T>(self, body: T) -> Result<Self::Output, Error>
    where
        T: Serialize + 'static + Send;

    /// Visits a streaming binary body.
    fn visit_binary<T>(self, body: T) -> Result<Self::Output, Error>
    where
        T: AsyncWriteBody<Self::BinaryWriter> + 'static + Send;
}

/// A trait implemented by streaming bodies.
pub trait WriteBody<W> {
    /// Writes the body out, in its entirety.
    fn write_body(self, w: &mut W) -> Result<(), Error>;
}

impl<W> WriteBody<W> for Vec<u8>
where
    W: Write,
{
    fn write_body(self, w: &mut W) -> Result<(), Error> {
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
    async fn write_body(self, w: Pin<&mut W>) -> Result<(), Error>;
}
