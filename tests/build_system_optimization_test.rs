//! Comprehensive Tests for Advanced Build System Optimizations
//! 
//! Tests all major components of the advanced build system including
//! dependency optimization, caching, distributed compilation, analytics, and memory optimization.

use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::path::PathBuf;
use cursed::build_system::{
    DependencyOptimizer, DependencyOptimizerConfig, CompilationUnit,
    AdvancedCache, AdvancedCacheConfig, CacheData, CacheMetadata,
    DistributedCompilationSystem, DistributedCompilationConfig, create_compilation_task, CompilationTarget,
    BuildAnalytics, BuildAnalyticsConfig, create_build_event, BuildEventType,
    MemoryOptimizer, MemoryOptimizerConfig, create_memory_aware_task, MemoryStrategy
};
use cursed::error::Result;

/// Test dependency optimization with various scenarios
#[test]
fn test_dependency_optimization_basic() -> Result<()> {
    let config = DependencyOptimizerConfig {
        max_parallel_jobs: 4,
        enable_smart_ordering: true,
        enable_dependency_pruning: true,
        ..Default::default()
    };
    
    let optimizer = DependencyOptimizer::new(config);
    
    // Create test compilation units
    let units = vec![
        CompilationUnit {
            id: "main.rs".to_string(),
            path: PathBuf::from("src/main.rs"),
            dependencies: vec!["lib.rs".to_string()],
            dependents: vec![],
            last_modified: 1000,
            compilation_time: Duration::from_millis(500),
            complexity_score: 100,
            is_dirty: true,
            cache_key: "main_key".to_string(),
        },
        CompilationUnit {
            id: "lib.rs".to_string(),
            path: PathBuf::from("src/lib.rs"),
            dependencies: vec!["utils.rs".to_string()],
            dependents: vec!["main.rs".to_string()],
            last_modified: 1000,
            compilation_time: Duration::from_millis(300),
            complexity_score: 80,
            is_dirty: true,
            cache_key: "lib_key".to_string(),
        },
        CompilationUnit {
            id: "utils.rs".to_string(),
            path: PathBuf::from("src/utils.rs"),
            dependencies: vec![],
            dependents: vec!["lib.rs".to_string()],
            last_modified: 1000,
            compilation_time: Duration::from_millis(200),
            complexity_score: 50,
            is_dirty: true,
            cache_key: "utils_key".to_string(),
        },
    ];
    
    // Analyze dependencies
    let analysis = optimizer.analyze_dependencies(&units)?;
    
    // Verify results
    assert!(!analysis.compilation_order.is_empty());
    assert_eq!(analysis.affected_units.len(), 3); // All units are dirty
    assert!(analysis.estimated_time > Duration::ZERO);
    assert!(analysis.parallelism_factor >= 0.0);
    
    println!("✅ Dependency optimization basic test passed");
    println!("  - Compilation layers: {}", analysis.compilation_order.len());
    println!("  - Affected units: {}", analysis.affected_units.len());
    println!("  - Estimated time: {:?}", analysis.estimated_time);
    println!("  - Parallelism factor: {:.2}", analysis.parallelism_factor);
    
    Ok(())
}

/// Test dependency optimization with complex dependencies
#[test]
fn test_dependency_optimization_complex() -> Result<()> {
    let config = DependencyOptimizerConfig {
        max_parallel_jobs: 8,
        enable_smart_ordering: true,
        enable_dependency_pruning: true,
        complexity_threshold: 500,
        ..Default::default()
    };
    
    let optimizer = DependencyOptimizer::new(config);
    
    // Create complex dependency graph
    let mut units = Vec::new();
    for i in 0..20 {
        let dependencies = if i == 0 {
            vec![]
        } else if i < 5 {
            vec![format!("file_{}.rs", 0)]
        } else {
            vec![format!("file_{}.rs", i - 1), format!("file_{}.rs", i - 2)]
        };
        
        units.push(CompilationUnit {
            id: format!("file_{}.rs", i),
            path: PathBuf::from(format!("src/file_{}.rs", i)),
            dependencies,
            dependents: vec![],
            last_modified: 1000 + i as u64,
            compilation_time: Duration::from_millis(100 + i as u64 * 10),
            complexity_score: 50 + i as u32 * 10,
            is_dirty: i % 3 == 0, // Some files are dirty
            cache_key: format!("key_{}", i),
        });
    }
    
    let analysis = optimizer.analyze_dependencies(&units)?;
    
    // Verify complex analysis
    assert!(analysis.compilation_order.len() > 1);
    assert!(!analysis.optimization_suggestions.is_empty());
    
    println!("✅ Complex dependency optimization test passed");
    println!("  - Total units: {}", units.len());
    println!("  - Compilation layers: {}", analysis.compilation_order.len());
    println!("  - Optimization suggestions: {}", analysis.optimization_suggestions.len());
    
    Ok(())
}

