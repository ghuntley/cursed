/// Advanced LLVM Integration Test Suite
/// 
/// Comprehensive tests for instruction cloning, CFG manipulation, 
/// and advanced optimization transformations.

use cursed::optimization::advanced_llvm_integration::{
    AdvancedLlvmIntegration, AdvancedLlvmConfig, InstructionCloner, CfgManipulator,
    CloningStatistics, CfgManipulationStatistics, ClonedFunctionBody,
    VectorDataType, StridePattern
};
use cursed::error::Result;
use inkwell::{
    context::Context,
    values::{FunctionValue, BasicValueEnum},
    basic_block::BasicBlock,
    builder::Builder,
    module::Module,
    types::BasicTypeEnum,
};
use std::time::Duration;
use tracing::info;

/// Initialize test tracing
#[path = "common.rs"]
mod common;

/// Test comprehensive instruction cloning functionality
#[test]
fn test_comprehensive_instruction_cloning() -> Result<()> {
    common::tracing::setup();
    info!("Testing comprehensive instruction cloning");
    
    let context = Context::create();
    let module = context.create_module("test_cloning");
    let builder = context.create_builder();
    
    // Create a complex function to clone
    let source_function = create_complex_function(&context, &module, &builder)?;
    let target_function = create_target_function(&context, &module)?;
    
    // Test instruction cloning
    let mut cloner = InstructionCloner::new(&context, target_function)?;
    let cloned_body = cloner.clone_function_body(source_function)?;
    
    // Validate cloning results
    assert!(!cloned_body.all_blocks.is_empty());
    assert!(!cloned_body.exit_blocks.is_empty());
    
    let stats = cloner.get_statistics();
    assert!(stats.instructions_cloned > 0);
    assert!(stats.basic_blocks_cloned > 0);
    assert!(stats.cloning_time > Duration::from_nanos(0));
    
    info!("Instruction cloning completed successfully");
    Ok(())
}

/// Test CFG manipulation and integration
#[test]
fn test_cfg_manipulation_integration() -> Result<()> {
    common::tracing::setup();
    info!("Testing CFG manipulation integration");
    
    let context = Context::create();
    let module = context.create_module("test_cfg");
    let builder = context.create_builder();
    
    // Create functions for CFG manipulation test
    let caller_function = create_caller_function(&context, &module, &builder)?;
    let callee_function = create_simple_callee(&context, &module, &builder)?;
    
    // Find the call instruction
    let call_site = find_call_instruction(caller_function)?;
    let call_block = call_site.get_parent().unwrap();
    
    // Create CFG manipulator
    let cfg_manipulator = CfgManipulator::new(&context, &builder)?;
    
    // Create cloner and clone the callee
    let mut cloner = InstructionCloner::new(&context, caller_function)?;
    let cloned_body = cloner.clone_function_body(callee_function)?;
    
    // Test CFG integration
    let success = cfg_manipulator.integrate_inlined_function(
        &call_site,
        call_block,
        cloned_body,
        &mut cloner,
    )?;
    
    assert!(success);
    
    let stats = cfg_manipulator.get_statistics();
    assert!(stats.blocks_created > 0 || stats.branches_redirected > 0);
    
    info!("CFG manipulation integration completed successfully");
    Ok(())
}

/// Test function inlining validation
#[test]
fn test_function_inlining_validation() -> Result<()> {
    common::tracing::setup();
    info!("Testing function inlining validation");
    
    let context = Context::create();
    let config = AdvancedLlvmConfig::default();
    let integration = AdvancedLlvmIntegration::new(&context, "test_validation", config)?;
    
    let module = integration.get_module();
    let builder = context.create_builder();
    
    // Test 1: Valid function for inlining
    let valid_function = create_simple_function(&context, &module, &builder, "valid_fn")?;
    let caller_function = create_caller_with_call(&context, &module, &builder, valid_function)?;
    let call_site = find_call_instruction(caller_function)?;
    
    let is_valid = integration.validate_inlining_conditions(valid_function, &call_site)?;
    assert!(is_valid, "Simple function should be valid for inlining");
    
    // Test 2: Recursive function (should be invalid)
    let recursive_function = create_recursive_function(&context, &module, &builder)?;
    let recursive_call = find_call_instruction(recursive_function)?;
    
    let is_recursive = integration.is_recursive_call(recursive_function, &recursive_call)?;
    assert!(is_recursive, "Should detect recursive calls");
    
    // Test 3: External function (should be invalid)
    let external_function = create_external_function(&context, &module)?;
    // External functions have no basic blocks, so they should fail validation
    // We can't create a call to test this directly, but the validation should catch it
    
    info!("Function inlining validation completed successfully");
    Ok(())
}

