use cursed::core::interface_registry::InterfaceRegistry;
use cursed::core::type_checker::Type;
use cursed::codegen::monomorphization::MonomorphizationManager;
use cursed::error::Error;

// Tests for the interface registry
//
// This tests the new interface registry for constraint checking during monomorphization.


// Import test helpers
#[path = "common.rs"]
mod common;

#[test]
fn test_interface_registry_primitive_types() {
    // Initialize tracing for better debug output
    common::tracing::setup();
    
    // Create the interface registry
    let registry = InterfaceRegistry::new_with_defaults();
    
    // Check primitive types that should implement the Comparable interface
    assert!(registry.check_implementation(&Type::Normie, "Comparable").unwrap();
    assert!(registry.check_implementation(&Type::Thicc, "Comparable").unwrap();
    assert!(registry.check_implementation(&Type::Snack, "Comparable").unwrap();
    assert!(registry.check_implementation(&Type::Meal, "Comparable").unwrap();
    assert!(registry.check_implementation(&Type::Tea, "Comparable").unwrap();
    assert!(registry.check_implementation(&Type::Lit, "Comparable").unwrap();
    
    // Check primitive types that should implement the Numeric interface
    assert!(registry.check_implementation(&Type::Normie, "Numeric").unwrap();
    assert!(registry.check_implementation(&Type::Thicc, "Numeric").unwrap();
    assert!(registry.check_implementation(&Type::Snack, "Numeric").unwrap();
    assert!(registry.check_implementation(&Type::Meal, "Numeric").unwrap();
    
    // Check primitive types that should NOT implement certain interfaces
    assert!(!registry.check_implementation(&Type::Tea, "Numeric").unwrap();
    assert!(!registry.check_implementation(&Type::Lit, "Numeric").unwrap();
}

#[test]
fn test_interface_registry_custom_types() {
    // Initialize tracing for better debug output
    common::tracing::setup();
    
    // Create the interface registry
    let registry = InterfaceRegistry::new_with_defaults();
    
    // Check custom types that should implement specific interfaces
    let point_type = Type::Struct("Point".to_string(), vec![]);
    assert!(registry.check_implementation(&point_type, "Comparable").unwrap();
    
    // Check that Point doesn't implement Numeric
    assert!(!registry.check_implementation(&point_type, "Numeric").unwrap();
    
    // Check StringStack with Container and Stack interfaces
    let string_stack = Type::Struct("StringStack".to_string(), vec![]);
    assert!(registry.check_implementation(&string_stack, "Container").unwrap();
    assert!(registry.check_implementation(&string_stack, "Stack").unwrap();
    
    // Check IntList with Container, List, and Numeric interfaces
    let int_list = Type::Struct("IntList".to_string(), vec![]);
    assert!(registry.check_implementation(&int_list, "Container").unwrap();
    assert!(registry.check_implementation(&int_list, "List").unwrap();
    assert!(registry.check_implementation(&int_list, "Numeric").unwrap();
}

#[test]
fn test_interface_registry_adding_custom_implementations() {
    // Initialize tracing for better debug output
    common::tracing::setup();
    
    // Create an empty registry (without defaults)
    let mut registry = InterfaceRegistry::new();
    
    // Add a custom implementation
    let vector_type = Type::Struct("Vector2D".to_string(), vec![]);
    registry.register_implementation(vector_type.clone(), "Numeric".to_string());
    
    // Check that it works
    assert!(registry.check_implementation(&vector_type, "Numeric").unwrap();
    
    // Check that it doesn't implement other interfaces
    assert!(!registry.check_implementation(&vector_type, "Comparable").unwrap();
    
    // Add another implementation to the same type
    registry.register_implementation(vector_type.clone(), "Comparable".to_string());
    
    // Now it should implement both
    assert!(registry.check_implementation(&vector_type, "Numeric").unwrap();
    assert!(registry.check_implementation(&vector_type, "Comparable").unwrap();
}

#[test]
fn test_monomorphization_with_interface_registry() {
    // Initialize tracing for better debug output
    common::tracing::setup();
    
    // Create a monomorphization manager (without type checker)
    let mono_manager = MonomorphizationManager::new();
    
    // Check constraint for Normie implementing Comparable (should use registry)
    let normie_result = mono_manager.check_constraint(&Type::Normie, "Comparable");
    assert!(normie_result.is_ok())
    
    // Custom struct from registry
    let point_type = Type::Struct("Point".to_string(), vec![]);
    let point_result = mono_manager.check_constraint(&point_type, "Comparable");
    assert!(point_result.is_ok())
    
    // Type that doesn't implement interface should return Err
    let point_numeric_result = mono_manager.check_constraint(&point_type, "Numeric");
    assert!(point_numeric_result.is_err())
    assert!(point_numeric_result.unwrap_err().to_string().contains("does not implement interface"))
}