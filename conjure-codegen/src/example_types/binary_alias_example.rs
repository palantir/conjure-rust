use conjure_object::serde::{de, ser};
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
pub struct BinaryAliasExample(pub conjure_object::ByteBuf);
impl std::ops::Deref for BinaryAliasExample {
    type Target = conjure_object::ByteBuf;
    #[inline]
    fn deref(&self) -> &conjure_object::ByteBuf {
        &self.0
    }
}
impl std::ops::DerefMut for BinaryAliasExample {
    #[inline]
    fn deref_mut(&mut self) -> &mut conjure_object::ByteBuf {
        &mut self.0
    }
}
impl ser::Serialize for BinaryAliasExample {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self.0.serialize(s)
    }
}
impl<'de> de::Deserialize<'de> for BinaryAliasExample {
    fn deserialize<D>(d: D) -> Result<BinaryAliasExample, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        de::Deserialize::deserialize(d).map(BinaryAliasExample)
    }
}
