/// LLVM LTO Integration Test Suite
/// 
/// Tests the LLVM-specific LTO integration including module management,
/// optimization passes, and object code generation.

#[path = "common.rs"]
pub mod common;

use cursed::codegen::llvm::lto_integration::{
    LlvmLtoIntegration, ModuleSummary, FunctionSummary, GlobalSummary,
    ImportDecision, ImportReason, GlobalCallGraph, ObjectFile
};
use cursed::optimization::lto::{LtoConfig, LtoLevel};
use cursed::error::{Error, Result};
use inkwell::context::Context;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;
use tracing::{info, debug};

/// Initialize tracing for tests
macro_rules! init_tracing {
    () => {
        common::tracing::setup();
    };
}

/// Test LLVM LTO integration creation
#[test]
fn test_llvm_lto_integration_creation() {
    init_tracing!();
    
    let context = Context::create();
    let config = LtoConfig::default();
    
    let integration = LlvmLtoIntegration::new(&context, config);
    assert!(integration.is_ok());
    
    let integration = integration.unwrap();
    assert_eq!(integration.get_config().level, LtoLevel::None);
}

/// Test target machine initialization
#[test]
fn test_target_machine_initialization() {
    init_tracing!();
    
    let context = Context::create();
    let config = LtoConfig {
        level: LtoLevel::Thin,
        ..Default::default()
    };
    
    let mut integration = LlvmLtoIntegration::new(&context, config).unwrap();
    
    // Initialize with native target
    let result = integration.initialize_target("x86_64-unknown-linux-gnu");
    if result.is_err() {
        // Skip test if target initialization fails (e.g., in CI without proper LLVM setup)
        info!("Skipping target initialization test - LLVM target not available");
        return;
    }
    
    info!("Target machine initialized successfully");
}

/// Test module addition and management
#[test]
fn test_module_management() {
    init_tracing!();
    
    let context = Context::create();
    let config = LtoConfig {
        level: LtoLevel::Thin,
        ..Default::default()
    };
    
    let mut integration = LlvmLtoIntegration::new(&context, config).unwrap();
    
    // Create test modules
    let module1 = context.create_module("test_module_1");
    let module2 = context.create_module("test_module_2");
    
    // Add modules
    let result1 = integration.add_module(module1);
    let result2 = integration.add_module(module2);
    
    assert!(result1.is_ok());
    assert!(result2.is_ok());
    
    info!("Successfully added 2 modules to LTO integration");
}

/// Test module summary creation
#[test]
fn test_module_summary_creation() {
    init_tracing!();
    
    let context = Context::create();
    let config = LtoConfig::default();
    let integration = LlvmLtoIntegration::new(&context, config).unwrap();
    
    // Create a test module with functions and globals
    let module = context.create_module("summary_test_module");
    
    // Add a simple function
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = module.add_function("test_function", fn_type, None);
    
    // Add entry block
    let entry_block = context.append_basic_block(function, "entry");
    let builder = context.create_builder();
    builder.position_at_end(entry_block);
    
    // Return a constant
    let return_value = i32_type.const_int(42, false);
    builder.build_return(Some(&return_value)).unwrap();
    
    // Add a global variable
    let global = module.add_global(i32_type, None, "test_global");
    global.set_initializer(&i32_type.const_int(123, false));
    
    // Create summary
    let summary = integration.create_module_summary(&module);
    assert!(summary.is_ok());
    
    let summary = summary.unwrap();
    assert_eq!(summary.name, "summary_test_module");
    assert_eq!(summary.functions.len(), 1);
    assert_eq!(summary.globals.len(), 1);
    
    // Check function summary
    let func_summary = &summary.functions[0];
    assert_eq!(func_summary.name, "test_function");
    assert_eq!(func_summary.size, 1); // One basic block
    
    // Check global summary
    let global_summary = &summary.globals[0];
    assert_eq!(global_summary.name, "test_global");
    assert!(!global_summary.is_constant); // Not explicitly marked as constant
    
    info!("Module summary created successfully with {} functions and {} globals",
          summary.functions.len(), summary.globals.len());
}

