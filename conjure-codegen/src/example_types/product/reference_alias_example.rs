use conjure_object::serde::{de, ser};
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ReferenceAliasExample(pub super::AnyExample);
impl std::ops::Deref for ReferenceAliasExample {
    type Target = super::AnyExample;
    #[inline]
    fn deref(&self) -> &super::AnyExample {
        &self.0
    }
}
impl std::ops::DerefMut for ReferenceAliasExample {
    #[inline]
    fn deref_mut(&mut self) -> &mut super::AnyExample {
        &mut self.0
    }
}
impl ser::Serialize for ReferenceAliasExample {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self.0.serialize(s)
    }
}
impl<'de> de::Deserialize<'de> for ReferenceAliasExample {
    fn deserialize<D>(d: D) -> Result<ReferenceAliasExample, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        de::Deserialize::deserialize(d).map(ReferenceAliasExample)
    }
}
