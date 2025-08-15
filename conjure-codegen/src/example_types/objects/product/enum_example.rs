#![allow(deprecated)]
use std::fmt;
use std::str;
/// This enumerates the numbers 1:2.
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
pub enum EnumExample {
    #[serde(rename = "ONE")]
    One,
    #[serde(rename = "TWO")]
    Two,
    /// An unknown variant.
    #[serde(untagged)]
    Unknown(Unknown),
}
impl EnumExample {
    /// Returns the string representation of the enum.
    #[inline]
    pub fn as_str(&self) -> &str {
        match self {
            EnumExample::One => "ONE",
            EnumExample::Two => "TWO",
            EnumExample::Unknown(v) => &*v,
        }
    }
}
impl fmt::Display for EnumExample {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), fmt)
    }
}
impl conjure_object::Plain for EnumExample {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        conjure_object::Plain::fmt(self.as_str(), fmt)
    }
}
impl str::FromStr for EnumExample {
    type Err = conjure_object::plain::ParseEnumError;
    #[inline]
    fn from_str(v: &str) -> Result<EnumExample, conjure_object::plain::ParseEnumError> {
        match v {
            "ONE" => Ok(EnumExample::One),
            "TWO" => Ok(EnumExample::Two),
            v => v.parse().map(|v| EnumExample::Unknown(Unknown(v))),
        }
    }
}
impl conjure_object::FromPlain for EnumExample {
    type Err = conjure_object::plain::ParseEnumError;
    #[inline]
    fn from_plain(
        v: &str,
    ) -> Result<EnumExample, conjure_object::plain::ParseEnumError> {
        v.parse()
    }
}
///An unknown variant of the EnumExample enum.
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
#[serde(crate = "conjure_object::serde", transparent)]
pub struct Unknown(conjure_object::private::Variant);
impl std::ops::Deref for Unknown {
    type Target = str;
    #[inline]
    fn deref(&self) -> &str {
        &self.0
    }
}
impl fmt::Display for Unknown {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, fmt)
    }
}
