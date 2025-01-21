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
pub struct BooleanAliasExample(pub bool);
impl std::fmt::Display for BooleanAliasExample {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, fmt)
    }
}
impl conjure_object::Plain for BooleanAliasExample {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        conjure_object::Plain::fmt(&self.0, fmt)
    }
}
impl conjure_object::FromPlain for BooleanAliasExample {
    type Err = <bool as conjure_object::FromPlain>::Err;
    #[inline]
    fn from_plain(s: &str) -> Result<BooleanAliasExample, Self::Err> {
        conjure_object::FromPlain::from_plain(s).map(BooleanAliasExample)
    }
}
impl std::convert::From<bool> for BooleanAliasExample {
    #[inline]
    fn from(v: bool) -> Self {
        BooleanAliasExample(std::convert::From::from(v))
    }
}
impl std::ops::Deref for BooleanAliasExample {
    type Target = bool;
    #[inline]
    fn deref(&self) -> &bool {
        &self.0
    }
}
impl std::ops::DerefMut for BooleanAliasExample {
    #[inline]
    fn deref_mut(&mut self) -> &mut bool {
        &mut self.0
    }
}
impl std::convert::AsRef<bool> for BooleanAliasExample {
    #[inline]
    fn as_ref(&self) -> &bool {
        &self.0
    }
}
