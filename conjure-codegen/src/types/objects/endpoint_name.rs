/// Should be in lowerCamelCase.
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
pub struct EndpointName(pub String);
impl std::fmt::Display for EndpointName {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, fmt)
    }
}
impl conjure_object::Plain for EndpointName {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        conjure_object::Plain::fmt(&self.0, fmt)
    }
}
impl conjure_object::FromPlain for EndpointName {
    type Err = <String as conjure_object::FromPlain>::Err;
    #[inline]
    fn from_plain(s: &str) -> Result<EndpointName, Self::Err> {
        conjure_object::FromPlain::from_plain(s).map(EndpointName)
    }
}
impl std::convert::From<String> for EndpointName {
    #[inline]
    fn from(v: String) -> Self {
        EndpointName(std::convert::From::from(v))
    }
}
impl std::ops::Deref for EndpointName {
    type Target = String;
    #[inline]
    fn deref(&self) -> &String {
        &self.0
    }
}
impl std::ops::DerefMut for EndpointName {
    #[inline]
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}
impl std::convert::AsRef<String> for EndpointName {
    #[inline]
    fn as_ref(&self) -> &String {
        &self.0
    }
}
