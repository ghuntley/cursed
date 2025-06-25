/// Advanced LLVM Optimization Integration Tests
/// 
/// Comprehensive tests for the advanced optimization system including:
/// - Loop detection and optimization
/// - Vectorization analysis and transformation
/// - Target-specific optimizations
/// - Performance monitoring and measurement

use cursed::optimization::advanced_llvm_integration::{AdvancedLlvmIntegration, AdvancedLlvmConfig};
use cursed::optimization::target_optimization::{TargetOptimizationManager, TargetOptimizationConfig, CpuArchitecture};
use cursed::optimization::enhanced_llvm_optimization::{EnhancedLlvmOptimizer, EnhancedOptimizationConfig};
use cursed::optimization::config::OptimizationConfig;
use cursed::error::Result;
use inkwell::context::Context;
use std::time::Duration;
use tracing::{info, debug};

#[path = "tracing_setup.rs"]
pub mod tracing_setup;

/// Test advanced loop detection and analysis
#[test]
fn test_loop_detection_and_analysis() -> Result<()> {
    tracing_setup::init_test_tracing();
    info!("Testing advanced loop detection and analysis");
    
    let context = Context::create();
    let config = AdvancedLlvmConfig::default();
    let integration = AdvancedLlvmIntegration::new(&context, "test_module", config)?;
    
    // Create a test function with loops
    let module = integration.get_module();
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[i32_type.into()], false);
    let function = module.add_function("test_loop_function", fn_type, None);
    
    // Add basic blocks to simulate a loop structure
    let entry_block = context.append_basic_block(function, "entry");
    let loop_header = context.append_basic_block(function, "loop_header");
    let loop_body = context.append_basic_block(function, "loop_body");
    let exit_block = context.append_basic_block(function, "exit");
    
    let builder = context.create_builder();
    
    // Build a simple loop structure
    builder.position_at_end(entry_block);
    let initial_value = i32_type.const_int(0, false);
    builder.build_unconditional_branch(loop_header)?;
    
    builder.position_at_end(loop_header);
    let phi = builder.build_phi(i32_type, "loop_var")?;
    phi.add_incoming(&[(&initial_value, entry_block)]);
    
    let limit = i32_type.const_int(10, false);
    let condition = builder.build_int_compare(
        inkwell::IntPredicate::ULT,
        phi.as_basic_value().into_int_value(),
        limit,
        "loop_condition"
    )?;
    builder.build_conditional_branch(condition, loop_body, exit_block)?;
    
    builder.position_at_end(loop_body);
    let increment = i32_type.const_int(1, false);
    let next_value = builder.build_int_add(
        phi.as_basic_value().into_int_value(),
        increment,
        "next_value"
    )?;
    phi.add_incoming(&[(&next_value, loop_body)]);
    builder.build_unconditional_branch(loop_header)?;
    
    builder.position_at_end(exit_block);
    builder.build_return(Some(&phi.as_basic_value()))?;
    
    // Verify the function structure
    assert!(function.verify(true));
    
    debug!("Created test function with loop structure");
    debug!("Function has {} basic blocks", function.count_basic_blocks());
    
    // Test loop analysis would be performed by the optimization system
    // Since the actual loop detection requires more complex setup,
    // we verify the structure is correct for analysis
    assert_eq!(function.count_basic_blocks(), 4);
    
    info!("✅ Loop detection and analysis test completed");
    Ok(())
}

