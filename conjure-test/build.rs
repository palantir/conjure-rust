use std::env;
use std::path::PathBuf;

fn main() {
    let input = "test-ir.json";
    println!("cargo:rerun-if-changed={input}");

    let output = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("conjure");
    conjure_codegen::Config::new()
        .strip_prefix("com.palantir.conjure".to_string())
        .generate_files(input, output)
        .unwrap();

    let output = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("conjure-exhaustive");
    conjure_codegen::Config::new()
        .strip_prefix("com.palantir.conjure".to_string())
        .exhaustive(true)
        .generate_files(input, output)
        .unwrap();
}
