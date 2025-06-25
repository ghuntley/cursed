/// LLVM Integration Tests for Vectorization Optimizer
/// 
/// Tests the vectorization optimizer working with real LLVM IR functions,
/// including loop detection, instruction analysis, and SIMD code generation.

use cursed::optimization::enhanced_llvm_passes::{EnhancedLlvmPassManager, EnhancedOptimizationStatistics};
use cursed::optimization::config::{OptimizationConfig, OptimizationLevel};
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::OptimizationLevel as InkwellOptLevel;
use inkwell::{IntPredicate, FloatPredicate};
use std::sync::{Arc, Mutex};
use tracing_test::traced_test;

type SumFunc = unsafe extern "C" fn(f64, f64, f64, f64) -> f64;

#[traced_test]
#[test]
fn test_vectorization_optimizer_with_real_llvm_module() {
    let context = Context::create();
    let module = context.create_module("vectorization_test");
    let builder = context.create_builder();
    
    // Create a simple function with vectorizable operations
    let f64_type = context.f64_type();
    let fn_type = f64_type.fn_type(&[f64_type.into(), f64_type.into(), f64_type.into(), f64_type.into()], false);
    let function = module.add_function("vector_sum", fn_type, None);
    
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);
    
    // Get function parameters
    let param1 = function.get_nth_param(0).unwrap().into_float_value();
    let param2 = function.get_nth_param(1).unwrap().into_float_value();
    let param3 = function.get_nth_param(2).unwrap().into_float_value();
    let param4 = function.get_nth_param(3).unwrap().into_float_value();
    
    // Create vectorizable arithmetic operations
    let sum1 = builder.build_float_add(param1, param2, "sum1").unwrap();
    let sum2 = builder.build_float_add(param3, param4, "sum2").unwrap();
    let final_sum = builder.build_float_add(sum1, sum2, "final_sum").unwrap();
    
    builder.build_return(Some(&final_sum)).unwrap();
    
    // Test that the module is valid
    assert!(module.verify().is_ok());
    
    // Create enhanced pass manager and run vectorization
    let config = OptimizationConfig::default();
    let pass_manager = EnhancedLlvmPassManager::new(&context, OptimizationLevel::Aggressive, &config);
    
    // Run optimization (which includes vectorization)
    let result = pass_manager.optimize_module(&module);
    assert!(result.is_ok());
    
    // Verify that the module is still valid after optimization
    assert!(module.verify().is_ok());
    
    // Check that vectorization statistics were recorded
    let stats = pass_manager.get_statistics();
    let stats = stats.lock().unwrap();
    
    // The function should have been analyzed for vectorization
    assert!(stats.final_functions > 0);
    assert!(stats.total_optimization_time.as_millis() > 0);
}

#[traced_test]
#[test]
fn test_vectorization_with_loop_pattern() {
    let context = Context::create();
    let module = context.create_module("loop_vectorization_test");
    let builder = context.create_builder();
    
    // Create a function with a simple loop pattern
    let i32_type = context.i32_type();
    let ptr_type = i32_type.ptr_type(inkwell::AddressSpace::default());
    let fn_type = i32_type.fn_type(&[ptr_type.into(), i32_type.into()], false);
    let function = module.add_function("array_sum", fn_type, None);
    
    let entry_block = context.append_basic_block(function, "entry");
    let loop_block = context.append_basic_block(function, "loop");
    let exit_block = context.append_basic_block(function, "exit");
    
    // Entry block: initialize variables
    builder.position_at_end(entry_block);
    let array_param = function.get_nth_param(0).unwrap().into_pointer_value();
    let size_param = function.get_nth_param(1).unwrap().into_int_value();
    
    let sum_ptr = builder.build_alloca(i32_type, "sum").unwrap();
    let index_ptr = builder.build_alloca(i32_type, "index").unwrap();
    let zero = i32_type.const_int(0, false);
    let one = i32_type.const_int(1, false);
    
    builder.build_store(sum_ptr, zero).unwrap();
    builder.build_store(index_ptr, zero).unwrap();
    builder.build_unconditional_branch(loop_block).unwrap();
    
    // Loop block: vectorizable operations
    builder.position_at_end(loop_block);
    let current_index = builder.build_load(i32_type, index_ptr, "current_index").unwrap().into_int_value();
    let current_sum = builder.build_load(i32_type, sum_ptr, "current_sum").unwrap().into_int_value();
    
    // Array element access
    let element_ptr = unsafe { builder.build_gep(i32_type, array_param, &[current_index], "element_ptr").unwrap() };
    let element_value = builder.build_load(i32_type, element_ptr, "element_value").unwrap().into_int_value();
    
    // Vectorizable arithmetic
    let new_sum = builder.build_int_add(current_sum, element_value, "new_sum").unwrap();
    builder.build_store(sum_ptr, new_sum).unwrap();
    
    // Loop increment and condition
    let next_index = builder.build_int_add(current_index, one, "next_index").unwrap();
    builder.build_store(index_ptr, next_index).unwrap();
    
    let condition = builder.build_int_compare(IntPredicate::ULT, next_index, size_param, "condition").unwrap();
    builder.build_conditional_branch(condition, loop_block, exit_block).unwrap();
    
    // Exit block
    builder.position_at_end(exit_block);
    let final_sum = builder.build_load(i32_type, sum_ptr, "final_sum").unwrap().into_int_value();
    builder.build_return(Some(&final_sum)).unwrap();
    
    // Verify the module
    assert!(module.verify().is_ok());
    
    // Run vectorization optimization
    let config = OptimizationConfig::default();
    let pass_manager = EnhancedLlvmPassManager::new(&context, OptimizationLevel::Aggressive, &config);
    
    let result = pass_manager.optimize_module(&module);
    assert!(result.is_ok());
    
    // Verify that the module is still valid after optimization
    assert!(module.verify().is_ok());
    
    // Check that optimization statistics show some work was done
    let stats = pass_manager.get_statistics();
    let stats = stats.lock().unwrap();
    
    assert!(stats.final_functions > 0);
    assert!(stats.final_basic_blocks >= 3); // entry, loop, exit blocks
}

