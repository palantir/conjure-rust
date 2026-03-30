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
use serde::ser::{self, SerializeMap};
use serde::{Serialize, Serializer};
use serde_cbor_2::Error;
use std::collections::BTreeMap;
use std::io::Write;

/// Serializes a value as CBOR into a byte buffer.
pub fn to_vec<T>(value: &T) -> Result<Vec<u8>, Error>
where
    T: ser::Serialize,
{
    serde_cbor_2::to_vec(value)
}

/// Serializes a value as CBOR into a writer.
pub fn to_writer<W, T>(writer: W, value: &T) -> Result<(), Error>
where
    W: Write,
    T: ser::Serialize,
{
    serde_cbor_2::to_writer(writer, value)
}

/// Serializes a `BTreeMap<Uuid, V>` with UUID keys as strings for Java compatibility.
///
/// Use this with `#[serde(serialize_with = "conjure_serde::cbor::serialize_uuid_map")]`
/// on fields with UUID keys to ensure Java can deserialize them.
pub fn serialize_uuid_map<V, S>(
    map: &BTreeMap<conjure_object::Uuid, V>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    V: Serialize,
    S: Serializer,
{
    let mut ser_map = serializer.serialize_map(Some(map.len()))?;
    for (key, value) in map {
        // Serialize UUID keys as strings for Java compatibility
        let key_str: String = key.to_string();
        ser_map.serialize_entry(&key_str, value)?;
    }
    ser_map.end()
}
