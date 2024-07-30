use trybuild::TestCases;

#[test]
fn omit() {
    let t = TestCases::new();

    t.pass("tests/omit/01-ident.rs");
    t.pass("tests/omit/02-zero-arg.rs");
    t.pass("tests/omit/03-multi-args.rs");
    t.compile_fail("tests/omit/04-no-ident.rs");
    t.compile_fail("tests/omit/05-no-fields.rs");
    t.compile_fail("tests/omit/06-empty-fields.rs");
    t.compile_fail("tests/omit/07-field-not-ident.rs");
    t.pass("tests/omit/08-field-not-exist.rs");
    t.pass("tests/omit/09-forward-attrs.rs");
}
