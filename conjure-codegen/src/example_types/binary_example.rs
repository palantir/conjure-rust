use conjure_object::serde::ser::SerializeMap as SerializeMap_;
use conjure_object::serde::{de, ser};
use std::fmt;
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct BinaryExample {
    binary: conjure_object::ByteBuf,
}
impl BinaryExample {
    #[doc = r" Returns a new builder."]
    #[inline]
    pub fn builder() -> Builder {
        Default::default()
    }
    #[inline]
    pub fn binary(&self) -> &[u8] {
        &*self.binary
    }
}
#[derive(Debug, Clone, Default)]
pub struct Builder {
    binary: Option<conjure_object::ByteBuf>,
}
impl Builder {
    #[doc = r""]
    #[doc = r" Required."]
    pub fn binary<T>(&mut self, binary: T) -> &mut Self
    where
        T: Into<Vec<u8>>,
    {
        self.binary = Some(binary.into().into());
        self
    }
    #[doc = r" Constructs a new instance of the type."]
    #[doc = r""]
    #[doc = r" # Panics"]
    #[doc = r""]
    #[doc = r" Panics if a required field was not set."]
    #[inline]
    pub fn build(&self) -> BinaryExample {
        BinaryExample {
            binary: self.binary.clone().expect("field binary was not set"),
        }
    }
}
impl From<BinaryExample> for Builder {
    #[inline]
    fn from(_v: BinaryExample) -> Builder {
        Builder {
            binary: Some(_v.binary),
        }
    }
}
impl ser::Serialize for BinaryExample {
    fn serialize<S_>(&self, s: S_) -> Result<S_::Ok, S_::Error>
    where
        S_: ser::Serializer,
    {
        let size = 1usize;
        let mut map = s.serialize_map(Some(size))?;
        map.serialize_entry(&"binary", &self.binary)?;
        map.end()
    }
}
impl<'de> de::Deserialize<'de> for BinaryExample {
    fn deserialize<D_>(d: D_) -> Result<BinaryExample, D_::Error>
    where
        D_: de::Deserializer<'de>,
    {
        d.deserialize_struct("BinaryExample", &["binary"], Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = BinaryExample;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A_>(self, mut map_: A_) -> Result<BinaryExample, A_::Error>
    where
        A_: de::MapAccess<'de>,
    {
        let mut binary = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::Binary => binary = Some(map_.next_value()?),
                Field_::Unknown_ => {
                    map_.next_value::<de::IgnoredAny>()?;
                }
            }
        }
        let binary = match binary {
            Some(v) => v,
            None => return Err(de::Error::missing_field("binary")),
        };
        Ok(BinaryExample { binary })
    }
}
enum Field_ {
    Binary,
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
            "binary" => Field_::Binary,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