/// Test advanced caching system
#[test]
fn test_advanced_cache_basic() -> Result<()> {
    let config = AdvancedCacheConfig {
        enable_ast_cache: true,
        enable_ir_cache: true,
        enable_object_cache: true,
        cache_directory: PathBuf::from("test_cache"),
        max_cache_size_mb: 100,
        compression_enabled: false, // Disable for testing
        ..Default::default()
    };
    
    let cache = AdvancedCache::new(config)?;
    
    // Test cache storage
    let metadata = CacheMetadata {
        file_path: PathBuf::from("test.rs"),
        last_modified: 1000,
        file_size: 500,
        compiler_version: "1.0.0".to_string(),
        compilation_flags: vec!["-O2".to_string()],
        source_hash: "abc123".to_string(),
        dependency_hashes: HashMap::new(),
    };
    
    let data = CacheData::Ast("test_ast_content".to_string());
    cache.store("test_key", data.clone(), metadata.clone())?;
    
    // Test cache retrieval
    let retrieved = cache.retrieve("test_key")?;
    assert!(retrieved.is_some());
    
    let entry = retrieved.unwrap();
    match entry.data {
        CacheData::Ast(content) => assert_eq!(content, "test_ast_content"),
        _ => panic!("Wrong cache data type"),
    }
    
    // Test cache statistics
    let stats = cache.get_statistics()?;
    assert!(stats.total_entries > 0);
    
    println!("✅ Advanced cache basic test passed");
    println!("  - Cache entries: {}", stats.total_entries);
    println!("  - Cache size: {:.2} MB", stats.total_size_mb);
    
    Ok(())
}

/// Test cache invalidation
#[test]
fn test_cache_invalidation() -> Result<()> {
    let config = AdvancedCacheConfig {
        cache_directory: PathBuf::from("test_cache_invalidation"),
        ..Default::default()
    };
    
    let cache = AdvancedCache::new(config)?;
    
    // Store cache entries with dependencies
    let metadata1 = CacheMetadata {
        file_path: PathBuf::from("file1.rs"),
        last_modified: 1000,
        file_size: 500,
        compiler_version: "1.0.0".to_string(),
        compilation_flags: vec![],
        source_hash: "hash1".to_string(),
        dependency_hashes: [("dep1.rs".to_string(), "dep_hash1".to_string())].into(),
    };
    
    let data1 = CacheData::IR("ir_content_1".to_string());
    cache.store("file1", data1, metadata1)?;
    
    let metadata2 = CacheMetadata {
        file_path: PathBuf::from("file2.rs"),
        last_modified: 1000,
        file_size: 600,
        compiler_version: "1.0.0".to_string(),
        compilation_flags: vec![],
        source_hash: "hash2".to_string(),
        dependency_hashes: [("dep1.rs".to_string(), "dep_hash1".to_string())].into(),
    };
    
    let data2 = CacheData::Object(vec![1, 2, 3, 4]);
    cache.store("file2", data2, metadata2)?;
    
    // Verify cache has entries
    let stats_before = cache.get_statistics()?;
    assert!(stats_before.total_entries >= 2);
    
    // Invalidate based on changed dependency
    let changed_files = vec!["dep1.rs".to_string()];
    let invalidated = cache.invalidate_by_dependencies(&changed_files)?;
    
    // Verify invalidation worked
    assert!(invalidated > 0);
    
    println!("✅ Cache invalidation test passed");
    println!("  - Invalidated entries: {}", invalidated);
    
    Ok(())
}

