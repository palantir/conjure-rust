use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeMap as SerializeMap_;
use conjure_object::private::{UnionField_, UnionTypeField_};
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ParameterType {
    Body(super::BodyParameterType),
    Header(super::HeaderParameterType),
    Path(super::PathParameterType),
    Query(super::QueryParameterType),
}
impl ser::Serialize for ParameterType {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut map = s.serialize_map(Some(2))?;
        match self {
            ParameterType::Body(value) => {
                map.serialize_entry(&"type", &"body")?;
                map.serialize_entry(&"body", value)?;
            }
            ParameterType::Header(value) => {
                map.serialize_entry(&"type", &"header")?;
                map.serialize_entry(&"header", value)?;
            }
            ParameterType::Path(value) => {
                map.serialize_entry(&"type", &"path")?;
                map.serialize_entry(&"path", value)?;
            }
            ParameterType::Query(value) => {
                map.serialize_entry(&"type", &"query")?;
                map.serialize_entry(&"query", value)?;
            }
        }
        map.end()
    }
}
impl<'de> de::Deserialize<'de> for ParameterType {
    fn deserialize<D>(d: D) -> Result<ParameterType, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_map(Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = ParameterType;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("union ParameterType")
    }
    fn visit_map<A>(self, mut map: A) -> Result<ParameterType, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let v = match map.next_key::<UnionField_<Variant_>>()? {
            Some(UnionField_::Type) => {
                let variant = map.next_value()?;
                let key = map.next_key()?;
                match (variant, key) {
                    (Variant_::Body, Some(Variant_::Body)) => {
                        let value = map.next_value()?;
                        ParameterType::Body(value)
                    }
                    (Variant_::Header, Some(Variant_::Header)) => {
                        let value = map.next_value()?;
                        ParameterType::Header(value)
                    }
                    (Variant_::Path, Some(Variant_::Path)) => {
                        let value = map.next_value()?;
                        ParameterType::Path(value)
                    }
                    (Variant_::Query, Some(Variant_::Query)) => {
                        let value = map.next_value()?;
                        ParameterType::Query(value)
                    }
                    (variant, Some(key)) => {
                        return Err(
                            de::Error::invalid_value(
                                de::Unexpected::Str(key.as_str()),
                                &variant.as_str(),
                            ),
                        );
                    }
                    (variant, None) => {
                        return Err(de::Error::missing_field(variant.as_str()));
                    }
                }
            }
            Some(UnionField_::Value(variant)) => {
                let value = match variant {
                    Variant_::Body => {
                        let value = map.next_value()?;
                        ParameterType::Body(value)
                    }
                    Variant_::Header => {
                        let value = map.next_value()?;
                        ParameterType::Header(value)
                    }
                    Variant_::Path => {
                        let value = map.next_value()?;
                        ParameterType::Path(value)
                    }
                    Variant_::Query => {
                        let value = map.next_value()?;
                        ParameterType::Query(value)
                    }
                };
                if map.next_key::<UnionTypeField_>()?.is_none() {
                    return Err(de::Error::missing_field("type"));
                }
                let type_variant = map.next_value::<Variant_>()?;
                if variant != type_variant {
                    return Err(
                        de::Error::invalid_value(
                            de::Unexpected::Str(type_variant.as_str()),
                            &variant.as_str(),
                        ),
                    );
                }
                value
            }
            None => return Err(de::Error::missing_field("type")),
        };
        if map.next_key::<UnionField_<Variant_>>()?.is_some() {
            return Err(de::Error::invalid_length(3, &"type and value fields"));
        }
        Ok(v)
    }
}
#[derive(PartialEq)]
enum Variant_ {
    Body,
    Header,
    Path,
    Query,
}
impl Variant_ {
    fn as_str(&self) -> &'static str {
        match *self {
            Variant_::Body => "body",
            Variant_::Header => "header",
            Variant_::Path => "path",
            Variant_::Query => "query",
        }
    }
}
impl<'de> de::Deserialize<'de> for Variant_ {
    fn deserialize<D>(d: D) -> Result<Variant_, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_str(VariantVisitor_)
    }
}
struct VariantVisitor_;
impl<'de> de::Visitor<'de> for VariantVisitor_ {
    type Value = Variant_;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("string")
    }
    fn visit_str<E>(self, value: &str) -> Result<Variant_, E>
    where
        E: de::Error,
    {
        let v = match value {
            "body" => Variant_::Body,
            "header" => Variant_::Header,
            "path" => Variant_::Path,
            "query" => Variant_::Query,
            value => {
                return Err(
                    de::Error::unknown_variant(
                        value,
                        &["body", "header", "path", "query"],
                    ),
                );
            }
        };
        Ok(v)
    }
}
