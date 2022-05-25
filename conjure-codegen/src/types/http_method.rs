use conjure_object::serde::{ser, de};
use std::fmt;
use std::str;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
}
impl HttpMethod {
    /// Returns the string representation of the enum.
    #[inline]
    pub fn as_str(&self) -> &str {
        match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Delete => "DELETE",
        }
    }
}
impl fmt::Display for HttpMethod {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), fmt)
    }
}
impl conjure_object::Plain for HttpMethod {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        conjure_object::Plain::fmt(self.as_str(), fmt)
    }
}
impl str::FromStr for HttpMethod {
    type Err = conjure_object::plain::ParseEnumError;
    #[inline]
    fn from_str(v: &str) -> Result<HttpMethod, conjure_object::plain::ParseEnumError> {
        match v {
            "GET" => Ok(HttpMethod::Get),
            "POST" => Ok(HttpMethod::Post),
            "PUT" => Ok(HttpMethod::Put),
            "DELETE" => Ok(HttpMethod::Delete),
            _ => Err(conjure_object::plain::ParseEnumError::new()),
        }
    }
}
impl conjure_object::FromPlain for HttpMethod {
    type Err = conjure_object::plain::ParseEnumError;
    #[inline]
    fn from_plain(v: &str) -> Result<HttpMethod, conjure_object::plain::ParseEnumError> {
        v.parse()
    }
}
impl ser::Serialize for HttpMethod {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        s.serialize_str(self.as_str())
    }
}
impl<'de> de::Deserialize<'de> for HttpMethod {
    fn deserialize<D>(d: D) -> Result<HttpMethod, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_str(Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = HttpMethod;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("a string")
    }
    fn visit_str<E>(self, v: &str) -> Result<HttpMethod, E>
    where
        E: de::Error,
    {
        match v.parse() {
            Ok(e) => Ok(e),
            Err(_) => {
                Err(de::Error::unknown_variant(v, &["GET", "POST", "PUT", "DELETE"]))
            }
        }
    }
}
