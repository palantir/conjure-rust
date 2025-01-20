#[derive(
    Debug,
    Clone,
    conjure_object::serde::Serialize,
    conjure_object::serde::Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash
)]
#[serde(crate = "conjure_object::serde")]
#[conjure_object::private::staged_builder::staged_builder]
#[builder(crate = conjure_object::private::staged_builder, update, inline)]
pub struct AnyExample {
    #[builder(
        custom(
            type = impl
            conjure_object::serde::Serialize,
            convert = |v|conjure_object::Any::new(v).expect("value failed to serialize")
        )
    )]
    #[serde(rename = "any")]
    any: conjure_object::Any,
}
impl AnyExample {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(any: impl conjure_object::serde::Serialize) -> Self {
        Self::builder().any(any).build()
    }
    #[inline]
    pub fn any(&self) -> &conjure_object::Any {
        &self.any
    }
}
