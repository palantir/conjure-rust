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
mod types {
    include!(concat!(env!("OUT_DIR"), "/conjure/mod.rs"));
}

#[allow(dead_code, unused_imports, clippy::all)]
mod staged_types {
    include!(concat!(env!("OUT_DIR"), "/conjure-staged/mod.rs"));
}

#[allow(dead_code, unused_imports, clippy::all)]
mod exhaustive_types {
    include!(concat!(env!("OUT_DIR"), "/conjure-exhaustive/mod.rs"));
}
