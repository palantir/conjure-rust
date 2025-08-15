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
pub struct IntegerExample {
    #[serde(rename = "integer")]
    integer: i32,
}
impl IntegerExample {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(integer: i32) -> Self {
        Self::builder().integer(integer).build()
    }
    #[inline]
    pub fn integer(&self) -> i32 {
        self.integer
    }
}
