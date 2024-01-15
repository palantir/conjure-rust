use conjure_object::serde::{ser, de};
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct StringAliasExample(pub String);
impl std::fmt::Display for StringAliasExample {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, fmt)
    }
}
impl conjure_object::Plain for StringAliasExample {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        conjure_object::Plain::fmt(&self.0, fmt)
    }
}
impl conjure_object::FromPlain for StringAliasExample {
    type Err = <String as conjure_object::FromPlain>::Err;
    #[inline]
    fn from_plain(s: &str) -> Result<StringAliasExample, Self::Err> {
        conjure_object::FromPlain::from_plain(s).map(StringAliasExample)
    }
}
impl std::convert::From<String> for StringAliasExample {
    #[inline]
    fn from(v: String) -> Self {
        StringAliasExample(std::convert::From::from(v))
    }
}
impl std::ops::Deref for StringAliasExample {
    type Target = String;
    #[inline]
    fn deref(&self) -> &String {
        &self.0
    }
}
impl std::ops::DerefMut for StringAliasExample {
    #[inline]
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}
impl ser::Serialize for StringAliasExample {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self.0.serialize(s)
    }
}
impl<'de> de::Deserialize<'de> for StringAliasExample {
    fn deserialize<D>(d: D) -> Result<StringAliasExample, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        de::Deserialize::deserialize(d).map(StringAliasExample)
    }
}
