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
pub struct BearerTokenAliasExample(pub conjure_object::BearerToken);
impl conjure_object::Plain for BearerTokenAliasExample {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        conjure_object::Plain::fmt(&self.0, fmt)
    }
}
impl conjure_object::FromPlain for BearerTokenAliasExample {
    type Err = <conjure_object::BearerToken as conjure_object::FromPlain>::Err;
    #[inline]
    fn from_plain(s: &str) -> Result<BearerTokenAliasExample, Self::Err> {
        conjure_object::FromPlain::from_plain(s).map(BearerTokenAliasExample)
    }
}
impl std::convert::From<conjure_object::BearerToken> for BearerTokenAliasExample {
    #[inline]
    fn from(v: conjure_object::BearerToken) -> Self {
        BearerTokenAliasExample(std::convert::From::from(v))
    }
}
impl std::ops::Deref for BearerTokenAliasExample {
    type Target = conjure_object::BearerToken;
    #[inline]
    fn deref(&self) -> &conjure_object::BearerToken {
        &self.0
    }
}
impl std::ops::DerefMut for BearerTokenAliasExample {
    #[inline]
    fn deref_mut(&mut self) -> &mut conjure_object::BearerToken {
        &mut self.0
    }
}
impl std::convert::AsRef<conjure_object::BearerToken> for BearerTokenAliasExample {
    #[inline]
    fn as_ref(&self) -> &conjure_object::BearerToken {
        &self.0
    }
}
