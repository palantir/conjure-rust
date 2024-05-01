use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, conjure_object::private::Educe)]
#[educe(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[conjure_object::private::staged_builder::staged_builder]
#[builder(crate = conjure_object::private::staged_builder, update, inline)]
pub struct AliasAsMapKeyExample {
    #[builder(
        default,
        map(key(type = super::StringAliasExample), value(type = super::ManyFieldExample))
    )]
    strings: std::collections::BTreeMap<
        super::StringAliasExample,
        super::ManyFieldExample,
    >,
    #[builder(
        default,
        map(key(type = super::RidAliasExample), value(type = super::ManyFieldExample))
    )]
    rids: std::collections::BTreeMap<super::RidAliasExample, super::ManyFieldExample>,
    #[builder(
        default,
        map(
            key(type = super::BearerTokenAliasExample),
            value(type = super::ManyFieldExample)
        )
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
    datetimes: std::collections::BTreeMap<
        super::DateTimeAliasExample,
        super::ManyFieldExample,
    >,
    #[builder(
        default,
        map(key(type = super::UuidAliasExample), value(type = super::ManyFieldExample))
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
impl ser::Serialize for AliasAsMapKeyExample {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut size = 0usize;
        let skip_strings = self.strings.is_empty();
        if !skip_strings {
            size += 1;
        }
        let skip_rids = self.rids.is_empty();
        if !skip_rids {
            size += 1;
        }
        let skip_bearertokens = self.bearertokens.is_empty();
        if !skip_bearertokens {
            size += 1;
        }
        let skip_integers = self.integers.is_empty();
        if !skip_integers {
            size += 1;
        }
        let skip_safelongs = self.safelongs.is_empty();
        if !skip_safelongs {
            size += 1;
        }
        let skip_datetimes = self.datetimes.is_empty();
        if !skip_datetimes {
            size += 1;
        }
        let skip_uuids = self.uuids.is_empty();
        if !skip_uuids {
            size += 1;
        }
        let mut s = s.serialize_struct("AliasAsMapKeyExample", size)?;
        if skip_strings {
            s.skip_field("strings")?;
        } else {
            s.serialize_field("strings", &self.strings)?;
        }
        if skip_rids {
            s.skip_field("rids")?;
        } else {
            s.serialize_field("rids", &self.rids)?;
        }
        if skip_bearertokens {
            s.skip_field("bearertokens")?;
        } else {
            s.serialize_field("bearertokens", &self.bearertokens)?;
        }
        if skip_integers {
            s.skip_field("integers")?;
        } else {
            s.serialize_field("integers", &self.integers)?;
        }
        if skip_safelongs {
            s.skip_field("safelongs")?;
        } else {
            s.serialize_field("safelongs", &self.safelongs)?;
        }
        if skip_datetimes {
            s.skip_field("datetimes")?;
        } else {
            s.serialize_field("datetimes", &self.datetimes)?;
        }
        if skip_uuids {
            s.skip_field("uuids")?;
        } else {
            s.serialize_field("uuids", &self.uuids)?;
        }
        s.end()
    }
}
impl<'de> de::Deserialize<'de> for AliasAsMapKeyExample {
    fn deserialize<D>(d: D) -> Result<AliasAsMapKeyExample, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct(
            "AliasAsMapKeyExample",
            &[
                "strings",
                "rids",
                "bearertokens",
                "integers",
                "safelongs",
                "datetimes",
                "uuids",
            ],
            Visitor_,
        )
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = AliasAsMapKeyExample;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<AliasAsMapKeyExample, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut strings = None;
        let mut rids = None;
        let mut bearertokens = None;
        let mut integers = None;
        let mut safelongs = None;
        let mut datetimes = None;
        let mut uuids = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::Strings => strings = Some(map_.next_value()?),
                Field_::Rids => rids = Some(map_.next_value()?),
                Field_::Bearertokens => bearertokens = Some(map_.next_value()?),
                Field_::Integers => integers = Some(map_.next_value()?),
                Field_::Safelongs => safelongs = Some(map_.next_value()?),
                Field_::Datetimes => datetimes = Some(map_.next_value()?),
                Field_::Uuids => uuids = Some(map_.next_value()?),
                Field_::Unknown_ => {
                    map_.next_value::<de::IgnoredAny>()?;
                }
            }
        }
        let strings = match strings {
            Some(v) => v,
            None => Default::default(),
        };
        let rids = match rids {
            Some(v) => v,
            None => Default::default(),
        };
        let bearertokens = match bearertokens {
            Some(v) => v,
            None => Default::default(),
        };
        let integers = match integers {
            Some(v) => v,
            None => Default::default(),
        };
        let safelongs = match safelongs {
            Some(v) => v,
            None => Default::default(),
        };
        let datetimes = match datetimes {
            Some(v) => v,
            None => Default::default(),
        };
        let uuids = match uuids {
            Some(v) => v,
            None => Default::default(),
        };
        Ok(AliasAsMapKeyExample {
            strings,
            rids,
            bearertokens,
            integers,
            safelongs,
            datetimes,
            uuids,
        })
    }
}
enum Field_ {
    Strings,
    Rids,
    Bearertokens,
    Integers,
    Safelongs,
    Datetimes,
    Uuids,
    Unknown_,
}
impl<'de> de::Deserialize<'de> for Field_ {
    fn deserialize<D>(d: D) -> Result<Field_, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_str(FieldVisitor_)
    }
}
struct FieldVisitor_;
impl<'de> de::Visitor<'de> for FieldVisitor_ {
    type Value = Field_;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("string")
    }
    fn visit_str<E>(self, value: &str) -> Result<Field_, E>
    where
        E: de::Error,
    {
        let v = match value {
            "strings" => Field_::Strings,
            "rids" => Field_::Rids,
            "bearertokens" => Field_::Bearertokens,
            "integers" => Field_::Integers,
            "safelongs" => Field_::Safelongs,
            "datetimes" => Field_::Datetimes,
            "uuids" => Field_::Uuids,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
