/// Advanced LLVM Optimization Integration Tests
/// 
/// Tests the comprehensive LLVM optimization passes implemented for the CURSED compiler.
/// These tests validate real optimization functionality and measure performance improvements.

use cursed::optimization::advanced_llvm_integration::{
    AdvancedLlvmIntegration, AdvancedLlvmConfig, AdvancedOptimizationStatistics,
};
use cursed::error::Result;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{FunctionValue, BasicValueEnum};
use inkwell::types::BasicTypeEnum;
use std::time::{Duration, Instant};
use tracing::{info, debug};

/// Initialize tracing for tests
fn init_test_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter("debug")
        .with_test_writer()
        .try_init()
        .ok(); // Ignore if already initialized
}

/// Test basic advanced LLVM integration creation and configuration
#[test]
fn test_advanced_llvm_integration_creation() {
    init_test_tracing();
    
    let context = Context::create();
    let config = AdvancedLlvmConfig {
        enable_advanced_inlining: true,
        enable_cfg_transformations: true,
        enable_target_specific: true,
        enable_vectorization: true,
        enable_advanced_loops: true,
        enable_ipo: true,
        inline_threshold: 150,
        max_inline_size: 800,
        max_inline_depth: 10,
        target_cpu: "x86-64".to_string(),
        target_features: "+sse4.2,+avx2,+fma".to_string(),
        optimization_level: 3,
    };
    
    let result = AdvancedLlvmIntegration::new(&context, "optimization_test_module", config);
    assert!(result.is_ok());
    
    let integration = result.unwrap();
    let module = integration.get_module();
    assert_eq!(module.get_name().to_str().unwrap(), "optimization_test_module");
    
    info!("✓ Advanced LLVM integration created successfully");
}

/// Test target-specific optimization features
#[test]
fn test_target_specific_optimizations() {
    init_test_tracing();
    
    let context = Context::create();
    let mut config = AdvancedLlvmConfig::default();
    config.enable_target_specific = true;
    config.target_features = "+avx2,+fma,+prefetch".to_string();
    
    let mut integration = AdvancedLlvmIntegration::new(&context, "target_test", config).unwrap();
    
    // Create a simple function for optimization
    let module = integration.get_module();
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[i32_type.into(), i32_type.into()], false);
    let function = module.add_function("target_optimized_function", fn_type, None);
    
    // Create function body with optimization opportunities
    let entry_block = context.append_basic_block(function, "entry");
    let builder = context.create_builder();
    builder.position_at_end(entry_block);
    
    let param1 = function.get_nth_param(0).unwrap().into_int_value();
    let param2 = function.get_nth_param(1).unwrap().into_int_value();
    
    // Create operations that can benefit from target-specific optimization
    let add_result = builder.build_int_add(param1, param2, "add").unwrap();
    let mul_result = builder.build_int_mul(add_result, param1, "mul").unwrap();
    let final_result = builder.build_int_sub(mul_result, param2, "sub").unwrap();
    
    builder.build_return(Some(&final_result)).unwrap();
    
    // Test the optimization
    integration.initialize_passes().unwrap();
    let stats = integration.optimize_module().unwrap();
    
    // Verify target-specific optimizations were applied
    assert!(stats.target_stats.target_instructions_used > 0 || 
            stats.target_stats.cache_optimizations_applied > 0);
    
    info!("✓ Target-specific optimizations applied successfully");
    debug!("Target optimization stats: {:?}", stats.target_stats);
}

