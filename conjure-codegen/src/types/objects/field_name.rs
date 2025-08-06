/// Should be in lowerCamelCase, but kebab-case and snake_case are also permitted.
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
pub struct FieldName(pub String);
impl std::fmt::Display for FieldName {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, fmt)
    }
}
impl conjure_object::Plain for FieldName {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        conjure_object::Plain::fmt(&self.0, fmt)
    }
}
impl conjure_object::FromPlain for FieldName {
    type Err = <String as conjure_object::FromPlain>::Err;
    #[inline]
    fn from_plain(s: &str) -> Result<FieldName, Self::Err> {
        conjure_object::FromPlain::from_plain(s).map(FieldName)
    }
}
impl std::convert::From<String> for FieldName {
    #[inline]
    fn from(v: String) -> Self {
        FieldName(std::convert::From::from(v))
    }
}
impl std::ops::Deref for FieldName {
    type Target = String;
    #[inline]
    fn deref(&self) -> &String {
        &self.0
    }
}
impl std::ops::DerefMut for FieldName {
    #[inline]
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}
impl std::convert::AsRef<String> for FieldName {
    #[inline]
    fn as_ref(&self) -> &String {
        &self.0
    }
}
