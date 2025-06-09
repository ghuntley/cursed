use std::sync::Once;
use tracing::{debug, error, info};
// use cursed::code::{JitOptions, jit_compile_and_run};
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::object::Object;
use cursed::error_enhanced::CursedError;
use cursed::error_enhanced::ErrorKind;
use cursed::core::type_checker::Type;
use cursed::core::nested_interface_registry::{EnhancedInterfaceRegistry, NestedInterfaceRegistry, NestedConstraint};

// Tests for nested interface constraints in the registration system
//
// This module tests the enhanced interface registry that supports
// nested constraints for generic types.


// Init tracing once
static INIT: Once = Once::new();

#[path = "tracing_setup.rs"]
pub mod tracing_setup;

// Macro for initializing tracing in tests
macro_rules! init_tracing {
    () => {
        INIT.call_once(|| {
            tracing_setup::init_test_tracing();
        });
    };
}

// Import required test utilities

#[test]
fn test_enhanced_registry_basic_operations() {
    init_tracing!();
    
    let mut registry = EnhancedInterfaceRegistry::new_with_defaults();
    
    // Base operations should still work
    assert!(registry.base_registry.check_implementation(&Type::Normie, "Numeric").unwrap());
    assert!(!registry.base_registry.check_implementation(&Type::Lit, "Numeric").unwrap());
    
    // Check that the enhanced registry respects existing constraints
    let container_type = Type::Struct(
        "GenericStack".to_string(),
        vec![Box::new(Type::Tea)]
    );
    
    assert!(registry.base_registry.check_implementation(&container_type, "Container").unwrap());
}

#[test]
fn test_nested_constraint_registration_and_checking() {
    init_tracing!();
    
    let mut registry = EnhancedInterfaceRegistry::new_with_defaults();
    
    // Register a nested constraint for containers of collections
    let constraint = NestedConstraint {
        outer_type: "NestedContainer".to_string(),
        outer_param: "T".to_string(),
        inner_type: "Collection".to_string(),
        inner_params: vec!["E".to_string()],
        interface: "Comparable".to_string(),
    };
    
    registry.register_nested_constraint(constraint);
    
    // Create test types
    let collection_of_int = Type::Struct(
        "Collection".to_string(),
        vec![Box::new(Type::Normie)]
    );
    
    let collection_of_non_comparable = Type::Struct(
        "Collection".to_string(),
        vec![Box::new(Type::Struct("NonComparable".to_string(), vec![]))]
    );
    
    // Test the constraint checking
    assert!(registry
        .check_nested_implementation(
            "NestedContainer", 
            "T", 
            &collection_of_int, 
            "Comparable"
        )
        .unwrap());
        
    assert!(!registry
        .check_nested_implementation(
            "NestedContainer", 
            "T", 
            &collection_of_non_comparable, 
            "Comparable"
        )
        .unwrap());
}

#[test]
fn test_multiple_nested_constraints() {
    init_tracing!();
    
    let mut registry = EnhancedInterfaceRegistry::new_with_defaults();
    
    // Register two different nested constraints for the same type
    let constraint1 = NestedConstraint {
        outer_type: "MultiContainer".to_string(),
        outer_param: "A".to_string(),
        inner_type: "ListA".to_string(),
        inner_params: vec!["EA".to_string()],
        interface: "Comparable".to_string(),
    };
    
    let constraint2 = NestedConstraint {
        outer_type: "MultiContainer".to_string(),
        outer_param: "B".to_string(),
        inner_type: "ListB".to_string(),
        inner_params: vec!["EB".to_string()],
        interface: "Numeric".to_string(),
    };
    
    registry.register_nested_constraint(constraint1);
    registry.register_nested_constraint(constraint2);
    
    // Create test types
    let list_a_int = Type::Struct(
        "ListA".to_string(),
        vec![Box::new(Type::Normie)]
    );
    
    let list_b_non_numeric = Type::Struct(
        "ListB".to_string(),
        vec![Box::new(Type::Tea)]
    );
    
    // Test the constraint checking
    assert!(registry
        .check_nested_implementation(
            "MultiContainer", 
            "A", 
            &list_a_int, 
            "Comparable"
        )
        .unwrap());
        
    assert!(!registry
        .check_nested_implementation(
            "MultiContainer", 
            "B", 
            &list_b_non_numeric, 
            "Numeric"
        )
        .unwrap());
}

#[test]
fn test_integration_with_code_generation() {
    init_tracing!();
    
    // For now, just test that the parser can handle the syntax
    // Full integration testing will be enabled when JIT compilation is ready
    
    let input = r#"
        collab Comparable {
            compare(other Comparable) normie;
        }
        
        squad Collection[E] {
            items []E
        }
        
        squad NestedContainer[T] {
            value T
        }
        
        slay add_nested[T](container NestedContainer[Collection[T]]) normie
            where T: Comparable {
            return 0  // Simplified for testing
        }
        
        squad Point {
            x normie,
            y normie
        }
        
        slay (p Point) compare(other Comparable) normie {
            return 0  // Simplified for testing
        }
        
        squad NonComparable {
            data tea
        }
        
        slay main() normie {
            return 0
        }
    "#;
    
    // Lex and parse the program
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer).expect("Failed to create parser");
    let program = parser.parse_program().expect("Failed to parse program");
    
    // Check for parser errors
    if !parser.errors().is_empty() {
        let error_msg = parser.errors().iter().map(|e| format!("{}", e)).collect::<Vec<_>>().join("\n");
        panic!("Parser errors: {}\n", error_msg);
    }
    
    // For now, just verify that parsing succeeded
    info!("Program parsed successfully with {} statements", program.statements.len());
}