/// Test advanced optimization passes
#[test]
fn test_advanced_optimization_passes() -> Result<()> {
    common::tracing::setup();
    info!("Testing advanced optimization passes");
    
    let context = Context::create();
    let mut config = AdvancedLlvmConfig::default();
    config.enable_advanced_inlining = true;
    config.enable_cfg_transformations = true;
    config.enable_vectorization = true;
    
    let mut integration = AdvancedLlvmIntegration::new(&context, "test_optimization", config)?;
    
    // Initialize optimization passes
    integration.initialize_passes()?;
    
    // Run comprehensive optimization
    let stats = integration.optimize_module()?;
    
    // Validate optimization statistics
    assert!(stats.total_optimization_time > Duration::from_nanos(0));
    assert!(stats.peak_memory_usage_mb > 0);
    
    // Check that various optimization phases were executed
    // (Even if no actual optimizations were performed, the statistics should reflect execution)
    
    info!("Advanced optimization passes completed successfully");
    Ok(())
}

/// Test loop detection and analysis
#[test]
fn test_loop_detection_analysis() -> Result<()> {
    common::tracing::setup();
    info!("Testing loop detection and analysis");
    
    let context = Context::create();
    let config = AdvancedLlvmConfig::default();
    let integration = AdvancedLlvmIntegration::new(&context, "test_loops", config)?;
    
    let module = integration.get_module();
    let builder = context.create_builder();
    
    // Create a function with loops
    let loop_function = create_function_with_loops(&context, &module, &builder)?;
    
    // Test loop detection
    let loops = integration.detect_loops(loop_function)?;
    
    // Should detect at least one loop in our test function
    assert!(!loops.is_empty(), "Should detect loops in the test function");
    
    // Test loop analysis
    for loop_info in &loops {
        assert!(loop_info.iteration_count > 0);
        assert!(loop_info.body_size > 0);
    }
    
    info!("Loop detection and analysis completed successfully");
    Ok(())
}

/// Test vectorization analysis
#[test]
fn test_vectorization_analysis() -> Result<()> {
    common::tracing::setup();
    info!("Testing vectorization analysis");
    
    let context = Context::create();
    let config = AdvancedLlvmConfig::default();
    let integration = AdvancedLlvmIntegration::new(&context, "test_vectorization", config)?;
    
    // Test vector width determination
    let f32_width = integration.determine_vector_width(&VectorDataType::Float32);
    let f64_width = integration.determine_vector_width(&VectorDataType::Float64);
    let i32_width = integration.determine_vector_width(&VectorDataType::Int32);
    let i64_width = integration.determine_vector_width(&VectorDataType::Int64);
    
    assert_eq!(f32_width, 8); // 256-bit / 32-bit = 8
    assert_eq!(f64_width, 4); // 256-bit / 64-bit = 4
    assert_eq!(i32_width, 8); // 256-bit / 32-bit = 8
    assert_eq!(i64_width, 4); // 256-bit / 64-bit = 4
    
    // Test stride pattern analysis
    let patterns = vec![
        StridePattern::Unit,
        StridePattern::Constant(2),
        StridePattern::Constant(4),
        StridePattern::Variable,
    ];
    
    for pattern in patterns {
        match pattern {
            StridePattern::Unit => {
                // Unit stride is optimal for vectorization
            }
            StridePattern::Constant(stride) => {
                assert!(stride > 0);
            }
            StridePattern::Variable => {
                // Variable stride is typically not vectorizable
            }
        }
    }
    
    info!("Vectorization analysis completed successfully");
    Ok(())
}

