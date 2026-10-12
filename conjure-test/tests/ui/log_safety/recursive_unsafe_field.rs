// Copyright 2026 Palantir Technologies, Inc.
// Licensed under the Apache License, Version 2.0.

#[derive(conjure_object::log_safety::derive::LogSafe)]
struct Node {
    value: String,
    next: Option<Box<Node>>,
}

fn main() {}
