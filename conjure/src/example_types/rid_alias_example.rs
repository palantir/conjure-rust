use crate::serde::{de, ser};
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct RidAliasExample(pub crate::ResourceIdentifier);
impl std::ops::Deref for RidAliasExample {
    type Target = crate::ResourceIdentifier;
    #[inline]
    fn deref(&self) -> &crate::ResourceIdentifier {
        &self.0
    }
}
impl std::ops::DerefMut for RidAliasExample {
    #[inline]
    fn deref_mut(&mut self) -> &mut crate::ResourceIdentifier {
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