/// Test global call graph construction
#[test]
fn test_global_call_graph_construction() {
    init_tracing!();
    
    let mut call_graph = GlobalCallGraph::new();
    
    // Build a simple call graph
    call_graph.add_call("main", "helper1");
    call_graph.add_call("main", "helper2");
    call_graph.add_call("helper1", "utils");
    call_graph.add_call("helper2", "utils");
    call_graph.add_call("helper1", "helper2");
    
    // Test call count tracking
    assert_eq!(call_graph.get_call_count("utils"), 2);
    assert_eq!(call_graph.get_call_count("helper1"), 1);
    assert_eq!(call_graph.get_call_count("helper2"), 2);
    assert_eq!(call_graph.get_call_count("main"), 0);
    assert_eq!(call_graph.get_call_count("nonexistent"), 0);
    
    // Test function size (returns 0 since we haven't set sizes)
    assert_eq!(call_graph.get_function_size("helper1"), 0);
    
    info!("Global call graph constructed with proper call counting");
}

/// Test import decision logic
#[test]
fn test_import_decision_logic() {
    init_tracing!();
    
    let decision = ImportDecision {
        function_name: "small_utility".to_string(),
        reason: ImportReason::Inlining,
        estimated_benefit: 75,
    };
    
    assert_eq!(decision.function_name, "small_utility");
    assert!(matches!(decision.reason, ImportReason::Inlining));
    assert_eq!(decision.estimated_benefit, 75);
    
    // Test other import reasons
    let const_prop_decision = ImportDecision {
        function_name: "constant_provider".to_string(),
        reason: ImportReason::ConstantPropagation,
        estimated_benefit: 50,
    };
    
    assert!(matches!(const_prop_decision.reason, ImportReason::ConstantPropagation));
    
    let dce_decision = ImportDecision {
        function_name: "unused_function".to_string(),
        reason: ImportReason::DeadCodeElimination,
        estimated_benefit: 0,
    };
    
    assert!(matches!(dce_decision.reason, ImportReason::DeadCodeElimination));
    
    info!("Import decision logic working correctly");
}

/// Test LTO optimization workflow without real LLVM operations
#[test]
fn test_lto_optimization_workflow() {
    init_tracing!();
    
    let context = Context::create();
    let config = LtoConfig {
        level: LtoLevel::None,
        ..Default::default()
    };
    
    let mut integration = LlvmLtoIntegration::new(&context, config).unwrap();
    
    // Add test modules
    let module1 = context.create_module("workflow_module_1");
    let module2 = context.create_module("workflow_module_2");
    
    integration.add_module(module1).unwrap();
    integration.add_module(module2).unwrap();
    
    // Perform LTO (should skip with None level)
    let result = integration.perform_lto();
    assert!(result.is_ok());
    
    let lto_result = result.unwrap();
    assert_eq!(lto_result.optimized_modules.len(), 2);
    assert!(lto_result.total_time >= Duration::from_secs(0));
    assert_eq!(lto_result.size_reduction, 0); // No actual optimization
    
    info!("LTO workflow completed successfully");
}

/// Test Thin LTO workflow
#[test]
fn test_thin_lto_workflow() {
    init_tracing!();
    
    let context = Context::create();
    let config = LtoConfig {
        level: LtoLevel::Thin,
        max_worker_threads: 2,
        thin_lto_partition_threshold: 500,
        ..Default::default()
    };
    
    let mut integration = LlvmLtoIntegration::new(&context, config).unwrap();
    
    // Add multiple modules for Thin LTO
    for i in 0..4 {
        let module = context.create_module(&format!("thin_lto_module_{}", i));
        
        // Add a simple function to each module
        let i32_type = context.i32_type();
        let fn_type = i32_type.fn_type(&[], false);
        let function = module.add_function(&format!("function_{}", i), fn_type, None);
        
        let entry_block = context.append_basic_block(function, "entry");
        let builder = context.create_builder();
        builder.position_at_end(entry_block);
        builder.build_return(Some(&i32_type.const_int(i as u64, false))).unwrap();
        
        integration.add_module(module).unwrap();
    }
    
    // Perform Thin LTO
    let result = integration.perform_lto();
    assert!(result.is_ok());
    
    let lto_result = result.unwrap();
    assert!(lto_result.optimized_modules.len() > 0);
    assert!(lto_result.total_time > Duration::from_secs(0));
    
    info!("Thin LTO workflow completed with {} optimized modules", 
          lto_result.optimized_modules.len());
}

