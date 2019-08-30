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
use conjure_error::Error;
use conjure_object::{BearerToken, Plain, ToPlain};
use http::header::{HeaderMap, HeaderName, HeaderValue, AUTHORIZATION, COOKIE};
use serde::de::DeserializeOwned;
use serde::{Deserializer, Serialize};
use std::collections::BTreeSet;
use std::error;
use std::marker::PhantomData;

use crate::client::{Accept, RequestBody, VisitRequestBody, VisitResponse, WriteBody};
use crate::{PathParams, QueryParams};

pub fn encode_path_param<T>(path_params: &mut PathParams, key: &str, value: T)
where
    T: Plain,
{
    path_params.insert(key, value.to_plain());
}

pub fn encode_query_param<T>(query_params: &mut QueryParams, key: &str, value: T)
where
    T: Plain,
{
    query_params.insert(key, value.to_plain());
}

pub fn encode_optional_query_param<T>(query_params: &mut QueryParams, key: &str, value: &Option<T>)
where
    T: Plain,
{
    if let Some(value) = value {
        encode_query_param(query_params, key, value);
    }
}

pub fn encode_list_query_param<T>(query_params: &mut QueryParams, key: &str, values: &[T])
where
    T: Plain,
{
    query_params.insert_all(key, values.iter().map(ToPlain::to_plain));
}

pub fn encode_set_query_param<T>(query_params: &mut QueryParams, key: &str, values: &BTreeSet<T>)
where
    T: Plain,
{
    query_params.insert_all(key, values.iter().map(ToPlain::to_plain));
}

pub fn encode_header<T>(
    headers: &mut HeaderMap,
    param: &str,
    header: &'static str,
    value: T,
) -> Result<(), Error>
where
    T: Plain,
{
    let header = HeaderName::from_static(header);
    let value = HeaderValue::from_shared(value.to_plain().into())
        .map_err(|e| Error::internal_safe(e).with_safe_param("param", param))?;
    headers.insert(header, value);
    Ok(())
}

pub fn encode_optional_header<T>(
    headers: &mut HeaderMap,
    param: &str,
    header: &'static str,
    value: &Option<T>,
) -> Result<(), Error>
where
    T: Plain,
{
    if let Some(value) = value {
        encode_header(headers, param, header, value)?;
    }

    Ok(())
}

pub fn encode_cookie_auth(headers: &mut HeaderMap, prefix: &str, value: &BearerToken) {
    encode_auth(headers, COOKIE, prefix, value);
}

pub fn encode_header_auth(headers: &mut HeaderMap, value: &BearerToken) {
    encode_auth(headers, AUTHORIZATION, "Bearer ", value);
}

fn encode_auth(headers: &mut HeaderMap, header: HeaderName, prefix: &str, value: &BearerToken) {
    let value = format!("{}{}", prefix, value.as_str());
    let value = HeaderValue::from_shared(value.into()).expect("bearer tokens are valid headers");
    headers.insert(header, value);
}

pub struct EmptyRequestBody;

impl<'a, W> RequestBody<'a, W> for EmptyRequestBody {
    fn accept<V>(self, visitor: V) -> V::Output
    where
        V: VisitRequestBody<'a, W>,
    {
        visitor.visit_empty()
    }
}

pub struct SerializableRequestBody<T>(pub T);

impl<'a, T, W> RequestBody<'a, W> for SerializableRequestBody<T>
where
    T: Serialize + 'a,
{
    fn accept<V>(self, visitor: V) -> V::Output
    where
        V: VisitRequestBody<'a, W>,
    {
        visitor.visit_serializable(self.0)
    }
}

pub struct BinaryRequestBody<T>(pub T);

impl<'a, T, W> RequestBody<'a, W> for BinaryRequestBody<T>
where
    T: WriteBody<W> + 'a,
{
    fn accept<V>(self, visitor: V) -> V::Output
    where
        V: VisitRequestBody<'a, W>,
    {
        visitor.visit_binary(self.0)
    }
}

pub struct EmptyResponseVisitor;

impl<T> VisitResponse<T> for EmptyResponseVisitor {
    type Output = ();

    fn accept(&self) -> Accept {
        Accept::Empty
    }

    fn visit_empty(self) -> Result<(), Error> {
        Ok(())
    }
}

#[derive(Default)]
pub struct SerializableResponseVisitor<T>(PhantomData<T>);

impl<T> SerializableResponseVisitor<T>
where
    T: DeserializeOwned,
{
    pub fn new() -> SerializableResponseVisitor<T> {
        SerializableResponseVisitor(PhantomData)
    }
}

impl<T, U> VisitResponse<U> for SerializableResponseVisitor<T>
where
    T: DeserializeOwned,
{
    type Output = T;

    fn accept(&self) -> Accept {
        Accept::Serializable
    }

    fn visit_serializable<'de, D>(self, deserializer: D) -> Result<T, Error>
    where
        D: Deserializer<'de>,
        D::Error: Into<Box<dyn error::Error + Sync + Send>>,
    {
        T::deserialize(deserializer).map_err(Error::internal)
    }
}

#[derive(Default)]
pub struct DefaultSerializableResponseVisitor<T>(PhantomData<T>);

impl<T> DefaultSerializableResponseVisitor<T>
where
    T: Default + DeserializeOwned,
{
    pub fn new() -> DefaultSerializableResponseVisitor<T> {
        DefaultSerializableResponseVisitor(PhantomData)
    }
}

impl<T, U> VisitResponse<U> for DefaultSerializableResponseVisitor<T>
where
    T: Default + DeserializeOwned,
{
    type Output = T;

    fn accept(&self) -> Accept {
        Accept::Serializable
    }

    fn visit_empty(self) -> Result<T, Error> {
        Ok(T::default())
    }

    fn visit_serializable<'de, D>(self, deserializer: D) -> Result<T, Error>
    where
        D: Deserializer<'de>,
        D::Error: Into<Box<dyn error::Error + Sync + Send>>,
    {
        T::deserialize(deserializer).map_err(Error::internal)
    }
}

pub struct BinaryResponseVisitor;

impl<T> VisitResponse<T> for BinaryResponseVisitor {
    type Output = T;

    fn accept(&self) -> Accept {
        Accept::Binary
    }

    fn visit_binary(self, body: T) -> Result<T, Error> {
        Ok(body)
    }
}

pub struct OptionalBinaryResponseVisitor;

impl<T> VisitResponse<T> for OptionalBinaryResponseVisitor {
    type Output = Option<T>;

    fn accept(&self) -> Accept {
        Accept::Binary
    }

    fn visit_empty(self) -> Result<Option<T>, Error> {
        Ok(None)
    }

    fn visit_binary(self, body: T) -> Result<Option<T>, Error> {
        Ok(Some(body))
    }
}
