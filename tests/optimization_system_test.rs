/// Comprehensive tests for the CURSED compiler optimization system
/// 
/// Tests optimization levels, LLVM pass integration, parallel compilation,
/// caching, and overall system performance.

use cursed::optimization::{
    OptimizationManager, OptimizationConfig, OptimizationLevel, LevelConfig,
    ParallelCompiler, CacheManager, LlvmPassManager,
};
use cursed::optimization::compilation_speed::{CompilationUnit, CompilationStatus};
use cursed::error::Result;
use std::path::PathBuf;
use std::time::{SystemTime, Duration};
use tracing_test::traced_test;

#[traced_test]
#[test]
fn test_optimization_levels() {
    // Test all optimization levels
    for level in [
        OptimizationLevel::None,
        OptimizationLevel::Basic,
        OptimizationLevel::Standard,
        OptimizationLevel::Aggressive,
    ] {
        let config = LevelConfig::for_level(level);
        assert_eq!(config.level, level);
        
        match level {
            OptimizationLevel::None => {
                assert!(!config.enable_inlining);
                assert!(!config.enable_loop_optimization);
                assert!(!config.enable_lto);
            }
            OptimizationLevel::Aggressive => {
                assert!(config.enable_inlining);
                assert!(config.enable_loop_optimization);
                assert!(config.enable_lto);
                assert!(config.enable_fast_math);
            }
            _ => {
                // Basic and Standard have some optimizations enabled
                assert!(config.enable_constant_folding);
                assert!(config.enable_dead_code_elimination);
            }
        }
    }
}

#[traced_test]
#[test]
fn test_optimization_manager_creation() -> Result<()> {
    let config = OptimizationConfig {
        enable_advanced_llvm: true,
        enable_parallel_compilation: true,
        enable_caching: true,
        optimization_level: 2,
        max_parallel_threads: 4,
        ..Default::default()
    };

    let manager = OptimizationManager::new(config)?;
    
    // Check that components are initialized
    assert!(manager.llvm_optimizer().is_some());
    assert!(manager.parallel_compiler().is_some());
    assert!(manager.cache_manager().is_some());
    assert!(manager.speed_optimizer().is_some());
    
    // Check optimization level
    assert_eq!(manager.optimization_level_manager().current_level(), OptimizationLevel::Standard);
    
    Ok(())
}

#[traced_test]
#[test]
fn test_parallel_compilation() -> Result<()> {
    let config = OptimizationConfig {
        enable_parallel_compilation: true,
        max_parallel_threads: 2,
        ..Default::default()
    };

    let parallel_compiler = ParallelCompiler::new(&config)?;
    
    // Create test compilation units
    let units = vec![
        CompilationUnit {
            id: "unit1".to_string(),
            source_path: PathBuf::from("unit1.csd"),
            module_name: "unit1".to_string(),
            source_code: "let x = 42;".to_string(),
            dependencies: vec![],
            last_modified: SystemTime::now(),
            status: CompilationStatus::Pending,
            priority: 1,
        },
        CompilationUnit {
            id: "unit2".to_string(),
            source_path: PathBuf::from("unit2.csd"),
            module_name: "unit2".to_string(),
            source_code: "let y = x + 1;".to_string(),
            dependencies: vec!["unit1".to_string()],
            last_modified: SystemTime::now(),
            status: CompilationStatus::Pending,
            priority: 2,
        },
    ];

    // Test parallel compilation
    let results = parallel_compiler.compile_parallel(units)?;
    
    assert_eq!(results.len(), 2);
    assert!(results.contains_key("unit1"));
    assert!(results.contains_key("unit2"));
    
    // Check statistics
    let stats = parallel_compiler.get_statistics();
    assert_eq!(stats.units_processed, 2);
    assert!(stats.wall_clock_time > Duration::ZERO);
    
    Ok(())
}

