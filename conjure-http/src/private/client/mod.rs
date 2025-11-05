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
pub use crate::private::client::uri_builder::UriBuilder;
use bytes::Bytes;
use conjure_error::Error;
use conjure_object::{BearerToken, Plain, ToPlain};
use http::header::{HeaderName, HeaderValue, AUTHORIZATION, COOKIE};
use http::Request;

mod uri_builder;

pub fn encode_header<B>(
    request: &mut Request<B>,
    header: &'static str,
    value: &dyn Plain,
) -> Result<(), Error> {
    let header = HeaderName::from_static(header);
    let value = HeaderValue::from_maybe_shared(Bytes::from(value.to_plain()))
        .map_err(Error::internal_safe)?;
    request.headers_mut().insert(header, value);

    Ok(())
}

pub fn encode_cookie_auth<B>(request: &mut Request<B>, prefix: &str, value: &BearerToken) {
    encode_auth(request, COOKIE, prefix, value)
}

pub fn encode_header_auth<B>(request: &mut Request<B>, value: &BearerToken) {
    encode_auth(request, AUTHORIZATION, "Bearer ", value);
}

fn encode_auth<B>(request: &mut Request<B>, header: HeaderName, prefix: &str, value: &BearerToken) {
    let value = format!("{}{}", prefix, value.as_str());
    let value = HeaderValue::from_maybe_shared(Bytes::from(value))
        .expect("bearer tokens are valid headers");
    request.headers_mut().insert(header, value);
}
