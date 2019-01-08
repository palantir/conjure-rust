use crate::serde::{de, ser};
#[derive(Debug, Clone, PartialEq, PartialOrd, Copy, Eq, Ord, Hash)]
pub struct UuidAliasExample(pub crate::Uuid);
impl std::ops::Deref for UuidAliasExample {
    type Target = crate::Uuid;
    #[inline]
    fn deref(&self) -> &crate::Uuid {
        &self.0
    }
}
impl std::ops::DerefMut for UuidAliasExample {
    #[inline]
    fn deref_mut(&mut self) -> &mut crate::Uuid {
        &mut self.0
    }
}
impl ser::Serialize for UuidAliasExample {
    fn serialize<S_>(&self, s: S_) -> Result<S_::Ok, S_::Error>
    where
        S_: ser::Serializer,
    {
        self.0.serialize(s)
    }
}
impl<'de> de::Deserialize<'de> for UuidAliasExample {
    fn deserialize<D_>(d: D_) -> Result<UuidAliasExample, D_::Error>
    where
        D_: de::Deserializer<'de>,
    {
        de::Deserialize::deserialize(d).map(UuidAliasExample)
    }
}
