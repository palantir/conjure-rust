#[derive(
    Debug,
    Clone,
    conjure_object::serde::Deserialize,
    conjure_object::serde::Serialize,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash
)]
#[serde(crate = "conjure_object::serde", transparent)]
pub struct DateTimeAliasExample(pub conjure_object::DateTime<conjure_object::Utc>);
impl std::fmt::Display for DateTimeAliasExample {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, fmt)
    }
}
impl conjure_object::Plain for DateTimeAliasExample {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        conjure_object::Plain::fmt(&self.0, fmt)
    }
}
impl conjure_object::FromPlain for DateTimeAliasExample {
    type Err = <conjure_object::DateTime<
        conjure_object::Utc,
    > as conjure_object::FromPlain>::Err;
    #[inline]
    fn from_plain(s: &str) -> Result<DateTimeAliasExample, Self::Err> {
        conjure_object::FromPlain::from_plain(s).map(DateTimeAliasExample)
    }
}
impl std::convert::From<conjure_object::DateTime<conjure_object::Utc>>
for DateTimeAliasExample {
    #[inline]
    fn from(v: conjure_object::DateTime<conjure_object::Utc>) -> Self {
        DateTimeAliasExample(std::convert::From::from(v))
    }
}
impl std::ops::Deref for DateTimeAliasExample {
    type Target = conjure_object::DateTime<conjure_object::Utc>;
    #[inline]
    fn deref(&self) -> &conjure_object::DateTime<conjure_object::Utc> {
        &self.0
    }
}
impl std::ops::DerefMut for DateTimeAliasExample {
    #[inline]
    fn deref_mut(&mut self) -> &mut conjure_object::DateTime<conjure_object::Utc> {
        &mut self.0
    }
}
