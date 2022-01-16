use crate::private::{async_read_body, read_body, APPLICATION_JSON, APPLICATION_OCTET_STREAM};
use crate::server::{AsyncResponseBody, AsyncWriteBody, ResponseBody, WriteBody};
use crate::PathParams;
use bytes::Bytes;
use conjure_error::{Error, InvalidArgument, PermissionDenied};
use conjure_object::{BearerToken, FromPlain};
use conjure_serde::json;
use futures_core::Stream;
use http::header::{HeaderName, HeaderValue, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, COOKIE};
use http::request;
use http::{Response, StatusCode};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::borrow::Cow;
use std::collections::{BTreeSet, HashMap};
use std::error;

const SERIALIZABLE_REQUEST_SIZE_LIMIT: usize = 50 * 1024 * 1024;

pub fn parse_path_param<T>(parts: &request::Parts, param: &str) -> Result<T, Error>
where
    T: FromPlain,
    T::Err: Into<Box<dyn error::Error + Sync + Send>>,
{
    let path_params = parts
        .extensions
        .get::<PathParams>()
        .expect("PathParams missing from request");
    from_plain(&path_params[param], param)
}

fn from_plain<T>(s: &str, param: &str) -> Result<T, Error>
where
    T: FromPlain,
    T::Err: Into<Box<dyn error::Error + Sync + Send>>,
{
    T::from_plain(s)
        .map_err(|e| Error::service_safe(e, InvalidArgument::new()).with_safe_param("param", param))
}

pub fn parse_query_params(parts: &request::Parts) -> HashMap<Cow<'_, str>, Vec<Cow<'_, str>>> {
    let query = match parts.uri.query() {
        Some(query) => query,
        None => return HashMap::new(),
    };

    let mut map = HashMap::new();
    for (key, value) in form_urlencoded::parse(query.as_bytes()) {
        map.entry(key).or_insert_with(Vec::new).push(value);
    }

    map
}

pub fn parse_query_param<T>(
    query_params: &HashMap<Cow<'_, str>, Vec<Cow<'_, str>>>,
    param: &str,
    param_id: &str,
) -> Result<T, Error>
where
    T: FromPlain,
    T::Err: Into<Box<dyn error::Error + Sync + Send>>,
{
    let values = &query_params[param_id];
    if values.len() != 1 {
        return Err(Error::service_safe(
            "expected exactly 1 query parameter",
            InvalidArgument::new(),
        )
        .with_safe_param("actual", values.len())
        .with_safe_param("param", param));
    }

    from_plain(&values[0], param)
}

pub fn parse_optional_query_param<T>(
    query_params: &HashMap<Cow<'_, str>, Vec<Cow<'_, str>>>,
    param: &str,
    param_id: &str,
    value: &mut Option<T>,
) -> Result<(), Error>
where
    T: FromPlain,
    T::Err: Into<Box<dyn error::Error + Sync + Send>>,
{
    let values = match query_params.get(param_id) {
        Some(values) => values,
        None => return Ok(()),
    };

    if values.len() != 1 {
        return Err(Error::service_safe(
            "expected exactly 1 query parameter",
            InvalidArgument::new(),
        )
        .with_safe_param("actual", values.len())
        .with_safe_param("param", param));
    }

    let parsed = from_plain(&values[0], param)?;
    *value = Some(parsed);

    Ok(())
}

pub fn parse_list_query_param<T>(
    query_params: &HashMap<Cow<'_, str>, Vec<Cow<'_, str>>>,
    param: &str,
    param_id: &str,
    value: &mut Vec<T>,
) -> Result<(), Error>
where
    T: FromPlain,
    T::Err: Into<Box<dyn error::Error + Sync + Send>>,
{
    let values = match query_params.get(param_id) {
        Some(values) => values,
        None => return Ok(()),
    };

    for query_param in values {
        let parsed = from_plain(query_param, param)?;
        value.push(parsed);
    }

    Ok(())
}

pub fn parse_set_query_param<T>(
    query_params: &HashMap<Cow<'_, str>, Vec<Cow<'_, str>>>,
    param: &str,
    param_id: &str,
    value: &mut BTreeSet<T>,
) -> Result<(), Error>
where
    T: FromPlain + Ord,
    T::Err: Into<Box<dyn error::Error + Sync + Send>>,
{
    let values = match query_params.get(param_id) {
        Some(values) => values,
        None => return Ok(()),
    };

    for query_param in values {
        let parsed = from_plain(query_param, param)?;
        value.insert(parsed);
    }

    Ok(())
}

