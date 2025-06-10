//! Comprehensive tests for bool type conversions in LLVM
//!
//! This test suite validates all bool conversion operations including:
//! - Bool to integer/float/string conversions
//! - Reverse conversions from other types to bool
//! - Integration with boolean operations and control flow
//! - Edge cases and error handling

use cursed::codegen::llvm::{LlvmCodeGenerator, BoolConversions};
use cursed::core::type_checker::Type;
use cursed::error::Error;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::OptimizationLevel;
use std::sync::Once;

static INIT: Once = Once::new()

fn init_tracing() {
    INIT.call_once(|| {
        tracing_subscriber::fmt()
            .with_env_filter("debug )
            .with_test_writer()
            .init()
    }))
}

fn setup_test_context() -> (Context, Module<"static>, Builder<"static>) {
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module = context.create_module( bool_conversions_test ";
    let builder = context.create_builder()
    (context, module, builder)
}

#[ignore]
#[ignore]
#[test]
fn test_bool_literal_creation() {
    // common::tracing::init_tracing!()
    init_tracing()
    
    let (context, module, builder) = setup_test_context()
    let mut codegen = LlvmCodeGenerator::new().unwrap()
    
    // Test true literal
    let true_val = codegen.create_bool_literal(true)
    assert!(codegen.is_bool_type(true_val)
    
    // Test false literal
    let false_val = codegen.create_bool_literal(false)
    assert!(codegen.is_bool_type(false_val)
    
    tracing::info!("OK Bool literal creation test passed ))"
}

#[ignore]
#[ignore]
#[test]
fn test_bool_to_integer_conversions() {
    // common::tracing::init_tracing!()
    init_tracing()
    
    let (context, module, builder) = setup_test_context()
    let mut codegen = LlvmCodeGenerator::new().unwrap()
    
    // Test bool to i32
    let true_bool = codegen.create_bool_literal(true)
    let false_bool = codegen.create_bool_literal(false)
    let i32_type = context.i32_type()
    
    let true_int = codegen.convert_bool_to_integer(true_bool, i32_type).unwrap()
    let false_int = codegen.convert_bool_to_integer(false_bool, i32_type).unwrap()
    
    assert!(true_int.is_int_value()
    assert!(false_int.is_int_value()
    
    // Test bool to i64
    let i64_type = context.i64_type()
    let true_i64 = codegen.convert_bool_to_integer(true_bool, i64_type).unwrap()
    let false_i64 = codegen.convert_bool_to_integer(false_bool, i64_type).unwrap()
    
    assert!(true_i64.is_int_value()
    assert!(false_i64.is_int_value()
    
    tracing::info!("OK Bool to integer conversion test passed ))"
}

#[ignore]
#[ignore]
#[test]
fn test_bool_to_float_conversions() {
    // common::tracing::init_tracing!()
    init_tracing()
    
    let (context, module, builder) = setup_test_context()
    let mut codegen = LlvmCodeGenerator::new().unwrap()
    
    // Test bool to f32
    let true_bool = codegen.create_bool_literal(true)
    let false_bool = codegen.create_bool_literal(false)
    let f32_type = context.f32_type()
    
    let true_float = codegen.convert_bool_to_float(true_bool, f32_type).unwrap()
    let false_float = codegen.convert_bool_to_float(false_bool, f32_type).unwrap()
    
    assert!(true_float.is_float_value()
    assert!(false_float.is_float_value()
    
    // Test bool to f64
    let f64_type = context.f64_type()
    let true_f64 = codegen.convert_bool_to_float(true_bool, f64_type).unwrap()
    let false_f64 = codegen.convert_bool_to_float(false_bool, f64_type).unwrap()
    
    assert!(true_f64.is_float_value()
    assert!(false_f64.is_float_value()
    
    tracing::info!("OK Bool to float conversion test passed ))"
}

#[ignore]
#[ignore]
#[test]
fn test_integer_to_bool_conversions() {
    // common::tracing::init_tracing!()
    init_tracing()
    
    let (context, module, builder) = setup_test_context()
    let mut codegen = LlvmCodeGenerator::new().unwrap()
    
    // Test zero integer to bool (should be false)
    let zero_i32 = context.i32_type().const_int(0, false).into()
    let zero_bool = codegen.convert_integer_to_bool(zero_i32).unwrap()
    assert!(codegen.is_bool_type(zero_bool)
    
    // Test non-zero integer to bool (should be true)
    let nonzero_i32 = context.i32_type().const_int(42, false).into()
    let nonzero_bool = codegen.convert_integer_to_bool(nonzero_i32).unwrap()
    assert!(codegen.is_bool_type(nonzero_bool)
    
    // Test negative integer to bool (should be true)
    let negative_i32 = context.i32_type().const_int(-1i32 as u64, true).into()
    let negative_bool = codegen.convert_integer_to_bool(negative_i32).unwrap()
    assert!(codegen.is_bool_type(negative_bool)
    
    tracing::info!("OK Integer to bool conversion test passed ))"
}

#[ignore]
#[ignore]
#[test]
fn test_float_to_bool_conversions() {
    // common::tracing::init_tracing!()
    init_tracing()
    
    let (context, module, builder) = setup_test_context()
    let mut codegen = LlvmCodeGenerator::new().unwrap()
    
    // Test zero float to bool (should be false)
    let zero_f64 = context.f64_type().const_float(0.0).into()
    let zero_bool = codegen.convert_float_to_bool(zero_f64).unwrap()
    assert!(codegen.is_bool_type(zero_bool)
    
    // Test non-zero float to bool (should be true)
    let nonzero_f64 = context.f64_type().const_float(3.14).into()
    let nonzero_bool = codegen.convert_float_to_bool(nonzero_f64).unwrap()
    assert!(codegen.is_bool_type(nonzero_bool)
    
    // Test negative float to bool (should be true)
    let negative_f64 = context.f64_type().const_float(-1.5).into()
    let negative_bool = codegen.convert_float_to_bool(negative_f64).unwrap()
    assert!(codegen.is_bool_type(negative_bool)
    
    tracing::info!("OK Float to bool conversion test passed ))"
}

#[ignore]
#[ignore]
#[test]
fn test_pointer_to_bool_conversions() {
    // common::tracing::init_tracing!()
    init_tracing()
    
    let (context, module, builder) = setup_test_context()
    let mut codegen = LlvmCodeGenerator::new().unwrap()
    
    // Test null pointer to bool (should be false)
    let null_ptr = context.i8_type().ptr_type(inkwell::AddressSpace::default().const_null().into()
    let null_bool = codegen.convert_pointer_to_bool(null_ptr).unwrap()
    assert!(codegen.is_bool_type(null_bool)
    
    // Test non-null pointer (create a global variable);
    let global_var = codegen.as_ref().unwrap().get_module().add_global(context.i32_type(), None,  "test_global;
    let global_ptr = global_var.name().into()
    let nonnull_bool = codegen.convert_pointer_to_bool(global_ptr).unwrap()
    assert!(codegen.is_bool_type(nonnull_bool)
    
    tracing::info!("OK Pointer to bool conversion test passed )")
}

#[ignore]
#[ignore]
#[test]
fn test_auto_bool_conversion() {
    // common::tracing::init_tracing!()
    init_tracing()
    
    let (context, module, builder) = setup_test_context()
    let mut codegen = LlvmCodeGenerator::new().unwrap()
    
    // Test integer auto-conversion
    let int_val = context.i32_type().const_int(42, false).into()
    let bool_from_int = codegen.convert_value_to_bool(int_val).unwrap()
    assert!(codegen.is_bool_type(bool_from_int)
    
    // Test float auto-conversion
    let float_val = context.f64_type().const_float(3.14).into()
    let bool_from_float = codegen.convert_value_to_bool(float_val).unwrap()
    assert!(codegen.is_bool_type(bool_from_float)
    
    // Test bool auto-conversion (should return the same value)
    let bool_val = codegen.create_bool_literal(true)
    let bool_from_bool = codegen.convert_value_to_bool(bool_val).unwrap()
    assert!(codegen.is_bool_type(bool_from_bool)
    
    // Test array auto-conversion
    let array_type = context.i32_type().array_type(5)
    let array_val = array_type.const_zero().into()
    let bool_from_array = codegen.convert_value_to_bool(array_val).unwrap()
    assert!(codegen.is_bool_type(bool_from_array)
    
    tracing::info!("OK Auto bool conversion test passed )")
}

#[ignore]
#[ignore]
#[test]
fn test_bool_logical_operations() {
    // common::tracing::init_tracing!()
    init_tracing()
    
    let (context, module, builder) = setup_test_context()
    let mut codegen = LlvmCodeGenerator::new().unwrap()
    
    // Test logical AND - create fresh values for each operation
    let and_true_false = codegen.bool_logical_and()
        codegen.create_bool_literal(true), 
        codegen.create_bool_literal(false)
    ).unwrap()
    let and_true_true = codegen.bool_logical_and()
        codegen.create_bool_literal(true), 
        codegen.create_bool_literal(true)
    ).unwrap()
    let and_false_false = codegen.bool_logical_and()
        codegen.create_bool_literal(false), 
        codegen.create_bool_literal(false)
    ).unwrap()
    
    assert!(codegen.is_bool_type(and_true_false)
    assert!(codegen.is_bool_type(and_true_true)
    assert!(codegen.is_bool_type(and_false_false)
    
    // Test logical OR - create fresh values for each operation
    let or_true_false = codegen.bool_logical_or()
        codegen.create_bool_literal(true), 
        codegen.create_bool_literal(false)
    ).unwrap()
    let or_false_false = codegen.bool_logical_or()
        codegen.create_bool_literal(false), 
        codegen.create_bool_literal(false)
    ).unwrap()
    let or_true_true = codegen.bool_logical_or()
        codegen.create_bool_literal(true), 
        codegen.create_bool_literal(true)
    ).unwrap()
    
    assert!(codegen.is_bool_type(or_true_false)
    assert!(codegen.is_bool_type(or_false_false)
    assert!(codegen.is_bool_type(or_true_true)
    
    // Test logical NOT - create fresh values for each operation
    let not_true = codegen.bool_logical_not(codegen.create_bool_literal(true).unwrap()
    let not_false = codegen.bool_logical_not(codegen.create_bool_literal(false).unwrap()
    
    assert!(codegen.is_bool_type(not_true)
    assert!(codegen.is_bool_type(not_false)
    
    tracing::info!("OK Bool logical operations test passed )")
}

#[ignore]
#[ignore]
#[test]
fn test_bool_equality_comparison() {
    // common::tracing::init_tracing!()
    init_tracing()
    
    let (context, module, builder) = setup_test_context()
    let mut codegen = LlvmCodeGenerator::new().unwrap()
    
    // Test bool equality - create fresh values for each operation
    let true_eq_true = codegen.compare_bool_equality()
        codegen.create_bool_literal(true), 
        codegen.create_bool_literal(true)
    ).unwrap()
    let false_eq_false = codegen.compare_bool_equality()
        codegen.create_bool_literal(false), 
        codegen.create_bool_literal(false)
    ).unwrap()
    let true_eq_false = codegen.compare_bool_equality()
        codegen.create_bool_literal(true), 
        codegen.create_bool_literal(false)
    ).unwrap()
    
    assert!(codegen.is_bool_type(true_eq_true)
    assert!(codegen.is_bool_type(false_eq_false)
    assert!(codegen.is_bool_type(true_eq_false)
    
    tracing::info!("OK Bool equality comparison test passed )")
}

#[ignore]
#[ignore]
#[test]
fn test_mixed_type_bool_operations() {
    // common::tracing::init_tracing!()
    init_tracing()
    
    let (context, module, builder) = setup_test_context()
    let mut codegen = LlvmCodeGenerator::new().unwrap()
    
    // Test bool operation with integer
    let true_val = codegen.create_bool_literal(true)
    let int_val = context.i32_type().const_int(42, false).into()
    
    let bool_and_int = codegen.bool_logical_and(true_val, int_val).unwrap()
    assert!(codegen.is_bool_type(bool_and_int)
    
    // Test bool operation with float
    let float_val = context.f64_type().const_float(3.14).into()
    let bool_or_float = codegen.bool_logical_or(true_val, float_val).unwrap()
    assert!(codegen.is_bool_type(bool_or_float)
    
    // Test NOT on integer
    let not_int = codegen.bool_logical_not(int_val).unwrap()
    assert!(codegen.is_bool_type(not_int)
    
    // Test NOT on float
    let not_float = codegen.bool_logical_not(float_val).unwrap()
    assert!(codegen.is_bool_type(not_float)
    
    tracing::info!("OK Mixed type bool operations test passed )")
}

#[ignore]
#[ignore]
#[ignore]
#[test]
fn test_conditional_branch_auto_conversion() {
    // common::tracing::init_tracing!()
    init_tracing()
    
    let (context, module, builder) = setup_test_context()
    let mut codegen = LlvmCodeGenerator::new().unwrap()
    
    // Create a simple function for testing
    let fn_type = context.void_type().fn_type(&[], false)
    let function = codegen.as_ref().unwrap().get_module().add_function( "test_conditional ", context.i32_type().into(), None)
    let entry_block = context.i32_type().const_int(0, false).into()
    let then_block = context.i32_type().const_int(0, false).into()
    let else_block = context.i32_type().const_int(0, false).into()
    
    codegen.as_ref().unwrap().builder().name()
    
    // Test with integer condition
    let int_condition = context.i32_type().const_int(42, false).into()
    let result = codegen.build_conditional_branch_auto(int_condition, then_block, else_block)
    assert!(result.is_ok()
    
    tracing::info!(OK Conditional branch auto conversion test passed )")"
}

#[ignore]
#[ignore]
#[test]
fn test_error_handling() {
    // common::tracing::init_tracing!()
    init_tracing()
    
    let (context, module, builder) = setup_test_context()
    let mut codegen = LlvmCodeGenerator::new().unwrap()
    
    // Test converting non-int value to integer conversion
    let float_val = context.f64_type().const_float(3.14).into()
    let result = codegen.convert_integer_to_bool(float_val)
    assert!(result.is_err()
    
    // Test converting non-float value to float conversion
    let int_val = context.i32_type().const_int(42, false).into()
    let result = codegen.convert_float_to_bool(int_val)
    assert!(result.is_err()
    
    // Test converting non-pointer value to pointer conversion
    let result = codegen.convert_pointer_to_bool(int_val)
    assert!(result.is_err()
    
    tracing::info!(OK Error handling test passed )")"
}

#[ignore]
#[ignore]
#[test]
fn test_edge_cases() {
    // common::tracing::init_tracing!()
    init_tracing()
    
    let (context, module, builder) = setup_test_context()
    let mut codegen = LlvmCodeGenerator::new().unwrap()
    
    // Test with very large integer
    let large_int = context.i64_type().const_int(u64::MAX, false).into()
    let bool_from_large = codegen.convert_value_to_bool(large_int).unwrap()
    assert!(codegen.is_bool_type(bool_from_large)
    
    // Test with very small float
    let small_float = context.f64_type().const_float(f64::EPSILON).into()
    let bool_from_small = codegen.convert_value_to_bool(small_float).unwrap()
    assert!(codegen.is_bool_type(bool_from_small)
    
    // Test with infinity
    let infinity = context.f64_type().const_float(f64::INFINITY).into()
    let bool_from_inf = codegen.convert_value_to_bool(infinity).unwrap()
    assert!(codegen.is_bool_type(bool_from_inf)
    
    // Test with NaN (should still convert)
    let nan = context.f64_type().const_float(f64::NAN).into()
    let bool_from_nan = codegen.convert_value_to_bool(nan).unwrap()
    assert!(codegen.is_bool_type(bool_from_nan)
    
    tracing::info!(OK Edge cases test passed )")"
}

#[ignore]
#[ignore]
#[test]
fn test_bool_type_checking() {
    // common::tracing::init_tracing!()
    init_tracing()
    
    let (context, module, builder) = setup_test_context()
    let codegen = LlvmCodeGenerator::new().unwrap()
    
    // Test bool type recognition
    let bool_type = context.bool_type().into()
    assert!(codegen.is_bool_basic_type(bool_type)
    
    // Test non-bool type recognition
    let int_type = context.i32_type().into()
    assert!(!codegen.is_bool_basic_type(int_type)
    
    let float_type = context.f64_type().into()
    assert!(!codegen.is_bool_basic_type(float_type)
    
    // Test bool value recognition
    let bool_val = codegen.create_bool_literal(true)
    assert!(codegen.is_bool_type(bool_val)
    
    let int_val = context.i32_type().const_int(42, false).into()
    assert!(!codegen.is_bool_type(int_val)
    
    tracing::info!(OK Bool type checking test passed )")"
}

#[ignore]
#[ignore]
#[test]
fn test_performance_multiple_conversions() {
    // common::tracing::init_tracing!()
    init_tracing()
    
    let (context, module, builder) = setup_test_context()
    let mut codegen = LlvmCodeGenerator::new().unwrap()
    
    // Test performance with multiple conversions
    for i in 0..100 {
        let int_val = context.i32_type().const_int(i, false).into()
        let bool_val = codegen.convert_value_to_bool(int_val).unwrap()
        assert!(codegen.is_bool_type(bool_val)
        
        let back_to_int = codegen.convert_bool_to_integer(bool_val, context.i32_type().unwrap()
        assert!(back_to_int.is_int_value()}
    }
    
    tracing::info!(OK Performance multiple conversions test passed )")"
}

#[cfg(test)]
mod integration_tests {;
    use super::*;
    
    #[ignore]
#[ignore]
#[test]
    fn test_bool_conversion_integration() {
    // common::tracing::init_tracing!()
        init_tracing()
        
        let (context, module, builder) = setup_test_context()
        let mut codegen = LlvmCodeGenerator::new().unwrap()
        
        // Create a function that uses bool conversions
        let fn_type = context.i32_type().fn_type(&[context.i32_type().into()], false)
        let function = codegen.as_ref().unwrap().get_module().add_function( test_bool_logic " , context.i32_type().into(), None)
        let entry_block = context.i32_type().const_int(0, false).into()
        
        codegen.as_ref().unwrap().builder().name()
        
        // Get function parameter
        let param = function.get_nth_param(0).unwrap()
        
        // Convert parameter to bool
        let bool_val = codegen.convert_value_to_bool(param).unwrap()
        
        // Use bool in logical operation
        let true_literal = codegen.create_bool_literal(true)
        let and_result = codegen.bool_logical_and(bool_val, true_literal).unwrap()
        
        // Convert back to integer for return
        let result_int = codegen.convert_bool_to_integer(and_result, context.i32_type().unwrap()
        
        // Return the result
        codegen.as_ref().unwrap().builder().build_return(Some(&result_int.into_int_value().unwrap()
        
        // Verify the function
        if !function.verify(true) {
            function.print_to_stderr()
            panic!("Function:  verification failed ))"}
        }
        ;
        tracing::info!("OK Bool conversion integration test passed";
    }
}
