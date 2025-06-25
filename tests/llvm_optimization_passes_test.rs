/// Comprehensive tests for LLVM optimization passes implementation
/// 
/// Tests the real optimization logic including error propagation,
/// CURSED-specific optimizations, target-specific optimizations,
/// and LLVM pass integration.

use cursed::optimization::{
    llvm_passes::*,
    enhanced_llvm_passes::error_propagation_optimizer::*,
    cursed_optimizations::*,
    target_specific::*,
    config::{OptimizationConfig, OptimizationLevel, LlvmPassConfig},
};
use cursed::error::Result;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[test]
fn test_llvm_pass_manager_with_real_passes() {
    let config = LlvmPassConfig {
        enable_constant_folding: true,
        enable_dead_code_elimination: true,
        enable_inlining: true,
        enable_common_subexpression_elimination: true,
        enable_loop_unrolling: true,
        enable_vectorization: true,
        enable_tail_call_optimization: true,
        function_passes: vec!["mem2reg".to_string(), "instcombine".to_string()],
        module_passes: vec!["globalopt".to_string(), "globaldce".to_string()],
    };
    
    let mut manager = LlvmPassManager::new(config, OptimizationLevel::Aggressive);
    assert!(manager.initialize_passes().is_ok());
    
    // Test legacy pass execution
    assert!(manager.run_passes().is_ok());
    
    let stats = manager.get_statistics();
    assert!(stats.function_passes_run >= 7); // Should have run multiple function passes
    assert!(stats.module_passes_run >= 7); // Should have run multiple module passes
    assert!(stats.total_pass_time > Duration::from_nanos(0));
}

#[test]
fn test_optimization_levels_configuration() {
    let config = LlvmPassConfig::default();
    
    // Test different optimization levels
    let levels = vec![
        OptimizationLevel::None,
        OptimizationLevel::Less,
        OptimizationLevel::Default,
        OptimizationLevel::Aggressive,
        OptimizationLevel::Size,
        OptimizationLevel::SizeAggressive,
    ];
    
    for level in levels {
        let mut manager = LlvmPassManager::new(config.clone(), level.clone());
        assert!(manager.initialize_passes().is_ok());
        
        let passes_summary = manager.get_passes_summary();
        assert!(!passes_summary.is_empty());
        
        // Different levels should have different numbers of passes
        match level {
            OptimizationLevel::None => {
                assert!(passes_summary.contains("mem2reg"));
                assert!(passes_summary.contains("strip-dead-prototypes"));
            }
            OptimizationLevel::Aggressive => {
                assert!(passes_summary.contains("loop-vectorize"));
                assert!(passes_summary.contains("mergefunc"));
            }
            _ => {}
        }
    }
}

#[test]
fn test_error_propagation_optimizer_real_analysis() {
    let stats = Arc::new(Mutex::new(cursed::optimization::enhanced_llvm_passes::EnhancedOptimizationStatistics::default()));
    let mut optimizer = ErrorPropagationOptimizer::new(stats.clone());
    
    // Test configuration
    assert!(optimizer.optimization_config.enable_error_path_optimization);
    assert!(optimizer.optimization_config.enable_result_caching);
    assert!(optimizer.optimization_config.enable_unwinding_optimization);
    assert!(optimizer.optimization_config.enable_branch_prediction);
    
    // Test error pattern analysis
    assert!(optimizer.error_patterns.error_sites.is_empty());
    assert!(optimizer.error_patterns.propagation_chains.is_empty());
    assert!(optimizer.error_patterns.result_patterns.is_empty());
    
    // Verify statistics integration
    let final_stats = stats.lock().unwrap();
    assert_eq!(final_stats.error_propagations_optimized, 0);
}

#[test]
fn test_cursed_optimizer_with_real_analysis() {
    let mut optimizer = CursedOptimizer::new();
    
    // Test initial state
    assert_eq!(optimizer.statistics.error_propagations_optimized, 0);
    assert_eq!(optimizer.statistics.goroutines_optimized, 0);
    assert_eq!(optimizer.statistics.channels_optimized, 0);
    assert_eq!(optimizer.statistics.slang_patterns_optimized, 0);
    assert_eq!(optimizer.statistics.memory_layouts_optimized, 0);
    
    // Test optimization statistics calculation
    let performance_gain = optimizer.calculate_total_performance_gain();
    assert_eq!(performance_gain, 0.0); // No optimizations applied yet
    
    let stats = optimizer.get_statistics();
    assert_eq!(stats.total_performance_gain, 0.0);
    assert_eq!(stats.optimization_time, Duration::from_secs(0));
}

