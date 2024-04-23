use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, conjure_object::private::Educe)]
#[educe(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[conjure_object::private::staged_builder::staged_builder]
#[builder(crate = conjure_object::private::staged_builder, update, inline)]
pub struct PrimitiveOptionalsExample {
    #[builder(default, into)]
    #[educe(
        PartialEq(method(conjure_object::private::DoubleOps::eq)),
        Ord(method(conjure_object::private::DoubleOps::cmp)),
        Hash(method(conjure_object::private::DoubleOps::hash)),
    )]
    num: Option<f64>,
    #[builder(default, into)]
    bool: Option<bool>,
    #[builder(default, into)]
    integer: Option<i32>,
    #[builder(default, into)]
    safelong: Option<conjure_object::SafeLong>,
    #[builder(default, into)]
    rid: Option<conjure_object::ResourceIdentifier>,
    #[builder(default, into)]
    bearertoken: Option<conjure_object::BearerToken>,
    #[builder(default, into)]
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
impl ser::Serialize for PrimitiveOptionalsExample {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
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
        let mut s = s.serialize_struct("PrimitiveOptionalsExample", size)?;
        if skip_num {
            s.skip_field("num")?;
        } else {
            s.serialize_field("num", &self.num)?;
        }
        if skip_bool {
            s.skip_field("bool")?;
        } else {
            s.serialize_field("bool", &self.bool)?;
        }
        if skip_integer {
            s.skip_field("integer")?;
        } else {
            s.serialize_field("integer", &self.integer)?;
        }
        if skip_safelong {
            s.skip_field("safelong")?;
        } else {
            s.serialize_field("safelong", &self.safelong)?;
        }
        if skip_rid {
            s.skip_field("rid")?;
        } else {
            s.serialize_field("rid", &self.rid)?;
        }
        if skip_bearertoken {
            s.skip_field("bearertoken")?;
        } else {
            s.serialize_field("bearertoken", &self.bearertoken)?;
        }
        if skip_uuid {
            s.skip_field("uuid")?;
        } else {
            s.serialize_field("uuid", &self.uuid)?;
        }
        s.end()
    }
}
impl<'de> de::Deserialize<'de> for PrimitiveOptionalsExample {
    fn deserialize<D>(d: D) -> Result<PrimitiveOptionalsExample, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct(
            "PrimitiveOptionalsExample",
            &["num", "bool", "integer", "safelong", "rid", "bearertoken", "uuid"],
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
    fn visit_map<A>(self, mut map_: A) -> Result<PrimitiveOptionalsExample, A::Error>
    where
        A: de::MapAccess<'de>,
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
