#[test]
fn compilation_tests() {
    let tests = trybuild::TestCases::new();
    tests.pass("tests/compilation_test_sources/struct_implements_send_requires_send.rs");
    tests.pass("tests/compilation_test_sources/struct_implements_send_sync_requires_send_sync.rs");
    tests.pass("tests/compilation_test_sources/enum_implements_send_requires_send.rs");
    tests.pass("tests/compilation_test_sources/enum_implements_send_sync_requires_send_sync.rs");

    tests.compile_fail("tests/compilation_test_sources/missing_param.rs");
    tests.compile_fail(
        "tests/compilation_test_sources/struct_violates_send_custom_requires_send_sync.rs",
    );
    tests.compile_fail(
        "tests/compilation_test_sources/struct_violates_send_custom_requires_send.rs",
    );
    tests.compile_fail(
        "tests/compilation_test_sources/struct_violates_send_sync_rc_requires_send_sync.rs",
    );
}
