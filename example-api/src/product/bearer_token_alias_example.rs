use conjure_object::serde::{ser, de};
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BearerTokenAliasExample(pub conjure_object::BearerToken);
impl conjure_object::Plain for BearerTokenAliasExample {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        conjure_object::Plain::fmt(&self.0, fmt)
    }
}
impl conjure_object::FromPlain for BearerTokenAliasExample {
    type Err = <conjure_object::BearerToken as conjure_object::FromPlain>::Err;
    #[inline]
    fn from_plain(s: &str) -> Result<BearerTokenAliasExample, Self::Err> {
        conjure_object::FromPlain::from_plain(s).map(BearerTokenAliasExample)
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
