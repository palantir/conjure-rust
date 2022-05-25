use conjure_object::serde::{ser, de};
use std::fmt;
use std::str;
///This enumerates the numbers 1:2.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EnumExample {
    One,
    Two,
    /// An unknown variant.
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
            v => {
                if conjure_object::private::valid_enum_variant(v) {
                    Ok(EnumExample::Unknown(Unknown(v.to_string().into_boxed_str())))
                } else {
                    Err(conjure_object::plain::ParseEnumError::new())
                }
            }
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
impl ser::Serialize for EnumExample {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        s.serialize_str(self.as_str())
    }
}
impl<'de> de::Deserialize<'de> for EnumExample {
    fn deserialize<D>(d: D) -> Result<EnumExample, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_str(Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = EnumExample;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("a string")
    }
    fn visit_str<E>(self, v: &str) -> Result<EnumExample, E>
    where
        E: de::Error,
    {
        match v.parse() {
            Ok(e) => Ok(e),
            Err(_) => Err(de::Error::unknown_variant(v, &["ONE", "TWO"])),
        }
    }
}
///An unknown variant of the EnumExample enum.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Unknown(Box<str>);
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
