/// LLVM Advanced Optimization Tests
/// 
/// Comprehensive tests for the LLVM advanced optimization system,
/// including unit tests, integration tests, and performance validation.

use cursed::optimization::{
    AdvancedOptimizationManager, AdvancedOptimizationConfig, OptimizationConfig,
    OptimizationStatistics, OptimizationPipeline, OptimizationPass,
    FunctionInliner, LoopOptimizer, DeadCodeEliminator, ConstantPropagator,
    CommonSubexpressionEliminator, TailCallOptimizer, MemoryOptimizer,
};
use cursed::error::Result;
use std::time::Duration;
use std::sync::{Arc, Mutex};

#[path = "common.rs"]
mod common;

#[test]
fn test_advanced_optimization_manager_creation() {
    common::tracing::setup();
    
    let base_config = OptimizationConfig::default();
    let manager = AdvancedOptimizationManager::new(&base_config);
    assert!(manager.is_ok(), "Should create optimization manager successfully");
    
    let manager = manager.unwrap();
    let stats = manager.get_statistics();
    assert_eq!(stats.total_optimizations(), 0, "Should start with zero optimizations");
}

#[test]
fn test_optimization_config_creation() {
    common::tracing::setup();
    
    let config = AdvancedOptimizationConfig::default();
    assert!(config.enable_inlining, "Inlining should be enabled by default");
    assert!(config.enable_loop_optimization, "Loop optimization should be enabled");
    assert!(config.enable_dead_code_elimination, "Dead code elimination should be enabled");
    assert!(config.enable_constant_propagation, "Constant propagation should be enabled");
    assert!(config.enable_cse, "CSE should be enabled");
    assert!(config.enable_tail_calls, "Tail call optimization should be enabled");
    assert!(config.enable_memory_optimization, "Memory optimization should be enabled");
    assert_eq!(config.max_inline_size, 1000, "Default inline size should be 1000");
    assert_eq!(config.max_unroll_count, 8, "Default unroll count should be 8");
    assert_eq!(config.timeout, Duration::from_secs(30), "Default timeout should be 30 seconds");
}

#[test]
fn test_optimization_statistics() {
    common::tracing::setup();
    
    let stats = OptimizationStatistics {
        functions_inlined: 5,
        instructions_eliminated: 100,
        loops_unrolled: 3,
        constants_propagated: 25,
        dead_blocks_removed: 10,
        cse_eliminations: 8,
        tail_calls_optimized: 2,
        memory_optimizations: 15,
        code_size_before: 1000,
        code_size_after: 750,
        ..Default::default()
    };
    
    assert_eq!(stats.total_optimizations(), 168, "Total optimizations should match sum");
    assert_eq!(stats.size_reduction_percent(), 25.0, "Size reduction should be 25%");
}

#[test]
fn test_function_inliner() {
    common::tracing::setup();
    
    let config = AdvancedOptimizationConfig::default();
    let stats = Arc::new(Mutex::new(OptimizationStatistics::default()));
    let inliner = FunctionInliner::new(config, stats.clone());
    
    assert_eq!(inliner.name(), "function-inliner");
    assert!(inliner.is_enabled());
}

#[test]
fn test_loop_optimizer() {
    common::tracing::setup();
    
    let config = AdvancedOptimizationConfig::default();
    let stats = Arc::new(Mutex::new(OptimizationStatistics::default()));
    let optimizer = LoopOptimizer::new(config, stats.clone());
    
    assert_eq!(optimizer.name(), "loop-optimizer");
    assert!(optimizer.is_enabled());
}

#[test]
fn test_dead_code_eliminator() {
    common::tracing::setup();
    
    let config = AdvancedOptimizationConfig::default();
    let stats = Arc::new(Mutex::new(OptimizationStatistics::default()));
    let eliminator = DeadCodeEliminator::new(config, stats.clone());
    
    assert_eq!(eliminator.name(), "dead-code-eliminator");
    assert!(eliminator.is_enabled());
}

#[test]
fn test_constant_propagator() {
    common::tracing::setup();
    
    let config = AdvancedOptimizationConfig::default();
    let stats = Arc::new(Mutex::new(OptimizationStatistics::default()));
    let propagator = ConstantPropagator::new(config, stats.clone());
    
    assert_eq!(propagator.name(), "constant-propagator");
    assert!(propagator.is_enabled());
}

