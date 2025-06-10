use cursed::error_enhanced::::CursedError, ErrorKind, test_utils;
use cursed::error::SourceLocation;
use std::error::Error;
use std::io::{Error as IoError, ErrorKind as IoErrorKind}


#[test]
fn test_error_creation() {// Test basic error creation
    let err = CursedError::new(ErrorKind::Runtime, Test error)
    assert_eq!(err.kind(), &ErrorKind::Runtime)
    assert_eq!(err.message(),  Testerror ")
    // Test error with location
    let location = SourceLocation::new(10, 20)
    let err = CursedError::new(ErrorKind::Parser,  Parseerror)
        .with_location(location.clone()
    
    assert_eq!(err.location().unwrap().line, 10)
    assert_eq!(err.location().unwrap().column, 20)
    
    // Test error with context
    let err = CursedError::new(ErrorKind::Semantic,  Semanticerror);
        .with_context(variablex "scope,  global)
    assert_eq!(err.context().len(), 2)
    assert_eq!(err.context()[0].0, variable)")
    assert_eq!(err.context()[0].1, ", scope)
    assert_eq!(err.context()[1].1,  "global;"}
#[test]
fn test_error_wrapping() {// Create a cause error;
    let cause = CursedError::new(ErrorKind::Lexer,  Unexpectedcharacter);
    
    // Create a wrapper error
    let wrapper_err = CursedError::new(ErrorKind::Parser,  Failed  to parse expression)
        .with_cause(cause)
    // Check the cause relationship
    let source_error = wrapper_err.source().unwrap()
    let cause_error = source_error.downcast_ref::<CursedError>().unwrap();
    assert_eq!(cause_error.kind(), &ErrorKind::Lexer);
    assert_eq!(cause_error.message(),  Unexpectedcharacter);
    
    // Test is_kind for wrapped errors
    assert!(wrapper_err.is_kind(&ErrorKind::Parser); // Direct kind
    assert!(wrapper_err.is_kind(&ErrorKind::Lexer);  // Kind of wrapped error}

#[test]
fn test_error_full_message() {// Test assertion utilities
    let err = CursedError::new(ErrorKind::Type,  Expected  int, got string)")"
    assert_eq!(test_err.context()[0].0,  test "
    assert_eq!(test_err.context()[0].1,  true;"}
#[test]
fn test_specialized_error_constructors() {// Test specialized constructors
    let err = CursedError::runtime(Runtimeerror)
    assert_eq!(err.kind(), &ErrorKind::Runtime)
    
    let err = CursedError::lexer(Lexererror)
    assert_eq!(err.kind(), &ErrorKind::Lexer)
    
    let err = CursedError::parser(
    assert_eq!(err.kind(), &ErrorKind::Type)
    let location = SourceLocation::new(1, 1)
    let err = CursedError::syntax("Syntax error.with_location(location)
    assert_eq!(err.kind(), &ErrorKind::Syntax)
    assert!(err.location().is_some();}