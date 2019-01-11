use conjure_types::serde::{de, ser};
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Inner_ {
    One,
    Two,
    Unknown(Box<str>),
}
#[doc = "This enumerates the numbers 1:2.\n"]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EnumExample(Inner_);
impl EnumExample {
    pub const ONE: EnumExample = EnumExample(Inner_::One);
    pub const TWO: EnumExample = EnumExample(Inner_::Two);
    #[doc = r" Returns the string representation of the enum."]
    #[inline]
    pub fn as_str(&self) -> &str {
        match &self.0 {
            Inner_::One => "ONE",
            Inner_::Two => "TWO",
            Inner_::Unknown(v) => v,
        }
    }
}
impl fmt::Display for EnumExample {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(self.as_str())
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
            "ONE" => Ok(EnumExample::ONE),
            "TWO" => Ok(EnumExample::TWO),
            v => Ok(EnumExample(Inner_::Unknown(v.to_string().into_boxed_str()))),
        }
    }
}
