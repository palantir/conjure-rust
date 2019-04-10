use std::env;
use std::path::PathBuf;

fn main() {
    let input = "test-ir.json";
    let output = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("conjure");

    println!("cargo:rerun-if-changed={}", input);
    conjure_codegen::Config::new()
        .run_rustfmt(false)
        .strip_prefix("com.palantir.conjure".to_string())
        .generate_files(input, output)
        .unwrap();
}
