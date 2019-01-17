//! JSON serialization support.
//!
//! Conjure specifies behavior that differs from serde_json's in a couple of ways:
//!
//! * serde_json serializes non-finite floating point values as `null`, while Conjure specifies `"Infinity"`,
//!     `"-Infinity"`, and `"NaN"` as appropriate.
//! * serde_json serializes byte sequences as arrays of numbers, while Conjure specifies Base64-encoded strings.
//!
//! Additionally, Conjure clients should ignore unknown fields while Conjure servers should trigger errors.
//!
//! This crate provides `Serializer` and `Deserializer` implementations which wrap serde_json's and handle these special
//! behaviors.

pub use crate::json::de::client::{
    client_from_reader, client_from_slice, client_from_str, ClientDeserializer,
};
pub use crate::json::de::server::{
    server_from_reader, server_from_slice, server_from_str, ServerDeserializer,
};
pub use crate::json::ser::{to_string, to_vec, to_writer, Serializer};

mod de;
mod ser;
#[cfg(test)]
mod test;
