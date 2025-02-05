// Copyright 2024 Palantir Technologies, Inc.
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
//! Runtime configuration for Conjure servers.

use crate::server::Encoding;
use crate::server::JsonEncoding;
use crate::server::SmileEncoding;
use conjure_error::Error;
use conjure_error::InvalidArgument;
use http::header::ACCEPT;
use http::header::CONTENT_TYPE;
use http::HeaderMap;
use mediatype::names;
use mediatype::MediaType;
use mediatype::MediaTypeList;
use mediatype::ReadParams;

/// A type providing server logic that is configured at runtime.
pub struct ConjureRuntime {
    encodings: Vec<Box<dyn Encoding + Sync + Send>>,
}

impl ConjureRuntime {
    /// Creates a new runtime with default settings.
    pub fn new() -> Self {
        Self::builder().build()
    }

    /// Creates a new builder.
    pub fn builder() -> Builder {
        Builder { encodings: vec![] }
    }

    /// Returns the appropriate [`Encoding`] to deserialize the request body.
    ///
    /// The implementation currently compares the request's `Content-Type` header against [`Encoding::content_type`],
    /// ignoring parameters.
    pub fn request_body_encoding(
        &self,
        headers: &HeaderMap,
    ) -> Result<&(dyn Encoding + Sync + Send), Error> {
        let Some(content_type) = headers.get(CONTENT_TYPE) else {
            return Err(Error::service_safe(
                "Content-Type header missing from request",
                InvalidArgument::new(),
            ));
        };

        let content_type = content_type
            .to_str()
            .map_err(|e| Error::service_safe(e, InvalidArgument::new()))?;
        let content_type = MediaType::parse(content_type)
            .map_err(|e| Error::service_safe(e, InvalidArgument::new()))?;

        self.encodings
            .iter()
            .map(|e| &**e)
            .find(|e| mime_matches(&content_type, *e))
            .ok_or_else(|| {
                Error::service_safe(
                    "request Content-Type not accepted by any encoding",
                    InvalidArgument::new(),
                )
            })
    }

    /// Returns the appropriate [`Encoding`] to serialize the response body.
    ///
    /// The MIME types in the request's `Accept` header are processed in accordance with [RFC 9110]. If two MIME types
    /// have equal preference by that algorithm, the implementation will prefer the type declared earlier in the header.
    ///
    /// MIME types are matched against [`Encoding::content_type`], ignoring parameters. If multiple [`Encoding`]s are
    /// matched against the same MIME type, the encoding registered first by [`Builder::encoding`] will be selected.
    ///
    /// The implementation treats the absence of the `Accept` header equivalently to `Accept: */*`.
    ///
    /// [RFC 9110]: https://httpwg.org/specs/rfc9110.html#field.accept
    pub fn response_body_encoding(
        &self,
        headers: &HeaderMap,
    ) -> Result<&(dyn Encoding + Sync + Send), Error> {
        let mut types = headers
            .get_all(ACCEPT)
            .iter()
            .filter_map(|h| h.to_str().ok())
            .flat_map(|h| MediaTypeList::new(h).filter_map(Result::ok))
            .enumerate()
            .map(|(idx, type_)| {
                let quality = mime_quality(&type_);
                (type_, quality, idx)
            })
            .collect::<Vec<_>>();

        // If there is no Accept header the client will take anything
        if types.is_empty() {
            types.push((MediaType::new(names::_STAR, names::_STAR), 1000, 0));
        }

        // Sort types descending by specificity, then descending by quality, then ascending by index
        types.sort_by(|(a, a_quality, a_idx), (b, b_quality, b_idx)| {
            mime_specificity(a)
                .cmp(&mime_specificity(b))
                .reverse()
                .then_with(|| a_quality.cmp(b_quality).reverse())
                .then_with(|| a_idx.cmp(b_idx))
        });

        self.encodings
            .iter()
            // Reverse so the max_by picks the first listed encoding when the client has no preference
            .rev()
            // Filter to acceptable encodings and associate with the most preferred type's quality and index
            .filter_map(|encoding| {
                types
                    .iter()
                    .find(|(type_, _, _)| accepts(type_, &**encoding))
                    // A quality of 0 opts-out of that media type
                    .filter(|(_, quality, _)| *quality != 0)
                    .map(|(_, quality, idx)| (encoding, quality, idx))
            })
            // Pick the encoding matching the type with the best quality and then lowest index
            .max_by(|(_, a_quality, a_idx), (_, b_quality, b_idx)| {
                a_quality
                    .cmp(b_quality)
                    .then_with(|| a_idx.cmp(b_idx).reverse())
            })
            .map(|(encoding, _, _)| &**encoding)
            .ok_or_else(|| {
                Error::service_safe("request was not acceptable", InvalidArgument::new())
            })
    }
}

