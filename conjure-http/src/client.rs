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

use conjure_error::Error;
use http::{HeaderMap, Method, Response};
use serde::Serialize;
use std::io::{Read, Write};

use crate::{PathParams, QueryParams};

/// A trait implemented by HTTP client implementations.
pub trait Client {
    /// The client's response body type.
    type ResponseBody: Read;

    /// Makes an HTTP request.
    ///
    /// The client is responsible for assembling the request URI. It is provided with the path template, unencoded path
    /// parameters, unencoded query parameters, and the request body.
    ///
    /// A response must only be returned if it has a 2xx status code. The client is responsible for handling all other
    /// status codes (for example, converting a 5xx response into a service error). The client is also responsible for
    /// decoding the response body if necessary.
    fn request<T>(
        &self,
        method: Method,
        path: &'static str,
        path_params: PathParams,
        query_params: QueryParams,
        headers: HeaderMap,
        body: T,
    ) -> Result<Response<Self::ResponseBody>, Error>
    where
        T: RequestBody;
}

/// A trait implemented by request bodies.
pub trait RequestBody {
    /// Accepts a visitor, calling the correct method corresponding to this body type.
    fn accept<V>(self, visitor: V) -> Result<V::Output, Error>
    where
        V: VisitRequestBody;
}

/// A visitor over request body formats.
pub trait VisitRequestBody {
    /// The output type returned by visit methods.
    type Output;

    /// Visits an empty body.
    fn visit_empty(self) -> Result<Self::Output, Error>;

    /// Visits a serializable body.
    fn visit_serializable<T>(self, body: T) -> Result<Self::Output, Error>
    where
        T: Serialize;

    /// Visits a streaming, binary body.
    fn visit_binary<T>(self, body: T) -> Result<Self::Output, Error>
    where
        T: WriteBody;
}

/// An empty request body.
pub struct EmptyRequestBody;

impl RequestBody for EmptyRequestBody {
    fn accept<V>(self, visitor: V) -> Result<V::Output, Error>
    where
        V: VisitRequestBody,
    {
        visitor.visit_empty()
    }
}

/// A serializable request body.
pub struct SerializableRequestBody<T>(pub T);

impl<T> RequestBody for SerializableRequestBody<T>
where
    T: Serialize,
{
    fn accept<V>(self, visitor: V) -> Result<V::Output, Error>
    where
        V: VisitRequestBody,
    {
        visitor.visit_serializable(self.0)
    }
}

/// A streaming binary request body.
pub struct BinaryRequestBody<T>(pub T);

impl<T> RequestBody for BinaryRequestBody<T>
where
    T: WriteBody,
{
    fn accept<V>(self, visitor: V) -> Result<V::Output, Error>
    where
        V: VisitRequestBody,
    {
        visitor.visit_binary(self.0)
    }
}

/// A trait implemented by streaming bodies.
pub trait WriteBody {
    /// Writes the body out, in its entirety.
    ///
    /// Behavior is unspecified if this method is called twice without a successful call to `reset` in between.
    fn write_body(&mut self, w: &mut dyn Write) -> Result<(), Error>;

    /// Attempts to reset the body so that it can be written out again.
    ///
    /// Returns `true` if successful. Behavior is unspecified if this is not called after a call to `write_body`.
    fn reset(&mut self) -> bool;
}

impl WriteBody for &[u8] {
    fn write_body(&mut self, w: &mut dyn Write) -> Result<(), Error> {
        w.write_all(self).map_err(Error::internal_safe)
    }

    fn reset(&mut self) -> bool {
        true
    }
}
