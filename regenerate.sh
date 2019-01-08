#!/bin/bash
set -eux

cargo run -p conjure-rust generate --conjure-types-path crate conjure-types/example-types-ir.json conjure-types/src/example_types
cargo run -p conjure-rust generate --exhaustive conjure-codegen/conjure-api-4.4.0.conjure.json conjure-codegen/src/types
