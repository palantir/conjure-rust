// Copyright 2021 Palantir Technologies, Inc.
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

//! A URI builder.

use bytes::BytesMut;
use conjure_object::{Plain, ToPlain};
use http::Uri;
use percent_encoding::{utf8_percent_encode, AsciiSet};

// https://url.spec.whatwg.org/#query-percent-encode-set
const QUERY: &AsciiSet = &percent_encoding::CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'#')
    .add(b'<')
    .add(b'>');

// https://url.spec.whatwg.org/#path-percent-encode-set
const PATH: &AsciiSet = &QUERY.add(b'?').add(b'`').add(b'{').add(b'}');

// https://url.spec.whatwg.org/#userinfo-percent-encode-set
const USERINFO: &AsciiSet = &PATH
    .add(b'/')
    .add(b':')
    .add(b';')
    .add(b'=')
    .add(b'@')
    .add(b'[')
    .add(b'\\')
    .add(b']')
    .add(b'^')
    .add(b'|');

// https://url.spec.whatwg.org/#component-percent-encode-set
const COMPONENT: &AsciiSet = &USERINFO.add(b'$').add(b'%').add(b'&').add(b'+').add(b',');

/// A builder for absolute-form URIs.
///
/// This is used by generated Conjure client code, and so makes assumptions about the validity of input to many of its
/// methods.
pub struct UriBuilder {
    buf: BytesMut,
    in_path: bool,
}

impl Default for UriBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl UriBuilder {
    /// Creates a new builder with an empty buffer.
    pub fn new() -> Self {
        UriBuilder {
            buf: BytesMut::new(),
            in_path: true,
        }
    }

    /// Adds a static part of the path directly to the buffer.
    ///
    /// The input is assumed to already be properly escaped; no processing is performed on it. It must be a complete
    /// sequence of path components, beginning with a `/`, but not ending with `/`. It must not be called after any
    /// query parameters have been pushed.
    pub fn push_literal(&mut self, components: &str) {
        debug_assert!(components.starts_with('/'));
        debug_assert!(!components.ends_with('/'));
        debug_assert!(self.in_path);

        self.buf.extend_from_slice(components.as_bytes());
    }

    /// Appends a dynamic path component to the buffer.
    ///
    /// The parameter is prefixed by `/`, and will be percent-escaped. It must not be called after any query parameters
    /// have been pushed.
    pub fn push_path_parameter(&mut self, parameter: &dyn Plain) {
        debug_assert!(self.in_path);

        self.buf.extend_from_slice(b"/");
        self.push_escaped(parameter);
    }

    /// Appends a query parameter to the buffer.
    ///
    /// The key is assumed to already be properly escaped; no processing is performed on it. The value will be
    /// percent-encoded. This must be called after the path has been pushed.
    pub fn push_query_parameter(&mut self, key: &str, value: &dyn Plain) {
        let prefix = if self.in_path { b"?" } else { b"&" };
        self.in_path = false;

        self.buf.extend_from_slice(prefix);
        self.buf.extend_from_slice(key.as_bytes());
        self.buf.extend_from_slice(b"=");
        self.push_escaped(value);
    }

    fn push_escaped(&mut self, value: &dyn Plain) {
        let value = value.to_plain();
        for chunk in utf8_percent_encode(&value, COMPONENT) {
            self.buf.extend_from_slice(chunk.as_bytes());
        }
    }

    /// Consumes the builder, returning a URI.
    pub fn build(self) -> Uri {
        debug_assert!(!self.buf.is_empty());

        Uri::from_maybe_shared(self.buf.freeze()).unwrap()
    }
}
