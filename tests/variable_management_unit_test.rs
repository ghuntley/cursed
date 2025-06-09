//! Unit tests for variable management focusing on core functionality
//! without complex LLVM dependencies

use cursed::codegen::llvm::variable_management::VariableManager;
use cursed::ast::{statements::LetStatement, expressions::Identifier};
use cursed::core::type_checker::Type;
use cursed::error::Error;
use inkwell::{context::Context, module::Module, builder::Builder};
use std::collections::HashMap;
use tracing::{info, debug};

#[path = "common.rs"]
mod common;

use common::tracing::{init_tracing, Timer};

/// Test basic variable manager creation and initialization
#[test]
fn test_variable_manager_creation() {
    init_tracing!();
    let _timer = Timer::new("variable_manager_creation");
    
    info!("Testing variable manager creation and initialization");
    
    let context = Context::create();
    let module = context.create_module("test_creation");
    let builder = context.create_builder();
    
    let manager = VariableManager::new(&context, &module, &builder);
    
    // Verify initial state
    assert_eq!(manager.get_current_scope_variables().len(), 0, "Initial variable count should be 0");
    
    info!("Variable manager creation test passed");
}

/// Test scope management without LLVM dependencies
#[test]
fn test_scope_operations() {
    init_tracing!();
    let _timer = Timer::new("scope_operations");
    
    info!("Testing scope entry and exit operations");
    
    let context = Context::create();
    let module = context.create_module("test_scopes");
    let builder = context.create_builder();
    
    let mut manager = VariableManager::new(&context, &module, &builder);
    
    // Test scope stack operations
    manager.enter_scope();
    manager.enter_scope();
    manager.enter_scope();
    
    // Test multiple exits
    manager.exit_scope();
    manager.exit_scope();
    manager.exit_scope();
    
    // Test that excessive exits don't cause panics
    manager.exit_scope();
    manager.exit_scope();
    
    info!("Scope operations test passed");
}

/// Test type conversion functionality
#[test]
fn test_llvm_type_conversion() {
    init_tracing!();
    let _timer = Timer::new("llvm_type_conversion");
    
    info!("Testing CURSED to LLVM type conversion");
    
    let context = Context::create();
    let module = context.create_module("test_types");
    let builder = context.create_builder();
    
    let manager = VariableManager::new(&context, &module, &builder);
    
    // Test that all basic types can be converted
    let basic_types = vec![
        Type::Smol,   // i8
        Type::Mid,    // i16
        Type::Normie, // i32
        Type::Thicc,  // i64
        Type::Snack,  // f32
        Type::Meal,   // f64
        Type::Lit,    // bool
        Type::Sip,    // char
        Type::Tea,    // string/pointer
        Type::Cap,    // null/void pointer
    ];
    
    for cursed_type in basic_types {
        debug!(?cursed_type, "Testing type conversion");
        
        let result = manager.get_llvm_type(&cursed_type);
        assert!(result.is_ok(), "Type conversion should succeed for {:?}: {:?}", cursed_type, result.err());
    }
    
    info!("LLVM type conversion test passed");
}

/// Test variable type inference
#[test]
fn test_type_inference() {
    init_tracing!();
    let _timer = Timer::new("type_inference");
    
    info!("Testing variable type inference");
    
    let context = Context::create();
    let module = context.create_module("test_inference");
    let builder = context.create_builder();
    
    let manager = VariableManager::new(&context, &module, &builder);
    
    // Test inference from different literal patterns
    let test_cases = vec![
        ("42", Type::Normie),           // Integer
        ("\"hello\"", Type::Tea),       // String
        ("based", Type::Lit),           // Boolean true
        ("sus", Type::Lit),             // Boolean false
        ("3.14", Type::Meal),           // Float
        ("'a'", Type::Sip),             // Character
    ];
    
    for (literal, expected_type) in test_cases {
        debug!(literal = %literal, ?expected_type, "Testing type inference");
        
        // Create a dummy let statement for testing inference
        let identifier = Identifier::new("test_var".to_string());
        let value = Box::new(Identifier::new(literal.to_string()));
        let let_stmt = LetStatement::new("sus".to_string(), identifier, Some(value));
        
        let inferred_type = manager.infer_variable_type(&let_stmt);
        assert!(inferred_type.is_ok(), "Type inference should succeed for '{}': {:?}", literal, inferred_type.err());
        assert_eq!(inferred_type.unwrap(), expected_type, "Type inference for '{}' should be {:?}", literal, expected_type);
    }
    
    info!("Type inference test passed");
}

