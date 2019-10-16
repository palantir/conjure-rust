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

//! Query parameters.
use std::collections::hash_map::{self, Entry, HashMap};
use std::ops::Index;

#[doc(inline)]
pub use crate::query_params::values::Values;

pub mod values;

/// A data structure storing the query parameters of a request.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct QueryParams(HashMap<String, Values>);

impl QueryParams {
    /// Creates a new, empty `QueryParams`.
    #[inline]
    pub fn new() -> QueryParams {
        QueryParams::default()
    }

    /// Inserts a parameter.
    ///
    /// If the key already exists, the new value will be added to the existing values.
    pub fn insert<T, U>(&mut self, key: T, value: U)
    where
        T: Into<String>,
        U: Into<String>,
    {
        self.0
            .entry(key.into())
            .or_insert_with(Values::new)
            .push(value)
    }

    /// Inserts multiple parameters.
    ///
    /// If the key already exists, the new values will be added to the existing values.
    pub fn insert_all<T, U, I>(&mut self, key: T, values: I)
    where
        T: Into<String>,
        U: Into<String>,
        I: IntoIterator<Item = U>,
    {
        match self.0.entry(key.into()) {
            Entry::Occupied(mut e) => e.get_mut().extend(values),
            Entry::Vacant(e) => {
                let mut new = Values::new();
                new.extend(values);
                if !new.is_empty() {
                    e.insert(new);
                }
            }
        }
    }

    /// Returns an iterator over the parameters.
    #[inline]
    pub fn iter(&self) -> Iter<'_> {
        Iter(self.0.iter())
    }
}

impl<'a> IntoIterator for &'a QueryParams {
    type Item = (&'a str, &'a Values);
    type IntoIter = Iter<'a>;

    #[inline]
    fn into_iter(self) -> Iter<'a> {
        self.iter()
    }
}

impl<'a> Index<&'a str> for QueryParams {
    type Output = Values;

    #[inline]
    fn index(&self, key: &'a str) -> &Values {
        static EMPTY: Values = Values::new();
        self.0.get(key).unwrap_or_else(|| &EMPTY)
    }
}

/// An iterator over query parameters.
pub struct Iter<'a>(hash_map::Iter<'a, String, Values>);

impl<'a> Iterator for Iter<'a> {
    type Item = (&'a str, &'a Values);

    #[inline]
    fn next(&mut self) -> Option<(&'a str, &'a Values)> {
        self.0.next().map(|v| (&**v.0, v.1))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'a> ExactSizeIterator for Iter<'a> {
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}
