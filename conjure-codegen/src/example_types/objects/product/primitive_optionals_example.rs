#[derive(
    Debug,
    Clone,
    conjure_object::serde::Serialize,
    conjure_object::serde::Deserialize,
    conjure_object::private::DeriveWith
)]
#[serde(crate = "conjure_object::serde")]
#[derive_with(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[conjure_object::private::staged_builder::staged_builder]
#[builder(crate = conjure_object::private::staged_builder, update, inline)]
pub struct PrimitiveOptionalsExample {
    #[builder(default, into)]
    #[serde(rename = "num", skip_serializing_if = "Option::is_none", default)]
    #[derive_with(with = conjure_object::private::DoubleWrapper)]
    num: Option<f64>,
    #[builder(default, into)]
    #[serde(rename = "bool", skip_serializing_if = "Option::is_none", default)]
    bool: Option<bool>,
    #[builder(default, into)]
    #[serde(rename = "integer", skip_serializing_if = "Option::is_none", default)]
    integer: Option<i32>,
    #[builder(default, into)]
    #[serde(rename = "safelong", skip_serializing_if = "Option::is_none", default)]
    safelong: Option<conjure_object::SafeLong>,
    #[builder(default, into)]
    #[serde(rename = "rid", skip_serializing_if = "Option::is_none", default)]
    rid: Option<conjure_object::ResourceIdentifier>,
    #[builder(default, into)]
    #[serde(rename = "bearertoken", skip_serializing_if = "Option::is_none", default)]
    bearertoken: Option<conjure_object::BearerToken>,
    #[builder(default, into)]
    #[serde(rename = "uuid", skip_serializing_if = "Option::is_none", default)]
    uuid: Option<conjure_object::Uuid>,
}
impl PrimitiveOptionalsExample {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new() -> Self {
        Self::builder().build()
    }
    #[inline]
    pub fn num(&self) -> Option<f64> {
        self.num.as_ref().map(|o| *o)
    }
    #[inline]
    pub fn bool(&self) -> Option<bool> {
        self.bool.as_ref().map(|o| *o)
    }
    #[inline]
    pub fn integer(&self) -> Option<i32> {
        self.integer.as_ref().map(|o| *o)
    }
    #[inline]
    pub fn safelong(&self) -> Option<conjure_object::SafeLong> {
        self.safelong.as_ref().map(|o| *o)
    }
    #[inline]
    pub fn rid(&self) -> Option<&conjure_object::ResourceIdentifier> {
        self.rid.as_ref().map(|o| &*o)
    }
    #[inline]
    pub fn bearertoken(&self) -> Option<&conjure_object::BearerToken> {
        self.bearertoken.as_ref().map(|o| &*o)
    }
    #[inline]
    pub fn uuid(&self) -> Option<conjure_object::Uuid> {
        self.uuid.as_ref().map(|o| *o)
    }
}
