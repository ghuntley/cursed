//! Tests for interface type assertion error propagation integration
//!
//! This file contains tests for the integration between the interface type assertion
//! error propagation mechanism and the Result type, ensuring proper ? operator support.

use cursed::error::Error;
use cursed::codegen::llvm::interface_type_assertion_error_propagation_integration::{is_type_mismatch_error, extract_type_info};
use cursed::error::type_assertion_error::TypeAssertionError;
use cursed::error_enhanced::CursedError;

/// Create a type assertion error for testing
fn create_test_error() -> Error {
    let error = TypeAssertionError::new("Reader", "Writer")
        .with_message("Types are incompatible");
    
    let cursed_error: CursedError = error.into();
    Error::TypeAssertion(cursed_error)
}

#[test]
fn test_is_type_mismatch_error() {
    let err = create_test_error();
    assert!(is_type_mismatch_error(&err));
    
    // Non-type assertion error should return false
    let other_err = Error::Compilation("This is not a type error".to_string());
    assert!(!is_type_mismatch_error(&other_err));
}

#[test]
fn test_extract_type_info() {
    let err = create_test_error();
    
    if let Some((source, target)) = extract_type_info(&err) {
        assert_eq!(source, "Reader");
        assert_eq!(target, "Writer");
    } else {
        panic!("Failed to extract type info");
    }
    
    // Other error types should return None
    let other_err = Error::Compilation("This is not a type error".to_string());
    assert!(extract_type_info(&other_err).is_none());
}

// Simulate the ? operator usage with various error types
#[test]
fn test_error_propagation_with_question_mark() {
    // Helper functions that use ? operator pattern
    fn returns_type_error() -> Result<(), Error> {
        Err(create_test_error())
    }
    
    fn calls_function_with_question_mark() -> Result<(), Error> {
        returns_type_error()?;
        Ok(())
    }
    
    // Test that errors propagate correctly
    let result = calls_function_with_question_mark();
    assert!(result.is_err());
    
    // Verify the error type is preserved
    if let Err(err) = result {
        assert!(is_type_mismatch_error(&err));
        if let Some((source, target)) = extract_type_info(&err) {
            assert_eq!(source, "Reader");
            assert_eq!(target, "Writer");
        } else {
            panic!("Error info was lost during propagation");
        }
    }
}

// Test conversion between error types
#[test]
fn test_error_conversion() {
    let assertion_error = TypeAssertionError::new("Stringer", "Reader")
        .with_message("Cannot convert string to reader")
        .with_interface_type_id(0x1234)
        .with_target_type_id(0x5678);
    
    // Convert to CursedError
    let cursed_error: CursedError = assertion_error.into();
    
    // Wrap in Error enum
    let error = Error::TypeAssertion(cursed_error);
    
    // Extract back
    if let Some((source, target)) = extract_type_info(&error) {
        assert_eq!(source, "Stringer");
        assert_eq!(target, "Reader");
    } else {
        panic!("Failed to extract type info after conversion");
    }
}