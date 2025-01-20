#![allow(deprecated)]
use std::fmt;
use std::str;
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    conjure_object::serde::Deserialize,
    conjure_object::serde::Serialize,
)]
#[serde(crate = "conjure_object::serde")]
pub enum ErrorCode {
    #[serde(rename = "PERMISSION_DENIED")]
    PermissionDenied,
    #[serde(rename = "INVALID_ARGUMENT")]
    InvalidArgument,
    #[serde(rename = "NOT_FOUND")]
    NotFound,
    #[serde(rename = "CONFLICT")]
    Conflict,
    #[serde(rename = "REQUEST_ENTITY_TOO_LARGE")]
    RequestEntityTooLarge,
    #[serde(rename = "FAILED_PRECONDITION")]
    FailedPrecondition,
    #[serde(rename = "INTERNAL")]
    Internal,
    #[serde(rename = "TIMEOUT")]
    Timeout,
    #[serde(rename = "CUSTOM_CLIENT")]
    CustomClient,
    #[serde(rename = "CUSTOM_SERVER")]
    CustomServer,
}
impl ErrorCode {
    /// Returns the string representation of the enum.
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
impl conjure_object::Plain for ErrorCode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        conjure_object::Plain::fmt(self.as_str(), fmt)
    }
}
impl str::FromStr for ErrorCode {
    type Err = conjure_object::plain::ParseEnumError;
    #[inline]
    fn from_str(v: &str) -> Result<ErrorCode, conjure_object::plain::ParseEnumError> {
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
            _ => Err(conjure_object::plain::ParseEnumError::new()),
        }
    }
}
impl conjure_object::FromPlain for ErrorCode {
    type Err = conjure_object::plain::ParseEnumError;
    #[inline]
    fn from_plain(v: &str) -> Result<ErrorCode, conjure_object::plain::ParseEnumError> {
        v.parse()
    }
}
