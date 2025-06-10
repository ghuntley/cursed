use cursed::error::Error;

#[test]
fn test_error_creation() {
    let error = Error::from_str("test");
    let formatted = format!("{:?}", error);
    assert!(formatted.contains("test"));
}