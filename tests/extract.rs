use trybuild::TestCases;

#[test]
fn extract() {
    let t = TestCases::new();

    t.pass("tests/extract/01-ident.rs");
    t.pass("tests/extract/02-zero-arg.rs");
    t.pass("tests/extract/03-multi-args.rs");
    t.compile_fail("tests/extract/04-no-ident.rs");
    t.compile_fail("tests/extract/05-empty-ident.rs");
    t.compile_fail("tests/extract/06-no-variants.rs");
    t.compile_fail("tests/extract/07-empty-variants.rs");
    t.pass("tests/extract/08-variant-not-exist.rs");
    t.pass("tests/extract/09-forward-attrs.rs");
}