#[traced_test]
#[test]
fn test_cache_system() -> Result<()> {
    let config = OptimizationConfig {
        enable_caching: true,
        ..Default::default()
    };

    let cache_manager = CacheManager::new(&config)?;
    
    // Create test compilation unit
    let unit = CompilationUnit {
        id: "test_unit".to_string(),
        source_path: PathBuf::from("test.csd"),
        module_name: "test".to_string(),
        source_code: "let x = 42;".to_string(),
        dependencies: vec![],
        last_modified: SystemTime::now(),
        status: CompilationStatus::Pending,
        priority: 1,
    };

    // Generate cache key
    let cache_key = cache_manager.generate_cache_key(&unit, OptimizationLevel::Standard)?;
    assert!(!cache_key.is_empty());
    
    // Test cache miss
    assert!(!cache_manager.is_cache_valid(&unit, &cache_key)?);
    
    // Store cache entry
    let test_data = b"compiled bytecode";
    cache_manager.store_cache_entry(
        &unit,
        OptimizationLevel::Standard,
        cursed::optimization::cache::CacheEntryType::Bytecode,
        test_data,
    )?;
    
    // Test cache hit
    assert!(cache_manager.is_cache_valid(&unit, &cache_key)?);
    
    // Retrieve cache entry
    let retrieved_data = cache_manager.retrieve_cache_entry(&cache_key)?;
    assert!(retrieved_data.is_some());
    assert_eq!(retrieved_data.unwrap(), test_data);
    
    // Check statistics
    let stats = cache_manager.get_statistics();
    assert_eq!(stats.cache_hits, 1);
    assert_eq!(stats.entry_count, 1);
    assert!(stats.hit_ratio() > 0.0);
    
    Ok(())
}

#[traced_test]
#[test]
fn test_llvm_pass_manager() -> Result<()> {
    use inkwell::context::Context;
    
    let context = Context::create();
    let config = LevelConfig::for_level(OptimizationLevel::Standard);
    
    let pass_manager = LlvmPassManager::new(&context, config)?;
    
    // Create a simple LLVM module for testing
    let module = context.create_module("test");
    
    // Test optimization
    pass_manager.optimize_module(&module)?;
    
    // Check statistics
    let stats = pass_manager.get_statistics();
    assert_eq!(stats.passes_executed, 1);
    
    Ok(())
}

#[traced_test]
#[test]
fn test_custom_optimization_configuration() -> Result<()> {
    // Test custom optimization configuration
    let custom_config = LevelConfig::custom(OptimizationLevel::Basic)
        .enable_lto(true)
        .enable_fast_math(true)
        .max_inline_size(500)
        .timeout(Duration::from_secs(120))
        .build();

    assert_eq!(custom_config.level, OptimizationLevel::Basic);
    assert!(custom_config.enable_lto);
    assert!(custom_config.enable_fast_math);
    assert_eq!(custom_config.max_inline_size, 500);
    assert_eq!(custom_config.timeout, Duration::from_secs(120));
    
    Ok(())
}

#[traced_test]
#[test]
fn test_optimization_level_switching() -> Result<()> {
    let config = OptimizationConfig::default();
    let mut manager = OptimizationManager::new(config)?;
    
    // Test switching optimization levels
    assert_eq!(manager.optimization_level_manager().current_level(), OptimizationLevel::Standard);
    
    manager.set_optimization_level(OptimizationLevel::Aggressive)?;
    assert_eq!(manager.optimization_level_manager().current_level(), OptimizationLevel::Aggressive);
    
    manager.set_optimization_level(OptimizationLevel::None)?;
    assert_eq!(manager.optimization_level_manager().current_level(), OptimizationLevel::None);
    
    Ok(())
}

#[traced_test]
#[test]
fn test_performance_characteristics() -> Result<()> {
    let start_time = std::time::Instant::now();
    
    // Test optimization system creation performance
    let config = OptimizationConfig {
        enable_advanced_llvm: true,
        enable_parallel_compilation: true,
        enable_caching: true,
        enable_jit_optimization: true,
        enable_memory_optimization: true,
        optimization_level: 2,
        max_parallel_threads: 4,
        ..Default::default()
    };

    let manager = OptimizationManager::new(config)?;
    let creation_time = start_time.elapsed();
    
    // Creation should be fast (< 1 second)
    assert!(creation_time < Duration::from_secs(1));
    
    // Test parallel compilation performance
    if let Some(parallel_compiler) = manager.parallel_compiler() {
        let units = (0..10).map(|i| CompilationUnit {
            id: format!("unit{}", i),
            source_path: PathBuf::from(format!("unit{}.csd", i)),
            module_name: format!("unit{}", i),
            source_code: format!("let x{} = {};", i, i),
            dependencies: vec![],
            last_modified: SystemTime::now(),
            status: CompilationStatus::Pending,
            priority: 1,
        }).collect();

        let compile_start = std::time::Instant::now();
        let results = parallel_compiler.compile_parallel(units)?;
        let compile_time = compile_start.elapsed();
        
        assert_eq!(results.len(), 10);
        
        // Parallel compilation should show some efficiency
        let stats = parallel_compiler.get_statistics();
        assert!(stats.efficiency() > 0.5); // At least 50% efficiency
        
        println!("Parallel compilation efficiency: {:.2}x", stats.efficiency());
        println!("Compilation time: {:?}", compile_time);
    }
    
    Ok(())
}

