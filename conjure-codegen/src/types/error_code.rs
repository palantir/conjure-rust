use conjure::serde::{de, ser};
use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Inner_ {
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
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ErrorCode(Inner_);
impl ErrorCode {
    pub const PERMISSION_DENIED: ErrorCode = ErrorCode(Inner_::PermissionDenied);
    pub const INVALID_ARGUMENT: ErrorCode = ErrorCode(Inner_::InvalidArgument);
    pub const NOT_FOUND: ErrorCode = ErrorCode(Inner_::NotFound);
    pub const CONFLICT: ErrorCode = ErrorCode(Inner_::Conflict);
    pub const REQUEST_ENTITY_TOO_LARGE: ErrorCode = ErrorCode(Inner_::RequestEntityTooLarge);
    pub const FAILED_PRECONDITION: ErrorCode = ErrorCode(Inner_::FailedPrecondition);
    pub const INTERNAL: ErrorCode = ErrorCode(Inner_::Internal);
    pub const TIMEOUT: ErrorCode = ErrorCode(Inner_::Timeout);
    pub const CUSTOM_CLIENT: ErrorCode = ErrorCode(Inner_::CustomClient);
    pub const CUSTOM_SERVER: ErrorCode = ErrorCode(Inner_::CustomServer);
    #[doc = r" Returns the string representation of the enum."]
    #[inline]
    pub fn as_str(&self) -> &str {
        match &self.0 {
            Inner_::PermissionDenied => "PERMISSION_DENIED",
            Inner_::InvalidArgument => "INVALID_ARGUMENT",
            Inner_::NotFound => "NOT_FOUND",
            Inner_::Conflict => "CONFLICT",
            Inner_::RequestEntityTooLarge => "REQUEST_ENTITY_TOO_LARGE",
            Inner_::FailedPrecondition => "FAILED_PRECONDITION",
            Inner_::Internal => "INTERNAL",
            Inner_::Timeout => "TIMEOUT",
            Inner_::CustomClient => "CUSTOM_CLIENT",
            Inner_::CustomServer => "CUSTOM_SERVER",
        }
    }
}
impl fmt::Display for ErrorCode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(self.as_str())
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
            "PERMISSION_DENIED" => Ok(ErrorCode::PERMISSION_DENIED),
            "INVALID_ARGUMENT" => Ok(ErrorCode::INVALID_ARGUMENT),
            "NOT_FOUND" => Ok(ErrorCode::NOT_FOUND),
            "CONFLICT" => Ok(ErrorCode::CONFLICT),
            "REQUEST_ENTITY_TOO_LARGE" => Ok(ErrorCode::REQUEST_ENTITY_TOO_LARGE),
            "FAILED_PRECONDITION" => Ok(ErrorCode::FAILED_PRECONDITION),
            "INTERNAL" => Ok(ErrorCode::INTERNAL),
            "TIMEOUT" => Ok(ErrorCode::TIMEOUT),
            "CUSTOM_CLIENT" => Ok(ErrorCode::CUSTOM_CLIENT),
            "CUSTOM_SERVER" => Ok(ErrorCode::CUSTOM_SERVER),
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
