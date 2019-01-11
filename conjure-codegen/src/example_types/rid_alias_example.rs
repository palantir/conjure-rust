use conjure_types::serde::{de, ser};
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct RidAliasExample(pub conjure_types::ResourceIdentifier);
impl std::ops::Deref for RidAliasExample {
    type Target = conjure_types::ResourceIdentifier;
    #[inline]
    fn deref(&self) -> &conjure_types::ResourceIdentifier {
        &self.0
    }
}
impl std::ops::DerefMut for RidAliasExample {
    #[inline]
    fn deref_mut(&mut self) -> &mut conjure_types::ResourceIdentifier {
        &mut self.0
    }
}
impl ser::Serialize for RidAliasExample {
    fn serialize<S_>(&self, s: S_) -> Result<S_::Ok, S_::Error>
    where
        S_: ser::Serializer,
    {
        self.0.serialize(s)
    }
}
impl<'de> de::Deserialize<'de> for RidAliasExample {
    fn deserialize<D_>(d: D_) -> Result<RidAliasExample, D_::Error>
    where
        D_: de::Deserializer<'de>,
    {
        de::Deserialize::deserialize(d).map(RidAliasExample)
    }
}