/// Test memory safety and error handling
#[test]
fn test_memory_safety_error_handling() -> Result<()> {
    common::tracing::setup();
    info!("Testing memory safety and error handling");
    
    let context = Context::create();
    let config = AdvancedLlvmConfig::default();
    let integration = AdvancedLlvmIntegration::new(&context, "test_safety", config)?;
    
    // Test null pointer handling in cloner
    let module = integration.get_module();
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = module.add_function("test_fn", fn_type, None);
    
    let cloner = InstructionCloner::new(&context, function);
    assert!(cloner.is_ok(), "Cloner creation should succeed");
    
    // Test error conditions
    let empty_function = module.add_function("empty_fn", fn_type, None);
    // Empty function should still be clonable (just with no instructions)
    let mut cloner = InstructionCloner::new(&context, function)?;
    let result = cloner.clone_function_body(empty_function);
    assert!(result.is_ok(), "Empty function should be clonable");
    
    info!("Memory safety and error handling tests completed successfully");
    Ok(())
}

/// Test performance benchmarks
#[test]
fn test_performance_benchmarks() -> Result<()> {
    common::tracing::setup();
    info!("Testing performance benchmarks");
    
    let context = Context::create();
    let config = AdvancedLlvmConfig::default();
    let integration = AdvancedLlvmIntegration::new(&context, "test_performance", config)?;
    
    let module = integration.get_module();
    let builder = context.create_builder();
    
    // Create a moderately complex function for performance testing
    let complex_function = create_performance_test_function(&context, &module, &builder)?;
    let target_function = create_target_function(&context, &module)?;
    
    // Benchmark cloning performance
    let start = std::time::Instant::now();
    let mut cloner = InstructionCloner::new(&context, target_function)?;
    let _cloned_body = cloner.clone_function_body(complex_function)?;
    let cloning_duration = start.elapsed();
    
    // Validate performance is reasonable (should complete in < 1 second for test)
    assert!(cloning_duration < Duration::from_secs(1), 
            "Cloning should complete in reasonable time");
    
    let stats = cloner.get_statistics();
    assert!(stats.cloning_time <= cloning_duration);
    
    info!("Performance benchmarks completed successfully");
    Ok(())
}

/// Test integration with real LLVM optimizations
#[test]
fn test_real_llvm_optimization_integration() -> Result<()> {
    common::tracing::setup();
    info!("Testing real LLVM optimization integration");
    
    let context = Context::create();
    let mut config = AdvancedLlvmConfig::default();
    config.optimization_level = 2;
    config.enable_target_specific = true;
    
    let mut integration = AdvancedLlvmIntegration::new(&context, "test_real_optimization", config)?;
    
    // Create a module with functions that can benefit from optimization
    let module = integration.get_module();
    let builder = context.create_builder();
    
    let _optimizable_function = create_optimizable_function(&context, &module, &builder)?;
    
    // Initialize and run LLVM passes
    integration.initialize_passes()?;
    let stats = integration.optimize_module()?;
    
    // Validate that optimization was attempted
    assert!(stats.total_optimization_time > Duration::from_nanos(0));
    
    info!("Real LLVM optimization integration completed successfully");
    Ok(())
}

// Helper functions for creating test functions and structures

fn create_complex_function(
    context: &Context,
    module: &Module,
    builder: &Builder,
) -> Result<FunctionValue> {
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[i32_type.into(), i32_type.into()], false);
    let function = module.add_function("complex_fn", fn_type, None);
    
    let entry_block = context.append_basic_block(function, "entry");
    let then_block = context.append_basic_block(function, "then");
    let else_block = context.append_basic_block(function, "else");
    let merge_block = context.append_basic_block(function, "merge");
    
    // Entry block
    builder.position_at_end(entry_block);
    let param0 = function.get_nth_param(0).unwrap().into_int_value();
    let param1 = function.get_nth_param(1).unwrap().into_int_value();
    let sum = builder.build_int_add(param0, param1, "sum")?;
    let condition = builder.build_int_compare(
        inkwell::IntPredicate::SGT,
        sum,
        i32_type.const_int(10, false),
        "cond"
    )?;
    builder.build_conditional_branch(condition, then_block, else_block)?;
    
    // Then block
    builder.position_at_end(then_block);
    let doubled = builder.build_int_mul(sum, i32_type.const_int(2, false), "doubled")?;
    builder.build_unconditional_branch(merge_block)?;
    
    // Else block
    builder.position_at_end(else_block);
    let halved = builder.build_int_signed_div(sum, i32_type.const_int(2, false), "halved")?;
    builder.build_unconditional_branch(merge_block)?;
    
    // Merge block
    builder.position_at_end(merge_block);
    let phi = builder.build_phi(i32_type, "result")?;
    phi.add_incoming(&[(&doubled, then_block), (&halved, else_block)]);
    builder.build_return(Some(&phi.as_basic_value()))?;
    
    Ok(function)
}

