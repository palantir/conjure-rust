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

//! The Conjure `rid` type.
#![warn(missing_docs, clippy::all)]

use lazy_static::lazy_static;
use regex::Regex;
use serde::de::{self, Deserialize, Deserializer, Unexpected};
use serde::ser::{Serialize, Serializer};
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::error::Error;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

#[cfg(test)]
mod test;

const RID_CLASS: &str = "ri";
const SEPARATOR: &str = ".";

lazy_static! {
    static ref PARSE_REGEX: Regex = Regex::new(
        r"(?x)
            ^
            ri
            \.
            ([a-z][a-z0-9\-]*) #service
            \.
            ((?:[a-z0-9][a-z0-9\-]*)?) #instance
            \.
            ([a-z][a-z0-9\-]*) #type
            \.
            ([a-zA-Z0-9_\-\.]+) #locator
            $
        ",
    )
    .unwrap();
}

/// A common format for wrapping existing unique identifiers to provide additional context.
///
/// Resource identifiers contain 4 components, prefixed by a format identifier `ri`, and separated with periods:
/// `ri.<service>.<instance>.<type>.<locator>`.
///
/// * Service: The service or application that namespaces the rest of the identifier. Must conform to the regex pattern
///   `[a-z][a-z0-9\-]*`.
/// * Instance: An optionally empty string that represents the specific service cluster, to allow for disambiduation of
///   artifacts from different service clusters. Must conform to the regex pattern `([a-z0-9][a-z0-9\-]*)?`.
/// * Type: A service-specific resource type to namespace a group of locators. Must conform to the regex pattern
///   `[a-z][a-z0-9\-\._]+`.
/// * Locator: A string used to uniquely locate the specific resource. Must conform to the regex pattern
///   `[a-zA-Z0-9\-\._]+`.
#[derive(Clone)]
pub struct ResourceIdentifier {
    rid: String,
    service_end: usize,
    instance_end: usize,
    type_end: usize,
}

impl ResourceIdentifier {
    /// Creates a resource identifier from a string.
    ///
    /// This function behaves identically to `ResourceIdentifier`'s `FromStr` implementation.
    #[inline]
    pub fn new(s: &str) -> Result<ResourceIdentifier, ParseError> {
        s.parse()
    }

    /// Creates a resource identifier from its individual components.
    pub fn from_components(
        service: &str,
        instance: &str,
        type_: &str,
        locator: &str,
    ) -> Result<ResourceIdentifier, ParseError> {
        // Make sure there aren't `.`s in componenents other than the locator up front, since that could cause an
        // invalid input to parse correctly.
        if service.contains('.') || instance.contains('.') || type_.contains('.') {
            return Err(ParseError(()));
        }

        format!("ri.{service}.{instance}.{type_}.{locator}").parse()
    }

    /// Returns the service component of the resource identifier.
    #[inline]
    pub fn service(&self) -> &str {
        let start = RID_CLASS.len() + SEPARATOR.len();
        &self.rid[start..self.service_end]
    }

    /// Returns the instance component of the resource identifier.
    #[inline]
    pub fn instance(&self) -> &str {
        let start = self.service_end + SEPARATOR.len();
        &self.rid[start..self.instance_end]
    }

    /// Returns the type component of the resource identifier.
    #[inline]
    pub fn type_(&self) -> &str {
        let start = self.instance_end + SEPARATOR.len();
        &self.rid[start..self.type_end]
    }

    /// Returns the locator component of the resource identifier.
    #[inline]
    pub fn locator(&self) -> &str {
        let start = self.type_end + SEPARATOR.len();
        &self.rid[start..]
    }

    /// Returns the string representation of the resource identifier.
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.rid
    }

    /// Consumes the resource identifier, returning its owned string representation.
    #[inline]
    pub fn into_string(self) -> String {
        self.rid
    }
}

impl FromStr for ResourceIdentifier {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<ResourceIdentifier, ParseError> {
        let captures = match PARSE_REGEX.captures(s) {
            Some(captures) => captures,
            None => return Err(ParseError(())),
        };

        Ok(ResourceIdentifier {
            rid: s.to_string(),
            service_end: captures.get(1).unwrap().end(),
            instance_end: captures.get(2).unwrap().end(),
            type_end: captures.get(3).unwrap().end(),
        })
    }
}

impl Serialize for ResourceIdentifier {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.rid.serialize(s)
    }
}

impl<'de> Deserialize<'de> for ResourceIdentifier {
    fn deserialize<D>(d: D) -> Result<ResourceIdentifier, D::Error>
    where
        D: Deserializer<'de>,
    {
        // FIXME avoid double-allocating string
        let s = String::deserialize(d)?;
        ResourceIdentifier::new(&s)
            .map_err(|_| de::Error::invalid_value(Unexpected::Str(&s), &"a resource identifier"))
    }
}

impl AsRef<str> for ResourceIdentifier {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.rid
    }
}

// NOTE: this is *only* OK because our Hash impl skips the other bits of the struct
impl Borrow<str> for ResourceIdentifier {
    #[inline]
    fn borrow(&self) -> &str {
        &self.rid
    }
}

// These are all manually implemented to delegate to the rid field
impl fmt::Debug for ResourceIdentifier {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.rid, fmt)
    }
}

impl fmt::Display for ResourceIdentifier {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.rid, fmt)
    }
}

impl PartialEq for ResourceIdentifier {
    #[inline]
    fn eq(&self, other: &ResourceIdentifier) -> bool {
        self.rid == other.rid
    }
}

impl Eq for ResourceIdentifier {}

impl PartialOrd for ResourceIdentifier {
    #[inline]
    fn partial_cmp(&self, other: &ResourceIdentifier) -> Option<Ordering> {
        Some(self.cmp(other))
    }

    #[inline]
    fn gt(&self, other: &ResourceIdentifier) -> bool {
        self.rid > other.rid
    }

    #[inline]
    fn ge(&self, other: &ResourceIdentifier) -> bool {
        self.rid >= other.rid
    }

    #[inline]
    fn lt(&self, other: &ResourceIdentifier) -> bool {
        self.rid < other.rid
    }

    #[inline]
    fn le(&self, other: &ResourceIdentifier) -> bool {
        self.rid <= other.rid
    }
}

impl Ord for ResourceIdentifier {
    #[inline]
    fn cmp(&self, other: &ResourceIdentifier) -> Ordering {
        self.rid.cmp(&other.rid)
    }
}

impl Hash for ResourceIdentifier {
    #[inline]
    fn hash<H>(&self, hasher: &mut H)
    where
        H: Hasher,
    {
        self.rid.hash(hasher)
    }
}

/// An error returned from parsing an invalid resource identifier.
#[derive(Debug)]
pub struct ParseError(());

impl fmt::Display for ParseError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("invalid resource identifier")
    }
}

impl Error for ParseError {}
