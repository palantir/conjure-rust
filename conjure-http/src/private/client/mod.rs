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
use crate::client::{AsyncBody, AsyncWriteBody, Body, WriteBody};
pub use crate::private::client::uri_builder::UriBuilder;
use crate::private::{async_read_body, read_body, APPLICATION_JSON, APPLICATION_OCTET_STREAM};
use bytes::Bytes;
use conjure_error::Error;
use conjure_object::{BearerToken, Plain, ToPlain};
use conjure_serde::json;
use futures_core::Stream;
use http::header::{
    HeaderName, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, COOKIE,
};
use http::{Request, Response, StatusCode};
use serde::de::{DeserializeOwned, IgnoredAny};
use serde::Serialize;
use std::pin::Pin;

mod uri_builder;

pub fn encode_empty_request<'a, W>() -> Request<Body<'a, W>>
where
    W: 'a,
{
    Request::new(Body::Empty)
}

pub fn async_encode_empty_request<'a, W>() -> Request<AsyncBody<'a, W>>
where
    W: 'a,
{
    Request::new(AsyncBody::Empty)
}

pub fn encode_serializable_request<T, S>(body: &T) -> Request<Body<S>>
where
    T: Serialize,
{
    inner_encode_serializable_request(body, Body::Fixed)
}

pub fn async_encode_serializable_request<T, S>(body: &T) -> Request<AsyncBody<S>>
where
    T: Serialize,
{
    inner_encode_serializable_request(body, AsyncBody::Fixed)
}

fn inner_encode_serializable_request<T, B, F>(body: &T, make_body: F) -> Request<B>
where
    T: Serialize,
    F: FnOnce(Bytes) -> B,
{
    let buf = json::to_vec(body).unwrap();
    let len = buf.len();

    let mut request = Request::new(make_body(Bytes::from(buf)));
    request
        .headers_mut()
        .insert(CONTENT_TYPE, APPLICATION_JSON.clone());
    request
        .headers_mut()
        .insert(CONTENT_LENGTH, HeaderValue::from(len));

    request
}

pub fn encode_binary_request<W>(body: &mut dyn WriteBody<W>) -> Request<Body<'_, W>> {
    inner_encode_binary_request(body, Body::Streaming)
}

pub fn async_encode_binary_request<W>(
    body: Pin<&mut (dyn AsyncWriteBody<W> + Send)>,
) -> Request<AsyncBody<'_, W>> {
    inner_encode_binary_request(body, AsyncBody::Streaming)
}

fn inner_encode_binary_request<W, B, F>(body: W, make_body: F) -> Request<B>
where
    F: FnOnce(W) -> B,
{
    let mut request = Request::new(make_body(body));
    request
        .headers_mut()
        .insert(CONTENT_TYPE, APPLICATION_OCTET_STREAM.clone());

    request
}

pub fn encode_empty_response_headers<B>(request: &mut Request<B>) {
    encode_serializable_response_headers(request);
}

pub fn encode_serializable_response_headers<B>(request: &mut Request<B>) {
    request
        .headers_mut()
        .insert(ACCEPT, APPLICATION_JSON.clone());
}

pub fn encode_binary_response_headers<B>(request: &mut Request<B>) {
    request
        .headers_mut()
        .insert(ACCEPT, APPLICATION_OCTET_STREAM.clone());
}

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

pub fn encode_optional_header<B, T>(
    request: &mut Request<B>,
    header: &'static str,
    value: &Option<T>,
) -> Result<(), Error>
where
    T: Plain,
{
    if let Some(value) = value {
        encode_header(request, header, value)?;
    }

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

// The logic here is unfortunately pretty much duplicated between blocking and async, but there isn't really a way to
// combine them :(.

pub fn decode_empty_response<I>(response: Response<I>) -> Result<(), Error>
where
    I: Iterator<Item = Result<Bytes, Error>>,
{
    if response.status() == StatusCode::NO_CONTENT {
        return Ok(());
    }

    // Servers can send a JSON response to an endpoint we expect to be void. Rather than just ignoring the response
    // body, we're going to "deserialize" it to IgnoredAny to validate that it is in fact a valid JSON body and to
    // consume the response body data so the socket can be reused for another request.
    decode_serializable_response::<IgnoredAny, _>(response)?;

    Ok(())
}

pub async fn async_decode_empty_response<I>(response: Response<I>) -> Result<(), Error>
where
    I: Stream<Item = Result<Bytes, Error>>,
{
    if response.status() == StatusCode::NO_CONTENT {
        return Ok(());
    }

    async_decode_serializable_response::<IgnoredAny, _>(response).await?;

    Ok(())
}

pub fn decode_default_serializable_response<T, I>(response: Response<I>) -> Result<T, Error>
where
    T: DeserializeOwned + Default,
    I: Iterator<Item = Result<Bytes, Error>>,
{
    if response.status() == StatusCode::NO_CONTENT {
        return Ok(T::default());
    }

    decode_serializable_response(response)
}

pub async fn async_decode_default_serializable_response<T, I>(
    response: Response<I>,
) -> Result<T, Error>
where
    T: DeserializeOwned + Default,
    I: Stream<Item = Result<Bytes, Error>>,
{
    if response.status() == StatusCode::NO_CONTENT {
        return Ok(T::default());
    }

    async_decode_serializable_response(response).await
}

pub fn decode_serializable_response<T, I>(response: Response<I>) -> Result<T, Error>
where
    T: DeserializeOwned,
    I: Iterator<Item = Result<Bytes, Error>>,
{
    if response.headers().get(CONTENT_TYPE) != Some(&APPLICATION_JSON) {
        return Err(Error::internal_safe("invalid response Content-Type"));
    }

    let body = read_body(response.into_body(), None)?;
    let body = json::client_from_slice(&body).map_err(Error::internal)?;

    Ok(body)
}

pub async fn async_decode_serializable_response<T, I>(response: Response<I>) -> Result<T, Error>
where
    T: DeserializeOwned,
    I: Stream<Item = Result<Bytes, Error>>,
{
    if response.headers().get(CONTENT_TYPE) != Some(&APPLICATION_JSON) {
        return Err(Error::internal("invalid response Content-Type"));
    }

    let body = async_read_body(response.into_body(), None).await?;
    let body = json::client_from_slice(&body).map_err(Error::internal)?;

    Ok(body)
}

pub fn decode_optional_binary_response<I>(response: Response<I>) -> Result<Option<I>, Error> {
    if response.status() == StatusCode::NO_CONTENT {
        return Ok(None);
    }

    decode_binary_response(response).map(Some)
}

pub fn decode_binary_response<I>(response: Response<I>) -> Result<I, Error> {
    if response.headers().get(CONTENT_TYPE) != Some(&APPLICATION_OCTET_STREAM) {
        return Err(Error::internal_safe("invalid response Content-Type"));
    }

    Ok(response.into_body())
}
