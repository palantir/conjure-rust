use conjure_object::serde::{de, ser};
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
pub struct NestedAliasedBinary(pub super::AliasedBinary);
impl conjure_object::Plain for NestedAliasedBinary {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        conjure_object::Plain::fmt(&self.0, fmt)
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
impl ser::Serialize for NestedAliasedBinary {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self.0.serialize(s)
    }
}
impl<'de> de::Deserialize<'de> for NestedAliasedBinary {
    fn deserialize<D>(d: D) -> Result<NestedAliasedBinary, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        de::Deserialize::deserialize(d).map(NestedAliasedBinary)
    }
}
