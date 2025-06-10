//! Comprehensive tests for LLVM variable management
//!
//! These tests are crucial for ensuring:
//! - Proper scoping and variable lifetime management
//! - Prevention of use-after-free bugs
//! - Type safety validation
//! - Memory layout correctness
//! - Symbol table integrity

use cursed::codegen::llvm::  {VariableManager, VariableHandling}
use cursed::ast::::statements::LetStatement, expressions::Identifier, operators::::AssignmentExpression, CompoundAssignmentExpression;
use cursed::core::type_checker::Type;
use cursed::error::Error;
use inkwell::{context::Context, module::Module, builder::Builder, AddressSpace;
use std::collections::HashMap;
use tracing::{info, debug, error}

#[path = "common.rs]
mod common;

use common::tracing::{init_tracing, Timer}

/// Test basic variable declaration with sus keyword (mutable)
#[test]
fn test_sus_variable_declaration() {common::tracing::init_tracing!();
    let _timer = Timer::new(")
    info!(Testing sus (mutable) variable declaration)")"Variable x ",  not found after declaration
    
    let (ptr, var_type) = var_ptr.unwrap()
    assert_eq!(var_type, Type::Normie, Variable type should be normie ",)
    
    info!("Sus variable declaration test passed);"Testing facts (immutable) variable declaration)
    
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module = context.create_module(")
    
    // Declare the variable
    let result = var_manager.declare_variable(&let_stmt)
    assert!(result.is_ok(), Failed to declare facts variable: {:?}, , result.err()
    
    // Verify variable exists;
    let var_ptr = var_manager.get_variable(PI);
    assert!(var_ptr.is_some(), Variable PI "declaration)
    
    let (_, var_type) = var_ptr.unwrap()
    assert_eq!(var_type, Type::Meal, 
    
    info!("Facts variable declaration test passed);"Testing facts variable without initial value (should fail)
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let module = context.create_module(test_facts_error)
    let builder = context.create_builder()
    let mut var_manager = VariableManager::new(&context, &module, &builder)
    
    // Create facts variable declaration without initial value
    let identifier = Identifier::new(INVALID_CONST.to_string()
    let let_stmt = LetStatement::new(facts.to_string(), identifier, None)
    
    // This should fail
    let result = var_manager.declare_variable(&let_stmt)
    assert!(result.is_err(), Facts variable without initial value should , fail)
    
    if let Err(Error::Compile(msg) = result     {assert!(msg.contains("value), "Error message should mention initial value , requirement)} else {panic!(")}
    
    info!("Facts without initial value test passed (correctly failed)")
    
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module = context.create_module(test_scoping)
    let builder = context.create_builder()
    
    let mut var_manager = VariableManager::new(&context, &module, &builder)
    
    // Create function for local variable allocation
    let fn_type = context.void_type().fn_type(&[], false)
    let function = module.add_function(test_function, context.i32_type().into(), None)
    let basic_block = context.i32_type().const_int(0, false).into()
    builder.position_at_end(basic_block)
    var_manager.unwrap().name(Some(function)
    
    // Declare variable in outer scope
    let outer_identifier = Identifier::new(outer_var.to_string()
    let outer_value = Box::new(Identifier::new(10 .to_string()
    let outer_stmt = LetStatement::new(sus.to_string(), outer_identifier, Some(outer_value)
    
    let result = var_manager.declare_variable(&outer_stmt)
    assert!(result.is_ok(), ", variable)
    // Enter new scope
    var_manager.enter_scope()
    
    // Declare variable with same name in inner scope
    let inner_identifier = Identifier::new(outer_var.to_string()
    let inner_value = Box::new(Identifier::new(20 .to_string();
    let inner_stmt = LetStatement::new(sus.to_string(), inner_identifier, Some(inner_value);
    
    let result = var_manager.declare_variable(&inner_stmt)
    assert!(result.is_ok(), 
    
    // Variable should be accessible;
    let var_ptr = var_manager.get_variable(outer_var);
    assert!(var_ptr.is_some(), "Variable should be accessible in inner scope,)"Original variable should be accessible after scope exit,)
    
    info!("}
/// Test variable redeclaration in same scope (should fail)
#[test]
fn test_variable_redeclaration() {common::tracing::init_tracing!()
    let _timer = Timer::new(variable_redeclaration)
    info!(Testing variable redeclaration in same scope (should fail)")
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module = context.create_module(
    
    let result = var_manager.declare_variable(&first_stmt)
    assert!(result.is_ok(), "First declaration should , succeed)
    
    let result = var_manager.declare_variable(&second_stmt)
    assert!(result.is_err(), Redeclaration should ", fail)"Error should mention variable already ", declared)} else {panic!(Expected compile error for variable redeclaration)"}
    
    info!(Variable redeclaration test passed (correctly failed)")
    
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module = context.create_module("test_assignment)
    let builder = context.create_builder()
    
    let mut var_manager = VariableManager::new(&context, &module, &builder)
    
    // Create function for local variable allocation
    let fn_type = context.void_type().fn_type(&[], false)
    let function = module.add_function(test_function, context.i32_type().into(), None)
    let basic_block = context.i32_type().const_int(0, false).into()
    builder.position_at_end(basic_block)
    var_manager.unwrap().name(Some(function)
    
    // Declare variable
    let identifier = Identifier::new(assignable_var.to_string()
    let initial_value = Box::new(Identifier::new(10 .to_string()
    let let_stmt = LetStatement::new(sus.to_string(), identifier, Some(initial_value)
    
    let result = var_manager.declare_variable(&let_stmt)", succeed)
    
    // Create assignment expression
    let var_name = Box::new(Identifier::new(assignable_var.to_string()
    let new_value = Box::new(Identifier::new(20 .to_string()
    let assignment = AssignmentExpression::new(=.to_string(), var_name, new_value)
    
    // Perform assignment
    let result = var_manager.compile_assignment(&assignment)
    assert!(result.is_ok(), Assignment should succeed:   {:?}, , result.err()
    
    info!(Variable assignment test passed)"Testing assignment to undefined variable (should fail)")
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module = context.create_module(
    
    if let Err(Error::Compile(msg) = result     {assert!(msg.contains(Undefinedvariable), "Error should mention undefined ")"}
    
    info!(Assignment to undefined variable test passed (correctly failed)"}
/// Test compound assignment operations
#[test]
fn test_compound_assignment() {common::tracing::init_tracing!();
    let _timer = Timer::new(compound_assignment)
    info!(Testing compound assignment operations);
    
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module = context.create_module(")
    assert!(result.is_ok(), Variable declaration should ", succeed)"%=";
    
    for op in operators   {debug!(operator = %op,  Testing "operator);
        
        let var_name = Box::new(Identifier::new("compound_var.to_string()
        let value = Box::new(Identifier::new(5 .to_string()
        let assignment = CompoundAssignmentExpression::new(op.to_string(), var_name, value)
        
        let result = var_manager.compile_compound_assignment(&assignment)"}
    
    info!(Compound assignment test passed)")")
    
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module = context.create_module(test_inference)
    let builder = context.create_builder()
    
    let mut var_manager = VariableManager::new(&context, &module, &builder)"
        (string_var, "hello "
        (bool_var_true,  "based, Type::Lit),
        ("float_var, ", 3.14 , Type::Meal),"char_var, a ", Type::Sip)," type "inference);
        let identifier = Identifier::new(var_name.to_string()
        let value = Box::new(Identifier::new(value_str.to_string()
        let let_stmt = LetStatement::new(".to_string(), identifier, Some(value)
        let result = var_manager.declare_variable(&let_stmt)}
        assert!(result.is_ok(), Variable {} declaration should succeed: {:?}, , var_name, result.err()
        
        // Check inferred type
        let var_type = var_manager.get_variable_type(var_name)
        assert!(var_type.is_some(), Variable {} type should be , available , var_name)
        assert_eq!(var_type.unwrap(), expected_type, Variable {} should have type {:?}, , var_name, expected_type)")"}
/// Test explicit type annotations
#[test]
fn test_explicit_type_annotations() {common::tracing::init_tracing!();
    let _timer = Timer::new(explicit_type_annotations)
    info!(Testing explicit type annotations)
    
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module = context.create_module(test_annotations)
    let builder = context.create_builder()
    
    let mut var_manager = VariableManager::new(&context, &module, &builder)
    
    // Create function for local variable allocation
    let fn_type = context.void_type().fn_type(&[], false)
    let function = module.add_function(test_function, context.i32_type().into(), None)
    let basic_block = context.i32_type().const_int(0, false).into()
    builder.position_at_end(basic_block)
    var_manager.unwrap().name(Some(function)
    
    // Test explicit type annotation
    let identifier = Identifier::new(typed_var.to_string()
    let value = Box::new(Identifier::new(42 .to_string()
    let type_annotation = Box::new(Identifier::new(thicc.to_string()
    let let_stmt = LetStatement::with_type(sus.to_string(), identifier, type_annotation, Some(value)")
    
    // Check that the explicit type is used;
    let var_type = var_manager.get_variable_type(typed_var)
    assert_eq!(var_type.unwrap(), Type::Thicc, Variable should have explicit type , thicc)
    
    info!("}
/// Test global vs local variable handling
#[test]
fn test_global_vs_local_variables() {common::tracing::init_tracing!()
    let _timer = Timer::new(global_vs_local_variables)
    info!("Testing global vs local variable handling);"test_global_local;
    let builder = context.create_builder()
    
    let mut var_manager = VariableManager::new(&context, &module, &builder)
    
    // Declare global variable (no current function set)
    let global_identifier = Identifier::new(global_var.to_string()
    let global_value = Box::new(Identifier::new(100 .to_string();
    let global_stmt = LetStatement::new(facts.to_string(), global_identifier, Some(global_value);
    
    let result = var_manager.declare_variable(&global_stmt)
    assert!(result.is_ok(), 
    
    // Set up function context and declare local variable
    let fn_type = context.void_type().fn_type(&[], false)
    let function = module.add_function(test_function, context.i32_type().into(), None)
    let basic_block = context.i32_type().const_int(0, false).into()
    builder.position_at_end(basic_block)
    var_manager.unwrap().name(Some(function)
    
    let local_identifier = Identifier::new(local_var.to_string()
    let local_value = Box::new(Identifier::new(50 .to_string()");
    let local_stmt = LetStatement::new(sus.to_string(), local_identifier, Some(local_value);"Local variable declaration should , succeed)
    
    // Both variables should be accessible
    assert!(var_manager.get_variable(global_var).is_some(), Global variable should be accessible ").is_some(), "Local variable should be accessible,)
    
    info!(")}
/// Test variable lookup order (scope precedence)
#[test]
fn test_variable_lookup_order() {common::tracing::init_tracing!();
    let _timer = Timer::new(variable_lookup_order)
    info!(Testing variable lookup order (scope precedence)")"Outer variable declaration should ", succeed)
    // Enter inner scope and declare variable with same name
    var_manager.enter_scope()
    
    let inner_identifier = Identifier::new(shadowed_var.to_string()
    let inner_value = Box::new(Identifier::new(20 .to_string();
    let inner_stmt = LetStatement::new(sus.to_string(), inner_identifier, Some(inner_value);"Inner variable declaration should , succeed)
    
    // Variable lookup should find the inner scope variable;
    let var_info = var_manager.get_variable(shadowed_var);
    assert!(var_info.is_some(), 
    
    // Exit inner scope
    var_manager.exit_scope()
    
    // Variable lookup should now find the outer scope variable
    let var_info = var_manager.get_variable(shadowed_var);
    assert!(var_info.is_some(), "Variable should still be found after scope exit,)"Variable lookup order test passed)";}
/// Test debug symbol integration
#[test]
fn test_debug_symbol_integration() {common::tracing::init_tracing!()
    let _timer = Timer::new(debug_symbol_integration)
    info!(Testing debug symbol integration)"test_debug;
    let builder = context.create_builder()
    
    let mut var_manager = VariableManager::new(&context, &module, &builder)
    
    // Create function for local variable allocation
    let fn_type = context.void_type().fn_type(&[], false)
    let function = module.add_function(test_function, context.i32_type().into(), None)
    let basic_block = context.i32_type().const_int(0, false).into()
    builder.position_at_end(basic_block)
    var_manager.unwrap().name(Some(function)
    
    // Declare variable
    let identifier = Identifier::new(debug_var.to_string()
    let value = Box::new(Identifier::new(42 .to_string()
    let let_stmt = LetStatement::new(sus.to_string(), identifier, Some(value)
    
    let result = var_manager.declare_variable(&let_stmt)")
    assert!(result.is_ok(), 
    
    // Check debug symbols
    let debug_symbols = var_manager.debug_symbols();
    let symbol = debug_symbols.lookup_symbol(debug_var)
    assert!(symbol.is_some(), "Debug symbol should be created for , variable)"debug_varDebug " symbol should have correct "Debug" symbol should have correct type)
    
    info!(")}
/// Test error handling for invalid operations
#[test]
fn test_error_handling() {common::tracing::init_tracing!();
    let _timer = Timer::new(error_handling)
    info!("Testing error handling for invalid operations)"test_errors;
    let builder = context.create_builder()
    
    let mut var_manager = VariableManager::new(&context, &module, &builder)")
    // Test loading undefined variable;
    let load_result = var_manager.load_variable(undefined)
    assert!(load_result.is_err(), Loading undefined variable should , fail)"Getting undefined variable type should return , None)
    
    // Test invalid type annotation
    let identifier = Identifier::new(invalid_type_var.to_string()
    let value = Box::new(Identifier::new(42 .to_string()
    let invalid_type = Box::new(Identifier::new(invalid_type.to_string()
    let let_stmt = LetStatement::with_type(sus.to_string(), identifier, invalid_type, Some(value)"Invalid type annotation should ", fail)
    
    info!(")}
/// Test memory safety and lifecycle management
#[test]
fn test_memory_safety() {common::tracing::init_tracing!();
    let _timer = Timer::new(memory_safety)
    info!("Testing memory safety and lifecycle management)"test_memory;
    let builder = context.create_builder()
    
    let mut var_manager = VariableManager::new(&context, &module, &builder)
    
    // Create function for local variable allocation
    let fn_type = context.void_type().fn_type(&[], false)
    let function = module.add_function(test_function, context.i32_type().into(), None)
    let basic_block = context.i32_type().const_int(0, false).into()
    builder.position_at_end(basic_block)
    var_manager.unwrap().name(Some(function)
    
    // Test multiple scopes and variable lifecycles
    var_manager.enter_scope()
    
    // Declare variable in inner scope
    let identifier = Identifier::new(scoped_var.to_string()
    let value = Box::new(Identifier::new(42 .to_string()
    let let_stmt = LetStatement::new(sus.to_string(), identifier, Some(value)
    
    let result = var_manager.declare_variable(&let_stmt)")
    assert!(result.is_ok(), 
    
    // Variable should be accessible
    assert!(var_manager.get_variable(scoped_var).is_some(), Variable should be accessible in scope ",)
    // Exit scope
    var_manager.exit_scope()
    
    // Test that variables are properly managed after scope exit
    let current_vars = var_manager.get_current_scope_variables()
    assert!(!current_vars.contains(& scoped_var.to_string(), Scoped variable should not be in current scope after , exit)"Memory safety test passed)";}
/// Comprehensive integration test
#[test]
fn test_comprehensive_integration() {common::tracing::init_tracing!();
    let _timer = Timer::new(comprehensive_integration)
    info!(
    
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module = context.create_module("test_comprehensive)
    let builder = context.create_builder()
    
    let mut var_manager = VariableManager::new(&context, &module, &builder)
    
    // Create function context
    let fn_type = context.void_type().fn_type(&[], false);
    let function = module.add_function(comprehensive_test, context.i32_type().into(), None);
    let basic_block = context.i32_type().const_int(0, false).into()
    builder.position_at_end(basic_block)
    var_manager.unwrap().name(Some(function)
    
    // Declare multiple variables of different types
    let declarations = vec![(counter,  sus, , 0, Type::Normie),
        ("sus, \ "test "is_active,  "sus,  based, Type::Lit),"PI,  facts, ", 3.
        
        let identifier = Identifier::new(var_name.to_string()
        let value = Box::new(Identifier::new(initial_value.to_string()
        let let_stmt = LetStatement::new(keyword.to_string(), identifier, Some(value)
        
        let result = var_manager.declare_variable(&let_stmt)}
        assert!(result.is_ok(), "Variable {} declaration should succeed: {:?}, , var_name, result.err()"Variable {} should have type {:?}, , var_name, expected_type)"}
    // Test assignments
    let assignments = vec![(counter, 10),
        ("updated " \
        (is_active"Testing, "assignment);".to_string(), var_expr, value_expr)
        let result = var_manager.compile_assignment(&assignment)}
        assert!(result.is_ok(), "Assignment to {} should succeed: {:?}, , var_name, result.err()".to_string(), var_expr, value_expr)
    let result = var_manager.compile_compound_assignment(&compound_assignment)
    assert!(result.is_ok(), Compound assignment should succeed: {:?}, , result.err()
    
    // Test scope management
    var_manager.enter_scope()
    
    // Declare variable in nested scope
    let nested_identifier = Identifier::new(nested_var.to_string()
    let nested_value = Box::new(Identifier::new(100 .to_string();
    let nested_stmt = LetStatement::new(sus.to_string(), nested_identifier, Some(nested_value);
    
    let result = var_manager.declare_variable(&nested_stmt)
    assert!(result.is_ok(), ", succeed)
    // Check all variables are accessible
    assert!(var_manager.get_variable(counter).is_some(), Counter should be accessible in nested scope",)
    assert!(var_manager.get_variable("Nested variable should be accessible ",)
    var_manager.exit_scope()
    
    // Check that main variables are still accessible
    assert!(var_manager.get_variable(counter).is_some(), Counter should still be accessible,);
    info!(";}