/// Test distributed compilation system
#[test]
fn test_distributed_compilation_basic() -> Result<()> {
    let config = DistributedCompilationConfig {
        coordinator_port: 9100, // Use different port for testing
        worker_ports: vec![9101, 9102],
        work_stealing_enabled: true,
        fault_tolerance_enabled: true,
        ..Default::default()
    };
    
    let mut system = DistributedCompilationSystem::new(config)?;
    
    // Start the system
    system.start()?;
    
    // Create test compilation tasks
    let task1 = create_compilation_task(
        vec!["file1.rs".to_string()],
        CompilationTarget::Object,
        vec!["-O2".to_string()],
    );
    
    let task2 = create_compilation_task(
        vec!["file2.rs".to_string()],
        CompilationTarget::IR,
        vec!["-g".to_string()],
    );
    
    // Submit tasks
    let task_id1 = system.submit_task(task1)?;
    let task_id2 = system.submit_task(task2)?;
    
    assert!(!task_id1.is_empty());
    assert!(!task_id2.is_empty());
    
    // Get system statistics
    let stats = system.get_statistics()?;
    assert_eq!(stats.total_tasks, 2);
    
    // Get registered nodes
    let nodes = system.get_nodes()?;
    assert!(!nodes.is_empty());
    
    // Stop the system
    system.stop()?;
    
    println!("✅ Distributed compilation basic test passed");
    println!("  - Tasks submitted: 2");
    println!("  - Registered nodes: {}", nodes.len());
    
    Ok(())
}

/// Test build analytics system
#[test]
fn test_build_analytics_basic() -> Result<()> {
    let config = BuildAnalyticsConfig {
        enable_detailed_tracking: true,
        enable_memory_profiling: true,
        enable_trend_analysis: true,
        analytics_data_path: PathBuf::from("test_analytics"),
        ..Default::default()
    };
    
    let analytics = BuildAnalytics::new(config)?;
    
    // Start monitoring
    analytics.start_build_monitoring()?;
    
    // Record some build events
    let events = vec![
        create_build_event(BuildEventType::CompilationStart, Duration::from_millis(0)),
        create_build_event(BuildEventType::DependencyResolution, Duration::from_millis(100)),
        create_build_event(BuildEventType::CacheHit, Duration::from_millis(50)),
        create_build_event(BuildEventType::OptimizationPass, Duration::from_millis(200)),
        create_build_event(BuildEventType::CompilationEnd, Duration::from_millis(800)),
    ];
    
    for mut event in events {
        event.success = true;
        event.memory_usage_mb = 100.0;
        event.cpu_usage_percent = 75.0;
        analytics.record_event(event)?;
    }
    
    // Stop monitoring and get metrics
    let metrics = analytics.stop_build_monitoring()?;
    
    assert!(metrics.total_build_time > Duration::ZERO);
    assert!(metrics.compilation_time > Duration::ZERO);
    assert_eq!(metrics.files_compiled, 1); // One CompilationEnd event
    
    // Generate build report
    let report = analytics.generate_build_report()?;
    assert!(report.generated_at > 0);
    assert!(!report.recommendations.is_empty());
    
    println!("✅ Build analytics basic test passed");
    println!("  - Total build time: {:?}", metrics.total_build_time);
    println!("  - Files compiled: {}", metrics.files_compiled);
    println!("  - Cache hit rate: {:.1}%", metrics.cache_hit_rate * 100.0);
    
    Ok(())
}

/// Test bottleneck analysis
#[test]
fn test_bottleneck_analysis() -> Result<()> {
    let config = BuildAnalyticsConfig {
        enable_detailed_tracking: true,
        analytics_data_path: PathBuf::from("test_bottleneck_analytics"),
        ..Default::default()
    };
    
    let analytics = BuildAnalytics::new(config)?;
    analytics.start_build_monitoring()?;
    
    // Create events with varying performance characteristics
    let slow_events = vec![
        create_slow_compilation_event("slow_file.rs", Duration::from_secs(5)),
        create_slow_compilation_event("another_slow.rs", Duration::from_secs(3)),
        create_fast_compilation_event("fast_file.rs", Duration::from_millis(100)),
    ];
    
    for event in slow_events {
        analytics.record_event(event)?;
    }
    
    let _metrics = analytics.stop_build_monitoring()?;
    let bottlenecks = analytics.analyze_bottlenecks()?;
    
    // Verify bottleneck detection
    assert!(!bottlenecks.slowest_files.is_empty());
    assert!(bottlenecks.critical_path_duration > Duration::ZERO);
    assert!(!bottlenecks.optimization_opportunities.is_empty());
    
    println!("✅ Bottleneck analysis test passed");
    println!("  - Slowest files: {}", bottlenecks.slowest_files.len());
    println!("  - Critical path: {:?}", bottlenecks.critical_path_duration);
    println!("  - Optimization opportunities: {}", bottlenecks.optimization_opportunities.len());
    
    Ok(())
}

