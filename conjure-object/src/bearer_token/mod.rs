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

//! The Conjure `bearertoken` type.
use serde::de::{self, Deserialize, Deserializer, Unexpected};
use serde::ser::{Serialize, Serializer};
use std::borrow::Borrow;
use std::error::Error;
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

#[cfg(test)]
mod test;

// A lookup table mapping valid characters to themselves and invalid characters to 0. We don't actually care what
// nonzero value valid characters map to, but it's easier to read this way. There's a test making sure that the mapping
// is consistent.
#[rustfmt::skip]
static VALID_CHARS: [u8; 256] = [
    // 0     1     2     3     4     5     6     7     8     9
       0,    0,    0,    0,    0,    0,    0,    0,    0,    0, //   x
       0,    0,    0,    0,    0,    0,    0,    0,    0,    0, //  1x
       0,    0,    0,    0,    0,    0,    0,    0,    0,    0, //  2x
       0,    0,    0,    0,    0,    0,    0,    0,    0,    0, //  3x
       0,    0,    0, b'+',    0, b'-', b'.', b'/', b'0', b'1', //  4x
    b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9',    0,    0, //  5x
       0,    0,    0,    0,    0, b'A', b'B', b'C', b'D', b'E', //  6x
    b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', //  7x
    b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', //  8x
    b'Z',    0,    0,    0,    0, b'_',    0, b'a', b'b', b'c', //  9x
    b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', // 10x
    b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', // 11x
    b'x', b'y', b'z',    0,    0,    0, b'~',    0,    0,    0, // 12x
       0,    0,    0,    0,    0,    0,    0,    0,    0,    0, // 13x
       0,    0,    0,    0,    0,    0,    0,    0,    0,    0, // 14x
       0,    0,    0,    0,    0,    0,    0,    0,    0,    0, // 15x
       0,    0,    0,    0,    0,    0,    0,    0,    0,    0, // 16x
       0,    0,    0,    0,    0,    0,    0,    0,    0,    0, // 17x
       0,    0,    0,    0,    0,    0,    0,    0,    0,    0, // 18x
       0,    0,    0,    0,    0,    0,    0,    0,    0,    0, // 19x
       0,    0,    0,    0,    0,    0,    0,    0,    0,    0, // 20x
       0,    0,    0,    0,    0,    0,    0,    0,    0,    0, // 21x
       0,    0,    0,    0,    0,    0,    0,    0,    0,    0, // 22x
       0,    0,    0,    0,    0,    0,    0,    0,    0,    0, // 23x
       0,    0,    0,    0,    0,    0,    0,    0,    0,    0, // 24x
       0,    0,    0,    0,    0,    0,                         // 25x
];

/// An authentication bearer token.
///
/// Bearer tokens are strings which match the regular expression `^[A-Za-z0-9\-\._~\+/]+=*$`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BearerToken(String);

impl BearerToken {
    /// Creates a bearer token from a string, validating that it is in the correct format.
    ///
    /// This function behaves identically to `BearerToken`'s `FromStr` implementation.
    #[inline]
    pub fn new(s: &str) -> Result<BearerToken, ParseError> {
        s.parse()
    }
}

impl Deref for BearerToken {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for BearerToken {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Borrow<str> for BearerToken {
    #[inline]
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for BearerToken {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, fmt)
    }
}

impl FromStr for BearerToken {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<BearerToken, ParseError> {
        if !is_valid(s) {
            return Err(ParseError(()));
        }

        Ok(BearerToken(s.to_string()))
    }
}

impl Serialize for BearerToken {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(s)
    }
}

impl<'de> Deserialize<'de> for BearerToken {
    fn deserialize<D>(d: D) -> Result<BearerToken, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(d)?;

        if is_valid(&s) {
            Ok(BearerToken(s))
        } else {
            Err(de::Error::invalid_value(
                Unexpected::Str(&s),
                &"a bearer token",
            ))
        }
    }
}

fn is_valid(s: &str) -> bool {
    let stripped = s.trim_end_matches('=');

    if stripped.is_empty() || !stripped.as_bytes().iter().cloned().all(valid_char) {
        return false;
    }

    true
}

// implementing this via a lookup table rather than a match is ~25% faster.
fn valid_char(b: u8) -> bool {
    VALID_CHARS[b as usize] != 0
}

/// An error parsing a string into a `BearerToken`.
#[derive(Debug)]
pub struct ParseError(());

impl fmt::Display for ParseError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("invalid bearer token")
    }
}

impl Error for ParseError {}
