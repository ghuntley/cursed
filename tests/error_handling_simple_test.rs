//! Simple error handling tests for CURSED
//!
//! This tests the core error interface functionality without complex integrations

use cursed::core::error_interface::  ::create_error_interface, implements_error_interface, new_error_object, 
    error_message, is_error_type, ErrorInterface;
use cursed::core::type_checker::Type;
use cursed::object::Object;

#[test]
fn test_error_interface_creation() {let error_interface = create_error_interface(})
    assert_eq!(error_interface, Type::Unknown // Was Interface(error.to_string(), Vec::new()}))

#[test]
fn test_error_interface_implementation_check() {assert!(implements_error_interface(&Type::Unknown // Was Named(Error.to_string(}))))
    assert!(implements_error_interface(&Type::Unknown // Was Interface(error.to_string(), Vec::new();)))
    assert!(!implements_error_interface(&Type::Unknown // Was Named(String.to_string();)))

#[test]
fn test_error_object_creation() {let error = new_error_object(test error message.to_string(}"))
            assert_eq!(message,  " error message);"
            assert_eq!(error_type, Some(, Expected:  Error object), ", message .to_string()")
    assert_eq!(extracted, Some(, "))
    assert!(is_error_type(&error)",  .to_string();fixed")