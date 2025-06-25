/// Tests for real LLVM optimization passes
/// 
/// This module tests the actual LLVM IR transformations and optimizations
/// implemented in the real optimization passes.

use cursed::optimization::real_llvm_passes::{RealLlvmPassManager, OptimizationStatistics};
use cursed::optimization::config::{OptimizationConfig, OptimizationLevel};
use cursed::codegen::llvm::real_optimization_integration::{RealLlvmOptimizationIntegration, IntegrationStatistics};
use inkwell::context::Context;
use std::time::Duration;

#[path = "common/mod.rs"]
mod common;

/// Test real LLVM pass manager creation and basic functionality
#[test]
fn test_real_pass_manager_creation() {
    common::tracing::setup();
    
    let context = Context::create();
    let manager = RealLlvmPassManager::new(&context, OptimizationLevel::Default);
    
    // Verify manager was created successfully
    let stats = manager.get_statistics();
    assert_eq!(stats.total_optimizations(), 0);
    assert_eq!(stats.initial_functions, 0);
    assert_eq!(stats.final_functions, 0);
}

/// Test optimization statistics functionality
#[test]
fn test_optimization_statistics() {
    common::tracing::setup();
    
    let mut stats = OptimizationStatistics::default();
    
    // Test initial state
    assert_eq!(stats.total_optimizations(), 0);
    assert_eq!(stats.instructions_saved(), 0);
    assert_eq!(stats.blocks_saved(), 0);
    
    // Add some statistics
    stats.initial_instructions = 100;
    stats.final_instructions = 80;
    stats.initial_basic_blocks = 20;
    stats.final_basic_blocks = 18;
    stats.functions_inlined = 5;
    stats.instructions_eliminated = 15;
    stats.constants_propagated = 10;
    
    // Test calculated values
    assert_eq!(stats.instructions_saved(), 20);
    assert_eq!(stats.blocks_saved(), 2);
    assert_eq!(stats.total_optimizations(), 30); // 5 + 15 + 0 + 10 + 0 + 0
}

/// Test real optimization integration creation
#[test]
fn test_real_optimization_integration_creation() {
    common::tracing::setup();
    
    let context = Context::create();
    let config = OptimizationConfig::default();
    
    let integration = RealLlvmOptimizationIntegration::new(&context, config);
    assert!(integration.is_ok());
    
    let integration = integration.unwrap();
    let stats = integration.get_statistics();
    assert_eq!(stats.total_optimizations(), 0);
}

/// Test optimization integration initialization
#[test]
fn test_optimization_integration_initialization() {
    common::tracing::setup();
    
    let context = Context::create();
    let config = OptimizationConfig::default();
    
    let mut integration = RealLlvmOptimizationIntegration::new(&context, config).unwrap();
    let result = integration.initialize();
    
    assert!(result.is_ok());
}

/// Test module optimization with empty module
#[test]
fn test_empty_module_optimization() {
    common::tracing::setup();
    
    let context = Context::create();
    let config = OptimizationConfig::default();
    
    let mut integration = RealLlvmOptimizationIntegration::new(&context, config).unwrap();
    integration.initialize().unwrap();
    
    let module = context.create_module("test_empty");
    let result = integration.optimize_module(&module);
    
    assert!(result.is_ok());
    
    let stats = integration.get_statistics();
    assert_eq!(stats.modules_optimized, 1);
}

/// Test module optimization with simple function
#[test]
fn test_simple_function_optimization() {
    common::tracing::setup();
    
    let context = Context::create();
    let config = OptimizationConfig::default();
    
    let mut integration = RealLlvmOptimizationIntegration::new(&context, config).unwrap();
    integration.initialize().unwrap();
    
    // Create a simple module with a function
    let module = context.create_module("test_simple");
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = module.add_function("test_function", fn_type, None);
    
    // Add a basic block with return
    let basic_block = context.append_basic_block(function, "entry");
    let builder = context.create_builder();
    builder.position_at_end(basic_block);
    let return_value = i32_type.const_int(42, false);
    builder.build_return(Some(&return_value)).unwrap();
    
    // Optimize the module
    let result = integration.optimize_module(&module);
    assert!(result.is_ok());
    
    let stats = integration.get_statistics();
    assert_eq!(stats.modules_optimized, 1);
}

/// Test different optimization levels
#[test]
fn test_optimization_levels() {
    common::tracing::setup();
    
    let context = Context::create();
    
    let levels = vec![
        OptimizationLevel::None,
        OptimizationLevel::Less,
        OptimizationLevel::Default,
        OptimizationLevel::Aggressive,
        OptimizationLevel::Size,
        OptimizationLevel::SizeAggressive,
    ];
    
    for level in levels {
        let config = OptimizationConfig {
            optimization_level: level.clone(),
            ..Default::default()
        };
        
        let mut integration = RealLlvmOptimizationIntegration::new(&context, config).unwrap();
        let result = integration.initialize();
        assert!(result.is_ok(), "Failed to initialize optimization level: {:?}", level);
    }
}

