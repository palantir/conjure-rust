use conjure_object::serde::{de, ser};
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct RidAliasExample(pub conjure_object::ResourceIdentifier);
impl std::fmt::Display for RidAliasExample {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, fmt)
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
