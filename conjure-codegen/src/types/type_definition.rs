use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeMap as SerializeMap_;
use conjure_object::private::{UnionField_, UnionTypeField_};
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TypeDefinition {
    Alias(super::AliasDefinition),
    Enum(super::EnumDefinition),
    Object(super::ObjectDefinition),
    Union(super::UnionDefinition),
}
impl ser::Serialize for TypeDefinition {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut map = s.serialize_map(Some(2))?;
        match self {
            TypeDefinition::Alias(value) => {
                map.serialize_entry(&"type", &"alias")?;
                map.serialize_entry(&"alias", value)?;
            }
            TypeDefinition::Enum(value) => {
                map.serialize_entry(&"type", &"enum")?;
                map.serialize_entry(&"enum", value)?;
            }
            TypeDefinition::Object(value) => {
                map.serialize_entry(&"type", &"object")?;
                map.serialize_entry(&"object", value)?;
            }
            TypeDefinition::Union(value) => {
                map.serialize_entry(&"type", &"union")?;
                map.serialize_entry(&"union", value)?;
            }
        }
        map.end()
    }
}
impl<'de> de::Deserialize<'de> for TypeDefinition {
    fn deserialize<D>(d: D) -> Result<TypeDefinition, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_map(Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = TypeDefinition;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("union TypeDefinition")
    }
    fn visit_map<A>(self, mut map: A) -> Result<TypeDefinition, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let v = match map.next_key::<UnionField_<Variant_>>()? {
            Some(UnionField_::Type) => {
                let variant = map.next_value()?;
                let key = map.next_key()?;
                match (variant, key) {
                    (Variant_::Alias, Some(Variant_::Alias)) => {
                        let value = map.next_value()?;
                        TypeDefinition::Alias(value)
                    }
                    (Variant_::Enum, Some(Variant_::Enum)) => {
                        let value = map.next_value()?;
                        TypeDefinition::Enum(value)
                    }
                    (Variant_::Object, Some(Variant_::Object)) => {
                        let value = map.next_value()?;
                        TypeDefinition::Object(value)
                    }
                    (Variant_::Union, Some(Variant_::Union)) => {
                        let value = map.next_value()?;
                        TypeDefinition::Union(value)
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
                    Variant_::Alias => {
                        let value = map.next_value()?;
                        TypeDefinition::Alias(value)
                    }
                    Variant_::Enum => {
                        let value = map.next_value()?;
                        TypeDefinition::Enum(value)
                    }
                    Variant_::Object => {
                        let value = map.next_value()?;
                        TypeDefinition::Object(value)
                    }
                    Variant_::Union => {
                        let value = map.next_value()?;
                        TypeDefinition::Union(value)
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
    Alias,
    Enum,
    Object,
    Union,
}
impl Variant_ {
    fn as_str(&self) -> &'static str {
        match *self {
            Variant_::Alias => "alias",
            Variant_::Enum => "enum",
            Variant_::Object => "object",
            Variant_::Union => "union",
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
            "alias" => Variant_::Alias,
            "enum" => Variant_::Enum,
            "object" => Variant_::Object,
            "union" => Variant_::Union,
            value => {
                return Err(
                    de::Error::unknown_variant(
                        value,
                        &["alias", "enum", "object", "union"],
                    ),
                );
            }
        };
        Ok(v)
    }
}
