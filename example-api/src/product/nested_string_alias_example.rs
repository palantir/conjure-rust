use conjure_object::serde::{de, ser};
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
pub struct NestedStringAliasExample(pub super::StringAliasExample);
impl std::fmt::Display for NestedStringAliasExample {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, fmt)
    }
}
impl conjure_object::Plain for NestedStringAliasExample {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        conjure_object::Plain::fmt(&self.0, fmt)
    }
}
impl conjure_object::FromPlain for NestedStringAliasExample {
    type Err = <super::StringAliasExample as conjure_object::FromPlain>::Err;
    #[inline]
    fn from_plain(s: &str) -> Result<NestedStringAliasExample, Self::Err> {
        conjure_object::FromPlain::from_plain(s).map(NestedStringAliasExample)
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
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self.0.serialize(s)
    }
}
impl<'de> de::Deserialize<'de> for NestedStringAliasExample {
    fn deserialize<D>(d: D) -> Result<NestedStringAliasExample, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        de::Deserialize::deserialize(d).map(NestedStringAliasExample)
    }
}
