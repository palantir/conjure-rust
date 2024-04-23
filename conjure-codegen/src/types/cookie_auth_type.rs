use conjure_object::serde::{ser, de};
use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[conjure_object::private::staged_builder::staged_builder]
#[builder(crate = conjure_object::private::staged_builder, update, inline)]
pub struct CookieAuthType {
    #[builder(into)]
    cookie_name: String,
}
impl CookieAuthType {
    /// Constructs a new instance of the type.
    #[inline]
    pub fn new(cookie_name: impl Into<String>) -> Self {
        Self::builder().cookie_name(cookie_name).build()
    }
    #[inline]
    pub fn cookie_name(&self) -> &str {
        &*self.cookie_name
    }
}
impl ser::Serialize for CookieAuthType {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let size = 1usize;
        let mut s = s.serialize_struct("CookieAuthType", size)?;
        s.serialize_field("cookieName", &self.cookie_name)?;
        s.end()
    }
}
impl<'de> de::Deserialize<'de> for CookieAuthType {
    fn deserialize<D>(d: D) -> Result<CookieAuthType, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_struct("CookieAuthType", &["cookieName"], Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = CookieAuthType;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("map")
    }
    fn visit_map<A>(self, mut map_: A) -> Result<CookieAuthType, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut cookie_name = None;
        while let Some(field_) = map_.next_key()? {
            match field_ {
                Field_::CookieName => cookie_name = Some(map_.next_value()?),
                Field_::Unknown_ => {
                    map_.next_value::<de::IgnoredAny>()?;
                }
            }
        }
        let cookie_name = match cookie_name {
            Some(v) => v,
            None => return Err(de::Error::missing_field("cookieName")),
        };
        Ok(CookieAuthType { cookie_name })
    }
}
enum Field_ {
    CookieName,
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
            "cookieName" => Field_::CookieName,
            _ => Field_::Unknown_,
        };
        Ok(v)
    }
}
