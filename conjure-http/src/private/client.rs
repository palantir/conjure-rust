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
use serde::de::DeserializeOwned;
use serde::{Deserializer, Serialize};
use std::error;
use std::marker::PhantomData;

use crate::client::{Accept, RequestBody, VisitRequestBody, VisitResponse, WriteBody};

pub struct EmptyRequestBody;

impl<'a> RequestBody<'a> for EmptyRequestBody {
    fn accept<V>(self, visitor: V) -> V::Output
    where
        V: VisitRequestBody<'a>,
    {
        visitor.visit_empty()
    }
}

pub struct SerializableRequestBody<T>(pub T);

impl<'a, T> RequestBody<'a> for SerializableRequestBody<T>
where
    T: Serialize + 'a,
{
    fn accept<V>(self, visitor: V) -> V::Output
    where
        V: VisitRequestBody<'a>,
    {
        visitor.visit_serializable(self.0)
    }
}

pub struct BinaryRequestBody<T>(pub T);

impl<'a, T> RequestBody<'a> for BinaryRequestBody<T>
where
    T: WriteBody + 'a,
{
    fn accept<V>(self, visitor: V) -> V::Output
    where
        V: VisitRequestBody<'a>,
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
        D::Error: Into<Box<error::Error + Sync + Send>>,
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
        D::Error: Into<Box<error::Error + Sync + Send>>,
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