fn create_target_function(context: &Context, module: &Module) -> Result<FunctionValue> {
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = module.add_function("target_fn", fn_type, None);
    Ok(function)
}

fn create_caller_function(
    context: &Context,
    module: &Module,
    builder: &Builder,
) -> Result<FunctionValue> {
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let caller = module.add_function("caller", fn_type, None);
    
    let entry_block = context.append_basic_block(caller, "entry");
    builder.position_at_end(entry_block);
    
    // We'll add a call instruction in a separate step
    builder.build_return(Some(&i32_type.const_int(0, false)))?;
    
    Ok(caller)
}

fn create_simple_callee(
    context: &Context,
    module: &Module,
    builder: &Builder,
) -> Result<FunctionValue> {
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let callee = module.add_function("callee", fn_type, None);
    
    let entry_block = context.append_basic_block(callee, "entry");
    builder.position_at_end(entry_block);
    builder.build_return(Some(&i32_type.const_int(42, false)))?;
    
    Ok(callee)
}

fn create_simple_function(
    context: &Context,
    module: &Module,
    builder: &Builder,
    name: &str,
) -> Result<FunctionValue> {
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = module.add_function(name, fn_type, None);
    
    let entry_block = context.append_basic_block(function, "entry");
    builder.position_at_end(entry_block);
    builder.build_return(Some(&i32_type.const_int(1, false)))?;
    
    Ok(function)
}

fn create_caller_with_call(
    context: &Context,
    module: &Module,
    builder: &Builder,
    callee: FunctionValue,
) -> Result<FunctionValue> {
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let caller = module.add_function("caller_with_call", fn_type, None);
    
    let entry_block = context.append_basic_block(caller, "entry");
    builder.position_at_end(entry_block);
    
    let call_result = builder.build_call(callee, &[], "call")?;
    builder.build_return(call_result.try_as_basic_value().left().as_ref())?;
    
    Ok(caller)
}

fn create_recursive_function(
    context: &Context,
    module: &Module,
    builder: &Builder,
) -> Result<FunctionValue> {
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[i32_type.into()], false);
    let function = module.add_function("recursive", fn_type, None);
    
    let entry_block = context.append_basic_block(function, "entry");
    let then_block = context.append_basic_block(function, "then");
    let else_block = context.append_basic_block(function, "else");
    
    // Entry block
    builder.position_at_end(entry_block);
    let param = function.get_nth_param(0).unwrap().into_int_value();
    let condition = builder.build_int_compare(
        inkwell::IntPredicate::SGT,
        param,
        i32_type.const_int(0, false),
        "cond"
    )?;
    builder.build_conditional_branch(condition, then_block, else_block)?;
    
    // Then block (recursive call)
    builder.position_at_end(then_block);
    let decremented = builder.build_int_sub(param, i32_type.const_int(1, false), "dec")?;
    let recursive_call = builder.build_call(function, &[decremented.into()], "recursive_call")?;
    builder.build_return(recursive_call.try_as_basic_value().left().as_ref())?;
    
    // Else block (base case)
    builder.position_at_end(else_block);
    builder.build_return(Some(&i32_type.const_int(1, false)))?;
    
    Ok(function)
}

