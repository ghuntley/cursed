/// Compilation Speed Optimization Tests
/// 
/// Tests for the enhanced compilation speed system including:
/// - Incremental compilation with caching
/// - Parallel processing and type checking
/// - Performance monitoring and bottleneck detection
/// - Cache invalidation and dependency tracking

use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};
use tempfile::TempDir;

use cursed::optimization::compilation_speed::{
    CompilationSpeedOptimizer, CompilationSpeedConfig, CompilationUnit, CompilationStatus,
    AstCache, CompilationPerformanceMonitor, ParallelTypeChecker, TypeCheckingOptimizer,
    CacheStatistics, BottleneckInfo
};
use cursed::optimization::OptimizationConfig;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::error::Result;

fn create_test_unit(id: &str, module_name: &str, source_code: &str, dependencies: Vec<String>) -> CompilationUnit {
    let mut unit = CompilationUnit {
        id: id.to_string(),
        source_path: PathBuf::from(format!("{}.csd", module_name)),
        module_name: module_name.to_string(),
        source_code: source_code.to_string(),
        dependencies,
        last_modified: SystemTime::now(),
        status: CompilationStatus::Pending,
        priority: 1,
        content_hash: String::new(),
    };
    
    unit.content_hash = AstCache::calculate_content_hash(&unit);
    unit
}

#[test]
fn test_incremental_compilation_basic() -> Result<()> {
    let config = OptimizationConfig {
        enable_parallel_compilation: true,
        enable_incremental_compilation: true,
        max_parallel_threads: 2,
        ..Default::default()
    };
    
    let optimizer = CompilationSpeedOptimizer::new(&config)?;
    
    // Create test units
    let units = vec![
        create_test_unit("unit1", "module1", "facts x = 42;", vec![]),
        create_test_unit("unit2", "module2", "facts y = x + 1;", vec!["module1".to_string()]),
        create_test_unit("unit3", "module3", "facts z = y * 2;", vec!["module2".to_string()]),
    ];
    
    // First compilation - everything should be compiled
    let results1 = optimizer.compile_incremental(units.clone())?;
    assert_eq!(results1.len(), 3);
    
    let stats1 = optimizer.get_statistics();
    assert_eq!(stats1.total_units, 3);
    assert_eq!(stats1.cached_units, 0); // Nothing cached on first run
    
    // Second compilation - everything should be cached
    let results2 = optimizer.compile_incremental(units.clone())?;
    assert_eq!(results2.len(), 3);
    
    let stats2 = optimizer.get_statistics();
    assert_eq!(stats2.cached_units, 3); // All units should be cached
    assert!(stats2.cache_hit_rate > 0.9); // High cache hit rate
    
    Ok(())
}

#[test]
fn test_cache_invalidation() -> Result<()> {
    let config = OptimizationConfig {
        enable_incremental_compilation: true,
        ..Default::default()
    };
    
    let optimizer = CompilationSpeedOptimizer::new(&config)?;
    
    // Create test units with dependencies
    let mut units = vec![
        create_test_unit("unit1", "module1", "facts x = 42;", vec![]),
        create_test_unit("unit2", "module2", "facts y = x + 1;", vec!["module1".to_string()]),
        create_test_unit("unit3", "module3", "facts z = y * 2;", vec!["module2".to_string()]),
    ];
    
    // First compilation
    optimizer.compile_incremental(units.clone())?;
    
    // Modify module1 (change source code)
    units[0].source_code = "facts x = 100;".to_string();
    units[0].content_hash = AstCache::calculate_content_hash(&units[0]);
    
    // Second compilation - module1 and dependents should be recompiled
    let results = optimizer.compile_incremental(units)?;
    assert_eq!(results.len(), 3);
    
    let stats = optimizer.get_statistics();
    // Should have some cache hits and some misses due to invalidation
    assert!(stats.cache_hit_rate < 1.0);
    
    Ok(())
}

#[test]
fn test_parallel_type_checking() -> Result<()> {
    let config = CompilationSpeedConfig {
        enable_parallel_type_checking: true,
        max_parallel_threads: 4,
        ..Default::default()
    };
    
    let type_optimizer = std::sync::Arc::new(TypeCheckingOptimizer::new(config.clone()));
    let parallel_checker = ParallelTypeChecker::new(config, type_optimizer);
    
    // Create test programs
    let programs = vec![
        ("module1".to_string(), {
            let lexer = Lexer::new("facts x = 42;".to_string());
            let mut parser = Parser::new(lexer)?;
            parser.parse_program()?
        }),
        ("module2".to_string(), {
            let lexer = Lexer::new("facts y = 24;".to_string());
            let mut parser = Parser::new(lexer)?;
            parser.parse_program()?
        }),
        ("module3".to_string(), {
            let lexer = Lexer::new("facts z = 36;".to_string());
            let mut parser = Parser::new(lexer)?;
            parser.parse_program()?
        }),
    ];
    
    // Perform parallel type checking
    let type_results = parallel_checker.check_types_parallel(programs)?;
    assert_eq!(type_results.len(), 3);
    
    // Check that all modules were processed
    let module_names: std::collections::HashSet<_> = type_results
        .iter()
        .map(|(name, _)| name.clone())
        .collect();
    
    assert!(module_names.contains("module1"));
    assert!(module_names.contains("module2"));
    assert!(module_names.contains("module3"));
    
    // Verify performance monitoring worked
    let monitor = parallel_checker.get_performance_monitor();
    let avg_time = monitor.get_average_phase_timing("parallel_type_checking");
    assert!(avg_time.is_some());
    
    Ok(())
}

