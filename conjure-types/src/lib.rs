// Copyright 2018 Palantir Technologies, Inc.
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
#![warn(clippy::all)]

pub use chrono::{self, DateTime, Utc};
pub use serde;
pub use serde_bytes::{self, ByteBuf};
pub use serde_value::{self, Value};
pub use uuid::{self, Uuid};

pub use crate::bearer_token::BearerToken;
pub use crate::resource_identifier::ResourceIdentifier;
pub use crate::safe_long::{SafeLong, SafeLongError};

mod bearer_token;
#[doc(hidden)]
pub mod private;
mod resource_identifier;
mod safe_long;

/// Examples of generated conjure code.
///
/// This module is only intended to be present in documentation; it shouldn't be relied on by any library code.
#[cfg(feature = "example-types")]
pub mod example_types;
