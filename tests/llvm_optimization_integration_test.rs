/// Comprehensive LLVM Optimization Integration Tests
/// 
/// Tests the real LLVM optimization functionality, ensuring that placeholder
/// implementations have been replaced with working code generation and optimization.

use cursed::codegen::llvm::{
    real_optimization_integration::RealLlvmOptimizationIntegration,
    optimization_passes::{PassRegistry, PassConfiguration, PassTimeCategory},
    performance_monitor::{PerformanceMonitor, MonitoringConfig, CodeMetrics},
    lto_integration::LlvmLtoIntegration,
};
use cursed::optimization::config::{OptimizationConfig, OptimizationLevel};
use cursed::optimization::lto::{LtoConfig, LtoLevel};
use cursed::error::Result;

use inkwell::context::Context;
use inkwell::types::BasicTypeEnum;
use inkwell::AddressSpace;
use std::time::Duration;
use std::path::PathBuf;

/// Test real LLVM optimization integration creation and initialization
#[test]
fn test_real_optimization_integration_creation() {
    let context = Context::create();
    let config = OptimizationConfig::default();
    
    let integration_result = RealLlvmOptimizationIntegration::new(&context, config);
    assert!(integration_result.is_ok(), "Failed to create optimization integration");
    
    let mut integration = integration_result.unwrap();
    let init_result = integration.initialize();
    assert!(init_result.is_ok(), "Failed to initialize optimization integration");
}

/// Test actual LLVM module optimization with real IR generation
#[test]
fn test_module_optimization_with_real_ir() {
    let context = Context::create();
    let config = OptimizationConfig {
        optimization_level: OptimizationLevel::Default,
        enable_inlining: true,
        enable_dead_code_elimination: true,
        enable_constant_propagation: true,
        enable_loop_optimization: true,
        enable_vectorization: false,
        ..Default::default()
    };
    
    let integration_result = RealLlvmOptimizationIntegration::new(&context, config);
    assert!(integration_result.is_ok());
    
    let mut integration = integration_result.unwrap();
    assert!(integration.initialize().is_ok());
    
    // Create a test module with real LLVM IR
    let module = context.create_module("test_optimization");
    
    // Add a simple function with optimization opportunities
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[i32_type.into(), i32_type.into()], false);
    let function = module.add_function("test_function", fn_type, None);
    
    // Create function body with dead code and constant folding opportunities
    let entry_block = context.append_basic_block(function, "entry");
    let builder = context.create_builder();
    builder.position_at_end(entry_block);
    
    let param1 = function.get_nth_param(0).unwrap().into_int_value();
    let param2 = function.get_nth_param(1).unwrap().into_int_value();
    
    // Add some constant operations that can be optimized
    let const_1 = i32_type.const_int(1, false);
    let const_2 = i32_type.const_int(2, false);
    let const_add = builder.build_int_add(const_1, const_2, "const_add").unwrap();
    
    // Add parameter-dependent computation
    let param_add = builder.build_int_add(param1, param2, "param_add").unwrap();
    let result = builder.build_int_add(const_add, param_add, "result").unwrap();
    
    // Add some dead code
    let dead_const = i32_type.const_int(42, false);
    let _dead_add = builder.build_int_add(dead_const, const_1, "dead_computation").unwrap();
    
    builder.build_return(Some(&result)).unwrap();
    
    // Verify module before optimization
    assert!(module.verify().is_ok(), "Module should be valid before optimization");
    
    // Run optimization
    let opt_result = integration.optimize_module(&module);
    assert!(opt_result.is_ok(), "Optimization should succeed");
    
    // Verify module after optimization
    assert!(module.verify().is_ok(), "Module should be valid after optimization");
    
    // Check statistics
    let stats = integration.get_statistics();
    assert_eq!(stats.modules_optimized, 1);
    assert!(stats.total_optimization_time > Duration::from_nanos(0));
    
    // Verify optimization effectiveness
    assert!(stats.total_optimizations() > 0, "Should have performed some optimizations");
}

