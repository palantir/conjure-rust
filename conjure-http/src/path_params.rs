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

//! Path parameters.

use std::collections::hash_map::{self, HashMap};
use std::ops::Index;

/// A data structure storing the raw, encoded, path parameters of the request.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PathParams(HashMap<String, String>);

impl PathParams {
    /// Creates a new, empty `PathParams`.
    #[inline]
    pub fn new() -> PathParams {
        PathParams::default()
    }

    /// Inserts a parameter.
    #[inline]
    pub fn insert<T, U>(&mut self, key: T, value: U)
    where
        T: Into<String>,
        U: Into<String>,
    {
        self.0.insert(key.into(), value.into());
    }

    /// Returns an iterator over the parameters.
    #[inline]
    pub fn iter(&self) -> Iter<'_> {
        Iter(self.0.iter())
    }
}

impl<'a> IntoIterator for &'a PathParams {
    type IntoIter = Iter<'a>;
    type Item = (&'a str, &'a str);

    #[inline]
    fn into_iter(self) -> Iter<'a> {
        self.iter()
    }
}

impl<'a> Index<&'a str> for PathParams {
    type Output = str;

    #[inline]
    fn index(&self, key: &'a str) -> &str {
        &self.0[key]
    }
}

/// An iterator over path parameters.
pub struct Iter<'a>(hash_map::Iter<'a, String, String>);

impl<'a> Iterator for Iter<'a> {
    type Item = (&'a str, &'a str);

    #[inline]
    fn next(&mut self) -> Option<(&'a str, &'a str)> {
        self.0.next().map(|v| (&**v.0, &**v.1))
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
