use conjure_types::serde::{de, ser};
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
pub struct HttpPath(pub String);
impl std::ops::Deref for HttpPath {
    type Target = String;
    #[inline]
    fn deref(&self) -> &String {
        &self.0
    }
}
impl std::ops::DerefMut for HttpPath {
    #[inline]
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}
impl ser::Serialize for HttpPath {
    fn serialize<S_>(&self, s: S_) -> Result<S_::Ok, S_::Error>
    where
        S_: ser::Serializer,
    {
        self.0.serialize(s)
    }
}
impl<'de> de::Deserialize<'de> for HttpPath {
    fn deserialize<D_>(d: D_) -> Result<HttpPath, D_::Error>
    where
        D_: de::Deserializer<'de>,
    {
        de::Deserialize::deserialize(d).map(HttpPath)
    }
}
