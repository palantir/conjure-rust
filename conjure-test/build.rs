use std::env;
use std::path::PathBuf;

fn main() {
    let input = concat!(env!("CARGO_MANIFEST_DIR"), "/test-ir.json");
    conjure_codegen::Config::new()
        .run_rustfmt(false)
        .generate_files(
            input,
            PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("conjure"),
        )
        .unwrap();
    println!("cargo:rerun-if-changed={}", input);
}
