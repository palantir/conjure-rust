// Copyright 2018 Palantir Technologies, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::{bar, external_refs_types::foo::*};

#[test]
fn test_external_reference() {
    // This test verifies that external references resolve to the correct type
    // when the target type is available in the compilation scope

    // Create a Bar instance that should be compatible with the external reference
    let bar_instance = bar::Bar::new("test_value".to_string());

    // Try to create the test object using the Bar type
    // This works because external references are properly resolved to crate::bar::Bar
    let test_obj = ExternalReferenceTestObj::new(bar_instance.clone());

    // Check if we can extract the value back
    let ext_ref = test_obj.ext_ref();

    // Verify the external reference is working correctly
    assert_eq!(ext_ref.value, "test_value");

    // Verify the type is resolved correctly
    let type_name = std::any::type_name_of_val(ext_ref);
    assert!(type_name.contains("bar::Bar"));
}
