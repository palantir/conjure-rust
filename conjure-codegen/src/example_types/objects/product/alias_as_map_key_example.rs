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
pub struct AliasAsMapKeyExample {
    #[builder(
        default,
        map(key(type = super::StringAliasExample), value(type = super::ManyFieldExample))
    )]
    #[serde(
        rename = "strings",
        skip_serializing_if = "std::collections::BTreeMap::is_empty",
        default
    )]
    strings: std::collections::BTreeMap<
        super::StringAliasExample,
        super::ManyFieldExample,
    >,
    #[builder(
        default,
        map(key(type = super::RidAliasExample), value(type = super::ManyFieldExample))
    )]
    #[serde(
        rename = "rids",
        skip_serializing_if = "std::collections::BTreeMap::is_empty",
        default
    )]
    rids: std::collections::BTreeMap<super::RidAliasExample, super::ManyFieldExample>,
    #[builder(
        default,
        map(
            key(type = super::BearerTokenAliasExample),
            value(type = super::ManyFieldExample)
        )
    )]
    #[serde(
        rename = "bearertokens",
        skip_serializing_if = "std::collections::BTreeMap::is_empty",
        default
    )]
    bearertokens: std::collections::BTreeMap<
        super::BearerTokenAliasExample,
        super::ManyFieldExample,
    >,
    #[builder(
        default,
        map(
            key(type = super::IntegerAliasExample),
            value(type = super::ManyFieldExample)
        )
    )]
    #[serde(
        rename = "integers",
        skip_serializing_if = "std::collections::BTreeMap::is_empty",
        default
    )]
    integers: std::collections::BTreeMap<
        super::IntegerAliasExample,
        super::ManyFieldExample,
    >,
    #[builder(
        default,
        map(
            key(type = super::SafeLongAliasExample),
            value(type = super::ManyFieldExample)
        )
    )]
    #[serde(
        rename = "safelongs",
        skip_serializing_if = "std::collections::BTreeMap::is_empty",
        default
    )]
    safelongs: std::collections::BTreeMap<
        super::SafeLongAliasExample,
        super::ManyFieldExample,
    >,
    #[builder(
        default,
        map(
            key(type = super::DateTimeAliasExample),
            value(type = super::ManyFieldExample)
        )
    )]
    #[serde(
        rename = "datetimes",
        skip_serializing_if = "std::collections::BTreeMap::is_empty",
        default
    )]
    datetimes: std::collections::BTreeMap<
        super::DateTimeAliasExample,
        super::ManyFieldExample,
    >,
    #[builder(
        default,
        map(key(type = super::UuidAliasExample), value(type = super::ManyFieldExample))
    )]
    #[serde(
        rename = "uuids",
        skip_serializing_if = "std::collections::BTreeMap::is_empty",
        default
    )]
    uuids: std::collections::BTreeMap<super::UuidAliasExample, super::ManyFieldExample>,
}
impl AliasAsMapKeyExample {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new() -> Self {
        Self::builder().build()
    }
    #[inline]
    pub fn strings(
        &self,
    ) -> &std::collections::BTreeMap<
        super::StringAliasExample,
        super::ManyFieldExample,
    > {
        &self.strings
    }
    #[inline]
    pub fn rids(
        &self,
    ) -> &std::collections::BTreeMap<super::RidAliasExample, super::ManyFieldExample> {
        &self.rids
    }
    #[inline]
    pub fn bearertokens(
        &self,
    ) -> &std::collections::BTreeMap<
        super::BearerTokenAliasExample,
        super::ManyFieldExample,
    > {
        &self.bearertokens
    }
    #[inline]
    pub fn integers(
        &self,
    ) -> &std::collections::BTreeMap<
        super::IntegerAliasExample,
        super::ManyFieldExample,
    > {
        &self.integers
    }
    #[inline]
    pub fn safelongs(
        &self,
    ) -> &std::collections::BTreeMap<
        super::SafeLongAliasExample,
        super::ManyFieldExample,
    > {
        &self.safelongs
    }
    #[inline]
    pub fn datetimes(
        &self,
    ) -> &std::collections::BTreeMap<
        super::DateTimeAliasExample,
        super::ManyFieldExample,
    > {
        &self.datetimes
    }
    #[inline]
    pub fn uuids(
        &self,
    ) -> &std::collections::BTreeMap<super::UuidAliasExample, super::ManyFieldExample> {
        &self.uuids
    }
}