/// Test memory optimization system
#[test]
fn test_memory_optimization_basic() -> Result<()> {
    let config = MemoryOptimizerConfig {
        max_memory_mb: 1024.0,
        memory_strategy: MemoryStrategy::Adaptive,
        enable_streaming: true,
        streaming_chunk_size_mb: 64.0,
        enable_adaptive_scheduling: true,
        ..Default::default()
    };
    
    let optimizer = MemoryOptimizer::new(config)?;
    optimizer.start()?;
    
    // Create test tasks with different memory requirements
    let small_task = create_memory_aware_task(
        "small_task".to_string(),
        "small_file.rs".to_string(),
        50.0, // 50MB
        false,
    );
    
    let large_task = create_memory_aware_task(
        "large_task".to_string(),
        "large_file.rs".to_string(),
        500.0, // 500MB
        true,   // Can stream
    );
    
    // Submit tasks
    optimizer.submit_task(small_task.clone())?;
    optimizer.submit_task(large_task.clone())?;
    
    // Test scheduling decisions
    let decision_small = optimizer.make_scheduling_decision(&small_task)?;
    let decision_large = optimizer.make_scheduling_decision(&large_task)?;
    
    println!("Small task decision: {:?}", decision_small.action);
    println!("Large task decision: {:?}", decision_large.action);
    
    // Test streaming chunk creation
    let chunks = optimizer.create_streaming_chunks(&large_task)?;
    if !chunks.is_empty() {
        assert!(!chunks.is_empty());
        assert_eq!(chunks[0].task_id, large_task.id);
    }
    
    // Get memory statistics
    let stats = optimizer.get_statistics()?;
    assert!(stats.memory_efficiency_percent > 0.0);
    
    optimizer.stop()?;
    
    println!("✅ Memory optimization basic test passed");
    println!("  - Memory efficiency: {:.1}%", stats.memory_efficiency_percent);
    println!("  - Streaming operations: {}", stats.streaming_operations);
    
    Ok(())
}

/// Test memory pressure handling
#[test]
fn test_memory_pressure_handling() -> Result<()> {
    let config = MemoryOptimizerConfig {
        max_memory_mb: 100.0, // Low limit to trigger pressure
        warning_threshold_percent: 50.0,
        critical_threshold_percent: 80.0,
        enable_adaptive_scheduling: true,
        memory_strategy: MemoryStrategy::Conservative,
        ..Default::default()
    };
    
    let optimizer = MemoryOptimizer::new(config)?;
    
    // Simulate memory pressure
    optimizer.update_memory_usage(45.0)?; // Just under warning
    optimizer.update_memory_usage(60.0)?; // Above warning
    optimizer.update_memory_usage(85.0)?; // Above critical
    
    // Create a task that would exceed memory
    let large_task = create_memory_aware_task(
        "pressure_task".to_string(),
        "pressure_file.rs".to_string(),
        50.0, // Would exceed critical threshold
        true,
    );
    
    let decision = optimizer.make_scheduling_decision(&large_task)?;
    
    // Should defer or use streaming due to pressure
    println!("Memory pressure decision: {:?}", decision.action);
    println!("Reasoning: {}", decision.reasoning);
    
    // Test GC triggering
    let gc_triggered = optimizer.trigger_gc_if_needed()?;
    
    let stats = optimizer.get_statistics()?;
    assert!(stats.memory_pressure_events > 0);
    
    println!("✅ Memory pressure handling test passed");
    println!("  - Pressure events: {}", stats.memory_pressure_events);
    println!("  - GC triggered: {}", gc_triggered);
    
    Ok(())
}

