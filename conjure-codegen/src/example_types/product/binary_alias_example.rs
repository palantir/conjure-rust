use conjure_object::serde::{ser, de};
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct BinaryAliasExample(pub conjure_object::Bytes);
impl conjure_object::Plain for BinaryAliasExample {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        conjure_object::Plain::fmt(&self.0, fmt)
    }
}
impl conjure_object::FromPlain for BinaryAliasExample {
    type Err = <conjure_object::Bytes as conjure_object::FromPlain>::Err;
    #[inline]
    fn from_plain(s: &str) -> Result<BinaryAliasExample, Self::Err> {
        conjure_object::FromPlain::from_plain(s).map(BinaryAliasExample)
    }
}
impl std::convert::From<conjure_object::Bytes> for BinaryAliasExample {
    #[inline]
    fn from(v: conjure_object::Bytes) -> Self {
        BinaryAliasExample(std::convert::From::from(v))
    }
}
impl std::ops::Deref for BinaryAliasExample {
    type Target = conjure_object::Bytes;
    #[inline]
    fn deref(&self) -> &conjure_object::Bytes {
        &self.0
    }
}
impl std::ops::DerefMut for BinaryAliasExample {
    #[inline]
    fn deref_mut(&mut self) -> &mut conjure_object::Bytes {
        &mut self.0
    }
}
impl ser::Serialize for BinaryAliasExample {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self.0.serialize(s)
    }
}
impl<'de> de::Deserialize<'de> for BinaryAliasExample {
    fn deserialize<D>(d: D) -> Result<BinaryAliasExample, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        de::Deserialize::deserialize(d).map(BinaryAliasExample)
    }
}
