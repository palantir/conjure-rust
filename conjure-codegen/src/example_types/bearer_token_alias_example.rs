use conjure_object::serde::{de, ser};
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct BearerTokenAliasExample(pub conjure_object::BearerToken);
impl std::ops::Deref for BearerTokenAliasExample {
    type Target = conjure_object::BearerToken;
    #[inline]
    fn deref(&self) -> &conjure_object::BearerToken {
        &self.0
    }
}
impl std::ops::DerefMut for BearerTokenAliasExample {
    #[inline]
    fn deref_mut(&mut self) -> &mut conjure_object::BearerToken {
        &mut self.0
    }
}
impl ser::Serialize for BearerTokenAliasExample {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self.0.serialize(s)
    }
}
impl<'de> de::Deserialize<'de> for BearerTokenAliasExample {
    fn deserialize<D>(d: D) -> Result<BearerTokenAliasExample, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        de::Deserialize::deserialize(d).map(BearerTokenAliasExample)
    }
}