/// Test optimization with simple arithmetic
#[test]
fn test_arithmetic_optimization() {
    common::tracing::setup();
    
    let context = Context::create();
    let config = OptimizationConfig {
        optimization_level: OptimizationLevel::Default,
        ..Default::default()
    };
    
    let mut integration = RealLlvmOptimizationIntegration::new(&context, config).unwrap();
    integration.initialize().unwrap();
    
    // Create module with arithmetic operations
    let module = context.create_module("test_arithmetic");
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = module.add_function("arithmetic_test", fn_type, None);
    
    let basic_block = context.append_basic_block(function, "entry");
    let builder = context.create_builder();
    builder.position_at_end(basic_block);
    
    // Create some arithmetic that can be optimized
    let const_1 = i32_type.const_int(10, false);
    let const_2 = i32_type.const_int(5, false);
    let add_result = builder.build_int_add(const_1, const_2, "add").unwrap();
    let const_3 = i32_type.const_int(3, false);
    let mul_result = builder.build_int_mul(add_result, const_3, "mul").unwrap();
    
    builder.build_return(Some(&mul_result)).unwrap();
    
    // Optimize the module
    let result = integration.optimize_module(&module);
    assert!(result.is_ok());
    
    let stats = integration.get_statistics();
    assert_eq!(stats.modules_optimized, 1);
}

/// Test function inlining optimization
#[test]
fn test_function_inlining() {
    common::tracing::setup();
    
    let context = Context::create();
    let config = OptimizationConfig {
        optimization_level: OptimizationLevel::Aggressive,
        ..Default::default()
    };
    
    let mut integration = RealLlvmOptimizationIntegration::new(&context, config).unwrap();
    integration.initialize().unwrap();
    
    // Create module with function to inline
    let module = context.create_module("test_inlining");
    let i32_type = context.i32_type();
    
    // Small function that should be inlined
    let small_fn_type = i32_type.fn_type(&[i32_type.into()], false);
    let small_function = module.add_function("small_function", small_fn_type, None);
    let small_block = context.append_basic_block(small_function, "entry");
    let builder = context.create_builder();
    builder.position_at_end(small_block);
    let param = small_function.get_nth_param(0).unwrap().into_int_value();
    let const_2 = i32_type.const_int(2, false);
    let result = builder.build_int_mul(param, const_2, "double").unwrap();
    builder.build_return(Some(&result)).unwrap();
    
    // Main function that calls the small function
    let main_fn_type = i32_type.fn_type(&[], false);
    let main_function = module.add_function("main_function", main_fn_type, None);
    let main_block = context.append_basic_block(main_function, "entry");
    builder.position_at_end(main_block);
    let const_5 = i32_type.const_int(5, false);
    let call_result = builder.build_call(small_function, &[const_5.into()], "call").unwrap();
    let call_value = call_result.try_as_basic_value().left().unwrap();
    builder.build_return(Some(&call_value)).unwrap();
    
    // Optimize the module
    let result = integration.optimize_module(&module);
    assert!(result.is_ok());
    
    let stats = integration.get_statistics();
    assert_eq!(stats.modules_optimized, 1);
}

/// Test dead code elimination
#[test]
fn test_dead_code_elimination() {
    common::tracing::setup();
    
    let context = Context::create();
    let config = OptimizationConfig {
        optimization_level: OptimizationLevel::Default,
        ..Default::default()
    };
    
    let mut integration = RealLlvmOptimizationIntegration::new(&context, config).unwrap();
    integration.initialize().unwrap();
    
    // Create module with dead code
    let module = context.create_module("test_dce");
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = module.add_function("dce_test", fn_type, None);
    
    let basic_block = context.append_basic_block(function, "entry");
    let builder = context.create_builder();
    builder.position_at_end(basic_block);
    
    // Create some dead code (unused computation)
    let const_1 = i32_type.const_int(10, false);
    let const_2 = i32_type.const_int(5, false);
    let _dead_add = builder.build_int_add(const_1, const_2, "dead_add").unwrap();
    
    // Return a constant (not using the dead computation)
    let return_value = i32_type.const_int(42, false);
    builder.build_return(Some(&return_value)).unwrap();
    
    // Optimize the module
    let result = integration.optimize_module(&module);
    assert!(result.is_ok());
    
    let stats = integration.get_statistics();
    assert_eq!(stats.modules_optimized, 1);
}

