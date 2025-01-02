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
pub struct CovariantListExample {
    #[builder(
        default,
        list(
            item(
                custom(
                    type = impl
                    conjure_object::serde::Serialize,
                    convert = |v|conjure_object::Any::new(
                        v
                    ).expect("value failed to serialize")
                )
            )
        )
    )]
    #[serde(rename = "items", skip_serializing_if = "Vec :: is_empty", default)]
    items: Vec<conjure_object::Any>,
    #[builder(default, list(item(type = String, into)))]
    #[serde(rename = "externalItems", skip_serializing_if = "Vec :: is_empty", default)]
    external_items: Vec<String>,
}
impl CovariantListExample {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new() -> Self {
        Self::builder().build()
    }
    #[inline]
    pub fn items(&self) -> &[conjure_object::Any] {
        &*self.items
    }
    #[inline]
    pub fn external_items(&self) -> &[String] {
        &*self.external_items
    }
}
