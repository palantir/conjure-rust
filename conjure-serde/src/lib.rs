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
//! # Examples
//!
//! ```
//! use std::f64;
//! use serde::Deserialize;
//!
//! let json = r#""Infinity""#;
//! let mut deserializer = conjure_serde::json::ClientDeserializer::from_str(json);
//! let value = f64::deserialize(&mut deserializer).unwrap();
//! deserializer.end().unwrap();
//! assert_eq!(value, f64::INFINITY);
//! ```
#![warn(clippy::all, missing_docs)]
#![doc(html_root_url = "https://docs.rs/conjure-serde/0.6")]

pub mod json;
