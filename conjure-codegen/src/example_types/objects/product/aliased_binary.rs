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
pub struct AliasedBinary(pub conjure_object::Bytes);
impl conjure_object::Plain for AliasedBinary {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        conjure_object::Plain::fmt(&self.0, fmt)
    }
}
impl conjure_object::FromPlain for AliasedBinary {
    type Err = <conjure_object::Bytes as conjure_object::FromPlain>::Err;
    #[inline]
    fn from_plain(s: &str) -> Result<AliasedBinary, Self::Err> {
        conjure_object::FromPlain::from_plain(s).map(AliasedBinary)
    }
}
impl std::convert::From<conjure_object::Bytes> for AliasedBinary {
    #[inline]
    fn from(v: conjure_object::Bytes) -> Self {
        AliasedBinary(std::convert::From::from(v))
    }
}
impl std::ops::Deref for AliasedBinary {
    type Target = conjure_object::Bytes;
    #[inline]
    fn deref(&self) -> &conjure_object::Bytes {
        &self.0
    }
}
impl std::ops::DerefMut for AliasedBinary {
    #[inline]
    fn deref_mut(&mut self) -> &mut conjure_object::Bytes {
        &mut self.0
    }
}
impl std::convert::AsRef<conjure_object::Bytes> for AliasedBinary {
    #[inline]
    fn as_ref(&self) -> &conjure_object::Bytes {
        &self.0
    }
}
