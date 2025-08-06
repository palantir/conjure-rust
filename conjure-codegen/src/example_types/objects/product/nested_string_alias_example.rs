#[derive(
    Debug,
    Clone,
    conjure_object::serde::Deserialize,
    conjure_object::serde::Serialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Default
)]
#[serde(crate = "conjure_object::serde", transparent)]
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
impl std::convert::From<String> for NestedStringAliasExample {
    #[inline]
    fn from(v: String) -> Self {
        NestedStringAliasExample(std::convert::From::from(v))
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
impl std::convert::AsRef<String> for NestedStringAliasExample {
    #[inline]
    fn as_ref(&self) -> &String {
        &self.0
    }
}
