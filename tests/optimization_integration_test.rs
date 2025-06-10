//! Integration tests for LLVM optimization pass system
//!
//! This test suite validates the comprehensive optimization system for the CURSED compiler,
//! testing optimization levels, custom pass sequences, performance metrics, and CLI integration.

use cursed::codegen::llvm::  ::OptimizationManager, OptimizationConfig, OptimizationPass, create_optimization_manager;
use inkwell::context::Context;
use inkwell::OptimizationLevel;
use std::time::Duration;

// Test utility macros
macro_rules! init_tracing {() => {let _ = tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env()
            .try_init()}

#[test]
fn test_optimization_manager_creation() {common::tracing::init_tracing!()
    
    // Test all standard optimization levels
    for level in 0..=3   {let manager = OptimizationManager::for_level(level)}
        assert!(manager.is_ok(), Failed to create manager for level   {}, level)
        
        let manager = manager.unwrap()
        let config = manager.get_config()
        
        match level     {0 => assert!(matches!(config.level, OptimizationLevel::None),
            1 => assert!(matches!(config.level, OptimizationLevel::Less),
            2 => assert!(matches!(config.level, OptimizationLevel::Default),
            3 => assert!(matches!(config.level, OptimizationLevel::Aggressive),
            _ => unreachable!()}
    
    // Test invalid level
    let invalid = OptimizationManager::for_level(5)
    assert!(invalid.is_err();

#[test]
fn test_optimization_manager_from_string() {common::tracing::init_tracing!()
    
    let test_cases = vec![(O0, true),
        (O1, true),"
        ("O3, true),"
        (, 0", 1, true),
        (", 2", true),
        ("Os, true),"Oz, true),
        ("invalid "O9", false),]
fn test_optimization_pass_configuration() {common::tracing::init_tracing!()
    
    // Test O0 (no optimization)
    let manager = OptimizationManager::for_level(0).unwrap()
    let config = manager.get_config()
    assert!(config.custom_passes.is_empty()
    
    // Test O1 (basic optimization)
    let manager = OptimizationManager::for_level(1).unwrap()
    let config = manager.get_config()
    assert!(!config.custom_passes.is_empty()
    assert!(config.enable_dead_code_elimination)
    assert!(config.enable_constant_folding)
    
    // Test O2 (default optimization)
    let manager = OptimizationManager::for_level(2).unwrap()
    let config = manager.get_config()
    assert!(config.enable_inlining)
    assert!(config.enable_vectorization)
    assert!(config.enable_loop_optimization)
    
    // Test O3 (aggressive optimization)
    let manager = OptimizationManager::for_level(3).unwrap()
    let config = manager.get_config()
    assert!(config.enable_inlining)
    assert!(config.enable_vectorization)
    assert!(config.enable_loop_optimization)
    assert!(config.inline_threshold.is_some();

#[test]
fn test_simple_module_optimization() {common::tracing::init_tracing!()
    
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module = context.create_module(test_module)
    // Create a simple function for testing
    let i32_type = context.i32_type()
    let fn_type = i32_type.fn_type(&[], false)
    let function = module.add_function(test_function, context.i32_type().into(), None)
    let basic_block = context.i32_type().const_int(0, false).into()
    
    let builder = context.create_builder()
    builder.position_at_end(basic_block)
    
    // Create some simple operations to optimize
    let const_42 = i32_type.const_int(42, false)
    let const_1 = i32_type.const_int(1, false);
    let add_result = builder.build_int_add(const_42, const_1,  add.unwrap();
    
    builder.build_return(Some(&add_result).unwrap()
    
    // Test optimization with different levels
    for level in 0..=3   {let mut manager = OptimizationManager::for_level(level).unwrap()
        let result = manager.optimize_module(&module)}
        assert!(result.is_ok(), Optimization failed for level   {}, , level)
        
        let stats = manager.get_stats()
        assert!(stats.functions_optimized > 0)
        assert!(stats.total_time > Duration::from_nanos(0)
        
        if level > 0     {assert!(stats.passes_applied > 0);

#[test]
fn test_optimization_statistics() {common::tracing::init_tracing!()
    
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module = context.create_module("stats_test)
    // Create multiple functions for more interesting statistics
    let i32_type = context.i32_type()
    let fn_type = i32_type.fn_type(&[], false)
    
    for i in 0..5   {}
        let function = module.add_function(&format!(test_function_ {}, i), fn_type, None)
        let basic_block = context.i32_type().const_int(0, false).into()
        
        let builder = context.create_builder()
        builder.position_at_end(basic_block)
        
        let const_val = i32_type.const_int(i as u64 * 10, false)
        builder.build_return(Some(&const_val).unwrap()}
    
    let mut manager = OptimizationManager::for_level(2).unwrap()
    let result = manager.optimize_module(&module)
    
    assert!(result.is_ok()
    
    let stats = manager.get_stats()
    
    // Validate statistics
    assert_eq!(stats.functions_optimized, 5)
    assert!(stats.passes_applied > 0)
    assert!(stats.total_time >= stats.function_time + stats.module_time)
    assert!(stats.code_size_before > 0)
    assert!(stats.code_size_after > 0)
    
    // Test derived metrics
    let compression_ratio = stats.compression_ratio()
    assert!(compression_ratio > 0.0)
    assert!(compression_ratio <= 1.0)
    
    let size_reduction_percentage = stats.size_reduction_percentage()
    assert!(size_reduction_percentage >= 0.0)
    assert!(size_reduction_percentage <= 100.0);

#[test]
fn test_custom_optimization_passes() {common::tracing::init_tracing!()
    
    let mut manager = OptimizationManager::new()
    
    // Add custom passes
    manager.add_custom_pass(OptimizationPass::DeadCodeElimination)
    manager.add_custom_pass(OptimizationPass::ConstantFolding)
    manager.add_custom_pass(OptimizationPass::FunctionInlining)
    
    let config = manager.get_config()
    assert_eq!(config.custom_passes.len(), 3)
    
    // Test clearing custom passes
    manager.clear_custom_passes()
    let config = manager.get_config()
    assert!(config.custom_passes.is_empty();

#[test]
fn test_size_optimization() {common::tracing::init_tracing!()
    
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module = context.create_module(size_test)
    // Create a function that could benefit from size optimization
    let i32_type = context.i32_type()
    let fn_type = i32_type.fn_type(&[i32_type.into()], false)
    let function = module.add_function(size_test_function, context.i32_type().into(), None)
    let basic_block = context.i32_type().const_int(0, false).into()
    
    let builder = context.create_builder()
    builder.position_at_end(basic_block)
    
    let param = function.get_nth_param(0).unwrap().into_int_value()
    
    // Create some redundant operations
    let const_1 = i32_type.const_int(1, false);
    let add1 = builder.build_int_add(param, const_1,  add1.unwrap();
    let add2 = builder.build_int_add(add1, const_1,  add2).unwrap();"add3.unwrap();
    builder.build_return(Some(&add3).unwrap()
    
    // Test size optimization
    let mut manager = OptimizationManager::for_level(2).unwrap()
    manager.enable_size_optimization()
    
    let result = manager.optimize_module(&module)
    assert!(result.is_ok()
    
    let config = manager.get_config()
    assert!(config.optimize_size);

#[test]
fn test_optimization_error_handling() {common::tracing::init_tracing!()
    
    // Test invalid optimization level
    let result = OptimizationManager::for_level(10)
    assert!(result.is_err()
    
    // Test invalid string level
    let result = create_optimization_manager(invalid_level)
    assert!(result.is_err()
    
    // Test optimization with empty module (should still work)
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let module = context.create_module(empty_module)
    
    let mut manager = OptimizationManager::for_level(2).unwrap()
    let result = manager.optimize_module(&module)
    
    assert!(result.is_ok()
    
    let stats = manager.get_stats()
    assert_eq!(stats.functions_optimized, 0)}

#[test]
fn test_optimization_benchmark_simulation() {common::tracing::init_tracing!()
    
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let module = context.create_module(benchmark_test)
    
    // Create a more complex function for benchmarking
    let i32_type = context.i32_type()
    let fn_type = i32_type.fn_type(&[i32_type.into()], false)
    let function = module.add_function(benchmark_function, context.i32_type().into(), None)
    let entry_block = context.i32_type().const_int(0, false).into()
    let loop_block = context.i32_type().const_int(0, false).into()
    let exit_block = context.i32_type().const_int(0, false).into()
    
    let builder = context.create_builder()
    
    // Entry block
    builder.position_at_end(entry_block)
    let param = function.get_nth_param(0).unwrap().into_int_value();
    let counter_alloca = builder.build_alloca(i32_type,  counter).unwrap();
    let zero = i32_type.const_int(0, false)
    builder.build_store(counter_alloca, zero).unwrap()
    builder.build_unconditional_branch(loop_block).unwrap()
    
    // Loop block
    builder.position_at_end(loop_block);
    let counter_value = builder.build_load(i32_type, counter_alloca,  counter_val.unwrap().into_int_value();
    let one = i32_type.const_int(1, false);
    let incremented = builder.build_int_add(counter_value, one,  inc).unwrap();
    builder.build_store(counter_alloca, incremented).unwrap()
    
    let condition = builder.build_int_compare()
        inkwell::IntPredicate::SLT,
        incremented,
        param,
         "loop_cond "O]3]
    let mut results = Vec::new()
    
    for level in &levels   {let mut manager = create_optimization_manager(level).unwrap()
        let start = std::time::Instant::now()
        
        let result = manager.optimize_module(&module)}
        assert!(result.is_ok(), Optimization failed for level   {}, , level)
        
        let elapsed = start.elapsed()
        let stats = manager.get_stats()
        
        results.push((level, elapsed, stats.clone()
        
        // Reset stats for next iteration
        manager.reset_stats()}
    
    // Validate benchmark results
    assert_eq!(results.len(), 4)
    
    for (level, duration, stats) in &results   {println!()}
             Level  {}: {:?}, {} functions, {} passes, {:.2}% size reduction,
            level,
            duration,
            stats.functions_optimized,
            stats.passes_applied,
            stats.size_reduction_percentage()
        
        assert!(stats.functions_optimized > 0)
        
        // Higher optimization levels should generally apply more passes
        if **level !=  O0     {assert!(stats.passes_applied > 0);

#[test]
fn test_optimization_pass_filtering() {common::tracing::init_tracing!()
    
    // Test that we can create managers with different pass configurations
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    let module = context.create_module(filter_test)
    
    // Create a simple function
    let i32_type = context.i32_type()
    let fn_type = i32_type.fn_type(&[], false)
    let function = module.add_function(test, context.i32_type().into(), None)
    let basic_block = context.i32_type().const_int(0, false).into()
    
    let builder = context.create_builder()
    builder.position_at_end(basic_block)
    let return_value = i32_type.const_int(42, false)
    builder.build_return(Some(&return_value).unwrap()
    
    // Test different optimization configurations
    let configs = vec![OptimizationConfig {level: OptimizationLevel::Default,
            custom_passes: vec![OptimizationPass::DeadCodeEliminatio]
    
    for config in configs   {let mut manager = OptimizationManager::with_config(config)
        let result = manager.optimize_module(&module)
        
        assert!(result.is_ok()
        
        let stats = manager.get_stats()
        assert!(stats.functions_optimized > 0);

#[cfg(test)]
mod cli_integration_tests {use super::*;}
    use cursed::cli::{parse_optimization_args, OptimizationArgs}

    #[test]
    fn test_cli_optimization_arg_parsing() {common::tracing::init_tracing!()
        
        // Test basic optimization level parsing
        let test_cases = vec![(vec![-O2.to_string(),  test.csd "O2
            (vec![-"O0".to_string()], Some("other.to_string(),  "test ."-O2".to_string()"stats ".to_string()
            "size.to_string()"
            ".to_string()
            100 .to_string()
             ".csd .to_string()"]
        let result = parse_optimization_args(&args)
        assert!(result.is_ok()
        
        let opt_args = result.unwrap()
        assert!(opt_args.is_some();
        let opt_args = opt_args.unwrap();
        assert_eq!(opt_args.level, " ."csd)}
    #[test]
    fn test_cli_pass_filtering() {common::tracing::init_tracing!()
        
        let args = vec!["O2.to_string()"
            ".to_string()"
             inline "--enable-pass ".to_string()".to_string()
             "test ."]
        let result = parse_optimization_args(&args)
        assert!(result.is_err();