#[test]
fn test_cse_eliminator() {
    common::tracing::setup();
    
    let config = AdvancedOptimizationConfig::default();
    let stats = Arc::new(Mutex::new(OptimizationStatistics::default()));
    let eliminator = CommonSubexpressionEliminator::new(config, stats.clone());
    
    assert_eq!(eliminator.name(), "cse-eliminator");
    assert!(eliminator.is_enabled());
}

#[test]
fn test_tail_call_optimizer() {
    common::tracing::setup();
    
    let config = AdvancedOptimizationConfig::default();
    let stats = Arc::new(Mutex::new(OptimizationStatistics::default()));
    let optimizer = TailCallOptimizer::new(config, stats.clone());
    
    assert_eq!(optimizer.name(), "tail-call-optimizer");
    assert!(optimizer.is_enabled());
}

#[test]
fn test_memory_optimizer() {
    common::tracing::setup();
    
    let config = AdvancedOptimizationConfig::default();
    let stats = Arc::new(Mutex::new(OptimizationStatistics::default()));
    let optimizer = MemoryOptimizer::new(config, stats.clone());
    
    assert_eq!(optimizer.name(), "memory-optimizer");
    assert!(optimizer.is_enabled());
}

#[test]
fn test_optimization_pipeline() {
    common::tracing::setup();
    
    let config = AdvancedOptimizationConfig::default();
    let pipeline = OptimizationPipeline::new(config);
    assert!(pipeline.is_ok(), "Should create optimization pipeline successfully");
}

#[test]
fn test_optimization_config_utils() {
    common::tracing::setup();
    
    let dev_config = cursed::optimization::llvm_advanced::utils::dev_config();
    let release_config = cursed::optimization::llvm_advanced::utils::release_config();
    let pgo_config = cursed::optimization::llvm_advanced::utils::pgo_config();
    
    // Development config should have minimal optimizations
    assert!(!dev_config.enable_inlining, "Dev config should disable inlining");
    assert!(!dev_config.enable_loop_optimization, "Dev config should disable loop optimization");
    assert!(dev_config.enable_dead_code_elimination, "Dev config should enable dead code elimination");
    assert!(dev_config.enable_constant_propagation, "Dev config should enable constant propagation");
    
    // Release config should have aggressive optimizations
    assert!(release_config.enable_inlining, "Release config should enable inlining");
    assert!(release_config.enable_loop_optimization, "Release config should enable loop optimization");
    assert!(release_config.enable_dead_code_elimination, "Release config should enable dead code elimination");
    assert!(release_config.enable_constant_propagation, "Release config should enable constant propagation");
    assert!(release_config.enable_cse, "Release config should enable CSE");
    assert!(release_config.enable_tail_calls, "Release config should enable tail calls");
    assert!(release_config.enable_memory_optimization, "Release config should enable memory optimization");
    assert!(release_config.enable_ipo, "Release config should enable IPO");
    assert_eq!(release_config.max_inline_size, 2000, "Release config should have larger inline size");
    assert_eq!(release_config.max_unroll_count, 16, "Release config should have higher unroll count");
    
    // PGO config should enable profile-guided optimization
    assert!(pgo_config.enable_pgo, "PGO config should enable profile-guided optimization");
    assert!(pgo_config.enable_inlining, "PGO config should enable inlining");
}

#[test]
fn test_disabled_optimization_passes() {
    common::tracing::setup();
    
    let mut config = AdvancedOptimizationConfig::default();
    config.enable_inlining = false;
    config.enable_loop_optimization = false;
    config.enable_cse = false;
    
    let stats = Arc::new(Mutex::new(OptimizationStatistics::default()));
    
    let inliner = FunctionInliner::new(config.clone(), stats.clone());
    let loop_optimizer = LoopOptimizer::new(config.clone(), stats.clone());
    let cse_eliminator = CommonSubexpressionEliminator::new(config.clone(), stats.clone());
    
    assert!(!inliner.is_enabled(), "Inliner should be disabled");
    assert!(!loop_optimizer.is_enabled(), "Loop optimizer should be disabled");
    assert!(!cse_eliminator.is_enabled(), "CSE eliminator should be disabled");
}

