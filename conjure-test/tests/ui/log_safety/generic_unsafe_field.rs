// Copyright 2026 Palantir Technologies, Inc.
// Licensed under the Apache License, Version 2.0.

#[derive(conjure_object::log_safety::derive::LogSafe)]
struct Wrapper<T>(Vec<T>);

fn assert_log_safe<T: conjure_object::log_safety::LogSafe>() {}

fn main() {
    assert_log_safe::<Wrapper<String>>();
}