impl Default for ConjureRuntime {
    fn default() -> Self {
        Self::new()
    }
}

/// A builder for [`ConjureRuntime`].
pub struct Builder {
    encodings: Vec<Box<dyn Encoding + Sync + Send>>,
}

impl Builder {
    /// Registers an encoding for serializable request and response bodies.
    ///
    /// The runtime defaults to using [`JsonEncoding`] and [`SmileEncoding`] if none are explicitly registered.
    pub fn encoding(mut self, encoding: impl Encoding + 'static + Sync + Send) -> Self {
        self.encodings.push(Box::new(encoding));
        self
    }

    /// Builds the [`ConjureRuntime`].
    pub fn build(mut self) -> ConjureRuntime {
        if self.encodings.is_empty() {
            self = self.encoding(JsonEncoding).encoding(SmileEncoding);
        }

        ConjureRuntime {
            encodings: self.encodings,
        }
    }
}

/// Returns a type which will order the MIME type by specificity as specified by [RFC 9110].
///
/// > Media ranges can be overridden by more specific media ranges or specific media types. If more than one media range
/// > applies to a given type, the most specific reference has precedence. For example,
///
/// > Accept: text/*, text/plain, text/plain;format=flowed, */*
/// > have the following precedence:
///
/// > text/plain;format=flowed
/// > text/plain
/// > text/*
/// > */*
///
/// [RFC 9110]: https://httpwg.org/specs/rfc9110.html#field.accept
fn mime_specificity(mime: &MediaType<'_>) -> impl Ord {
    (
        mime.ty != names::_STAR,
        mime.subty != names::_STAR,
        mime.params.iter().filter(|(k, _)| *k != names::Q).count(),
    )
}

/// Extracts the "quality" of the MIME type, as specified by [RFC 9110].
///
/// We represent it as an integer in the range 0..=1000 instead of a float.
///
/// > The content negotiation fields defined by this specification use a common parameter, named "q" (case-insensitive),
/// > to assign a relative "weight" to the preference for that associated kind of content. This weight is referred to as
/// > a "quality value" (or "qvalue") because the same parameter name is often used within server configurations to
/// > assign a weight to the relative quality of the various representations that can be selected for a resource.
///
/// > The weight is normalized to a real number in the range 0 through 1, where 0.001 is the least preferred and 1 is
/// > the most preferred; a value of 0 means "not acceptable". If no "q" parameter is present, the default weight is 1.
///
/// >  weight = OWS ";" OWS "q=" qvalue
/// >  qvalue = ( "0" [ "." 0*3DIGIT ] )
/// >         / ( "1" [ "." 0*3("0") ] )
///
/// > A sender of qvalue MUST NOT generate more than three digits after the decimal point. User configuration of these
/// > values ought to be limited in the same fashion.
///
/// [RFC 9110]: https://httpwg.org/specs/rfc9110.html#quality.values
fn mime_quality(mime: &MediaType) -> u32 {
    mime_quality_inner(mime).unwrap_or(1000)
}

fn mime_quality_inner(mime: &MediaType) -> Option<u32> {
    let quality = mime.get_param(names::Q)?;

    let mut value = 0;
    let mut it = quality.as_str().chars();
    match it.next() {
        Some('1') => value = 1000,
        Some('0') => {}
        Some(_) | None => return None,
    }
    match it.next() {
        Some('.') => {}
        Some(_) => return None,
        None => return Some(value),
    }

    if it.as_str().len() > 3 {
        return None;
    }

    for (idx, ch) in it.enumerate() {
        value += ch.to_digit(10)? * (10u32.pow(2 - idx as u32))
    }

    Some(value)
}

fn mime_matches(target_mime: &MediaType, encoding: &dyn Encoding) -> bool {
    let encoding_type = encoding.content_type();
    let Some(encoding_mime) = encoding_type
        .to_str()
        .ok()
        .and_then(|t| MediaType::parse(t).ok())
    else {
        return false;
    };

    // We're ignoring parameters for now
    target_mime.essence() == encoding_mime.essence()
}