/// Test advanced loop optimization passes
#[test]
fn test_loop_optimizations() {
    init_test_tracing();
    
    let context = Context::create();
    let mut config = AdvancedLlvmConfig::default();
    config.enable_advanced_loops = true;
    config.enable_vectorization = true;
    
    let mut integration = AdvancedLlvmIntegration::new(&context, "loop_test", config).unwrap();
    
    // Create a function with a loop structure
    let module = integration.get_module();
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[i32_type.into()], false);
    let function = module.add_function("loop_function", fn_type, None);
    
    let entry_block = context.append_basic_block(function, "entry");
    let loop_header = context.append_basic_block(function, "loop_header");
    let loop_body = context.append_basic_block(function, "loop_body");
    let exit_block = context.append_basic_block(function, "exit");
    
    let builder = context.create_builder();
    
    // Entry block
    builder.position_at_end(entry_block);
    let param = function.get_nth_param(0).unwrap().into_int_value();
    let zero = i32_type.const_int(0, false);
    let one = i32_type.const_int(1, false);
    builder.build_unconditional_branch(loop_header).unwrap();
    
    // Loop header with phi node
    builder.position_at_end(loop_header);
    let phi = builder.build_phi(i32_type, "loop_counter").unwrap();
    phi.add_incoming(&[(&zero, entry_block)]);
    
    let condition = builder.build_int_compare(
        inkwell::IntPredicate::SLT,
        phi.as_basic_value().into_int_value(),
        param,
        "loop_condition"
    ).unwrap();
    
    builder.build_conditional_branch(condition, loop_body, exit_block).unwrap();
    
    // Loop body with computation
    builder.position_at_end(loop_body);
    let incremented = builder.build_int_add(
        phi.as_basic_value().into_int_value(),
        one,
        "increment"
    ).unwrap();
    
    // Add phi incoming edge from loop body
    phi.add_incoming(&[(&incremented, loop_body)]);
    builder.build_unconditional_branch(loop_header).unwrap();
    
    // Exit block
    builder.position_at_end(exit_block);
    builder.build_return(Some(&phi.as_basic_value())).unwrap();
    
    // Test loop optimizations
    integration.initialize_passes().unwrap();
    let stats = integration.optimize_module().unwrap();
    
    // Verify loop optimizations were considered
    assert!(stats.loop_stats.loops_analyzed > 0 ||
            stats.vectorization_stats.vectorizable_loops > 0);
    
    info!("✓ Loop optimizations analyzed and applied");
    debug!("Loop optimization stats: {:?}", stats.loop_stats);
    debug!("Vectorization stats: {:?}", stats.vectorization_stats);
}

/// Test function inlining with multi-block support
#[test]
fn test_advanced_function_inlining() {
    init_test_tracing();
    
    let context = Context::create();
    let mut config = AdvancedLlvmConfig::default();
    config.enable_advanced_inlining = true;
    config.inline_threshold = 50;
    config.max_inline_size = 200;
    
    let mut integration = AdvancedLlvmIntegration::new(&context, "inline_test", config).unwrap();
    
    let module = integration.get_module();
    let i32_type = context.i32_type();
    
    // Create a small function suitable for inlining
    let small_fn_type = i32_type.fn_type(&[i32_type.into()], false);
    let small_function = module.add_function("small_function", small_fn_type, None);
    
    let small_entry = context.append_basic_block(small_function, "entry");
    let builder = context.create_builder();
    builder.position_at_end(small_entry);
    
    let param = small_function.get_nth_param(0).unwrap().into_int_value();
    let two = i32_type.const_int(2, false);
    let doubled = builder.build_int_mul(param, two, "doubled").unwrap();
    builder.build_return(Some(&doubled)).unwrap();
    
    // Create a caller function
    let caller_fn_type = i32_type.fn_type(&[i32_type.into()], false);
    let caller_function = module.add_function("caller_function", caller_fn_type, None);
    
    let caller_entry = context.append_basic_block(caller_function, "entry");
    builder.position_at_end(caller_entry);
    
    let caller_param = caller_function.get_nth_param(0).unwrap().into_int_value();
    let call_result = builder.build_call(
        small_function,
        &[caller_param.into()],
        "call_small"
    ).unwrap();
    
    let result_value = call_result.try_as_basic_value().left().unwrap();
    builder.build_return(Some(&result_value)).unwrap();
    
    // Test inlining optimization
    integration.initialize_passes().unwrap();
    let stats = integration.optimize_module().unwrap();
    
    // Verify inlining was analyzed
    assert!(stats.inlining_stats.functions_analyzed > 0);
    
    info!("✓ Function inlining analysis completed");
    debug!("Inlining stats: {:?}", stats.inlining_stats);
}

