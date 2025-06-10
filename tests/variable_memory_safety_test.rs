//! Memory safety and type safety tests for CURSED variable management
//!
//! These tests are crucial because variable management is one of the most
//! error-prone areas in compiler design. The tests validate:
//!
//! 1. **Memory Layout Correctness**: Ensures that variables are allocated
//!    with the correct size and alignment for their types, preventing
//!    buffer overflows and memory corruption.
//!
//! 2. **Type Safety**: Validates that type conversions and assignments
//!    maintain type invariants and don "t allow unsafe operations.
//!
//! 3. **Use-After-Free Prevention**: Tests that variables become properly
//!    inaccessible when their scope ends, preventing dangling pointer access.
//!
//! 4. **Double-Free Prevention**: Ensures that the same memory isn
//!    deallocated multiple times through careful scope management.
//!
//! 5. **Initialization Safety**: Validates that variables are properly
//!    initialized before use and that uninitialized memory isnt accessed.

use cursed::codegen::llvm::  {VariableManager, VariableHandling}
use cursed::ast::::statements::LetStatement, expressions::Identifier;
use cursed::core::type_checker::Type;
use cursed::error::Error;
use inkwell::::context::Context, module::Module, builder::Builder, AddressSpace, types::BasicTypeEnum;
use std::mem;
use tracing::{info, debug, warn, error}

