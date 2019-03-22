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

//! The Conjure `safelong` type.
use serde::{de, ser};
use std::error::Error;
use std::fmt;
use std::ops::Deref;

use crate::private;

/// An i64 limited to a range safely representable in JSON.
///
/// JSON does not specify requirements of its numeric type, which can lead to issues interoperating between different
/// JSON libraries and languages. In particular, some implementations (including Javascript) interpret numbers as double
/// precision floating point values. Sufficiently large 64-bit integers are not exactly representable as doubles which
/// can cause bugs as numbers change value as they're transmitted from place to place.
///
/// The `SafeLong` type wraps an i64, and avoids these issues by limiting its value to the range that is exactly
/// representable in a double: values between -2<sup>53</sup> + 1 and 2<sup>53</sup> - 1.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SafeLong(i64);

impl SafeLong {
    /// Returns the smallest valid `SafeLong`.
    #[inline]
    pub fn min_value() -> SafeLong {
        SafeLong(-(1 << 53) + 1)
    }

    /// Returns the largest valid `SafeLong`.
    #[inline]
    pub fn max_value() -> SafeLong {
        SafeLong((1 << 53) - 1)
    }

    /// Creates a new `SafeLong` from an `i64`.
    ///
    /// Returns an error if the value is out of range.
    #[inline]
    #[allow(clippy::new_ret_no_self)] // FIXME remove when clippy's fixed
    pub fn new(value: i64) -> Result<SafeLong, BoundsError> {
        if value >= *SafeLong::min_value() && value <= *SafeLong::max_value() {
            Ok(SafeLong(value))
        } else {
            Err(BoundsError(()))
        }
    }
}

impl Deref for SafeLong {
    type Target = i64;

    #[inline]
    fn deref(&self) -> &i64 {
        &self.0
    }
}

impl fmt::Display for SafeLong {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, fmt)
    }
}

impl ser::Serialize for SafeLong {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let _guard = private::set_at_safelong();
        s.serialize_i64(self.0)
    }
}

impl<'de> de::Deserialize<'de> for SafeLong {
    fn deserialize<D>(d: D) -> Result<SafeLong, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let _guard = private::set_at_safelong();
        let value = i64::deserialize(d)?;
        SafeLong::new(value)
            .map_err(|_| de::Error::invalid_value(de::Unexpected::Signed(value), &"a safe long"))
    }
}

macro_rules! impl_from {
    ($($t:ty),*) => {
        $(
            impl From<$t> for SafeLong {
                #[inline]
                fn from(n: $t) -> SafeLong {
                    SafeLong(i64::from(n))
                }
            }
        )*
    }
}

impl_from!(u8, i8, u16, i16, u32, i32);

macro_rules! impl_into {
    ($($t:ty),*) => {
        $(
            impl From<SafeLong> for $t {
                #[inline]
                fn from(n: SafeLong) -> $t {
                    n.0.into()
                }
            }
        )*
    }
}

impl_into!(i64, i128);

/// The error returned from constructing an out-of bounds `SafeLong`.
#[derive(Debug, Clone)]
pub struct BoundsError(());

impl fmt::Display for BoundsError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("value was out of bounds of a safe long")
    }
}

impl Error for BoundsError {}
