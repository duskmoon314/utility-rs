use trybuild::TestCases;

#[test]
fn required() {
    let t = TestCases::new();

    t.pass("tests/required/01-ident.rs");
    t.compile_fail("tests/required/02-no-ident.rs");
    t.compile_fail("tests/required/03-no-ident.rs");
    t.compile_fail("tests/required/04-no-ident.rs");
    t.pass("tests/required/05-ident-str.rs");
    t.pass("tests/required/06-derive-empty.rs");
    t.pass("tests/required/07-forward-attrs.rs");
}