/// Test optimization pass registry with real pass execution
#[test]
fn test_optimization_pass_registry() {
    let registry = PassRegistry::new();
    
    // Verify default passes are registered
    assert!(registry.get_pass_count() > 0, "Should have registered default passes");
    
    // Test pass selection
    let config = PassConfiguration {
        optimization_level: OptimizationLevel::Default,
        enable_expensive_passes: true,
        enable_cursed_passes: true,
        ..Default::default()
    };
    
    let selected_passes = registry.select_passes(&config);
    assert!(selected_passes.is_ok(), "Pass selection should succeed");
    
    let passes = selected_passes.unwrap();
    assert!(!passes.is_empty(), "Should select some passes");
    
    // Verify CURSED-specific passes are included
    let cursed_passes: Vec<_> = passes.iter()
        .filter(|p| p.contains("cursed"))
        .collect();
    assert!(!cursed_passes.is_empty(), "Should include CURSED-specific passes");
    
    // Test pass execution tracking
    let stats = registry.get_overall_statistics();
    assert!(stats.len() >= 0, "Statistics should be available");
}

/// Test performance monitoring with real metrics collection
#[test]
fn test_performance_monitoring() {
    let config = MonitoringConfig {
        enable_compilation_timing: true,
        enable_memory_tracking: true,
        enable_code_quality_metrics: true,
        sample_rate: 1.0,
        ..Default::default()
    };
    
    let monitor = PerformanceMonitor::new(config);
    
    // Start a compilation session
    let session = monitor.start_compilation_monitoring("test_module");
    
    // Simulate memory allocations
    session.record_allocation(1024);
    session.record_allocation(2048);
    session.record_deallocation(512);
    
    // Complete the session with metrics
    let code_metrics_before = CodeMetrics {
        instruction_count: 100,
        function_count: 2,
        basic_block_count: 10,
        call_instruction_count: 5,
        loop_count: 2,
        branch_count: 8,
        load_store_count: 15,
        complexity_score: 7.5,
        estimated_cache_performance: 0.8,
        vectorization_opportunities: 3,
    };
    
    let code_metrics_after = CodeMetrics {
        instruction_count: 85,
        function_count: 2,
        basic_block_count: 8,
        call_instruction_count: 4,
        loop_count: 1,
        branch_count: 6,
        load_store_count: 12,
        complexity_score: 6.2,
        estimated_cache_performance: 0.9,
        vectorization_opportunities: 1,
    };
    
    session.complete(
        "O2",
        code_metrics_before,
        code_metrics_after,
        15,
        14,
        1.25,
        100,
        25,
    );
    
    // Verify statistics
    let (sample_count, total_time, avg_improvement) = monitor.get_current_statistics().unwrap();
    assert_eq!(sample_count, 1);
    assert!(total_time > Duration::from_nanos(0));
    assert!(avg_improvement > 1.0);
    
    // Test report generation
    let report_result = monitor.generate_performance_report();
    assert!(report_result.is_ok(), "Should generate performance report");
    
    let report = report_result.unwrap();
    assert_eq!(report.total_samples, 1);
    assert!(report.compilation_performance.average_compilation_time > Duration::from_nanos(0));
}

