///For header parameters, the parameter id must be in Upper-Kebab-Case. For query parameters, the parameter id must be in lowerCamelCase. Numbers are permitted, but not at the beginning of a word.
#[derive(
    Debug,
    Clone,
    conjure_object::serde::Deserialize,
    conjure_object::serde::Serialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Default
)]
#[serde(crate = "conjure_object::serde", transparent)]
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
impl std::convert::From<String> for ParameterId {
    #[inline]
    fn from(v: String) -> Self {
        ParameterId(std::convert::From::from(v))
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
impl std::convert::AsRef<String> for ParameterId {
    #[inline]
    fn as_ref(&self) -> &String {
        &self.0
    }
}