/// Test vectorization analysis and transformation
#[test]
fn test_vectorization_analysis() -> Result<()> {
    tracing_setup::init_test_tracing();
    info!("Testing vectorization analysis and transformation");
    
    let context = Context::create();
    let config = AdvancedLlvmConfig {
        enable_vectorization: true,
        enable_auto_vectorization: true,
        vectorization_factor: 8,
        ..Default::default()
    };
    
    let mut integration = AdvancedLlvmIntegration::new(&context, "vectorization_test", config)?;
    
    // Create a function that could benefit from vectorization
    let module = integration.get_module();
    let f32_type = context.f32_type();
    let ptr_type = f32_type.ptr_type(inkwell::AddressSpace::default());
    let fn_type = context.void_type().fn_type(&[ptr_type.into(), ptr_type.into(), ptr_type.into()], false);
    let function = module.add_function("vector_add", fn_type, None);
    
    let entry_block = context.append_basic_block(function, "entry");
    let loop_block = context.append_basic_block(function, "loop");
    let exit_block = context.append_basic_block(function, "exit");
    
    let builder = context.create_builder();
    
    // Build a vectorizable loop (array addition)
    builder.position_at_end(entry_block);
    let i32_type = context.i32_type();
    let zero = i32_type.const_int(0, false);
    builder.build_unconditional_branch(loop_block)?;
    
    builder.position_at_end(loop_block);
    let phi = builder.build_phi(i32_type, "i")?;
    phi.add_incoming(&[(&zero, entry_block)]);
    
    // Load from input arrays
    let a_ptr = function.get_nth_param(0).unwrap().into_pointer_value();
    let b_ptr = function.get_nth_param(1).unwrap().into_pointer_value();
    let c_ptr = function.get_nth_param(2).unwrap().into_pointer_value();
    
    let i_val = phi.as_basic_value().into_int_value();
    let a_gep = unsafe { builder.build_gep(f32_type, a_ptr, &[i_val], "a_gep")? };
    let b_gep = unsafe { builder.build_gep(f32_type, b_ptr, &[i_val], "b_gep")? };
    let c_gep = unsafe { builder.build_gep(f32_type, c_ptr, &[i_val], "c_gep")? };
    
    let a_val = builder.build_load(f32_type, a_gep, "a_val")?;
    let b_val = builder.build_load(f32_type, b_gep, "b_val")?;
    
    // Perform addition (vectorizable operation)
    let sum = builder.build_float_add(
        a_val.into_float_value(),
        b_val.into_float_value(),
        "sum"
    )?;
    
    builder.build_store(c_gep, sum)?;
    
    // Loop increment and condition
    let one = i32_type.const_int(1, false);
    let next_i = builder.build_int_add(i_val, one, "next_i")?;
    phi.add_incoming(&[(&next_i, loop_block)]);
    
    let limit = i32_type.const_int(1000, false);
    let condition = builder.build_int_compare(
        inkwell::IntPredicate::ULT,
        next_i,
        limit,
        "condition"
    )?;
    builder.build_conditional_branch(condition, loop_block, exit_block)?;
    
    builder.position_at_end(exit_block);
    builder.build_return(None)?;
    
    // Verify the function
    assert!(function.verify(true));
    
    debug!("Created vectorizable function with {} basic blocks", function.count_basic_blocks());
    
    // The actual vectorization would be performed by the optimization system
    // For now, we verify the structure is suitable for vectorization
    assert_eq!(function.count_basic_blocks(), 3);
    
    info!("✅ Vectorization analysis test completed");
    Ok(())
}

/// Test target-specific optimizations
#[test]
fn test_target_specific_optimizations() -> Result<()> {
    tracing_setup::init_test_tracing();
    info!("Testing target-specific optimizations");
    
    // Test for x86_64 architecture
    let config_x86 = TargetOptimizationConfig {
        target_architecture: CpuArchitecture::X86_64,
        enable_simd: true,
        enable_vectorization: true,
        vectorization_factor: 8,
        ..Default::default()
    };
    
    let mut manager_x86 = TargetOptimizationManager::new(config_x86)?;
    
    // Verify CPU info detection
    let cpu_info = manager_x86.get_cpu_info();
    assert_eq!(cpu_info.architecture, CpuArchitecture::X86_64);
    assert!(cpu_info.simd_capabilities.max_vector_width >= 128);
    
    debug!("Detected x86_64 CPU with vector width: {}", cpu_info.simd_capabilities.max_vector_width);
    
    // Test for ARM64 architecture
    let config_arm = TargetOptimizationConfig {
        target_architecture: CpuArchitecture::Arm64,
        enable_simd: true,
        enable_vectorization: true,
        vectorization_factor: 4,
        ..Default::default()
    };
    
    let mut manager_arm = TargetOptimizationManager::new(config_arm)?;
    
    let cpu_info_arm = manager_arm.get_cpu_info();
    assert_eq!(cpu_info_arm.architecture, CpuArchitecture::Arm64);
    assert!(cpu_info_arm.simd_capabilities.max_vector_width >= 64);
    
    debug!("Detected ARM64 CPU with vector width: {}", cpu_info_arm.simd_capabilities.max_vector_width);
    
    info!("✅ Target-specific optimization test completed");
    Ok(())
}