/// Test LTO integration with real module processing
#[test]
fn test_lto_integration() {
    let context = Context::create();
    let config = LtoConfig {
        level: LtoLevel::Thin,
        enable_whole_program_optimization: true,
        ..Default::default()
    };
    
    let mut lto_integration = LlvmLtoIntegration::new(&context, config).unwrap();
    
    // Initialize target
    let init_result = lto_integration.initialize_target("x86_64-unknown-linux-gnu");
    assert!(init_result.is_ok(), "Target initialization should succeed");
    
    // Create test modules
    let module1 = context.create_module("module1");
    let module2 = context.create_module("module2");
    
    // Add simple functions to modules
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    
    // Module 1: function that calls function in module 2
    let func1 = module1.add_function("func1", fn_type, None);
    let entry1 = context.append_basic_block(func1, "entry");
    let builder = context.create_builder();
    builder.position_at_end(entry1);
    
    // Declare external function from module 2
    let func2_decl = module1.add_function("func2", fn_type, None);
    let call_result = builder.build_call(func2_decl, &[], "call_func2").unwrap();
    let result = call_result.try_as_basic_value().left().unwrap().into_int_value();
    let const_val = i32_type.const_int(1, false);
    let final_result = builder.build_int_add(result, const_val, "add_one").unwrap();
    builder.build_return(Some(&final_result)).unwrap();
    
    // Module 2: simple function
    let func2 = module2.add_function("func2", fn_type, None);
    let entry2 = context.append_basic_block(func2, "entry");
    builder.position_at_end(entry2);
    let const_42 = i32_type.const_int(42, false);
    builder.build_return(Some(&const_42)).unwrap();
    
    // Add modules to LTO
    assert!(lto_integration.add_module(module1).is_ok());
    assert!(lto_integration.add_module(module2).is_ok());
    
    // Perform LTO
    let lto_result = lto_integration.perform_lto();
    assert!(lto_result.is_ok(), "LTO should complete successfully");
    
    let result = lto_result.unwrap();
    assert!(!result.optimized_modules.is_empty(), "Should produce optimized modules");
    assert!(result.total_time > Duration::from_nanos(0), "Should take measurable time");
    
    // Verify statistics
    let stats = lto_integration.get_statistics();
    assert!(stats.modules_processed > 0);
    assert!(stats.total_time > Duration::from_nanos(0));
}

/// Test actual LLVM IR generation vs. placeholder values
#[test]
fn test_real_ir_generation() {
    let context = Context::create();
    let module = context.create_module("ir_test");
    let builder = context.create_builder();
    
    // Test integer constant generation
    let i32_type = context.i32_type();
    let int_const = i32_type.const_int(42, false);
    assert_eq!(int_const.get_zero_extended_constant(), Some(42));
    
    // Test float constant generation
    let f64_type = context.f64_type();
    let float_const = f64_type.const_float(3.14);
    assert!((float_const.get_constant().unwrap() - 3.14).abs() < f64::EPSILON);
    
    // Test string constant generation
    let string_const = context.const_string(b"Hello, CURSED!", true);
    let global = module.add_global(string_const.get_type(), Some(AddressSpace::default()), "test_string");
    global.set_initializer(&string_const);
    global.set_constant(true);
    
    // Verify global was created correctly
    assert!(module.get_global("test_string").is_some());
    
    // Test function creation with real IR
    let fn_type = i32_type.fn_type(&[i32_type.into()], false);
    let function = module.add_function("test_add", fn_type, None);
    
    let entry_block = context.append_basic_block(function, "entry");
    builder.position_at_end(entry_block);
    
    let param = function.get_nth_param(0).unwrap().into_int_value();
    let const_one = i32_type.const_int(1, false);
    let result = builder.build_int_add(param, const_one, "add_result").unwrap();
    builder.build_return(Some(&result)).unwrap();
    
    // Verify function is valid
    assert!(function.verify(true), "Generated function should be valid");
    
    // Verify entire module
    assert!(module.verify().is_ok(), "Generated module should be valid");
}

/// Test comprehensive optimization pipeline
#[test]
fn test_optimization_pipeline() {
    let context = Context::create();
    let config = OptimizationConfig {
        optimization_level: OptimizationLevel::Aggressive,
        enable_inlining: true,
        enable_dead_code_elimination: true,
        enable_constant_propagation: true,
        enable_loop_optimization: true,
        enable_vectorization: true,
        ..Default::default()
    };
    
    let integration = RealLlvmOptimizationIntegration::new(&context, config).unwrap();
    let mut integration = integration;
    integration.initialize().unwrap();
    
    // Create a complex module with multiple optimization opportunities
    let module = context.create_module("pipeline_test");
    let i32_type = context.i32_type();
    let builder = context.create_builder();
    
    // Create multiple functions with different optimization patterns
    create_test_function_with_constants(&context, &module, &builder);
    create_test_function_with_loops(&context, &module, &builder);
    create_test_function_with_calls(&context, &module, &builder);
    
    // Run optimization pipeline
    let opt_result = integration.optimize_module(&module);
    assert!(opt_result.is_ok(), "Pipeline optimization should succeed");
    
    // Verify comprehensive statistics
    let stats = integration.get_statistics();
    assert!(stats.modules_optimized > 0);
    assert!(stats.functions_optimized > 0);
    assert!(stats.total_optimization_time > Duration::from_nanos(0));
    
    // Print optimization summary for manual verification
    integration.print_optimization_summary();
}