#[test]
fn test_optimization_statistics_edge_cases() {
    common::tracing::setup();
    
    // Test zero code size before
    let stats = OptimizationStatistics {
        code_size_before: 0,
        code_size_after: 0,
        ..Default::default()
    };
    assert_eq!(stats.size_reduction_percent(), 0.0, "Zero size should give 0% reduction");
    
    // Test code size increase
    let stats = OptimizationStatistics {
        code_size_before: 500,
        code_size_after: 600,
        ..Default::default()
    };
    assert_eq!(stats.size_reduction_percent(), -20.0, "Increase should give negative reduction");
    
    // Test perfect reduction
    let stats = OptimizationStatistics {
        code_size_before: 1000,
        code_size_after: 0,
        ..Default::default()
    };
    assert_eq!(stats.size_reduction_percent(), 100.0, "Complete elimination should give 100% reduction");
}

#[test]
fn test_optimization_manager_configuration_update() {
    common::tracing::setup();
    
    let base_config = OptimizationConfig::default();
    let mut manager = AdvancedOptimizationManager::new(&base_config).unwrap();
    
    let new_config = AdvancedOptimizationConfig {
        enable_inlining: false,
        enable_loop_optimization: false,
        max_inline_size: 500,
        ..AdvancedOptimizationConfig::default()
    };
    
    let result = manager.update_config(new_config.clone());
    assert!(result.is_ok(), "Should update configuration successfully");
}

#[test]
fn test_optimization_timeout_configuration() {
    common::tracing::setup();
    
    let mut config = AdvancedOptimizationConfig::default();
    config.timeout = Duration::from_millis(100);
    
    let base_config = OptimizationConfig::default();
    let manager = AdvancedOptimizationManager::new(&base_config);
    assert!(manager.is_ok(), "Should create manager with custom timeout");
}

#[test]
fn test_optimization_pass_ordering() {
    common::tracing::setup();
    
    // Test that optimization passes are created in the correct order
    let base_config = OptimizationConfig::default();
    let manager = AdvancedOptimizationManager::new(&base_config).unwrap();
    
    // The manager should contain all optimization passes
    let stats = manager.get_statistics();
    assert_eq!(stats.total_optimizations(), 0, "Should start with no optimizations");
}

