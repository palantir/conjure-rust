#!/bin/bash
set -eux

cargo run -p conjure-rust generate --exhaustive conjure-codegen/conjure-api-4.4.0.conjure.json conjure-codegen/src/types
cargo run -p conjure-rust generate --conjure-path crate conjure/example-types-ir.json conjure/src/example_types
