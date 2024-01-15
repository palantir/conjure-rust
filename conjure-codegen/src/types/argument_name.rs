use conjure_object::serde::{ser, de};
///Must be in lowerCamelCase. Numbers are permitted, but not at the beginning of a word. Allowed argument names: "fooBar", "build2Request". Disallowed names: "FooBar", "2BuildRequest".
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ArgumentName(pub String);
impl std::fmt::Display for ArgumentName {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, fmt)
    }
}
impl conjure_object::Plain for ArgumentName {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        conjure_object::Plain::fmt(&self.0, fmt)
    }
}
impl conjure_object::FromPlain for ArgumentName {
    type Err = <String as conjure_object::FromPlain>::Err;
    #[inline]
    fn from_plain(s: &str) -> Result<ArgumentName, Self::Err> {
        conjure_object::FromPlain::from_plain(s).map(ArgumentName)
    }
}
impl std::convert::From<String> for ArgumentName {
    #[inline]
    fn from(v: String) -> Self {
        ArgumentName(std::convert::From::from(v))
    }
}
impl std::ops::Deref for ArgumentName {
    type Target = String;
    #[inline]
    fn deref(&self) -> &String {
        &self.0
    }
}
impl std::ops::DerefMut for ArgumentName {
    #[inline]
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}
impl ser::Serialize for ArgumentName {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self.0.serialize(s)
    }
}
impl<'de> de::Deserialize<'de> for ArgumentName {
    fn deserialize<D>(d: D) -> Result<ArgumentName, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        de::Deserialize::deserialize(d).map(ArgumentName)
    }
}
