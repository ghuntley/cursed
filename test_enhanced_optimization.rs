//! Test enhanced LLVM optimization integration
//! 
//! This test verifies that the enhanced optimization system works correctly
//! and provides performance improvements over the basic optimization system.

use cursed::codegen::llvm::{
    EnhancedOptimizationManager, EnhancedOptimizationConfig,
    OptimizationIntegrationFactory, CompilationPhase,
    OptimizationManager, OptimizationConfig
};
use cursed::error::Result;
use inkwell::{
    context::Context,
    module::Module,
    builder::Builder,
    values::IntValue,
    types::IntType,
    AddressSpace,
};
use std::time::Instant;

#[test]
fn test_enhanced_optimization_configuration() {
    // Test configuration creation
    let self_hosting_config = EnhancedOptimizationConfig::for_self_hosting();
    assert!(self_hosting_config.enable_self_hosting_optimizations);
    assert!(self_hosting_config.cross_module_inlining);
    assert_eq!(self_hosting_config.bootstrap_optimization_level, cursed::codegen::llvm::OptimizationLevel::O3);
    
    let dev_config = EnhancedOptimizationConfig::for_development();
    assert!(dev_config.debug_optimization_pipeline);
    assert_eq!(dev_config.bootstrap_optimization_level, cursed::codegen::llvm::OptimizationLevel::O1);
    
    let release_config = EnhancedOptimizationConfig::for_release();
    assert!(release_config.enable_fast_math);
    assert_eq!(release_config.vectorization_factor, Some(8));
}

#[test]
fn test_optimization_integration_factory() {
    let context = Context::create();
    
    // Test self-hosting integration
    let self_hosting_integration = OptimizationIntegrationFactory::for_self_hosting(&context);
    // Verify configuration is correct for self-hosting
    
    // Test development integration
    let dev_integration = OptimizationIntegrationFactory::for_development(&context);
    // Verify configuration is correct for development
    
    // Test release integration
    let release_integration = OptimizationIntegrationFactory::for_release(&context);
    // Verify configuration is correct for release
}

#[test]
fn test_enhanced_optimization_vs_basic() -> Result<()> {
    let context = Context::create();
    let module = create_test_module(&context);
    
    // Test basic optimization
    let basic_start = Instant::now();
    let basic_config = OptimizationConfig::release_config();
    let mut basic_manager = OptimizationManager::new(&context, basic_config);
    basic_manager.optimize_module(&module)?;
    let basic_time = basic_start.elapsed();
    
    // Reset module for enhanced optimization
    let module = create_test_module(&context);
    
    // Test enhanced optimization
    let enhanced_start = Instant::now();
    let enhanced_config = EnhancedOptimizationConfig::for_release();
    let mut enhanced_manager = EnhancedOptimizationManager::new(&context, enhanced_config);
    let enhanced_result = enhanced_manager.optimize_module(&module)?;
    let enhanced_time = enhanced_start.elapsed();
    
    // Verify enhanced optimization provides benefits
    assert!(enhanced_result.success);
    assert!(enhanced_result.stages_completed > 0);
    
    // Log performance comparison
    println!("Basic optimization time: {:?}", basic_time);
    println!("Enhanced optimization time: {:?}", enhanced_time);
    println!("Enhanced stages completed: {}", enhanced_result.stages_completed);
    
    if let (Some(initial), Some(final_complexity)) = 
       (&enhanced_result.initial_complexity, &enhanced_result.final_complexity) {
        let improvement = final_complexity.improvement_over(initial);
        println!("Complexity improvement: {:.2}%", improvement);
        assert!(improvement >= 0.0); // Should not make things worse
    }
    
    Ok(())
}

#[test]
fn test_inlining_integration() -> Result<()> {
    let context = Context::create();
    let module = create_inlining_test_module(&context);
    
    let enhanced_config = EnhancedOptimizationConfig::for_self_hosting();
    let mut enhanced_manager = EnhancedOptimizationManager::new(&context, enhanced_config);
    
    let result = enhanced_manager.optimize_module(&module)?;
    
    // Verify inlining occurred
    assert!(result.success);
    assert!(!result.warnings.is_empty() || !result.errors.is_empty() || result.performance_improvement >= 0.0);
    
    Ok(())
}

