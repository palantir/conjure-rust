// Copyright 2019 Palantir Technologies, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use conjure_object::log_safety::MaybeLogSafe;
use conjure_object::Any;
use serde::Serialize;
use std::borrow::Cow;
use std::collections::hash_map::{self, HashMap};
use std::fmt;
use std::ops::{Deref, Index};
use std::str::FromStr;
use std::sync::OnceLock;
use std::time::Duration;
use std::{backtrace, error};

use crate::{ErrorType, Internal, SerializableError};

/// The cause of a QoS error, when known to differ from the default.
///
/// By default, a `503 Service Unavailable` response is assumed to be the result of a node-wide limit being reached, and
/// a `429 Too Many Requests` response is assumed to be specific to an individual endpoint on a node. A `DueTo` relays
/// the server's intent when those assumptions do not hold. It is sent to the client in the `Qos-Due-To` response
/// header.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DueTo {
    /// A cause the RPC system isn't directly aware of, for example a user or user-agent specific limit, or a limit on a
    /// specific resource being accessed, as opposed to the target node or endpoint as a whole.
    ///
    /// QoS errors with this cause shouldn't impact things like client-side concurrency limiters.
    Custom,
    /// A cause not recognized by this library, preserved so that it can be propagated.
    Unknown(UnknownDueTo),
}

impl DueTo {
    /// Returns the string representation of the cause, as used in the `Qos-Due-To` header.
    #[inline]
    pub fn as_str(&self) -> &str {
        match self {
            DueTo::Custom => "custom",
            DueTo::Unknown(v) => v,
        }
    }
}

impl fmt::Display for DueTo {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(self.as_str())
    }
}

/// Parses a `Qos-Due-To` header value.
///
/// Known causes are matched case-insensitively. Any other non-empty value consisting of printable ASCII characters,
/// spaces, and tabs is preserved verbatim as [`DueTo::Unknown`].
impl FromStr for DueTo {
    type Err = InvalidDueTo;

    fn from_str(s: &str) -> Result<DueTo, InvalidDueTo> {
        if s.eq_ignore_ascii_case("custom") {
            return Ok(DueTo::Custom);
        }

        if !is_valid_qos_header_value(s) {
            return Err(InvalidDueTo(()));
        }

        Ok(DueTo::Unknown(UnknownDueTo(s.into())))
    }
}

/// An unrecognized [`DueTo`] value.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnknownDueTo(Box<str>);

impl Deref for UnknownDueTo {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for UnknownDueTo {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(&self.0)
    }
}

/// The error returned when parsing an invalid [`DueTo`].
#[derive(Debug)]
pub struct InvalidDueTo(());

impl fmt::Display for InvalidDueTo {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("invalid QoS due-to value")
    }
}

impl error::Error for InvalidDueTo {}

/// The server's opinion on whether a QoS error should be retried, or propagated back to the caller as a failure.
///
/// There is no guarantee that clients will respect the hint; it should be considered best-effort. It is sent to the
/// client in the `Qos-Retry-Hint` response header.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum RetryHint {
    /// Clients should not retry the request, and should instead propagate the failure back to the initial caller.
    DoNotRetry,
    /// A hint not recognized by this library, preserved so that it can be propagated.
    Unknown(UnknownRetryHint),
}

impl RetryHint {
    /// Returns the string representation of the hint, as used in the `Qos-Retry-Hint` header.
    #[inline]
    pub fn as_str(&self) -> &str {
        match self {
            RetryHint::DoNotRetry => "do-not-retry",
            RetryHint::Unknown(v) => v,
        }
    }
}

impl fmt::Display for RetryHint {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(self.as_str())
    }
}

/// Parses a `Qos-Retry-Hint` header value.
///
/// Known hints are matched case-insensitively. Any other non-empty value consisting of printable ASCII characters,
/// spaces, and tabs is preserved verbatim as [`RetryHint::Unknown`].
impl FromStr for RetryHint {
    type Err = InvalidRetryHint;