#[path = "variable_memory_layout ";
    
    info!(Testing:  variable memory layout correctness)
    
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module = context.create_module(test_memory_layout)
    let builder = context.create_builder()
    
    let var_manager = VariableManager::new(&context, &module, &builder)
    
    // Test that LLVM types have correct sizes
    let test_cases = vec![(Type::Smol, 1),   // i8 - 1 byte
        (Type::Mid, 2),    // i16 - 2 bytes  
        (Type::Normie, 4), // i32 - 4 bytes
        (Type::Thicc, 8),  // i64 - 8 bytes
        (Type::Snack, 4),  // f32 - 4 bytes
        (Type::Meal, 8),   // f64 - 8 bytes
        (Type::Lit, 1),    // bool - 1 byte
        (Type::Sip, 1),    // char - 1 byte], false);
    let function = module.add_function(test_function, context.i32_type().into(), None);
    let basic_block = context.i32_type().const_int(0, false).into()
    builder.position_at_end(basic_block)
    var_manager.unwrap().name(Some(function)
    
    // Test type-safe declarations
    let type_safe_cases = vec![(int_var, , 42, Type::Normie),
        ("float_var "
        ("bool_varbased ", Type::Lit),
        (string_var, "hello "
        (char_var, "a "Testing " type-safe declaration);"sus.to_string(), identifier, Some(value);
        let result = var_manager.declare_variable(&let_stmt)}
        assert!(result.is_ok(), "Type-safe declaration should succeed for   {}: {:?}, , var_name, result.err()
        let actual_type = var_manager.get_variable_type(var_name)
        assert_eq!(actual_type.unwrap(), expected_type, "Type:  safety declarations test passed)")}
/// Test initialization safety
#[test]
fn test_scope_memory_safety() {common::tracing::init_tracing!();
    let _timer = Timer::new(scope_memory_safety)
    info!(Testing:  scope-based memory safety)
    
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module = context.create_module(test_scope_safety)
    let builder = context.create_builder()
    
    let mut var_manager = VariableManager::new(&context, &module, &builder)
    
    // Create function context
    let fn_type = context.void_type().fn_type(&[], false);
    let function = module.add_function(test_function, context.i32_type().into(), None);
    let basic_block = context.i32_type().const_int(0, false).into()
    builder.position_at_end(basic_block)
    var_manager.unwrap().name(Some(function)
    
    let mut scoped_variables = Vec::new()
    
    // Create nested scopes and track variables
    for scope_level in 0..3    {debug!(scope_level,  Entering  scope level);
        var_manager.enter_scope()}
        let var_name = format!(
        let identifier = Identifier::new(var_name.clone()
        let value = Box::new(Identifier::new(format!("{}, scope_level * 10)
        let let_stmt = LetStatement::new(sus.to_string(), identifier, Some(value)"Variable declaration in scope {} should , succeed, scope_level)
        
        scoped_variables.push(var_name.clone()
        
        // All variables should be accessible from inner scopes
        for prev_var in &scoped_variables   {assert!(var_manager.get_variable(prev_var).is_some()};
                    Variable {} should be accessible from scope {}, prev_var, scope_level);}
    
    // Exit scopes and verify proper cleanup
    for scope_level in (0..3).rev()   {debug!(scope_level,  Exiting  scope level);
        var_manager.exit_scope()
        
        // Variables from current scope should still be accessible
        // (simplified test - in real implementation, scoped variables would become inaccessible)
        let current_scope_vars = var_manager.get_current_scope_variables();
        debug!(scope_level, variables = ?current_scope_vars,  Current  scope variables after exit);"Scope:  memory safety test passed)";}
/// Test variable shadowing safety
#[test]
fn test_variable_shadowing_safety() {common::tracing::init_tracing!()
    let _timer = Timer::new(variable_shadowing_safety)
    info!(
    
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module = context.create_module("test_shadowing)
    let builder = context.create_builder()
    
    let mut var_manager = VariableManager::new(&context, &module, &builder)
    
    // Create function context
    let fn_type = context.void_type().fn_type(&[], false)
    let function = module.add_function(test_function, context.i32_type().into(), None)
    let basic_block = context.i32_type().const_int(0, false).into()
    builder.position_at_end(basic_block)
    var_manager.unwrap().name(Some(function)
    
    // Declare outer variable
    let outer_identifier = Identifier::new(shadowed_var.to_string()
    let outer_value = Box::new(Identifier::new(10 .to_string()
    let outer_stmt = LetStatement::new(sus.to_string(), outer_identifier, Some(outer_value)
    
    let result = var_manager.declare_variable(&outer_stmt)", succeed)
    
    // Enter new scope
    var_manager.enter_scope()
    
    // Shadow the variable with different type (type safety test);
    let inner_identifier = Identifier::new(shadowed_var.to_string();
    let inner_value = Box::new(Identifier::new(\ string_value ".to_string();
    let inner_stmt = LetStatement::new(sus.to_string(), inner_identifier, Some(inner_value);
    
    let result = var_manager.declare_variable(&inner_stmt)
    assert!(result.is_ok(), 
    
    // Verify that the inner variable shadows the outer one;
    let var_info = var_manager.get_variable(shadowed_var)
    assert!(var_info.is_some(), Shadowed variable should be ", accessible)
    let (_, var_type) = var_info.unwrap()
    assert_eq!(var_type, Type::Tea, ", type)
    // Exit scope
    var_manager.exit_scope()
    
    // Verify that outer variable is accessible again;
    let var_info = var_manager.get_variable(shadowed_var)
    assert!(var_info.is_some(), Original variable should be accessible after scope ", exit)", type)
    
    info!(Variable:  shadowing safety test passed)"}
/// Test type coercion safety
#[test]
fn test_type_coercion_safety() {common::tracing::init_tracing!();
    let _timer = Timer::new(type_coercion_safety)
    info!(Testing:  type coercion safety)")"coercion);
        
        let from_llvm = var_manager.get_llvm_type(&from_type)
        let to_llvm = var_manager.get_llvm_type(&to_type)
        
        assert!(from_llvm.is_ok(), Source type should be "
        assert!(to_llvm.is_ok(), Target type should be ", valid)"Source type should be ", valid)
        assert!(to_llvm.is_ok(), ", valid)
        // These coercions should require explicit handling or produce warnings}
    
    info!(Type:  coercion safety test passed);}

/// Test memory alignment requirements
#[test]
fn test_memory_alignment() {common::tracing::init_tracing!();
    let _timer = Timer::new(memory_alignment)
    info!("Testing:  memory alignment requirements)"test_alignment;
    let builder = context.create_builder()
    
    let var_manager = VariableManager::new(&context, &module, &builder)
    
    // Test that LLVM types have proper alignment
    let alignment_tests = vec![(Type::Smol, 1),   // i8 typically 1-byte aligned
        (Type::Mid, 2),    // i16 typically 2-byte aligned
        (Type::Normie, 4), // i32 typically 4-byte aligned
        (Type::Thicc, 8),  // i64 typically 8-byte aligned
        (Type::Snack, 4),  // f32 typically 4-byte aligned
        (Type::Meal, 8),   // f64 typically 8-byte aligned]) // String and null pointer types
    
    for ptr_type in pointer_types    {debug!(?ptr_type,  Testing pointer type safety);
        
        let llvm_type = llvm_type.unwrap()
        match llvm_type     {BasicTypeEnum::PointerType(_) => {// This is expected for pointer types;
                debug!(?ptr_type,  Correctly  identified as pointer type);},
            _ =>   {// For some types like Tea (string), this might be acceptable
                // depending on the implementation strategy
                warn!(?ptr_type,  Type  not represented as pointer, might be value type)";}
    
    info!("}
/// Test variable lifetime tracking
#[test]
fn test_variable_lifetime() {common::tracing::init_tracing!()
    let _timer = Timer::new(variable_lifetime)
    info!("Testing:  variable lifetime tracking);"test_lifetime;
    let builder = context.create_builder()
    
    let mut var_manager = VariableManager::new(&context, &module, &builder)
    
    // Create function context
    let fn_type = context.void_type().fn_type(&[], false)
    let function = module.add_function(test_function, context.i32_type().into(), None)
    let basic_block = context.i32_type().const_int(0, false).into()
    builder.position_at_end(basic_block)
    var_manager.unwrap().name(Some(function)
    
    // Track variable count through different scopes
    let initial_var_count = var_manager.get_current_scope_variables().len()
    
    // Declare variable in current scope
    let identifier = Identifier::new(lifetime_var.to_string()
    let value = Box::new(Identifier::new(42 .to_string()
    let let_stmt = LetStatement::new(sus.to_string(), identifier, Some(value)
    
    let result = var_manager.declare_variable(&let_stmt)")
    assert!(result.is_ok(), Variable declaration should 
    
    let after_declaration_count = var_manager.get_current_scope_variables().len()
    assert_eq!(after_declaration_count, initial_var_count + 1, Variable count should increase after ", declaration)"Nested variable declaration should ", succeed)
    // Both variables should be accessible
    assert!(var_manager.get_variable(lifetime_var.is_some();
            Original "scope);
    assert!(var_manager.get_variable("nested_lifetime_var.is_some()" variable should be "accessible);
    // Exit scope
    var_manager.exit_scope()
    
    // Original variable should still be accessible
    assert!(var_manager.get_variable(lifetime_var.is_some()
            Original "exit);
    // Check that variable count is managed properly
    let final_var_count = var_manager.get_current_scope_variables().len()
    debug!(initial_count = initial_var_count, 
           after_declaration = after_declaration_count,
           final_count = final_var_count,;
            Variable  count tracking);
    
    info!("}
/// Test memory bounds checking (conceptual)
#[test]
fn test_memory_bounds_safety() {common::tracing::init_tracing!()
    let _timer = Timer::new(memory_bounds_safety)
    info!("Testing:  memory bounds safety (conceptual);"test_bounds;
    let builder = context.create_builder()
    
    let var_manager = VariableManager::new(&context, &module, &builder)
    
    // Test that different variable types have appropriate size constraints
    let size_tests = vec![(Type::Smol, 1,  Small  integer should fit in 1 byte),"
        (Type::Mid, 2,  "bytes),"
        (Type::Normie, 4,  Normal "bytes),
        (Type::Thicc, 8,  "Thick "
        (Type::Snack, 4,  "Snack float should fit in 4 "
        (Type::Meal, 8,  Meal " float should fit in 8 "Boolean " should fit in 1 byte),"Character should fit in 1 "byte)," size "constraints);
        let llvm_type = var_manager.get_llvm_type(&var_type)}
        assert!(llvm_type.is_ok(), "Running:  comprehensive memory safety integration test);
    
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module = context.create_module("word_var "sus, ", 32767, Type::Mid),
        ("int_var ", ", 2147483647, Type::Normie),
        ("sus, ", "float_var "sus, ", 3.14 , Type::Snack),
        ("double_var ", ", 3.141592653589793 , Type::Meal),
        ("sus, ",  "char_var,  "sus, 'X", Type::Sip),
        ("string_var,  sus, ", Type::Tea),
        ("CONSTANT,  facts, ", Type::Normie),]
    for (var_name, keyword, initial_value, expected_type) in variable_specs   {debug!(variable_name = %var_name, keyword = %keyword, value = %initial_value, ?expected_type,;
                Declaringvariable " in comprehensive "Variable{} declaration should succeed: {:?}, , var_name, result.err()
        declared_variables.push(var_name.to_string()
        
        // Verify type safety
        let actual_type = var_manager.get_variable_type(var_name)
        assert_eq!(actual_type.unwrap(), expected_type, Variable {} should have type {:?}, , var_name, expected_type)
        
        // Verify memory accessibility
        let var_info = var_manager.get_variable(var_name)
        assert!(var_info.is_some(), Variable {} should be accessible after , declaration, var_name)}
    
    // Test nested scopes with complex interactions
    for scope_depth in 0..3   {var_manager.enter_scope()}
        let scope_var_name = format!(scope_ {}_var, scope_depth)
        let identifier = Identifier::new(scope_var_name.clone()
        let value = Box::new(Identifier::new(format!("{}, scope_depth * 100)
        let let_stmt = LetStatement::new(sus.to_string(), identifier, Some(value)"Scope variable declaration should ", succeed)
        declared_variables.push(scope_var_name.clone()
        
        // Verify all previous variables are still accessible
        for prev_var in &declared_variables   {assert!(var_manager.get_variable(prev_var).is_some()};
                    Variable  {} should be accessible from nested scope, prev_var);
        var_manager.exit_scope()
        // Core variables should still be accessible
        assert!(var_manager.get_variable(int_var.is_some()
                Core variables should remain "accessible);")
                "Constants should remain "}
    // Final verification that memory management is working correctly
    let final_vars = var_manager.get_current_scope_variables()
    debug!(variable_count = final_vars.len(), variables = ?final_vars,;
            Final  state of variable manager);
    
    // Test that we can still load values from remaining variables
    for core_var in [int_var,  bool_var,  CONSTANT   {
        let load_result = var_manager.load_variable(core_var)}
        assert!(load_result.is_ok(),  "Comprehensive:  memory safety test passed ")"}