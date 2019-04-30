use conjure_object::serde::{de, ser};
use std::fmt;
use std::str;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PrimitiveType {
    String,
    Datetime,
    Integer,
    Double,
    Safelong,
    Binary,
    Any,
    Boolean,
    Uuid,
    Rid,
    Bearertoken,
}
impl PrimitiveType {
    #[doc = r" Returns the string representation of the enum."]
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
    fn from_str(v: &str) -> Result<PrimitiveType, conjure_object::plain::ParseEnumError> {
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
    fn from_plain(v: &str) -> Result<PrimitiveType, conjure_object::plain::ParseEnumError> {
        v.parse()
    }
}
impl ser::Serialize for PrimitiveType {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        s.serialize_str(self.as_str())
    }
}
impl<'de> de::Deserialize<'de> for PrimitiveType {
    fn deserialize<D>(d: D) -> Result<PrimitiveType, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_str(Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = PrimitiveType;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("a string")
    }
    fn visit_str<E>(self, v: &str) -> Result<PrimitiveType, E>
    where
        E: de::Error,
    {
        match v.parse() {
            Ok(e) => Ok(e),
            Err(_) => Err(de::Error::unknown_variant(
                v,
                &[
                    "STRING",
                    "DATETIME",
                    "INTEGER",
                    "DOUBLE",
                    "SAFELONG",
                    "BINARY",
                    "ANY",
                    "BOOLEAN",
                    "UUID",
                    "RID",
                    "BEARERTOKEN",
                ],
            )),
        }
    }
}
