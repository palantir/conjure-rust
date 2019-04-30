use conjure_object::serde::{de, ser};
#[doc = "For header parameters, the parameter id must be in Upper-Kebab-Case. For query parameters, the parameter id must be in lowerCamelCase. Numbers are permitted, but not at the beginning of a word."]
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
pub struct ParameterId(pub String);
impl std::fmt::Display for ParameterId {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, fmt)
    }
}
impl conjure_object::Plain for ParameterId {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        conjure_object::Plain::fmt(&self.0, fmt)
    }
}
impl conjure_object::FromPlain for ParameterId {
    type Err = <String as conjure_object::FromPlain>::Err;
    #[inline]
    fn from_plain(s: &str) -> Result<ParameterId, Self::Err> {
        conjure_object::FromPlain::from_plain(s).map(ParameterId)
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
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self.0.serialize(s)
    }
}
impl<'de> de::Deserialize<'de> for ParameterId {
    fn deserialize<D>(d: D) -> Result<ParameterId, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        de::Deserialize::deserialize(d).map(ParameterId)
    }
}
