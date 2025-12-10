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

//! Implementations for Conjure-generated endpoints.

use std::{error, iter::FromIterator, marker::PhantomData};

use bytes::Bytes;
use conjure_error::{Error, InvalidArgument};
use conjure_object::FromPlain;
use futures_core::Stream;
use http::{header::CONTENT_TYPE, HeaderMap, HeaderValue, Response};
use serde::{de::DeserializeOwned, Serialize};

use crate::private::APPLICATION_OCTET_STREAM;

use super::{
    AsyncDeserializeRequest, AsyncResponseBody, AsyncSerializeResponse, AsyncWriteBody,
    ConjureRuntime, DecodeHeader, DecodeParam, DeserializeRequest, EmptyResponseSerializer,
    LocalAsyncDeserializeRequest, LocalAsyncResponseBody, LocalAsyncSerializeResponse,
    LocalAsyncWriteBody, ResponseBody, SerializeResponse, StdRequestDeserializer,
    StdResponseSerializer, WriteBody,
};

/// A request deserializer for optional body types.
pub enum OptionalRequestDeserializer {}

impl<T, R> DeserializeRequest<Option<T>, R> for OptionalRequestDeserializer
where
    T: DeserializeOwned,
    R: Iterator<Item = Result<Bytes, Error>>,
{
    fn deserialize(
        runtime: &ConjureRuntime,
        headers: &HeaderMap,
        body: R,
    ) -> Result<Option<T>, Error> {
        if !headers.contains_key(CONTENT_TYPE) {
            return Ok(None);
        }

        <StdRequestDeserializer as DeserializeRequest<_, _>>::deserialize(runtime, headers, body)
    }
}

impl<T, R> AsyncDeserializeRequest<Option<T>, R> for OptionalRequestDeserializer
where
    T: DeserializeOwned,
    R: Stream<Item = Result<Bytes, Error>> + Send,
{
    async fn deserialize(
        runtime: &ConjureRuntime,
        headers: &HeaderMap,
        body: R,
    ) -> Result<Option<T>, Error> {
        if !headers.contains_key(CONTENT_TYPE) {
            return Ok(None);
        }

        <StdRequestDeserializer as AsyncDeserializeRequest<_, _>>::deserialize(
            runtime, headers, body,
        )
        .await
    }
}

impl<T, R> LocalAsyncDeserializeRequest<Option<T>, R> for OptionalRequestDeserializer
where
    T: DeserializeOwned,
    R: Stream<Item = Result<Bytes, Error>>,
{
    async fn deserialize(
        runtime: &ConjureRuntime,
        headers: &HeaderMap,
        body: R,
    ) -> Result<Option<T>, Error> {
        if !headers.contains_key(CONTENT_TYPE) {
            return Ok(None);
        }

        <StdRequestDeserializer as LocalAsyncDeserializeRequest<_, _>>::deserialize(
            runtime, headers, body,
        )
        .await
    }
}

/// A request deserializer for binary body types.
pub enum BinaryRequestDeserializer {}

impl BinaryRequestDeserializer {
    fn deserialize_inner<R>(headers: &HeaderMap, body: R) -> Result<R, Error> {
        if headers.get(CONTENT_TYPE) != Some(&APPLICATION_OCTET_STREAM) {
            return Err(Error::service_safe(
                "unexpected Content-Type",
                InvalidArgument::new(),
            ));
        }

        Ok(body)
    }
}

impl<R> DeserializeRequest<R, R> for BinaryRequestDeserializer {
    fn deserialize(_runtime: &ConjureRuntime, headers: &HeaderMap, body: R) -> Result<R, Error> {
        Self::deserialize_inner(headers, body)
    }
}

impl<R> AsyncDeserializeRequest<R, R> for BinaryRequestDeserializer
where
    R: Send,
{
    async fn deserialize(
        _runtime: &ConjureRuntime,
        headers: &HeaderMap,
        body: R,
    ) -> Result<R, Error> {
        Self::deserialize_inner(headers, body)
    }
}

impl<R> LocalAsyncDeserializeRequest<R, R> for BinaryRequestDeserializer {
    async fn deserialize(
        _runtime: &ConjureRuntime,
        headers: &HeaderMap,
        body: R,
    ) -> Result<R, Error> {
        Self::deserialize_inner(headers, body)
    }
}

/// A body serializer for collection types.
pub enum CollectionResponseSerializer {}

