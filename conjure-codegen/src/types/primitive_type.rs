use conjure::serde::{de, ser};
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Inner_ {
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
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PrimitiveType(Inner_);
impl PrimitiveType {
    pub const STRING: PrimitiveType = PrimitiveType(Inner_::String);
    pub const DATETIME: PrimitiveType = PrimitiveType(Inner_::Datetime);
    pub const INTEGER: PrimitiveType = PrimitiveType(Inner_::Integer);
    pub const DOUBLE: PrimitiveType = PrimitiveType(Inner_::Double);
    pub const SAFELONG: PrimitiveType = PrimitiveType(Inner_::Safelong);
    pub const BINARY: PrimitiveType = PrimitiveType(Inner_::Binary);
    pub const ANY: PrimitiveType = PrimitiveType(Inner_::Any);
    pub const BOOLEAN: PrimitiveType = PrimitiveType(Inner_::Boolean);
    pub const UUID: PrimitiveType = PrimitiveType(Inner_::Uuid);
    pub const RID: PrimitiveType = PrimitiveType(Inner_::Rid);
    pub const BEARERTOKEN: PrimitiveType = PrimitiveType(Inner_::Bearertoken);
    #[doc = r" Returns the string representation of the enum."]
    #[inline]
    pub fn as_str(&self) -> &str {
        match &self.0 {
            Inner_::String => "STRING",
            Inner_::Datetime => "DATETIME",
            Inner_::Integer => "INTEGER",
            Inner_::Double => "DOUBLE",
            Inner_::Safelong => "SAFELONG",
            Inner_::Binary => "BINARY",
            Inner_::Any => "ANY",
            Inner_::Boolean => "BOOLEAN",
            Inner_::Uuid => "UUID",
            Inner_::Rid => "RID",
            Inner_::Bearertoken => "BEARERTOKEN",
        }
    }
}
impl fmt::Display for PrimitiveType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(self.as_str())
    }
}
impl ser::Serialize for PrimitiveType {
    fn serialize<S_>(&self, s: S_) -> Result<S_::Ok, S_::Error>
    where
        S_: ser::Serializer,
    {
        s.serialize_str(self.as_str())
    }
}
impl<'de> de::Deserialize<'de> for PrimitiveType {
    fn deserialize<D_>(d: D_) -> Result<PrimitiveType, D_::Error>
    where
        D_: de::Deserializer<'de>,
    {
        d.deserialize_str(Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = PrimitiveType;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("string")
    }
    fn visit_str<E_>(self, v: &str) -> Result<PrimitiveType, E_>
    where
        E_: de::Error,
    {
        match v {
            "STRING" => Ok(PrimitiveType::STRING),
            "DATETIME" => Ok(PrimitiveType::DATETIME),
            "INTEGER" => Ok(PrimitiveType::INTEGER),
            "DOUBLE" => Ok(PrimitiveType::DOUBLE),
            "SAFELONG" => Ok(PrimitiveType::SAFELONG),
            "BINARY" => Ok(PrimitiveType::BINARY),
            "ANY" => Ok(PrimitiveType::ANY),
            "BOOLEAN" => Ok(PrimitiveType::BOOLEAN),
            "UUID" => Ok(PrimitiveType::UUID),
            "RID" => Ok(PrimitiveType::RID),
            "BEARERTOKEN" => Ok(PrimitiveType::BEARERTOKEN),
            v => Err(de::Error::unknown_variant(
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
