// Copyright 2026 Palantir Technologies, Inc.
// Licensed under the Apache License, Version 2.0.

#[test]
fn unsafe_fields_are_rejected() {
    trybuild::TestCases::new().compile_fail("tests/ui/log_safety/*.rs");
}
