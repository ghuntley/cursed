/// Test for error module functionality
use cursed::error::{Error, Result, CursedError};

#[test]
fn test_error_types() {
    let io_error = Error::Io("test io error".to_string());
    let parse_error = Error::Parse("test parse error".to_string());
    let type_error = Error::Type("test type error".to_string());
    
    assert!(matches!(io_error, Error::Io(_)));
    assert!(matches!(parse_error, Error::Parse(_)));
    assert!(matches!(type_error, Error::Type(_)));
}

#[test]
fn test_result_type() {
    let success: Result<i32> = Ok(42);
    let failure: Result<i32> = Err(Error::General("test error".to_string()));
    
    assert!(success.is_ok());
    assert!(failure.is_err());
    assert_eq!(success.unwrap(), 42);
}

#[test]
fn test_cursed_error_alias() {
    let error: CursedError = Error::Runtime("test runtime error".to_string());
    assert!(matches!(error, Error::Runtime(_)));
}

#[test]
fn test_error_display() {
    let error = Error::Type("variable 'x' has type 'int' but expected 'string'".to_string());
    let error_string = format!("{}", error);
    assert!(error_string.contains("Type error"));
    assert!(error_string.contains("variable 'x'"));
}

#[test]
fn test_error_from_string() {
    let error: Error = "test error message".into();
    assert!(matches!(error, Error::Runtime(_)));
    
    let error2: Error = "another error".to_string().into();
    assert!(matches!(error2, Error::Runtime(_)));
}
