use conjure_object::serde::{de, ser};
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct BearerTokenAliasExample(pub conjure_object::BearerToken);
impl std::fmt::Display for BearerTokenAliasExample {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, fmt)
    }
}
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
