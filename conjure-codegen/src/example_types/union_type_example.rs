use conjure_object::private::{UnionField_, UnionTypeField_};
use conjure_object::serde::ser::SerializeMap as SerializeMap_;
use conjure_object::serde::{de, ser};
use std::fmt;
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum UnionTypeExample {
    #[doc = "Docs for when UnionTypeExample is of type StringExample."]
    StringExample(super::StringExample),
    Set(std::collections::BTreeSet<String>),
    ThisFieldIsAnInteger(i32),
    AlsoAnInteger(i32),
    If(i32),
    New(i32),
    Interface(i32),
    #[doc = r" An unknown variant."]
    Unknown(Unknown),
}
impl ser::Serialize for UnionTypeExample {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut map = s.serialize_map(Some(2))?;
        match self {
            UnionTypeExample::StringExample(value) => {
                map.serialize_entry(&"type", &"stringExample")?;
                map.serialize_entry(&"stringExample", value)?;
            }
            UnionTypeExample::Set(value) => {
                map.serialize_entry(&"type", &"set")?;
                map.serialize_entry(&"set", value)?;
            }
            UnionTypeExample::ThisFieldIsAnInteger(value) => {
                map.serialize_entry(&"type", &"thisFieldIsAnInteger")?;
                map.serialize_entry(&"thisFieldIsAnInteger", value)?;
            }
            UnionTypeExample::AlsoAnInteger(value) => {
                map.serialize_entry(&"type", &"alsoAnInteger")?;
                map.serialize_entry(&"alsoAnInteger", value)?;
            }
            UnionTypeExample::If(value) => {
                map.serialize_entry(&"type", &"if")?;
                map.serialize_entry(&"if", value)?;
            }
            UnionTypeExample::New(value) => {
                map.serialize_entry(&"type", &"new")?;
                map.serialize_entry(&"new", value)?;
            }
            UnionTypeExample::Interface(value) => {
                map.serialize_entry(&"type", &"interface")?;
                map.serialize_entry(&"interface", value)?;
            }
            UnionTypeExample::Unknown(value) => {
                map.serialize_entry(&"type", &value.type_)?;
                map.serialize_entry(&value.type_, &value.value)?;
            }
        }
        map.end()
    }
}
impl<'de> de::Deserialize<'de> for UnionTypeExample {
    fn deserialize<D>(d: D) -> Result<UnionTypeExample, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_map(Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = UnionTypeExample;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("union UnionTypeExample")
    }
    fn visit_map<A>(self, mut map: A) -> Result<UnionTypeExample, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let v = match map.next_key::<UnionField_<Variant_>>()? {
            Some(UnionField_::Type) => {
                let variant = map.next_value()?;
                let key = map.next_key()?;
                match (variant, key) {
                    (Variant_::StringExample, Some(Variant_::StringExample)) => {
                        let value = map.next_value()?;
                        UnionTypeExample::StringExample(value)
                    }
                    (Variant_::Set, Some(Variant_::Set)) => {
                        let value = map.next_value()?;
                        UnionTypeExample::Set(value)
                    }
                    (Variant_::ThisFieldIsAnInteger, Some(Variant_::ThisFieldIsAnInteger)) => {
                        let value = map.next_value()?;
                        UnionTypeExample::ThisFieldIsAnInteger(value)
                    }
                    (Variant_::AlsoAnInteger, Some(Variant_::AlsoAnInteger)) => {
                        let value = map.next_value()?;
                        UnionTypeExample::AlsoAnInteger(value)
                    }
                    (Variant_::If, Some(Variant_::If)) => {
                        let value = map.next_value()?;
                        UnionTypeExample::If(value)
                    }
                    (Variant_::New, Some(Variant_::New)) => {
                        let value = map.next_value()?;
                        UnionTypeExample::New(value)
                    }
                    (Variant_::Interface, Some(Variant_::Interface)) => {
                        let value = map.next_value()?;
                        UnionTypeExample::Interface(value)
                    }
                    (Variant_::Unknown(type_), Some(Variant_::Unknown(b))) => {
                        if type_ == b {
                            let value = map.next_value()?;
                            UnionTypeExample::Unknown(Unknown { type_, value })
                        } else {
                            return Err(de::Error::invalid_value(de::Unexpected::Str(&type_), &&*b));
                        }
                    }
                    (variant, Some(key)) => {
                        return Err(de::Error::invalid_value(
                            de::Unexpected::Str(key.as_str()),
                            &variant.as_str(),
                        ));
                    }
                    (variant, None) => return Err(de::Error::missing_field(variant.as_str())),
                }
            }
            Some(UnionField_::Value(variant)) => {
                let value = match &variant {
                    Variant_::StringExample => {
                        let value = map.next_value()?;
                        UnionTypeExample::StringExample(value)
                    }
                    Variant_::Set => {
                        let value = map.next_value()?;
                        UnionTypeExample::Set(value)
                    }
                    Variant_::ThisFieldIsAnInteger => {
                        let value = map.next_value()?;
                        UnionTypeExample::ThisFieldIsAnInteger(value)
                    }
                    Variant_::AlsoAnInteger => {
                        let value = map.next_value()?;
                        UnionTypeExample::AlsoAnInteger(value)
                    }
                    Variant_::If => {
                        let value = map.next_value()?;
                        UnionTypeExample::If(value)
                    }
                    Variant_::New => {
                        let value = map.next_value()?;
                        UnionTypeExample::New(value)
                    }
                    Variant_::Interface => {
                        let value = map.next_value()?;
                        UnionTypeExample::Interface(value)
                    }
                    Variant_::Unknown(type_) => {
                        let value = map.next_value()?;
                        UnionTypeExample::Unknown(Unknown {
                            type_: type_.clone(),
                            value,
                        })
                    }
                };
                if map.next_key::<UnionTypeField_>()?.is_none() {
                    return Err(de::Error::missing_field("type"));
                }
                let type_variant = map.next_value::<Variant_>()?;
                if variant != type_variant {
                    return Err(de::Error::invalid_value(
                        de::Unexpected::Str(type_variant.as_str()),
                        &variant.as_str(),
                    ));
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
    StringExample,
    Set,
    ThisFieldIsAnInteger,
    AlsoAnInteger,
    If,
    New,
    Interface,
    Unknown(Box<str>),
}
impl Variant_ {
    fn as_str(&self) -> &'static str {
        match self {
            Variant_::StringExample => "stringExample",
            Variant_::Set => "set",
            Variant_::ThisFieldIsAnInteger => "thisFieldIsAnInteger",
            Variant_::AlsoAnInteger => "alsoAnInteger",
            Variant_::If => "if",
            Variant_::New => "new",
            Variant_::Interface => "interface",
            Variant_::Unknown(_) => "unknown variant",
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
            "stringExample" => Variant_::StringExample,
            "set" => Variant_::Set,
            "thisFieldIsAnInteger" => Variant_::ThisFieldIsAnInteger,
            "alsoAnInteger" => Variant_::AlsoAnInteger,
            "if" => Variant_::If,
            "new" => Variant_::New,
            "interface" => Variant_::Interface,
            value => Variant_::Unknown(value.to_string().into_boxed_str()),
        };
        Ok(v)
    }
}
#[doc = "An unknown variant of the UnionTypeExample union."]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Unknown {
    type_: Box<str>,
    value: conjure_object::Value,
}
impl Unknown {
    #[doc = r" Returns the unknown variant's type name."]
    #[inline]
    pub fn type_(&self) -> &str {
        &self.type_
    }
}
