use conjure_object::serde::{de, ser};
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
pub struct MapAliasExample(pub std::collections::BTreeMap<String, conjure_object::Value>);
impl std::ops::Deref for MapAliasExample {
    type Target = std::collections::BTreeMap<String, conjure_object::Value>;
    #[inline]
    fn deref(&self) -> &std::collections::BTreeMap<String, conjure_object::Value> {
        &self.0
    }
}
impl std::ops::DerefMut for MapAliasExample {
    #[inline]
    fn deref_mut(&mut self) -> &mut std::collections::BTreeMap<String, conjure_object::Value> {
        &mut self.0
    }
}
impl ser::Serialize for MapAliasExample {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self.0.serialize(s)
    }
}
impl<'de> de::Deserialize<'de> for MapAliasExample {
    fn deserialize<D>(d: D) -> Result<MapAliasExample, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        de::Deserialize::deserialize(d).map(MapAliasExample)
    }
}
