use conjure_types::serde::{de, ser};
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ErrorCode {
    PermissionDenied,
    InvalidArgument,
    NotFound,
    Conflict,
    RequestEntityTooLarge,
    FailedPrecondition,
    Internal,
    Timeout,
    CustomClient,
    CustomServer,
}
impl ErrorCode {
    #[doc = r" Returns the string representation of the enum."]
    #[inline]
    pub fn as_str(&self) -> &str {
        match self {
            ErrorCode::PermissionDenied => "PERMISSION_DENIED",
            ErrorCode::InvalidArgument => "INVALID_ARGUMENT",
            ErrorCode::NotFound => "NOT_FOUND",
            ErrorCode::Conflict => "CONFLICT",
            ErrorCode::RequestEntityTooLarge => "REQUEST_ENTITY_TOO_LARGE",
            ErrorCode::FailedPrecondition => "FAILED_PRECONDITION",
            ErrorCode::Internal => "INTERNAL",
            ErrorCode::Timeout => "TIMEOUT",
            ErrorCode::CustomClient => "CUSTOM_CLIENT",
            ErrorCode::CustomServer => "CUSTOM_SERVER",
        }
    }
}
impl fmt::Display for ErrorCode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), fmt)
    }
}
impl ser::Serialize for ErrorCode {
    fn serialize<S_>(&self, s: S_) -> Result<S_::Ok, S_::Error>
    where
        S_: ser::Serializer,
    {
        s.serialize_str(self.as_str())
    }
}
impl<'de> de::Deserialize<'de> for ErrorCode {
    fn deserialize<D_>(d: D_) -> Result<ErrorCode, D_::Error>
    where
        D_: de::Deserializer<'de>,
    {
        d.deserialize_str(Visitor_)
    }
}
struct Visitor_;
impl<'de> de::Visitor<'de> for Visitor_ {
    type Value = ErrorCode;
    fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("string")
    }
    fn visit_str<E_>(self, v: &str) -> Result<ErrorCode, E_>
    where
        E_: de::Error,
    {
        match v {
            "PERMISSION_DENIED" => Ok(ErrorCode::PermissionDenied),
            "INVALID_ARGUMENT" => Ok(ErrorCode::InvalidArgument),
            "NOT_FOUND" => Ok(ErrorCode::NotFound),
            "CONFLICT" => Ok(ErrorCode::Conflict),
            "REQUEST_ENTITY_TOO_LARGE" => Ok(ErrorCode::RequestEntityTooLarge),
            "FAILED_PRECONDITION" => Ok(ErrorCode::FailedPrecondition),
            "INTERNAL" => Ok(ErrorCode::Internal),
            "TIMEOUT" => Ok(ErrorCode::Timeout),
            "CUSTOM_CLIENT" => Ok(ErrorCode::CustomClient),
            "CUSTOM_SERVER" => Ok(ErrorCode::CustomServer),
            v => Err(de::Error::unknown_variant(
                v,
                &[
                    "PERMISSION_DENIED",
                    "INVALID_ARGUMENT",
                    "NOT_FOUND",
                    "CONFLICT",
                    "REQUEST_ENTITY_TOO_LARGE",
                    "FAILED_PRECONDITION",
                    "INTERNAL",
                    "TIMEOUT",
                    "CUSTOM_CLIENT",
                    "CUSTOM_SERVER",
                ],
            )),
        }
    }
}