    fn from_str(s: &str) -> Result<RetryHint, InvalidRetryHint> {
        if s.eq_ignore_ascii_case("do-not-retry") {
            return Ok(RetryHint::DoNotRetry);
        }

        if !is_valid_qos_header_value(s) {
            return Err(InvalidRetryHint(()));
        }

        Ok(RetryHint::Unknown(UnknownRetryHint(s.into())))
    }
}

/// An unrecognized [`RetryHint`] value.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnknownRetryHint(Box<str>);

impl Deref for UnknownRetryHint {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for UnknownRetryHint {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(&self.0)
    }
}

/// The error returned when parsing an invalid [`RetryHint`].
#[derive(Debug)]
pub struct InvalidRetryHint(());

impl fmt::Display for InvalidRetryHint {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("invalid QoS retry hint value")
    }
}

impl error::Error for InvalidRetryHint {}

fn is_valid_qos_header_value(s: &str) -> bool {
    !s.is_empty()
        && s.bytes()
            .all(|b| b.is_ascii_graphic() || b == b' ' || b == b'\t')
}

/// Information about a throttle error.
#[derive(Debug)]
pub struct ThrottleError {
    duration: Option<Duration>,
    due_to: Option<DueTo>,
    retry_hint: Option<RetryHint>,
}

impl ThrottleError {
    /// Returns the amount of time the client should wait before retrying, if provided.
    #[inline]
    pub fn duration(&self) -> Option<Duration> {
        self.duration
    }

    /// Returns the cause of the error, if provided.
    #[inline]
    pub fn due_to(&self) -> Option<&DueTo> {
        self.due_to.as_ref()
    }

    /// Returns the server's retry hint, if provided.
    #[inline]
    pub fn retry_hint(&self) -> Option<&RetryHint> {
        self.retry_hint.as_ref()
    }
}

/// Information about an unavailable error.
#[derive(Debug)]
pub struct UnavailableError {
    due_to: Option<DueTo>,
    retry_hint: Option<RetryHint>,
}

impl UnavailableError {
    /// Returns the cause of the error, if provided.
    #[inline]
    pub fn due_to(&self) -> Option<&DueTo> {
        self.due_to.as_ref()
    }

    /// Returns the server's retry hint, if provided.
    #[inline]
    pub fn retry_hint(&self) -> Option<&RetryHint> {
        self.retry_hint.as_ref()
    }
}

/// Information about the specific type of an `Error`.
#[derive(Debug)]
#[non_exhaustive]
pub enum ErrorKind {
    /// A general service error.
    Service(SerializableError),
    /// A QoS error indicating that the client should throttle itself.
    Throttle(ThrottleError),
    /// A QoS error indicating that the server was unable to handle the request.
    Unavailable(UnavailableError),
}

#[derive(Debug)]
struct Inner {
    cause: Box<dyn error::Error + Sync + Send>,
    cause_safe: bool,
    kind: ErrorKind,
    safe_params: HashMap<Cow<'static, str>, Any>,
    unsafe_params: HashMap<Cow<'static, str>, Any>,
    backtraces: Vec<Backtrace>,
}

/// A standard error type for network services.
///
/// An error consists of several components:
///
/// * The cause of the error, represented as a type implementing the Rust `Error` trait. The cause can either be
///   declared safe or unsafe to log.
/// * The error's kind, indicating how the service should handle the error e.g. in a response to a client.
/// * Backtraces, including one taken at the time the error was created.
/// * Parameters adding extra context about the error. They can be declared either safe or unsafe to log.
///
/// Note that this type does *not* implement the standard library's `Error` trait.
#[derive(Debug)]
pub struct Error(Box<Inner>);

impl Error {
    /// Creates a service error with an unsafe cause.
    pub fn service<E, T>(cause: E, error_type: T) -> Error
    where
        E: Into<Box<dyn error::Error + Sync + Send>>,
        T: ErrorType + Serialize,
    {
        Error::service_inner(
            cause.into(),
            false,
            crate::encode(&error_type),
            T::safe_args(),
        )
    }

