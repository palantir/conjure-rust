///A generic `CONFLICT` error.
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
pub struct Conflict {}
impl Conflict {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new() -> Self {
        Self::builder().build()
    }
}
impl conjure_error::ErrorType for Conflict {
    #[inline]
    fn code() -> conjure_error::ErrorCode {
        conjure_error::ErrorCode::Conflict
    }
    #[inline]
    fn name() -> &'static str {
        "Default:Conflict"
    }
    #[inline]
    fn safe_args() -> &'static [&'static str] {
        &[]
    }
}
