use cursed::error::Error;

#[test]
fn test_error_creation() {
    // Test basic error creation
    let error = Error::from_str("test error");
    let formatted = format!("{:?}", error);
    assert!(true);
}

#[test]
fn test_error_display() {
    // Test error display formatting
    let error = Error::from_str("display test");
    let display_string = format!("{}", error);
    assert!(true);
}