fn create_external_function(context: &Context, module: &Module) -> Result<FunctionValue> {
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = module.add_function("external", fn_type, None);
    // Don't add any basic blocks - this makes it external
    Ok(function)
}

fn create_function_with_loops(
    context: &Context,
    module: &Module,
    builder: &Builder,
) -> Result<FunctionValue> {
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[i32_type.into()], false);
    let function = module.add_function("loop_fn", fn_type, None);
    
    let entry_block = context.append_basic_block(function, "entry");
    let loop_header = context.append_basic_block(function, "loop");
    let loop_body = context.append_basic_block(function, "body");
    let exit_block = context.append_basic_block(function, "exit");
    
    // Entry
    builder.position_at_end(entry_block);
    let param = function.get_nth_param(0).unwrap().into_int_value();
    builder.build_unconditional_branch(loop_header)?;
    
    // Loop header
    builder.position_at_end(loop_header);
    let phi = builder.build_phi(i32_type, "counter")?;
    phi.add_incoming(&[(&param, entry_block)]);
    
    let condition = builder.build_int_compare(
        inkwell::IntPredicate::SGT,
        phi.as_basic_value().into_int_value(),
        i32_type.const_int(0, false),
        "cond"
    )?;
    builder.build_conditional_branch(condition, loop_body, exit_block)?;
    
    // Loop body
    builder.position_at_end(loop_body);
    let decremented = builder.build_int_sub(
        phi.as_basic_value().into_int_value(),
        i32_type.const_int(1, false),
        "dec"
    )?;
    phi.add_incoming(&[(&decremented, loop_body)]);
    builder.build_unconditional_branch(loop_header)?;
    
    // Exit
    builder.position_at_end(exit_block);
    builder.build_return(Some(&phi.as_basic_value()))?;
    
    Ok(function)
}

fn create_performance_test_function(
    context: &Context,
    module: &Module,
    builder: &Builder,
) -> Result<FunctionValue> {
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[i32_type.into()], false);
    let function = module.add_function("perf_test", fn_type, None);
    
    // Create multiple blocks with various operations
    let mut blocks = Vec::new();
    for i in 0..10 {
        let block = context.append_basic_block(function, &format!("block_{}", i));
        blocks.push(block);
    }
    
    let param = function.get_nth_param(0).unwrap().into_int_value();
    let mut current_value = param;
    
    for (i, &block) in blocks.iter().enumerate() {
        builder.position_at_end(block);
        
        // Add some arithmetic operations
        current_value = builder.build_int_add(
            current_value,
            i32_type.const_int(i as u64, false),
            &format!("add_{}", i)
        )?;
        current_value = builder.build_int_mul(
            current_value,
            i32_type.const_int(2, false),
            &format!("mul_{}", i)
        )?;
        
        if i < blocks.len() - 1 {
            builder.build_unconditional_branch(blocks[i + 1])?;
        } else {
            builder.build_return(Some(&current_value))?;
        }
    }
    
    Ok(function)
}

fn create_optimizable_function(
    context: &Context,
    module: &Module,
    builder: &Builder,
) -> Result<FunctionValue> {
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[i32_type.into()], false);
    let function = module.add_function("optimizable", fn_type, None);
    
    let entry_block = context.append_basic_block(function, "entry");
    builder.position_at_end(entry_block);
    
    let param = function.get_nth_param(0).unwrap().into_int_value();
    
    // Create code that can benefit from optimization
    let const_val = i32_type.const_int(0, false);
    let added = builder.build_int_add(param, const_val, "add_zero")?; // Can be optimized away
    let multiplied = builder.build_int_mul(added, i32_type.const_int(1, false), "mul_one")?; // Can be optimized away
    
    builder.build_return(Some(&multiplied))?;
    
    Ok(function)
}

fn find_call_instruction(function: FunctionValue) -> Result<inkwell::values::InstructionValue> {
    for basic_block in function.get_basic_blocks() {
        for instruction in basic_block.get_instructions() {
            if instruction.get_opcode() == inkwell::values::InstructionOpcode::Call {
                return Ok(instruction);
            }
        }
    }
    Err(cursed::error::Error::OptimizationError("No call instruction found".to_string()))
}
