// Copyright 2026 Palantir Technologies, Inc.
// Licensed under the Apache License, Version 2.0.

struct Unmarked<T>(T);

#[derive(conjure_object::log_safety::derive::LogSafe)]
struct Node {
    next: Option<Box<Unmarked<Node>>>,
}

fn main() {}
