use trybuild::TestCases;

#[test]
fn exclude() {
    let t = TestCases::new();

    t.pass("tests/exclude/01-ident.rs");
    t.pass("tests/exclude/02-zero-arg.rs");
    t.pass("tests/exclude/03-multi-args.rs");
    t.compile_fail("tests/exclude/04-no-ident.rs");
    t.compile_fail("tests/exclude/05-empty-ident.rs");
    t.compile_fail("tests/exclude/06-no-variants.rs");
    t.compile_fail("tests/exclude/07-empty-variants.rs");
    t.pass("tests/exclude/08-variant-not-exist.rs");
}
