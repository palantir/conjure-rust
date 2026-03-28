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
use serde::ser;
use serde_cbor_2::Error;
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
