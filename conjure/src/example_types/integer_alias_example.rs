use crate::serde::{de, ser};
#[derive(Debug, Clone, PartialEq, PartialOrd, Copy, Eq, Ord, Hash, Default)]
pub struct IntegerAliasExample(pub i32);
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
impl ser::Serialize for IntegerAliasExample {
    fn serialize<S_>(&self, s: S_) -> Result<S_::Ok, S_::Error>
    where
        S_: ser::Serializer,
    {
        self.0.serialize(s)
    }
}
impl<'de> de::Deserialize<'de> for IntegerAliasExample {
    fn deserialize<D_>(d: D_) -> Result<IntegerAliasExample, D_::Error>
    where
        D_: de::Deserializer<'de>,
    {
        de::Deserialize::deserialize(d).map(IntegerAliasExample)
    }
}