/// Test CFG transformations and basic block optimizations
#[test] 
fn test_cfg_transformations() {
    init_test_tracing();
    
    let context = Context::create();
    let mut config = AdvancedLlvmConfig::default();
    config.enable_cfg_transformations = true;
    
    let mut integration = AdvancedLlvmIntegration::new(&context, "cfg_test", config).unwrap();
    
    let module = integration.get_module();
    let i32_type = context.i32_type();
    let void_type = context.void_type();
    let fn_type = void_type.fn_type(&[i32_type.into()], false);
    let function = module.add_function("cfg_function", fn_type, None);
    
    // Create a CFG with optimization opportunities
    let entry_block = context.append_basic_block(function, "entry");
    let then_block = context.append_basic_block(function, "then");
    let else_block = context.append_basic_block(function, "else");
    let merge_block = context.append_basic_block(function, "merge");
    let dead_block = context.append_basic_block(function, "dead"); // This should be eliminated
    
    let builder = context.create_builder();
    
    // Entry block with conditional branch
    builder.position_at_end(entry_block);
    let param = function.get_nth_param(0).unwrap().into_int_value();
    let zero = i32_type.const_int(0, false);
    let condition = builder.build_int_compare(
        inkwell::IntPredicate::SGT,
        param,
        zero,
        "is_positive"
    ).unwrap();
    builder.build_conditional_branch(condition, then_block, else_block).unwrap();
    
    // Then block (can be merged with merge block)
    builder.position_at_end(then_block);
    builder.build_unconditional_branch(merge_block).unwrap();
    
    // Else block (can be merged with merge block)
    builder.position_at_end(else_block);
    builder.build_unconditional_branch(merge_block).unwrap();
    
    // Merge block
    builder.position_at_end(merge_block);
    builder.build_return(None).unwrap();
    
    // Dead block (unreachable)
    builder.position_at_end(dead_block);
    builder.build_return(None).unwrap();
    
    // Test CFG optimizations
    integration.initialize_passes().unwrap();
    let stats = integration.optimize_module().unwrap();
    
    // Verify CFG transformations were applied
    assert!(stats.cfg_stats.blocks_merged > 0 || 
            stats.cfg_stats.dead_blocks_removed > 0 ||
            stats.cfg_stats.branches_simplified > 0);
    
    info!("✓ CFG transformations applied successfully");
    debug!("CFG transformation stats: {:?}", stats.cfg_stats);
}

/// Test memory access optimization and cache efficiency
#[test]
fn test_memory_access_optimizations() {
    init_test_tracing();
    
    let context = Context::create();
    let mut config = AdvancedLlvmConfig::default();
    config.enable_target_specific = true;
    config.target_features = "+prefetch".to_string();
    
    let mut integration = AdvancedLlvmIntegration::new(&context, "memory_test", config).unwrap();
    
    let module = integration.get_module();
    let i32_type = context.i32_type();
    let ptr_type = i32_type.ptr_type(inkwell::AddressSpace::default());
    let fn_type = i32_type.fn_type(&[ptr_type.into(), i32_type.into()], false);
    let function = module.add_function("memory_function", fn_type, None);
    
    let entry_block = context.append_basic_block(function, "entry");
    let builder = context.create_builder();
    builder.position_at_end(entry_block);
    
    let array_ptr = function.get_nth_param(0).unwrap().into_pointer_value();
    let size = function.get_nth_param(1).unwrap().into_int_value();
    
    // Create memory access patterns that can be optimized
    let zero = i32_type.const_int(0, false);
    let one = i32_type.const_int(1, false);
    let two = i32_type.const_int(2, false);
    
    // Sequential memory accesses (good for spatial locality optimization)
    let ptr0 = unsafe { builder.build_gep(i32_type, array_ptr, &[zero], "ptr0").unwrap() };
    let ptr1 = unsafe { builder.build_gep(i32_type, array_ptr, &[one], "ptr1").unwrap() };
    let ptr2 = unsafe { builder.build_gep(i32_type, array_ptr, &[two], "ptr2").unwrap() };
    
    let val0 = builder.build_load(i32_type, ptr0, "val0").unwrap();
    let val1 = builder.build_load(i32_type, ptr1, "val1").unwrap();
    let val2 = builder.build_load(i32_type, ptr2, "val2").unwrap();
    
    // Computation that can benefit from cache optimization
    let sum1 = builder.build_int_add(val0.into_int_value(), val1.into_int_value(), "sum1").unwrap();
    let sum2 = builder.build_int_add(sum1, val2.into_int_value(), "sum2").unwrap();
    
    builder.build_return(Some(&sum2)).unwrap();
    
    // Test memory optimizations
    integration.initialize_passes().unwrap();
    let stats = integration.optimize_module().unwrap();
    
    // Verify memory optimizations were considered
    assert!(stats.target_stats.cache_optimizations_applied > 0 || 
            stats.target_stats.memory_layout_optimizations > 0);
    
    info!("✓ Memory access optimizations applied");
    debug!("Memory optimization stats: {:?}", stats.target_stats);
}

