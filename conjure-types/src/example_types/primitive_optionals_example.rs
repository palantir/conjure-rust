use crate::serde::ser::SerializeMap as SerializeMap_;
use crate::serde::{de, ser};
use std::fmt;
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PrimitiveOptionalsExample {
    num: Option<f64>,
    bool: Option<bool>,
    integer: Option<i32>,
    safelong: Option<crate::SafeLong>,
    rid: Option<crate::ResourceIdentifier>,
    bearertoken: Option<crate::BearerToken>,
    uuid: Option<crate::Uuid>,
}
impl PrimitiveOptionalsExample {
    #[doc = r" Returns a new builder."]
    #[inline]
    pub fn builder() -> Builder {
        Default::default()
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
    pub fn safelong(&self) -> Option<crate::SafeLong> {
        self.safelong.as_ref().map(|o| *o)
    }
    #[inline]
    pub fn rid(&self) -> Option<&crate::ResourceIdentifier> {
        self.rid.as_ref().map(|o| &*o)
    }
    #[inline]
    pub fn bearertoken(&self) -> Option<&crate::BearerToken> {
        self.bearertoken.as_ref().map(|o| &*o)
    }
    #[inline]
    pub fn uuid(&self) -> Option<crate::Uuid> {
        self.uuid.as_ref().map(|o| *o)
    }
}
#[derive(Debug, Clone, Default)]
pub struct Builder {
    num: Option<f64>,
    bool: Option<bool>,
    integer: Option<i32>,
    safelong: Option<crate::SafeLong>,
    rid: Option<crate::ResourceIdentifier>,
    bearertoken: Option<crate::BearerToken>,
    uuid: Option<crate::Uuid>,
}
impl Builder {
    pub fn num<T>(&mut self, num: T) -> &mut Self
    where
        T: Into<Option<f64>>,
    {
        self.num = num.into();
        self
    }
    pub fn bool<T>(&mut self, bool: T) -> &mut Self
    where
        T: Into<Option<bool>>,
    {
        self.bool = bool.into();
        self
    }
    pub fn integer<T>(&mut self, integer: T) -> &mut Self
    where
        T: Into<Option<i32>>,
    {
        self.integer = integer.into();
        self
    }
    pub fn safelong<T>(&mut self, safelong: T) -> &mut Self
    where
        T: Into<Option<crate::SafeLong>>,
    {
        self.safelong = safelong.into();
        self
    }
    pub fn rid<T>(&mut self, rid: T) -> &mut Self
    where
        T: Into<Option<crate::ResourceIdentifier>>,
    {
        self.rid = rid.into();
        self
    }
    pub fn bearertoken<T>(&mut self, bearertoken: T) -> &mut Self
    where
        T: Into<Option<crate::BearerToken>>,
    {
        self.bearertoken = bearertoken.into();
        self
    }
    pub fn uuid<T>(&mut self, uuid: T) -> &mut Self
    where
        T: Into<Option<crate::Uuid>>,
    {
        self.uuid = uuid.into();
        self
    }
    #[doc = r" Constructs a new instance of the type."]
    #[doc = r""]
    #[doc = r" # Panics"]
    #[doc = r""]
    #[doc = r" Panics if a required field was not set."]
    #[inline]
    pub fn build(&self) -> PrimitiveOptionalsExample {
        PrimitiveOptionalsExample {
            num: self.num.clone(),
            bool: self.bool.clone(),
            integer: self.integer.clone(),
            safelong: self.safelong.clone(),
            rid: self.rid.clone(),
            bearertoken: self.bearertoken.clone(),
            uuid: self.uuid.clone(),
        }
    }
}
impl From<PrimitiveOptionalsExample> for Builder {
    #[inline]
    fn from(_v: PrimitiveOptionalsExample) -> Builder {
        Builder {
            num: _v.num,
            bool: _v.bool,
            integer: _v.integer,
            safelong: _v.safelong,
            rid: _v.rid,
            bearertoken: _v.bearertoken,
            uuid: _v.uuid,
        }
    }
}
impl ser::Serialize for PrimitiveOptionalsExample {
    fn serialize<S_>(&self, s: S_) -> Result<S_::Ok, S_::Error>
    where
        S_: ser::Serializer,
    {
        let mut size = 0usize;
        let skip_num = self.num.is_none();
        if !skip_num {
            size += 1;
        }
        let skip_bool = self.bool.is_none();
        if !skip_bool {
            size += 1;
        }
        let skip_integer = self.integer.is_none();
        if !skip_integer {
            size += 1;
        }
        let skip_safelong = self.safelong.is_none();
        if !skip_safelong {
            size += 1;
        }
        let skip_rid = self.rid.is_none();
        if !skip_rid {
            size += 1;
        }
        let skip_bearertoken = self.bearertoken.is_none();
        if !skip_bearertoken {
            size += 1;
        }
        let skip_uuid = self.uuid.is_none();
        if !skip_uuid {
            size += 1;
        }
        let mut map = s.serialize_map(Some(size))?;
        if !skip_num {
            map.serialize_entry(&"num", &self.num)?;
        }
        if !skip_bool {
            map.serialize_entry(&"bool", &self.bool)?;
        }
        if !skip_integer {
            map.serialize_entry(&"integer", &self.integer)?;
        }
        if !skip_safelong {
            map.serialize_entry(&"safelong", &self.safelong)?;
        }
        if !skip_rid {
            map.serialize_entry(&"rid", &self.rid)?;
        }
        if !skip_bearertoken {
            map.serialize_entry(&"bearertoken", &self.bearertoken)?;
        }
        if !skip_uuid {
            map.serialize_entry(&"uuid", &self.uuid)?;
        }
        map.end()
    }
}
impl<'de> de::Deserialize<'de> for PrimitiveOptionalsExample {
    fn deserialize<D_>(d: D_) -> Result<PrimitiveOptionalsExample, D_::Error>
    where
        D_: de::Deserializer<'de>,
    {
        d.deserialize_struct(
            "PrimitiveOptionalsExample",
            &[
                "num",
                "bool",
                "integer",
                "safelong",
                "rid",
                "bearertoken",
                "uuid",
            ],
            Visitor_,
        )
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = PrimitiveOptionalsExample;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A_>(self, mut map_: A_) -> Result<PrimitiveOptionalsExample, A_::Error>
    where
        A_: de::MapAccess<'de>,
    {
        let mut num = None;
        let mut bool = None;
        let mut integer = None;
        let mut safelong = None;
        let mut rid = None;
        let mut bearertoken = None;
        let mut uuid = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::Num => num = Some(map_.next_value()?),
                Field_::Bool => bool = Some(map_.next_value()?),
                Field_::Integer => integer = Some(map_.next_value()?),
                Field_::Safelong => safelong = Some(map_.next_value()?),
                Field_::Rid => rid = Some(map_.next_value()?),
                Field_::Bearertoken => bearertoken = Some(map_.next_value()?),
                Field_::Uuid => uuid = Some(map_.next_value()?),
                Field_::Unknown_ => {
                    map_.next_value::<de::IgnoredAny>()?;
                }
            }
        }
        let num = match num {
            Some(v) => v,
            None => Default::default(),
        };
        let bool = match bool {
            Some(v) => v,
            None => Default::default(),
        };
        let integer = match integer {
            Some(v) => v,
            None => Default::default(),
        };
        let safelong = match safelong {
            Some(v) => v,
            None => Default::default(),
        };
        let rid = match rid {
            Some(v) => v,
            None => Default::default(),
        };
        let bearertoken = match bearertoken {
            Some(v) => v,
            None => Default::default(),
        };
        let uuid = match uuid {
            Some(v) => v,
            None => Default::default(),
        };
        Ok(PrimitiveOptionalsExample {
            num,
            bool,
            integer,
            safelong,
            rid,
            bearertoken,
            uuid,
        })
    }
}
enum Field_ {
    Num,
    Bool,
    Integer,
    Safelong,
    Rid,
    Bearertoken,
    Uuid,
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
            "num" => Field_::Num,
            "bool" => Field_::Bool,
            "integer" => Field_::Integer,
            "safelong" => Field_::Safelong,
            "rid" => Field_::Rid,
            "bearertoken" => Field_::Bearertoken,
            "uuid" => Field_::Uuid,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
