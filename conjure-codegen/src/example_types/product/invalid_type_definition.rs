///Invalid Conjure type definition.
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
pub struct InvalidTypeDefinition {
    #[builder(into)]
    #[serde(rename = "typeName")]
    type_name: String,
    #[builder(
        custom(
            type = impl
            conjure_object::serde::Serialize,
            convert = |v|conjure_object::Any::new(v).expect("value failed to serialize")
        )
    )]
    #[serde(rename = "typeDef")]
    type_def: conjure_object::Any,
}
impl InvalidTypeDefinition {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(
        type_name: impl Into<String>,
        type_def: impl conjure_object::serde::Serialize,
    ) -> Self {
        Self::builder().type_name(type_name).type_def(type_def).build()
    }
    #[inline]
    pub fn type_name(&self) -> &str {
        &*self.type_name
    }
    #[inline]
    pub fn type_def(&self) -> &conjure_object::Any {
        &self.type_def
    }
}
impl conjure_error::ErrorType for InvalidTypeDefinition {
    #[inline]
    fn code() -> conjure_error::ErrorCode {
        conjure_error::ErrorCode::InvalidArgument
    }
    #[inline]
    fn name() -> &'static str {
        "Conjure:InvalidTypeDefinition"
    }
    #[inline]
    fn safe_args() -> &'static [&'static str] {
        &["typeName"]
    }
}
