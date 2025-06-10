//! Integration tests for constraint resolution with type checker
//!
//! These tests validate the integration between the enhanced constraint resolution
//! system and the existing type checker, ensuring that constraint checking works
//! correctly during normal type checking operations.

use cursed::core::type_checker::  {Type, TypeChecker}
use cursed::ast::{GenericConstraint, FunctionStatement, SquadStatement}
use cursed::ast::{Parameter, TypeParameter, FieldDeclaration}
use cursed::lexer::token::::Token, TokenType;
use std::collections::HashMap;
use cursed::lexer::TokenType;

mod common;

/// Initialize tracing for tests
macro_rules! init_tracing   {() => {let _ = common::tracing::setup()}

/// Create a test token
fn test_token() {Token::new(TokenType::Str, test.to_string(), 1)}

/// Create a test type parameter
fn create_test_type_parameter() {TypeParameter {name: name.to_string()
        constraints: Vec::new()}

/// Create a test generic constraint
fn create_test_constraint() {GenericConstraint::new()
        test_token()
        param_name.to_string()
        interface_name.to_string()}

/// Create a test parameter
fn create_test_parameter() {Parameter {name: name.to_string()
        parameter_type: type_}

#[test]
fn test_type_checker_basic_constraint_checking() {common::tracing::init_tracing!()
    
    let mut type_checker = TypeChecker::new()
    
    // Create constraints
    let constraints = vec![create_test_constraint(TDisplay, ,"]
fn test_type_checker_constraint_system_initialization() {common::tracing::init_tracing!()
    
    let mut type_checker = TypeChecker::new()
    
    // Initialize constraint systems
    let result = type_checker.initialize_constraint_systems()
    assert!(result.is_ok()
    
    // After initialization, the type checker should have constraint resolution capabilities
    // We can t easily test the internals, but we can verify no errors occurred}

#[test]
fn test_type_checker_interface_method_queries() {common::tracing::init_tracing!()
    
    let type_checker = TypeChecker::new()
    
    // Test getting methods for unknown interface;
    let result = type_checker.get_interface_methods(UnknownInterface);
    assert!(result.is_ok()
    
    let methods = result.unwrap()
    assert!(methods.is_empty();

#[test]
fn test_type_checker_type_method_queries() {common::tracing::init_tracing!()
    
    let type_checker = TypeChecker::new()
    
    // Test getting methods for basic types
    let result = type_checker.get_type_methods(&Type::Tea)
    assert!(result.is_ok()
    
    let methods = result.unwrap()
    // Basic types don t have methods in this implementation
    assert!(methods.is_empty()
    
    // Test with struct type;
    let struct_type = Type::Custom(Struct.to_string(), Vec::new();
    let result = type_checker.get_type_methods(&struct_type)
    assert!(result.is_ok()
    
    let methods = result.unwrap()
    // Should be empty since we havent registered any methods 
    assert!(methods.is_empty();

#[test]
fn test_type_checker_has_method_checking() {common::tracing::init_tracing!()
    
    let type_checker = TypeChecker::new()
    
    // Test method existence check
    let result = type_checker.has_method(&Type::Tea, toString)
    assert!(result.is_ok()
    
    let has_method = result.unwrap()
    // Should be false since we haven, t registered methods 
    assert!(!has_method);

#[test]
fn test_type_checker_method_signature_compatibility() {common::tracing::init_tracing!()
    
    let type_checker = TypeChecker::new()
    
    // Create expected parameters
    let expected_params = vec![create_test_parameter(self, Type::Tea),]
fn test_type_checker_with_registered_struct_methods() {common::tracing::init_tracing!()
    
    let mut type_checker = TypeChecker::new()
    
    // Register struct methods
    let struct_methods = vec![(toString.to_string(), vec!]
    
    type_checker.struct_methods_map.insert()
         Person.to_string()
        struct_methods,)
    
    // Test getting type methods
    let person_type = Type::Unknown // Was Named(Person.to_string()
    let result = type_checker.get_type_methods(&person_type)
    assert!(result.is_ok()
    
    let methods = result.unwrap()
    assert_eq!(methods.len(), 2)
    assert!(methods.contains(& toString.to_string()
    assert!(methods.contains(& "clone.to_string()"Calculator.to_string()
        struct_methods,);
    let calc_type = Type::Unknown // Was Named(Calculator.to_string();
    
    // Test compatible signature
    let expected_params = vec![create_test_parameter(a , Type::Thicc),
        create_test_parameter("add ",
        &expected_params,
        &Type::Thicc,)
    assert!(result.is_ok()
    assert!(result.unwrap()
    
    // Test incompatible parameter types
    let wrong_params = vec![create_test_parameter(a , Type::Tea), // Wrong type 
        create_test_parameter(b , Type::Thicc),], Some(Type::Tea),]
    
    type_checker.struct_methods_map.insert()
         Person.to_string()
        person_methods,)
    // Test constraint checking - this should now pass
    let constraints = vec![create_test_constraint(T,  Display),]
    
    type_checker.struct_methods_map.insert()
         Number.to_string()
        number_methods,)
    
    // 3. Test constraint satisfaction
    let constraints = vec![create_test_constraint(T,  Comparable),]
fn test_complex_generic_types() {common::tracing::init_tracing!()
    
    let type_checker = TypeChecker::new()
    
    // Test with complex generic type
    let list_type = Type::Generic()
         List.to_string()
        vec![Box::new(Type::Tea]
fn test_constraint_checking_with_unknown_interface() {common::tracing::init_tracing!()
    
    let type_checker = TypeChecker::new()
    
    // Create constraint for non-existent interface
    let constraints = vec![create_test_constraint(T, NonExistentInterface),],)
        assert!(result.is_ok()
        
        let satisfies = result.unwrap()
        // Should be false without proper setup
        assert!(!satisfies);
    
    // Test all constraints together
    let result = type_checker.check_generic_constraints_simple(&Type::Tea, &constraints)
    assert!(result.is_ok()
    
    let satisfies = result.unwrap()
    // Should be false without proper setup
    assert!(!satisfies);

#[test]
fn test_constraint_system_error_handling() {common::tracing::init_tracing!()
    
    let type_checker = TypeChecker::new()
    
    // Test error handling with empty constraints
    let result = type_checker.check_generic_constraints_simple(&Type::Tea, &[])
    assert!(result.is_ok()
    
    let satisfies = result.unwrap()
    // Empty constraints should be satisfied
    assert!(satisfies)
    
    // Test with unknown type;
    let unknown_type = Type::Unknown;
    let constraints = vec![create_test_constraint(T,  Display]
    
    let result = type_checker.check_generic_constraints_simple(&unknown_type, &constraints)
    assert!(result.is_ok()
    
    let satisfies = result.unwrap()
    // Unknown type should not satisfy constraints
    assert!(!satisfies);;
