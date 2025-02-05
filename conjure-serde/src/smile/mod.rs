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
//! Smile serialization support.
//!
//! Conjure specifies behavior that differs from serde-smile's in a couple of ways:
//!
//! * serde-smile does not support binary, floating point, or boolean keys, while Conjure does.
//!
//! Additionally, Conjure clients should ignore unknown fields while Conjure servers should trigger errors.
//!
//! This module provides `Serializer` and `Deserializer` implementations which wrap serde-smile's and handle these
//! special behaviors.

pub use crate::smile::de::client::{
    client_from_mut_slice, client_from_reader, client_from_slice, ClientDeserializer,
};
pub use crate::smile::de::server::{
    server_from_mut_slice, server_from_reader, server_from_slice, ServerDeserializer,
};
pub use crate::smile::ser::{to_vec, to_writer, Serializer};
pub use serde_smile::de::{IoRead, MutSliceRead, SliceRead};

mod de;
mod ser;
#[cfg(test)]
mod test;
