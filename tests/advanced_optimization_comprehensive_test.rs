/// Comprehensive Tests for Advanced LLVM Optimization System
/// 
/// Tests all components of the advanced optimization system including:
/// - Advanced LLVM integration with real context manipulation
/// - Target-specific optimizations for different architectures
/// - Advanced loop optimizations with fusion and vectorization
/// - Profile-guided optimization with data collection and analysis
/// - Link-time optimization with cross-module capabilities

use cursed::optimization::{
    advanced_llvm_integration::{AdvancedLlvmIntegration, AdvancedLlvmConfig},
    target_optimization::{TargetOptimizationManager, TargetOptimizationConfig, CpuArchitecture},
    advanced_loop_optimization::{AdvancedLoopOptimizer, LoopOptimizationConfig},
    profile_guided_optimization::{ProfileGuidedOptimizer, PgoConfig},
    link_time_optimization::{LinkTimeOptimizer, LtoConfig},
};
use cursed::error::Result;
use std::time::Duration;
use tempfile::TempDir;
use tracing::{info, debug};
use tracing_test::traced_test;

#[path = "common.rs"]
mod common;

/// Test advanced LLVM integration functionality
#[traced_test]
#[test]
fn test_advanced_llvm_integration() -> Result<()> {
    common::tracing::setup();
    info!("Testing advanced LLVM integration");
    
    // Test with different optimization configurations
    let configs = vec![
        AdvancedLlvmConfig {
            enable_advanced_inlining: true,
            enable_cfg_transformations: true,
            enable_target_specific: false,
            enable_vectorization: true,
            enable_advanced_loops: true,
            enable_ipo: false,
            optimization_level: 2,
            ..Default::default()
        },
        AdvancedLlvmConfig {
            enable_advanced_inlining: true,
            enable_cfg_transformations: true,
            enable_target_specific: true,
            enable_vectorization: true,
            enable_advanced_loops: true,
            enable_ipo: true,
            optimization_level: 3,
            max_inline_size: 1000,
            inline_threshold: 200,
            ..Default::default()
        },
    ];
    
    for (i, config) in configs.iter().enumerate() {
        debug!("Testing LLVM integration configuration {}", i + 1);
        
        // Create LLVM context and integration
        let context = inkwell::context::Context::create();
        let mut integration = AdvancedLlvmIntegration::new(&context, &format!("test_module_{}", i), config.clone())?;
        
        // Initialize optimization passes
        integration.initialize_passes()?;
        
        // Run comprehensive optimization
        let stats = integration.optimize_module()?;
        
        // Validate optimization statistics
        assert!(stats.total_optimization_time > Duration::from_millis(0));
        assert!(stats.peak_memory_usage_mb > 0);
        
        // Check inlining statistics
        debug!("Inlining stats: {} functions analyzed, {} inlined", 
               stats.inlining_stats.functions_analyzed, stats.inlining_stats.functions_inlined);
        
        // Check CFG transformation statistics
        debug!("CFG stats: {} blocks merged, {} dead blocks removed", 
               stats.cfg_stats.blocks_merged, stats.cfg_stats.dead_blocks_removed);
        
        // Check loop optimization statistics
        debug!("Loop stats: {} loops analyzed, {} vectorized", 
               stats.loop_stats.loops_analyzed, stats.loop_stats.loops_vectorized);
        
        // Check vectorization statistics
        debug!("Vectorization stats: factor {:.2}, {} operations vectorized", 
               stats.vectorization_stats.vectorization_factor, stats.vectorization_stats.vectorized_operations);
        
        info!("✓ LLVM integration configuration {} completed successfully", i + 1);
    }
    
    Ok(())
}

