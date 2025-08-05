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
pub struct HttpPath(pub String);
impl std::fmt::Display for HttpPath {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, fmt)
    }
}
impl conjure_object::Plain for HttpPath {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        conjure_object::Plain::fmt(&self.0, fmt)
    }
}
impl conjure_object::FromPlain for HttpPath {
    type Err = <String as conjure_object::FromPlain>::Err;
    #[inline]
    fn from_plain(s: &str) -> Result<HttpPath, Self::Err> {
        conjure_object::FromPlain::from_plain(s).map(HttpPath)
    }
}
impl std::convert::From<String> for HttpPath {
    #[inline]
    fn from(v: String) -> Self {
        HttpPath(std::convert::From::from(v))
    }
}
impl std::ops::Deref for HttpPath {
    type Target = String;
    #[inline]
    fn deref(&self) -> &String {
        &self.0
    }
}
impl std::ops::DerefMut for HttpPath {
    #[inline]
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}
