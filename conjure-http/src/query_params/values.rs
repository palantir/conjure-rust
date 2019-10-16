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

//! Query parameter values.

use std::ops::Index;
use std::slice;

/// A data structure storing the values for a specific query parameter.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Values(Vec<String>);

impl Values {
    #[inline]
    pub(crate) const fn new() -> Values {
        Values(Vec::new())
    }

    #[inline]
    pub(crate) fn push<T>(&mut self, value: T)
    where
        T: Into<String>,
    {
        self.0.push(value.into());
    }

    #[inline]
    pub(crate) fn extend<T, I>(&mut self, values: I)
    where
        T: Into<String>,
        I: IntoIterator<Item = T>,
    {
        self.0.extend(values.into_iter().map(Into::into));
    }

    /// Returns `true` if there are no values corresponding to the key.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the number of values.
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns an iterator over the values.
    #[inline]
    pub fn iter(&self) -> Iter<'_> {
        Iter(self.0.iter())
    }
}

impl Index<usize> for Values {
    type Output = str;

    #[inline]
    fn index(&self, idx: usize) -> &str {
        &self.0[idx]
    }
}

impl<'a> IntoIterator for &'a Values {
    type Item = &'a str;
    type IntoIter = Iter<'a>;

    #[inline]
    fn into_iter(self) -> Iter<'a> {
        self.iter()
    }
}

/// An iterator over query parameter values.
pub struct Iter<'a>(slice::Iter<'a, String>);

impl<'a> Iterator for Iter<'a> {
    type Item = &'a str;

    #[inline]
    fn next(&mut self) -> Option<&'a str> {
        self.0.next().map(|v| &**v)
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
