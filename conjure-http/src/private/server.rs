use crate::server::{
    AsyncDeserializeRequest, AsyncResponseBody, AsyncSerializeResponse, ConjureRuntime,
    DecodeHeader, DecodeParam, DeserializeRequest, LocalAsyncDeserializeRequest,
    LocalAsyncResponseBody, LocalAsyncSerializeResponse, ResponseBody, SerializeResponse,
};
use crate::PathParams;
use conjure_error::{Error, PermissionDenied};
use conjure_object::BearerToken;
use http::header::{HeaderName, AUTHORIZATION, COOKIE};
use http::{request, HeaderMap, Response};
use std::borrow::Cow;
use std::collections::HashMap;

pub const SERIALIZABLE_REQUEST_SIZE_LIMIT: usize = 50 * 1024 * 1024;

pub fn path_param<T, D>(
    runtime: &ConjureRuntime,
    parts: &request::Parts,
    param: &str,
    log_as: &str,
) -> Result<T, Error>
where
    D: DecodeParam<T>,
{
    let path_params = parts
        .extensions
        .get::<PathParams>()
        .expect("PathParams missing from request");
    let value = &path_params[param];
    let params = value
        .split('/')
        .map(percent_encoding::percent_decode_str)
        .map(|v| v.decode_utf8_lossy());
    D::decode(runtime, params).map_err(|e| e.with_safe_param("param", log_as))
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

pub fn query_param<T, D>(
    runtime: &ConjureRuntime,
    query_params: &HashMap<Cow<'_, str>, Vec<Cow<'_, str>>>,
    key: &str,
    log_as: &str,
) -> Result<T, Error>
where
    D: DecodeParam<T>,
{
    let values = query_params.get(key).into_iter().flatten();
    D::decode(runtime, values).map_err(|e| e.with_safe_param("param", log_as))
}

pub fn header_param<T, D>(
    runtime: &ConjureRuntime,
    parts: &request::Parts,
    header: &str,
    log_as: &str,
) -> Result<T, Error>
where
    D: DecodeHeader<T>,
{
    D::decode(runtime, parts.headers.get_all(header))
        .map_err(|e| e.with_safe_param("param", log_as))
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

pub fn body_arg<D, T, I>(
    runtime: &ConjureRuntime,
    headers: &HeaderMap,
    body: I,
    log_as: &str,
) -> Result<T, Error>
where
    D: DeserializeRequest<T, I>,
{
    D::deserialize(runtime, headers, body).map_err(|e| e.with_safe_param("param", log_as))
}

pub async fn async_body_arg<D, T, I>(
    runtime: &ConjureRuntime,
    headers: &HeaderMap,
    body: I,
    log_as: &str,
) -> Result<T, Error>
where
    D: AsyncDeserializeRequest<T, I>,
{
    D::deserialize(runtime, headers, body)
        .await
        .map_err(|e| e.with_safe_param("param", log_as))
}

pub async fn local_async_body_arg<D, T, I>(
    runtime: &ConjureRuntime,
    headers: &HeaderMap,
    body: I,
    log_as: &str,
) -> Result<T, Error>
where
    D: LocalAsyncDeserializeRequest<T, I>,
{
    D::deserialize(runtime, headers, body)
        .await
        .map_err(|e| e.with_safe_param("param", log_as))
}

pub fn response<S, T, W>(
    runtime: &ConjureRuntime,
    request_headers: &HeaderMap,
    value: T,
) -> Result<Response<ResponseBody<W>>, Error>
where
    S: SerializeResponse<T, W>,
{
    S::serialize(runtime, request_headers, value)
}

pub fn async_response<S, T, W>(
    runtime: &ConjureRuntime,
    request_headers: &HeaderMap,
    value: T,
) -> Result<Response<AsyncResponseBody<W>>, Error>
where
    S: AsyncSerializeResponse<T, W>,
{
    S::serialize(runtime, request_headers, value)
}

pub fn local_async_response<S, T, W>(
    runtime: &ConjureRuntime,
    request_headers: &HeaderMap,
    value: T,
) -> Result<Response<LocalAsyncResponseBody<W>>, Error>
where
    S: LocalAsyncSerializeResponse<T, W>,
{
    S::serialize(runtime, request_headers, value)
}
