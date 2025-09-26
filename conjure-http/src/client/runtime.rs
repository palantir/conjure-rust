// Copyright 2025 Palantir Technologies, Inc.
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
//! Runtime configuration for Conjure clients.

use crate::client::encoding::{JsonEncoding, SmileEncoding};
use crate::encoding::Encoding;
use conjure_error::Error;
use http::header::CONTENT_TYPE;
use http::{HeaderMap, HeaderValue};
use mediatype::MediaType;
use std::fmt;
use std::io::Write;

/// A type providing client logic that is configured at runtime.
#[derive(Debug)]
pub struct ConjureRuntime {
    request_encoding: DebugEncoding,
    accept_encodings: Vec<DebugEncoding>,
    accept: HeaderValue,
}

struct DebugEncoding(Box<dyn Encoding + Sync + Send>);

impl fmt::Debug for DebugEncoding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0.content_type(), f)
    }
}

impl ConjureRuntime {
    /// Creates a new runtime with default settings.
    pub fn new() -> Self {
        Self::builder().build()
    }

    /// Creates a new builder.
    pub fn builder() -> Builder {
        Builder {
            request_encoding: None,
            accept_encodings: vec![],
        }
    }

    /// Returns an `Accept` header value based on the configured accept encodings.
    pub fn accept(&self) -> HeaderValue {
        self.accept.clone()
    }

    /// Returns the configured request body [`Encoding`].
    pub fn request_body_encoding(&self) -> &(dyn Encoding + Sync + Send) {
        &*self.request_encoding.0
    }

    /// Returns the appropriate [`Encoding`] to deserialize the response body.
    ///
    /// The implementation currently compares the response's `Content-Type` header against [`Encoding::content_type`],
    /// ignoring parameters.
    pub fn response_body_encoding(
        &self,
        headers: &HeaderMap,
    ) -> Result<&(dyn Encoding + Sync + Send), Error> {
        let content_mime = headers
            .get(CONTENT_TYPE)
            .ok_or_else(|| Error::internal_safe("response missing Content-Type header"))
            .and_then(|h| h.to_str().map_err(Error::internal_safe))
            .and_then(|s| MediaType::parse(s).map_err(Error::internal_safe))?;

        for encoding in &self.accept_encodings {
            let encoding_type = encoding.0.content_type();
            let Some(encoding_mime) = encoding_type
                .to_str()
                .ok()
                .and_then(|s| MediaType::parse(s).ok())
            else {
                continue;
            };

            // We're ignoring parameters for now
            if content_mime.essence() == encoding_mime.essence() {
                return Ok(&*encoding.0);
            }
        }

        Err(
            Error::internal_safe("encoding not found for response body Content-Type")
                .with_safe_param("Content-Type", content_mime.to_string()),
        )
    }
}

impl Default for ConjureRuntime {
    fn default() -> Self {
        Self::new()
    }
}

/// A builder for [`ConjureRuntime`].
pub struct Builder {
    request_encoding: Option<Box<dyn Encoding + Sync + Send>>,
    accept_encodings: Vec<(Box<dyn Encoding + Sync + Send>, f32)>,
}

impl Builder {
    /// Sets the encoding for serializable request bodies.
    ///
    /// The runtime defaults to using [`JsonEncoding`].
    pub fn request_encoding(mut self, encoding: impl Encoding + 'static + Sync + Send) -> Self {
        self.request_encoding = Some(Box::new(encoding));
        self
    }

    /// Adds an encoding used for serializable response bodies with the specified weight.
    ///
    /// The runtime defaults to using [`SmileEncoding`] with weight 1 and [`JsonEncoding`] with weight 0.9 if none are
    /// explicitly registered.
    ///
    /// # Panics
    ///
    /// Panics if the weight is not between 0 and 1, inclusive.
    pub fn accept_encoding(
        mut self,
        encoding: impl Encoding + 'static + Sync + Send,
        weight: f32,
    ) -> Self {
        assert!(
            (0. ..=1.).contains(&weight),
            "weight must be between 0 and 1",
        );
        self.accept_encodings.push((Box::new(encoding), weight));
        self
    }

    /// Builds the [`ConjureRuntime`].
    pub fn build(self) -> ConjureRuntime {
        let request_encoding = DebugEncoding(
            self.request_encoding
                .unwrap_or_else(|| Box::new(JsonEncoding)),
        );

        let mut accept_encodings = if self.accept_encodings.is_empty() {
            vec![
                (Box::new(SmileEncoding) as _, 1.),
                (Box::new(JsonEncoding) as _, 0.9),
            ]
        } else {
            self.accept_encodings
        };

        // Sort descending by weight
        accept_encodings.sort_by(|a, b| a.1.total_cmp(&b.1).reverse());

        let mut accept = vec![];
        for (i, (encoding, weight)) in accept_encodings.iter().enumerate() {
            if i != 0 {
                accept.extend_from_slice(b", ");
            }

            accept.extend_from_slice(encoding.content_type().as_bytes());

            if *weight == 0. {
                accept.extend_from_slice(b"; q=0");
            } else if *weight != 1. {
                write!(accept, "; q={weight:.3}").unwrap();

                // `{weight:.3}` will always output 3 decimal digits, so pop off trailing 0s
                while accept.pop_if(|b| *b == b'0').is_some() {}
            }
        }

        let accept_encodings = accept_encodings
            .into_iter()
            .map(|(e, _)| DebugEncoding(e))
            .collect();
        let accept = HeaderValue::try_from(accept).unwrap();

        ConjureRuntime {
            request_encoding,
            accept_encodings,
            accept,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        let runtime = ConjureRuntime::new();

        assert_eq!(
            runtime.accept(),
            "application/x-jackson-smile, application/json; q=0.9"
        );

        let cases = [
            (None, Err(())),
            (Some("application/json"), Ok("application/json")),
            (
                Some("application/json; encoding=utf-8"),
                Ok("application/json"),
            ),
            (
                Some("application/x-jackson-smile"),
                Ok("application/x-jackson-smile"),
            ),
            (Some("application/cbor"), Err(())),
            (Some("application/*"), Err(())),
            (Some("*/*"), Err(())),
        ];

        for (content_type, result) in cases {
            let mut headers = HeaderMap::new();
            if let Some(content_type) = content_type {
                headers.insert(CONTENT_TYPE, HeaderValue::from_str(content_type).unwrap());
            }

            match (result, runtime.response_body_encoding(&headers)) {
                (Ok(expected), Ok(encoding)) => assert_eq!(encoding.content_type(), expected),
                (Ok(expected), Err(e)) => panic!("expected Ok({expected}), got Err({e:?})"),
                (Err(()), Err(_)) => {}
                (Err(()), Ok(encoding)) => {
                    panic!("expected Err(), got Ok({:?}", encoding.content_type())
                }
            }
        }
    }

    #[test]
    fn q_values() {
        let runtime = ConjureRuntime::builder()
            .accept_encoding(SmileEncoding, 0.)
            .accept_encoding(JsonEncoding, 1. / 3.)
            .build();

        assert_eq!(
            runtime.accept(),
            "application/json; q=0.333, application/x-jackson-smile; q=0"
        )
    }
}
