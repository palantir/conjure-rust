use conjure_object::serde::{de, ser};
#[derive(Debug, Clone, PartialEq, PartialOrd, Copy, Eq, Ord, Hash, Default)]
pub struct BooleanAliasExample(pub bool);
impl std::fmt::Display for BooleanAliasExample {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, fmt)
    }
}
impl std::ops::Deref for BooleanAliasExample {
    type Target = bool;
    #[inline]
    fn deref(&self) -> &bool {
        &self.0
    }
}
impl std::ops::DerefMut for BooleanAliasExample {
    #[inline]
    fn deref_mut(&mut self) -> &mut bool {
        &mut self.0
    }
}
impl ser::Serialize for BooleanAliasExample {
    fn serialize<S_>(&self, s: S_) -> Result<S_::Ok, S_::Error>
    where
        S_: ser::Serializer,
    {
        self.0.serialize(s)
    }
}
impl<'de> de::Deserialize<'de> for BooleanAliasExample {
    fn deserialize<D_>(d: D_) -> Result<BooleanAliasExample, D_::Error>
    where
        D_: de::Deserializer<'de>,
    {
        de::Deserialize::deserialize(d).map(BooleanAliasExample)
    }
}
