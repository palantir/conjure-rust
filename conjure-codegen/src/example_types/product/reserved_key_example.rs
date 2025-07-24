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
pub struct ReservedKeyExample {
    #[builder(into)]
    #[serde(rename = "package")]
    package: String,
    #[builder(into)]
    #[serde(rename = "interface")]
    interface: String,
    #[builder(into)]
    #[serde(rename = "field-name-with-dashes")]
    field_name_with_dashes: String,
    #[serde(rename = "primitve-field-name-with-dashes")]
    primitve_field_name_with_dashes: i32,
    #[serde(rename = "memoizedHashCode")]
    memoized_hash_code: i32,
    #[builder(into)]
    #[serde(rename = "build")]
    build_: String,
}
impl ReservedKeyExample {
    #[inline]
    pub fn package(&self) -> &str {
        &*self.package
    }
    #[inline]
    pub fn interface(&self) -> &str {
        &*self.interface
    }
    #[inline]
    pub fn field_name_with_dashes(&self) -> &str {
        &*self.field_name_with_dashes
    }
    #[inline]
    pub fn primitve_field_name_with_dashes(&self) -> i32 {
        self.primitve_field_name_with_dashes
    }
    #[inline]
    pub fn memoized_hash_code(&self) -> i32 {
        self.memoized_hash_code
    }
    #[inline]
    pub fn build_(&self) -> &str {
        &*self.build_
    }
}