/// Test comprehensive optimization pipeline performance
#[test]
fn test_optimization_pipeline_performance() {
    init_test_tracing();
    
    let context = Context::create();
    let config = AdvancedLlvmConfig {
        enable_advanced_inlining: true,
        enable_cfg_transformations: true,
        enable_target_specific: true,
        enable_vectorization: true,
        enable_advanced_loops: true,
        enable_ipo: true,
        inline_threshold: 100,
        max_inline_size: 500,
        max_inline_depth: 8,
        target_cpu: "x86-64".to_string(),
        target_features: "+sse4.2,+avx2,+fma,+prefetch".to_string(),
        optimization_level: 3,
    };
    
    let mut integration = AdvancedLlvmIntegration::new(&context, "performance_test", config).unwrap();
    
    // Create a more complex function for comprehensive testing
    let module = integration.get_module();
    let i32_type = context.i32_type();
    let f64_type = context.f64_type();
    let ptr_type = f64_type.ptr_type(inkwell::AddressSpace::default());
    
    let fn_type = f64_type.fn_type(&[
        ptr_type.into(),
        ptr_type.into(), 
        i32_type.into()
    ], false);
    let function = module.add_function("complex_computation", fn_type, None);
    
    // Create a function that exercises multiple optimization opportunities
    let entry_block = context.append_basic_block(function, "entry");
    let loop_header = context.append_basic_block(function, "loop_header");
    let loop_body = context.append_basic_block(function, "loop_body");
    let exit_block = context.append_basic_block(function, "exit");
    
    let builder = context.create_builder();
    
    // Entry block
    builder.position_at_end(entry_block);
    let input_ptr = function.get_nth_param(0).unwrap().into_pointer_value();
    let output_ptr = function.get_nth_param(1).unwrap().into_pointer_value();
    let size = function.get_nth_param(2).unwrap().into_int_value();
    
    let zero_i = i32_type.const_int(0, false);
    let one_i = i32_type.const_int(1, false);
    let zero_f = f64_type.const_float(0.0);
    
    builder.build_unconditional_branch(loop_header).unwrap();
    
    // Loop header
    builder.position_at_end(loop_header);
    let i_phi = builder.build_phi(i32_type, "i").unwrap();
    let sum_phi = builder.build_phi(f64_type, "sum").unwrap();
    
    i_phi.add_incoming(&[(&zero_i, entry_block)]);
    sum_phi.add_incoming(&[(&zero_f, entry_block)]);
    
    let loop_condition = builder.build_int_compare(
        inkwell::IntPredicate::SLT,
        i_phi.as_basic_value().into_int_value(),
        size,
        "loop_condition"
    ).unwrap();
    
    builder.build_conditional_branch(loop_condition, loop_body, exit_block).unwrap();
    
    // Loop body with vectorizable operations
    builder.position_at_end(loop_body);
    let i_val = i_phi.as_basic_value().into_int_value();
    let sum_val = sum_phi.as_basic_value().into_float_value();
    
    // Memory operations with good access patterns
    let input_gep = unsafe { builder.build_gep(f64_type, input_ptr, &[i_val], "input_gep").unwrap() };
    let output_gep = unsafe { builder.build_gep(f64_type, output_ptr, &[i_val], "output_gep").unwrap() };
    
    let input_val = builder.build_load(f64_type, input_gep, "input_val").unwrap().into_float_value();
    
    // Floating point operations that can use FMA
    let two_f = f64_type.const_float(2.0);
    let multiplied = builder.build_float_mul(input_val, two_f, "multiplied").unwrap();
    let new_sum = builder.build_float_add(sum_val, multiplied, "new_sum").unwrap();
    
    // Store result
    builder.build_store(output_gep, multiplied).unwrap();
    
    // Update phi nodes
    let i_next = builder.build_int_add(i_val, one_i, "i_next").unwrap();
    i_phi.add_incoming(&[(&i_next, loop_body)]);
    sum_phi.add_incoming(&[(&new_sum, loop_body)]);
    
    builder.build_unconditional_branch(loop_header).unwrap();
    
    // Exit block
    builder.position_at_end(exit_block);
    builder.build_return(Some(&sum_phi.as_basic_value())).unwrap();
    
    // Measure optimization performance
    let start_time = Instant::now();
    
    integration.initialize_passes().unwrap();
    let stats = integration.optimize_module().unwrap();
    
    let optimization_time = start_time.elapsed();
    
    // Verify comprehensive optimizations
    assert!(stats.inlining_stats.functions_analyzed > 0);
    assert!(stats.cfg_stats.blocks_merged > 0 || stats.cfg_stats.branches_simplified > 0);
    assert!(stats.loop_stats.loops_analyzed > 0);
    assert!(stats.total_optimization_time > Duration::from_millis(0));
    
    // Performance requirements
    assert!(optimization_time < Duration::from_secs(5), 
            "Optimization took too long: {:?}", optimization_time);
    
    info!("✓ Comprehensive optimization pipeline completed successfully");
    info!("Optimization took: {:?}", optimization_time);
    debug!("Final optimization statistics: {:#?}", stats);
    
    // Log performance summary
    info!("🔧 Performance Summary:");
    info!("   Functions analyzed: {}", stats.inlining_stats.functions_analyzed);
    info!("   Functions inlined: {}", stats.inlining_stats.functions_inlined);
    info!("   Blocks merged: {}", stats.cfg_stats.blocks_merged);
    info!("   Dead blocks removed: {}", stats.cfg_stats.dead_blocks_removed);
    info!("   Loops analyzed: {}", stats.loop_stats.loops_analyzed);
    info!("   Vectorizable loops: {}", stats.vectorization_stats.vectorizable_loops);
    info!("   Target optimizations: {}", stats.target_stats.target_instructions_used);
    info!("   Total optimization time: {:?}", stats.total_optimization_time);
}

