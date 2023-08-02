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

use conjure_object::Any;
use serde::Serialize;
use std::collections::hash_map::{self, HashMap};
use std::fmt;
use std::ops::Index;
use std::time::Duration;
use std::{backtrace, error};

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
    safe_params: HashMap<String, Any>,
    unsafe_params: HashMap<String, Any>,
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
            let key = key.clone();
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
    pub fn with_safe_param<K, V>(mut self, key: K, value: V) -> Error
    where
        K: Into<String>,
        V: Serialize,
    {
        let value = Any::new(value).expect("value failed to serialize");
        self.0.safe_params.insert(key.into(), value);
        self
    }

    /// Adds a new unsafe parameter to the error.
    ///
    /// # Panics
    ///
    /// Panics if the value fails to serialize.
    pub fn with_unsafe_param<K, V>(mut self, key: K, value: V) -> Error
    where
        K: Into<String>,
        V: Serialize,
    {
        let value = Any::new(value).expect("value failed to serialize");
        self.0.unsafe_params.insert(key.into(), value);
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

    /// Adds a new custom backtrace to the head of the list.
    #[inline]
    pub fn with_prepended_custom_backtrace(mut self, backtrace: String) -> Error {
        self.0.backtraces.insert(0, Backtrace::custom(backtrace));
        self
    }

    /// Adds a new custom backtrace to the error.
    #[inline]
    pub fn with_custom_backtrace(mut self, backtrace: String) -> Error {
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
pub struct Params<'a>(&'a HashMap<String, Any>);

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
pub struct ParamsIter<'a>(hash_map::Iter<'a, String, Any>);

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
        Backtrace(BacktraceInner::Rust(backtrace::Backtrace::force_capture()))
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
