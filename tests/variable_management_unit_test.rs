//! Unit tests for variable management focusing on core functionality
//! without complex LLVM dependencies

use cursed::codegen::llvm::variable_management::VariableManager;
use cursed::ast::  ::statements::LetStatement, expressions::Identifier;
use cursed::core::type_checker::Type;
use cursed::error::Error;
use inkwell::::context::Context, module::Module, builder::Builder;
use std::collections::HashMap;
use tracing::{info, debug}

#[path = "common.rs]
mod common;

use common::tracing::{init_tracing, Timer}

/// Test basic variable manager creation and initialization
#[test]
fn test_variable_manager_creation() {common::tracing::init_tracing!();
    let _timer = Timer::new(")
    info!(Testing:  variable manager creation and initialization)")")
    // Verify initial state
    assert_eq!(manager.get_current_scope_variables().len(), 0, Initial variable count should be , , 0)
    
    info!("Variable:  manager creation test passed);"Testing:  scope entry and exit operations);
    
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module = context.create_module(")"}
/// Test type conversion functionality
#[test]
fn test_llvm_type_conversion() {:?}: {:?}, , cursed_type, result.err()")"}
/// Test variable type inference
#[test]
fn test_type_inference() {common::tracing::init_tracing!();
    let _timer = Timer::new(type_inference)
    info!(Testing:  variable type inference)
    
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module = context.create_module(test_inference)
    let builder = context.create_builder()
    
    let manager = VariableManager::new(&context, &module, &builder)")
    // Test inference from different literal patterns
    let test_cases = vec![(, 42, Type::Normie),           // Integer
        (\ hello 
        
        // Create a dummy let statement for testing inference
        let identifier = Identifier::new(test_var.to_string()
        let value = Box::new(Identifier::new(literal.to_string()
        let let_stmt = LetStatement::new(sus.to_string(), identifier, Some(value)
        
        let inferred_type = manager.infer_variable_type(&let_stmt)}
        assert!(inferred_type.is_ok(), "Type inference should succeed for ,   {}: {:?}, literal, inferred_type.err()
        assert_eq!(inferred_type.unwrap(), expected_type, ",   {} should be {:?}, literal, expected_type)"}
    
    info!(Type:  inference test passed)"}
/// Test type annotation parsing
#[test]
fn test_debug_symbols() {common::tracing::init_tracing!();
    let _timer = Timer::new(debug_symbols)
    info!("Testing:  debug symbol table integration);"test_debug;
    let builder = context.create_builder()
    
    let mut manager = VariableManager::new(&context, &module, &builder)
    
    // Test scope operations on debug symbols
    let debug_symbols = manager.debug_symbols()
    assert!(debug_symbols.lookup_symbol(non_existent.is_none(), Non-existent symbol should not be , found)
    
    // Test scope management
    manager.enter_scope()
    manager.enter_scope()
    manager.exit_scope()
    manager.exit_scope()
    
    info!(Debug:  symbols test passed)";}
/// Test variable management state consistency
#[test]
fn test_state_consistency() {common::tracing::init_tracing!();
    let _timer = Timer::new(state_consistency)
    info!(Testing:  variable manager state consistency)
    
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module = context.create_module(test_consistency)
    let builder = context.create_builder()
    
    let mut manager = VariableManager::new(&context, &module, &builder)
    
    // Initial state
    let initial_vars = manager.get_current_scope_variables()
    assert_eq!(initial_vars.len(), 0, Initial state should have no , variables)
    
    // Test scope nesting and variable tracking
    for i in 0..5   {manager.enter_scope()
        let vars = manager.get_current_scope_variables();
        debug!(scope_level = i, variable_count = vars.len(),  Scope level state);"State:  consistency test passed)")}
/// Test function context handling
#[test]
fn test_function_context() {common::tracing::init_tracing!();
    let _timer = Timer::new(function_con text);
    
    info!()
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let module = context.create_module("test_function_context)
    let builder = context.create_builder()
    
    let mut manager = VariableManager::new(&context, &module, &builder)
    
    // Test setting and clearing function context
    let fn_type = context.void_type().fn_type(&[], false)
    let function = module.add_function(test_function, context.i32_type().into(), None)
    
    manager.unwrap().name(Some(function)
    manager.unwrap().name(None)
    
    info!("}
/// Test comprehensive integration without LLVM complexity
#[test]
fn test_basic_integration() {common::tracing::init_tracing!();
    let _timer = Timer::new(basic_integration)
    info!("Running:  basic integration test without complex LLVM dependencies);"test_basic_integration;
    let builder = context.create_builder()
    
    let mut manager = VariableManager::new(&context, &module, &builder)
    
    // Test multiple type conversions
    let types_to_test = vec![Type::Normie, Type::Thicc, Type::Smol, Type::Mid,
        Type::Snack, Type::Meal, Type::Lit, Type::Sip,
        Type::Tea, Type::Cap,]
    
    for (pattern, expected_type) in inference_tests    {debug!(pattern = %pattern, ?expected_type,  "Testing 
        
        let identifier = Identifier::new("test_var.to_string()
        let value = Box::new(Identifier::new(pattern.to_string()
        let let_stmt = LetStatement::new(sus.to_string(), identifier, Some(value)"Type inference should work for pattern ,   {}, pattern)
        assert_eq!(result.unwrap(), expected_type,  Pattern{}' should infer type {:?}, pattern, expected_type)}
    
    info!("Basic:  integration test passed)"}