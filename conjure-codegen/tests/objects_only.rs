// Copyright 2026 Palantir Technologies, Inc.
// Licensed under the Apache License, Version 2.0.
use conjure_codegen::Config;
use std::fs;
use std::path::Path;

#[test]
fn objects_only() {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("objects-only");
    fs::create_dir_all(&dir).unwrap();
    let ir = dir.join("ir.json");
    fs::write(
        &ir,
        r#"{
            "version": 1,
            "types": [{"type": "object", "object": {"typeName": {"name": "Foo", "package": "com.palantir.foo"}}}],
            "errors": [{"errorName": {"name": "FooNotFound", "package": "com.palantir.foo"}, "namespace": "Foo", "code": "NOT_FOUND"}],
            "services": [{"serviceName": {"name": "FooService", "package": "com.palantir.foo"}}],
            "extensions": {"recommended-product-dependencies": [{"product-group": "com.palantir.foo", "product-name": "foo", "minimum-version": "1.0.0", "maximum-version": "1.x.x"}]}
        }"#,
    )
    .unwrap();

    let out = dir.join("crate");
    Config::new()
        .objects_only(true)
        .build_crate("foo-api", "1.0.0")
        .generate_files(&ir, &out)
        .unwrap();

    let manifest = fs::read_to_string(out.join("Cargo.toml"))
        .unwrap()
        .parse::<toml::Table>()
        .unwrap();
    assert!(manifest["package"].get("metadata").is_none());
    assert_eq!(
        manifest["dependencies"]
            .as_table()
            .unwrap()
            .keys()
            .collect::<Vec<_>>(),
        ["conjure-error", "conjure-object"],
    );

    let lib = fs::read_to_string(out.join("src/lib.rs")).unwrap();
    assert_eq!(
        lib.lines()
            .filter(|l| l.starts_with("pub mod "))
            .collect::<Vec<_>>(),
        ["pub mod errors;", "pub mod objects;"],
    );
}
