#!/bin/bash
set -eux

cargo build -p conjure-rust


rm -rf conjure-codegen/src/exmaple-types
./target/debug/conjure-rust generate --strip-prefix com.palantir conjure-codegen/example-types-ir.json conjure-codegen/src/example_types
rm -rf conjure-codegen/src/types
./target/debug/conjure-rust generate --strip-prefix com.palantir.conjure.spec --exhaustive conjure-codegen/conjure-api-4.4.0.conjure.json conjure-codegen/src/types
rm -rf conjure-error/src/types
./target/debug/conjure-rust generate --strip-prefix com.palantir.conjure.error --exhaustive conjure-error/error-types.conjure.json conjure-error/src/types
