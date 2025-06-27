#![allow(deprecated)]
use std::fmt;
use std::str;
/// Safety with regards to logging based on [safe-logging](https://github.com/palantir/safe-logging) concepts.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    conjure_object::serde::Deserialize,
    conjure_object::serde::Serialize,
)]
#[serde(crate = "conjure_object::serde")]
pub enum LogSafety {
    /// Explicitly marks an element as safe.
    #[serde(rename = "SAFE")]
    Safe,
    /// Explicitly marks an element as unsafe, diallowing contents from being logged as `SAFE`.
    #[serde(rename = "UNSAFE")]
    Unsafe,
    /// Marks elements that must never be logged. For example, credentials, keys, and other secrets cannot be logged because such an action would compromise security.
    #[serde(rename = "DO_NOT_LOG")]
    DoNotLog,
}
impl LogSafety {
    /// Returns the string representation of the enum.
    #[inline]
    pub fn as_str(&self) -> &str {
        match self {
            LogSafety::Safe => "SAFE",
            LogSafety::Unsafe => "UNSAFE",
            LogSafety::DoNotLog => "DO_NOT_LOG",
        }
    }
}
impl fmt::Display for LogSafety {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), fmt)
    }
}
impl conjure_object::Plain for LogSafety {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        conjure_object::Plain::fmt(self.as_str(), fmt)
    }
}
impl str::FromStr for LogSafety {
    type Err = conjure_object::plain::ParseEnumError;
    #[inline]
    fn from_str(v: &str) -> Result<LogSafety, conjure_object::plain::ParseEnumError> {
        match v {
            "SAFE" => Ok(LogSafety::Safe),
            "UNSAFE" => Ok(LogSafety::Unsafe),
            "DO_NOT_LOG" => Ok(LogSafety::DoNotLog),
            _ => Err(conjure_object::plain::ParseEnumError::new()),
        }
    }
}
impl conjure_object::FromPlain for LogSafety {
    type Err = conjure_object::plain::ParseEnumError;
    #[inline]
    fn from_plain(v: &str) -> Result<LogSafety, conjure_object::plain::ParseEnumError> {
        v.parse()
    }
}
