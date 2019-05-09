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
use conjure_error::{Error, InvalidArgument};
use http::{HeaderMap, Method};
use serde::{Deserializer, Serialize};
use std::error;
use std::io::Write;

use crate::{PathParams, QueryParams};

/// A type definition for the endpoint handler function pointer type.
pub type Handler<T, B, R, O> = fn(
    service: &T,
    path_params: &PathParams,
    query_params: &QueryParams,
    headers: &HeaderMap,
    body: B,
    response_visitor: R,
) -> Result<O, Error>;

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
pub struct Endpoint<T, B, R>
where
    R: VisitResponse,
{
    name: &'static str,
    method: Method,
    path: &'static str,
    handler: Handler<T, B, R, R::Output>,
    parameters: &'static [Parameter],
}

impl<T, B, R> Endpoint<T, B, R>
where
    R: VisitResponse,
{
    /// Creates a new endpoint.
    pub fn new(
        name: &'static str,
        method: Method,
        path: &'static str,
        handler: Handler<T, B, R, R::Output>,
        parameters: &'static [Parameter],
    ) -> Endpoint<T, B, R> {
        Endpoint {
            name,
            method,
            path,
            handler,
            parameters,
        }
    }

    /// Returns the endpoint's name.
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// Returns the endpoint's HTTP method.
    pub fn method(&self) -> &Method {
        &self.method
    }

    /// Returns the endpoint's HTTP path template.
    pub fn path(&self) -> &'static str {
        self.path
    }

    /// Returns the endpoint's handler function pointer.
    pub fn handler(&self) -> Handler<T, B, R, R::Output> {
        self.handler
    }

    /// Returns the endpoint's parameters.
    pub fn parameters(&self) -> &'static [Parameter] {
        self.parameters
    }
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
        B: RequestBody<Body = I>,
        R: VisitResponse<BinaryWriter = O>;
}

/// An HTTP request body.
pub trait RequestBody {
    /// The binary body type.
    type Body;

    /// Accepts a visitor, calling the correct method corresponding to this body type.
    fn accept<V>(self, visitor: V) -> Result<V::Output, Error>
    where
        V: VisitRequestBody<Self::Body>;
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
        D::Error: Into<Box<error::Error + Sync + Send>>,
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

/// A visitor over response body formats.
pub trait VisitResponse {
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