#[test]
fn test_ast_cache_persistence() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let cache_dir = temp_dir.path().to_path_buf();
    
    // Create first cache instance
    let cache1 = AstCache::new(cache_dir.clone())?;
    
    let unit = create_test_unit("test", "test_module", "facts value = 123;", vec![]);
    
    // Parse and store in cache
    let lexer = Lexer::new(unit.source_code.clone());
    let mut parser = Parser::new(lexer)?;
    let program = parser.parse_program()?;
    
    cache1.store_ast(&unit, program.clone(), 500)?;
    
    // Create second cache instance (simulating restart)
    let cache2 = AstCache::new(cache_dir.clone())?;
    
    // Should be able to retrieve from disk cache
    let cached = cache2.get_cached_ast(&unit);
    assert!(cached.is_some());
    
    let cached_ast = cached.unwrap();
    assert_eq!(cached_ast.content_hash, unit.content_hash);
    assert_eq!(cached_ast.compilation_time_us, 500);
    
    Ok(())
}

#[test]
fn test_performance_monitoring() -> Result<()> {
    let monitor = CompilationPerformanceMonitor::new();
    
    // Record various phase timings
    monitor.record_phase_timing("parsing", Duration::from_millis(20));
    monitor.record_phase_timing("type_checking", Duration::from_millis(50));
    monitor.record_phase_timing("slow_optimization", Duration::from_millis(150)); // Should be bottleneck
    monitor.record_phase_timing("code_generation", Duration::from_millis(30));
    
    // Record memory usage
    monitor.record_memory_usage(1024 * 1024); // 1MB
    monitor.record_memory_usage(2048 * 1024); // 2MB peak
    monitor.record_memory_usage(1536 * 1024); // 1.5MB
    
    // Check bottleneck detection
    let bottlenecks = monitor.get_bottlenecks();
    assert_eq!(bottlenecks.len(), 1);
    assert_eq!(bottlenecks[0].phase_name, "slow_optimization");
    assert_eq!(bottlenecks[0].occurrence_count, 1);
    
    // Check average timings
    let parsing_avg = monitor.get_average_phase_timing("parsing");
    assert!(parsing_avg.is_some());
    assert_eq!(parsing_avg.unwrap(), Duration::from_millis(20));
    
    // Generate and validate report
    let report = monitor.generate_report();
    assert!(report.contains("slow_optimization"));
    assert!(report.contains("Detected Bottlenecks"));
    assert!(report.contains("Memory Usage"));
    assert!(report.contains("Peak memory: 2 MB"));
    
    Ok(())
}

#[test]
fn test_dependency_graph_optimization() -> Result<()> {
    let config = OptimizationConfig {
        enable_incremental_compilation: true,
        ..Default::default()
    };
    
    let optimizer = CompilationSpeedOptimizer::new(&config)?;
    
    // Create a complex dependency graph
    let units = vec![
        create_test_unit("core", "core", "facts CORE_VALUE = 1;", vec![]),
        create_test_unit("utils", "utils", "facts UTIL_VALUE = CORE_VALUE + 1;", vec!["core".to_string()]),
        create_test_unit("math", "math", "facts MATH_VALUE = CORE_VALUE * 2;", vec!["core".to_string()]),
        create_test_unit("app", "app", "facts APP_VALUE = UTIL_VALUE + MATH_VALUE;", 
                        vec!["utils".to_string(), "math".to_string()]),
        create_test_unit("main", "main", "facts MAIN_VALUE = APP_VALUE;", vec!["app".to_string()]),
    ];
    
    // Compile everything
    let results = optimizer.compile_incremental(units)?;
    assert_eq!(results.len(), 5);
    
    // All should succeed (simple variable declarations)
    for (_, result) in &results {
        assert!(result.is_ok());
    }
    
    // Test cache invalidation with dependency tracking
    let changed_files = vec![PathBuf::from("core.csd")];
    optimizer.invalidate_cache_for_changes(&changed_files)?;
    
    let stats = optimizer.get_statistics();
    assert!(stats.cache_hit_rate < 1.0); // Some cache should be invalidated
    
    Ok(())
}

