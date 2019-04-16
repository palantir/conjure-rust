use conjure_object::serde::ser::SerializeStruct as SerializeStruct_;
use conjure_object::serde::{de, ser};
use std::fmt;
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct CookieAuthType {
    cookie_name: String,
}
impl CookieAuthType {
    #[doc = r" Constructs a new instance of the type."]
    #[inline]
    pub fn new<T>(cookie_name: T) -> CookieAuthType
    where
        T: Into<String>,
    {
        CookieAuthType {
            cookie_name: cookie_name.into(),
        }
    }
    #[doc = r" Returns a new builder."]
    #[inline]
    pub fn builder() -> Builder {
        Default::default()
    }
    #[inline]
    pub fn cookie_name(&self) -> &str {
        &*self.cookie_name
    }
}
#[derive(Debug, Clone, Default)]
pub struct Builder {
    cookie_name: Option<String>,
}
impl Builder {
    #[doc = r""]
    #[doc = r" Required."]
    pub fn cookie_name<T>(&mut self, cookie_name: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.cookie_name = Some(cookie_name.into());
        self
    }
    #[doc = r" Constructs a new instance of the type."]
    #[doc = r""]
    #[doc = r" # Panics"]
    #[doc = r""]
    #[doc = r" Panics if a required field was not set."]
    #[inline]
    pub fn build(&self) -> CookieAuthType {
        CookieAuthType {
            cookie_name: self
                .cookie_name
                .clone()
                .expect("field cookie_name was not set"),
        }
    }
}
impl From<CookieAuthType> for Builder {
    #[inline]
    fn from(_v: CookieAuthType) -> Builder {
        Builder {
            cookie_name: Some(_v.cookie_name),
        }
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