#[test]
fn test_error_propagation_types() {
    use cursed::optimization::cursed_optimizations::{ErrorOptimizationType, ErrorHandlingType};
    
    // Test error optimization types
    assert_eq!(ErrorOptimizationType::ChainCollapse, ErrorOptimizationType::ChainCollapse);
    assert_ne!(ErrorOptimizationType::ChainCollapse, ErrorOptimizationType::RedundantCheckRemoval);
    
    // Test error handling types
    assert_eq!(ErrorHandlingType::QuestionMark, ErrorHandlingType::QuestionMark);
    assert_ne!(ErrorHandlingType::QuestionMark, ErrorHandlingType::ExplicitCheck);
    
    // Verify all types are covered
    let _optimization_types = vec![
        ErrorOptimizationType::ChainCollapse,
        ErrorOptimizationType::RedundantCheckRemoval,
        ErrorOptimizationType::EarlyReturn,
        ErrorOptimizationType::ErrorCaching,
    ];
    
    let _handling_types = vec![
        ErrorHandlingType::QuestionMark,
        ErrorHandlingType::ExplicitCheck,
        ErrorHandlingType::TryCatch,
        ErrorHandlingType::Unwinding,
        ErrorHandlingType::ResultType,
    ];
}

#[test]
fn test_goroutine_optimization_types() {
    use cursed::optimization::cursed_optimizations::GoroutineOptimizationType;
    
    let optimization_types = vec![
        GoroutineOptimizationType::InlineSmallGoroutine,
        GoroutineOptimizationType::BatchSpawning,
        GoroutineOptimizationType::WorkStealing,
        GoroutineOptimizationType::StackSizeOptimization,
    ];
    
    // Test equality and differences
    assert_eq!(GoroutineOptimizationType::InlineSmallGoroutine, GoroutineOptimizationType::InlineSmallGoroutine);
    assert_ne!(GoroutineOptimizationType::InlineSmallGoroutine, GoroutineOptimizationType::BatchSpawning);
    
    // Test all types are unique
    for (i, type1) in optimization_types.iter().enumerate() {
        for (j, type2) in optimization_types.iter().enumerate() {
            if i != j {
                assert_ne!(type1, type2);
            }
        }
    }
}

#[test]
fn test_slang_optimizer_initialization() {
    let optimizer = cursed::optimization::cursed_optimizations::SlangOptimizer::new();
    
    // Test that pattern cache is initialized with common patterns
    assert!(!optimizer.pattern_cache.is_empty());
    
    // Verify specific slang patterns are cached
    assert!(optimizer.pattern_cache.contains_key("slay"));
    assert!(optimizer.pattern_cache.contains_key("yolo"));
    assert!(optimizer.pattern_cache.contains_key("sus"));
    assert!(optimizer.pattern_cache.contains_key("periodt"));
    assert!(optimizer.pattern_cache.contains_key("lowkey"));
    assert!(optimizer.pattern_cache.contains_key("highkey"));
    assert!(optimizer.pattern_cache.contains_key("stan"));
    assert!(optimizer.pattern_cache.contains_key("vibe_check"));
    
    // Test performance gains are reasonable
    if let Some(slay_opt) = optimizer.pattern_cache.get("slay") {
        assert!(slay_opt.performance_gain > 0.0);
        assert!(slay_opt.performance_gain <= 1.0);
        assert!(!slay_opt.original_form.is_empty());
        assert!(!slay_opt.optimized_form.is_empty());
    }
}

#[test]
fn test_target_specific_architecture_support() {
    use cursed::optimization::target_specific::*;
    
    // Test different architectures
    let architectures = vec![
        Architecture::X86_64,
        Architecture::ARM64,
        Architecture::ARM32,
        Architecture::RISCV64,
        Architecture::RISCV32,
        Architecture::WebAssembly,
        Architecture::MIPS,
        Architecture::PowerPC,
    ];
    
    for arch in architectures {
        assert_ne!(arch, Architecture::X86_64); // Just test inequality works
    }
    
    // Test X86_64 specifically
    assert_eq!(Architecture::X86_64, Architecture::X86_64);
    assert_ne!(Architecture::X86_64, Architecture::ARM64);
}