fn accepts(target_mime: &MediaType, encoding: &dyn Encoding) -> bool {
    let encoding_type = encoding.content_type();
    let Some(encoding_mime) = encoding_type
        .to_str()
        .ok()
        .and_then(|t| MediaType::parse(t).ok())
    else {
        return false;
    };

    if target_mime.essence() == MediaType::new(names::_STAR, names::_STAR) {
        return true;
    }

    if target_mime.ty == encoding_mime.ty && target_mime.subty == names::_STAR {
        return true;
    }

    // We're ignoring parameters for now
    target_mime.essence() == encoding_mime.essence()
}

#[cfg(test)]
mod test {
    use super::*;
    use http::HeaderValue;
    use mediatype::MediaTypeBuf;

    #[test]
    fn request_encodings() {
        let runtime = ConjureRuntime::builder()
            .encoding(JsonEncoding)
            .encoding(SmileEncoding)
            .build();

        let cases = [
            (Some("application/json"), Ok("application/json")),
            (
                Some("application/json; charset=UTF-8"),
                Ok("application/json"),
            ),
            (
                Some("application/x-jackson-smile"),
                Ok("application/x-jackson-smile"),
            ),
            (Some("text/plain"), Err(())),
            (Some("application/*"), Err(())),
            (Some("*/*"), Err(())),
            (None, Err(())),
        ];

        for (content_type, result) in cases {
            let mut headers = HeaderMap::new();
            if let Some(content_type) = content_type {
                headers.insert(CONTENT_TYPE, HeaderValue::from_str(content_type).unwrap());
            }

            match (result, runtime.request_body_encoding(&headers)) {
                (Ok(expected), Ok(encoder)) => assert_eq!(expected, encoder.content_type()),
                (Ok(expected), Err(e)) => panic!("expected Ok({expected}), got Err({e:?})"),
                (Err(()), Err(_)) => {}
                (Err(()), Ok(encoding)) => {
                    panic!("expected Err(), got Ok({:?})", encoding.content_type())
                }
            }
        }
    }

    #[test]
    fn response_encodings() {
        let runtime = ConjureRuntime::builder()
            .encoding(JsonEncoding)
            .encoding(SmileEncoding)
            .build();

        let cases = [
            (None, Ok("application/json")),
            (Some("*/*"), Ok("application/json")),
            (
                Some("*/*, application/json; q=0.5"),
                Ok("application/x-jackson-smile"),
            ),
            (
                Some("*/*, application/json; q=0"),
                Ok("application/x-jackson-smile"),
            ),
            (
                Some("application/json; encoding=UTF-8"),
                Ok("application/json"),
            ),
            (
                Some("application/x-jackson-smile"),
                Ok("application/x-jackson-smile"),
            ),
            (
                Some("text/plain, application/json, application/x-jackson-smile"),
                Ok("application/json"),
            ),
            (
                Some("text/plain, application/x-jackson-smile, application/json"),
                Ok("application/x-jackson-smile"),
            ),
            (
                Some("application/json; q=0.5, application/x-jackson-smile"),
                Ok("application/x-jackson-smile"),
            ),
            (
                Some("text/html, image/gif, image/jpeg, */*; q=0.2"),
                Ok("application/json"),
            ),
            (
                Some("text/html, image/gif, image/jpeg, application/*; q=0.2"),
                Ok("application/json"),
            ),
            (Some("text/plain"), Err(())),
            (Some("application/json; q=0, text/plain"), Err(())),
        ];

        for (accept, result) in cases {
            let mut headers = HeaderMap::new();
            if let Some(accept) = accept {
                headers.insert(ACCEPT, HeaderValue::from_str(accept).unwrap());
            }

            match (result, runtime.response_body_encoding(&headers)) {
                (Ok(expected), Ok(encoding)) => assert_eq!(expected, encoding.content_type()),
                (Ok(expected), Err(e)) => panic!("expected Ok({expected}), got Err({e:?})"),
                (Err(()), Err(_)) => {}
                (Err(()), Ok(encoding)) => {
                    panic!("expected Err(), got Ok({:?})", encoding.content_type())
                }
            }
        }
    }

    #[test]
    fn mime_quality() {
        let cases = [
            ("1", 1000),
            ("0", 0),
            ("1.", 1000),
            ("0.", 0),
            ("1.0", 1000),
            ("0.0", 0),
            ("1.00", 1000),
            ("0.00", 0),
            ("1.000", 1000),
            ("0.000", 0),
            ("0.2", 200),
            ("0.02", 20),
            ("0.002", 2),
        ];

        for (input, result) in cases {
            let mime = format!("foo/bar; q={input}")
                .parse::<MediaTypeBuf>()
                .unwrap();
            assert_eq!(result, super::mime_quality(&mime.to_ref()));
        }
    }
}
