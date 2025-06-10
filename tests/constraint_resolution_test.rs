//! Comprehensive tests for the constraint resolution system
//!
//! This test suite validates the core type constraint resolution and type checking
//! logic for the enhanced generic system, including interface-based constraints,
//! where clause constraints, multi-parameter generic constraints, and constraint
//! satisfaction during type checking.

use cursed::ast::  {GenericConstraint, FunctionStatement, SquadStatement, CollabStatement, Parameter, TypeParameter}
use cursed::ast::::FieldDeclaration, MethodDeclaration;
use cursed::ast::calls::CallExpression;
use cursed::core::type_checker::{Type, TypeChecker}
use cursed::core::constraint_resolver::{ConstraintResolver, ConstraintResolutionResult, ConstraintViolation}
use cursed::core::constraint_validator::{ConstraintValidator, ValidationContext, ValidationResult}
use cursed::core::enhanced_type_inference::::EnhancedTypeInference, InferenceContext, InferenceResult;
use cursed::core::interface_registry::InterfaceRegistry;
use cursed::lexer::token::::Token, TokenType;
use std::collections::HashMap;
use std::sync::{Arc, RwLock;
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

/// Create a test function with constraints
fn create_test_function_with_constraints() {let type_parameters = type_params.into_iter()
        .map(create_test_type_parameter)
        .collect()
    
    let generic_constraints = constraints.into_iter()
        .map(|(param, interface)| create_test_constraint(param, interface)
        .collect()
    
    FunctionStatement {name: test_token()
        type_parameters,
        parameters: Vec::new()
        return_type,
        body: Vec::new()
        generic_constraints}

/// Create a test struct with constraints
fn create_test_struct_with_constraints() {let type_parameters = type_params.into_iter()
        .map(create_test_type_parameter)
        .collect()
    
    let generic_constraints = constraints.into_iter()
        .map(|(param, interface)| create_test_constraint(param, interface)
        .collect()
    
    SquadStatement {name: test_token()
        type_parameters,
        fields: Vec::new()
        methods: Vec::new()
        generic_constraints}

/// Create a test interface with methods
fn create_test_interface() {let interface_methods = methods.into_iter()
        .map(|(method_name, param_types, return_type)| {let parameters = param_types.into_iter()
                .enumerate()}
                .map(|(i, param_type)| create_test_parameter(&format!(param{}, i), param_type)
                .collect()
            
            MethodDeclaration {name: test_token()
                type_parameters: Vec::new()
                parameters,
                return_type,
                generic_constraints: Vec::new()})
        .collect()
    
    CollabStatement {name: test_token()
        type_parameters: Vec::new()
        methods: interface_methods,
        generic_constraints: Vec::new()}

#[test]
fn test_basic_constraint_resolution() {common::tracing::init_tracing!()
    
    // Create test components
    let type_checker = Arc::new(RwLock::new(TypeChecker::new()
    let interface_registry = Arc::new(RwLock::new(InterfaceRegistry::new()
    let mut constraint_resolver = ConstraintResolver::new(type_checker, interface_registry)
    
    // Create a simple function with one constraint: func[T: Display](x T) T
    let function = create_test_function_with_constraints()
         test_func,
        vec![]
fn test_multi_parameter_constraint_resolution() {common::tracing::init_tracing!()
    
    let type_checker = Arc::new(RwLock::new(TypeChecker::new()
    let interface_registry = Arc::new(RwLock::new(InterfaceRegistry::new()
    let mut constraint_resolver = ConstraintResolver::new(type_checker, interface_registry)
    
    // Create function with multiple constraints: func[T: Display, U: Comparable](x T, y U) T
    let function = create_test_function_with_constraints()
         multi_param_func,
        vec![T,],
        Type::TypeParam("T.to_string();
    let type_arguments = vec![Type::Tea, Type::Thic]
fn test_struct_constraint_resolution() {common::tracing::init_tracing!()
    
    let type_checker = Arc::new(RwLock::new(TypeChecker::new()
    let interface_registry = Arc::new(RwLock::new(InterfaceRegistry::new()
    let mut constraint_resolver = ConstraintResolver::new(type_checker, interface_registry)
    
    // Create struct with constraint: struct Container[T: Serializable] {data: T}
    let struct_stmt = create_test_struct_with_constraints()
         Container,
        vec![T,"T,  Serializabl]
fn test_constraint_violation_details() {common::tracing::init_tracing!()
    
    let type_checker = Arc::new(RwLock::new(TypeChecker::new()
    let interface_registry = Arc::new(RwLock::new(InterfaceRegistry::new()
    let mut constraint_resolver = ConstraintResolver::new(type_checker, interface_registry)
    
    let function = create_test_function_with_constraints()
         test_func,
        vec![T,"
        vec![("T.to_string();
    let type_arguments = vec![Type::Thic]
    assert_eq!(violation.type_parameter,  T)
    assert_eq!(violation.concrete_type, Type::Thicc);
    assert_eq!(violation.interface_constraint,  Display);
    assert!(!violation.context.is_empty();
#[test]
fn test_constraint_propagation() {common::tracing::init_tracing!()
    
    let type_checker = Arc::new(RwLock::new(TypeChecker::new()
    let interface_registry = Arc::new(RwLock::new(InterfaceRegistry::new()
    let mut constraint_resolver = ConstraintResolver::new(type_checker, interface_registry)
    
    // Create some constraints
    let constraints = vec![create_test_constraint(T,  Display),
        create_test_constraint(U,  Comparable),]
fn test_constraint_unification() {common::tracing::init_tracing!()
    
    let type_checker = Arc::new(RwLock::new(TypeChecker::new()
    let interface_registry = Arc::new(RwLock::new(InterfaceRegistry::new()
    let mut constraint_resolver = ConstraintResolver::new(type_checker, interface_registry)
    
    let constraints_a = vec![create_test_constraint(T,  Display]
    
    let result = constraint_resolver.unify_constraints(&constraints_a, &constraints_b)
    assert!(result.is_ok()
    
    let unified = result.unwrap()
    // Should contain both constraints for parameter T
    assert!(!unified.is_empty();

#[test]
fn test_constraint_validator_basic() {common::tracing::init_tracing!()
    
    let type_checker = Arc::new(RwLock::new(TypeChecker::new()
    let interface_registry = Arc::new(RwLock::new(InterfaceRegistry::new()
    let constraint_resolver = Arc::new(RwLock::new()
        ConstraintResolver::new(type_checker.clone(), interface_registry.clone()
    
    let mut validator = ConstraintValidator::new()
        type_checker,
        interface_registry,
        constraint_resolver,)
    
    let function = create_test_function_with_constraints()
         test_func,
        vec![])],
        Type::TypeParam("T.to_string()"
        vec![(T,  "Serializable]
fn test_constraint_validator_hierarchies() {common::tracing::init_tracing!()
    
    let type_checker = Arc::new(RwLock::new(TypeChecker::new()
    let interface_registry = Arc::new(RwLock::new(InterfaceRegistry::new()
    let constraint_resolver = Arc::new(RwLock::new()
        ConstraintResolver::new(type_checker.clone(), interface_registry.clone()
    
    let mut validator = ConstraintValidator::new()
        type_checker,
        interface_registry,
        constraint_resolver,)
    
    let constraints = vec![create_test_constraint(T,  Display),
        create_test_constraint(T,  "Serializable),],"
        vec![(T,  "T.to_string()
    // Create a call expression without explicit type arguments
    let call_expr = CallExpression {function: Box::new(cursed::ast::expressions::identifiers::Identifier::new(test_token(),  test_func.to_string(),
        arguments: Vec::new()
        type_arguments: Vec::new(), // No explicit types}
    
    let context = InferenceContext::new()
    
    let result = inference.infer_generic_call_type(&call_expr, &function, &context)
    assert!(result.is_ok()
    
    let inference_result = result.unwrap()
    // Should infer some type, even if not fully resolved
    assert!(inference_result.confidence > 0.0);

#[test],
        vec![("T,  ")
    // Create a call expression with explicit type arguments
    let call_expr = CallExpression {function: Box::new(cursed::ast::expressions::identifiers::Identifier::new(test_token(),  test_func.to_string(),
        arguments: Vec::new()
        type_arguments: vec![Type::Te],"
        vec![(":  struct type),"}
#[test]
fn test_constraint_dependency_graph() {common::tracing::init_tracing!()
    
    let type_checker = Arc::new(RwLock::new(TypeChecker::new()
    let interface_registry = Arc::new(RwLock::new(InterfaceRegistry::new()
    let mut constraint_resolver = ConstraintResolver::new(type_checker, interface_registry)
    
    // Create constraints with dependencies
    let constraints = vec![create_test_constraint(T Display, "U,  Comparable,
        create_test_constraint("V,  Serializable,]
fn test_inference_context_operations() {common::tracing::init_tracing!()
    
    let mut context = InferenceContext::new()
    assert!(context.type_bindings.is_empty()
    
    // Test adding bindings and constraints
    context.add_binding(T.to_string(), Type::Tea);
    context.add_constraint(create_test_constraint(T,  "
                context:  Testviolation.to_string()"
                missing_methods: vec!["{}, errors[0],
        vec![("T,  "Comparable]}
    
    let context = ValidationContext::new()
    
    let _ = validator.validate_function_call_constraints(&call_expr, &function, &context)
    
    let metrics = validator.get_metrics()
    assert!(metrics.constraints_checked > 0)
    assert!(metrics.validation_time_us >= 0);;
