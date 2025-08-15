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

//! Implementations for Conjure-generated endpoints.

use std::convert::TryFrom;

use bytes::Bytes;
use conjure_error::Error;
use conjure_object::{Plain, ToPlain};
use futures_core::Stream;
use http::{header::CONTENT_TYPE, HeaderValue, Response, StatusCode};
use serde::de::{DeserializeOwned, IgnoredAny};

use crate::private::APPLICATION_OCTET_STREAM;

use super::{
    AsyncDeserializeResponse, AsyncRequestBody, AsyncSerializeRequest, AsyncWriteBody,
    BoxAsyncWriteBody, DeserializeResponse, EncodeHeader, EncodeParam, RequestBody,
    SerializeRequest, StdResponseDeserializer, WriteBody,
};

/// A body serializer for streaming requests.
pub enum BinaryRequestSerializer {}

impl<'a, T, R> SerializeRequest<'a, T, R> for BinaryRequestSerializer
where
    T: WriteBody<R> + 'a,
{
    fn content_type(_: &T) -> HeaderValue {
        APPLICATION_OCTET_STREAM
    }

    fn serialize(value: T) -> Result<RequestBody<'a, R>, Error> {
        Ok(RequestBody::Streaming(Box::new(value)))
    }
}

impl<'a, T, R> AsyncSerializeRequest<'a, T, R> for BinaryRequestSerializer
where
    T: AsyncWriteBody<R> + Send + 'a,
{
    fn content_type(_: &T) -> HeaderValue {
        APPLICATION_OCTET_STREAM
    }

    fn serialize(value: T) -> Result<AsyncRequestBody<'a, R>, Error> {
        Ok(AsyncRequestBody::Streaming(BoxAsyncWriteBody::new(value)))
    }
}

/// A body deserializer for collection types.
pub enum CollectionResponseDeserializer {}

impl<T, R> DeserializeResponse<T, R> for CollectionResponseDeserializer
where
    T: DeserializeOwned + Default,
    R: Iterator<Item = Result<Bytes, Error>>,
{
    fn accept() -> Option<HeaderValue> {
        <StdResponseDeserializer as DeserializeResponse<T, R>>::accept()
    }

    fn deserialize(response: Response<R>) -> Result<T, Error> {
        if response.status() == StatusCode::NO_CONTENT {
            return Ok(T::default());
        }

        <StdResponseDeserializer as DeserializeResponse<T, R>>::deserialize(response)
    }
}

impl<T, R> AsyncDeserializeResponse<T, R> for CollectionResponseDeserializer
where
    T: DeserializeOwned + Default,
    R: Stream<Item = Result<Bytes, Error>> + Send,
{
    fn accept() -> Option<HeaderValue> {
        <StdResponseDeserializer as AsyncDeserializeResponse<T, R>>::accept()
    }

    async fn deserialize(response: Response<R>) -> Result<T, Error> {
        if response.status() == StatusCode::NO_CONTENT {
            return Ok(T::default());
        }

        <StdResponseDeserializer as AsyncDeserializeResponse<T, R>>::deserialize(response).await
    }
}

/// A body deserializer for binary types.
pub enum BinaryResponseDeserializer {}

impl<R> DeserializeResponse<R, R> for BinaryResponseDeserializer {
    fn accept() -> Option<HeaderValue> {
        Some(APPLICATION_OCTET_STREAM)
    }

    fn deserialize(response: Response<R>) -> Result<R, Error> {
        if response.headers().get(CONTENT_TYPE) != Some(&APPLICATION_OCTET_STREAM) {
            return Err(Error::internal_safe("invalid response Content-Type"));
        }

        Ok(response.into_body())
    }
}

impl<R> AsyncDeserializeResponse<R, R> for BinaryResponseDeserializer
where
    R: Send,
{
    fn accept() -> Option<HeaderValue> {
        Some(APPLICATION_OCTET_STREAM)
    }

    async fn deserialize(response: Response<R>) -> Result<R, Error> {
        if response.headers().get(CONTENT_TYPE) != Some(&APPLICATION_OCTET_STREAM) {
            return Err(Error::internal_safe("invalid response Content-Type"));
        }

        Ok(response.into_body())
    }
}