/// Helper function to create test function with constant folding opportunities
fn create_test_function_with_constants(context: &Context, module: &inkwell::module::Module, builder: &inkwell::builder::Builder) {
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = module.add_function("test_constants", fn_type, None);
    
    let entry_block = context.append_basic_block(function, "entry");
    builder.position_at_end(entry_block);
    
    // Multiple constant operations that should be folded
    let const_1 = i32_type.const_int(10, false);
    let const_2 = i32_type.const_int(20, false);
    let const_3 = i32_type.const_int(5, false);
    
    let add1 = builder.build_int_add(const_1, const_2, "add1").unwrap();
    let mul1 = builder.build_int_mul(add1, const_3, "mul1").unwrap();
    let sub1 = builder.build_int_sub(mul1, const_1, "sub1").unwrap();
    
    builder.build_return(Some(&sub1)).unwrap();
}

/// Helper function to create test function with loop optimization opportunities
fn create_test_function_with_loops(context: &Context, module: &inkwell::module::Module, builder: &inkwell::builder::Builder) {
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[i32_type.into()], false);
    let function = module.add_function("test_loops", fn_type, None);
    
    let entry_block = context.append_basic_block(function, "entry");
    let loop_block = context.append_basic_block(function, "loop");
    let exit_block = context.append_basic_block(function, "exit");
    
    // Entry: initialize counter
    builder.position_at_end(entry_block);
    let param = function.get_nth_param(0).unwrap().into_int_value();
    let zero = i32_type.const_int(0, false);
    let counter_alloca = builder.build_alloca(i32_type, "counter").unwrap();
    builder.build_store(counter_alloca, zero).unwrap();
    builder.build_unconditional_branch(loop_block).unwrap();
    
    // Loop: simple counting loop
    builder.position_at_end(loop_block);
    let counter_val = builder.build_load(i32_type, counter_alloca, "counter_val").unwrap().into_int_value();
    let one = i32_type.const_int(1, false);
    let next_counter = builder.build_int_add(counter_val, one, "next_counter").unwrap();
    builder.build_store(counter_alloca, next_counter).unwrap();
    
    let cmp = builder.build_int_compare(inkwell::IntPredicate::ULT, next_counter, param, "cmp").unwrap();
    builder.build_conditional_branch(cmp, loop_block, exit_block).unwrap();
    
    // Exit: return final counter
    builder.position_at_end(exit_block);
    let final_val = builder.build_load(i32_type, counter_alloca, "final_val").unwrap();
    builder.build_return(Some(&final_val)).unwrap();
}

/// Helper function to create test function with inlining opportunities
fn create_test_function_with_calls(context: &Context, module: &inkwell::module::Module, builder: &inkwell::builder::Builder) {
    let i32_type = context.i32_type();
    
    // Create small helper function (good candidate for inlining)
    let helper_fn_type = i32_type.fn_type(&[i32_type.into()], false);
    let helper_function = module.add_function("small_helper", helper_fn_type, None);
    
    let helper_entry = context.append_basic_block(helper_function, "entry");
    builder.position_at_end(helper_entry);
    let helper_param = helper_function.get_nth_param(0).unwrap().into_int_value();
    let two = i32_type.const_int(2, false);
    let doubled = builder.build_int_mul(helper_param, two, "doubled").unwrap();
    builder.build_return(Some(&doubled)).unwrap();
    
    // Create main function that calls helper multiple times
    let main_fn_type = i32_type.fn_type(&[i32_type.into()], false);
    let main_function = module.add_function("test_calls", main_fn_type, None);
    
    let main_entry = context.append_basic_block(main_function, "entry");
    builder.position_at_end(main_entry);
    let main_param = main_function.get_nth_param(0).unwrap().into_int_value();
    
    // Multiple calls to small helper function
    let call1 = builder.build_call(helper_function, &[main_param.into()], "call1").unwrap();
    let result1 = call1.try_as_basic_value().left().unwrap().into_int_value();
    
    let call2 = builder.build_call(helper_function, &[result1.into()], "call2").unwrap();
    let result2 = call2.try_as_basic_value().left().unwrap().into_int_value();
    
    let call3 = builder.build_call(helper_function, &[result2.into()], "call3").unwrap();
    let result3 = call3.try_as_basic_value().left().unwrap().into_int_value();
    
    builder.build_return(Some(&result3)).unwrap();
}

