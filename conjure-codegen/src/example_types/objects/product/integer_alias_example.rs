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
pub struct IntegerAliasExample(pub i32);
impl std::fmt::Display for IntegerAliasExample {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, fmt)
    }
}
impl conjure_object::Plain for IntegerAliasExample {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        conjure_object::Plain::fmt(&self.0, fmt)
    }
}
impl conjure_object::FromPlain for IntegerAliasExample {
    type Err = <i32 as conjure_object::FromPlain>::Err;
    #[inline]
    fn from_plain(s: &str) -> Result<IntegerAliasExample, Self::Err> {
        conjure_object::FromPlain::from_plain(s).map(IntegerAliasExample)
    }
}
impl std::convert::From<i32> for IntegerAliasExample {
    #[inline]
    fn from(v: i32) -> Self {
        IntegerAliasExample(std::convert::From::from(v))
    }
}
impl std::ops::Deref for IntegerAliasExample {
    type Target = i32;
    #[inline]
    fn deref(&self) -> &i32 {
        &self.0
    }
}
impl std::ops::DerefMut for IntegerAliasExample {
    #[inline]
    fn deref_mut(&mut self) -> &mut i32 {
        &mut self.0
    }
}
impl std::convert::AsRef<i32> for IntegerAliasExample {
    #[inline]
    fn as_ref(&self) -> &i32 {
        &self.0
    }
}
