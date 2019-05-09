use conjure_error::{Error, InvalidArgument, PermissionDenied};
use conjure_object::{BearerToken, FromPlain};
use http::header::{HeaderMap, HeaderName, HeaderValue, AUTHORIZATION, COOKIE};
use serde::de::DeserializeOwned;
use serde::{Deserializer, Serialize};
use std::collections::BTreeSet;
use std::error;
use std::marker::PhantomData;

use crate::server::{Response, VisitRequestBody, VisitResponse, WriteBody};
use crate::{PathParams, QueryParams};

pub fn parse_path_param<T>(path_params: &PathParams, param: &str) -> Result<T, Error>
where
    T: FromPlain,
    T::Err: Into<Box<error::Error + Sync + Send>>,
{
    from_plain(&path_params[param], param)
}

fn from_plain<T>(s: &str, param: &str) -> Result<T, Error>
where
    T: FromPlain,
    T::Err: Into<Box<error::Error + Sync + Send>>,
{
    T::from_plain(s)
        .map_err(|e| Error::service_safe(e, InvalidArgument::new()).with_safe_param("param", param))
}

pub fn parse_query_param<T>(
    query_params: &QueryParams,
    param: &str,
    param_id: &str,
) -> Result<T, Error>
where
    T: FromPlain,
    T::Err: Into<Box<error::Error + Sync + Send>>,
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
    query_params: &QueryParams,
    param: &str,
    param_id: &str,
    value: &mut Option<T>,
) -> Result<(), Error>
where
    T: FromPlain,
    T::Err: Into<Box<error::Error + Sync + Send>>,
{
    let values = &query_params[param_id];
    if values.is_empty() {
        return Ok(());
    }
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
    query_params: &QueryParams,
    param: &str,
    param_id: &str,
    value: &mut Vec<T>,
) -> Result<(), Error>
where
    T: FromPlain,
    T::Err: Into<Box<error::Error + Sync + Send>>,
{
    for query_param in &query_params[param_id] {
        let parsed = from_plain(query_param, param)?;
        value.push(parsed);
    }

    Ok(())
}

pub fn parse_set_query_param<T>(
    query_params: &QueryParams,
    param: &str,
    param_id: &str,
    value: &mut BTreeSet<T>,
) -> Result<(), Error>
where
    T: FromPlain + Ord,
    T::Err: Into<Box<error::Error + Sync + Send>>,
{
    for query_param in &query_params[param_id] {
        let parsed = from_plain(query_param, param)?;
        value.insert(parsed);
    }

    Ok(())
}

pub fn parse_required_header<T>(
    headers: &HeaderMap,
    param: &str,
    param_id: &str,
) -> Result<T, Error>
where
    T: FromPlain,
    T::Err: Into<Box<error::Error + Sync + Send>>,
{
    headers
        .get(param_id)
        .ok_or_else(|| {
            Error::service_safe("required header parameter missing", InvalidArgument::new())
                .with_safe_param("param", param)
        })
        .and_then(|h| parse_header(h, param))
}

pub fn parse_optional_header<T>(
    headers: &HeaderMap,
    param: &str,
    param_id: &str,
    value: &mut Option<T>,
) -> Result<(), Error>
where
    T: FromPlain,
    T::Err: Into<Box<error::Error + Sync + Send>>,
{
    if let Some(header) = headers.get(param_id) {
        let header = parse_header(header, param)?;
        *value = Some(header);
    }

    Ok(())
}

fn parse_header<T>(header: &HeaderValue, param: &str) -> Result<T, Error>
where
    T: FromPlain,
    T::Err: Into<Box<error::Error + Sync + Send>>,
{
    header
        .to_str()
        .map_err(|e| Error::service_safe(e, InvalidArgument::new()).with_safe_param("param", param))
        .and_then(|h| from_plain(h, param))
}

pub fn parse_cookie_auth(headers: &HeaderMap, prefix: &str) -> Result<BearerToken, Error> {
    parse_auth_inner(headers, prefix, COOKIE)
}

pub fn parse_header_auth(headers: &HeaderMap) -> Result<BearerToken, Error> {
    parse_auth_inner(headers, "Bearer ", AUTHORIZATION)
}

