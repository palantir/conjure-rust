use crate::serde::{de, ser};
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
pub struct MapAliasExample(pub std::collections::BTreeMap<String, crate::Value>);
impl std::ops::Deref for MapAliasExample {
    type Target = std::collections::BTreeMap<String, crate::Value>;
    #[inline]
    fn deref(&self) -> &std::collections::BTreeMap<String, crate::Value> {
        &self.0
    }
}
impl std::ops::DerefMut for MapAliasExample {
    #[inline]
    fn deref_mut(&mut self) -> &mut std::collections::BTreeMap<String, crate::Value> {
        &mut self.0
    }
}
impl ser::Serialize for MapAliasExample {
    fn serialize<S_>(&self, s: S_) -> Result<S_::Ok, S_::Error>
    where
        S_: ser::Serializer,
    {
        self.0.serialize(s)
    }
}
impl<'de> de::Deserialize<'de> for MapAliasExample {
    fn deserialize<D_>(d: D_) -> Result<MapAliasExample, D_::Error>
    where
        D_: de::Deserializer<'de>,
    {
        de::Deserialize::deserialize(d).map(MapAliasExample)
    }
}
