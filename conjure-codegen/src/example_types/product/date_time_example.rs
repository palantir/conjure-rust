#[derive(
    Debug,
    Clone,
    conjure_object::serde::Serialize,
    conjure_object::serde::Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Copy
)]
#[serde(crate = "conjure_object::serde")]
#[conjure_object::private::staged_builder::staged_builder]
#[builder(crate = conjure_object::private::staged_builder, update, inline)]
pub struct DateTimeExample {
    #[serde(rename = "datetime")]
    datetime: conjure_object::DateTime<conjure_object::Utc>,
}
impl DateTimeExample {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(datetime: conjure_object::DateTime<conjure_object::Utc>) -> Self {
        Self::builder().datetime(datetime).build()
    }
    #[inline]
    pub fn datetime(&self) -> conjure_object::DateTime<conjure_object::Utc> {
        self.datetime
    }
}