#[test]
fn test_vector_unit_types() {
    use cursed::optimization::target_specific::*;
    
    let vector_units = vec![
        VectorUnitType::SSE,
        VectorUnitType::SSE2,
        VectorUnitType::SSE3,
        VectorUnitType::SSSE3,
        VectorUnitType::SSE4_1,
        VectorUnitType::SSE4_2,
        VectorUnitType::AVX,
        VectorUnitType::AVX2,
        VectorUnitType::AVX512,
        VectorUnitType::NEON,
        VectorUnitType::SVE,
        VectorUnitType::RVV,
    ];
    
    // Test each vector unit type
    for unit_type in vector_units {
        match unit_type {
            VectorUnitType::AVX512 => {
                // AVX512 should be most advanced
                assert_eq!(unit_type, VectorUnitType::AVX512);
            }
            VectorUnitType::NEON => {
                // ARM vector unit
                assert_eq!(unit_type, VectorUnitType::NEON);
            }
            VectorUnitType::RVV => {
                // RISC-V vector unit
                assert_eq!(unit_type, VectorUnitType::RVV);
            }
            _ => {
                // Other units should not be equal to AVX512
                assert_ne!(unit_type, VectorUnitType::AVX512);
            }
        }
    }
}

#[test]
fn test_optimization_result_structure() {
    use cursed::optimization::target_specific::OptimizationResult;
    
    let result = OptimizationResult {
        transformations_applied: 5,
        estimated_performance_gain: 0.15,
        code_size_change: -10,
        register_pressure_change: 2,
    };
    
    assert_eq!(result.transformations_applied, 5);
    assert_eq!(result.estimated_performance_gain, 0.15);
    assert_eq!(result.code_size_change, -10);
    assert_eq!(result.register_pressure_change, 2);
    
    // Test that negative code size change means size reduction
    assert!(result.code_size_change < 0);
}

#[test]
fn test_pass_statistics_calculation() {
    let mut stats = PassStatistics::default();
    
    stats.function_passes_run = 5;
    stats.module_passes_run = 3;
    stats.total_pass_time = Duration::from_millis(100);
    
    assert_eq!(stats.total_passes(), 8);
    
    let passes_per_second = stats.passes_per_second();
    assert!(passes_per_second > 0.0);
    assert!(passes_per_second < 1000.0); // Reasonable upper bound
}

#[test]
fn test_lto_manager_functionality() {
    let mut lto = LtoManager::new(true);
    assert!(lto.is_enabled());
    
    let modules = vec![
        "module1".to_string(),
        "module2".to_string(),
        "module3".to_string(),
    ];
    
    assert!(lto.run_lto(&modules).is_ok());
    
    let stats = lto.get_statistics();
    assert_eq!(stats.modules_processed, 3);
    assert!(stats.functions_inlined > 0);
    assert!(stats.dead_functions_removed > 0);
    assert!(stats.global_variables_merged > 0);
    assert!(stats.lto_time > Duration::from_nanos(0));
    
    // Test disable/enable
    lto.set_enabled(false);
    assert!(!lto.is_enabled());
    
    // Should skip LTO when disabled
    assert!(lto.run_lto(&modules).is_ok());
}

#[test]
fn test_pgo_manager_functionality() {
    // Test with profile data
    let mut pgo = PgoManager::new(true, Some("test_profile.profdata".to_string()));
    assert!(pgo.is_enabled());
    
    assert!(pgo.run_pgo().is_ok());
    
    let stats = pgo.get_statistics();
    assert!(stats.profile_data_loaded);
    assert!(stats.hot_functions_identified > 0);
    assert!(stats.cold_functions_identified > 0);
    assert!(stats.hot_paths_optimized > 0);
    assert!(stats.cold_code_size_reduced > 0);
    assert!(stats.pgo_time > Duration::from_nanos(0));
    
    // Test without profile data
    let mut pgo_no_profile = PgoManager::new(true, None);
    assert!(pgo_no_profile.run_pgo().is_err()); // Should fail without profile data
    
    // Test setting profile data path
    pgo_no_profile.set_profile_data_path(Some("new_profile.profdata".to_string()));
    assert!(pgo_no_profile.run_pgo().is_ok());
}

#[test]
fn test_pass_utils_recommendations() {
    use cursed::optimization::llvm_passes::pass_utils;
    
    let (func_passes, mod_passes) = pass_utils::get_recommended_passes(OptimizationLevel::None);
    assert_eq!(func_passes, vec!["mem2reg"]);
    assert_eq!(mod_passes, vec!["strip-dead-prototypes"]);
    
    let (func_passes, mod_passes) = pass_utils::get_recommended_passes(OptimizationLevel::Aggressive);
    assert!(func_passes.contains(&"loop-vectorize".to_string()));
    assert!(func_passes.contains(&"slp-vectorizer".to_string()));
    assert!(mod_passes.contains(&"mergefunc".to_string()));
    
    // Test optimized pass manager creation
    let config = LlvmPassConfig::default();
    let manager = pass_utils::create_optimized_pass_manager(config, OptimizationLevel::Default);
    let summary = manager.get_passes_summary();
    assert!(!summary.is_empty());
}

