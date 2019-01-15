use conjure_object::serde::{de, ser};
#[doc = "Must be in lowerCamelCase. Numbers are permitted, but not at the beginning of a word. Allowed argument names: \"fooBar\", \"build2Request\". Disallowed names: \"FooBar\", \"2BuildRequest\".\n"]
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
pub struct ArgumentName(pub String);
impl std::fmt::Display for ArgumentName {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, fmt)
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
    fn serialize<S_>(&self, s: S_) -> Result<S_::Ok, S_::Error>
    where
        S_: ser::Serializer,
    {
        self.0.serialize(s)
    }
}
impl<'de> de::Deserialize<'de> for ArgumentName {
    fn deserialize<D_>(d: D_) -> Result<ArgumentName, D_::Error>
    where
        D_: de::Deserializer<'de>,
    {
        de::Deserialize::deserialize(d).map(ArgumentName)
    }
}