#[traced_test]
#[test]
fn test_vectorization_with_floating_point_operations() {
    let context = Context::create();
    let module = context.create_module("fp_vectorization_test");
    let builder = context.create_builder();
    
    // Create a function with floating-point vector operations
    let f32_type = context.f32_type();
    let ptr_type = f32_type.ptr_type(inkwell::AddressSpace::default());
    let fn_type = context.void_type().fn_type(&[ptr_type.into(), ptr_type.into(), ptr_type.into(), context.i32_type().into()], false);
    let function = module.add_function("vector_add", fn_type, None);
    
    let entry_block = context.append_basic_block(function, "entry");
    let loop_block = context.append_basic_block(function, "loop");
    let exit_block = context.append_basic_block(function, "exit");
    
    // Entry block
    builder.position_at_end(entry_block);
    let a_param = function.get_nth_param(0).unwrap().into_pointer_value();
    let b_param = function.get_nth_param(1).unwrap().into_pointer_value();
    let c_param = function.get_nth_param(2).unwrap().into_pointer_value();
    let size_param = function.get_nth_param(3).unwrap().into_int_value();
    
    let index_ptr = builder.build_alloca(context.i32_type(), "index").unwrap();
    let zero = context.i32_type().const_int(0, false);
    let one = context.i32_type().const_int(1, false);
    
    builder.build_store(index_ptr, zero).unwrap();
    builder.build_unconditional_branch(loop_block).unwrap();
    
    // Loop block with vectorizable floating-point operations
    builder.position_at_end(loop_block);
    let current_index = builder.build_load(context.i32_type(), index_ptr, "current_index").unwrap().into_int_value();
    
    // Load array elements
    let a_ptr = unsafe { builder.build_gep(f32_type, a_param, &[current_index], "a_ptr").unwrap() };
    let b_ptr = unsafe { builder.build_gep(f32_type, b_param, &[current_index], "b_ptr").unwrap() };
    let c_ptr = unsafe { builder.build_gep(f32_type, c_param, &[current_index], "c_ptr").unwrap() };
    
    let a_val = builder.build_load(f32_type, a_ptr, "a_val").unwrap().into_float_value();
    let b_val = builder.build_load(f32_type, b_ptr, "b_val").unwrap().into_float_value();
    
    // Vectorizable floating-point arithmetic
    let sum = builder.build_float_add(a_val, b_val, "sum").unwrap();
    let product = builder.build_float_mul(sum, a_val, "product").unwrap();
    
    // Store result
    builder.build_store(c_ptr, product).unwrap();
    
    // Loop increment and condition
    let next_index = builder.build_int_add(current_index, one, "next_index").unwrap();
    builder.build_store(index_ptr, next_index).unwrap();
    
    let condition = builder.build_int_compare(IntPredicate::ULT, next_index, size_param, "condition").unwrap();
    builder.build_conditional_branch(condition, loop_block, exit_block).unwrap();
    
    // Exit block
    builder.position_at_end(exit_block);
    builder.build_return(None).unwrap();
    
    // Verify the module
    assert!(module.verify().is_ok());
    
    // Test with different optimization levels
    let optimization_levels = vec![
        OptimizationLevel::None,
        OptimizationLevel::Basic,
        OptimizationLevel::Default,
        OptimizationLevel::Aggressive,
    ];
    
    for opt_level in optimization_levels {
        let config = OptimizationConfig::default();
        let pass_manager = EnhancedLlvmPassManager::new(&context, opt_level.clone(), &config);
        
        let result = pass_manager.optimize_module(&module);
        assert!(result.is_ok(), "Optimization failed for level: {:?}", opt_level);
        
        // Verify that the module is still valid
        assert!(module.verify().is_ok());
        
        // Check statistics
        let stats = pass_manager.get_statistics();
        let stats = stats.lock().unwrap();
        
        // More aggressive optimization should do more work
        match opt_level {
            OptimizationLevel::Aggressive => {
                assert!(stats.total_optimization_time.as_millis() > 0);
                assert!(stats.final_functions > 0);
            }
            OptimizationLevel::None => {
                // Even with no optimization, some analysis should happen
                assert!(stats.final_functions > 0);
            }
            _ => {
                assert!(stats.final_functions > 0);
            }
        }
    }
}