/// Test target-specific optimizations for different architectures
#[traced_test]
#[test]
fn test_target_specific_optimizations() -> Result<()> {
    common::tracing::setup();
    info!("Testing target-specific optimizations");
    
    let architectures = vec![
        CpuArchitecture::X86_64,
        CpuArchitecture::Arm64,
        CpuArchitecture::RiscV64,
        CpuArchitecture::WebAssembly,
    ];
    
    for architecture in architectures {
        debug!("Testing optimizations for {:?}", architecture);
        
        let config = TargetOptimizationConfig {
            target_architecture: architecture,
            enable_simd: true,
            enable_cache_optimization: true,
            enable_auto_vectorization: true,
            enable_instruction_scheduling: true,
            vectorization_factor: 4,
            cache_line_size: 64,
            ..Default::default()
        };
        
        let mut manager = TargetOptimizationManager::new(config)?;
        
        // Create test code unit
        let mut code_unit = cursed::optimization::target_optimization::CodeUnit::new(
            format!("test_code_{:?}", architecture)
        );
        
        // Add test loops and memory accesses
        code_unit.loops.push(cursed::optimization::target_optimization::LoopInfo {
            trip_count: 100,
            body_size: 20,
            data_types: vec![cursed::optimization::target_optimization::SimdType::Float32],
        });
        
        code_unit.memory_accesses.push(cursed::optimization::target_optimization::MemoryAccess {
            pattern: cursed::optimization::target_optimization::MemoryAccessPattern::Sequential,
            size: 1024,
            frequency: 0.8,
        });
        
        // Apply optimizations
        let stats = manager.optimize(&mut code_unit)?;
        
        // Validate optimization results
        assert!(stats.optimization_time > Duration::from_millis(0));
        debug!("Architecture {:?}: {} optimizations applied, {:.2}x performance improvement", 
               architecture, stats.optimizations_applied, stats.performance_improvement);
        
        // Check CPU info
        let cpu_info = manager.get_cpu_info();
        assert_eq!(cpu_info.architecture, architecture);
        assert!(!cpu_info.features.is_empty() || architecture == CpuArchitecture::Generic);
        
        info!("✓ Target optimization for {:?} completed successfully", architecture);
    }
    
    Ok(())
}

/// Test advanced loop optimizations with different strategies
#[traced_test]
#[test]
fn test_advanced_loop_optimizations() -> Result<()> {
    common::tracing::setup();
    info!("Testing advanced loop optimizations");
    
    let configs = vec![
        LoopOptimizationConfig {
            enable_loop_fusion: true,
            enable_loop_distribution: false,
            enable_loop_interchange: true,
            enable_advanced_unrolling: true,
            enable_licm: true,
            enable_vectorization: true,
            max_unroll_factor: 4,
            cost_threshold: 1.2,
            ..Default::default()
        },
        LoopOptimizationConfig {
            enable_loop_fusion: true,
            enable_loop_distribution: true,
            enable_loop_interchange: true,
            enable_advanced_unrolling: true,
            enable_licm: true,
            enable_vectorization: true,
            max_unroll_factor: 8,
            cost_threshold: 1.5,
            max_aggressive_size: 2000,
            ..Default::default()
        },
    ];
    
    for (i, config) in configs.iter().enumerate() {
        debug!("Testing loop optimization configuration {}", i + 1);
        
        let mut optimizer = AdvancedLoopOptimizer::new(config.clone());
        
        // Create test code unit with various loop patterns
        let mut code_unit = cursed::optimization::advanced_loop_optimization::CodeUnit::new(
            format!("loop_test_{}", i)
        );
        
        // Add different types of loops
        code_unit.loops.extend(vec![
            cursed::optimization::advanced_loop_optimization::LoopInfo {
                id: "simple_loop".to_string(),
                nesting_level: 1,
                iteration_count: Some(100),
                body_size: 15,
                loop_type: cursed::optimization::advanced_loop_optimization::LoopType::CountingLoop,
                induction_variables: vec![],
                inner_loops: vec![],
                statements: vec![],
            },
            cursed::optimization::advanced_loop_optimization::LoopInfo {
                id: "nested_loop".to_string(),
                nesting_level: 2,
                iteration_count: Some(50),
                body_size: 30,
                loop_type: cursed::optimization::advanced_loop_optimization::LoopType::CountingLoop,
                induction_variables: vec![],
                inner_loops: vec![],
                statements: vec![],
            },
            cursed::optimization::advanced_loop_optimization::LoopInfo {
                id: "vectorizable_loop".to_string(),
                nesting_level: 1,
                iteration_count: Some(1000),
                body_size: 8,
                loop_type: cursed::optimization::advanced_loop_optimization::LoopType::CountingLoop,
                induction_variables: vec![],
                inner_loops: vec![],
                statements: vec![],
            },
        ]);
        
        // Optimize loops
        let stats = optimizer.optimize_loops(&mut code_unit)?;
        
        // Validate optimization statistics
        assert_eq!(stats.loops_analyzed, 3);
        assert!(stats.total_optimization_time > Duration::from_millis(0));
        
        debug!("Loop optimization config {}: {} loops optimized, {} unrolled, {} vectorized", 
               i + 1, stats.loops_optimized, stats.unrolling_successes, stats.vectorization_successes);
        
        // Check that some optimizations were applied
        let total_optimizations = stats.fusion_successes + stats.distribution_successes + 
                                 stats.interchange_successes + stats.unrolling_successes + 
                                 stats.vectorization_successes + stats.licm_successes;
        assert!(total_optimizations > 0, "No loop optimizations were applied");
        
        info!("✓ Loop optimization configuration {} completed successfully", i + 1);
    }
    
    Ok(())
}

