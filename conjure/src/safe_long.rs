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
use serde::{de, ser};
use std::error::Error;
use std::fmt;
use std::ops::Deref;

const MIN_SAFE_LONG: i64 = -(1 << 53) + 1;
const MAX_SAFE_LONG: i64 = (1 << 53) - 1;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SafeLong(i64);

impl SafeLong {
    #[inline]
    #[allow(clippy::new_ret_no_self)] // FIXME remove when clippy's fixed
    pub fn new(value: i64) -> Result<SafeLong, SafeLongError> {
        if value >= MIN_SAFE_LONG && value <= MAX_SAFE_LONG {
            Ok(SafeLong(value))
        } else {
            Err(SafeLongError(()))
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

impl ser::Serialize for SafeLong {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        s.serialize_i64(self.0)
    }
}

impl<'de> de::Deserialize<'de> for SafeLong {
    fn deserialize<D>(d: D) -> Result<SafeLong, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let value = i64::deserialize(d)?;
        SafeLong::new(value)
            .map_err(|_| de::Error::invalid_value(de::Unexpected::Signed(value), &"a safe long"))
    }
}

#[derive(Debug, Clone)]
pub struct SafeLongError(());

impl fmt::Display for SafeLongError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("value was out of bounds of a safe long")
    }
}

impl Error for SafeLongError {}
