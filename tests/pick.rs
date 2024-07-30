use trybuild::TestCases;

#[test]
fn pick() {
    let t = TestCases::new();

    t.pass("tests/pick/01-ident.rs");
    t.pass("tests/pick/02-zero-arg.rs");
    t.pass("tests/pick/03-multi-args.rs");
    t.compile_fail("tests/pick/04-no-ident.rs");
    t.compile_fail("tests/pick/05-no-fields.rs");
    t.compile_fail("tests/pick/06-empty-fields.rs");
    t.compile_fail("tests/pick/07-field-not-ident.rs");
    t.pass("tests/pick/08-field-not-exist.rs");
    t.pass("tests/pick/09-forward-attrs.rs");
}
