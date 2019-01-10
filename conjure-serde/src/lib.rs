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

//! Serde serializer and deserializer wrappers compatible with Conjure.
//!
//! Conjure specifies behavior that differs from serde_json's in a couple of ways:
//!
//! * serde_json serializes non-finite floating point values as `null`, while Conjure specifies `"Infinity"`,
//!     `"-Infinity"`, and `"NaN"` as appropriate.
//! * serde_json serializes byte sequences as arrays of numbers, while Conjure specifies Base64-encoded strings.
//!
//! Additionally, Conjure clients should ignore unknown fields while Conjure servers should trigger errors.
//!
//! This crate provides `Serializer` and `Deserializer` implementations which wrap another and handle these special
//! behaviors.
//!
//! # Examples
//!
//! ```
//! use std::f64;
//! use serde::{Deserialize, Serialize};
//!
//! let json = r#""Infinity""#;
//! let json_deserializer = &mut serde_json::Deserializer::from_str(json);
//! let conjure_deserializer = conjure_serde::ClientDeserializer::new(json_deserializer);
//! let value = f64::deserialize(conjure_deserializer).unwrap();
//! assert_eq!(value, f64::INFINITY);
//! ```
pub use crate::de::client::ClientDeserializer;
pub use crate::de::server::ServerDeserializer;
pub use crate::ser::Serializer;

mod de;
mod ser;
#[cfg(test)]
mod test;
