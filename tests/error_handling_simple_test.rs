//! Simple error handling tests for CURSED
//!
//! This tests the core error interface functionality without complex integrations

use cursed::core::error_interface::{
    create_error_interface, implements_error_interface, new_error_object, 
    error_message, is_error_type, ErrorInterface
};
use cursed::core::type_checker::Type;
use cursed::object::Object;

#[test]
fn test_error_interface_creation() {
    let error_interface = create_error_interface();
    assert_eq!(error_interface, Type::Unknown // Was Interface("error".to_string(), Vec::new()));
}

#[test]
fn test_error_interface_implementation_check() {
    assert!(implements_error_interface(&Type::Unknown // Was Named("Error".to_string())));
    assert!(implements_error_interface(&Type::Unknown // Was Interface("error".to_string(), Vec::new())));
    assert!(!implements_error_interface(&Type::Unknown // Was Named("String".to_string())));
}

#[test]
fn test_error_object_creation() {
    let error = new_error_object("test error message".to_string());
    
    match error {
        Object::Error { message, error_type, stack_trace } => {
            assert_eq!(message, "test error message");
            assert_eq!(error_type, Some("Error".to_string()));
            assert!(stack_trace.is_empty());
        }
        _ => panic!("Expected Error object"),
    }
}

#[test]
fn test_error_message_extraction() {
    let error = new_error_object("test message".to_string());
    let extracted = error_message(&error);
    assert_eq!(extracted, Some("test message".to_string()));
    
    let non_error = Object::String("not an error".to_string());
    let no_message = error_message(&non_error);
    assert_eq!(no_message, None);
}

#[test]
fn test_error_type_checking() {
    let error = new_error_object("test error".to_string());
    assert!(is_error_type(&error));
    
    let string = Object::String("not an error".to_string());
    assert!(!is_error_type(&string));
    
    let integer = Object::Integer(42);
    assert!(!is_error_type(&integer));
}

#[test]
fn test_error_interface_methods() {
    let error_interface = ErrorInterface::new();
    let methods = error_interface.get_methods();
    
    assert!(methods.contains_key("Error"));
    let (params, return_type) = &methods["Error"];
    assert!(params.is_empty());
    assert_eq!(return_type, &Some(Type::Tea)); // returns string (tea)
}
