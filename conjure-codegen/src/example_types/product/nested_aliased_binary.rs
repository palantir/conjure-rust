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
pub struct NestedAliasedBinary(pub super::AliasedBinary);
impl conjure_object::Plain for NestedAliasedBinary {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        conjure_object::Plain::fmt(&self.0, fmt)
    }
}
impl conjure_object::FromPlain for NestedAliasedBinary {
    type Err = <super::AliasedBinary as conjure_object::FromPlain>::Err;
    #[inline]
    fn from_plain(s: &str) -> Result<NestedAliasedBinary, Self::Err> {
        conjure_object::FromPlain::from_plain(s).map(NestedAliasedBinary)
    }
}
impl std::convert::From<conjure_object::Bytes> for NestedAliasedBinary {
    #[inline]
    fn from(v: conjure_object::Bytes) -> Self {
        NestedAliasedBinary(std::convert::From::from(v))
    }
}
impl std::ops::Deref for NestedAliasedBinary {
    type Target = super::AliasedBinary;
    #[inline]
    fn deref(&self) -> &super::AliasedBinary {
        &self.0
    }
}
impl std::ops::DerefMut for NestedAliasedBinary {
    #[inline]
    fn deref_mut(&mut self) -> &mut super::AliasedBinary {
        &mut self.0
    }
}
