use conjure_object::serde::{de, ser};
#[doc = "For header parameters, the parameter id must be in Upper-Kebab-Case. For query parameters, the parameter id must be in lowerCamelCase. Numbers are permitted, but not at the beginning of a word.\n"]
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
pub struct ParameterId(pub String);
impl std::fmt::Display for ParameterId {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, fmt)
    }
}
impl std::ops::Deref for ParameterId {
    type Target = String;
    #[inline]
    fn deref(&self) -> &String {
        &self.0
    }
}
impl std::ops::DerefMut for ParameterId {
    #[inline]
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}
impl ser::Serialize for ParameterId {
    fn serialize<S_>(&self, s: S_) -> Result<S_::Ok, S_::Error>
    where
        S_: ser::Serializer,
    {
        self.0.serialize(s)
    }
}
impl<'de> de::Deserialize<'de> for ParameterId {
    fn deserialize<D_>(d: D_) -> Result<ParameterId, D_::Error>
    where
        D_: de::Deserializer<'de>,
    {
        de::Deserialize::deserialize(d).map(ParameterId)
    }
}
