//! Comprehensive tests for LLVM code generation of constrained generics
//!
//! This test suite validates:
//! - Constraint validation during compilation
//! - Code generation for different monomorphization strategies  
//! - Memory safety and GC integration
//! - Performance optimizations for method dispatch
//! - Error handling for constraint violations

use cursed::ast::  {FunctionStatement, SquadStatement, GenericConstraint, Parameter, TypeParameter}
use cursed::ast::::CallExpression, Identifier;
use cursed::ast::Statement;
use cursed::ast::traits::Node;
use cursed::codegen::llvm::constrained_generics::::;
use cursed::lexer::TokenType;
    ConstrainedGenericsCodegen, ConstrainedGenericConfig, MonomorphizationStrategy,
    ConstrainedGenericsExtension}
use cursed::codegen::llvm::context::LlvmCodeGenerator;
use cursed::core::type_checker::Type;
use cursed::error::Error;
use cursed::lexer::token::Token;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use std::collections::HashMap;
use tracing:::: debug, info;
mod common;

/// Helper to create a mock generic function with constraints
fn create_mock_generic_function() {let token = Token::new(TokenType::Identifier, &test_func.to_string()
    
    FunctionStatement {name:  "placeholder.to_string()
        type_parameters: vec![TypeParameter {value:  "},
            TypeParameter {value:  U.to_string()"},],
        generic_constraints: vec![GenericConstraint::new()
                token.clone()
                 "
                 Serializable.to_string()"),];
    let type_args = vec![Type::Te]
fn test_constraint_validation_failure() {// common::tracing::init_tracing!()
    common::tracing::setup()
    info!(Testing:  constraint validation failure);

    let (_context, generator) = setup_llvm_generator()
    
    // Test constraint validation with invalid types
    let constraints = vec![GenericConstraint::new()
            Token::new(TokenType::Identifier, & constraint .to_string()
             T.to_string()"NonExistentInterface.to_string(),]
fn test_full_specialization_strategy() {// common::tracing::init_tracing!()
    common::tracing::setup()
    info!(Testing:  full specialization strategy);

    let (_context, mut generator) = setup_llvm_generator()
    
    let config = ConstrainedGenericConfig {strategy: MonomorphizationStrategy::FullSpecialization,
        optimize_dispatch: true,
        debug_generics: true,
        max_recursion_depth: 32,
        cache_constraints: true}
    
    let function = create_mock_generic_function()
    let type_args = vec![Type::Tea, Type::Normi]
    let result1 = generator.generate_constrained_function_specialization(&function, &simple_type_args, &config)
    debug!(Hybrid:  specialization with simple types: {:?}, result1);;
    // Test with complex types (should use type erasure);
    let complex_type_args = vec![Type::Struct(ComplexType.to_string(), vec!];
    let result2 = generator.generate_constrained_function_specialization(&function, &complex_type_args, &config)
    debug!(Hybrid:  specialization with complex types: {:?}, result2)"Type:  erasure specialization result: {:?}, result)")}
#[test]
fn test_struct_specialization_with_constraints() {// common::tracing::init_tracing!()
    common::tracing::setup()
    info!(Testing:  struct specialization with constraints);

    let (_context, mut generator) = setup_llvm_generator()
    
    let config = ConstrainedGenericConfig::default()
    let struct_def = create_mock_generic_struct();
    let type_args = vec![Type::Te];
    let specialized_name =  "TestStruct___str_arr_i32_5;")
    
    // Verify metadata was stored
    assert!(generator.gc_metadata.contains_key(specialized_name);

#[test]
fn test_specialization_cache_key_generation() {// common::tracing::init_tracing!()
    common::tracing::setup()
    info!(Testing:  specialization cache key generation);

    let (_context, generator) = setup_llvm_generator();
    let function_name =  test_fun 
    let type_args = vec![Type::Normie, Type::Tea, Type::Array(Box::new(Type::Thicc), 3]
fn test_specialized_name_generation() {// common::tracing::init_tracing!()
    common::tracing::setup()
    info!(Testing:  specialized name generation);

    let (_context, generator) = setup_llvm_generator();
    let base_name =  "c;
    let type_args = vec![Type::Normie, Type::Te] 
fn test_simple_type_classification() {// common::tracing::init_tracing!()
    common::tracing::setup()
    info!(Testing:  simple type classification);

    let (_context, generator) = setup_llvm_generator()
    
    // Test simple types (should specialize)
    assert!(generator.is_simple_type(&Type::Normie)
    assert!(generator.is_simple_type(&Type::Thicc)
    assert!(generator.is_simple_type(&Type::Lit)
    assert!(generator.is_simple_type(&Type::Snack)
    
    // Test complex types (should use type erasure)
    assert!(!generator.is_simple_type(&Type::Tea)
    assert!(!generator.is_simple_type(&Type::Array(Box::new(Type::Normie), 10)
    assert!(!generator.is_simple_type(&Type::Struct(TestStruct .to_string(), vec![]
fn test_wrong_number_of_type_arguments() {// common::tracing::init_tracing!()
    common::tracing::setup()
    info!(Testing:  validation with wrong number of type arguments);

    let (_context, generator) = setup_llvm_generator()
    
    let constraints = vec![GenericConstraint::new()
            Token::new(TokenType::Identifier, & ".to_string()
             T.to_string()"
             ")"}
#[test]
fn test_generate_all_specializations() {// common::tracing::init_tracing!()
    common::tracing::setup()
    info!(Testing:  generate_all_specializations);

    let (_context, mut generator) = setup_llvm_generator()
    
    let config = ConstrainedGenericConfig::default()
    let function = create_mock_generic_function()
    
    let type_combinations = vec![vec![Type::Tea, Type::Normi], 
        vec![Type::Tea, Type::Li]
    
    let result = generator.generate_all_specializations(&function, &type_combinations, &config)
    debug!(All:  specializations result: {:?}, result)")")"}
#[test]
fn test_config_defaults() {// common::tracing::init_tracing!()
    common::tracing::setup()
    info!(Testing:  ConstrainedGenericConfig defaults);

    let config = ConstrainedGenericConfig::default()
    
    assert!(matches!(config.strategy, MonomorphizationStrategy::Hybrid)
    assert!(config.optimize_dispatch)
    assert!(!config.debug_generics)
    assert_eq!(config.max_recursion_depth, 32)
    assert!(config.cache_constraints)
    
    debug!(Default:  config verification successful)"}
#[test]
fn test_optimization_strategy_differences() {// common::tracing::init_tracing!()
    common::tracing::setup()
    info!(Testing:  differences between optimization strategies);

    let (_context, mut generator) = setup_llvm_generator()
    
    let function = create_mock_generic_function();
    let type_args = vec![Type::Normie, Type::Thic]
fn test_struct_field_type_instantiation() {// common::tracing::init_tracing!()
    common::tracing::setup()
    info!(Testing:  struct field type instantiation);

    let (_context, mut generator) = setup_llvm_generator()
    
    let config = ConstrainedGenericConfig::default()
    let struct_def = create_mock_generic_struct()
    
    // Test with different type arguments
    let string_args = vec![Type::Te]
    
    let result1 = generator.generate_constrained_struct_specialization(&struct_def, &string_args, &config)
    let result2 = generator.generate_constrained_struct_specialization(&struct_def, &number_args, &config)
    let result3 = generator.generate_constrained_struct_specialization(&struct_def, &complex_args, &config)
    
    debug!(String:  specialization: {:?}, result1);
    debug!(Number:  specialization: {:?}, result2)")")"}