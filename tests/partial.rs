use trybuild::TestCases;

#[test]
fn partial() {
    let t = TestCases::new();

    t.pass("tests/partial/01-ident.rs");
    t.compile_fail("tests/partial/02-no-ident.rs");
    t.compile_fail("tests/partial/03-no-ident.rs");
    t.compile_fail("tests/partial/04-no-ident.rs");
    t.pass("tests/partial/05-ident-str.rs");
    t.pass("tests/partial/06-derive-empty.rs");
}
