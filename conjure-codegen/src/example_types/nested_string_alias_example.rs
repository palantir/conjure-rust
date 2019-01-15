use conjure_types::serde::{de, ser};
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
pub struct NestedStringAliasExample(pub super::StringAliasExample);
impl std::fmt::Display for NestedStringAliasExample {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, fmt)
    }
}
impl std::ops::Deref for NestedStringAliasExample {
    type Target = super::StringAliasExample;
    #[inline]
    fn deref(&self) -> &super::StringAliasExample {
        &self.0
    }
}
impl std::ops::DerefMut for NestedStringAliasExample {
    #[inline]
    fn deref_mut(&mut self) -> &mut super::StringAliasExample {
        &mut self.0
    }
}
impl ser::Serialize for NestedStringAliasExample {
    fn serialize<S_>(&self, s: S_) -> Result<S_::Ok, S_::Error>
    where
        S_: ser::Serializer,
    {
        self.0.serialize(s)
    }
}
impl<'de> de::Deserialize<'de> for NestedStringAliasExample {
    fn deserialize<D_>(d: D_) -> Result<NestedStringAliasExample, D_::Error>
    where
        D_: de::Deserializer<'de>,
    {
        de::Deserialize::deserialize(d).map(NestedStringAliasExample)
    }
}
