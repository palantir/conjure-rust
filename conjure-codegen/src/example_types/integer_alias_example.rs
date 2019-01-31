use conjure_object::serde::{de, ser};
#[derive(Debug, Clone, PartialEq, PartialOrd, Copy, Eq, Ord, Hash, Default)]
pub struct IntegerAliasExample(pub i32);
impl std::fmt::Display for IntegerAliasExample {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, fmt)
    }
}
impl std::ops::Deref for IntegerAliasExample {
    type Target = i32;
    #[inline]
    fn deref(&self) -> &i32 {
        &self.0
    }
}
impl std::ops::DerefMut for IntegerAliasExample {
    #[inline]
    fn deref_mut(&mut self) -> &mut i32 {
        &mut self.0
    }
}
impl ser::Serialize for IntegerAliasExample {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self.0.serialize(s)
    }
}
impl<'de> de::Deserialize<'de> for IntegerAliasExample {
    fn deserialize<D>(d: D) -> Result<IntegerAliasExample, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        de::Deserialize::deserialize(d).map(IntegerAliasExample)
    }
}