#[test]
fn test_compilation_statistics() -> Result<()> {
    let config = OptimizationConfig {
        enable_parallel_compilation: true,
        enable_incremental_compilation: true,
        max_parallel_threads: 4,
        ..Default::default()
    };
    
    let optimizer = CompilationSpeedOptimizer::new(&config)?;
    
    let units = vec![
        create_test_unit("unit1", "module1", "facts a = 1;", vec![]),
        create_test_unit("unit2", "module2", "facts b = 2;", vec![]),
        create_test_unit("unit3", "module3", "facts c = 3;", vec![]),
    ];
    
    let start_time = std::time::Instant::now();
    let results = optimizer.compile_incremental(units)?;
    let compilation_time = start_time.elapsed();
    
    assert_eq!(results.len(), 3);
    
    let stats = optimizer.get_statistics();
    assert_eq!(stats.total_units, 3);
    assert_eq!(stats.completed_units, 3);
    assert_eq!(stats.failed_units, 0);
    assert!(stats.total_compilation_time <= compilation_time + Duration::from_millis(10));
    assert!(stats.parallelization_efficiency >= 0.0);
    
    // Generate comprehensive report
    let report = optimizer.generate_performance_report();
    assert!(report.contains("Comprehensive Compilation Performance Report"));
    assert!(report.contains("Overall Statistics"));
    assert!(report.contains("Cache Performance"));
    assert!(report.contains("Parallel Processing"));
    assert!(report.contains("Speed Metrics"));
    
    Ok(())
}

#[test]
fn test_cache_size_management() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let cache = AstCache::new(temp_dir.path().to_path_buf())?;
    
    // Store multiple ASTs in cache
    for i in 0..10 {
        let unit = create_test_unit(
            &format!("unit_{}", i),
            &format!("module_{}", i),
            &format!("facts value_{} = {};", i, i * 10),
            vec![]
        );
        
        let lexer = Lexer::new(unit.source_code.clone());
        let mut parser = Parser::new(lexer)?;
        let program = parser.parse_program()?;
        
        cache.store_ast(&unit, program, 100 + i as u64)?;
    }
    
    let stats = cache.get_statistics();
    assert_eq!(stats.cache_hits, 0); // No hits during storage
    assert_eq!(stats.cache_misses, 10); // 10 misses during get_cached_ast calls
    assert!(stats.total_cache_size_bytes > 0);
    
    // Test cache clearing
    cache.clear()?;
    let cleared_stats = cache.get_statistics();
    assert_eq!(cleared_stats.cache_hits, 0);
    assert_eq!(cleared_stats.cache_misses, 0);
    assert_eq!(cleared_stats.total_cache_size_bytes, 0);
    
    Ok(())
}

#[test]
fn test_content_hash_consistency() -> Result<()> {
    let unit1 = create_test_unit("test", "test", "facts x = 42;", vec![]);
    let unit2 = create_test_unit("test", "test", "facts x = 42;", vec![]);
    let unit3 = create_test_unit("test", "test", "facts x = 43;", vec![]);
    
    // Same content should produce same hash
    assert_eq!(unit1.content_hash, unit2.content_hash);
    
    // Different content should produce different hash
    assert_ne!(unit1.content_hash, unit3.content_hash);
    
    // Hash should include dependencies
    let unit4 = create_test_unit("test", "test", "facts x = 42;", vec!["dep1".to_string()]);
    assert_ne!(unit1.content_hash, unit4.content_hash);
    
    Ok(())
}

#[cfg(test)]
mod benchmarks {
    use super::*;
    use std::time::Instant;
    
    #[test]
    fn benchmark_compilation_speed() -> Result<()> {
        let config = OptimizationConfig {
            enable_parallel_compilation: true,
            enable_incremental_compilation: true,
            max_parallel_threads: 8,
            ..Default::default()
        };
        
        let optimizer = CompilationSpeedOptimizer::new(&config)?;
        
        // Create many compilation units
        let units: Vec<_> = (0..100)
            .map(|i| create_test_unit(
                &format!("unit_{}", i),
                &format!("module_{}", i),
                &format!("facts value_{} = {} + {} * 2;", i, i, i + 1),
                if i > 0 { vec![format!("module_{}", i - 1)] } else { vec![] }
            ))
            .collect();
        
        println!("Benchmarking compilation of {} units...", units.len());
        
        // First compilation (cold)
        let start = Instant::now();
        let results1 = optimizer.compile_incremental(units.clone())?;
        let cold_time = start.elapsed();
        
        assert_eq!(results1.len(), 100);
        println!("Cold compilation: {}ms", cold_time.as_millis());
        
        // Second compilation (warm cache)
        let start = Instant::now();
        let results2 = optimizer.compile_incremental(units)?;
        let warm_time = start.elapsed();
        
        assert_eq!(results2.len(), 100);
        println!("Warm compilation: {}ms", warm_time.as_millis());
        
        // Warm should be significantly faster
        let speedup = cold_time.as_millis() as f64 / warm_time.as_millis() as f64;
        println!("Cache speedup: {:.1}x", speedup);
        
        // Generate performance report
        let report = optimizer.generate_performance_report();
        println!("\n{}", report);
        
        // Warm compilation should be at least 2x faster
        assert!(speedup >= 2.0, "Cache speedup was only {:.1}x, expected at least 2x", speedup);
        
        Ok(())
    }
}
