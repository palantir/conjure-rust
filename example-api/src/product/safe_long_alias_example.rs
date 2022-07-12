use conjure_object::serde::{ser, de};
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SafeLongAliasExample(pub conjure_object::SafeLong);
impl std::fmt::Display for SafeLongAliasExample {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, fmt)
    }
}
impl conjure_object::Plain for SafeLongAliasExample {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        conjure_object::Plain::fmt(&self.0, fmt)
    }
}
impl conjure_object::FromPlain for SafeLongAliasExample {
    type Err = <conjure_object::SafeLong as conjure_object::FromPlain>::Err;
    #[inline]
    fn from_plain(s: &str) -> Result<SafeLongAliasExample, Self::Err> {
        conjure_object::FromPlain::from_plain(s).map(SafeLongAliasExample)
    }
}
impl std::ops::Deref for SafeLongAliasExample {
    type Target = conjure_object::SafeLong;
    #[inline]
    fn deref(&self) -> &conjure_object::SafeLong {
        &self.0
    }
}
impl std::ops::DerefMut for SafeLongAliasExample {
    #[inline]
    fn deref_mut(&mut self) -> &mut conjure_object::SafeLong {
        &mut self.0
    }
}
impl ser::Serialize for SafeLongAliasExample {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self.0.serialize(s)
    }
}
impl<'de> de::Deserialize<'de> for SafeLongAliasExample {
    fn deserialize<D>(d: D) -> Result<SafeLongAliasExample, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        de::Deserialize::deserialize(d).map(SafeLongAliasExample)
    }
}
