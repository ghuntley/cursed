use cursed::ast::base::*;
use cursed::ast::declarations::*;
use cursed::ast::expressions::*;
use cursed::ast::literals::*;
use cursed::ast::traits::Node;
use cursed::core::type_checker::{Type, TypeChecker};
use cursed::codegen::monomorphization::MonomorphizationManager;
use cursed::error::Error;
use std::sync::{Arc, RwLock};
use cursed::core::interface_registry::InterfaceRegistry;

// Tests for constraint checking during monomorphization
// 
// This test verifies that the monomorphization system properly checks
// that concrete types satisfy interface constraints when specializing generic code.


// Import test helpers
#[path = "common/mod.rs"]
mod common;

// Helper function to set up a type checker with interfaces and implementations
fn setup_type_checker() -> TypeChecker {
    let mut type_checker = TypeChecker::new();
    
    // Register a Comparable interface
    let comparable_methods = vec![
        ("compare".to_string(), vec![Type::TypeParam("T".to_string())], Some(Type::Normie)),
    ];
    type_checker.register_interface("Comparable", comparable_methods, vec!["T".to_string()]);
    
    // Register a Numeric interface
    let numeric_methods = vec![
        ("add".to_string(), vec![Type::TypeParam("T".to_string())], Some(Type::TypeParam("T".to_string()))),
        ("subtract".to_string(), vec![Type::TypeParam("T".to_string())], Some(Type::TypeParam("T".to_string()))),
    ];
    type_checker.register_interface("Numeric", numeric_methods, vec!["T".to_string()]);
    
    // Register implementations for primitive types
    let int_methods = vec![
        ("compare".to_string(), vec![Type::Normie], Some(Type::Normie)),
        ("add".to_string(), vec![Type::Normie], Some(Type::Normie)),
        ("subtract".to_string(), vec![Type::Normie], Some(Type::Normie)),
    ];
    type_checker.register_methods_for_struct("Normie", int_methods);
    
    let float_methods = vec![
        ("compare".to_string(), vec![Type::Snack], Some(Type::Normie)),
        ("add".to_string(), vec![Type::Snack], Some(Type::Snack)),
        ("subtract".to_string(), vec![Type::Snack], Some(Type::Snack)),
    ];
    type_checker.register_methods_for_struct("Snack", float_methods);
    
    // Register implementations for custom types
    let point_methods = vec![
        ("compare".to_string(), vec![Type::Struct("Point".to_string(), vec![])], Some(Type::Normie)),
    ];
    type_checker.register_methods_for_struct("Point", point_methods);
    
    type_checker
}

#[test]
fn test_constraint_checking_during_monomorphization() {
    // Initialize tracing for better debug output
    common::tracing::setup();
    
    // Set up the registry with known interface implementations
    
    // Test both implementations
    test_with_registry();
    test_with_type_checker();
}

/// Test constraint checking using the registry approach
fn test_with_registry() {
    // Create the interface registry directly
    let registry = InterfaceRegistry::new_with_defaults();
    
    // Check constraint: Normie implements Comparable
    let normie_result = registry.check_implementation(&Type::Normie, "Comparable");
    assert!(normie_result.is_ok())
    assert!(normie_result.unwrap();
    
    // Check constraint: Normie implements Numeric
    let numeric_result = registry.check_implementation(&Type::Normie, "Numeric");
    assert!(numeric_result.is_ok())
    assert!(numeric_result.unwrap();
    
    // Check constraint: Lit doesn't implement Numeric
    let lit_result = registry.check_implementation(&Type::Lit, "Numeric");
    assert!(lit_result.is_ok())
    assert!(!lit_result.unwrap();
    
    // Check constraint: Custom struct implements an interface
    let point_result = registry.check_implementation(
        &Type::Struct("Point".to_string(), vec![]), 
        "Comparable"
    );
    assert!(point_result.is_ok())
    assert!(point_result.unwrap();
    
    // Check constraint: Custom struct doesn't implement non-registered interface
    let point_numeric_result = registry.check_implementation(
        &Type::Struct("Point".to_string(), vec![]), 
        "Numeric"
    );
    assert!(point_numeric_result.is_ok());
    assert!(!point_numeric_result.unwrap());
}

/// Test constraint checking using the monomorphization manager with type checker
fn test_with_type_checker() {
    // Set up the type checker with interfaces and implementations
    let type_checker = setup_type_checker();
    let type_checker_rc = Arc::new(RwLock::new(type_checker));
    
    // Create a monomorphization manager with the type checker
    let mono_manager = MonomorphizationManager::new().with_type_checker(type_checker_rc);
    
    // Check constraint: Normie implements Comparable
    let normie_result = mono_manager.check_constraint(&Type::Normie, "Comparable");
    assert!(normie_result.is_ok())
    
    // Check constraint: Normie implements Numeric
    let numeric_result = mono_manager.check_constraint(&Type::Normie, "Numeric");
    assert!(numeric_result.is_ok())
    
    // Check constraint: Lit doesn't implement Numeric
    let lit_result = mono_manager.check_constraint(&Type::Lit, "Numeric");
    assert!(lit_result.is_err())
    assert!(lit_result.unwrap_err().to_string().contains("does not implement interface"))
    
    // Check constraint: Custom struct implements an interface
    let point_result = mono_manager.check_constraint(
        &Type::Struct("Point".to_string(), vec![]), 
        "Comparable"
    );
    assert!(point_result.is_ok())
    
    // Check constraint: Custom struct doesn't implement non-registered interface
    let point_numeric_result = mono_manager.check_constraint(
        &Type::Struct("Point".to_string(), vec![]), 
        "Numeric"
    );
    assert!(point_numeric_result.is_err())
    assert!(point_numeric_result.unwrap_err().to_string().contains("does not implement interface"))
}