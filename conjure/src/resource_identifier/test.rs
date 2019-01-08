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
use crate::ResourceIdentifier;

#[test]
fn basic() {
    let rid = ResourceIdentifier::new("ri.service.instance.type.locator.locator").unwrap();
    assert_eq!(rid.service(), "service");
    assert_eq!(rid.instance(), "instance");
    assert_eq!(rid.type_(), "type");
    assert_eq!(rid.locator(), "locator.locator");
}

#[test]
fn empty_instance() {
    let rid = ResourceIdentifier::new("ri.service..type.locator").unwrap();
    assert_eq!(rid.service(), "service");
    assert_eq!(rid.instance(), "");
    assert_eq!(rid.type_(), "type");
    assert_eq!(rid.locator(), "locator");
}

#[test]
fn no_multi_components() {
    assert!(ResourceIdentifier::from_components("a.b", "c", "d", "e").is_err());
    assert!(ResourceIdentifier::from_components("a", "b.c", "d", "e").is_err());
    assert!(ResourceIdentifier::from_components("a", "b", "c.d", "e").is_err());
}
