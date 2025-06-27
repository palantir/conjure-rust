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
pub struct Documentation(pub String);
impl std::fmt::Display for Documentation {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, fmt)
    }
}
impl conjure_object::Plain for Documentation {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        conjure_object::Plain::fmt(&self.0, fmt)
    }
}
impl conjure_object::FromPlain for Documentation {
    type Err = <String as conjure_object::FromPlain>::Err;
    #[inline]
    fn from_plain(s: &str) -> Result<Documentation, Self::Err> {
        conjure_object::FromPlain::from_plain(s).map(Documentation)
    }
}
impl std::convert::From<String> for Documentation {
    #[inline]
    fn from(v: String) -> Self {
        Documentation(std::convert::From::from(v))
    }
}
impl std::ops::Deref for Documentation {
    type Target = String;
    #[inline]
    fn deref(&self) -> &String {
        &self.0
    }
}
impl std::ops::DerefMut for Documentation {
    #[inline]
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}
