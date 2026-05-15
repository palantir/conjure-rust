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
//! Type-level marking of values as safe to log.
//!
//! Enable the `log-safety` cargo feature to opt in.

use std::collections::{BTreeMap, HashMap};

/// Marker trait for types whose serialized representation is safe to include
/// in logs and error parameters.
pub trait Safe {}

// conjure-object types that are safe
impl Safe for crate::ResourceIdentifier {}
impl Safe for crate::Uuid {}

// containers
impl<T: Safe + ?Sized> Safe for &T {}
impl<T: Safe + ?Sized> Safe for &mut T {}
impl<T: Safe + ?Sized> Safe for Box<T> {}
impl<T: Safe> Safe for Option<T> {}
impl<T: Safe> Safe for Vec<T> {}
impl<T: Safe, const N: usize> Safe for [T; N] {}
impl<K: Safe, V: Safe> Safe for BTreeMap<K, V> {}
impl<K: Safe, V: Safe> Safe for HashMap<K, V> {}

pub use conjure_macros::Safe;