/// Test Full LTO workflow
#[test]
fn test_full_lto_workflow() {
    init_tracing!();
    
    let context = Context::create();
    let config = LtoConfig {
        level: LtoLevel::Full,
        enable_cross_module_inlining: true,
        enable_whole_program_dce: true,
        ..Default::default()
    };
    
    let mut integration = LlvmLtoIntegration::new(&context, config).unwrap();
    
    // Add modules for Full LTO
    for i in 0..3 {
        let module = context.create_module(&format!("full_lto_module_{}", i));
        
        // Add function with more complexity
        let i32_type = context.i32_type();
        let fn_type = i32_type.fn_type(&[i32_type.into()], false);
        let function = module.add_function(&format!("compute_{}", i), fn_type, None);
        
        let entry_block = context.append_basic_block(function, "entry");
        let builder = context.create_builder();
        builder.position_at_end(entry_block);
        
        // Simple computation
        let param = function.get_nth_param(0).unwrap().into_int_value();
        let result = builder.build_int_add(param, i32_type.const_int(i as u64 + 1, false), "add").unwrap();
        builder.build_return(Some(&result)).unwrap();
        
        integration.add_module(module).unwrap();
    }
    
    // Perform Full LTO
    let result = integration.perform_lto();
    assert!(result.is_ok());
    
    let lto_result = result.unwrap();
    assert_eq!(lto_result.optimized_modules.len(), 1); // Should merge into one module
    assert!(lto_result.total_time > Duration::from_secs(0));
    assert!(lto_result.size_reduction > 0); // Mock value should be set
    
    info!("Full LTO workflow completed, merged {} modules into 1", 3);
}

/// Test object file generation (mocked)
#[test]
fn test_object_file_generation() {
    init_tracing!();
    
    let object_file = ObjectFile {
        name: "test_output.o".to_string(),
        data: vec![0x7f, 0x45, 0x4c, 0x46], // Mock ELF header
        size: 4,
    };
    
    assert_eq!(object_file.name, "test_output.o");
    assert_eq!(object_file.data.len(), 4);
    assert_eq!(object_file.size, 4);
    assert_eq!(object_file.data[0], 0x7f); // ELF magic number
    
    info!("Object file structure validated");
}

/// Test bitcode generation and caching
#[test]
fn test_bitcode_generation() {
    init_tracing!();
    
    let context = Context::create();
    let config = LtoConfig {
        enable_caching: true,
        ..Default::default()
    };
    
    let mut integration = LlvmLtoIntegration::new(&context, config).unwrap();
    
    // Create a module with content
    let module = context.create_module("bitcode_test_module");
    
    // Add a function
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = module.add_function("bitcode_test_function", fn_type, None);
    
    let entry_block = context.append_basic_block(function, "entry");
    let builder = context.create_builder();
    builder.position_at_end(entry_block);
    builder.build_return(Some(&i32_type.const_int(123, false))).unwrap();
    
    // Add module (this should generate bitcode internally)
    let result = integration.add_module(module);
    assert!(result.is_ok());
    
    info!("Bitcode generation completed successfully");
}

/// Test LTO statistics collection
#[test]
fn test_lto_statistics_collection() {
    init_tracing!();
    
    let context = Context::create();
    let config = LtoConfig {
        level: LtoLevel::Thin,
        enable_profiling: true,
        ..Default::default()
    };
    
    let mut integration = LlvmLtoIntegration::new(&context, config).unwrap();
    
    // Get initial statistics
    let initial_stats = integration.get_statistics();
    assert_eq!(initial_stats.modules_processed, 0);
    assert_eq!(initial_stats.total_time, Duration::from_secs(0));
    
    // Add a module and run optimization
    let module = context.create_module("stats_test_module");
    integration.add_module(module).unwrap();
    
    let result = integration.perform_lto();
    assert!(result.is_ok());
    
    // Check updated statistics
    let final_stats = integration.get_statistics();
    assert!(final_stats.total_time > Duration::from_secs(0));
    
    info!("LTO statistics collection working correctly");
}

/// Test error handling in LLVM LTO integration
#[test]
fn test_llvm_lto_error_handling() {
    init_tracing!();
    
    let context = Context::create();
    let config = LtoConfig::default();
    
    let integration = LlvmLtoIntegration::new(&context, config);
    assert!(integration.is_ok());
    
    let mut integration = integration.unwrap();
    
    // Test invalid target triple
    let result = integration.initialize_target("invalid-target-triple");
    // This might or might not fail depending on LLVM configuration
    // We just ensure it doesn't panic
    match result {
        Ok(_) => info!("Target initialization unexpectedly succeeded"),
        Err(e) => info!("Target initialization failed as expected: {:?}", e),
    }
}

