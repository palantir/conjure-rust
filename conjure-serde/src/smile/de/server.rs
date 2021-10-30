// Copyright 2021 Palantir Technologies, Inc.
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
use crate::de::unknown_fields_behavior::UnknownFieldsBehavior;
use crate::smile::de::client::ValueBehavior;
use serde::de;
use serde_smile::de::{IoRead, MutSliceRead, Read, SliceRead};
use serde_smile::Error;
use std::io::BufRead;

/// Deserializes a value from a reader of Smile data.
pub fn server_from_reader<R, T>(reader: R) -> Result<T, Error>
where
    R: BufRead,
    T: de::DeserializeOwned,
{
    let mut de = ServerDeserializer::from_reader(reader)?;
    let value = T::deserialize(&mut de)?;
    de.end()?;
    Ok(value)
}

/// Deserializes a value from a slice of Smile data.
pub fn server_from_slice<'a, T>(s: &'a [u8]) -> Result<T, Error>
where
    T: de::Deserialize<'a>,
{
    let mut de = ServerDeserializer::from_slice(s)?;
    let value = T::deserialize(&mut de)?;
    de.end()?;
    Ok(value)
}

/// Deserializes a value from a mutable slice of Smile data.
pub fn server_from_mut_slice<'a, T>(s: &'a mut [u8]) -> Result<T, Error>
where
    T: de::Deserialize<'a>,
{
    let mut de = ServerDeserializer::from_mut_slice(s)?;
    let value = T::deserialize(&mut de)?;
    de.end()?;
    Ok(value)
}

/// A serde Smile deserializer appropriate for use by Conjure servers.
pub struct ServerDeserializer<'de, R>(serde_smile::Deserializer<'de, R>);

impl<'de, R> ServerDeserializer<'de, IoRead<R>>
where
    R: BufRead,
{
    /// Creates a Conjure Smile server deserializer from an `io::Read`.
    pub fn from_reader(reader: R) -> Result<Self, Error> {
        serde_smile::Deserializer::from_reader(reader).map(ServerDeserializer)
    }
}

impl<'a> ServerDeserializer<'a, SliceRead<'a>> {
    /// Creates a Conjure Smile server deserializer from a `&[u8]`.
    pub fn from_slice(bytes: &'a [u8]) -> Result<Self, Error> {
        serde_smile::Deserializer::from_slice(bytes).map(ServerDeserializer)
    }
}

impl<'a> ServerDeserializer<'a, MutSliceRead<'a>> {
    /// Creates a Conjure Smile server deserializer from a `&mut [u8]`.
    pub fn from_mut_slice(bytes: &'a mut [u8]) -> Result<Self, Error> {
        serde_smile::Deserializer::from_mut_slice(bytes).map(ServerDeserializer)
    }
}

impl<'de, R> ServerDeserializer<'de, R>
where
    R: Read<'de>,
{
    /// Validates that the input stream is at the end or the Smile end of stream token.
    pub fn end(&mut self) -> Result<(), Error> {
        self.0.end()
    }
}

impl<'a, 'de, R> de::Deserializer<'de> for &'a mut ServerDeserializer<'de, R>
where
    R: Read<'de>,
{
    impl_deserialize_body!(
        &'a mut serde_smile::Deserializer<'de, R>,
        UnknownFieldsBehavior<ValueBehavior>
    );

    fn is_human_readable(&self) -> bool {
        false
    }
}