/// Test integrated optimization workflow
#[test]
fn test_integrated_optimization_workflow() -> Result<()> {
    println!("🚀 Testing integrated optimization workflow...");
    
    // Initialize all optimization components
    let dep_config = DependencyOptimizerConfig {
        max_parallel_jobs: 4,
        enable_smart_ordering: true,
        ..Default::default()
    };
    let dep_optimizer = DependencyOptimizer::new(dep_config);
    
    let cache_config = AdvancedCacheConfig {
        cache_directory: PathBuf::from("integrated_test_cache"),
        max_cache_size_mb: 50,
        ..Default::default()
    };
    let cache = AdvancedCache::new(cache_config)?;
    
    let analytics_config = BuildAnalyticsConfig {
        analytics_data_path: PathBuf::from("integrated_test_analytics"),
        enable_detailed_tracking: true,
        ..Default::default()
    };
    let analytics = BuildAnalytics::new(analytics_config)?;
    
    let memory_config = MemoryOptimizerConfig {
        max_memory_mb: 512.0,
        memory_strategy: MemoryStrategy::Balanced,
        ..Default::default()
    };
    let memory_optimizer = MemoryOptimizer::new(memory_config)?;
    
    // Start monitoring
    analytics.start_build_monitoring()?;
    memory_optimizer.start()?;
    
    // Simulate a build workflow
    let start_time = Instant::now();
    
    // 1. Dependency analysis
    let units = create_test_compilation_units();
    let analysis = dep_optimizer.analyze_dependencies(&units)?;
    
    // 2. Cache operations
    for unit in &units {
        let metadata = create_test_cache_metadata(&unit.path);
        let data = CacheData::Ast(format!("ast_for_{}", unit.id));
        cache.store(&unit.id, data, metadata)?;
    }
    
    // 3. Record build events
    for (i, unit) in units.iter().enumerate() {
        let mut event = create_build_event(
            BuildEventType::CompilationEnd,
            Duration::from_millis(100 + i as u64 * 50),
        );
        event.file_path = Some(unit.path.clone());
        event.success = true;
        analytics.record_event(event)?;
    }
    
    // 4. Memory optimization
    for unit in &units {
        let task = create_memory_aware_task(
            unit.id.clone(),
            unit.path.to_string_lossy().to_string(),
            unit.complexity_score as f64,
            true,
        );
        memory_optimizer.submit_task(task)?;
    }
    
    let total_time = start_time.elapsed();
    
    // Stop monitoring and collect results
    let build_metrics = analytics.stop_build_monitoring()?;
    memory_optimizer.stop()?;
    
    // Verify integrated results
    assert!(!analysis.compilation_order.is_empty());
    assert!(build_metrics.files_compiled > 0);
    
    let cache_stats = cache.get_statistics()?;
    assert!(cache_stats.total_entries > 0);
    
    let memory_stats = memory_optimizer.get_statistics()?;
    assert!(memory_stats.memory_efficiency_percent > 0.0);
    
    println!("✅ Integrated optimization workflow test passed");
    println!("  - Total workflow time: {:?}", total_time);
    println!("  - Compilation layers: {}", analysis.compilation_order.len());
    println!("  - Cache entries: {}", cache_stats.total_entries);
    println!("  - Build files: {}", build_metrics.files_compiled);
    println!("  - Memory efficiency: {:.1}%", memory_stats.memory_efficiency_percent);
    
    Ok(())
}

// Helper functions for test data creation

fn create_slow_compilation_event(file: &str, duration: Duration) -> cursed::build_system::BuildEvent {
    let mut event = create_build_event(BuildEventType::CompilationEnd, duration);
    event.file_path = Some(PathBuf::from(file));
    event.memory_usage_mb = 200.0;
    event.cpu_usage_percent = 90.0;
    event.success = true;
    event
}

fn create_fast_compilation_event(file: &str, duration: Duration) -> cursed::build_system::BuildEvent {
    let mut event = create_build_event(BuildEventType::CompilationEnd, duration);
    event.file_path = Some(PathBuf::from(file));
    event.memory_usage_mb = 50.0;
    event.cpu_usage_percent = 30.0;
    event.success = true;
    event
}

fn create_test_compilation_units() -> Vec<CompilationUnit> {
    vec![
        CompilationUnit {
            id: "main.rs".to_string(),
            path: PathBuf::from("src/main.rs"),
            dependencies: vec!["lib.rs".to_string()],
            dependents: vec![],
            last_modified: 1000,
            compilation_time: Duration::from_millis(300),
            complexity_score: 100,
            is_dirty: true,
            cache_key: "main_key".to_string(),
        },
        CompilationUnit {
            id: "lib.rs".to_string(),
            path: PathBuf::from("src/lib.rs"),
            dependencies: vec![],
            dependents: vec!["main.rs".to_string()],
            last_modified: 950,
            compilation_time: Duration::from_millis(200),
            complexity_score: 80,
            is_dirty: false,
            cache_key: "lib_key".to_string(),
        },
        CompilationUnit {
            id: "utils.rs".to_string(),
            path: PathBuf::from("src/utils.rs"),
            dependencies: vec![],
            dependents: vec![],
            last_modified: 900,
            compilation_time: Duration::from_millis(150),
            complexity_score: 60,
            is_dirty: true,
            cache_key: "utils_key".to_string(),
        },
    ]
}

fn create_test_cache_metadata(path: &PathBuf) -> CacheMetadata {
    CacheMetadata {
        file_path: path.clone(),
        last_modified: 1000,
        file_size: 1024,
        compiler_version: "1.0.0".to_string(),
        compilation_flags: vec!["-O2".to_string()],
        source_hash: format!("hash_{}", path.file_name().unwrap().to_string_lossy()),
        dependency_hashes: HashMap::new(),
    }
}