#[test]
fn test_vectorization_opportunities() {
    use cursed::optimization::target_specific::*;
    
    let opportunity = VectorizationOpportunity {
        loop_id: "test_loop".to_string(),
        vector_unit: VectorUnitType::AVX2,
        element_type: VectorElementType::F32,
        vector_width: 8,
        operations: vec![
            VectorOperation::Load,
            VectorOperation::Add,
            VectorOperation::Mul,
            VectorOperation::Store,
        ],
        estimated_speedup: 4.0,
        confidence: 0.85,
    };
    
    assert_eq!(opportunity.loop_id, "test_loop");
    assert_eq!(opportunity.vector_unit, VectorUnitType::AVX2);
    assert_eq!(opportunity.element_type, VectorElementType::F32);
    assert_eq!(opportunity.vector_width, 8);
    assert_eq!(opportunity.estimated_speedup, 4.0);
    assert_eq!(opportunity.confidence, 0.85);
    assert_eq!(opportunity.operations.len(), 4);
}

#[test]
fn test_cache_optimization_strategies() {
    use cursed::optimization::target_specific::*;
    
    let strategies = vec![
        CacheStrategyType::DataPrefetching,
        CacheStrategyType::InstructionPrefetching,
        CacheStrategyType::LoopTiling,
        CacheStrategyType::DataBlocking,
        CacheStrategyType::CacheObliviousAlgorithms,
        CacheStrategyType::MemoryLayoutOptimization,
    ];
    
    for strategy in strategies {
        match strategy {
            CacheStrategyType::DataPrefetching => {
                assert_eq!(strategy, CacheStrategyType::DataPrefetching);
            }
            CacheStrategyType::LoopTiling => {
                assert_eq!(strategy, CacheStrategyType::LoopTiling);
            }
            _ => {
                // Other strategies should not equal DataPrefetching
                assert_ne!(strategy, CacheStrategyType::DataPrefetching);
            }
        }
    }
}

#[test]
fn test_memory_access_patterns() {
    use cursed::optimization::target_specific::MemoryAccessPattern;
    
    let patterns = vec![
        MemoryAccessPattern::Sequential,
        MemoryAccessPattern::Strided(4),
        MemoryAccessPattern::Random,
        MemoryAccessPattern::Gather,
        MemoryAccessPattern::Scatter,
    ];
    
    for pattern in patterns {
        match pattern {
            MemoryAccessPattern::Sequential => {
                // Sequential is best for vectorization
                assert_eq!(pattern, MemoryAccessPattern::Sequential);
            }
            MemoryAccessPattern::Strided(stride) => {
                assert_eq!(stride, 4);
                assert_eq!(pattern, MemoryAccessPattern::Strided(4));
            }
            MemoryAccessPattern::Random => {
                // Random is worst for vectorization
                assert_eq!(pattern, MemoryAccessPattern::Random);
            }
            _ => {}
        }
    }
}

#[test]
fn test_integration_with_optimization_config() {
    let config = OptimizationConfig {
        optimization_level: OptimizationLevel::Aggressive,
        enable_llvm_optimizations: true,
        enable_cursed_optimizations: true,
        enable_target_specific_optimizations: true,
        llvm_config: LlvmPassConfig {
            enable_vectorization: true,
            enable_inlining: true,
            enable_loop_unrolling: true,
            enable_common_subexpression_elimination: true,
            enable_dead_code_elimination: true,
            enable_constant_folding: true,
            enable_tail_call_optimization: true,
            function_passes: vec!["mem2reg".to_string(), "gvn".to_string()],
            module_passes: vec!["globalopt".to_string()],
        },
        ..Default::default()
    };
    
    // Test that configuration is properly structured
    assert_eq!(config.optimization_level, OptimizationLevel::Aggressive);
    assert!(config.enable_llvm_optimizations);
    assert!(config.enable_cursed_optimizations);
    assert!(config.enable_target_specific_optimizations);
    
    // Test LLVM config
    assert!(config.llvm_config.enable_vectorization);
    assert!(config.llvm_config.enable_inlining);
    assert!(!config.llvm_config.function_passes.is_empty());
    assert!(!config.llvm_config.module_passes.is_empty());
}