/// A body deserializer for optional binary types.
pub enum OptionalBinaryResponseDeserializer {}

impl<R> DeserializeResponse<Option<R>, R> for OptionalBinaryResponseDeserializer {
    fn accept() -> Option<HeaderValue> {
        <BinaryResponseDeserializer as DeserializeResponse<R, R>>::accept()
    }

    fn deserialize(response: Response<R>) -> Result<Option<R>, Error> {
        if response.status() == StatusCode::NO_CONTENT {
            return Ok(None);
        }

        <BinaryResponseDeserializer as DeserializeResponse<R, R>>::deserialize(response).map(Some)
    }
}

impl<R> AsyncDeserializeResponse<Option<R>, R> for OptionalBinaryResponseDeserializer
where
    R: Send,
{
    fn accept() -> Option<HeaderValue> {
        <BinaryResponseDeserializer as DeserializeResponse<R, R>>::accept()
    }

    async fn deserialize(response: Response<R>) -> Result<Option<R>, Error> {
        if response.status() == StatusCode::NO_CONTENT {
            return Ok(None);
        }

        <BinaryResponseDeserializer as AsyncDeserializeResponse<R, R>>::deserialize(response)
            .await
            .map(Some)
    }
}

/// A body deserializer for unit types.
pub enum EmptyResponseDeserializer {}

impl<R> DeserializeResponse<(), R> for EmptyResponseDeserializer
where
    R: Iterator<Item = Result<Bytes, Error>>,
{
    fn accept() -> Option<HeaderValue> {
        <StdResponseDeserializer as DeserializeResponse<(), R>>::accept()
    }

    fn deserialize(response: Response<R>) -> Result<(), Error> {
        if response.status() == StatusCode::NO_CONTENT {
            return Ok(());
        }

        <StdResponseDeserializer as DeserializeResponse<IgnoredAny, R>>::deserialize(response)?;

        Ok(())
    }
}

impl<R> AsyncDeserializeResponse<(), R> for EmptyResponseDeserializer
where
    R: Stream<Item = Result<Bytes, Error>> + Send,
{
    fn accept() -> Option<HeaderValue> {
        <StdResponseDeserializer as AsyncDeserializeResponse<(), R>>::accept()
    }

    async fn deserialize(response: Response<R>) -> Result<(), Error> {
        if response.status() == StatusCode::NO_CONTENT {
            return Ok(());
        }

        <StdResponseDeserializer as AsyncDeserializeResponse<IgnoredAny, R>>::deserialize(response)
            .await?;

        Ok(())
    }
}

/// An encoder which converts values via their `Plain` implementation.
pub enum PlainEncoder {}

impl<T> EncodeParam<T> for PlainEncoder
where
    T: Plain,
{
    fn encode(value: T) -> Result<Vec<String>, Error> {
        Ok(vec![value.to_plain()])
    }
}

impl<T> EncodeHeader<T> for PlainEncoder
where
    T: Plain,
{
    fn encode(value: T) -> Result<Vec<HeaderValue>, Error> {
        HeaderValue::try_from(value.to_plain())
            .map_err(Error::internal_safe)
            .map(|v| vec![v])
    }
}

/// An encoder which converts a sequence of values via their individual `Plain` implementations.
pub enum PlainSeqEncoder {}

impl<T, U> EncodeParam<T> for PlainSeqEncoder
where
    T: IntoIterator<Item = U>,
    U: Plain,
{
    fn encode(value: T) -> Result<Vec<String>, Error> {
        Ok(value.into_iter().map(|v| v.to_plain()).collect())
    }
}

impl<T, U> EncodeHeader<T> for PlainSeqEncoder
where
    T: IntoIterator<Item = U>,
    U: Plain,
{
    fn encode(value: T) -> Result<Vec<HeaderValue>, Error> {
        value
            .into_iter()
            .map(|v| HeaderValue::try_from(v.to_plain()).map_err(Error::internal_safe))
            .collect()
    }
}
