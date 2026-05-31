// Copyright 2026 Palantir Technologies, Inc.
// Licensed under the Apache License, Version 2.0.

use conjure_error::Error;
use conjure_object::log_safety::LogSafe;
use conjure_object::{ResourceIdentifier, Uuid};
use std::vec;

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

    impl LogSafe for CustomId {}

    let _ = dummy().with_safe_param("custom", CustomId(99));
}

#[test]
fn derive_is_accepted() {
    #[derive(serde::Serialize, conjure_object::log_safety::derive::LogSafe)]
    struct DerivedId {
        inner: u64,
    }

    let _ = dummy().with_safe_param("derived", DerivedId { inner: 1 });
}

#[test]
fn containers_propagate_safety() {
    #[derive(serde::Serialize, conjure_object::log_safety::derive::LogSafe)]
    struct Wrapper(i64);

    let _ = dummy().with_safe_param("opt", Some(Wrapper(1_i64)));
    let _ = dummy().with_safe_param("vec", vec![Wrapper(1_i64), Wrapper(2), Wrapper(3)]);

    let mut m = vec::Vec::new();
    m.insert(0, Wrapper(1_i64));
    let _ = dummy().with_safe_param("map", m);
}