    /// Creates a service error with a safe cause.
    pub fn service_safe<E, T>(cause: E, error_type: T) -> Error
    where
        E: Into<Box<dyn error::Error + Sync + Send>>,
        T: ErrorType + Serialize,
    {
        Error::service_inner(
            cause.into(),
            true,
            crate::encode(&error_type),
            T::safe_args(),
        )
    }

    /// Creates a service error from a propagated error description and an unsafe cause.
    pub fn propagated_service<E>(cause: E, error: SerializableError) -> Error
    where
        E: Into<Box<dyn error::Error + Sync + Send>>,
    {
        Error::service_inner(cause.into(), false, error, &[])
    }

    /// Creates a service error from a propagated error description and a safe cause.
    pub fn propagated_service_safe<E>(cause: E, error: SerializableError) -> Error
    where
        E: Into<Box<dyn error::Error + Sync + Send>>,
    {
        Error::service_inner(cause.into(), true, error, &[])
    }

    fn service_inner(
        cause: Box<dyn error::Error + Sync + Send>,
        cause_safe: bool,
        error: SerializableError,
        safe_args: &[&str],
    ) -> Error {
        let mut safe_params = HashMap::new();
        let mut unsafe_params = HashMap::new();

        for (key, value) in error.parameters() {
            let key = Cow::Owned(key.clone());
            let value = Any::new(value).unwrap();
            if safe_args.contains(&&*key) {
                safe_params.insert(key, value);
            } else {
                unsafe_params.insert(key, value);
            }
        }

        let mut error = Error::new(cause, cause_safe, ErrorKind::Service(error));
        error.0.safe_params = safe_params;
        error.0.unsafe_params = unsafe_params;
        error
    }

    /// Creates an error indicating that the client should throttle itself with an unsafe cause.
    pub fn throttle<E>(cause: E) -> Error
    where
        E: Into<Box<dyn error::Error + Sync + Send>>,
    {
        Error::new(
            cause.into(),
            false,
            ErrorKind::Throttle(ThrottleError {
                duration: None,
                due_to: None,
                retry_hint: None,
            }),
        )
    }

    /// Creates an error indicating that the client should throttle itself with a safe cause.
    pub fn throttle_safe<E>(cause: E) -> Error
    where
        E: Into<Box<dyn error::Error + Sync + Send>>,
    {
        Error::new(
            cause.into(),
            true,
            ErrorKind::Throttle(ThrottleError {
                duration: None,
                due_to: None,
                retry_hint: None,
            }),
        )
    }

    /// Creates an error indicating that the client should throttle itself for a specific duration with an unsafe
    /// cause.
    pub fn throttle_for<E>(cause: E, duration: Duration) -> Error
    where
        E: Into<Box<dyn error::Error + Sync + Send>>,
    {
        Error::new(
            cause.into(),
            false,
            ErrorKind::Throttle(ThrottleError {
                duration: Some(duration),
                due_to: None,
                retry_hint: None,
            }),
        )
    }

    /// Creates an error indicating that the client should throttle itself for a specific duration with a safe
    /// cause.
    pub fn throttle_for_safe<E>(cause: E, duration: Duration) -> Error
    where
        E: Into<Box<dyn error::Error + Sync + Send>>,
    {
        Error::new(
            cause.into(),
            true,
            ErrorKind::Throttle(ThrottleError {
                duration: Some(duration),
                due_to: None,
                retry_hint: None,
            }),
        )
    }

    /// Creates an error indicating that the server was unable to serve the client's request with an unsafe cause.
    pub fn unavailable<E>(cause: E) -> Error
    where
        E: Into<Box<dyn error::Error + Sync + Send>>,
    {
        Error::new(
            cause.into(),
            false,
            ErrorKind::Unavailable(UnavailableError {
                due_to: None,
                retry_hint: None,
            }),
        )
    }

    /// Creates an error indicating that the server was unable to serve the client's request with a safe cause.
    pub fn unavailable_safe<E>(cause: E) -> Error
    where
        E: Into<Box<dyn error::Error + Sync + Send>>,
    {
        Error::new(
            cause.into(),
            true,
            ErrorKind::Unavailable(UnavailableError {
                due_to: None,
                retry_hint: None,
            }),
        )
    }