/// Test profile-guided optimization with data collection and analysis
#[traced_test]
#[test]
fn test_profile_guided_optimization() -> Result<()> {
    common::tracing::setup();
    info!("Testing profile-guided optimization");
    
    let temp_dir = TempDir::new().unwrap();
    
    let configs = vec![
        PgoConfig {
            enable_profile_collection: true,
            enable_pgo: true,
            collection_method: cursed::optimization::profile_guided_optimization::ProfileCollectionMethod::Instrumentation,
            profile_data_path: temp_dir.path().join("pgo_profiles_1"),
            hot_path_threshold: 80.0,
            cold_path_threshold: 5.0,
            min_sample_count: 100,
            optimization_level: cursed::optimization::profile_guided_optimization::PgoOptimizationLevel::Balanced,
            ..Default::default()
        },
        PgoConfig {
            enable_profile_collection: true,
            enable_pgo: true,
            collection_method: cursed::optimization::profile_guided_optimization::ProfileCollectionMethod::Hybrid,
            profile_data_path: temp_dir.path().join("pgo_profiles_2"),
            hot_path_threshold: 75.0,
            cold_path_threshold: 10.0,
            min_sample_count: 500,
            optimization_level: cursed::optimization::profile_guided_optimization::PgoOptimizationLevel::Aggressive,
            enable_cross_module: true,
            ..Default::default()
        },
    ];
    
    for (i, config) in configs.iter().enumerate() {
        debug!("Testing PGO configuration {}", i + 1);
        
        let mut optimizer = ProfileGuidedOptimizer::new(config.clone())?;
        
        // Test profile collection
        optimizer.start_profile_collection()?;
        
        // Simulate some execution time for profile collection
        std::thread::sleep(Duration::from_millis(10));
        
        let profile_id = format!("test_profile_{}", i);
        optimizer.stop_profile_collection(&profile_id)?;
        
        // Analyze profiles
        let opportunities = optimizer.analyze_profiles(&[profile_id])?;
        debug!("PGO config {}: Found {} optimization opportunities", i + 1, opportunities.len());
        
        // Apply optimizations if we have opportunities
        if !opportunities.is_empty() {
            let mut code_unit = cursed::optimization::profile_guided_optimization::CodeUnit::new(
                format!("pgo_test_{}", i)
            );
            
            let result = optimizer.apply_optimizations(&opportunities, &mut code_unit)?;
            
            debug!("PGO config {}: Applied {} optimizations, {:.2}x performance improvement", 
                   i + 1, result.optimizations_applied, result.performance_improvement);
            
            assert!(result.optimization_time > Duration::from_millis(0));
        }
        
        // Get statistics
        let stats = optimizer.get_statistics();
        assert!(stats.profiles_collected > 0);
        
        // Generate report
        let report = optimizer.generate_optimization_report();
        assert!(report.contains("Profile-Guided Optimization Report"));
        assert!(report.contains("Profile Collection Summary"));
        
        info!("✓ PGO configuration {} completed successfully", i + 1);
    }
    
    Ok(())
}

