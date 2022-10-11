use conjure_object::serde::{ser, de};
use std::fmt;
use std::str;
///Safety with regards to logging based on [safe-logging](https://github.com/palantir/safe-logging) concepts.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LogSafety {
    ///Explicitly marks an element as safe.
    Safe,
    ///Explicitly marks an element as unsafe, diallowing contents from being logged as `SAFE`.
    Unsafe,
    ///Marks elements that must never be logged. For example, credentials, keys, and other secrets cannot be logged because such an action would compromise security.
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
impl ser::Serialize for LogSafety {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        s.serialize_str(self.as_str())
    }
}
impl<'de> de::Deserialize<'de> for LogSafety {
    fn deserialize<D>(d: D) -> Result<LogSafety, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_str(Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = LogSafety;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("a string")
    }
    fn visit_str<E>(self, v: &str) -> Result<LogSafety, E>
    where
        E: de::Error,
    {
        match v.parse() {
            Ok(e) => Ok(e),
            Err(_) => {
                Err(de::Error::unknown_variant(v, &["SAFE", "UNSAFE", "DO_NOT_LOG"]))
            }
        }
    }
}
