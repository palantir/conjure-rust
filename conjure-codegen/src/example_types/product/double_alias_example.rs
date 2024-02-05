use conjure_object::serde::{ser, de};
#[derive(Debug, Clone, Copy, conjure_object::private::Educe, Default)]
#[educe(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DoubleAliasExample(
    #[educe(
        PartialEq(method(conjure_object::private::DoubleOps::eq)),
        Ord(method(conjure_object::private::DoubleOps::cmp)),
        Hash(method(conjure_object::private::DoubleOps::hash)),
    )]
    pub f64,
);
impl std::fmt::Display for DoubleAliasExample {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, fmt)
    }
}
impl conjure_object::Plain for DoubleAliasExample {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        conjure_object::Plain::fmt(&self.0, fmt)
    }
}
impl conjure_object::FromPlain for DoubleAliasExample {
    type Err = <f64 as conjure_object::FromPlain>::Err;
    #[inline]
    fn from_plain(s: &str) -> Result<DoubleAliasExample, Self::Err> {
        conjure_object::FromPlain::from_plain(s).map(DoubleAliasExample)
    }
}
impl std::convert::From<f64> for DoubleAliasExample {
    #[inline]
    fn from(v: f64) -> Self {
        DoubleAliasExample(std::convert::From::from(v))
    }
}
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
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self.0.serialize(s)
    }
}
impl<'de> de::Deserialize<'de> for DoubleAliasExample {
    fn deserialize<D>(d: D) -> Result<DoubleAliasExample, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        de::Deserialize::deserialize(d).map(DoubleAliasExample)
    }
}