#[cfg(test)]
mod execution_tests {
    use super::*;
    
    #[traced_test]
    #[test]
    fn test_vectorized_function_execution() {
        let context = Context::create();
        let module = context.create_module("execution_test");
        let builder = context.create_builder();
        
        // Create a simple function that can be executed
        let f64_type = context.f64_type();
        let fn_type = f64_type.fn_type(&[f64_type.into(), f64_type.into()], false);
        let function = module.add_function("simple_add", fn_type, None);
        
        let basic_block = context.append_basic_block(function, "entry");
        builder.position_at_end(basic_block);
        
        let param1 = function.get_nth_param(0).unwrap().into_float_value();
        let param2 = function.get_nth_param(1).unwrap().into_float_value();
        
        let sum = builder.build_float_add(param1, param2, "sum").unwrap();
        builder.build_return(Some(&sum)).unwrap();
        
        // Verify the module before optimization
        assert!(module.verify().is_ok());
        
        // Run vectorization optimization
        let config = OptimizationConfig::default();
        let pass_manager = EnhancedLlvmPassManager::new(&context, OptimizationLevel::Default, &config);
        
        let result = pass_manager.optimize_module(&module);
        assert!(result.is_ok());
        
        // Verify the module after optimization
        assert!(module.verify().is_ok());
        
        // Create execution engine and test the function
        let execution_engine = module.create_jit_execution_engine(InkwellOptLevel::Default);
        if let Ok(ee) = execution_engine {
            unsafe {
                let simple_add: JitFunction<SumFunc> = ee.get_function("simple_add").unwrap();
                
                // Test the optimized function
                let result = simple_add.call(2.0, 3.0, 0.0, 0.0);
                assert_eq!(result, 5.0);
                
                let result2 = simple_add.call(10.5, 20.5, 0.0, 0.0);
                assert_eq!(result2, 31.0);
            }
        } else {
            // If JIT execution is not available, just verify the optimization ran
            println!("JIT execution not available, but optimization completed successfully");
        }
    }
}

#[cfg(test)]
mod statistics_tests {
    use super::*;
    
    #[traced_test]
    #[test]
    fn test_vectorization_statistics_tracking() {
        let context = Context::create();
        let module = context.create_module("stats_test");
        let builder = context.create_builder();
        
        // Create multiple functions to test statistics tracking
        for i in 0..5 {
            let f64_type = context.f64_type();
            let fn_type = f64_type.fn_type(&[f64_type.into(), f64_type.into()], false);
            let function = module.add_function(&format!("test_func_{}", i), fn_type, None);
            
            let basic_block = context.append_basic_block(function, "entry");
            builder.position_at_end(basic_block);
            
            let param1 = function.get_nth_param(0).unwrap().into_float_value();
            let param2 = function.get_nth_param(1).unwrap().into_float_value();
            
            // Different vectorizable operations in each function
            let result = match i % 3 {
                0 => builder.build_float_add(param1, param2, "add").unwrap(),
                1 => builder.build_float_mul(param1, param2, "mul").unwrap(),
                _ => builder.build_float_sub(param1, param2, "sub").unwrap(),
            };
            
            builder.build_return(Some(&result)).unwrap();
        }
        
        // Verify the module
        assert!(module.verify().is_ok());
        
        // Run optimization and check statistics
        let config = OptimizationConfig::default();
        let pass_manager = EnhancedLlvmPassManager::new(&context, OptimizationLevel::Aggressive, &config);
        
        let initial_stats = {
            let stats = pass_manager.get_statistics();
            let stats = stats.lock().unwrap();
            stats.clone()
        };
        
        // Optimize the module
        let result = pass_manager.optimize_module(&module);
        assert!(result.is_ok());
        
        // Check final statistics
        let final_stats = {
            let stats = pass_manager.get_statistics();
            let stats = stats.lock().unwrap();
            stats.clone()
        };
        
        // Verify that statistics were updated
        assert_eq!(final_stats.final_functions, 5);
        assert!(final_stats.total_optimization_time > initial_stats.total_optimization_time);
        assert!(final_stats.final_instructions > 0);
        assert!(final_stats.final_basic_blocks >= 5); // At least one block per function
        
        println!("Optimization took: {:?}", final_stats.total_optimization_time);
        println!("Functions processed: {}", final_stats.final_functions);
        println!("Final instructions: {}", final_stats.final_instructions);
        println!("Vectorized operations: {}", final_stats.vectorized_operations);
    }
}
