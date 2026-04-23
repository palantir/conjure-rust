#[test]
fn compile_fail_tests() {
    let t = trybuild::TestCases::new();
    t.compile_fail("src/compile_fail_tests/files/1-conjure-client.rs");
}
