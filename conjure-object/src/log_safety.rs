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
pub trait LogSafe {}

/// Wrapper struct for known-safe data.
#[derive(serde::Serialize)]
pub struct AssertLogSafe<T>(pub T);
impl<T> LogSafe for AssertLogSafe<T> {}

/// Marker bound that resolves to `LogSafe` when `log-safety` is enabled,
/// or is satisfied by any type when it's not.
#[cfg(feature = "log-safety")]
pub trait MaybeLogSafe: LogSafe {}

#[cfg(feature = "log-safety")]
impl<T: LogSafe> MaybeLogSafe for T {}

/// Marker bound that resolves to `LogSafe` when `log-safety` is enabled,
/// or is satisfied by any type when it's not.
#[cfg(not(feature = "log-safety"))]
pub trait MaybeLogSafe {}

#[cfg(not(feature = "log-safety"))]
impl<T> MaybeLogSafe for T {}

// conjure-object types that are safe
impl LogSafe for crate::ResourceIdentifier {}
impl LogSafe for crate::Uuid {}

// containers
impl<T: LogSafe + ?Sized> LogSafe for &T {}
impl<T: LogSafe + ?Sized> LogSafe for &mut T {}
impl<T: LogSafe + ?Sized> LogSafe for Box<T> {}
impl<T: LogSafe> LogSafe for Option<T> {}
impl<T: LogSafe> LogSafe for Vec<T> {}
impl<T: LogSafe, const N: usize> LogSafe for [T; N] {}
impl<K: LogSafe, V: LogSafe> LogSafe for BTreeMap<K, V> {}
impl<K: LogSafe, V: LogSafe> LogSafe for HashMap<K, V> {}

/// Re-exports LogSafe derive macro
pub mod derive {
    pub use conjure_macros::LogSafe;
}
