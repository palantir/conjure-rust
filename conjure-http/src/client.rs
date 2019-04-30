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
use http::{HeaderMap, Response, Method};
use std::io::{Read, Write};

use crate::{PathParams, QueryParams};

/// A trait implemented by HTTP client implementations.
pub trait Client {
    /// The client's response body type.
    type ResponseBody: Read;

    /// Makes an HTTP request.
    ///
    /// The client is responsible for assembling the request URI. It is provided with the path template, unencoded path
    /// parameters, and unencoded query parameters. The request body is also unencoded, and the header map will not
    /// include a `Content-Length` header.
    ///
    /// A response must only be returned if it has a 2xx status code. The client is responsible for handling all other
    /// status codes (for example, converting a 5xx response into a service error). The client is also responsible for
    /// decoding the response body if necessary.
    fn request(
        &self,
        method: Method,
        path: &'static str,
        path_params: PathParams,
        query_params: QueryParams,
        headers: HeaderMap,
        body: Body<'_>,
    ) -> Result<Response<Self::ResponseBody>, Error>;
}

/// The body type used by a request.
pub enum Body<'a> {
    /// An empty body.
    Empty,
    /// A fixed-size body.
    Fixed(Vec<u8>),
    /// An indeterminate-size, streaming body.
    Streaming(&'a mut dyn WriteBody),
}

/// Convert a type into a `WriteBody` implementation.
pub trait IntoWriteBody {
    /// The `WriteBody` implementation for this type.
    type WriteBody: WriteBody;

    /// Converts this value into a `WriteBody` implementation.
    fn into_write_body(self) -> Self::WriteBody;
}

impl<'a> IntoWriteBody for &'a [u8] {
    type WriteBody = &'a [u8];

    #[inline]
    fn into_write_body(self) -> &'a [u8] {
        self
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
    /// Returns `true` if successful.
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
