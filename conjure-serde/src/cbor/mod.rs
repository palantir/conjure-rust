// Copyright 2026 Palantir Technologies, Inc.
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
//! CBOR serialization support.
//!
//! Conjure specifies behavior that differs from standard CBOR serialization:
//!
//! * `null` deserializes into empty collection types (Vec, Set, Map)
//! * Conjure clients ignore unknown fields
//! * Conjure servers reject unknown fields
//!
//! This module provides both convenience functions and `Deserializer` types that wrap
//! `serde_cbor_2` to implement these Conjure-specific behaviors using the behavior composition
//! pattern shared with JSON and Smile modules.
//!
//! Note: There are specific modifications over base serde_cbor_2 to handle compatibility with Java Jackson CBOR
//!
//! 1. Rust serializes UUID keys as 16-byte CBOR byte strings, but Java expects string format.
//!   - Added `serialize_map_keys_as_strings` helper function that converts UUID keys to strings
//!   - Modified Conjure codegen to automatically add `#[serde(serialize_with = "...")]` attribute for maps with UUID keys
//! 2. Binary data compatibility: Conjure uses `bytes::Bytes` which naturally handles both CBOR byte strings (major type 2, as sent by Java)
//!    and CBOR arrays (major type 4), eliminating the need for special conversion logic.

pub use crate::cbor::de::client::{client_from_reader, client_from_slice, ClientDeserializer};
pub use crate::cbor::de::server::{server_from_reader, server_from_slice, ServerDeserializer};
pub use crate::cbor::ser::{serialize_map_keys_as_strings, to_vec, to_writer};

#[cfg(test)]
mod cbor_map_key_tests;
mod de;
mod ser;
#[cfg(test)]
mod test;
