use cursed::error::Error;

#[test]
fn test_error_creation() {
    let error = Error::from_str("test error");
    assert!(error.to_string().contains("test error"));
}

#[test]
fn test_error_formatting() {
    let error = Error::from_str("formatting test");
    let formatted = format!("{}", error);
    assert!(formatted.contains("formatting test"));
}