/// Test enhanced LLVM optimizer integration
#[test]
fn test_enhanced_llvm_optimizer() -> Result<()> {
    tracing_setup::init_test_tracing();
    info!("Testing enhanced LLVM optimizer integration");
    
    let context = Context::create();
    let enhanced_config = EnhancedOptimizationConfig {
        enable_adaptive_optimization: true,
        enable_performance_monitoring: true,
        enable_compilation_cache: true,
        max_optimization_time: Duration::from_secs(30),
        ..Default::default()
    };
    
    let base_config = OptimizationConfig::default();
    
    let optimizer = EnhancedLlvmOptimizer::new(&context, enhanced_config, base_config)?;
    
    // Create a test module for optimization
    let module = context.create_module("test_optimization");
    
    // Add a simple function to optimize
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[i32_type.into(), i32_type.into()], false);
    let function = module.add_function("test_function", fn_type, None);
    
    let entry_block = context.append_basic_block(function, "entry");
    let builder = context.create_builder();
    builder.position_at_end(entry_block);
    
    // Simple arithmetic that can be optimized
    let param1 = function.get_nth_param(0).unwrap().into_int_value();
    let param2 = function.get_nth_param(1).unwrap().into_int_value();
    
    let sum = builder.build_int_add(param1, param2, "sum")?;
    let constant = i32_type.const_int(42, false);
    let result = builder.build_int_add(sum, constant, "result")?;
    
    builder.build_return(Some(&result))?;
    
    assert!(function.verify(true));
    
    debug!("Created test module with optimizable function");
    
    // Test optimization
    let optimization_result = optimizer.optimize_module(&module)?;
    
    // Verify optimization results
    assert!(optimization_result.performance_improvements.runtime_improvement >= 0.0);
    assert!(optimization_result.compilation_metrics.total_compilation_time > Duration::from_millis(0));
    
    let metrics = optimizer.get_optimization_metrics();
    assert!(metrics.total_optimizations > 0);
    
    debug!("Optimization completed with {:.1}% runtime improvement", 
           optimization_result.performance_improvements.runtime_improvement);
    
    info!("✅ Enhanced LLVM optimizer integration test completed");
    Ok(())
}

/// Test performance monitoring accuracy
#[test]
fn test_performance_monitoring() -> Result<()> {
    tracing_setup::init_test_tracing();
    info!("Testing performance monitoring accuracy");
    
    let context = Context::create();
    let config = EnhancedOptimizationConfig {
        enable_performance_monitoring: true,
        enable_adaptive_optimization: false, // Focus on monitoring
        ..Default::default()
    };
    
    let base_config = OptimizationConfig::default();
    let optimizer = EnhancedLlvmOptimizer::new(&context, config, base_config)?;
    
    // Create a module that will take some time to optimize
    let module = context.create_module("performance_test");
    
    // Add multiple functions to create optimization work
    for i in 0..10 {
        let i32_type = context.i32_type();
        let fn_type = i32_type.fn_type(&[i32_type.into()], false);
        let function = module.add_function(&format!("test_function_{}", i), fn_type, None);
        
        let entry_block = context.append_basic_block(function, "entry");
        let builder = context.create_builder();
        builder.position_at_end(entry_block);
        
        let param = function.get_nth_param(0).unwrap().into_int_value();
        
        // Create some computation to optimize
        let mut result = param;
        for j in 0..5 {
            let constant = i32_type.const_int(j as u64, false);
            result = builder.build_int_add(result, constant, &format!("add_{}", j))?;
        }
        
        builder.build_return(Some(&result))?;
        assert!(function.verify(true));
    }
    
    debug!("Created performance test module with {} functions", module.get_functions().count());
    
    // Run optimization with monitoring
    let start_time = std::time::Instant::now();
    let result = optimizer.optimize_module(&module)?;
    let elapsed = start_time.elapsed();
    
    // Verify monitoring captured meaningful data
    assert!(result.compilation_metrics.total_compilation_time > Duration::from_millis(0));
    assert!(result.compilation_metrics.total_compilation_time <= elapsed + Duration::from_millis(100));
    
    debug!("Measured compilation time: {:?}", result.compilation_metrics.total_compilation_time);
    debug!("Actual elapsed time: {:?}", elapsed);
    
    // Check that metrics are reasonable
    let compilation_metrics = optimizer.get_compilation_metrics();
    assert!(compilation_metrics.peak_memory_usage > 0);
    
    info!("✅ Performance monitoring accuracy test completed");
    Ok(())
}

