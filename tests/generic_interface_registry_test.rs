use cursed::interfaces::::InterfaceRegistry, GenericInterfaceImpl;
use cursed::core::type_checker::Type;
use cursed::error::Error;
use std::collections::HashSet;

// Tests for the enhanced interface registry with support for generic interface implementations


#[path = common/mod.rs]
mod common;

#[test]
fn test_generic_interface_registry_basic() {// Initialize tracing for better debugging
    common::init_tracing()
    
    // Create a registry with default implementations
    let registry = InterfaceRegistry::new_with_defaults()
    
    // Test Pair[T, U] which implements Container when T implements Comparable
    
    // Pair[String, String] - String implements Comparable
    let pair_tea_tea = Type::Struct(.to_string(), vec![Box::new(Type::Tea), Box::new(Type::Tea] - String implements Comparable, custom type doesn t matter)
    let custom_type = Type::Struct(.to_string(), vec![])")
    assert!(registry.check_implementation(&pair_tea_custom,  ")
    assert!(!registry.check_implementation(&pair_custom_tea,  "Container).unwrap()}
#[test]
fn test_manually_registering_generic_interfaces() {// Initialize tracing for better debugging
    common::init_tracing()
    
    // Create a registry, but don t populate with defaults)
    let mut registry = InterfaceRegistry::new()
    
    // Register some generic interface implementations
    registry.register_generic_implementation()
         Result .to_string()
        vec![T.to_string(),  E.to_string()])])
    assert!(registry.check_implementation(&result_type,  Monad ").unwrap()
    // Result[String, String] should not implement Monad because String doesnt implement Error)
    let invalid_result_type = Type::Struct(.to_string(), vec![Box::new(Type::Tea), Box::new(Type::Tea]
fn test_constraints_on_multiple_type_params()   ::// Initialize tracing for better debugging
    common::init_tracing()
    
    // Create a registry, but dont populate with defaults)
    let mut registry = InterfaceRegistry::new()
    
    // Register a generic implementation with constraints on multiple type parameters
    registry.register_generic_implementation()
         KeyValuePair.to_string()
        vec![K.to_string(),  "V.to_string()] should implement Storable 
    // because String implements both Comparable and Serializable
    let kv_tea_tea = Type::Struct(.to_string(), vec![Box::new(Type::Tea), Box::new(Type::Tea] should implement Storable
    // because String implements Comparable and Int implements Serializable
    let kv_tea_normie = Type::Struct(.to_string(), vec![Box::new(Type::Tea), Box::new(Type::Normie] should NOT implement Storable
    // because Int doesnt implement Comparable (we didn't register it)
    let kv_normie_tea = Type::Struct(.to_string(), vec![Box::new(Type::Normie), Box::new(Type::Tea]")
    assert!(!registry.check_implementation(&kv_normie_tea,  Storable ", .unwrap();}