    /// A convenience function to construct an internal service error with an unsafe cause.
    pub fn internal<E>(cause: E) -> Error
    where
        E: Into<Box<dyn error::Error + Sync + Send>>,
    {
        Error::service(cause, Internal::new())
    }

    /// A convenience function to construct an internal service error with a safe cause.
    pub fn internal_safe<E>(cause: E) -> Error
    where
        E: Into<Box<dyn error::Error + Sync + Send>>,
    {
        Error::service_safe(cause, Internal::new())
    }

    fn new(cause: Box<dyn error::Error + Sync + Send>, cause_safe: bool, kind: ErrorKind) -> Error {
        let inner = Inner {
            cause,
            cause_safe,
            kind,
            safe_params: HashMap::new(),
            unsafe_params: HashMap::new(),
            backtraces: vec![],
        };
        Error(Box::new(inner)).with_backtrace()
    }

    /// Returns the error's cause.
    ///
    /// Use the `cause_safe` method to determine if the error is safe or not.
    #[inline]
    pub fn cause(&self) -> &(dyn error::Error + 'static + Sync + Send) {
        &*self.0.cause
    }

    /// Returns whether or not the error's cause is considered safe.
    #[inline]
    pub fn cause_safe(&self) -> bool {
        self.0.cause_safe
    }

    /// Returns kind-specific error information.
    #[inline]
    pub fn kind(&self) -> &ErrorKind {
        &self.0.kind
    }

    /// Sets the cause of a QoS error.
    ///
    /// This has no effect on service errors.
    pub fn with_due_to(mut self, due_to: DueTo) -> Error {
        match &mut self.0.kind {
            ErrorKind::Throttle(e) => e.due_to = Some(due_to),
            ErrorKind::Unavailable(e) => e.due_to = Some(due_to),
            ErrorKind::Service(_) => {}
        }
        self
    }

    /// Sets the server's retry hint for a QoS error.
    ///
    /// This has no effect on service errors.
    pub fn with_retry_hint(mut self, retry_hint: RetryHint) -> Error {
        match &mut self.0.kind {
            ErrorKind::Throttle(e) => e.retry_hint = Some(retry_hint),
            ErrorKind::Unavailable(e) => e.retry_hint = Some(retry_hint),
            ErrorKind::Service(_) => {}
        }
        self
    }

    /// Adds a new safe parameter to the error.
    ///
    /// The `MaybeLogSafe` bound is gated on the `log-safety` feature.
    /// When enabled it requires `T: LogSafe`.
    /// When disabled it accepts any `T`.
    /// To log a non-`LogSafe` value, wrap it in [`AssertLogSafe`].
    ///
    /// # Panics
    ///
    /// Panics if the value fails to serialize.
    pub fn with_safe_param<T>(mut self, key: &'static str, value: T) -> Error
    where
        T: Serialize + MaybeLogSafe,
    {
        let value = Any::new(value).expect("value failed to serialize");
        self.0.safe_params.insert(Cow::Borrowed(key), value);
        self
    }

    /// Adds a new unsafe parameter to the error.
    ///
    /// # Panics
    ///
    /// Panics if the value fails to serialize.
    pub fn with_unsafe_param<T>(mut self, key: &'static str, value: T) -> Error
    where
        T: Serialize,
    {
        let value = Any::new(value).expect("value failed to serialize");
        self.0.unsafe_params.insert(Cow::Borrowed(key), value);
        self
    }

    /// Returns the error's safe parameters.
    #[inline]
    pub fn safe_params(&self) -> Params<'_> {
        Params(&self.0.safe_params)
    }

    /// Returns the error's unsafe parameters.
    #[inline]
    pub fn unsafe_params(&self) -> Params<'_> {
        Params(&self.0.unsafe_params)
    }

