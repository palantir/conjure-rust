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
use http::{Request, Response};
use std::io::{Cursor, Read, Write};

/// A trait implemented by HTTP client implementations.
pub trait Client {
    /// The client's response body type.
    type ResponseBody: Read;

    /// Makes an HTTP request.
    ///
    /// The request's URI will be absolute-form, and it is the responsibility of the client to add the authority and
    /// any extra context path required. The request body will be unencoded, and the request will not include a
    /// `Content-Length` header.
    ///
    /// A response must only be returned if it has a 2xx status code. The client is responsible for handling all other
    /// status codes (for example, converting a 5xx response into a service error). The client is also responsible for
    /// decoding the response body if necessary.
    fn request(&self, req: Request<Body>) -> Result<Response<Self::ResponseBody>, Error>;
}

/// The body type used by a request.
pub enum Body {
    /// An empty body.
    Empty,
    /// A fixed-size body.
    Fixed(Vec<u8>),
    /// An indeterminate-size, streaming body.
    Streaming(Box<dyn WriteBody>),
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

impl<T> WriteBody for Cursor<T>
where
    T: AsRef<[u8]>,
{
    fn write_body(&mut self, w: &mut dyn Write) -> Result<(), Error> {
        let buf = &self.get_ref().as_ref()[self.position() as usize..];
        w.write_all(buf).map_err(Error::internal_safe)?;
        self.set_position(self.get_ref().as_ref().len() as u64);
        Ok(())
    }

    fn reset(&mut self) -> bool {
        self.set_position(0);
        true
    }
}