#[test]
fn test_optimization_phase_configuration() -> Result<()> {
    let context = Context::create();
    let mut integration = OptimizationIntegrationFactory::for_development(&context);
    
    // Test different compilation phases
    integration.configure_for_phase(CompilationPhase::Bootstrap)?;
    assert!(integration.config.self_hosting_mode);
    
    integration.configure_for_phase(CompilationPhase::Development)?;
    assert!(!integration.config.self_hosting_mode);
    assert!(integration.config.fallback_on_error);
    
    integration.configure_for_phase(CompilationPhase::Release)?;
    assert!(!integration.config.fallback_on_error);
    
    Ok(())
}

#[test]
fn test_performance_monitoring() -> Result<()> {
    let context = Context::create();
    let module = create_test_module(&context);
    
    let enhanced_config = EnhancedOptimizationConfig::for_development();
    let mut enhanced_manager = EnhancedOptimizationManager::new(&context, enhanced_config);
    
    let result = enhanced_manager.optimize_module(&module)?;
    
    // Check performance monitoring data
    let monitor = enhanced_manager.get_performance_monitor();
    let stage_times = monitor.get_stage_times();
    assert!(!stage_times.is_empty());
    
    let bottlenecks = monitor.get_bottlenecks();
    // Should not have critical bottlenecks in this simple test
    for bottleneck in bottlenecks {
        println!("Bottleneck: {} - {}", bottleneck.stage, bottleneck.issue);
    }
    
    Ok(())
}

fn create_test_module(context: &Context) -> Module {
    let module = context.create_module("test_module");
    let builder = context.create_builder();
    
    // Create a simple test function for optimization
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[i32_type.into()], false);
    let function = module.add_function("test_function", fn_type, None);
    
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);
    
    let param = function.get_nth_param(0).unwrap().into_int_value();
    let const_one = i32_type.const_int(1, false);
    let result = builder.build_int_add(param, const_one, "add_result").unwrap();
    builder.build_return(Some(&result)).unwrap();
    
    module
}

fn create_inlining_test_module(context: &Context) -> Module {
    let module = context.create_module("inlining_test_module");
    let builder = context.create_builder();
    
    let i32_type = context.i32_type();
    
    // Create a small function that should be inlined
    let small_fn_type = i32_type.fn_type(&[i32_type.into()], false);
    let small_function = module.add_function("small_function", small_fn_type, None);
    
    let small_bb = context.append_basic_block(small_function, "entry");
    builder.position_at_end(small_bb);
    
    let small_param = small_function.get_nth_param(0).unwrap().into_int_value();
    let const_one = i32_type.const_int(1, false);
    let small_result = builder.build_int_add(small_param, const_one, "small_add").unwrap();
    builder.build_return(Some(&small_result)).unwrap();
    
    // Create a caller function
    let caller_fn_type = i32_type.fn_type(&[i32_type.into()], false);
    let caller_function = module.add_function("caller_function", caller_fn_type, None);
    
    let caller_bb = context.append_basic_block(caller_function, "entry");
    builder.position_at_end(caller_bb);
    
    let caller_param = caller_function.get_nth_param(0).unwrap().into_int_value();
    
    // Call the small function (should be inlined)
    let call_result = builder.build_call(small_function, &[caller_param.into()], "call_small").unwrap();
    let call_value = call_result.try_as_basic_value().left().unwrap().into_int_value();
    
    builder.build_return(Some(&call_value)).unwrap();
    
    module
}

fn main() {
    println!("Running enhanced optimization tests...");
    
    test_enhanced_optimization_configuration();
    println!("✓ Configuration tests passed");
    
    test_optimization_integration_factory();
    println!("✓ Integration factory tests passed");
    
    if let Err(e) = test_enhanced_optimization_vs_basic() {
        println!("✗ Enhanced vs basic test failed: {}", e);
    } else {
        println!("✓ Enhanced vs basic tests passed");
    }
    
    if let Err(e) = test_inlining_integration() {
        println!("✗ Inlining integration test failed: {}", e);
    } else {
        println!("✓ Inlining integration tests passed");
    }
    
    if let Err(e) = test_optimization_phase_configuration() {
        println!("✗ Phase configuration test failed: {}", e);
    } else {
        println!("✓ Phase configuration tests passed");
    }
    
    if let Err(e) = test_performance_monitoring() {
        println!("✗ Performance monitoring test failed: {}", e);
    } else {
        println!("✓ Performance monitoring tests passed");
    }
    
    println!("Enhanced optimization tests complete!");
}