pub fn parse_required_header<T>(
    parts: &request::Parts,
    param: &str,
    param_id: &str,
) -> Result<T, Error>
where
    T: FromPlain,
    T::Err: Into<Box<dyn error::Error + Sync + Send>>,
{
    parts
        .headers
        .get(param_id)
        .ok_or_else(|| {
            Error::service_safe("required header parameter missing", InvalidArgument::new())
                .with_safe_param("param", param)
        })
        .and_then(|h| parse_header(h, param))
}

pub fn parse_optional_header<T>(
    parts: &request::Parts,
    param: &str,
    param_id: &str,
    value: &mut Option<T>,
) -> Result<(), Error>
where
    T: FromPlain,
    T::Err: Into<Box<dyn error::Error + Sync + Send>>,
{
    if let Some(header) = parts.headers.get(param_id) {
        let header = parse_header(header, param)?;
        *value = Some(header);
    }

    Ok(())
}

fn parse_header<T>(header: &HeaderValue, param: &str) -> Result<T, Error>
where
    T: FromPlain,
    T::Err: Into<Box<dyn error::Error + Sync + Send>>,
{
    header
        .to_str()
        .map_err(|e| Error::service_safe(e, InvalidArgument::new()).with_safe_param("param", param))
        .and_then(|h| from_plain(h, param))
}

pub fn parse_cookie_auth(parts: &request::Parts, prefix: &str) -> Result<BearerToken, Error> {
    parse_auth_inner(parts, prefix, COOKIE)
}

pub fn parse_header_auth(parts: &request::Parts) -> Result<BearerToken, Error> {
    parse_auth_inner(parts, "Bearer ", AUTHORIZATION)
}

fn parse_auth_inner(
    parts: &request::Parts,
    prefix: &str,
    header: HeaderName,
) -> Result<BearerToken, Error> {
    let header = match parts.headers.get(header) {
        Some(header) => header,
        None => {
            return Err(Error::service_safe(
                "required auth header missing",
                PermissionDenied::new(),
            ));
        }
    };

    let header = header
        .to_str()
        .map_err(|e| Error::service_safe(e, PermissionDenied::new()))?;

    let value = header.strip_prefix(prefix).ok_or_else(|| {
        Error::service_safe("invalid auth header format", PermissionDenied::new())
    })?;

    value
        .parse()
        .map_err(|e| Error::service_safe(e, PermissionDenied::new()))
}

pub fn decode_empty_request<I>(parts: &request::Parts, _body: I) -> Result<(), Error> {
    if parts.headers.contains_key(CONTENT_TYPE) {
        return Err(Error::service_safe(
            "unexpected Content-Type",
            InvalidArgument::new(),
        ));
    }

    Ok(())
}

pub fn decode_serializable_request<I, T>(parts: &request::Parts, body: I) -> Result<T, Error>
where
    I: Iterator<Item = Result<Bytes, Error>>,
    T: DeserializeOwned,
{
    check_deserializable_request_headers(parts)?;
    let body = read_body(body, Some(SERIALIZABLE_REQUEST_SIZE_LIMIT))?;

    json::server_from_slice(&body).map_err(|e| Error::service(e, InvalidArgument::new()))
}

pub async fn async_decode_serializable_request<I, T>(
    parts: &request::Parts,
    body: I,
) -> Result<T, Error>
where
    I: Stream<Item = Result<Bytes, Error>>,
    T: DeserializeOwned,
{
    check_deserializable_request_headers(parts)?;
    let body = async_read_body(body, Some(SERIALIZABLE_REQUEST_SIZE_LIMIT)).await?;

    json::server_from_slice(&body).map_err(|e| Error::service(e, InvalidArgument::new()))
}

fn check_deserializable_request_headers(parts: &request::Parts) -> Result<(), Error> {
    if parts.headers.get(CONTENT_TYPE) != Some(&APPLICATION_JSON) {
        return Err(Error::service_safe(
            "unexpected Content-Type",
            InvalidArgument::new(),
        ));
    }

    Ok(())
}

