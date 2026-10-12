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
        #[assert_is_safe]
        inner: u64,
    }

    let _ = dummy().with_safe_param("derived", DerivedId { inner: 1 });
}

#[test]
fn containers_propagate_safety() {
    #[derive(serde::Serialize, conjure_object::log_safety::derive::LogSafe)]
    struct Wrapper(#[assert_is_safe] i64);

    let _ = dummy().with_safe_param("opt", Some(Wrapper(1_i64)));
    let _ = dummy().with_safe_param("vec", vec![Wrapper(1_i64), Wrapper(2), Wrapper(3)]);

    let mut m = Vec::new();
    m.insert(0, Wrapper(1_i64));
    let _ = dummy().with_safe_param("map", m);
}

fn assert_log_safe<T: LogSafe + ?Sized>() {}

#[test]
fn recursive_struct_is_safe() {
    #[derive(serde::Serialize, conjure_object::log_safety::derive::LogSafe)]
    struct Unit {
        base_unit: Uuid,
        per_unit: Option<Box<Unit>>,
        #[assert_is_safe]
        value_scalar: Option<f64>,
    }

    assert_log_safe::<Unit>();
}

#[test]
fn mutually_recursive_types_are_safe() {
    #[derive(serde::Serialize, conjure_object::log_safety::derive::LogSafe)]
    struct Node {
        edges: Vec<Edge>,
    }

    #[derive(serde::Serialize, conjure_object::log_safety::derive::LogSafe)]
    enum Edge {
        Next(Box<Node>),
        Leaf(Uuid),
    }

    assert_log_safe::<Node>();
    assert_log_safe::<Edge>();
    let _ = Edge::Next(Box::new(Node { edges: Vec::new() }));
    let _ = Edge::Leaf(Uuid::nil());
}

#[test]
fn recursive_self_type_is_safe() {
    #[derive(serde::Serialize, conjure_object::log_safety::derive::LogSafe)]
    struct Node(Option<Box<Self>>);

    assert_log_safe::<Node>();
}

#[test]
fn generated_recursive_object_is_safe() {
    assert_log_safe::<crate::types::objects::RecursiveObject>();
    assert_log_safe::<crate::exhaustive_types::objects::RecursiveObject>();
}

#[test]
fn recursive_lifetime_and_const_generics_are_safe() {
    #[derive(conjure_object::log_safety::derive::LogSafe)]
    struct Node<'a, const N: usize> {
        values: &'a [Uuid; N],
        next: Option<Box<Self>>,
    }

    assert_log_safe::<Node<'_, 2>>();
    let node = Node {
        values: &[Uuid::nil(); 2],
        next: None,
    };
    let _ = (node.values, node.next);
}

#[test]
fn generic_field_bounds_are_preserved() {
    #[derive(conjure_object::log_safety::derive::LogSafe)]
    struct Wrapper<T>(Vec<T>);

    #[derive(conjure_object::log_safety::derive::LogSafe)]
    struct Asserted<T>(#[assert_is_safe] T);

    #[derive(conjure_object::log_safety::derive::LogSafe)]
    struct SafeWrapper<T>(conjure_object::log_safety::AssertLogSafe<T>);

    #[derive(conjure_object::log_safety::derive::LogSafe)]
    struct Unsized<T: ?Sized>(T);

    assert_log_safe::<Wrapper<Uuid>>();
    assert_log_safe::<Asserted<String>>();
    assert_log_safe::<SafeWrapper<String>>();
    assert_log_safe::<Unsized<dyn LogSafe>>();
    let _ = Wrapper(vec![Uuid::nil()]).0;
}
