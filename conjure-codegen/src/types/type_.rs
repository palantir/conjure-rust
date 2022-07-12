use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeMap as SerializeMap_;
use conjure_object::private::{UnionField_, UnionTypeField_};
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Type {
    Primitive(super::PrimitiveType),
    Optional(super::OptionalType),
    List(super::ListType),
    Set(super::SetType),
    Map(super::MapType),
    ///The name and package of a custom Conjure type. The custom type must be defined in the "types" section.
    Reference(super::TypeName),
    External(super::ExternalReference),
}
impl ser::Serialize for Type {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut map = s.serialize_map(Some(2))?;
        match self {
            Type::Primitive(value) => {
                map.serialize_entry(&"type", &"primitive")?;
                map.serialize_entry(&"primitive", value)?;
            }
            Type::Optional(value) => {
                map.serialize_entry(&"type", &"optional")?;
                map.serialize_entry(&"optional", value)?;
            }
            Type::List(value) => {
                map.serialize_entry(&"type", &"list")?;
                map.serialize_entry(&"list", value)?;
            }
            Type::Set(value) => {
                map.serialize_entry(&"type", &"set")?;
                map.serialize_entry(&"set", value)?;
            }
            Type::Map(value) => {
                map.serialize_entry(&"type", &"map")?;
                map.serialize_entry(&"map", value)?;
            }
            Type::Reference(value) => {
                map.serialize_entry(&"type", &"reference")?;
                map.serialize_entry(&"reference", value)?;
            }
            Type::External(value) => {
                map.serialize_entry(&"type", &"external")?;
                map.serialize_entry(&"external", value)?;
            }
        }
        map.end()
    }
}
impl<'de> de::Deserialize<'de> for Type {
    fn deserialize<D>(d: D) -> Result<Type, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_map(Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = Type;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("union Type")
    }
    fn visit_map<A>(self, mut map: A) -> Result<Type, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let v = match map.next_key::<UnionField_<Variant_>>()? {
            Some(UnionField_::Type) => {
                let variant = map.next_value()?;
                let key = map.next_key()?;
                match (variant, key) {
                    (Variant_::Primitive, Some(Variant_::Primitive)) => {
                        let value = map.next_value()?;
                        Type::Primitive(value)
                    }
                    (Variant_::Optional, Some(Variant_::Optional)) => {
                        let value = map.next_value()?;
                        Type::Optional(value)
                    }
                    (Variant_::List, Some(Variant_::List)) => {
                        let value = map.next_value()?;
                        Type::List(value)
                    }
                    (Variant_::Set, Some(Variant_::Set)) => {
                        let value = map.next_value()?;
                        Type::Set(value)
                    }
                    (Variant_::Map, Some(Variant_::Map)) => {
                        let value = map.next_value()?;
                        Type::Map(value)
                    }
                    (Variant_::Reference, Some(Variant_::Reference)) => {
                        let value = map.next_value()?;
                        Type::Reference(value)
                    }
                    (Variant_::External, Some(Variant_::External)) => {
                        let value = map.next_value()?;
                        Type::External(value)
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
                let value = match &variant {
                    Variant_::Primitive => {
                        let value = map.next_value()?;
                        Type::Primitive(value)
                    }
                    Variant_::Optional => {
                        let value = map.next_value()?;
                        Type::Optional(value)
                    }
                    Variant_::List => {
                        let value = map.next_value()?;
                        Type::List(value)
                    }
                    Variant_::Set => {
                        let value = map.next_value()?;
                        Type::Set(value)
                    }
                    Variant_::Map => {
                        let value = map.next_value()?;
                        Type::Map(value)
                    }
                    Variant_::Reference => {
                        let value = map.next_value()?;
                        Type::Reference(value)
                    }
                    Variant_::External => {
                        let value = map.next_value()?;
                        Type::External(value)
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
    Primitive,
    Optional,
    List,
    Set,
    Map,
    Reference,
    External,
}
impl Variant_ {
    fn as_str(&self) -> &'static str {
        match self {
            Variant_::Primitive => "primitive",
            Variant_::Optional => "optional",
            Variant_::List => "list",
            Variant_::Set => "set",
            Variant_::Map => "map",
            Variant_::Reference => "reference",
            Variant_::External => "external",
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
            "primitive" => Variant_::Primitive,
            "optional" => Variant_::Optional,
            "list" => Variant_::List,
            "set" => Variant_::Set,
            "map" => Variant_::Map,
            "reference" => Variant_::Reference,
            "external" => Variant_::External,
            value => {
                return Err(
                    de::Error::unknown_variant(
                        value,
                        &[
                            "primitive",
                            "optional",
                            "list",
                            "set",
                            "map",
                            "reference",
                            "external",
                        ],
                    ),
                );
            }
        };
        Ok(v)
    }
}
