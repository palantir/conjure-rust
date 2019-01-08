use conjure::serde::{de, ser};
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Inner_ {
    Get,
    Post,
    Put,
    Delete,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HttpMethod(Inner_);
impl HttpMethod {
    pub const GET: HttpMethod = HttpMethod(Inner_::Get);
    pub const POST: HttpMethod = HttpMethod(Inner_::Post);
    pub const PUT: HttpMethod = HttpMethod(Inner_::Put);
    pub const DELETE: HttpMethod = HttpMethod(Inner_::Delete);
    #[doc = r" Returns the string representation of the enum."]
    #[inline]
    pub fn as_str(&self) -> &str {
        match &self.0 {
            Inner_::Get => "GET",
            Inner_::Post => "POST",
            Inner_::Put => "PUT",
            Inner_::Delete => "DELETE",
        }
    }
}
impl fmt::Display for HttpMethod {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(self.as_str())
    }
}
impl ser::Serialize for HttpMethod {
    fn serialize<S_>(&self, s: S_) -> Result<S_::Ok, S_::Error>
    where
        S_: ser::Serializer,
    {
        s.serialize_str(self.as_str())
    }
}
impl<'de> de::Deserialize<'de> for HttpMethod {
    fn deserialize<D_>(d: D_) -> Result<HttpMethod, D_::Error>
    where
        D_: de::Deserializer<'de>,
    {
        d.deserialize_str(Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = HttpMethod;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("string")
    }
    fn visit_str<E_>(self, v: &str) -> Result<HttpMethod, E_>
    where
        E_: de::Error,
    {
        match v {
            "GET" => Ok(HttpMethod::GET),
            "POST" => Ok(HttpMethod::POST),
            "PUT" => Ok(HttpMethod::PUT),
            "DELETE" => Ok(HttpMethod::DELETE),
            v => Err(de::Error::unknown_variant(
                v,
                &["GET", "POST", "PUT", "DELETE"],
            )),
        }
    }
}