impl<T, W> SerializeResponse<T, W> for CollectionResponseSerializer
where
    T: Serialize + PartialEq + Default,
{
    fn serialize(
        runtime: &ConjureRuntime,
        request_headers: &HeaderMap,
        value: T,
    ) -> Result<Response<ResponseBody<W>>, Error> {
        if value == T::default() {
            <EmptyResponseSerializer as SerializeResponse<_, _>>::serialize(
                runtime,
                request_headers,
                (),
            )
        } else {
            <StdResponseSerializer as SerializeResponse<_, _>>::serialize(
                runtime,
                request_headers,
                value,
            )
        }
    }
}

impl<T, W> AsyncSerializeResponse<T, W> for CollectionResponseSerializer
where
    T: Serialize + PartialEq + Default,
{
    fn serialize(
        runtime: &ConjureRuntime,
        request_headers: &HeaderMap,
        value: T,
    ) -> Result<Response<AsyncResponseBody<W>>, Error> {
        if value == T::default() {
            <EmptyResponseSerializer as AsyncSerializeResponse<_, _>>::serialize(
                runtime,
                request_headers,
                (),
            )
        } else {
            <StdResponseSerializer as AsyncSerializeResponse<_, _>>::serialize(
                runtime,
                request_headers,
                value,
            )
        }
    }
}

impl<T, W> LocalAsyncSerializeResponse<T, W> for CollectionResponseSerializer
where
    T: Serialize + PartialEq + Default,
{
    fn serialize(
        runtime: &ConjureRuntime,
        request_headers: &HeaderMap,
        value: T,
    ) -> Result<Response<LocalAsyncResponseBody<W>>, Error> {
        if value == T::default() {
            <EmptyResponseSerializer as LocalAsyncSerializeResponse<_, _>>::serialize(
                runtime,
                request_headers,
                (),
            )
        } else {
            <StdResponseSerializer as LocalAsyncSerializeResponse<_, _>>::serialize(
                runtime,
                request_headers,
                value,
            )
        }
    }
}

/// A response serializer for binary types.
pub enum BinaryResponseSerializer {}

impl BinaryResponseSerializer {
    fn serialize_inner<B>(body: B) -> Result<Response<B>, Error> {
        let mut response = Response::new(body);

        response
            .headers_mut()
            .insert(CONTENT_TYPE, APPLICATION_OCTET_STREAM);
        Ok(response)
    }
}

impl<T, W> SerializeResponse<T, W> for BinaryResponseSerializer
where
    T: WriteBody<W> + 'static,
{
    fn serialize(
        _runtime: &ConjureRuntime,
        _request_headers: &HeaderMap,
        value: T,
    ) -> Result<Response<ResponseBody<W>>, Error> {
        Self::serialize_inner(ResponseBody::Streaming(Box::new(value)))
    }
}

impl<T, W> AsyncSerializeResponse<T, W> for BinaryResponseSerializer
where
    T: AsyncWriteBody<W> + 'static + Send,
{
    fn serialize(
        _runtime: &ConjureRuntime,
        _request_headers: &HeaderMap,
        value: T,
    ) -> Result<Response<AsyncResponseBody<W>>, Error> {
        Self::serialize_inner(AsyncResponseBody::Streaming(super::BoxAsyncWriteBody::new(
            value,
        )))
    }
}

impl<T, W> LocalAsyncSerializeResponse<T, W> for BinaryResponseSerializer
where
    T: LocalAsyncWriteBody<W> + 'static,
{
    fn serialize(
        _runtime: &ConjureRuntime,
        _request_headers: &HeaderMap,
        value: T,
    ) -> Result<Response<LocalAsyncResponseBody<W>>, Error> {
        Self::serialize_inner(LocalAsyncResponseBody::Streaming(
            super::BoxLocalAsyncWriteBody::new(value),
        ))
    }
}

/// A response serializer for optional binary types.
pub enum OptionalBinaryResponseSerializer {}

impl<T, W> SerializeResponse<Option<T>, W> for OptionalBinaryResponseSerializer
where
    T: WriteBody<W> + 'static,
{
    fn serialize(
        runtime: &ConjureRuntime,
        request_headers: &HeaderMap,
        value: Option<T>,
    ) -> Result<Response<ResponseBody<W>>, Error> {
        match value {
            Some(value) => <BinaryResponseSerializer as SerializeResponse<_, _>>::serialize(
                runtime,
                request_headers,
                value,
            ),
            None => <EmptyResponseSerializer as SerializeResponse<_, _>>::serialize(
                runtime,
                request_headers,
                (),
            ),
        }
    }
}

