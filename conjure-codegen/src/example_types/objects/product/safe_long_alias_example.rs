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
    Hash,
    Default
)]
#[serde(crate = "conjure_object::serde", transparent)]
pub struct SafeLongAliasExample(pub conjure_object::SafeLong);
impl std::fmt::Display for SafeLongAliasExample {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, fmt)
    }
}
impl conjure_object::Plain for SafeLongAliasExample {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        conjure_object::Plain::fmt(&self.0, fmt)
    }
}
impl conjure_object::FromPlain for SafeLongAliasExample {
    type Err = <conjure_object::SafeLong as conjure_object::FromPlain>::Err;
    #[inline]
    fn from_plain(s: &str) -> Result<SafeLongAliasExample, Self::Err> {
        conjure_object::FromPlain::from_plain(s).map(SafeLongAliasExample)
    }
}
impl std::convert::From<conjure_object::SafeLong> for SafeLongAliasExample {
    #[inline]
    fn from(v: conjure_object::SafeLong) -> Self {
        SafeLongAliasExample(std::convert::From::from(v))
    }
}
impl std::ops::Deref for SafeLongAliasExample {
    type Target = conjure_object::SafeLong;
    #[inline]
    fn deref(&self) -> &conjure_object::SafeLong {
        &self.0
    }
}
impl std::ops::DerefMut for SafeLongAliasExample {
    #[inline]
    fn deref_mut(&mut self) -> &mut conjure_object::SafeLong {
        &mut self.0
    }
}
