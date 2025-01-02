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
pub struct ObjectDefinition {
    #[builder(custom(type = super::TypeName, convert = Box::new))]
    #[serde(rename = "typeName")]
    type_name: Box<super::TypeName>,
    #[builder(default, list(item(type = super::FieldDefinition)))]
    #[serde(rename = "fields", skip_serializing_if = "Vec :: is_empty", default)]
    fields: Vec<super::FieldDefinition>,
    #[builder(default, into)]
    #[serde(rename = "docs", skip_serializing_if = "Option :: is_none", default)]
    docs: Option<super::Documentation>,
}
impl ObjectDefinition {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(type_name: super::TypeName) -> Self {
        Self::builder().type_name(type_name).build()
    }
    #[inline]
    pub fn type_name(&self) -> &super::TypeName {
        &*self.type_name
    }
    #[inline]
    pub fn fields(&self) -> &[super::FieldDefinition] {
        &*self.fields
    }
    #[inline]
    pub fn docs(&self) -> Option<&super::Documentation> {
        self.docs.as_ref().map(|o| &*o)
    }
}
