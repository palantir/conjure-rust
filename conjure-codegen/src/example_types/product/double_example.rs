#[derive(
    Debug,
    Clone,
    conjure_object::serde::Serialize,
    conjure_object::serde::Deserialize,
    conjure_object::private::Educe,
    Copy
)]
#[serde(crate = "conjure_object::serde")]
#[educe(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[conjure_object::private::staged_builder::staged_builder]
#[builder(crate = conjure_object::private::staged_builder, update, inline)]
pub struct DoubleExample {
    #[serde(rename = "doubleValue")]
    #[educe(
        PartialEq(method(conjure_object::private::DoubleOps::eq)),
        Ord(method(conjure_object::private::DoubleOps::cmp)),
        Hash(method(conjure_object::private::DoubleOps::hash)),
    )]
    double_value: f64,
}
impl DoubleExample {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(double_value: f64) -> Self {
        Self::builder().double_value(double_value).build()
    }
    #[inline]
    pub fn double_value(&self) -> f64 {
        self.double_value
    }
}
