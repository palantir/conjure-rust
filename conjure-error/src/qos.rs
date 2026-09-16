// Copyright 2026 Palantir Technologies, Inc.
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

use std::borrow::Cow;
use std::convert::Infallible;
use std::fmt;
use std::str::FromStr;

/// Describes why a QoS error was created and how clients should handle it.
///
/// Reason strings should be low-cardinality, safe-to-log labels suitable for metric tags.
/// They are local diagnostic metadata, not transmitted to clients.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct QosReason {
    reason: &'static str,
    retry_hint: Option<QosRetryHint>,
    due_to: Option<QosDueTo>,
}

impl QosReason {
    /// Creates a reason with no retry or scope override.
    ///
    /// # Panics
    ///
    /// Panics unless the reason contains 1 to 50 lowercase ASCII letters, digits, or hyphens.
    pub fn new(reason: &'static str) -> QosReason {
        assert!(
            !reason.is_empty()
                && reason.len() <= 50
                && reason
                    .bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-'),
            "QoS reason must contain 1 to 50 lowercase ASCII letters, digits, or hyphens"
        );
        QosReason {
            reason,
            retry_hint: None,
            due_to: None,
        }
    }

    /// Sets the server's retry recommendation.
    #[inline]
    pub fn with_retry_hint(mut self, retry_hint: QosRetryHint) -> QosReason {
        self.retry_hint = Some(retry_hint);
        self
    }

    /// Sets the scope responsible for the QoS error.
    #[inline]
    pub fn with_due_to(mut self, due_to: QosDueTo) -> QosReason {
        self.due_to = Some(due_to);
        self
    }

    /// Returns the local diagnostic label.
    #[inline]
    pub fn reason(&self) -> &'static str {
        self.reason
    }

    /// Returns the server's retry recommendation, if provided.
    #[inline]
    pub fn retry_hint(&self) -> Option<&QosRetryHint> {
        self.retry_hint.as_ref()
    }

    /// Returns the scope responsible for the QoS error, if provided.
    #[inline]
    pub fn due_to(&self) -> Option<&QosDueTo> {
        self.due_to.as_ref()
    }
}

impl fmt::Display for QosReason {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(self.reason)
    }
}

/// A best-effort recommendation on whether clients should retry a QoS error.
///
/// Parses and displays `Qos-Retry-Hint` values, preserving unknown values for propagation.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct QosRetryHint(Cow<'static, str>);

impl QosRetryHint {
    /// Clients should propagate the failure without retrying it.
    pub const DO_NOT_RETRY: QosRetryHint = QosRetryHint(Cow::Borrowed("do-not-retry"));

    /// Returns the wire representation.
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for QosRetryHint {
    type Err = Infallible;

    fn from_str(value: &str) -> Result<QosRetryHint, Infallible> {
        if value.eq_ignore_ascii_case(Self::DO_NOT_RETRY.as_str()) {
            Ok(Self::DO_NOT_RETRY)
        } else {
            Ok(QosRetryHint(Cow::Owned(value.to_string())))
        }
    }
}

impl fmt::Display for QosRetryHint {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(self.as_str())
    }
}

/// The scope responsible for a QoS error.
///
/// Without an override, clients treat a 503 as node-wide overload and a 429 as endpoint-specific overload.
/// Parses and displays `Qos-Due-To` values, preserving unknown values for propagation.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct QosDueTo(Cow<'static, str>);

impl QosDueTo {
    /// A limit scoped to a user, user-agent, or resource rather than a node or endpoint.
    ///
    /// Clients should not adjust shared concurrency limits or node health in response to this error.
    pub const CUSTOM: QosDueTo = QosDueTo(Cow::Borrowed("custom"));

    /// Returns the wire representation.
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for QosDueTo {
    type Err = Infallible;

    fn from_str(value: &str) -> Result<QosDueTo, Infallible> {
        if value.eq_ignore_ascii_case(Self::CUSTOM.as_str()) {
            Ok(Self::CUSTOM)
        } else {
            Ok(QosDueTo(Cow::Owned(value.to_string())))
        }
    }
}

impl fmt::Display for QosDueTo {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(self.as_str())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn reason() {
        let reason = QosReason::new("resource-limit");
        assert_eq!(reason.reason(), "resource-limit");
        assert_eq!(reason.to_string(), "resource-limit");
        assert_eq!(reason.due_to(), None);
        assert_eq!(reason.retry_hint(), None);

        let reason = reason.with_due_to(QosDueTo::CUSTOM);
        assert_eq!(reason.due_to(), Some(&QosDueTo::CUSTOM));
        assert_eq!(reason.retry_hint(), None);

        let reason = reason.with_retry_hint(QosRetryHint::DO_NOT_RETRY);
        assert_eq!(reason.due_to(), Some(&QosDueTo::CUSTOM));
        assert_eq!(reason.retry_hint(), Some(&QosRetryHint::DO_NOT_RETRY));
        assert_eq!(reason.reason(), "resource-limit");

        let reason = QosReason::new("no-retry").with_retry_hint(QosRetryHint::DO_NOT_RETRY);
        assert_eq!(reason.retry_hint(), Some(&QosRetryHint::DO_NOT_RETRY));
        assert_eq!(reason.due_to(), None);
    }

    #[test]
    fn reason_validation() {
        for valid in [
            "a",
            "model-123",
            "12345678901234567890123456789012345678901234567890",
        ] {
            assert_eq!(QosReason::new(valid).reason(), valid);
        }
        for invalid in [
            "",
            "Uppercase",
            "under_score",
            "two words",
            "non-ascii-é",
            "newline\n",
            "123456789012345678901234567890123456789012345678901",
        ] {
            assert!(std::panic::catch_unwind(|| QosReason::new(invalid)).is_err());
        }
    }

    #[test]
    fn known_values_are_case_insensitive() {
        for value in ["custom", "CUSTOM", "Custom"] {
            let due_to: QosDueTo = value.parse().unwrap();
            assert_eq!(due_to, QosDueTo::CUSTOM);
            assert_eq!(due_to.as_str(), "custom");
            assert_eq!(due_to.to_string(), "custom");
        }
        for value in ["do-not-retry", "DO-NOT-RETRY", "Do-Not-Retry"] {
            let retry_hint: QosRetryHint = value.parse().unwrap();
            assert_eq!(retry_hint, QosRetryHint::DO_NOT_RETRY);
            assert_eq!(retry_hint.as_str(), "do-not-retry");
            assert_eq!(retry_hint.to_string(), "do-not-retry");
        }
    }

    #[test]
    fn unknown_values_are_preserved() {
        let due_to: QosDueTo = "Future-Scope".parse().unwrap();
        let retry_hint: QosRetryHint = "Future-Hint".parse().unwrap();
        let reason = QosReason::new("client-qos-response")
            .with_due_to(due_to.clone())
            .with_retry_hint(retry_hint.clone());
        assert_ne!(due_to, QosDueTo::CUSTOM);
        assert_ne!(retry_hint, QosRetryHint::DO_NOT_RETRY);
        assert_eq!(reason.due_to().unwrap().as_str(), "Future-Scope");
        assert_eq!(reason.retry_hint().unwrap().as_str(), "Future-Hint");
        assert_eq!(due_to.to_string().parse::<QosDueTo>().unwrap(), due_to);
        assert_eq!(
            retry_hint.to_string().parse::<QosRetryHint>().unwrap(),
            retry_hint
        );
    }
}
