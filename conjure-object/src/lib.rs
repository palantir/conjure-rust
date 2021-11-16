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

//! Rust implementations of Conjure types.
//!
//! This crate consists of reexports and definitions of the Rust types that correspond to Conjure types. It is a
//! required dependency of crates which contain Conjure-generated code.
#![warn(clippy::all, missing_docs)]
#![doc(html_root_url = "https://docs.rs/conjure-object/0.6")]

pub use chrono::{self, DateTime, Utc};
pub use serde;
pub use serde_bytes::{self, ByteBuf};
pub use uuid::{self, Uuid};

#[doc(inline)]
pub use crate::any::Any;
#[doc(inline)]
pub use crate::bearer_token::BearerToken;
#[doc(inline)]
pub use crate::double_key::{AsDouble, DoubleKey};
#[doc(inline)]
pub use crate::plain::{FromPlain, Plain, ToPlain};
#[doc(inline)]
pub use crate::resource_identifier::ResourceIdentifier;
#[doc(inline)]
pub use crate::safe_long::SafeLong;

pub mod any;
pub mod bearer_token;
pub mod double_key;
pub mod plain;
pub mod resource_identifier;
pub mod safe_long;

#[doc(hidden)]
pub mod private;
