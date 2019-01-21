use conjure_object::serde::{de, ser};
use std::fmt;
#[doc = "This enumerates the numbers 1:2."]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EnumExample {
    One,
    Two,
    #[doc = r" An unknown variant."]
    Unknown(Unknown),
}
impl EnumExample {
    #[doc = r" Returns the string representation of the enum."]
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
impl ser::Serialize for EnumExample {
    fn serialize<S_>(&self, s: S_) -> Result<S_::Ok, S_::Error>
    where
        S_: ser::Serializer,
    {
        s.serialize_str(self.as_str())
    }
}
impl<'de> de::Deserialize<'de> for EnumExample {
    fn deserialize<D_>(d: D_) -> Result<EnumExample, D_::Error>
    where
        D_: de::Deserializer<'de>,
    {
        d.deserialize_str(Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = EnumExample;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("string")
    }
    fn visit_str<E_>(self, v: &str) -> Result<EnumExample, E_>
    where
        E_: de::Error,
    {
        match v {
            "ONE" => Ok(EnumExample::One),
            "TWO" => Ok(EnumExample::Two),
            v => {
                if conjure_object::private::valid_enum_variant(v) {
                    Ok(EnumExample::Unknown(Unknown(
                        v.to_string().into_boxed_str(),
                    )))
                } else {
                    Err(de::Error::unknown_variant(v, &["ONE", "TWO"]))
                }
            }
        }
    }
}
#[doc = "An unknown variant of the EnumExample enum."]
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