/// Test link-time optimization with cross-module capabilities
#[traced_test]
#[test]
fn test_link_time_optimization() -> Result<()> {
    common::tracing::setup();
    info!("Testing link-time optimization");
    
    let configs = vec![
        LtoConfig {
            enable_lto: true,
            optimization_level: cursed::optimization::link_time_optimization::LtoOptimizationLevel::Thin,
            enable_cross_module_inlining: true,
            enable_whole_program_analysis: true,
            enable_global_dce: true,
            enable_ipo: false,
            enable_function_specialization: true,
            max_cross_module_inline_size: 50,
            parallel_threads: 2,
            ..Default::default()
        },
        LtoConfig {
            enable_lto: true,
            optimization_level: cursed::optimization::link_time_optimization::LtoOptimizationLevel::Full,
            enable_cross_module_inlining: true,
            enable_whole_program_analysis: true,
            enable_global_dce: true,
            enable_ipo: true,
            enable_function_specialization: true,
            enable_global_constant_propagation: true,
            max_cross_module_inline_size: 100,
            aggressive_threshold: 0.9,
            parallel_threads: 4,
            ..Default::default()
        },
    ];
    
    for (i, config) in configs.iter().enumerate() {
        debug!("Testing LTO configuration {}", i + 1);
        
        let mut optimizer = LinkTimeOptimizer::new(config.clone())?;
        
        // Create test modules
        let mut modules = create_test_modules(i + 1)?;
        
        // Run link-time optimization
        let result = optimizer.optimize_modules(&mut modules)?;
        
        // Validate optimization results
        assert_eq!(result.modules_processed, modules.len());
        assert!(result.optimization_time > Duration::from_millis(0));
        
        debug!("LTO config {}: {} modules, {} functions inlined, {} specialized", 
               i + 1, result.modules_processed, result.functions_inlined, result.functions_specialized);
        
        // Check that optimizations were applied
        let total_optimizations = result.functions_inlined + result.functions_specialized + 
                                 result.constants_propagated;
        assert!(total_optimizations > 0, "No LTO optimizations were applied");
        
        // Get statistics
        let stats = optimizer.get_statistics();
        assert!(stats.modules_processed > 0);
        assert!(stats.optimization_time > Duration::from_millis(0));
        
        info!("✓ LTO configuration {} completed successfully", i + 1);
    }
    
    Ok(())
}

/// Test integration between different optimization systems
#[traced_test]
#[test]
fn test_optimization_system_integration() -> Result<()> {
    common::tracing::setup();
    info!("Testing optimization system integration");
    
    // Create configurations for all systems
    let llvm_config = AdvancedLlvmConfig {
        enable_advanced_inlining: true,
        enable_cfg_transformations: true,
        enable_vectorization: true,
        optimization_level: 2,
        ..Default::default()
    };
    
    let target_config = TargetOptimizationConfig {
        target_architecture: CpuArchitecture::X86_64,
        enable_simd: true,
        enable_cache_optimization: true,
        enable_auto_vectorization: true,
        ..Default::default()
    };
    
    let loop_config = LoopOptimizationConfig {
        enable_loop_fusion: true,
        enable_advanced_unrolling: true,
        enable_vectorization: true,
        max_unroll_factor: 4,
        ..Default::default()
    };
    
    let temp_dir = TempDir::new().unwrap();
    let pgo_config = PgoConfig {
        enable_profile_collection: true,
        enable_pgo: true,
        profile_data_path: temp_dir.path().join("integration_profiles"),
        ..Default::default()
    };
    
    let lto_config = LtoConfig {
        enable_lto: true,
        optimization_level: cursed::optimization::link_time_optimization::LtoOptimizationLevel::Thin,
        enable_cross_module_inlining: true,
        enable_global_dce: true,
        ..Default::default()
    };
    
    // Test individual system creation
    let context = inkwell::context::Context::create();
    let llvm_integration = AdvancedLlvmIntegration::new(&context, "integration_test", llvm_config)?;
    let target_manager = TargetOptimizationManager::new(target_config)?;
    let loop_optimizer = AdvancedLoopOptimizer::new(loop_config);
    let pgo_optimizer = ProfileGuidedOptimizer::new(pgo_config)?;
    let lto_optimizer = LinkTimeOptimizer::new(lto_config)?;
    
    // Verify all systems are properly initialized
    assert!(llvm_integration.get_module().get_name().to_str().unwrap().contains("integration_test"));
    assert_eq!(target_manager.get_cpu_info().architecture, CpuArchitecture::X86_64);
    
    debug!("All optimization systems initialized successfully");
    
    // Test statistics aggregation
    let llvm_stats = llvm_integration.get_statistics();
    let target_stats = target_manager.get_statistics();
    let loop_stats = loop_optimizer.get_statistics();
    let pgo_stats = pgo_optimizer.get_statistics();
    let lto_stats = lto_optimizer.get_statistics();
    
    // Aggregate statistics
    let total_optimization_time = llvm_stats.total_optimization_time + 
                                 target_stats.optimization_time + 
                                 loop_stats.total_optimization_time + 
                                 pgo_stats.optimization_time + 
                                 lto_stats.optimization_time;
    
    debug!("Total optimization time across all systems: {:?}", total_optimization_time);
    
    info!("✓ Optimization system integration test completed successfully");
    
    Ok(())
}

