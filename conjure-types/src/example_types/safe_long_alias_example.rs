use crate::serde::{de, ser};
#[derive(Debug, Clone, PartialEq, PartialOrd, Copy, Eq, Ord, Hash, Default)]
pub struct SafeLongAliasExample(pub crate::SafeLong);
impl std::ops::Deref for SafeLongAliasExample {
    type Target = crate::SafeLong;
    #[inline]
    fn deref(&self) -> &crate::SafeLong {
        &self.0
    }
}
impl std::ops::DerefMut for SafeLongAliasExample {
    #[inline]
    fn deref_mut(&mut self) -> &mut crate::SafeLong {
        &mut self.0
    }
}
impl ser::Serialize for SafeLongAliasExample {
    fn serialize<S_>(&self, s: S_) -> Result<S_::Ok, S_::Error>
    where
        S_: ser::Serializer,
    {
        self.0.serialize(s)
    }
}
impl<'de> de::Deserialize<'de> for SafeLongAliasExample {
    fn deserialize<D_>(d: D_) -> Result<SafeLongAliasExample, D_::Error>
    where
        D_: de::Deserializer<'de>,
    {
        de::Deserialize::deserialize(d).map(SafeLongAliasExample)
    }
}
