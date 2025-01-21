#[derive(
    Debug,
    Clone,
    conjure_object::serde::Serialize,
    conjure_object::serde::Deserialize,
    conjure_object::private::Educe
)]
#[serde(crate = "conjure_object::serde")]
#[educe(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[conjure_object::private::staged_builder::staged_builder]
#[builder(crate = conjure_object::private::staged_builder, update, inline)]
pub struct PrimitiveOptionalsExample {
    #[builder(default, into)]
    #[serde(rename = "num", default)]
    #[educe(
        PartialEq(method(conjure_object::private::DoubleOps::eq)),
        Ord(method(conjure_object::private::DoubleOps::cmp)),
        Hash(method(conjure_object::private::DoubleOps::hash)),
    )]
    num: Option<f64>,
    #[builder(default, into)]
    #[serde(rename = "bool", default)]
    bool: Option<bool>,
    #[builder(default, into)]
    #[serde(rename = "integer", default)]
    integer: Option<i32>,
    #[builder(default, into)]
    #[serde(rename = "safelong", default)]
    safelong: Option<conjure_object::SafeLong>,
    #[builder(default, into)]
    #[serde(rename = "rid", default)]
    rid: Option<conjure_object::ResourceIdentifier>,
    #[builder(default, into)]
    #[serde(rename = "bearertoken", default)]
    bearertoken: Option<conjure_object::BearerToken>,
    #[builder(default, into)]
    #[serde(rename = "uuid", default)]
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