fn parse_auth_inner(
    headers: &HeaderMap,
    prefix: &str,
    header: HeaderName,
) -> Result<BearerToken, Error> {
    let header = match headers.get(header) {
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

    if !header.starts_with(prefix) {
        return Err(Error::service_safe(
            "invalid auth header format",
            PermissionDenied::new(),
        ));
    }

    header[prefix.len()..]
        .parse()
        .map_err(|e| Error::service_safe(e, PermissionDenied::new()))
}

pub struct EmptyRequestBodyVisitor;

impl<T> VisitRequestBody<T> for EmptyRequestBodyVisitor {
    type Output = ();

    fn visit_empty(self) -> Result<(), Error> {
        Ok(())
    }
}

#[derive(Default)]
pub struct SerializableRequestBodyVisitor<T>(PhantomData<T>);

impl<T> SerializableRequestBodyVisitor<T>
where
    T: DeserializeOwned,
{
    pub fn new() -> SerializableRequestBodyVisitor<T> {
        SerializableRequestBodyVisitor(PhantomData)
    }
}

impl<T, U> VisitRequestBody<U> for SerializableRequestBodyVisitor<T>
where
    T: DeserializeOwned,
{
    type Output = T;

    fn visit_serializable<'de, D>(self, deserializer: D) -> Result<T, Error>
    where
        D: Deserializer<'de>,
        D::Error: Into<Box<error::Error + Sync + Send>>,
    {
        T::deserialize(deserializer).map_err(|e| Error::service(e, InvalidArgument::new()))
    }
}

#[derive(Default)]
pub struct DefaultSerializableRequestBodyVisitor<T>(PhantomData<T>);

impl<T> DefaultSerializableRequestBodyVisitor<T>
where
    T: Default + DeserializeOwned,
{
    pub fn new() -> DefaultSerializableRequestBodyVisitor<T> {
        DefaultSerializableRequestBodyVisitor(PhantomData)
    }
}

impl<T, U> VisitRequestBody<U> for DefaultSerializableRequestBodyVisitor<T>
where
    T: Default + DeserializeOwned,
{
    type Output = T;

    fn visit_empty(self) -> Result<T, Error> {
        Ok(T::default())
    }

    fn visit_serializable<'de, D>(self, deserializer: D) -> Result<T, Error>
    where
        D: Deserializer<'de>,
        D::Error: Into<Box<error::Error + Sync + Send>>,
    {
        T::deserialize(deserializer).map_err(|e| Error::service(e, InvalidArgument::new()))
    }
}

pub struct BinaryRequestBodyVisitor;

impl<T> VisitRequestBody<T> for BinaryRequestBodyVisitor {
    type Output = T;

    fn visit_binary(self, body: T) -> Result<T, Error> {
        Ok(body)
    }
}

pub struct EmptyResponse;

impl<W> Response<W> for EmptyResponse {
    fn accept<V>(self, visitor: V) -> Result<V::Output, Error>
    where
        V: VisitResponse<BinaryWriter = W>,
    {
        visitor.visit_empty()
    }
}

pub struct SerializableResponse<T>(pub T);

impl<T, W> Response<W> for SerializableResponse<T>
where
    T: Serialize + 'static,
{
    fn accept<V>(self, visitor: V) -> Result<V::Output, Error>
    where
        V: VisitResponse<BinaryWriter = W>,
    {
        visitor.visit_serializable(self.0)
    }
}

pub struct DefaultSerializableResponse<T>(pub T);

impl<T, W> Response<W> for DefaultSerializableResponse<T>
where
    T: PartialEq + Default + Serialize + 'static,
{
    fn accept<V>(self, visitor: V) -> Result<V::Output, Error>
    where
        V: VisitResponse<BinaryWriter = W>,
    {
        if self.0 == T::default() {
            visitor.visit_empty()
        } else {
            visitor.visit_serializable(self.0)
        }
    }
}

pub struct BinaryResponse<T>(pub T);

impl<T, W> Response<W> for BinaryResponse<T>
where
    T: WriteBody<W> + 'static,
{
    fn accept<V>(self, visitor: V) -> Result<V::Output, Error>
    where
        V: VisitResponse<BinaryWriter = W>,
    {
        visitor.visit_binary(self.0)
    }
}

pub struct OptionalBinaryResponse<T>(pub Option<T>);

impl<T, W> Response<W> for OptionalBinaryResponse<T>
where
    T: WriteBody<W> + 'static,
{
    fn accept<V>(self, visitor: V) -> Result<V::Output, Error>
    where
        V: VisitResponse<BinaryWriter = W>,
    {
        match self.0 {
            Some(body) => visitor.visit_binary(body),
            None => visitor.visit_empty(),
        }
    }
}
