use crate::serde::{de, ser};
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct BearerTokenAliasExample(pub crate::BearerToken);
impl std::ops::Deref for BearerTokenAliasExample {
    type Target = crate::BearerToken;
    #[inline]
    fn deref(&self) -> &crate::BearerToken {
        &self.0
    }
}
impl std::ops::DerefMut for BearerTokenAliasExample {
    #[inline]
    fn deref_mut(&mut self) -> &mut crate::BearerToken {
        &mut self.0
    }
}
impl ser::Serialize for BearerTokenAliasExample {
    fn serialize<S_>(&self, s: S_) -> Result<S_::Ok, S_::Error>
    where
        S_: ser::Serializer,
    {
        self.0.serialize(s)
    }
}
impl<'de> de::Deserialize<'de> for BearerTokenAliasExample {
    fn deserialize<D_>(d: D_) -> Result<BearerTokenAliasExample, D_::Error>
    where
        D_: de::Deserializer<'de>,
    {
        de::Deserialize::deserialize(d).map(BearerTokenAliasExample)
    }
}