    /// Adds a new backtrace to the error.
    ///
    /// An error always takes a backtrace at the time of its construction, but this method can be used to add extra
    /// backtraces to it. For example, this might be used when transferring an error from one thread to another.
    #[inline]
    pub fn with_backtrace(mut self) -> Error {
        self.0.backtraces.push(Backtrace::new());
        self
    }

    /// Adds a new custom backtrace to the error which is safe to log
    #[inline]
    pub fn with_custom_safe_backtrace(mut self, backtrace: String) -> Error {
        self.0.backtraces.push(Backtrace::custom(backtrace));
        self
    }

    /// Returns the error's backtraces, ordered from oldest to newest.
    #[inline]
    pub fn backtraces(&self) -> &[Backtrace] {
        &self.0.backtraces
    }
}

/// A collection of error parameters, either safe or unsafe.
#[derive(Debug)]
pub struct Params<'a>(&'a HashMap<Cow<'static, str>, Any>);

impl<'a> Params<'a> {
    /// Returns an iterator over the key-value parameter pairs.
    #[inline]
    pub fn iter(&self) -> ParamsIter<'a> {
        ParamsIter(self.0.iter())
    }

    /// Returns the number of parameters.
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Determines if there are no parameters.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Index<&str> for Params<'_> {
    type Output = Any;

    #[inline]
    fn index(&self, key: &str) -> &Any {
        &self.0[key]
    }
}

impl<'a> IntoIterator for &Params<'a> {
    type Item = (&'a str, &'a Any);
    type IntoIter = ParamsIter<'a>;

    #[inline]
    fn into_iter(self) -> ParamsIter<'a> {
        self.iter()
    }
}

/// An iterator over the parameters of an error.
pub struct ParamsIter<'a>(hash_map::Iter<'a, Cow<'static, str>, Any>);

impl<'a> Iterator for ParamsIter<'a> {
    type Item = (&'a str, &'a Any);

    #[inline]
    fn next(&mut self) -> Option<(&'a str, &'a Any)> {
        self.0.next().map(|(a, b)| (&**a, b))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

static BACKTRACE_PROVIDER: OnceLock<Box<dyn Fn() -> String + Send + Sync>> = OnceLock::new();

/// Overrides backtrace capture. The returned string lands verbatim in the
/// `stacktrace` field of service.1 records and is treated as safe-to-log. Used for platforms
/// like wasm with that backtrace::Backtrace doesn't support.
pub fn set_safe_custom_backtrace_provider<F>(provider: F) -> Result<(), Error>
where
    F: Fn() -> String + Send + Sync + 'static,
{
    BACKTRACE_PROVIDER
        .set(Box::new(provider))
        .or(Err(Error::internal_safe("backtrace provider already set")))
}

/// A backtrace associated with an `Error`.
pub struct Backtrace(BacktraceInner);

impl fmt::Debug for Backtrace {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            BacktraceInner::Rust(b) => fmt::Display::fmt(b, fmt),
            BacktraceInner::Custom(b) => fmt::Display::fmt(b, fmt),
        }
    }
}

impl Backtrace {
    #[inline]
    fn new() -> Backtrace {
        if let Some(provider) = BACKTRACE_PROVIDER.get() {
            Backtrace(BacktraceInner::Custom(provider()))
        } else {
            Backtrace(BacktraceInner::Rust(backtrace::Backtrace::force_capture()))
        }
    }

    #[inline]
    fn custom(s: String) -> Backtrace {
        Backtrace(BacktraceInner::Custom(s))
    }
}

enum BacktraceInner {
    Rust(backtrace::Backtrace),
    Custom(String),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn due_to_defaults_to_none() {
        match Error::throttle("").kind() {
            ErrorKind::Throttle(e) => assert_eq!(e.due_to(), None),
            _ => panic!("expected throttle"),
        }
        match Error::unavailable("").kind() {
            ErrorKind::Unavailable(e) => assert_eq!(e.due_to(), None),
            _ => panic!("expected unavailable"),
        }
    }

