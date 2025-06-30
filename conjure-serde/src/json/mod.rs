// Copyright 2018 Palantir Technologies, Inc.
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
//! JSON serialization support.
//!
//! Conjure specifies behavior that differs from serde_json's in a couple of ways:
//!
//! * serde_json serializes non-finite floating point values as `null`, while Conjure specifies `"Infinity"`,
//!   `"-Infinity"`, and `"NaN"` as appropriate.
//! * serde_json serializes byte sequences as arrays of numbers, while Conjure specifies Base64-encoded strings.
//! * serde_json does not support binary, floating point, or boolean keys, while Conjure does.
//! * serde_json does not deserialize `null` into empty collection types, while Conjure does.
//!
//! Additionally, Conjure clients should ignore unknown fields while Conjure servers should trigger errors.
//!
//! This module provides `Serializer` and `Deserializer` implementations which wrap serde_json's and handle these
//! special behaviors.

pub use crate::json::de::client::{
    client_from_reader, client_from_slice, client_from_str, ClientDeserializer,
};
pub use crate::json::de::server::{
    server_from_reader, server_from_slice, server_from_str, ServerDeserializer,
};
pub use crate::json::ser::{to_string, to_vec, to_writer, Serializer};
pub use serde_json::de::{IoRead, SliceRead, StrRead};

pub(crate) mod de;
pub(crate) mod ser;
#[cfg(test)]
mod test;
