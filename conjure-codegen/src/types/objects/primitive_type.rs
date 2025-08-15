#![allow(deprecated)]
use std::fmt;
use std::str;
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
pub enum PrimitiveType {
    #[serde(rename = "STRING")]
    String,
    #[serde(rename = "DATETIME")]
    Datetime,
    #[serde(rename = "INTEGER")]
    Integer,
    #[serde(rename = "DOUBLE")]
    Double,
    #[serde(rename = "SAFELONG")]
    Safelong,
    #[serde(rename = "BINARY")]
    Binary,
    #[serde(rename = "ANY")]
    Any,
    #[serde(rename = "BOOLEAN")]
    Boolean,
    #[serde(rename = "UUID")]
    Uuid,
    #[serde(rename = "RID")]
    Rid,
    #[serde(rename = "BEARERTOKEN")]
    Bearertoken,
}
impl PrimitiveType {
    /// Returns the string representation of the enum.
    #[inline]
    pub fn as_str(&self) -> &str {
        match self {
            PrimitiveType::String => "STRING",
            PrimitiveType::Datetime => "DATETIME",
            PrimitiveType::Integer => "INTEGER",
            PrimitiveType::Double => "DOUBLE",
            PrimitiveType::Safelong => "SAFELONG",
            PrimitiveType::Binary => "BINARY",
            PrimitiveType::Any => "ANY",
            PrimitiveType::Boolean => "BOOLEAN",
            PrimitiveType::Uuid => "UUID",
            PrimitiveType::Rid => "RID",
            PrimitiveType::Bearertoken => "BEARERTOKEN",
        }
    }
}
impl fmt::Display for PrimitiveType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), fmt)
    }
}
impl conjure_object::Plain for PrimitiveType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        conjure_object::Plain::fmt(self.as_str(), fmt)
    }
}
impl str::FromStr for PrimitiveType {
    type Err = conjure_object::plain::ParseEnumError;
    #[inline]
    fn from_str(
        v: &str,
    ) -> Result<PrimitiveType, conjure_object::plain::ParseEnumError> {
        match v {
            "STRING" => Ok(PrimitiveType::String),
            "DATETIME" => Ok(PrimitiveType::Datetime),
            "INTEGER" => Ok(PrimitiveType::Integer),
            "DOUBLE" => Ok(PrimitiveType::Double),
            "SAFELONG" => Ok(PrimitiveType::Safelong),
            "BINARY" => Ok(PrimitiveType::Binary),
            "ANY" => Ok(PrimitiveType::Any),
            "BOOLEAN" => Ok(PrimitiveType::Boolean),
            "UUID" => Ok(PrimitiveType::Uuid),
            "RID" => Ok(PrimitiveType::Rid),
            "BEARERTOKEN" => Ok(PrimitiveType::Bearertoken),
            _ => Err(conjure_object::plain::ParseEnumError::new()),
        }
    }
}
impl conjure_object::FromPlain for PrimitiveType {
    type Err = conjure_object::plain::ParseEnumError;
    #[inline]
    fn from_plain(
        v: &str,
    ) -> Result<PrimitiveType, conjure_object::plain::ParseEnumError> {
        v.parse()
    }
}