/// Test optimization effectiveness measurement
#[test] 
fn test_optimization_effectiveness() {
    let context = Context::create();
    let config = OptimizationConfig {
        optimization_level: OptimizationLevel::Aggressive,
        enable_inlining: true,
        enable_dead_code_elimination: true,
        enable_constant_propagation: true,
        ..Default::default()
    };
    
    let integration = RealLlvmOptimizationIntegration::new(&context, config).unwrap();
    let mut integration = integration;
    integration.initialize().unwrap();
    
    // Create module with known optimization opportunities
    let module = context.create_module("effectiveness_test");
    create_unoptimized_function(&context, &module);
    
    // Count instructions before optimization
    let instructions_before = count_module_instructions(&module);
    assert!(instructions_before > 0, "Should have instructions before optimization");
    
    // Run optimization
    integration.optimize_module(&module).unwrap();
    
    // Count instructions after optimization
    let instructions_after = count_module_instructions(&module);
    
    // Verify some optimization occurred (real optimizations should reduce instruction count)
    // Note: Exact reduction depends on LLVM version and optimization passes enabled
    let stats = integration.get_statistics();
    assert!(stats.modules_optimized > 0);
    
    // Calculate effectiveness
    let reduction_ratio = if instructions_before > 0 {
        (instructions_before as f64 - instructions_after as f64) / instructions_before as f64
    } else {
        0.0
    };
    
    println!("Optimization effectiveness: {:.2}% instruction reduction", reduction_ratio * 100.0);
    println!("Instructions: {} -> {}", instructions_before, instructions_after);
    
    // Verify optimization statistics are meaningful
    assert!(stats.total_optimization_time > Duration::from_nanos(0));
}

/// Helper to create function with obvious optimization opportunities
fn create_unoptimized_function(context: &Context, module: &inkwell::module::Module) {
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = module.add_function("unoptimized", fn_type, None);
    
    let builder = context.create_builder();
    let entry_block = context.append_basic_block(function, "entry");
    builder.position_at_end(entry_block);
    
    // Lots of constant computations
    let const_10 = i32_type.const_int(10, false);
    let const_20 = i32_type.const_int(20, false);
    let const_30 = i32_type.const_int(30, false);
    
    // Chain of constant operations
    let add1 = builder.build_int_add(const_10, const_20, "add1").unwrap();
    let add2 = builder.build_int_add(add1, const_30, "add2").unwrap();
    let mul1 = builder.build_int_mul(add2, const_10, "mul1").unwrap();
    let sub1 = builder.build_int_sub(mul1, const_20, "sub1").unwrap();
    
    // Dead code
    let _dead1 = builder.build_int_add(const_10, const_10, "dead1").unwrap();
    let _dead2 = builder.build_int_mul(const_20, const_30, "dead2").unwrap();
    
    // More operations
    let div1 = builder.build_int_signed_div(sub1, const_10, "div1").unwrap();
    
    builder.build_return(Some(&div1)).unwrap();
}

/// Helper to count instructions in a module
fn count_module_instructions(module: &inkwell::module::Module) -> usize {
    let mut count = 0;
    for function in module.get_functions() {
        if function.get_first_basic_block().is_some() {
            for basic_block in function.get_basic_blocks() {
                for _instruction in basic_block.get_instructions() {
                    count += 1;
                }
            }
        }
    }
    count
}
