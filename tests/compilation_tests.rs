fn source_path(file_name: &str) -> String {
    format!("tests/compilation_test_sources/{}", file_name)
}

#[test]
fn compilation_tests() {
    let tests = trybuild::TestCases::new();
    // Basic valid usages
    tests.pass(source_path("struct_implements_send_requires_send.rs"));
    tests.pass(source_path(
        "struct_implements_send_sync_requires_send_sync.rs",
    ));
    tests.pass(source_path("enum_implements_send_requires_send.rs"));
    tests.pass(source_path(
        "enum_implements_send_sync_requires_send_sync.rs",
    ));

    // Invalid usages
    tests.compile_fail(source_path("missing_param.rs"));
    tests.compile_fail(source_path("struct_generics.rs"));

    // Basic true positives
    tests.compile_fail(source_path(
        "struct_violates_send_custom_requires_send_sync.rs",
    ));
    tests.compile_fail(source_path("struct_violates_send_custom_requires_send.rs"));
    tests.compile_fail(source_path(
        "struct_violates_send_sync_rc_requires_send_sync.rs",
    ));

    // Valid usage variations
    tests.pass(source_path(
        "struct_implements_trait_derives_after_must_implement.rs",
    ));
    tests.compile_fail(source_path(
        "struct_violates_trait_derives_after_must_implement.rs",
    ));
    tests.pass(source_path(
        "struct_implements_traits_separate_attributes.rs",
    ));
    tests.pass(source_path("struct_where_no_generics.rs"));
}
