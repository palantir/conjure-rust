#!/bin/bash
set -eux

cargo run -p conjure-rust generate --strip-prefix com.palantir.product conjure-codegen/example-types-ir.json conjure-codegen/src/example_types
cargo run -p conjure-rust generate --strip-prefix com.palantir.conjure --exhaustive conjure-codegen/conjure-api-4.4.0.conjure.json conjure-codegen/src/types
