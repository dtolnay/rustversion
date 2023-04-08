#[rustversion::attr(not(nightly), ignore)]
#[cfg_attr(miri, ignore)]
#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}

#[rustversion::attr(not(nightly), ignore)]
#[cfg_attr(miri, ignore)]
#[test]
fn const_impls() {
    let t = trybuild::TestCases::new();
    t.pass("tests/nightly/const_impls.rs");
}