/// Test type annotation parsing
#[test]
fn test_type_annotations() {
    init_tracing!();
    let _timer = Timer::new("type_annotations");
    
    info!("Testing explicit type annotations");
    
    let context = Context::create();
    let module = context.create_module("test_annotations");
    let builder = context.create_builder();
    
    let manager = VariableManager::new(&context, &module, &builder);
    
    // Test all supported type annotations
    let type_annotations = vec![
        ("normie", Type::Normie),
        ("thicc", Type::Thicc),
        ("smol", Type::Smol),
        ("mid", Type::Mid),
        ("snack", Type::Snack),
        ("meal", Type::Meal),
        ("lit", Type::Lit),
        ("sip", Type::Sip),
        ("tea", Type::Tea),
        ("cap", Type::Cap),
    ];
    
    for (annotation, expected_type) in type_annotations {
        debug!(annotation = %annotation, ?expected_type, "Testing type annotation");
        
        let type_expr = Identifier::new(annotation.to_string());
        let result = manager.type_from_annotation(&type_expr);
        
        assert!(result.is_ok(), "Type annotation '{}' should be valid: {:?}", annotation, result.err());
        assert_eq!(result.unwrap(), expected_type, "Type annotation '{}' should resolve to {:?}", annotation, expected_type);
    }
    
    // Test invalid type annotation
    let invalid_type = Identifier::new("invalid_type".to_string());
    let result = manager.type_from_annotation(&invalid_type);
    assert!(result.is_err(), "Invalid type annotation should fail");
    
    info!("Type annotations test passed");
}

/// Test error handling in variable operations
#[test]
fn test_error_handling() {
    init_tracing!();
    let _timer = Timer::new("error_handling");
    
    info!("Testing error handling in variable operations");
    
    let context = Context::create();
    let module = context.create_module("test_errors");
    let builder = context.create_builder();
    
    let manager = VariableManager::new(&context, &module, &builder);
    
    // Test getting non-existent variable
    let result = manager.get_variable("non_existent");
    assert!(result.is_none(), "Non-existent variable should return None");
    
    // Test getting type of non-existent variable
    let result = manager.get_variable_type("non_existent");
    assert!(result.is_none(), "Non-existent variable type should return None");
    
    // Test invalid type expression
    let identifier = Identifier::new("test_var".to_string());
    let value = Box::new(Identifier::new("unknown_literal_pattern".to_string()));
    let let_stmt = LetStatement::new("sus".to_string(), identifier, Some(value));
    
    let result = manager.infer_variable_type(&let_stmt);
    // This should default to normie, so it should succeed
    assert!(result.is_ok(), "Unknown literal should default to normie type");
    assert_eq!(result.unwrap(), Type::Normie, "Unknown literal should default to normie");
    
    info!("Error handling test passed");
}

/// Test debug symbol integration
#[test]
fn test_debug_symbols() {
    init_tracing!();
    let _timer = Timer::new("debug_symbols");
    
    info!("Testing debug symbol table integration");
    
    let context = Context::create();
    let module = context.create_module("test_debug");
    let builder = context.create_builder();
    
    let mut manager = VariableManager::new(&context, &module, &builder);
    
    // Test scope operations on debug symbols
    let debug_symbols = manager.debug_symbols();
    assert!(debug_symbols.lookup_symbol("non_existent").is_none(), "Non-existent symbol should not be found");
    
    // Test scope management
    manager.enter_scope();
    manager.enter_scope();
    manager.exit_scope();
    manager.exit_scope();
    
    info!("Debug symbols test passed");
}

