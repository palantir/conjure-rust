use crate::serde::{de, ser};
#[derive(Debug, Clone, PartialEq, PartialOrd, Copy, Default)]
pub struct DoubleAliasExample(pub f64);
impl std::ops::Deref for DoubleAliasExample {
    type Target = f64;
    #[inline]
    fn deref(&self) -> &f64 {
        &self.0
    }
}
impl std::ops::DerefMut for DoubleAliasExample {
    #[inline]
    fn deref_mut(&mut self) -> &mut f64 {
        &mut self.0
    }
}
impl ser::Serialize for DoubleAliasExample {
    fn serialize<S_>(&self, s: S_) -> Result<S_::Ok, S_::Error>
    where
        S_: ser::Serializer,
    {
        self.0.serialize(s)
    }
}
impl<'de> de::Deserialize<'de> for DoubleAliasExample {
    fn deserialize<D_>(d: D_) -> Result<DoubleAliasExample, D_::Error>
    where
        D_: de::Deserializer<'de>,
    {
        de::Deserialize::deserialize(d).map(DoubleAliasExample)
    }
}
