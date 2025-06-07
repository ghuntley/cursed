use cursed::run_file;


#[test]
fn test_vibez_functions() {
    let result = run_file("tests/vibez_test.csd");
    assert!(
        result.is_ok(),
        "Failed to run vibez_test.csd: {:?}",
        result.err()
    );
}

#[test]
fn test_scan_functions() {
    let result = run_file("tests/scan_functions_test.csd");
    assert!(
        result.is_ok(),
        "Failed to run scan_functions_test.csd: {:?}",
        result.err()
    );
}

#[test]
fn test_dropz_file_functions() {
    let result = run_file("tests/dropz_file_test.csd");
    assert!(
        result.is_ok(),
        "Failed to run dropz_file_test.csd: {:?}",
        result.err()
    );
}
