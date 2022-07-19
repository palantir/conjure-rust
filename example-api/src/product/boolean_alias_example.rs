use conjure_object::serde::{ser, de};
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct BooleanAliasExample(pub bool);
impl std::fmt::Display for BooleanAliasExample {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, fmt)
    }
}
impl conjure_object::Plain for BooleanAliasExample {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        conjure_object::Plain::fmt(&self.0, fmt)
    }
}
impl conjure_object::FromPlain for BooleanAliasExample {
    type Err = <bool as conjure_object::FromPlain>::Err;
    #[inline]
    fn from_plain(s: &str) -> Result<BooleanAliasExample, Self::Err> {
        conjure_object::FromPlain::from_plain(s).map(BooleanAliasExample)
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
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self.0.serialize(s)
    }
}
impl<'de> de::Deserialize<'de> for BooleanAliasExample {
    fn deserialize<D>(d: D) -> Result<BooleanAliasExample, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        de::Deserialize::deserialize(d).map(BooleanAliasExample)
    }
}