impl<T, W> AsyncSerializeResponse<Option<T>, W> for OptionalBinaryResponseSerializer
where
    T: AsyncWriteBody<W> + 'static + Send,
{
    fn serialize(
        runtime: &ConjureRuntime,
        request_headers: &HeaderMap,
        value: Option<T>,
    ) -> Result<Response<AsyncResponseBody<W>>, Error> {
        match value {
            Some(value) => <BinaryResponseSerializer as AsyncSerializeResponse<_, _>>::serialize(
                runtime,
                request_headers,
                value,
            ),
            None => <EmptyResponseSerializer as AsyncSerializeResponse<_, _>>::serialize(
                runtime,
                request_headers,
                (),
            ),
        }
    }
}

impl<T, W> LocalAsyncSerializeResponse<Option<T>, W> for OptionalBinaryResponseSerializer
where
    T: LocalAsyncWriteBody<W> + 'static,
{
    fn serialize(
        runtime: &ConjureRuntime,
        request_headers: &HeaderMap,
        value: Option<T>,
    ) -> Result<Response<LocalAsyncResponseBody<W>>, Error> {
        match value {
            Some(value) => {
                <BinaryResponseSerializer as LocalAsyncSerializeResponse<_, _>>::serialize(
                    runtime,
                    request_headers,
                    value,
                )
            }
            None => <EmptyResponseSerializer as LocalAsyncSerializeResponse<_, _>>::serialize(
                runtime,
                request_headers,
                (),
            ),
        }
    }
}

/// A decoder which converts a single value using its [`FromPlain`] implementation.
pub enum FromPlainDecoder {}

impl<T> DecodeHeader<T> for FromPlainDecoder
where
    T: FromPlain,
    T::Err: Into<Box<dyn error::Error + Sync + Send>>,
{
    fn decode<'a, I>(_: &ConjureRuntime, headers: I) -> Result<T, Error>
    where
        I: IntoIterator<Item = &'a HeaderValue>,
    {
        T::from_plain(
            super::only_item(headers)?
                .to_str()
                .map_err(|e| Error::service(e, InvalidArgument::new()))?,
        )
        .map_err(|e| Error::service(e, InvalidArgument::new()))
    }
}

impl<T> DecodeParam<T> for FromPlainDecoder
where
    T: FromPlain,
    T::Err: Into<Box<dyn error::Error + Sync + Send>>,
{
    fn decode<I>(_: &ConjureRuntime, params: I) -> Result<T, Error>
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        T::from_plain(super::only_item(params)?.as_ref())
            .map_err(|e| Error::service(e, InvalidArgument::new()))
    }
}

/// A decoder which converts an optional value using its [`FromPlain`] implementation.
pub enum FromPlainOptionDecoder {}

impl<T> DecodeHeader<Option<T>> for FromPlainOptionDecoder
where
    T: FromPlain,
    T::Err: Into<Box<dyn error::Error + Sync + Send>>,
{
    fn decode<'a, I>(_: &ConjureRuntime, headers: I) -> Result<Option<T>, Error>
    where
        I: IntoIterator<Item = &'a HeaderValue>,
    {
        let Some(header) = super::optional_item(headers)? else {
            return Ok(None);
        };
        let value = T::from_plain(
            header
                .to_str()
                .map_err(|e| Error::service(e, InvalidArgument::new()))?,
        )
        .map_err(|e| Error::service(e, InvalidArgument::new()))?;
        Ok(Some(value))
    }
}

impl<T> DecodeParam<Option<T>> for FromPlainOptionDecoder
where
    T: FromPlain,
    T::Err: Into<Box<dyn error::Error + Sync + Send>>,
{
    fn decode<I>(_: &ConjureRuntime, params: I) -> Result<Option<T>, Error>
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        let Some(param) = super::optional_item(params)? else {
            return Ok(None);
        };
        let value =
            T::from_plain(param.as_ref()).map_err(|e| Error::service(e, InvalidArgument::new()))?;
        Ok(Some(value))
    }
}

/// A decoder which converts a sequence of values via its [`FromPlain`] implementation into a
/// collection via a [`FromIterator`] implementation.
pub struct FromPlainSeqDecoder<U> {
    _p: PhantomData<U>,
}

impl<T, U> DecodeParam<T> for FromPlainSeqDecoder<U>
where
    T: FromIterator<U>,
    U: FromPlain,
    U::Err: Into<Box<dyn error::Error + Sync + Send>>,
{
    fn decode<I>(_: &ConjureRuntime, params: I) -> Result<T, Error>
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        params
            .into_iter()
            .map(|s| {
                U::from_plain(s.as_ref()).map_err(|e| Error::service(e, InvalidArgument::new()))
            })
            .collect()
    }
}