    #[test]
    fn with_due_to() {
        let error = Error::throttle_for("", Duration::from_secs(1)).with_due_to(DueTo::Custom);
        match error.kind() {
            ErrorKind::Throttle(e) => {
                assert_eq!(e.duration(), Some(Duration::from_secs(1)));
                assert_eq!(e.due_to(), Some(&DueTo::Custom));
            }
            _ => panic!("expected throttle"),
        }

        let error = Error::unavailable("").with_due_to(DueTo::Custom);
        match error.kind() {
            ErrorKind::Unavailable(e) => assert_eq!(e.due_to(), Some(&DueTo::Custom)),
            _ => panic!("expected unavailable"),
        }

        let error = Error::internal("").with_due_to(DueTo::Custom);
        assert!(matches!(error.kind(), ErrorKind::Service(_)));
    }

    #[test]
    fn due_to_header_value() {
        assert_eq!(DueTo::Custom.as_str(), "custom");
        assert_eq!(DueTo::Custom.to_string(), "custom");
    }

    #[test]
    fn with_retry_hint() {
        let error = Error::throttle("")
            .with_due_to(DueTo::Custom)
            .with_retry_hint(RetryHint::DoNotRetry);
        match error.kind() {
            ErrorKind::Throttle(e) => {
                assert_eq!(e.due_to(), Some(&DueTo::Custom));
                assert_eq!(e.retry_hint(), Some(&RetryHint::DoNotRetry));
            }
            _ => panic!("expected throttle"),
        }

        let error = Error::unavailable("").with_retry_hint(RetryHint::DoNotRetry);
        match error.kind() {
            ErrorKind::Unavailable(e) => {
                assert_eq!(e.due_to(), None);
                assert_eq!(e.retry_hint(), Some(&RetryHint::DoNotRetry));
            }
            _ => panic!("expected unavailable"),
        }

        match Error::throttle("").kind() {
            ErrorKind::Throttle(e) => assert_eq!(e.retry_hint(), None),
            _ => panic!("expected throttle"),
        }

        let error = Error::internal("").with_retry_hint(RetryHint::DoNotRetry);
        assert!(matches!(error.kind(), ErrorKind::Service(_)));
    }

    #[test]
    fn parse_retry_hint() {
        assert_eq!(RetryHint::DoNotRetry.as_str(), "do-not-retry");
        assert_eq!(
            "do-not-retry".parse::<RetryHint>().unwrap(),
            RetryHint::DoNotRetry
        );
        assert_eq!(
            "Do-Not-Retry".parse::<RetryHint>().unwrap(),
            RetryHint::DoNotRetry
        );

        let unknown = "some-future-hint".parse::<RetryHint>().unwrap();
        match &unknown {
            RetryHint::Unknown(v) => assert_eq!(&**v, "some-future-hint"),
            _ => panic!("expected unknown"),
        }
        assert_eq!(unknown.as_str(), "some-future-hint");

        assert!("".parse::<RetryHint>().is_err());
        assert!("bad\nvalue".parse::<RetryHint>().is_err());
    }

    #[test]
    fn parse_due_to() {
        assert_eq!("custom".parse::<DueTo>().unwrap(), DueTo::Custom);
        assert_eq!("CUSTOM".parse::<DueTo>().unwrap(), DueTo::Custom);

        let unknown = "some-future-cause".parse::<DueTo>().unwrap();
        match &unknown {
            DueTo::Unknown(v) => assert_eq!(&**v, "some-future-cause"),
            _ => panic!("expected unknown"),
        }
        assert_eq!(unknown.as_str(), "some-future-cause");
        assert_eq!(unknown.to_string(), "some-future-cause");

        let error = Error::throttle("").with_due_to(unknown.clone());
        match error.kind() {
            ErrorKind::Throttle(e) => assert_eq!(e.due_to(), Some(&unknown)),
            _ => panic!("expected throttle"),
        }

        assert!("".parse::<DueTo>().is_err());
        assert!("bad\nvalue".parse::<DueTo>().is_err());
        assert!("caf\u{e9}".parse::<DueTo>().is_err());
    }
}
