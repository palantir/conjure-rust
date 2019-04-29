use conjure_object::serde::{de, ser};
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
pub struct AliasedBinary(pub conjure_object::ByteBuf);
impl conjure_object::Plain for AliasedBinary {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        conjure_object::Plain::fmt(&self.0, fmt)
    }
}
impl std::ops::Deref for AliasedBinary {
    type Target = conjure_object::ByteBuf;
    #[inline]
    fn deref(&self) -> &conjure_object::ByteBuf {
        &self.0
    }
}
impl std::ops::DerefMut for AliasedBinary {
    #[inline]
    fn deref_mut(&mut self) -> &mut conjure_object::ByteBuf {
        &mut self.0
    }
}
impl ser::Serialize for AliasedBinary {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self.0.serialize(s)
    }
}
impl<'de> de::Deserialize<'de> for AliasedBinary {
    fn deserialize<D>(d: D) -> Result<AliasedBinary, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        de::Deserialize::deserialize(d).map(AliasedBinary)
    }
}