pub fn decode_optional_serializable_request<I, T>(
    parts: &request::Parts,
    body: I,
) -> Result<Option<T>, Error>
where
    I: Iterator<Item = Result<Bytes, Error>>,
    T: DeserializeOwned,
{
    if !parts.headers.contains_key(CONTENT_TYPE) {
        return Ok(None);
    }

    decode_serializable_request(parts, body).map(Some)
}

pub async fn async_decode_optional_serializable_request<I, T>(
    parts: &request::Parts,
    body: I,
) -> Result<Option<T>, Error>
where
    I: Stream<Item = Result<Bytes, Error>>,
    T: DeserializeOwned,
{
    if !parts.headers.contains_key(CONTENT_TYPE) {
        return Ok(None);
    }

    async_decode_serializable_request(parts, body)
        .await
        .map(Some)
}

pub fn decode_binary_request<I>(parts: &request::Parts, body: I) -> Result<I, Error> {
    if parts.headers.get(CONTENT_TYPE) != Some(&APPLICATION_OCTET_STREAM) {
        return Err(Error::service_safe(
            "unexpected Content-Type",
            InvalidArgument::new(),
        ));
    }

    Ok(body)
}

pub fn encode_empty_response<O>() -> Response<ResponseBody<O>> {
    inner_encode_empty_response(ResponseBody::Empty)
}

pub fn async_encode_empty_response<O>() -> Response<AsyncResponseBody<O>> {
    inner_encode_empty_response(AsyncResponseBody::Empty)
}

fn inner_encode_empty_response<B>(body: B) -> Response<B> {
    let mut response = Response::new(body);
    *response.status_mut() = StatusCode::NO_CONTENT;

    response
}

pub fn encode_serializable_response<T, O>(value: &T) -> Response<ResponseBody<O>>
where
    T: Serialize,
{
    inner_encode_serializable_response(value, ResponseBody::Fixed)
}

pub fn async_encode_serializable_response<T, O>(value: &T) -> Response<AsyncResponseBody<O>>
where
    T: Serialize,
{
    inner_encode_serializable_response(value, AsyncResponseBody::Fixed)
}

fn inner_encode_serializable_response<T, B, F>(value: &T, make_body: F) -> Response<B>
where
    T: Serialize,
    F: FnOnce(Bytes) -> B,
{
    let body = json::to_vec(value).expect("Conjure types can serialize to JSON");
    let len = body.len();

    let mut response = Response::new(make_body(Bytes::from(body)));
    response
        .headers_mut()
        .insert(CONTENT_TYPE, APPLICATION_JSON);
    response
        .headers_mut()
        .insert(CONTENT_LENGTH, HeaderValue::from(len));

    response
}

pub fn encode_default_serializable_response<T, O>(value: &T) -> Response<ResponseBody<O>>
where
    T: Serialize + Default + PartialEq,
{
    if value == &T::default() {
        encode_empty_response()
    } else {
        encode_serializable_response(value)
    }
}

pub fn async_encode_default_serializable_response<T, O>(value: &T) -> Response<AsyncResponseBody<O>>
where
    T: Serialize + Default + PartialEq,
{
    if value == &T::default() {
        async_encode_empty_response()
    } else {
        async_encode_serializable_response(value)
    }
}

pub fn encode_binary_response<T, O>(value: T) -> Response<ResponseBody<O>>
where
    T: WriteBody<O> + 'static,
{
    let mut response = Response::new(ResponseBody::Streaming(Box::new(value)));
    response
        .headers_mut()
        .insert(CONTENT_TYPE, APPLICATION_OCTET_STREAM);

    response
}

pub fn async_encode_binary_response<T, O>(value: T) -> Response<AsyncResponseBody<O>>
where
    T: AsyncWriteBody<O> + 'static + Send,
{
    let mut response = Response::new(AsyncResponseBody::Streaming(Box::new(value)));
    response
        .headers_mut()
        .insert(CONTENT_TYPE, APPLICATION_OCTET_STREAM);

    response
}

pub fn encode_optional_binary_response<T, O>(value: Option<T>) -> Response<ResponseBody<O>>
where
    T: WriteBody<O> + 'static,
{
    match value {
        Some(value) => encode_binary_response(value),
        None => encode_empty_response(),
    }
}

pub fn async_encode_optional_binary_response<T, O>(
    value: Option<T>,
) -> Response<AsyncResponseBody<O>>
where
    T: AsyncWriteBody<O> + 'static + Send,
{
    match value {
        Some(value) => async_encode_binary_response(value),
        None => async_encode_empty_response(),
    }
}
