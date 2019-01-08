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
use proptest::{proptest, proptest_helper};

use super::*;

proptest! {
    #[test]
    fn parse_valid_tokens(ref s in "[A-Za-z0-9\\-\\._~\\+/]+=*") {
        s.parse::<BearerToken>().unwrap();
    }
}

#[test]
fn reject_bad_tokens() {
    "".parse::<BearerToken>().unwrap_err();
    "=".parse::<BearerToken>().unwrap_err();
    "==".parse::<BearerToken>().unwrap_err();
    " =".parse::<BearerToken>().unwrap_err();
    "= ".parse::<BearerToken>().unwrap_err();
    "a ".parse::<BearerToken>().unwrap_err();
    " a".parse::<BearerToken>().unwrap_err();
    "a\n".parse::<BearerToken>().unwrap_err();
}

// all that matters is if the value is nonzero or not, but for readability we want to use the char itself
#[test]
fn valid_chars_format() {
    for (i, &b) in VALID_CHARS.iter().enumerate() {
        assert!(b == 0 || b == i as u8);
    }
}