#[traced_test]
#[test]
fn test_memory_usage_optimization() -> Result<()> {
    // Test that the optimization system doesn't use excessive memory
    let config = OptimizationConfig {
        enable_memory_optimization: true,
        enable_caching: true,
        ..Default::default()
    };

    let manager = OptimizationManager::new(config)?;
    
    // Memory optimizer should be available
    assert!(manager.memory_optimizer().is_some());
    
    // Cache should be available
    if let Some(cache_manager) = manager.cache_manager() {
        let stats = cache_manager.get_statistics();
        
        // Initial memory usage should be minimal
        assert_eq!(stats.entry_count, 0);
        assert_eq!(stats.total_size, 0);
    }
    
    Ok(())
}

#[traced_test]
#[test]
fn test_optimization_statistics() -> Result<()> {
    let config = OptimizationConfig {
        enable_profiling: true,
        enable_advanced_llvm: true,
        enable_parallel_compilation: true,
        enable_caching: true,
        ..Default::default()
    };

    let manager = OptimizationManager::new(config)?;
    
    // Test that we can collect statistics from various components
    if let Some(profiler) = manager.profiler() {
        let metrics = profiler.get_performance_metrics();
        assert!(metrics.compilation_time >= Duration::ZERO);
    }
    
    if let Some(cache_manager) = manager.cache_manager() {
        let stats = cache_manager.get_statistics();
        assert_eq!(stats.cache_hits + stats.cache_misses, 0); // No operations yet
    }
    
    if let Some(parallel_compiler) = manager.parallel_compiler() {
        let stats = parallel_compiler.get_statistics();
        assert_eq!(stats.units_processed, 0); // No compilation yet
    }
    
    Ok(())
}

#[traced_test]
#[test]
fn test_error_handling() -> Result<()> {
    // Test error handling in optimization system
    
    // Test invalid optimization level conversion
    let mut config = OptimizationConfig::default();
    config.optimization_level = 999; // Invalid level
    
    let manager = OptimizationManager::new(config)?;
    // Should default to Standard level
    assert_eq!(manager.optimization_level_manager().current_level(), OptimizationLevel::Standard);
    
    Ok(())
}

#[traced_test]
#[test] 
fn test_comprehensive_optimization_workflow() -> Result<()> {
    // Test a complete optimization workflow
    let config = OptimizationConfig {
        enable_advanced_llvm: true,
        enable_parallel_compilation: true,
        enable_caching: true,
        enable_jit_optimization: true,
        enable_memory_optimization: true,
        enable_profiling: true,
        optimization_level: 3, // Aggressive
        max_parallel_threads: 4,
        ..Default::default()
    };

    let mut manager = OptimizationManager::new(config)?;
    
    // Verify all components are initialized
    assert!(manager.llvm_optimizer().is_some());
    assert!(manager.parallel_compiler().is_some());
    assert!(manager.cache_manager().is_some());
    assert!(manager.jit_optimizer().is_some());
    assert!(manager.memory_optimizer().is_some());
    assert!(manager.profiler().is_some());
    
    // Test optimization level management
    assert_eq!(manager.optimization_level_manager().current_level(), OptimizationLevel::Aggressive);
    
    // Test level switching
    manager.set_optimization_level(OptimizationLevel::Basic)?;
    assert_eq!(manager.optimization_level_manager().current_level(), OptimizationLevel::Basic);
    
    // Test LTO manager (should be disabled for Basic level)
    assert!(manager.lto_manager().is_none());
    
    // Switch to aggressive and verify LTO is available
    manager.set_optimization_level(OptimizationLevel::Aggressive)?;
    assert!(manager.lto_manager().is_some());
    
    println!("✅ Comprehensive optimization workflow test completed successfully");
    
    Ok(())
}