/// Test optimization statistics and reporting
#[test]
fn test_optimization_statistics() {
    init_test_tracing();
    
    let context = Context::create();
    let config = AdvancedLlvmConfig::default();
    let mut integration = AdvancedLlvmIntegration::new(&context, "stats_test", config).unwrap();
    
    // Get initial statistics
    let initial_stats = integration.get_statistics();
    assert_eq!(initial_stats.inlining_stats.functions_analyzed, 0);
    assert_eq!(initial_stats.cfg_stats.blocks_merged, 0);
    assert_eq!(initial_stats.loop_stats.loops_analyzed, 0);
    
    // Create a simple function and optimize
    let module = integration.get_module();
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = module.add_function("simple_function", fn_type, None);
    
    let entry_block = context.append_basic_block(function, "entry");
    let builder = context.create_builder();
    builder.position_at_end(entry_block);
    
    let return_value = i32_type.const_int(42, false);
    builder.build_return(Some(&return_value)).unwrap();
    
    // Run optimization
    integration.initialize_passes().unwrap();
    let final_stats = integration.optimize_module().unwrap();
    
    // Verify statistics were updated
    assert!(final_stats.total_optimization_time > Duration::from_millis(0));
    assert!(final_stats.peak_memory_usage_mb > 0);
    
    info!("✓ Optimization statistics collection working correctly");
    debug!("Final statistics: {:#?}", final_stats);
}