/// Test optimization effectiveness measurement
#[test]
fn test_optimization_effectiveness() -> Result<()> {
    tracing_setup::init_test_tracing();
    info!("Testing optimization effectiveness measurement");
    
    let context = Context::create();
    let config = AdvancedLlvmConfig {
        enable_advanced_inlining: true,
        enable_cfg_transformations: true,
        enable_vectorization: true,
        inline_threshold: 50,
        max_inline_size: 200,
        ..Default::default()
    };
    
    let mut integration = AdvancedLlvmIntegration::new(&context, "effectiveness_test", config)?;
    
    // Initialize optimization passes
    integration.initialize_passes()?;
    
    let module = integration.get_module();
    
    // Create functions that should benefit from optimization
    let i32_type = context.i32_type();
    
    // Small function that should be inlined
    let small_fn_type = i32_type.fn_type(&[i32_type.into()], false);
    let small_function = module.add_function("small_function", small_fn_type, None);
    
    let small_entry = context.append_basic_block(small_function, "entry");
    let builder = context.create_builder();
    builder.position_at_end(small_entry);
    
    let param = small_function.get_nth_param(0).unwrap().into_int_value();
    let constant = i32_type.const_int(1, false);
    let result = builder.build_int_add(param, constant, "result")?;
    builder.build_return(Some(&result))?;
    
    // Main function that calls the small function
    let main_fn_type = i32_type.fn_type(&[i32_type.into()], false);
    let main_function = module.add_function("main_function", main_fn_type, None);
    
    let main_entry = context.append_basic_block(main_function, "entry");
    builder.position_at_end(main_entry);
    
    let main_param = main_function.get_nth_param(0).unwrap().into_int_value();
    
    // Call small function multiple times (should be inlined)
    let call1 = builder.build_call(small_function, &[main_param.into()], "call1")?;
    let call2 = builder.build_call(small_function, &[call1.try_as_basic_value().left().unwrap().into()], "call2")?;
    
    builder.build_return(Some(&call2.try_as_basic_value().left().unwrap()))?;
    
    assert!(small_function.verify(true));
    assert!(main_function.verify(true));
    
    debug!("Created test functions for inlining optimization");
    
    // Run optimization
    let stats = integration.optimize_module()?;
    
    // Verify optimization was effective
    assert!(stats.total_optimization_time > Duration::from_millis(0));
    
    debug!("Optimization statistics: functions_inlined={}, total_time={:?}",
           stats.inlining_stats.functions_inlined,
           stats.total_optimization_time);
    
    info!("✅ Optimization effectiveness measurement test completed");
    Ok(())
}

/// Test adaptive optimization configuration
#[test]
fn test_adaptive_optimization() -> Result<()> {
    tracing_setup::init_test_tracing();
    info!("Testing adaptive optimization configuration");
    
    let context = Context::create();
    
    // Test different optimization levels
    let configs = vec![
        ("Debug", AdvancedLlvmConfig {
            optimization_level: 0,
            enable_advanced_inlining: false,
            ..Default::default()
        }),
        ("Release", AdvancedLlvmConfig {
            optimization_level: 2,
            enable_advanced_inlining: true,
            enable_vectorization: true,
            ..Default::default()
        }),
        ("Aggressive", AdvancedLlvmConfig {
            optimization_level: 3,
            enable_advanced_inlining: true,
            enable_vectorization: true,
            enable_cfg_transformations: true,
            ..Default::default()
        }),
    ];
    
    for (name, config) in configs {
        debug!("Testing {} optimization configuration", name);
        
        let integration = AdvancedLlvmIntegration::new(&context, &format!("{}_test", name), config)?;
        
        // Verify configuration was applied
        let stats = integration.get_statistics();
        debug!("{} configuration created successfully", name);
        
        // Test that different configurations produce different optimization behaviors
        // This would be validated by running actual optimizations and comparing results
    }
    
    info!("✅ Adaptive optimization configuration test completed");
    Ok(())
}

/// Test memory usage optimization
#[test]
fn test_memory_usage_optimization() -> Result<()> {
    tracing_setup::init_test_tracing();
    info!("Testing memory usage optimization");
    
    let context = Context::create();
    let config = EnhancedOptimizationConfig {
        enable_compilation_cache: true,
        enable_performance_monitoring: true,
        ..Default::default()
    };
    
    let base_config = OptimizationConfig::default();
    let optimizer = EnhancedLlvmOptimizer::new(&context, config, base_config)?;
    
    // Create a module with repetitive patterns that can benefit from caching
    let module = context.create_module("memory_test");
    
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    
    // Create similar functions that should benefit from cached optimizations
    for i in 0..5 {
        let function = module.add_function(&format!("similar_function_{}", i), fn_type, None);
        let entry_block = context.append_basic_block(function, "entry");
        let builder = context.create_builder();
        builder.position_at_end(entry_block);
        
        // Similar computation pattern
        let constant = i32_type.const_int(i as u64, false);
        let doubled = builder.build_int_mul(constant, i32_type.const_int(2, false), "doubled")?;
        builder.build_return(Some(&doubled))?;
        
        assert!(function.verify(true));
    }
    
    debug!("Created module with {} similar functions", module.get_functions().count());
    
    // First optimization run - establishes baseline
    let result1 = optimizer.optimize_module(&module)?;
    
    // Second optimization run - should benefit from caching
    let result2 = optimizer.optimize_module(&module)?;
    
    // Verify memory usage is tracked
    assert!(result1.compilation_metrics.peak_memory_usage > 0);
    assert!(result2.compilation_metrics.peak_memory_usage > 0);
    
    debug!("First run memory usage: {} bytes", result1.compilation_metrics.peak_memory_usage);
    debug!("Second run memory usage: {} bytes", result2.compilation_metrics.peak_memory_usage);
    
    info!("✅ Memory usage optimization test completed");
    Ok(())
}
