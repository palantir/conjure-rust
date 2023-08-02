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

//! Safe-loggable request parameters.

use conjure_object::Any;
use serde::Serialize;
use std::collections::{hash_map, HashMap};

/// A data structure storing safe-loggable parameters of a request.
///
/// This can be included in the response extensions of a request to be included in request logs.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SafeParams(HashMap<String, Any>);

impl SafeParams {
    /// Creates a new, empty `SafeParams`.
    #[inline]
    pub fn new() -> Self {
        SafeParams::default()
    }

    /// Inserts a parameter.
    ///
    /// # Panics
    ///
    /// Panics if the value fails to serialize into an [`Any`].
    pub fn insert<K, V>(&mut self, name: K, value: &V)
    where
        K: Into<String>,
        V: Serialize,
    {
        self.0.insert(
            name.into(),
            Any::new(value).expect("safe param failed to serialize"),
        );
    }

    /// Returns an iterator over the parameters.
    #[inline]
    pub fn iter(&self) -> Iter<'_> {
        Iter(self.0.iter())
    }
}

impl<'a> IntoIterator for &'a SafeParams {
    type IntoIter = Iter<'a>;
    type Item = (&'a str, &'a Any);

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// An iterator over safe parameters.
pub struct Iter<'a>(hash_map::Iter<'a, String, Any>);

impl<'a> Iterator for Iter<'a> {
    type Item = (&'a str, &'a Any);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(k, v)| (&**k, v))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl ExactSizeIterator for Iter<'_> {
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}
