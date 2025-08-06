#[derive(
    Debug,
    Clone,
    conjure_object::serde::Deserialize,
    conjure_object::serde::Serialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Default
)]
#[serde(crate = "conjure_object::serde", transparent)]
pub struct MapAliasExample(pub std::collections::BTreeMap<String, conjure_object::Any>);
impl std::iter::FromIterator<(String, conjure_object::Any)> for MapAliasExample {
    fn from_iter<T>(iter: T) -> Self
    where
        T: std::iter::IntoIterator<Item = (String, conjure_object::Any)>,
    {
        MapAliasExample(std::iter::FromIterator::from_iter(iter))
    }
}
impl std::convert::From<std::collections::BTreeMap<String, conjure_object::Any>>
for MapAliasExample {
    #[inline]
    fn from(v: std::collections::BTreeMap<String, conjure_object::Any>) -> Self {
        MapAliasExample(std::convert::From::from(v))
    }
}
impl std::ops::Deref for MapAliasExample {
    type Target = std::collections::BTreeMap<String, conjure_object::Any>;
    #[inline]
    fn deref(&self) -> &std::collections::BTreeMap<String, conjure_object::Any> {
        &self.0
    }
}
impl std::ops::DerefMut for MapAliasExample {
    #[inline]
    fn deref_mut(
        &mut self,
    ) -> &mut std::collections::BTreeMap<String, conjure_object::Any> {
        &mut self.0
    }
}
impl std::convert::AsRef<std::collections::BTreeMap<String, conjure_object::Any>>
for MapAliasExample {
    #[inline]
    fn as_ref(&self) -> &std::collections::BTreeMap<String, conjure_object::Any> {
        &self.0
    }
}