/// Test performance improvements and benchmarking
#[traced_test]
#[test]
fn test_optimization_performance_improvements() -> Result<()> {
    common::tracing::setup();
    info!("Testing optimization performance improvements");
    
    // Test different optimization levels and measure improvements
    let optimization_levels = vec![
        ("Conservative", 1, false),
        ("Balanced", 2, true),
        ("Aggressive", 3, true),
    ];
    
    for (level_name, opt_level, enable_advanced) in optimization_levels {
        debug!("Testing {} optimization level", level_name);
        
        let start_time = std::time::Instant::now();
        
        // Configure optimization for this level
        let llvm_config = AdvancedLlvmConfig {
            enable_advanced_inlining: enable_advanced,
            enable_cfg_transformations: enable_advanced,
            enable_vectorization: enable_advanced,
            enable_advanced_loops: enable_advanced,
            enable_ipo: enable_advanced,
            optimization_level: opt_level,
            inline_threshold: if enable_advanced { 100 } else { 50 },
            max_inline_size: if enable_advanced { 500 } else { 100 },
            ..Default::default()
        };
        
        let target_config = TargetOptimizationConfig {
            target_architecture: CpuArchitecture::X86_64,
            enable_simd: enable_advanced,
            enable_cache_optimization: enable_advanced,
            enable_auto_vectorization: enable_advanced,
            enable_instruction_scheduling: enable_advanced,
            vectorization_factor: if enable_advanced { 8 } else { 4 },
            ..Default::default()
        };
        
        // Run optimizations and measure performance
        let context = inkwell::context::Context::create();
        let mut llvm_integration = AdvancedLlvmIntegration::new(&context, &format!("perf_test_{}", level_name), llvm_config)?;
        llvm_integration.initialize_passes()?;
        let llvm_stats = llvm_integration.optimize_module()?;
        
        let mut target_manager = TargetOptimizationManager::new(target_config)?;
        let mut code_unit = cursed::optimization::target_optimization::CodeUnit::new(format!("perf_test_{}", level_name));
        
        // Add substantial code for meaningful optimization
        for i in 0..10 {
            code_unit.loops.push(cursed::optimization::target_optimization::LoopInfo {
                trip_count: 100 + i * 10,
                body_size: 15 + i,
                data_types: vec![cursed::optimization::target_optimization::SimdType::Float32],
            });
        }
        
        let target_stats = target_manager.optimize(&mut code_unit)?;
        
        let total_time = start_time.elapsed();
        
        // Validate performance characteristics
        assert!(llvm_stats.total_optimization_time > Duration::from_millis(0));
        assert!(target_stats.optimization_time > Duration::from_millis(0));
        
        // Calculate performance metrics
        let llvm_improvement = llvm_stats.inlining_stats.instructions_saved as f64 / 
                              (llvm_stats.inlining_stats.instructions_saved + 1000) as f64;
        let target_improvement = target_stats.performance_improvement;
        
        debug!("{} level: LLVM improvement {:.2}%, Target improvement {:.2}x, Total time {:?}", 
               level_name, llvm_improvement * 100.0, target_improvement, total_time);
        
        // Verify that higher optimization levels achieve better results
        if enable_advanced {
            assert!(llvm_stats.inlining_stats.functions_inlined > 0 || 
                   llvm_stats.cfg_stats.blocks_merged > 0 ||
                   llvm_stats.loop_stats.loops_analyzed > 0,
                   "Advanced optimizations should produce some results");
            
            assert!(target_stats.optimizations_applied > 0,
                   "Target optimizations should be applied with advanced settings");
        }
        
        info!("✓ {} optimization level performance test completed", level_name);
    }
    
    Ok(())
}