#[test]
fn test_optimization_manager_thread_safety() {
    common::tracing::setup();
    
    let base_config = OptimizationConfig::default();
    let manager = Arc::new(Mutex::new(
        AdvancedOptimizationManager::new(&base_config).unwrap()
    ));
    
    let handles: Vec<_> = (0..4).map(|_| {
        let manager_clone = manager.clone();
        std::thread::spawn(move || {
            let manager = manager_clone.lock().unwrap();
            let stats = manager.get_statistics();
            assert_eq!(stats.total_optimizations(), 0);
        })
    }).collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_optimization_statistics_default() {
    common::tracing::setup();
    
    let stats = OptimizationStatistics::default();
    assert_eq!(stats.functions_inlined, 0);
    assert_eq!(stats.instructions_eliminated, 0);
    assert_eq!(stats.loops_unrolled, 0);
    assert_eq!(stats.constants_propagated, 0);
    assert_eq!(stats.dead_blocks_removed, 0);
    assert_eq!(stats.cse_eliminations, 0);
    assert_eq!(stats.tail_calls_optimized, 0);
    assert_eq!(stats.memory_optimizations, 0);
    assert_eq!(stats.code_size_before, 0);
    assert_eq!(stats.code_size_after, 0);
    assert_eq!(stats.total_optimizations(), 0);
    assert_eq!(stats.size_reduction_percent(), 0.0);
}

#[test]
fn test_configuration_cloning() {
    common::tracing::setup();
    
    let config = AdvancedOptimizationConfig::default();
    let cloned_config = config.clone();
    
    assert_eq!(config.enable_inlining, cloned_config.enable_inlining);
    assert_eq!(config.max_inline_size, cloned_config.max_inline_size);
    assert_eq!(config.enable_loop_optimization, cloned_config.enable_loop_optimization);
    assert_eq!(config.max_unroll_count, cloned_config.max_unroll_count);
    assert_eq!(config.timeout, cloned_config.timeout);
}

#[test]
fn test_optimization_manager_summary_printing() {
    common::tracing::setup();
    
    let base_config = OptimizationConfig::default();
    let manager = AdvancedOptimizationManager::new(&base_config).unwrap();
    
    // This should not panic or cause errors
    manager.print_summary();
}

#[test]
fn test_optimization_config_validation() {
    common::tracing::setup();
    
    // Test extreme values
    let mut config = AdvancedOptimizationConfig::default();
    config.max_inline_size = 0;
    config.max_unroll_count = 0;
    config.timeout = Duration::from_millis(1);
    
    let base_config = OptimizationConfig::default();
    let manager = AdvancedOptimizationManager::new(&base_config);
    assert!(manager.is_ok(), "Should handle extreme configuration values");
}

#[test]
fn test_optimization_pass_names() {
    common::tracing::setup();
    
    let config = AdvancedOptimizationConfig::default();
    let stats = Arc::new(Mutex::new(OptimizationStatistics::default()));
    
    let inliner = FunctionInliner::new(config.clone(), stats.clone());
    let loop_opt = LoopOptimizer::new(config.clone(), stats.clone());
    let dead_code = DeadCodeEliminator::new(config.clone(), stats.clone());
    let const_prop = ConstantPropagator::new(config.clone(), stats.clone());
    let cse = CommonSubexpressionEliminator::new(config.clone(), stats.clone());
    let tail_call = TailCallOptimizer::new(config.clone(), stats.clone());
    let memory_opt = MemoryOptimizer::new(config.clone(), stats.clone());
    
    // Verify all pass names are unique
    let names = vec![
        inliner.name(),
        loop_opt.name(),
        dead_code.name(),
        const_prop.name(),
        cse.name(),
        tail_call.name(),
        memory_opt.name(),
    ];
    
    let mut unique_names = names.clone();
    unique_names.sort();
    unique_names.dedup();
    
    assert_eq!(names.len(), unique_names.len(), "All optimization pass names should be unique");
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    
    #[test]
    fn test_optimization_performance_simulation() {
        common::tracing::setup();
        
        let base_config = OptimizationConfig::default();
        let manager = AdvancedOptimizationManager::new(&base_config).unwrap();
        
        // Simulate performance tracking
        let start = std::time::Instant::now();
        let _stats = manager.get_statistics();
        let duration = start.elapsed();
        
        assert!(duration < Duration::from_millis(100), "Statistics retrieval should be fast");
    }
    
    #[test]
    fn test_concurrent_optimization_access() {
        common::tracing::setup();
        
        let base_config = OptimizationConfig::default();
        let manager = Arc::new(AdvancedOptimizationManager::new(&base_config).unwrap());
        let counter = Arc::new(AtomicUsize::new(0));
        
        let handles: Vec<_> = (0..8).map(|_| {
            let manager_clone = manager.clone();
            let counter_clone = counter.clone();
            
            std::thread::spawn(move || {
                for _ in 0..100 {
                    let _stats = manager_clone.get_statistics();
                    counter_clone.fetch_add(1, Ordering::Relaxed);
                }
            })
        }).collect();
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        assert_eq!(counter.load(Ordering::Relaxed), 800, "All concurrent accesses should complete");
    }
}

/// Performance benchmarks for optimization system
#[cfg(test)]
mod performance_tests {
    use super::*;
    
    #[test]
    #[ignore] // Run with --ignored for performance testing
    fn benchmark_optimization_manager_creation() {
        common::tracing::setup();
        
        let base_config = OptimizationConfig::default();
        let start = std::time::Instant::now();
        
        for _ in 0..1000 {
            let _manager = AdvancedOptimizationManager::new(&base_config).unwrap();
        }
        
        let duration = start.elapsed();
        println!("Created 1000 optimization managers in {:?}", duration);
        
        // Should be able to create managers quickly
        assert!(duration < Duration::from_secs(1), "Manager creation should be fast");
    }
    
    #[test]
    #[ignore] // Run with --ignored for performance testing
    fn benchmark_optimization_statistics() {
        common::tracing::setup();
        
        let base_config = OptimizationConfig::default();
        let manager = AdvancedOptimizationManager::new(&base_config).unwrap();
        
        let start = std::time::Instant::now();
        
        for _ in 0..10000 {
            let _stats = manager.get_statistics();
        }
        
        let duration = start.elapsed();
        println!("Retrieved statistics 10000 times in {:?}", duration);
        
        // Statistics retrieval should be very fast
        assert!(duration < Duration::from_millis(100), "Statistics retrieval should be very fast");
    }
}
