#[derive(
    Debug,
    Clone,
    conjure_object::serde::Deserialize,
    conjure_object::serde::Serialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash
)]
#[serde(crate = "conjure_object::serde", transparent)]
pub struct RidAliasExample(pub conjure_object::ResourceIdentifier);
impl std::fmt::Display for RidAliasExample {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, fmt)
    }
}
impl conjure_object::Plain for RidAliasExample {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        conjure_object::Plain::fmt(&self.0, fmt)
    }
}
impl conjure_object::FromPlain for RidAliasExample {
    type Err = <conjure_object::ResourceIdentifier as conjure_object::FromPlain>::Err;
    #[inline]
    fn from_plain(s: &str) -> Result<RidAliasExample, Self::Err> {
        conjure_object::FromPlain::from_plain(s).map(RidAliasExample)
    }
}
impl std::convert::From<conjure_object::ResourceIdentifier> for RidAliasExample {
    #[inline]
    fn from(v: conjure_object::ResourceIdentifier) -> Self {
        RidAliasExample(std::convert::From::from(v))
    }
}
impl std::ops::Deref for RidAliasExample {
    type Target = conjure_object::ResourceIdentifier;
    #[inline]
    fn deref(&self) -> &conjure_object::ResourceIdentifier {
        &self.0
    }
}
impl std::ops::DerefMut for RidAliasExample {
    #[inline]
    fn deref_mut(&mut self) -> &mut conjure_object::ResourceIdentifier {
        &mut self.0
    }
}
