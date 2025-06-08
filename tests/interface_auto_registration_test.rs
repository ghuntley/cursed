use cursed::core::type_checker::{Type, TypeChecker};
use cursed::core::type_checker_interface_registry::AutoInterfaceRegistration;
use cursed::error::Error;

// Test for automatic registration of interface implementations during type checking


#[path = "common/mod.rs"]
mod common;

#[test]
fn test_auto_interface_registration() {
    // Set up tracing for better debugging
    common::tracing::setup();
    
    // Create a type checker
    let mut type_checker = TypeChecker::new();
    
    // Register a struct type
    let mut struct_fields = std::collections::HashMap::new();
    struct_fields.insert("name".to_string(), Type::Tea);
    struct_fields.insert("age".to_string(), Type::Normie);
    type_checker.register_struct("Person", struct_fields, Vec::new());
    
    // Register methods for the struct
    let struct_methods = vec![
        ("getName".to_string(), Vec::new(), Some(Type::Tea)),
        ("getAge".to_string(), Vec::new(), Some(Type::Normie)),
    ];
    type_checker.struct_methods_map.insert("Person".to_string(), struct_methods);
    
    // Register an interface
    let interface_methods = vec![
        ("getName".to_string(), Vec::new(), Some(Type::Tea)),
    ];
    type_checker.interface_map.insert("Named".to_string(), interface_methods);
    
    // The Person type
    let person_type = Type::Struct("Person".to_string(), Vec::new());
    
    // The Named interface
    let named_interface = Type::Interface("Named".to_string(), Vec::new());
    
    // Check if Person implements Named
    let implements = type_checker.check_interface_implementation(&person_type, &named_interface).unwrap();
    assert!(implements, "Person should implement Named interface");
    
    // Access the registry to verify that Person was registered as implementing Named
    let registry = type_checker.interface_registry.clone();
    let registry = registry.lock().unwrap();
    
    assert!(registry.check_implementation(&person_type, "Named").unwrap());
}

#[test]
fn test_auto_interface_registration_with_generic_types() {
    // Set up tracing for better debugging
    common::tracing::setup();
    
    // Create a type checker
    let mut type_checker = TypeChecker::new();
    
    // Register a generic struct type: Container[T]
    let mut container_fields = std::collections::HashMap::new();
    container_fields.insert("items".to_string(), Type::Unknown); // Will be Slice[T] in reality
    container_fields.insert("count".to_string(), Type::Normie);
    type_checker.register_struct("Container", container_fields, vec!["T".to_string()]);
    
    // Register methods for the Container struct
    let container_methods = vec![
        (
            "add".to_string(), 
            vec![Type::TypeParam("T".to_string())], 
            None
        ),
        (
            "get".to_string(), 
            vec![Type::Normie], 
            Some(Type::TypeParam("T".to_string()))
        ),
        (
            "isEmpty".to_string(), 
            Vec::new(), 
            Some(Type::Lit)
        ),
    ];
    type_checker.struct_methods_map.insert("Container".to_string(), container_methods);
    
    // Register a Collection interface
    let collection_methods = vec![
        (
            "isEmpty".to_string(), 
            Vec::new(), 
            Some(Type::Lit)
        ),
    ];
    type_checker.interface_map.insert("Collection".to_string(), collection_methods);
    
    // The Container[T] type without concrete type arguments
    let container_type = Type::Struct("Container".to_string(), Vec::new());
    
    // The Collection interface
    let collection_interface = Type::Interface("Collection".to_string(), Vec::new());
    
    // Check if Container implements Collection
    let implements = type_checker.check_interface_implementation(&container_type, &collection_interface).unwrap();
    assert!(implements, "Container should implement Collection interface");
    
    // Use our auto-registration trait to explicitly check and register
    let implemented = type_checker.check_and_register_interface_implementation(
        &container_type, 
        &collection_interface
    ).unwrap();
    assert!(implemented, "Container should implement Collection with explicit registration");
    
    // Access the registry to verify that Container was registered as implementing Collection
    let registry = type_checker.interface_registry.clone();
    let registry = registry.lock().unwrap();
    
    // Check a specific instance of Container with a concrete type argument
    let container_tea = Type::Struct(
        "Container".to_string(), 
        vec![Box::new(Type::Tea)]
    );
    
    assert!(registry.check_implementation(&container_tea, "Collection").unwrap());
}

#[test]
fn test_auto_registration_during_program_type_checking() {
    // This would be a more complete test that verifies registration during
    // program type checking. For now, this is a placeholder.
    // In a full implementation, this would parse and type check a program
    // and verify that implementations are registered automatically.
    
    // Set up tracing for better debugging
    common::tracing::setup();
    
    tracing::info!("Auto-registration during program type checking test not yet implemented");
    // This is a placeholder for a future test
}