/// Test constant propagation
#[test]
fn test_constant_propagation() {
    common::tracing::setup();
    
    let context = Context::create();
    let config = OptimizationConfig {
        optimization_level: OptimizationLevel::Default,
        ..Default::default()
    };
    
    let mut integration = RealLlvmOptimizationIntegration::new(&context, config).unwrap();
    integration.initialize().unwrap();
    
    // Create module with constant computations
    let module = context.create_module("test_const_prop");
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = module.add_function("const_prop_test", fn_type, None);
    
    let basic_block = context.append_basic_block(function, "entry");
    let builder = context.create_builder();
    builder.position_at_end(basic_block);
    
    // Create constant computations that can be folded
    let const_10 = i32_type.const_int(10, false);
    let const_5 = i32_type.const_int(5, false);
    let const_2 = i32_type.const_int(2, false);
    
    let add_result = builder.build_int_add(const_10, const_5, "add").unwrap();
    let mul_result = builder.build_int_mul(add_result, const_2, "mul").unwrap();
    
    builder.build_return(Some(&mul_result)).unwrap();
    
    // Optimize the module
    let result = integration.optimize_module(&module);
    assert!(result.is_ok());
    
    let stats = integration.get_statistics();
    assert_eq!(stats.modules_optimized, 1);
}

/// Test integration statistics functionality
#[test]
fn test_integration_statistics() {
    common::tracing::setup();
    
    let mut stats = IntegrationStatistics::default();
    
    // Test initial state
    assert_eq!(stats.total_optimizations(), 0);
    assert_eq!(stats.optimization_rate(), 0.0);
    
    // Add some statistics
    stats.functions_inlined = 5;
    stats.instructions_eliminated = 10;
    stats.constants_propagated = 8;
    stats.goroutine_optimizations = 3;
    stats.channel_optimizations = 2;
    stats.total_optimization_time = Duration::from_millis(100);
    
    // Test calculated values
    assert_eq!(stats.total_optimizations(), 28); // 5 + 10 + 0 + 8 + 0 + 3 + 2 + 0 + 0
    assert_eq!(stats.optimization_rate(), 0.28); // 28 / 100ms
}

/// Test optimization with all levels for compatibility
#[test]
fn test_all_optimization_levels_compatibility() {
    common::tracing::setup();
    
    let context = Context::create();
    
    let levels = vec![
        ("None", OptimizationLevel::None),
        ("Less", OptimizationLevel::Less),
        ("Default", OptimizationLevel::Default),
        ("Aggressive", OptimizationLevel::Aggressive),
        ("Size", OptimizationLevel::Size),
        ("SizeAggressive", OptimizationLevel::SizeAggressive),
    ];
    
    for (level_name, level) in levels {
        let config = OptimizationConfig {
            optimization_level: level,
            ..Default::default()
        };
        
        let mut integration = RealLlvmOptimizationIntegration::new(&context, config).unwrap();
        assert!(integration.initialize().is_ok(), "Failed to initialize {}", level_name);
        
        // Test with simple module
        let module = context.create_module(&format!("test_{}", level_name));
        let result = integration.optimize_module(&module);
        assert!(result.is_ok(), "Failed to optimize module with {}", level_name);
        
        let stats = integration.get_statistics();
        assert_eq!(stats.modules_optimized, 1, "Module count wrong for {}", level_name);
    }
}

/// Test error handling in optimization
#[test]
fn test_optimization_error_handling() {
    common::tracing::setup();
    
    let context = Context::create();
    let config = OptimizationConfig::default();
    
    let integration = RealLlvmOptimizationIntegration::new(&context, config);
    assert!(integration.is_ok());
    
    // Test that invalid operations don't crash
    let integration = integration.unwrap();
    let stats = integration.get_statistics();
    assert_eq!(stats.modules_optimized, 0);
}

/// Test pass manager with different configurations
#[test]
fn test_pass_manager_configurations() {
    common::tracing::setup();
    
    let context = Context::create();
    
    // Test O0 configuration
    let manager_o0 = RealLlvmPassManager::new(&context, OptimizationLevel::None);
    let stats_o0 = manager_o0.get_statistics();
    assert_eq!(stats_o0.total_optimizations(), 0);
    
    // Test O2 configuration
    let manager_o2 = RealLlvmPassManager::new(&context, OptimizationLevel::Default);
    let stats_o2 = manager_o2.get_statistics();
    assert_eq!(stats_o2.total_optimizations(), 0);
    
    // Test O3 configuration
    let manager_o3 = RealLlvmPassManager::new(&context, OptimizationLevel::Aggressive);
    let stats_o3 = manager_o3.get_statistics();
    assert_eq!(stats_o3.total_optimizations(), 0);
}
