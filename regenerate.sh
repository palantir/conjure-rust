#!/bin/bash
set -eux

cargo build -p conjure-rust

rm -rf conjure-codegen/src/example-types
./target/debug/conjure-rust generate --stripPrefix com.palantir conjure-codegen/example-types-ir.json conjure-codegen/src/example_types
rm -rf conjure-codegen/src/types
./target/debug/conjure-rust generate --stripPrefix com.palantir.conjure.spec --exhaustive conjure-codegen/conjure-api-4.32.0.conjure.json conjure-codegen/src/types
rm -rf conjure-error/src/types
./target/debug/conjure-rust generate --stripPrefix com.palantir.conjure.error --exhaustive conjure-error/error-types.conjure.json conjure-error/src/types