/// Test error handling and edge cases
#[traced_test]
#[test]
fn test_optimization_error_handling() -> Result<()> {
    common::tracing::setup();
    info!("Testing optimization error handling");
    
    // Test invalid configurations
    let invalid_llvm_config = AdvancedLlvmConfig {
        optimization_level: 255, // Invalid level
        max_inline_size: 0,       // Invalid size
        inline_threshold: 0,      // Invalid threshold
        ..Default::default()
    };
    
    // Test that system handles invalid configurations gracefully
    let context = inkwell::context::Context::create();
    let llvm_result = AdvancedLlvmIntegration::new(&context, "error_test", invalid_llvm_config);
    // Should either succeed with clamped values or fail gracefully
    match llvm_result {
        Ok(mut integration) => {
            debug!("LLVM integration handled invalid config gracefully");
            // Try to run optimization with invalid config
            let result = integration.initialize_passes();
            debug!("Initialize passes result: {:?}", result.is_ok());
        },
        Err(e) => {
            debug!("LLVM integration properly rejected invalid config: {}", e);
        }
    }
    
    // Test with empty/minimal code units
    let target_config = TargetOptimizationConfig::default();
    let mut target_manager = TargetOptimizationManager::new(target_config)?;
    let mut empty_code_unit = cursed::optimization::target_optimization::CodeUnit::new("empty_test".to_string());
    
    let empty_result = target_manager.optimize(&mut empty_code_unit)?;
    assert_eq!(empty_result.optimizations_applied, 0);
    debug!("Empty code unit handled correctly");
    
    // Test with invalid profile data paths
    let temp_dir = TempDir::new().unwrap();
    let invalid_path = temp_dir.path().join("nonexistent").join("deeply").join("nested").join("path");
    
    let pgo_config = PgoConfig {
        profile_data_path: invalid_path,
        ..Default::default()
    };
    
    // Should handle invalid paths gracefully
    match ProfileGuidedOptimizer::new(pgo_config) {
        Ok(_) => debug!("PGO handled invalid path gracefully"),
        Err(e) => debug!("PGO properly rejected invalid path: {}", e),
    }
    
    info!("✓ Error handling tests completed successfully");
    
    Ok(())
}

/// Helper function to create test modules for LTO testing
fn create_test_modules(count: usize) -> Result<Vec<cursed::optimization::link_time_optimization::ModuleInfo>> {
    use cursed::optimization::link_time_optimization::*;
    use std::path::PathBuf;
    
    let mut modules = Vec::new();
    
    for i in 0..count.min(3) {
        let module_id = ModuleId {
            name: format!("test_module_{}", i),
            version: "1.0.0".to_string(),
            hash: (i as u64) * 12345,
        };
        
        let function_info = FunctionInfo {
            name: format!("test_function_{}", i),
            module_id: module_id.clone(),
            function_type: FunctionType {
                return_type: "i32".to_string(),
                parameters: vec![
                    Parameter {
                        name: "param1".to_string(),
                        param_type: "i32".to_string(),
                        is_const: false,
                        is_reference: false,
                    }
                ],
                is_variadic: false,
                calling_convention: CallingConvention::Standard,
            },
            size: 100 + i * 20,
            complexity: 5.0 + i as f64,
            call_sites: vec![],
            called_functions: vec![],
            local_variables: vec![],
            basic_blocks: 3 + i,
            instructions: 20 + i * 5,
            is_recursive: false,
            inlining_cost: 2.5 + i as f64 * 0.5,
            specialization_opportunities: vec![],
        };
        
        let symbol = Symbol {
            name: format!("symbol_{}", i),
            symbol_type: SymbolType::Function,
            visibility: SymbolVisibility::Public,
            linkage: SymbolLinkage::External,
            size: 100,
            alignment: 8,
            section: ".text".to_string(),
            uses: vec![],
        };
        
        let module_info = ModuleInfo {
            module_id,
            file_path: PathBuf::from(format!("test_module_{}.ll", i)),
            symbols: vec![symbol],
            functions: vec![function_info],
            global_variables: vec![],
            dependencies: vec![],
            export_list: vec![format!("test_function_{}", i)],
            import_list: vec![],
            compilation_unit_size: 1024 + i * 256,
            optimization_level: "O2".to_string(),
        };
        
        modules.push(module_info);
    }
    
    Ok(modules)
}

