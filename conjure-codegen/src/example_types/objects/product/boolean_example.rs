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
pub struct BooleanExample {
    #[serde(rename = "coin")]
    coin: bool,
}
impl BooleanExample {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(coin: bool) -> Self {
        Self::builder().coin(coin).build()
    }
    #[inline]
    pub fn coin(&self) -> bool {
        self.coin
    }
}
