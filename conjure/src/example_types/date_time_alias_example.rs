use crate::serde::{de, ser};
#[derive(Debug, Clone, PartialEq, PartialOrd, Copy, Eq, Ord, Hash)]
pub struct DateTimeAliasExample(pub crate::DateTime<crate::Utc>);
impl std::ops::Deref for DateTimeAliasExample {
    type Target = crate::DateTime<crate::Utc>;
    #[inline]
    fn deref(&self) -> &crate::DateTime<crate::Utc> {
        &self.0
    }
}
impl std::ops::DerefMut for DateTimeAliasExample {
    #[inline]
    fn deref_mut(&mut self) -> &mut crate::DateTime<crate::Utc> {
        &mut self.0
    }
}
impl ser::Serialize for DateTimeAliasExample {
    fn serialize<S_>(&self, s: S_) -> Result<S_::Ok, S_::Error>
    where
        S_: ser::Serializer,
    {
        self.0.serialize(s)
    }
}
impl<'de> de::Deserialize<'de> for DateTimeAliasExample {
    fn deserialize<D_>(d: D_) -> Result<DateTimeAliasExample, D_::Error>
    where
        D_: de::Deserializer<'de>,
    {
        de::Deserialize::deserialize(d).map(DateTimeAliasExample)
    }
}