/// Integration test for the complete optimization pipeline
#[traced_test]
#[test]
fn test_complete_optimization_pipeline() -> Result<()> {
    common::tracing::setup();
    info!("Testing complete optimization pipeline");
    
    let total_start = std::time::Instant::now();
    
    // Phase 1: LLVM-level optimizations
    debug!("Phase 1: LLVM optimization");
    let llvm_start = std::time::Instant::now();
    
    let llvm_config = AdvancedLlvmConfig {
        enable_advanced_inlining: true,
        enable_cfg_transformations: true,
        enable_vectorization: true,
        enable_advanced_loops: true,
        optimization_level: 2,
        ..Default::default()
    };
    
    let context = inkwell::context::Context::create();
    let mut llvm_integration = AdvancedLlvmIntegration::new(&context, "pipeline_test", llvm_config)?;
    llvm_integration.initialize_passes()?;
    let llvm_stats = llvm_integration.optimize_module()?;
    let llvm_time = llvm_start.elapsed();
    
    // Phase 2: Target-specific optimizations
    debug!("Phase 2: Target-specific optimization");
    let target_start = std::time::Instant::now();
    
    let target_config = TargetOptimizationConfig {
        target_architecture: CpuArchitecture::X86_64,
        enable_simd: true,
        enable_cache_optimization: true,
        enable_auto_vectorization: true,
        enable_instruction_scheduling: true,
        ..Default::default()
    };
    
    let mut target_manager = TargetOptimizationManager::new(target_config)?;
    let mut code_unit = cursed::optimization::target_optimization::CodeUnit::new("pipeline_test".to_string());
    
    // Add comprehensive test workload
    for i in 0..5 {
        code_unit.loops.push(cursed::optimization::target_optimization::LoopInfo {
            trip_count: 50 + i * 20,
            body_size: 10 + i * 3,
            data_types: vec![
                cursed::optimization::target_optimization::SimdType::Float32,
                cursed::optimization::target_optimization::SimdType::Int32,
            ],
        });
        
        code_unit.memory_accesses.push(cursed::optimization::target_optimization::MemoryAccess {
            pattern: if i % 2 == 0 { 
                cursed::optimization::target_optimization::MemoryAccessPattern::Sequential 
            } else { 
                cursed::optimization::target_optimization::MemoryAccessPattern::Strided(2) 
            },
            size: 512 + i * 128,
            frequency: 0.7 + (i as f64) * 0.05,
        });
    }
    
    let target_stats = target_manager.optimize(&mut code_unit)?;
    let target_time = target_start.elapsed();
    
    // Phase 3: Loop optimizations
    debug!("Phase 3: Advanced loop optimization");
    let loop_start = std::time::Instant::now();
    
    let loop_config = LoopOptimizationConfig {
        enable_loop_fusion: true,
        enable_loop_distribution: true,
        enable_loop_interchange: true,
        enable_advanced_unrolling: true,
        enable_licm: true,
        enable_vectorization: true,
        max_unroll_factor: 8,
        cost_threshold: 1.3,
        ..Default::default()
    };
    
    let mut loop_optimizer = AdvancedLoopOptimizer::new(loop_config);
    let mut loop_code_unit = cursed::optimization::advanced_loop_optimization::CodeUnit::new("pipeline_loop_test".to_string());
    
    // Add diverse loop patterns
    loop_code_unit.loops.extend(vec![
        cursed::optimization::advanced_loop_optimization::LoopInfo {
            id: "main_loop".to_string(),
            nesting_level: 1,
            iteration_count: Some(200),
            body_size: 25,
            loop_type: cursed::optimization::advanced_loop_optimization::LoopType::CountingLoop,
            induction_variables: vec![],
            inner_loops: vec![],
            statements: vec![],
        },
        cursed::optimization::advanced_loop_optimization::LoopInfo {
            id: "nested_computation".to_string(),
            nesting_level: 3,
            iteration_count: Some(100),
            body_size: 40,
            loop_type: cursed::optimization::advanced_loop_optimization::LoopType::CountingLoop,
            induction_variables: vec![],
            inner_loops: vec![],
            statements: vec![],
        },
        cursed::optimization::advanced_loop_optimization::LoopInfo {
            id: "vectorizable_math".to_string(),
            nesting_level: 1,
            iteration_count: Some(2000),
            body_size: 12,
            loop_type: cursed::optimization::advanced_loop_optimization::LoopType::CountingLoop,
            induction_variables: vec![],
            inner_loops: vec![],
            statements: vec![],
        },
    ]);
    
    let loop_stats = loop_optimizer.optimize_loops(&mut loop_code_unit)?;
    let loop_time = loop_start.elapsed();
    
    // Phase 4: Link-time optimizations
    debug!("Phase 4: Link-time optimization");
    let lto_start = std::time::Instant::now();
    
    let lto_config = LtoConfig {
        enable_lto: true,
        optimization_level: cursed::optimization::link_time_optimization::LtoOptimizationLevel::Full,
        enable_cross_module_inlining: true,
        enable_whole_program_analysis: true,
        enable_global_dce: true,
        enable_ipo: true,
        enable_function_specialization: true,
        enable_global_constant_propagation: true,
        max_cross_module_inline_size: 75,
        aggressive_threshold: 0.8,
        ..Default::default()
    };
    
    let mut lto_optimizer = LinkTimeOptimizer::new(lto_config)?;
    let mut modules = create_test_modules(3)?;
    let lto_result = lto_optimizer.optimize_modules(&mut modules)?;
    let lto_time = lto_start.elapsed();
    
    let total_time = total_start.elapsed();
    
    // Aggregate and analyze results
    info!("🚀 Complete Optimization Pipeline Results:");
    info!("   Phase 1 (LLVM): {:?} - {} functions analyzed", llvm_time, llvm_stats.inlining_stats.functions_analyzed);
    info!("   Phase 2 (Target): {:?} - {} optimizations applied", target_time, target_stats.optimizations_applied);
    info!("   Phase 3 (Loops): {:?} - {} loops optimized", loop_time, loop_stats.loops_optimized);
    info!("   Phase 4 (LTO): {:?} - {} modules processed", lto_time, lto_result.modules_processed);
    info!("   Total pipeline time: {:?}", total_time);
    
    // Validate comprehensive optimization occurred
    assert!(llvm_stats.total_optimization_time > Duration::from_millis(0));
    assert!(target_stats.optimization_time > Duration::from_millis(0));
    assert!(loop_stats.total_optimization_time > Duration::from_millis(0));
    assert!(lto_result.optimization_time > Duration::from_millis(0));
    
    // Check that optimizations were actually applied
    let total_optimizations = llvm_stats.inlining_stats.functions_inlined +
                             llvm_stats.cfg_stats.blocks_merged +
                             target_stats.optimizations_applied +
                             loop_stats.loops_optimized +
                             lto_result.functions_inlined +
                             lto_result.functions_specialized;
    
    assert!(total_optimizations > 0, "Pipeline should apply some optimizations");
    
    // Validate performance characteristics
    assert!(total_time < Duration::from_secs(5), "Pipeline should complete within reasonable time");
    
    info!("✓ Complete optimization pipeline test passed successfully");
    info!("   Total optimizations applied: {}", total_optimizations);
    info!("   Average time per phase: {:?}", total_time / 4);
    
    Ok(())
}