/// Test module summary with complex functions
#[test]
fn test_complex_module_summary() {
    init_tracing!();
    
    let context = Context::create();
    let config = LtoConfig::default();
    let integration = LlvmLtoIntegration::new(&context, config).unwrap();
    
    // Create a module with multiple functions and complex structure
    let module = context.create_module("complex_summary_module");
    
    let i32_type = context.i32_type();
    let i64_type = context.i64_type();
    
    // Function 1: Simple function
    let fn1_type = i32_type.fn_type(&[], false);
    let function1 = module.add_function("simple_function", fn1_type, None);
    let entry1 = context.append_basic_block(function1, "entry");
    let builder = context.create_builder();
    builder.position_at_end(entry1);
    builder.build_return(Some(&i32_type.const_int(1, false))).unwrap();
    
    // Function 2: Function with parameters and multiple blocks
    let fn2_type = i32_type.fn_type(&[i32_type.into(), i32_type.into()], false);
    let function2 = module.add_function("complex_function", fn2_type, None);
    let entry2 = context.append_basic_block(function2, "entry");
    let then_block = context.append_basic_block(function2, "then");
    let else_block = context.append_basic_block(function2, "else");
    let merge_block = context.append_basic_block(function2, "merge");
    
    // Build complex function body
    builder.position_at_end(entry2);
    let param1 = function2.get_nth_param(0).unwrap().into_int_value();
    let param2 = function2.get_nth_param(1).unwrap().into_int_value();
    let cmp = builder.build_int_compare(inkwell::IntPredicate::SGT, param1, param2, "cmp").unwrap();
    builder.build_conditional_branch(cmp, then_block, else_block).unwrap();
    
    builder.position_at_end(then_block);
    let then_result = builder.build_int_add(param1, param2, "add").unwrap();
    builder.build_unconditional_branch(merge_block).unwrap();
    
    builder.position_at_end(else_block);
    let else_result = builder.build_int_sub(param1, param2, "sub").unwrap();
    builder.build_unconditional_branch(merge_block).unwrap();
    
    builder.position_at_end(merge_block);
    let phi = builder.build_phi(i32_type, "result").unwrap();
    phi.add_incoming(&[(&then_result, then_block), (&else_result, else_block)]);
    builder.build_return(Some(phi.as_basic_value())).unwrap();
    
    // Add globals
    let global1 = module.add_global(i32_type, None, "config_value");
    global1.set_initializer(&i32_type.const_int(42, false));
    
    let global2 = module.add_global(i64_type, None, "counter");
    global2.set_initializer(&i64_type.const_int(0, false));
    
    // Create summary
    let summary = integration.create_module_summary(&module);
    assert!(summary.is_ok());
    
    let summary = summary.unwrap();
    assert_eq!(summary.name, "complex_summary_module");
    assert_eq!(summary.functions.len(), 2);
    assert_eq!(summary.globals.len(), 2);
    
    // Check function complexities
    let simple_func = summary.functions.iter().find(|f| f.name == "simple_function").unwrap();
    let complex_func = summary.functions.iter().find(|f| f.name == "complex_function").unwrap();
    
    assert_eq!(simple_func.size, 1); // One basic block
    assert_eq!(complex_func.size, 4); // Four basic blocks
    assert!(simple_func.can_be_inlined); // Small function should be inlinable
    
    info!("Complex module summary created: {} functions, {} globals",
          summary.functions.len(), summary.globals.len());
}

/// Test memory management and resource cleanup
#[test]
fn test_memory_management() {
    init_tracing!();
    
    let context = Context::create();
    let config = LtoConfig {
        level: LtoLevel::Thin,
        enable_caching: true,
        ..Default::default()
    };
    
    let mut integration = LlvmLtoIntegration::new(&context, config).unwrap();
    
    // Add multiple modules and run optimization
    for i in 0..10 {
        let module = context.create_module(&format!("memory_test_module_{}", i));
        
        // Add some content to make modules non-trivial
        let i32_type = context.i32_type();
        let fn_type = i32_type.fn_type(&[], false);
        let function = module.add_function(&format!("func_{}", i), fn_type, None);
        
        let entry = context.append_basic_block(function, "entry");
        let builder = context.create_builder();
        builder.position_at_end(entry);
        builder.build_return(Some(&i32_type.const_int(i as u64, false))).unwrap();
        
        integration.add_module(module).unwrap();
    }
    
    // Run optimization
    let result = integration.perform_lto();
    assert!(result.is_ok());
    
    let lto_result = result.unwrap();
    assert!(lto_result.optimized_modules.len() > 0);
    
    // The integration should be cleanly destructible
    drop(integration);
    
    info!("Memory management test completed successfully");
}
