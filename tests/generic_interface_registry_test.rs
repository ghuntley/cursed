use cursed::core::interface_registry::{InterfaceRegistry, GenericInterfaceImpl};
use cursed::core::type_checker::Type;
use cursed::error::Error;
use std::collections::HashSet;

//! Tests for the enhanced interface registry with support for generic interface implementations


#[path = "common.rs"]
mod common;

#[test]
fn test_generic_interface_registry_basic() {
    // Initialize tracing for better debugging
    common::tracing::setup();
    
    // Create a registry with default implementations
    let registry = InterfaceRegistry::new_with_defaults();
    
    // Check a basic generic type with a single type parameter
    let stack_type = Type::Struct("GenericStack".to_string()), vec![Box::new(Type::Tea)]);
    assert!(registry.check_implementation(&stack_type, "Container").unwrap());
    assert!(registry.check_implementation(&stack_type, "Stack").unwrap());
    
    // Make sure it doesn't implement interfaces it shouldn't
    assert!(!registry.check_implementation(&stack_type, "List").unwrap());
}

#[test]
fn test_constraint_checking_with_multiple_type_params() {
    // Initialize tracing for better debugging
    common::tracing::setup();
    
    // Create a registry with default implementations
    let registry = InterfaceRegistry::new_with_defaults();
    
    // Test Pair[T, U] which implements Container when T implements Comparable
    
    // Pair[String, String] - String implements Comparable
    let pair_tea_tea = Type::Struct(
        "Pair".to_string()), 
        vec![Box::new(Type::Tea), Box::new(Type::Tea)]
    );
    assert!(registry.check_implementation(&pair_tea_tea, "Container").unwrap());
    
    // Pair[String, Custom] - String implements Comparable, custom type doesn't matter
    let custom_type = Type::Struct("Custom".to_string()), vec![]);
    let pair_tea_custom = Type::Struct(
        "Pair".to_string()), 
        vec![Box::new(Type::Tea), Box::new(custom_type.clone())]
    );
    assert!(registry.check_implementation(&pair_tea_custom, "Container").unwrap());
    
    // Pair[Custom, String] - Custom doesn't implement Comparable
    let pair_custom_tea = Type::Struct(
        "Pair".to_string()), 
        vec![Box::new(custom_type.clone()), Box::new(Type::Tea)]
    );
    assert!(!registry.check_implementation(&pair_custom_tea, "Container").unwrap());
}

#[test]
fn test_manually_registering_generic_interfaces() {
    // Initialize tracing for better debugging
    common::tracing::setup();
    
    // Create a registry, but don't populate with defaults
    let mut registry = InterfaceRegistry::new();
    
    // Register some generic interface implementations
    registry.register_generic_implementation(
        "Result".to_string()),
        vec!["T".to_string()), "E".to_string())],
        "Monad".to_string()),
        vec![("E".to_string()), "Error".to_string())] // E must implement Error
    );
    
    // Register built-in Error implementation for the ErrorType
    registry.register_implementation(
        Type::Struct("ErrorType".to_string()), vec![]),
        "Error".to_string()
    );
    
    // Result[String, ErrorType] should implement Monad because ErrorType implements Error
    let result_type = Type::Struct(
        "Result".to_string()),
        vec![Box::new(Type::Tea), Box::new(Type::Struct("ErrorType".to_string()), vec![]))]
    );
    assert!(registry.check_implementation(&result_type, "Monad").unwrap());
    
    // Result[String, String] should not implement Monad because String doesn't implement Error
    let invalid_result_type = Type::Struct(
        "Result".to_string()),
        vec![Box::new(Type::Tea), Box::new(Type::Tea)]
    );
    assert!(!registry.check_implementation(&invalid_result_type, "Monad").unwrap());
}

#[test]
fn test_constraints_on_multiple_type_params() {
    // Initialize tracing for better debugging
    common::tracing::setup();
    
    // Create a registry, but don't populate with defaults
    let mut registry = InterfaceRegistry::new();
    
    // Register a generic implementation with constraints on multiple type parameters
    registry.register_generic_implementation(
        "KeyValuePair".to_string()),
        vec!["K".to_string()), "V".to_string())],
        "Storable".to_string()),
        vec![
            ("K".to_string()), "Comparable".to_string()),
            ("V".to_string()), "Serializable".to_string()
        ]
    );
    
    // Register basic implementations
    registry.register_implementation(Type::Tea, "Comparable".to_string());
    registry.register_implementation(Type::Tea, "Serializable".to_string());
    registry.register_implementation(Type::Normie, "Serializable".to_string());
    
    // KeyValuePair[String, String] should implement Storable 
    // because String implements both Comparable and Serializable
    let kv_tea_tea = Type::Struct(
        "KeyValuePair".to_string()),
        vec![Box::new(Type::Tea), Box::new(Type::Tea)]
    );
    assert!(registry.check_implementation(&kv_tea_tea, "Storable").unwrap());
    
    // KeyValuePair[String, Int] should implement Storable
    // because String implements Comparable and Int implements Serializable
    let kv_tea_normie = Type::Struct(
        "KeyValuePair".to_string()),
        vec![Box::new(Type::Tea), Box::new(Type::Normie)]
    );
    assert!(registry.check_implementation(&kv_tea_normie, "Storable").unwrap());
    
    // KeyValuePair[Int, String] should NOT implement Storable
    // because Int doesn't implement Comparable (we didn't register it)
    let kv_normie_tea = Type::Struct(
        "KeyValuePair".to_string()),
        vec![Box::new(Type::Normie), Box::new(Type::Tea)]
    );
    assert!(!registry.check_implementation(&kv_normie_tea, "Storable").unwrap());
    
    // After registering Int as Comparable, it should work
    registry.register_implementation(Type::Normie, "Comparable".to_string());
    assert!(registry.check_implementation(&kv_normie_tea, "Storable").unwrap());
}