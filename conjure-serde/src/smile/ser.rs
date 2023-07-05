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
use crate::json::ser::KeyBehavior;
use crate::ser::Behavior;
use serde::ser;
use serde_smile::Error;
use std::io::Write;

/// Serializes a value as Smile into a byte buffer.
pub fn to_vec<T>(value: &T) -> Result<Vec<u8>, Error>
where
    T: ?Sized + ser::Serialize,
{
    let mut buf = Vec::with_capacity(128);
    let mut ser = Serializer::new(&mut buf);
    value.serialize(&mut ser)?;
    Ok(buf)
}

/// Serializes a value as Smile into a writer.
pub fn to_writer<W, T>(writer: W, value: &T) -> Result<(), Error>
where
    W: Write,
    T: ?Sized + ser::Serialize,
{
    let mut ser = Serializer::new(writer);
    value.serialize(&mut ser)?;
    Ok(())
}

/// A serde Smile serializer compatible with the Conjure specification.
pub struct Serializer<W>(serde_smile::Serializer<W>);

impl<W> Serializer<W>
where
    W: Write,
{
    /// Creates a new Conjure Smile serializer.
    pub fn new(writer: W) -> Serializer<W> {
        Serializer(
            serde_smile::Serializer::builder()
                .raw_binary(true)
                .build(writer),
        )
    }

    /// Returns a shared reference to the inner writer.
    pub fn get_ref(&self) -> &W {
        self.0.get_ref()
    }

    /// Returns a mutable reference to the inner writer.
    pub fn get_mut(&mut self) -> &mut W {
        self.0.get_mut()
    }

    /// Consumes the `Serializer`, returning the inner writer.
    pub fn into_inner(self) -> W {
        self.0.into_inner()
    }
}

impl<'a, W> ser::Serializer for &'a mut Serializer<W>
where
    W: Write,
{
    impl_serialize_body!(&'a mut serde_smile::Serializer<W>, ValueBehavior);

    fn is_human_readable(&self) -> bool {
        false
    }
}

pub enum ValueBehavior {}

impl Behavior for ValueBehavior {
    // Smile uses the same key behavior as JSON
    type KeyBehavior = KeyBehavior;
}