/// Test variable management state consistency
#[test]
fn test_state_consistency() {
    init_tracing!();
    let _timer = Timer::new("state_consistency");
    
    info!("Testing variable manager state consistency");
    
    let context = Context::create();
    let module = context.create_module("test_consistency");
    let builder = context.create_builder();
    
    let mut manager = VariableManager::new(&context, &module, &builder);
    
    // Initial state
    let initial_vars = manager.get_current_scope_variables();
    assert_eq!(initial_vars.len(), 0, "Initial state should have no variables");
    
    // Test scope nesting and variable tracking
    for i in 0..5 {
        manager.enter_scope();
        let vars = manager.get_current_scope_variables();
        debug!(scope_level = i, variable_count = vars.len(), "Scope level state");
    }
    
    // Exit all scopes
    for i in (0..5).rev() {
        manager.exit_scope();
        let vars = manager.get_current_scope_variables();
        debug!(scope_level = i, variable_count = vars.len(), "Scope exit state");
    }
    
    // Final state should match initial
    let final_vars = manager.get_current_scope_variables();
    assert_eq!(final_vars.len(), initial_vars.len(), "Final state should match initial state");
    
    info!("State consistency test passed");
}

/// Test function context handling
#[test]
fn test_function_context() {
    init_tracing!();
    let _timer = Timer::new("function_context");
    
    info!("Testing function context management");
    
    let context = Context::create();
    let module = context.create_module("test_function_context");
    let builder = context.create_builder();
    
    let mut manager = VariableManager::new(&context, &module, &builder);
    
    // Test setting and clearing function context
    let fn_type = context.void_type().fn_type(&[], false);
    let function = module.add_function("test_function", fn_type, None);
    
    manager.set_current_function(Some(function));
    manager.set_current_function(None);
    
    info!("Function context test passed");
}

/// Test comprehensive integration without LLVM complexity
#[test]
fn test_basic_integration() {
    init_tracing!();
    let _timer = Timer::new("basic_integration");
    
    info!("Running basic integration test without complex LLVM dependencies");
    
    let context = Context::create();
    let module = context.create_module("test_basic_integration");
    let builder = context.create_builder();
    
    let mut manager = VariableManager::new(&context, &module, &builder);
    
    // Test multiple type conversions
    let types_to_test = vec![
        Type::Normie, Type::Thicc, Type::Smol, Type::Mid,
        Type::Snack, Type::Meal, Type::Lit, Type::Sip,
        Type::Tea, Type::Cap,
    ];
    
    for test_type in types_to_test {
        debug!(?test_type, "Testing type in integration");
        
        let result = manager.get_llvm_type(&test_type);
        assert!(result.is_ok(), "Type conversion should work for {:?}", test_type);
    }
    
    // Test scope operations
    for scope_depth in 0..3 {
        debug!(scope_depth, "Entering scope in integration test");
        manager.enter_scope();
    }
    
    for scope_depth in (0..3).rev() {
        debug!(scope_depth, "Exiting scope in integration test");
        manager.exit_scope();
    }
    
    // Test type inference for different patterns
    let inference_tests = vec![
        ("10", Type::Normie),
        ("\"text\"", Type::Tea),
        ("based", Type::Lit),
        ("3.14159", Type::Meal),
        ("'c'", Type::Sip),
    ];
    
    for (pattern, expected_type) in inference_tests {
        debug!(pattern = %pattern, ?expected_type, "Testing inference in integration");
        
        let identifier = Identifier::new("test_var".to_string());
        let value = Box::new(Identifier::new(pattern.to_string()));
        let let_stmt = LetStatement::new("sus".to_string(), identifier, Some(value));
        
        let result = manager.infer_variable_type(&let_stmt);
        assert!(result.is_ok(), "Type inference should work for pattern '{}'", pattern);
        assert_eq!(result.unwrap(), expected_type, "Pattern '{}' should infer type {:?}", pattern, expected_type);
    }
    
    info!("Basic integration test passed");
}
