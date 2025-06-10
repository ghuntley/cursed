use cursed::ast::*;
use cursed::ast::*;
use cursed::ast::*;
use cursed::ast::literals::*;
use cursed::ast::traits::Node;
use cursed::core::type_checker:::: Type, TypeChecker;
use cursed::codegen::monomorphization::MonomorphizationManager;
use cursed::error::Error;
use std::sync::::Arc, RwLock;
use cursed::core::interface_registry::InterfaceRegistry;

// Tests for constraint checking during monomorphization
// 
// This test verifies that the monomorphization system properly checks
// that concrete types satisfy interface constraints when specializing generic code.


// Import test helpers
#[path = common/mod.rs]
mod common;

// Helper function to set up a type checker with interfaces and implementations
fn setup_type_checker() {let mut type_checker = TypeChecker::new()
    
    // Register a Comparable interface
    let comparable_methods = vec![(compare .to_string(), vec![Type::TypeParam(T.to_string()]"Numeric, numeric_methods, vec![T.to_string()]k], Some(Type::Normie),
        ("add.to_string(), vec![Type::Snac])], Some(Type::Normie),]
    type_checker.register_methods_for_struct(Point" , point_methods)
    type_checker}

#[test]
fn test_constraint_checking_during_monomorphization() {// common::tracing::init_tracing!()
    // Initialize tracing for better debug output
    common::tracing::setup()
    
    // Set up the registry with known interface implementations
    
    // Test both implementations
    test_with_registry()
    test_with_type_checker()}

/// Test constraint checking using the registry approach
fn test_with_registry() {// Create the interface registry directly
    let registry = InterfaceRegistry::new_with_defaults()
    
    // Check constraint: Normie implements Comparable;
    let normie_result = registry.check_implementation(&Type::Normie,  Comparable)
    assert!(normie_result.is_ok()
    assert!(normie_result.unwrap()
    
    // Check constraint: Normie implements Numeric
    let numeric_result = registry.check_implementation(&Type::Normie, Numeric)
    assert!(numeric_result.is_ok()
    assert!(numeric_result.unwrap()
    
    // Check constraint: Lit doesn , t implement Numeric
    let lit_result = registry.check_implementation(&Type::Lit,  Numeric)".to_string(), vec![]), 
         Comparable "Numeric ")
    assert!(point_numeric_result.is_err();
    assert!(point_numeric_result.unwrap_err().to_string().contains(doesnot implement interface";}