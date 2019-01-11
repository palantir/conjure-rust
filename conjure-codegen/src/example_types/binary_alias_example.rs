use conjure_types::serde::{de, ser};
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
pub struct BinaryAliasExample(pub conjure_types::ByteBuf);
impl std::ops::Deref for BinaryAliasExample {
    type Target = conjure_types::ByteBuf;
    #[inline]
    fn deref(&self) -> &conjure_types::ByteBuf {
        &self.0
    }
}
impl std::ops::DerefMut for BinaryAliasExample {
    #[inline]
    fn deref_mut(&mut self) -> &mut conjure_types::ByteBuf {
        &mut self.0
    }
}
impl ser::Serialize for BinaryAliasExample {
    fn serialize<S_>(&self, s: S_) -> Result<S_::Ok, S_::Error>
    where
        S_: ser::Serializer,
    {
        self.0.serialize(s)
    }
}
impl<'de> de::Deserialize<'de> for BinaryAliasExample {
    fn deserialize<D_>(d: D_) -> Result<BinaryAliasExample, D_::Error>
    where
        D_: de::Deserializer<'de>,
    {
        de::Deserialize::deserialize(d).map(BinaryAliasExample)
    }
}
