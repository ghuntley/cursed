//! Tests for generic constraint checking during monomorphization
//!
//! This test verifies that generic type parameters properly check interface constraints
//! using the improved monomorphization system.

use cursed::ast::traits::Node;
use cursed::core::type_checker::{Type, TypeChecker};
use cursed::codegen::monomorphization::MonomorphizationManager;
use cursed::error::Error;
use std::sync::{Arc, RwLock};

// Import test helpers
#[path = "common.rs"]
mod common;

#[test]
fn test_register_methods_for_struct() {
    // Initialize tracing for better debug output
    common::tracing::setup();
    
    // Create a type checker
    let mut type_checker = TypeChecker::new();
    
    // Register methods for a test struct
    let methods = vec![
        ("push".to_string(), vec![Type::Tea], None),
        ("pop".to_string(), vec![], Some(Type::Tea)),
        ("isEmpty".to_string(), vec![], Some(Type::Lit)),
    ];
    
    // Register the methods for the struct
    type_checker.register_methods_for_struct("TestStack", methods.clone());
    
    // Get the methods for the struct
    let retrieved_methods = type_checker.get_struct_methods("TestStack").unwrap();
    
    // Verify the methods were registered correctly
    assert_eq!(retrieved_methods.len(), methods.len());
    for (i, (name, params, ret)) in methods.iter().enumerate() {
        assert_eq!(retrieved_methods[i].0, *name);
        assert_eq!(retrieved_methods[i].1.len(), params.len());
        assert_eq!(retrieved_methods[i].2.is_some(), ret.is_some());
    }
}

#[test]
fn test_constraint_checking_with_type_checker() {
    // Initialize tracing for better debug output
    common::tracing::setup();
    
    // Create a type checker
    let mut type_checker = TypeChecker::new();
    
    // Register an interface with a required method
    let methods = vec![
        ("push".to_string(), vec![Type::Tea], None),
        ("pop".to_string(), vec![], Some(Type::Tea)),
    ];
    type_checker.register_interface("Stack", methods, vec![]);
    
    // Register a struct that implements the interface
    let struct_methods = vec![
        ("push".to_string(), vec![Type::Tea], None),
        ("pop".to_string(), vec![], Some(Type::Tea)),
        ("isEmpty".to_string(), vec![], Some(Type::Lit)),
    ];
    type_checker.register_methods_for_struct("StringStack", struct_methods);
    
    // Create a monomorphization manager with the type checker
    let type_checker_rc = Arc::new(RwLock::new(type_checker));
    let mono_manager = MonomorphizationManager::new().with_type_checker(type_checker_rc);
    
    // Check if the struct implements the interface
    let struct_type = Type::Struct("StringStack".to_string(), vec![]);
    let result = mono_manager.check_constraint(&struct_type, "Stack");
    
    // The struct should implement the interface
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
fn test_constraint_checking_missing_interface() {
    // Initialize tracing for better debug output
    common::tracing::setup();
    
    // Create a type checker
    let mut type_checker = TypeChecker::new();
    
    // Register an interface with a required method
    let methods = vec![
        ("add".to_string(), vec![Type::Normie], None),
        ("remove".to_string(), vec![Type::Normie], None),
    ];
    type_checker.register_interface("Collection", methods, vec![]);
    
    // Register a struct that does NOT implement the interface correctly
    let struct_methods = vec![
        ("add".to_string(), vec![Type::Normie], None),
        // Missing the 'remove' method
    ];
    type_checker.register_methods_for_struct("PartialCollection", struct_methods);
    
    // Create a monomorphization manager with the type checker
    let type_checker_rc = Arc::new(RwLock::new(type_checker));
    let mono_manager = MonomorphizationManager::new().with_type_checker(type_checker_rc);
    
    // Check if the struct implements the interface
    let struct_type = Type::Struct("PartialCollection".to_string(), vec![]);
    let result = mono_manager.check_constraint(&struct_type, "Collection");
    
    // The struct should NOT implement the interface
    assert!(result.is_err());
    assert!(result.err().unwrap().to_string().contains("does not implement interface"));
}

#[test]
fn test_enhanced_monomorphization_specialized_name() {
    // Initialize tracing for better debug output
    common::tracing::setup();
    
    // Create a monomorphization manager
    let mono_manager = MonomorphizationManager::new();
    
    // Test different type combinations
    let type_args1 = vec![Type::Normie, Type::Tea];
    let specialized_name1 = mono_manager.generate_specialized_name("test_function", &type_args1);
    assert_eq!(specialized_name1, "test_function__Normie_Tea");
    
    let type_args2 = vec![Type::Array(Box::new(Type::Normie), 5)];
    let specialized_name2 = mono_manager.generate_specialized_name("array_function", &type_args2);
    assert_eq!(specialized_name2, "array_function__Array_Normie_5_");
    
    let type_args3 = vec![Type::Slice(Box::new(Type::Tea))];
    let specialized_name3 = mono_manager.generate_specialized_name("slice_function", &type_args3);
    assert_eq!(specialized_name3, "slice_function__Slice_Tea");
}