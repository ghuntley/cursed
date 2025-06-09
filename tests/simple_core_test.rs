use cursed::error::Error;

#[test]
fn test_error_creation() {
    let error = Error::repl_error("test error".to_string());
    assert!(error.to_string().contains("test error"));
}

#[test]
fn test_error_formatting() {
    let error = Error::repl_error("formatting test".to_string());
    let formatted = format!("{}", error);
    assert!(formatted.contains("formatting test"));
}
