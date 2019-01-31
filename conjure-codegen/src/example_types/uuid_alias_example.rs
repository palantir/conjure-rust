use conjure_object::serde::{de, ser};
#[derive(Debug, Clone, PartialEq, PartialOrd, Copy, Eq, Ord, Hash)]
pub struct UuidAliasExample(pub conjure_object::Uuid);
impl std::fmt::Display for UuidAliasExample {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, fmt)
    }
}
impl std::ops::Deref for UuidAliasExample {
    type Target = conjure_object::Uuid;
    #[inline]
    fn deref(&self) -> &conjure_object::Uuid {
        &self.0
    }
}
impl std::ops::DerefMut for UuidAliasExample {
    #[inline]
    fn deref_mut(&mut self) -> &mut conjure_object::Uuid {
        &mut self.0
    }
}
impl ser::Serialize for UuidAliasExample {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self.0.serialize(s)
    }
}
impl<'de> de::Deserialize<'de> for UuidAliasExample {
    fn deserialize<D>(d: D) -> Result<UuidAliasExample, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        de::Deserialize::deserialize(d).map(UuidAliasExample)
    }
}
