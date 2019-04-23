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
use base64::display::Base64Display;
use chrono::format::{Fixed, Item};
use chrono::{DateTime, Utc};
use serde_bytes::ByteBuf;
use std::f64;
use std::fmt;
use std::iter;
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
        if self.is_nan() {
            fmt::Display::fmt("NaN", fmt)
        } else if *self == f64::INFINITY {
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
