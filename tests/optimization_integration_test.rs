//! Integration tests for LLVM optimization pass system
//!
//! This test suite validates the comprehensive optimization system for the CURSED compiler,
//! testing optimization levels, custom pass sequences, performance metrics, and CLI integration.

use cursed::codegen::llvm::  ::OptimizationManager, OptimizationConfig, OptimizationPass, create_optimization_manager;
use inkwell::context::Context;
use inkwell::OptimizationLevel;
use std::time::Duration;

// Test utility macros
macro_rules! init_tracing {(} => {let _ = tracing_subscriber::fmt(}))
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env();)
            .try_init()}

#[test]
fn test_optimization_manager_creation() {common::tracing::init_tracing!(})
    
    // Test all standard optimization levels
    for level in 0..=3   {let manager = OptimizationManager::for_level(level}})
        assert!(manager.is_ok(), Failed to create manager for level   {}, level)
        
        let manager = manager.unwrap();
        let config = manager.get_config();
        match level     {0 => assert!(matches!(config.level, OptimizationLevel::None},))
            1 => assert!(matches!(config.level, OptimizationLevel::Less),)
            2 => assert!(matches!(config.level, OptimizationLevel::Default),)
            3 => assert!(matches!(config.level, OptimizationLevel::Aggressive),)
            _ => unreachable!()}
    
    // Test invalid level
    let invalid = OptimizationManager::for_level(5);
    assert!(invalid.is_err();)

#[test]
fn test_optimization_manager_from_string() {common::tracing::init_tracing!(})
    
    let test_cases = vec![(O0, true),]
        (O1, true),"
        (", ", true),
        (, 0", 1, true),"
        (, 2"")
        (, , true),""
        (, , false),]""
    let module = context.create_module(, "")
    let add2 = builder.build_int_add(add1, const_1,  add2).unwrap();add3.unwrap();"
         ", 
        let test_cases = vec![(vec![-O2.to_string(),  test.csd , "")]]
            (vec![-".to_string()], Some(", .to_string(),  test ."-".to_string(), ))
            ", ".to_string().to_string();
             ".csd .to_string()"
        assert_eq!(opt_args.level,  ."fixed)
        let args = vec![O2.to_string().to_string()"]
             inline "--enable-pass .to_string().to_string()"
             ,  ."fixed"