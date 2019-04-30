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

//! The Conjure PLAIN format.

use base64::display::Base64Display;
use base64::DecodeError;
use chrono::format::{Fixed, Item, ParseError};
use chrono::{DateTime, Utc};
use serde_bytes::ByteBuf;
use std::error::Error;
use std::f64;
use std::fmt;
use std::iter;
use std::num::ParseFloatError;
use std::str::FromStr;
use uuid::Uuid;

use crate::{BearerToken, ResourceIdentifier, SafeLong};

/// Format trait for the Conjure PLAIN format.
pub trait Plain {
    /// Formats this value in its Conjure PLAIN format.
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result;
}

impl<T> Plain for &T
where
    T: ?Sized + Plain,
{
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        Plain::fmt(&**self, fmt)
    }
}

macro_rules! as_display {
    ($t:ty) => {
        impl Plain for $t {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Display::fmt(self, fmt)
            }
        }
    };
}

as_display!(bool);
as_display!(i32);
as_display!(ResourceIdentifier);
as_display!(SafeLong);
as_display!(str);
as_display!(String);
as_display!(Uuid);

impl Plain for BearerToken {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), fmt)
    }
}

impl Plain for DateTime<Utc> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(
            &self.format_with_items(iter::once(Item::Fixed(Fixed::RFC3339))),
            fmt,
        )
    }
}

impl Plain for f64 {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        // f64's display uses `inf` and `-inf` for infinities, but works otherwise
        if *self == f64::INFINITY {
            fmt::Display::fmt("Infinity", fmt)
        } else if *self == f64::NEG_INFINITY {
            fmt::Display::fmt("-Infinity", fmt)
        } else {
            fmt::Display::fmt(self, fmt)
        }
    }
}

impl Plain for [u8] {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&Base64Display::with_config(self, base64::STANDARD), fmt)
    }
}

impl Plain for Vec<u8> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        Plain::fmt(&**self, fmt)
    }
}

impl Plain for ByteBuf {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        Plain::fmt(&**self, fmt)
    }
}

/// A trait for converting a value to its Conjure PLAIN string representation.
///
/// This is implemented for all types that implement the `Plain` trait.
pub trait ToPlain {
    /// Returns the conjure PLAIN string representation of this value.
    fn to_plain(&self) -> String;
}

impl<T> ToPlain for T
where
    T: ?Sized + Plain,
{
    fn to_plain(&self) -> String {
        struct Adaptor<T>(T);

        impl<T> fmt::Display for Adaptor<T>
        where
            T: Plain,
        {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                Plain::fmt(&self.0, fmt)
            }
        }

        Adaptor(self).to_string()
    }
}

/// Parse a value from its Conjure PLAIN string representation.
pub trait FromPlain: Sized {
    /// The error type returned when parsing fails.
    type Err;

    /// Parse a value from its Conjure PLAIN string representation.
    fn from_plain(s: &str) -> Result<Self, Self::Err>;
}

macro_rules! as_from_str {
    ($t:ty) => {
        impl FromPlain for $t {
            type Err = <$t as FromStr>::Err;

            #[inline]
            fn from_plain(s: &str) -> Result<Self, Self::Err> {
                s.parse()
            }
        }
    };
}

as_from_str!(BearerToken);
as_from_str!(bool);
as_from_str!(i32);
as_from_str!(ResourceIdentifier);
as_from_str!(SafeLong);
as_from_str!(String);
as_from_str!(Uuid);

impl FromPlain for ByteBuf {
    type Err = ParseBinaryError;

    #[inline]
    fn from_plain(s: &str) -> Result<ByteBuf, ParseBinaryError> {
        let buf = base64::decode(s).map_err(ParseBinaryError)?;
        Ok(ByteBuf::from(buf))
    }
}

/// An error parsing a binary value from its Conjure PLAIN format.
#[derive(Debug)]
pub struct ParseBinaryError(DecodeError);

impl fmt::Display for ParseBinaryError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, fmt)
    }
}

impl Error for ParseBinaryError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.0.source()
    }
}

impl FromPlain for DateTime<Utc> {
    type Err = ParseError;

    #[inline]
    fn from_plain(s: &str) -> Result<DateTime<Utc>, ParseError> {
        DateTime::parse_from_rfc3339(s).map(|t| t.with_timezone(&Utc))
    }
}

impl FromPlain for f64 {
    type Err = ParseFloatError;

    #[inline]
    fn from_plain(s: &str) -> Result<f64, ParseFloatError> {
        // f64's normal parser works except for its handling of infinities
        match s {
            "Infinity" => Ok(f64::INFINITY),
            "-Infinity" => Ok(f64::NEG_INFINITY),
            s => s.parse(),
        }
    }
}

/// An error parsing an enum from its Conjure PLAIN format.
#[derive(Debug, Default)]
pub struct ParseEnumError(());

impl ParseEnumError {
    /// Creates a new `ParseEnumError`.
    #[inline]
    pub fn new() -> ParseEnumError {
        ParseEnumError(())
    }
}

impl fmt::Display for ParseEnumError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("invalid enum variant")
    }
}

impl Error for ParseEnumError {}
