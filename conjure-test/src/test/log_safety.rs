// Copyright 2026 Palantir Technologies, Inc.
// Licensed under the Apache License, Version 2.0.

use conjure_error::Error;
use conjure_object::log_safety::Safe;
use conjure_object::{ResourceIdentifier, SafeLong, Uuid};

fn dummy() -> Error {
    Error::internal_safe("test error")
}

#[test]
fn conjure_object_types_are_safe() {
    let _ = dummy().with_safe_param(
        "rid",
        "ri.foo.bar.baz.qux".parse::<ResourceIdentifier>().unwrap(),
    );
    let _ = dummy().with_safe_param("uuid", Uuid::nil());
}

#[test]
fn manual_impl_is_accepted() {
    #[derive(serde::Serialize)]
    struct CustomId(u64);

    impl Safe for CustomId {}

    let _ = dummy().with_safe_param("custom", CustomId(99));
}

#[test]
fn derive_is_accepted() {
    #[derive(serde::Serialize, conjure_object::log_safety::Safe)]
    struct DerivedId {
        inner: u64,
    }

    let _ = dummy().with_safe_param("derived", DerivedId { inner: 1 });
}

#[test]
fn containers_propagate_safety() {
    #[derive(serde::Serialize, conjure_object::log_safety::Safe)]
    struct Wrapper(i64);

    let _ = dummy().with_safe_param("opt", Some(Wrapper(1_64)));
    let _ = dummy().with_safe_param("vec", vec![Wrapper(1_64), Wrapper(2), Wrapper(3)]);

    let mut m = std::collections::BTreeMap::new();
    m.insert(Wrapper(1_64), true);
    let _ = dummy().with_safe_param("map", m);
}
