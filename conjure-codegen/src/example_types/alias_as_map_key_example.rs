use conjure_types::serde::ser::SerializeMap as SerializeMap_;
use conjure_types::serde::{de, ser};
use std::fmt;
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct AliasAsMapKeyExample {
    strings: std::collections::BTreeMap<super::StringAliasExample, super::ManyFieldExample>,
    rids: std::collections::BTreeMap<super::RidAliasExample, super::ManyFieldExample>,
    bearertokens:
        std::collections::BTreeMap<super::BearerTokenAliasExample, super::ManyFieldExample>,
    integers: std::collections::BTreeMap<super::IntegerAliasExample, super::ManyFieldExample>,
    safelongs: std::collections::BTreeMap<super::SafeLongAliasExample, super::ManyFieldExample>,
    datetimes: std::collections::BTreeMap<super::DateTimeAliasExample, super::ManyFieldExample>,
    uuids: std::collections::BTreeMap<super::UuidAliasExample, super::ManyFieldExample>,
}
impl AliasAsMapKeyExample {
    #[doc = r" Returns a new builder."]
    #[inline]
    pub fn builder() -> Builder {
        Default::default()
    }
    #[inline]
    pub fn strings(
        &self,
    ) -> &std::collections::BTreeMap<super::StringAliasExample, super::ManyFieldExample> {
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
    ) -> &std::collections::BTreeMap<super::BearerTokenAliasExample, super::ManyFieldExample> {
        &self.bearertokens
    }
    #[inline]
    pub fn integers(
        &self,
    ) -> &std::collections::BTreeMap<super::IntegerAliasExample, super::ManyFieldExample> {
        &self.integers
    }
    #[inline]
    pub fn safelongs(
        &self,
    ) -> &std::collections::BTreeMap<super::SafeLongAliasExample, super::ManyFieldExample> {
        &self.safelongs
    }
    #[inline]
    pub fn datetimes(
        &self,
    ) -> &std::collections::BTreeMap<super::DateTimeAliasExample, super::ManyFieldExample> {
        &self.datetimes
    }
    #[inline]
    pub fn uuids(
        &self,
    ) -> &std::collections::BTreeMap<super::UuidAliasExample, super::ManyFieldExample> {
        &self.uuids
    }
}
#[derive(Debug, Clone, Default)]
pub struct Builder {
    strings: std::collections::BTreeMap<super::StringAliasExample, super::ManyFieldExample>,
    rids: std::collections::BTreeMap<super::RidAliasExample, super::ManyFieldExample>,
    bearertokens:
        std::collections::BTreeMap<super::BearerTokenAliasExample, super::ManyFieldExample>,
    integers: std::collections::BTreeMap<super::IntegerAliasExample, super::ManyFieldExample>,
    safelongs: std::collections::BTreeMap<super::SafeLongAliasExample, super::ManyFieldExample>,
    datetimes: std::collections::BTreeMap<super::DateTimeAliasExample, super::ManyFieldExample>,
    uuids: std::collections::BTreeMap<super::UuidAliasExample, super::ManyFieldExample>,
}
impl Builder {
    pub fn strings<T>(&mut self, strings: T) -> &mut Self
    where
        T: IntoIterator<Item = (super::StringAliasExample, super::ManyFieldExample)>,
    {
        self.strings = strings.into_iter().collect();
        self
    }
    pub fn extend_strings<T>(&mut self, strings: T) -> &mut Self
    where
        T: IntoIterator<Item = (super::StringAliasExample, super::ManyFieldExample)>,
    {
        self.strings.extend(strings);
        self
    }
    pub fn rids<T>(&mut self, rids: T) -> &mut Self
    where
        T: IntoIterator<Item = (super::RidAliasExample, super::ManyFieldExample)>,
    {
        self.rids = rids.into_iter().collect();
        self
    }
    pub fn extend_rids<T>(&mut self, rids: T) -> &mut Self
    where
        T: IntoIterator<Item = (super::RidAliasExample, super::ManyFieldExample)>,
    {
        self.rids.extend(rids);
        self
    }
    pub fn bearertokens<T>(&mut self, bearertokens: T) -> &mut Self
    where
        T: IntoIterator<Item = (super::BearerTokenAliasExample, super::ManyFieldExample)>,
    {
        self.bearertokens = bearertokens.into_iter().collect();
        self
    }
    pub fn extend_bearertokens<T>(&mut self, bearertokens: T) -> &mut Self
    where
        T: IntoIterator<Item = (super::BearerTokenAliasExample, super::ManyFieldExample)>,
    {
        self.bearertokens.extend(bearertokens);
        self
    }
    pub fn integers<T>(&mut self, integers: T) -> &mut Self
    where
        T: IntoIterator<Item = (super::IntegerAliasExample, super::ManyFieldExample)>,
    {
        self.integers = integers.into_iter().collect();
        self
    }
    pub fn extend_integers<T>(&mut self, integers: T) -> &mut Self
    where
        T: IntoIterator<Item = (super::IntegerAliasExample, super::ManyFieldExample)>,
    {
        self.integers.extend(integers);
        self
    }
    pub fn safelongs<T>(&mut self, safelongs: T) -> &mut Self
    where
        T: IntoIterator<Item = (super::SafeLongAliasExample, super::ManyFieldExample)>,
    {
        self.safelongs = safelongs.into_iter().collect();
        self
    }
    pub fn extend_safelongs<T>(&mut self, safelongs: T) -> &mut Self
    where
        T: IntoIterator<Item = (super::SafeLongAliasExample, super::ManyFieldExample)>,
    {
        self.safelongs.extend(safelongs);
        self
    }
    pub fn datetimes<T>(&mut self, datetimes: T) -> &mut Self
    where
        T: IntoIterator<Item = (super::DateTimeAliasExample, super::ManyFieldExample)>,
    {
        self.datetimes = datetimes.into_iter().collect();
        self
    }
    pub fn extend_datetimes<T>(&mut self, datetimes: T) -> &mut Self
    where
        T: IntoIterator<Item = (super::DateTimeAliasExample, super::ManyFieldExample)>,
    {
        self.datetimes.extend(datetimes);
        self
    }
    pub fn uuids<T>(&mut self, uuids: T) -> &mut Self
    where
        T: IntoIterator<Item = (super::UuidAliasExample, super::ManyFieldExample)>,
    {
        self.uuids = uuids.into_iter().collect();
        self
    }
    pub fn extend_uuids<T>(&mut self, uuids: T) -> &mut Self
    where
        T: IntoIterator<Item = (super::UuidAliasExample, super::ManyFieldExample)>,
    {
        self.uuids.extend(uuids);
        self
    }
    #[doc = r" Constructs a new instance of the type."]
    #[doc = r""]
    #[doc = r" # Panics"]
    #[doc = r""]
    #[doc = r" Panics if a required field was not set."]
    #[inline]
    pub fn build(&self) -> AliasAsMapKeyExample {
        AliasAsMapKeyExample {
            strings: self.strings.clone(),
            rids: self.rids.clone(),
            bearertokens: self.bearertokens.clone(),
            integers: self.integers.clone(),
            safelongs: self.safelongs.clone(),
            datetimes: self.datetimes.clone(),
            uuids: self.uuids.clone(),
        }
    }
}
impl From<AliasAsMapKeyExample> for Builder {
    #[inline]
    fn from(_v: AliasAsMapKeyExample) -> Builder {
        Builder {
            strings: _v.strings,
            rids: _v.rids,
            bearertokens: _v.bearertokens,
            integers: _v.integers,
            safelongs: _v.safelongs,
            datetimes: _v.datetimes,
            uuids: _v.uuids,
        }
    }
}
impl ser::Serialize for AliasAsMapKeyExample {
    fn serialize<S_>(&self, s: S_) -> Result<S_::Ok, S_::Error>
    where
        S_: ser::Serializer,
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
        let mut map = s.serialize_map(Some(size))?;
        if !skip_strings {
            map.serialize_entry(&"strings", &self.strings)?;
        }
        if !skip_rids {
            map.serialize_entry(&"rids", &self.rids)?;
        }
        if !skip_bearertokens {
            map.serialize_entry(&"bearertokens", &self.bearertokens)?;
        }
        if !skip_integers {
            map.serialize_entry(&"integers", &self.integers)?;
        }
        if !skip_safelongs {
            map.serialize_entry(&"safelongs", &self.safelongs)?;
        }
        if !skip_datetimes {
            map.serialize_entry(&"datetimes", &self.datetimes)?;
        }
        if !skip_uuids {
            map.serialize_entry(&"uuids", &self.uuids)?;
        }
        map.end()
    }
}
impl<'de> de::Deserialize<'de> for AliasAsMapKeyExample {
    fn deserialize<D_>(d: D_) -> Result<AliasAsMapKeyExample, D_::Error>
    where
        D_: de::Deserializer<'de>,
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
    fn visit_map<A_>(self, mut map_: A_) -> Result<AliasAsMapKeyExample, A_::Error>
    where
        A_: de::MapAccess<'de>,
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
    fn deserialize<D_>(d: D_) -> Result<Field_, D_::Error>
    where
        D_: de::Deserializer<'de>,
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
    fn visit_str<E_>(self, value: &str) -> Result<Field_, E_>
    where
        E_: de::Error,
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
