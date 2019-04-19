use backtrace::Backtrace;
use conjure_object::Value;
use serde::Serialize;
use std::borrow::Cow;
use std::collections::hash_map::{self, HashMap};
use std::error;
use std::ops::Index;
use std::time::Duration;

use crate::{ErrorType, Internal, SerializableError};

/// Information about a throttle error.
#[derive(Debug)]
pub struct ThrottleError {
    duration: Option<Duration>,
}

impl ThrottleError {
    /// Returns the amount of time the client should wait before retrying, if provided.
    #[inline]
    pub fn duration(&self) -> Option<Duration> {
        self.duration
    }
}

/// Information about an unavailable error.
#[derive(Debug)]
pub struct UnavailableError(());

/// Information about the specific type of an `Error`.
#[derive(Debug)]
pub enum ErrorKind {
    /// A general service error.
    Service(SerializableError),
    /// A QoS error indicating that the client should throttle itself.
    Throttle(ThrottleError),
    /// A QoS error indicating that the server was unable to handle the request.
    Unavailable(UnavailableError),
    #[doc(hidden)]
    __NonExhaustive,
}

#[derive(Debug)]
struct Inner {
    cause: Box<dyn error::Error + Sync + Send>,
    cause_safe: bool,
    kind: ErrorKind,
    safe_params: HashMap<Cow<'static, str>, Value>,
    unsafe_params: HashMap<Cow<'static, str>, Value>,
    backtraces: Vec<Backtrace>,
}

/// A standard error type for network services.
///
/// An error consists of several components:
///
/// * The cause of the error, represented as a type implementing the Rust `Error` trait. The cause can either be
///     declared safe or unsafe to log.
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
            error_type.safe_args(),
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
            error_type.safe_args(),
        )
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
            let value = Value::String(value.clone());
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
            ErrorKind::Throttle(ThrottleError { duration: None }),
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
            ErrorKind::Throttle(ThrottleError { duration: None }),
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
            ErrorKind::Unavailable(UnavailableError(())),
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
            ErrorKind::Unavailable(UnavailableError(())),
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

    /// Adds a new safe parameter to the error.
    ///
    /// # Panics
    ///
    /// Panics if the value fails to serialize.
    pub fn with_safe_param<T>(mut self, key: &'static str, value: T) -> Error
    where
        T: Serialize,
    {
        let value = serde_value::to_value(value).expect("value failed to serialize");
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
        let value = serde_value::to_value(value).expect("value failed to serialize");
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

    /// Returns the error's backtraces, ordered from oldest to newest.
    #[inline]
    pub fn backtraces(&self) -> &[Backtrace] {
        &self.0.backtraces
    }
}

/// A collection of error parameters, either safe or unsafe.
pub struct Params<'a>(&'a HashMap<Cow<'static, str>, Value>);

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

impl<'a> Index<&str> for Params<'a> {
    type Output = Value;

    #[inline]
    fn index(&self, key: &str) -> &Value {
        &self.0[key]
    }
}

impl<'a> IntoIterator for &Params<'a> {
    type IntoIter = ParamsIter<'a>;
    type Item = (&'a str, &'a Value);

    #[inline]
    fn into_iter(self) -> ParamsIter<'a> {
        self.iter()
    }
}

/// An iterator over the parameters of an error.
pub struct ParamsIter<'a>(hash_map::Iter<'a, Cow<'static, str>, Value>);

impl<'a> Iterator for ParamsIter<'a> {
    type Item = (&'a str, &'a Value);

    #[inline]
    fn next(&mut self) -> Option<(&'a str, &'a Value)> {
        self.0.next().map(|(a, b)| (&**a, b))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}
