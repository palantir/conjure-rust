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

#[cfg(test)]
mod test;

#[allow(dead_code, unused_imports, clippy::all)]
pub mod types {
    include!(concat!(env!("OUT_DIR"), "/conjure/mod.rs"));
}

#[allow(dead_code, unused_imports, clippy::all)]
pub mod exhaustive_types {
    include!(concat!(env!("OUT_DIR"), "/conjure-exhaustive/mod.rs"));
}

#[allow(dead_code, unused_imports, clippy::all)]
pub mod external_refs_types {
    include!(concat!(env!("OUT_DIR"), "/conjure-external-refs/mod.rs"));
}

// Provide the external reference target that should be resolved
// This needs to be at the root level where the generated code can find it
#[allow(dead_code)]
pub mod bar {
    use conjure_object::serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
    #[serde(crate = "conjure_object::serde")]
    pub struct Bar {
        pub value: String,
    }

    impl Bar {
        pub fn new(value: String) -> Self {
            Self { value }
        }
    }
}
