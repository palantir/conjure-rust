#!/bin/bash
set -eux

cargo build -p conjure-rust

rm -rf example-api
./target/debug/conjure-rust generate --strip-prefix com.palantir --crate-name example-api --crate-version 0.1.0 conjure-codegen/example-types-ir.json example-api
rm -rf conjure-codegen/src/exmaple-types
./target/debug/conjure-rust generate --strip-prefix com.palantir conjure-codegen/example-types-ir.json conjure-codegen/src/example_types
rm -rf conjure-codegen/src/types
./target/debug/conjure-rust generate --strip-prefix com.palantir.conjure.spec --exhaustive conjure-codegen/conjure-api-4.14.0.conjure.json conjure-codegen/src/types
rm -rf conjure-error/src/types
./target/debug/conjure-rust generate --strip-prefix com.palantir.conjure.error --exhaustive conjure-error/error-types.conjure.json conjure-error/src/types
