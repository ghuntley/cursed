use cursed::error_enhanced::{CursedError, ErrorKind, test_utils};
use cursed::error::SourceLocation;
use std::error::Error;
use std::io::{Error as IoError, ErrorKind as IoErrorKind};


#[test]
fn test_error_creation() {
    // Test basic error creation
    let err = CursedError::new(ErrorKind::Runtime, "Test error");
    assert_eq!(err.kind(), &ErrorKind::Runtime);
    assert_eq!(err.message(), "Test error");
    
    // Test error with location
    let location = SourceLocation::new(10, 20);
    let err = CursedError::new(ErrorKind::Parser, "Parse error")
        .with_location(location.clone());
    
    assert_eq!(err.location().unwrap().line, 10);
    assert_eq!(err.location().unwrap().column, 20);
    
    // Test error with context
    let err = CursedError::new(ErrorKind::Semantic, "Semantic error")
        .with_context("variable", "x")
        .with_context("scope", "global");
    
    assert_eq!(err.context().len(), 2);
    assert_eq!(err.context()[0].0, "variable");
    assert_eq!(err.context()[0].1, "x");
    assert_eq!(err.context()[1].0, "scope");
    assert_eq!(err.context()[1].1, "global");
    
    // Test error with code
    let err = CursedError::new(ErrorKind::Type, "Type mismatch")
        .with_code("E1001");
    
    assert_eq!(err.code(), Some("E1001"));
}

#[test]
fn test_error_wrapping() {
    // Create a cause error
    let cause = CursedError::new(ErrorKind::Lexer, "Unexpected character");
    
    // Create a wrapper error
    let wrapper_err = CursedError::new(ErrorKind::Parser, "Failed to parse expression")
        .with_cause(cause);
    
    // Check the cause relationship
    let source_error = wrapper_err.source().unwrap();
    let cause_error = source_error.downcast_ref::<CursedError>().unwrap();
    
    assert_eq!(cause_error.kind(), &ErrorKind::Lexer);
    assert_eq!(cause_error.message(), "Unexpected character");
    
    // Test is_kind for wrapped errors
    assert!(wrapper_err.is_kind(&ErrorKind::Parser)); // Direct kind
    assert!(wrapper_err.is_kind(&ErrorKind::Lexer));  // Kind of wrapped error
}

#[test]
fn test_error_full_message() {
    let err = CursedError::new(ErrorKind::Runtime, "Division by zero")
        .with_code("E2001")
        .with_context("operation", "division")
        .with_location(SourceLocation::new(42, 10));
    
    let message = err.full_message();
    
    // Check that the full message contains all the expected parts
    assert!(message.contains("[Runtime/E2001]"));
    assert!(message.contains("Division by zero"));
    assert!(message.contains("line 42, column 10"));
    assert!(message.contains("operation: division"));
}

#[test]
fn test_error_conversion() {
    // Test conversion from &str
    let err: CursedError = "Simple error".into();
    assert_eq!(err.kind(), &ErrorKind::Unknown);
    assert_eq!(err.message(), "Simple error");
    
    // Test conversion from String
    let err: CursedError = String::from("String error").into();
    assert_eq!(err.kind(), &ErrorKind::Unknown);
    assert_eq!(err.message(), "String error");
    
    // Test conversion from std::io::Error
    let io_err = IoError::new(IoErrorKind::NotFound, "File not found");
    let err: CursedError = io_err.into();
    
    assert_eq!(err.kind(), &ErrorKind::IO);
    assert!(err.message().contains("File not found"));
}

#[test]
fn test_error_utils() {
    // Test assertion utilities
    let err = CursedError::new(ErrorKind::Type, "Expected int, got string")
        .with_location(SourceLocation::new(5, 10));
    
    // Test kind assertion
    test_utils::assert_error_kind(err.clone(), ErrorKind::Type);
    
    // Test message assertion
    test_utils::assert_error_message_contains(err.clone(), "Expected int");
    
    // Test location assertion
    test_utils::assert_error_location(err, 5, 10);
    
    // Test creating test errors
    let test_err = test_utils::create_test_error(ErrorKind::Semantic, "Test error");
    assert_eq!(test_err.code(), Some("TEST-001"));
    assert_eq!(test_err.context()[0].0, "test");
    assert_eq!(test_err.context()[0].1, "true");
}

#[test]
fn test_specialized_error_constructors() {
    // Test specialized constructors
    let err = CursedError::runtime("Runtime error");
    assert_eq!(err.kind(), &ErrorKind::Runtime);
    
    let err = CursedError::lexer("Lexer error");
    assert_eq!(err.kind(), &ErrorKind::Lexer);
    
    let err = CursedError::parser("Parser error");
    assert_eq!(err.kind(), &ErrorKind::Parser);
    
    let err = CursedError::type_error("Type error");
    assert_eq!(err.kind(), &ErrorKind::Type);
    
    let location = SourceLocation::new(1, 1);
    let err = CursedError::syntax("Syntax error").with_location(location);
    assert_eq!(err.kind(), &ErrorKind::Syntax);
    assert!(err.location().is_some());
}