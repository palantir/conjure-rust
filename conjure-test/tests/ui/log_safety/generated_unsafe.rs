// Copyright 2026 Palantir Technologies, Inc.
// Licensed under the Apache License, Version 2.0.

fn assert_log_safe<T: conjure_object::log_safety::LogSafe>() {}

fn main() {
    assert_log_safe::<conjure_test::types::objects::UnsafeStringAlias>();
    assert_log_safe::<conjure_test::types::objects::TestObject>();
    assert_log_safe::<conjure_test::types::objects::UnsafeRecursiveA>();
}
