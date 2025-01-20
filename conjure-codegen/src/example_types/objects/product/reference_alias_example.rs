#[derive(
    Debug,
    Clone,
    conjure_object::serde::Deserialize,
    conjure_object::serde::Serialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash
)]
#[serde(crate = "conjure_object::serde", transparent)]
pub struct ReferenceAliasExample(pub super::AnyExample);
impl std::convert::From<super::AnyExample> for ReferenceAliasExample {
    #[inline]
    fn from(v: super::AnyExample) -> Self {
        ReferenceAliasExample(std::convert::From::from(v))
    }
}
impl std::ops::Deref for ReferenceAliasExample {
    type Target = super::AnyExample;
    #[inline]
    fn deref(&self) -> &super::AnyExample {
        &self.0
    }
}
impl std::ops::DerefMut for ReferenceAliasExample {
    #[inline]
    fn deref_mut(&mut self) -> &mut super::AnyExample {
        &mut self.0
    }
}
