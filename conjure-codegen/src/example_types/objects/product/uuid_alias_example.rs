#[derive(
    Debug,
    Clone,
    conjure_object::serde::Deserialize,
    conjure_object::serde::Serialize,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash
)]
#[serde(crate = "conjure_object::serde", transparent)]
pub struct UuidAliasExample(pub conjure_object::Uuid);
impl std::fmt::Display for UuidAliasExample {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, fmt)
    }
}
impl conjure_object::Plain for UuidAliasExample {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        conjure_object::Plain::fmt(&self.0, fmt)
    }
}
impl conjure_object::FromPlain for UuidAliasExample {
    type Err = <conjure_object::Uuid as conjure_object::FromPlain>::Err;
    #[inline]
    fn from_plain(s: &str) -> Result<UuidAliasExample, Self::Err> {
        conjure_object::FromPlain::from_plain(s).map(UuidAliasExample)
    }
}
impl std::convert::From<conjure_object::Uuid> for UuidAliasExample {
    #[inline]
    fn from(v: conjure_object::Uuid) -> Self {
        UuidAliasExample(std::convert::From::from(v))
    }
}
impl std::ops::Deref for UuidAliasExample {
    type Target = conjure_object::Uuid;
    #[inline]
    fn deref(&self) -> &conjure_object::Uuid {
        &self.0
    }
}
impl std::ops::DerefMut for UuidAliasExample {
    #[inline]
    fn deref_mut(&mut self) -> &mut conjure_object::Uuid {
        &mut self.0
    }
}
