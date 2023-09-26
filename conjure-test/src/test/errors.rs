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

use conjure_error::{ErrorCode, ErrorType};
use std::collections::BTreeMap;

use crate::types::*;

#[test]
fn error_serialization() {
    #[allow(deprecated)]
    let error = SimpleError::builder()
        .foo("hello")
        .bar(15)
        .baz(EmptyObject::new())
        .unsafe_foo(false)
        .build();

    assert_eq!(error.code(), ErrorCode::Internal);
    assert_eq!(error.name(), "Test:SimpleError");
    assert_eq!(error.safe_args(), &["bar", "baz", "foo"]);

    let encoded = conjure_error::encode(&error);

    assert_eq!(*encoded.error_code(), ErrorCode::Internal);
    assert_eq!(encoded.error_name(), "Test:SimpleError");

    let mut params = BTreeMap::new();
    params.insert("foo".to_string(), "hello".to_string());
    params.insert("bar".to_string(), "15".to_string());
    params.insert("unsafeFoo".to_string(), "false".to_string());
    assert_eq!(*encoded.parameters(), params);
}
