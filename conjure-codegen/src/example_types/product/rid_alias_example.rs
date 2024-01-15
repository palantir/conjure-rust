use conjure_object::serde::{ser, de};
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RidAliasExample(pub conjure_object::ResourceIdentifier);
impl std::fmt::Display for RidAliasExample {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, fmt)
    }
}
impl conjure_object::Plain for RidAliasExample {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        conjure_object::Plain::fmt(&self.0, fmt)
    }
}
impl conjure_object::FromPlain for RidAliasExample {
    type Err = <conjure_object::ResourceIdentifier as conjure_object::FromPlain>::Err;
    #[inline]
    fn from_plain(s: &str) -> Result<RidAliasExample, Self::Err> {
        conjure_object::FromPlain::from_plain(s).map(RidAliasExample)
    }
}
impl std::convert::From<conjure_object::ResourceIdentifier> for RidAliasExample {
    #[inline]
    fn from(v: conjure_object::ResourceIdentifier) -> Self {
        RidAliasExample(std::convert::From::from(v))
    }
}
impl std::ops::Deref for RidAliasExample {
    type Target = conjure_object::ResourceIdentifier;
    #[inline]
    fn deref(&self) -> &conjure_object::ResourceIdentifier {
        &self.0
    }
}
impl std::ops::DerefMut for RidAliasExample {
    #[inline]
    fn deref_mut(&mut self) -> &mut conjure_object::ResourceIdentifier {
        &mut self.0
    }
}
impl ser::Serialize for RidAliasExample {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self.0.serialize(s)
    }
}
impl<'de> de::Deserialize<'de> for RidAliasExample {
    fn deserialize<D>(d: D) -> Result<RidAliasExample, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        de::Deserialize::deserialize(d).map(RidAliasExample)
    }
}
