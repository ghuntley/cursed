//! Build Analytics Integration Tests
//!
//! Comprehensive tests for the build system analytics and optimization features.

use cursed::build_system::{
    analytics::{BuildAnalytics, BuildAnalyticsConfig, BuildEvent, BuildEventType, create_build_event},
    advanced_cache::{AdvancedCache, AdvancedCacheConfig, CacheData, CacheMetadata},
    memory_optimizer::{MemoryOptimizer, MemoryOptimizerConfig, create_memory_aware_task},
    incremental_cache::{IncrementalCache, CacheManager},
};
use std::path::PathBuf;
use std::time::Duration;
use std::collections::HashMap;
use tempfile::tempdir;

#[test]
fn test_build_analytics_creation_and_configuration() {
    let temp_dir = tempdir().unwrap();
    let config = BuildAnalyticsConfig {
        analytics_data_path: temp_dir.path().to_path_buf(),
        enable_detailed_tracking: true,
        enable_memory_profiling: true,
        enable_cpu_profiling: true,
        ..Default::default()
    };
    
    let analytics = BuildAnalytics::new(config);
    assert!(analytics.is_ok());
    
    let analytics = analytics.unwrap();
    
    // Test starting and stopping monitoring
    assert!(analytics.start_build_monitoring().is_ok());
    assert!(analytics.stop_build_monitoring().is_ok());
}

#[test]
fn test_build_event_recording_and_analysis() {
    let temp_dir = tempdir().unwrap();
    let config = BuildAnalyticsConfig {
        analytics_data_path: temp_dir.path().to_path_buf(),
        enable_detailed_tracking: true,
        ..Default::default()
    };
    
    let analytics = BuildAnalytics::new(config).unwrap();
    
    // Start monitoring
    analytics.start_build_monitoring().unwrap();
    
    // Record some build events
    let events = vec![
        create_build_event(BuildEventType::CompilationStart, Duration::from_millis(0)),
        create_build_event(BuildEventType::CompilationEnd, Duration::from_millis(500)),
        create_build_event(BuildEventType::CacheHit, Duration::from_millis(10)),
        create_build_event(BuildEventType::CacheMiss, Duration::from_millis(5)),
        create_build_event(BuildEventType::Linking, Duration::from_millis(200)),
    ];
    
    for event in events {
        analytics.record_event(event).unwrap();
    }
    
    // Stop monitoring and get metrics
    let metrics = analytics.stop_build_monitoring().unwrap();
    
    assert!(metrics.files_compiled > 0);
    assert!(metrics.cache_hit_rate >= 0.0 && metrics.cache_hit_rate <= 1.0);
    assert!(metrics.total_build_time > Duration::ZERO);
}

#[test]
fn test_build_report_generation() {
    let temp_dir = tempdir().unwrap();
    let config = BuildAnalyticsConfig {
        analytics_data_path: temp_dir.path().to_path_buf(),
        report_generation_enabled: true,
        ..Default::default()
    };
    
    let analytics = BuildAnalytics::new(config).unwrap();
    
    // Generate a build report
    let report = analytics.generate_build_report();
    assert!(report.is_ok());
    
    let report = report.unwrap();
    assert!(report.generated_at > 0);
    assert!(!report.recommendations.is_empty());
}

#[test]
fn test_bottleneck_analysis() {
    let temp_dir = tempdir().unwrap();
    let config = BuildAnalyticsConfig {
        analytics_data_path: temp_dir.path().to_path_buf(),
        ..Default::default()
    };
    
    let analytics = BuildAnalytics::new(config).unwrap();
    
    analytics.start_build_monitoring().unwrap();
    
    // Simulate some slow compilation events
    let slow_events = vec![
        BuildEvent {
            id: "slow_file_1".to_string(),
            event_type: BuildEventType::CompilationEnd,
            timestamp: 1000,
            duration: Duration::from_secs(5),
            file_path: Some(PathBuf::from("src/slow_module.csd")),
            module_name: Some("slow_module".to_string()),
            memory_usage_mb: 512.0,
            cpu_usage_percent: 95.0,
            success: true,
            error_message: None,
            metadata: HashMap::new(),
        },
        BuildEvent {
            id: "memory_intensive".to_string(),
            event_type: BuildEventType::CompilationEnd,
            timestamp: 2000,
            duration: Duration::from_secs(3),
            file_path: Some(PathBuf::from("src/big_file.csd")),
            module_name: Some("big_file".to_string()),
            memory_usage_mb: 1024.0,
            cpu_usage_percent: 80.0,
            success: true,
            error_message: None,
            metadata: HashMap::new(),
        },
    ];
    
    for event in slow_events {
        analytics.record_event(event).unwrap();
    }
    
    analytics.stop_build_monitoring().unwrap();
    
    let bottlenecks = analytics.analyze_bottlenecks().unwrap();
    
    assert!(!bottlenecks.slowest_files.is_empty());
    assert!(!bottlenecks.memory_intensive_operations.is_empty());
    assert!(!bottlenecks.cpu_intensive_operations.is_empty());
    assert!(!bottlenecks.optimization_opportunities.is_empty());
}

#[test]
fn test_advanced_cache_functionality() {
    let temp_dir = tempdir().unwrap();
    let config = AdvancedCacheConfig {
        cache_directory: temp_dir.path().to_path_buf(),
        compression_enabled: true,
        enable_ast_cache: true,
        enable_ir_cache: true,
        max_cache_size_mb: 100,
        ..Default::default()
    };
    
    let cache = AdvancedCache::new(config).unwrap();
    
    // Test storing and retrieving cache entries
    let metadata = CacheMetadata {
        file_path: PathBuf::from("test.csd"),
        last_modified: 1000,
        file_size: 1024,
        compiler_version: "0.1.0".to_string(),
        compilation_flags: vec!["--optimize".to_string()],
        source_hash: "abc123".to_string(),
        dependency_hashes: HashMap::new(),
    };
    
    let data = CacheData::Ast("parsed_ast_data".to_string());
    
    // Store cache entry
    assert!(cache.store("test_key", data, metadata).is_ok());
    
    // Retrieve cache entry
    let retrieved = cache.retrieve("test_key").unwrap();
    assert!(retrieved.is_some());
    
    let entry = retrieved.unwrap();
    assert_eq!(entry.key, "test_key");
    
    // Test cache statistics
    let stats = cache.get_statistics().unwrap();
    assert_eq!(stats.total_entries, 1);
    assert!(stats.total_size_mb > 0.0);
}

#[test]
fn test_cache_optimization_and_eviction() {
    let temp_dir = tempdir().unwrap();
    let config = AdvancedCacheConfig {
        cache_directory: temp_dir.path().to_path_buf(),
        max_cache_size_mb: 1, // Very small cache for testing eviction
        ..Default::default()
    };
    
    let cache = AdvancedCache::new(config).unwrap();
    
    // Fill cache with multiple entries
    for i in 0..10 {
        let metadata = CacheMetadata {
            file_path: PathBuf::from(format!("test{}.csd", i)),
            last_modified: 1000 + i,
            file_size: 1024,
            compiler_version: "0.1.0".to_string(),
            compilation_flags: vec![],
            source_hash: format!("hash{}", i),
            dependency_hashes: HashMap::new(),
        };
        
        let data = CacheData::Ast(format!("ast_data_{}", i));
        let _ = cache.store(&format!("key_{}", i), data, metadata);
    }
    
    // Test cache optimization (should trigger eviction)
    let evicted_count = cache.optimize_cache().unwrap();
    assert!(evicted_count > 0);
    
    let stats = cache.get_statistics().unwrap();
    println!("Cache stats after optimization: entries={}, size_mb={:.2}", 
             stats.total_entries, stats.total_size_mb);
}

#[test]
fn test_memory_optimizer_basic_functionality() {
    let config = MemoryOptimizerConfig {
        max_memory_mb: 1024.0,
        enable_streaming: true,
        enable_adaptive_scheduling: true,
        ..Default::default()
    };
    
    let optimizer = MemoryOptimizer::new(config).unwrap();
    
    // Test starting and stopping
    assert!(optimizer.start().is_ok());
    
    // Create and submit memory-aware tasks
    let task1 = create_memory_aware_task(
        "task1".to_string(),
        "src/file1.csd".to_string(),
        50.0, // 50 MB
        true,
    );
    
    let task2 = create_memory_aware_task(
        "task2".to_string(),
        "src/file2.csd".to_string(),
        200.0, // 200 MB (memory intensive)
        true,
    );
    
    assert!(optimizer.submit_task(task1).is_ok());
    assert!(optimizer.submit_task(task2).is_ok());
    
    // Get statistics
    let stats = optimizer.get_statistics().unwrap();
    assert!(stats.current_usage_mb >= 0.0);
    
    // Test scheduling decisions
    let test_task = create_memory_aware_task(
        "test".to_string(),
        "test.csd".to_string(),
        100.0,
        true,
    );
    
    let decision = optimizer.make_scheduling_decision(&test_task).unwrap();
    assert!(!decision.reasoning.is_empty());
    
    assert!(optimizer.stop().is_ok());
}

#[test]
fn test_incremental_cache_rebuild_detection() {
    let temp_dir = tempdir().unwrap();
    let cache_dir = temp_dir.path().join("cache");
    
    let mut cache = IncrementalCache::new(cache_dir).unwrap();
    
    // Create a test file
    let test_file = temp_dir.path().join("test.csd");
    std::fs::write(&test_file, "// test content").unwrap();
    
    let source_paths = vec![test_file.clone()];
    
    // Should need rebuild (no cache entry)
    assert!(cache.needs_rebuild("test_target", &source_paths).unwrap());
    
    // Add cache entry
    cache.insert(
        "test_target",
        vec![PathBuf::from("output.exe")],
        HashMap::new(),
        1,
    ).unwrap();
    
    // Should not need rebuild now
    assert!(!cache.needs_rebuild("test_target", &source_paths).unwrap());
    
    // Modify the file
    std::thread::sleep(Duration::from_millis(10));
    std::fs::write(&test_file, "// modified content").unwrap();
    
    // Should need rebuild now
    assert!(cache.needs_rebuild("test_target", &source_paths).unwrap());
}

#[test]
fn test_cache_manager_multi_project_support() {
    let temp_dir = tempdir().unwrap();
    let cache_dir = temp_dir.path().to_path_buf();
    
    let mut manager = CacheManager::new(cache_dir).unwrap();
    
    // Get caches for different projects
    let cache1 = manager.get_cache("project1").unwrap();
    cache1.insert("target1", vec![], HashMap::new(), 1).unwrap();
    
    let cache2 = manager.get_cache("project2").unwrap();
    cache2.insert("target2", vec![], HashMap::new(), 1).unwrap();
    
    // Test global statistics
    let stats = manager.get_global_statistics();
    assert_eq!(stats.total_projects, 2);
    assert_eq!(stats.total_entries, 2);
    
    // Test cleanup
    let cleaned = manager.cleanup_all(Duration::from_nanos(1)).unwrap();
    assert_eq!(cleaned, 2); // Should remove all entries due to very short max age
}

#[test]
fn test_performance_trend_analysis() {
    let temp_dir = tempdir().unwrap();
    let config = BuildAnalyticsConfig {
        analytics_data_path: temp_dir.path().to_path_buf(),
        enable_trend_analysis: true,
        enable_regression_detection: true,
        regression_threshold_percent: 20.0,
        ..Default::default()
    };
    
    let analytics = BuildAnalytics::new(config).unwrap();
    
    // Simulate multiple builds with increasing times (regression)
    for i in 0..10 {
        analytics.start_build_monitoring().unwrap();
        
        // Simulate build events with increasing duration
        let event = BuildEvent {
            id: format!("build_{}", i),
            event_type: BuildEventType::CompilationEnd,
            timestamp: 1000 + i * 100,
            duration: Duration::from_millis(100 + i * 50), // Increasing build time
            file_path: Some(PathBuf::from("src/main.csd")),
            module_name: Some("main".to_string()),
            memory_usage_mb: 100.0,
            cpu_usage_percent: 50.0,
            success: true,
            error_message: None,
            metadata: HashMap::new(),
        };
        
        analytics.record_event(event).unwrap();
        analytics.stop_build_monitoring().unwrap();
    }
    
    // Generate report with trend analysis
    let report = analytics.generate_build_report().unwrap();
    
    // Should detect performance degradation
    assert!(!report.trend_analysis.performance_regression_alerts.is_empty());
    
    // Should have performance comparison data
    assert!(report.performance_comparison.compared_to_last_build != 0.0 ||
            report.performance_comparison.compared_to_average != 0.0);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_integration_components_work_together() {
        // This test demonstrates how all the analytics components work together
        let temp_dir = tempdir().unwrap();
        
        // Set up analytics
        let analytics_config = BuildAnalyticsConfig {
            analytics_data_path: temp_dir.path().join("analytics"),
            enable_detailed_tracking: true,
            enable_memory_profiling: true,
            enable_regression_detection: true,
            ..Default::default()
        };
        let analytics = BuildAnalytics::new(analytics_config).unwrap();
        
        // Set up advanced cache
        let cache_config = AdvancedCacheConfig {
            cache_directory: temp_dir.path().join("cache"),
            compression_enabled: true,
            enable_ast_cache: true,
            ..Default::default()
        };
        let cache = AdvancedCache::new(cache_config).unwrap();
        
        // Set up memory optimizer
        let memory_config = MemoryOptimizerConfig {
            max_memory_mb: 512.0,
            enable_adaptive_scheduling: true,
            ..Default::default()
        };
        let memory_optimizer = MemoryOptimizer::new(memory_config).unwrap();
        
        // Start all systems
        analytics.start_build_monitoring().unwrap();
        memory_optimizer.start().unwrap();
        
        // Simulate a build process
        // 1. Record compilation events
        let compilation_event = create_build_event(
            BuildEventType::CompilationStart,
            Duration::from_millis(100)
        );
        analytics.record_event(compilation_event).unwrap();
        
        // 2. Store compilation results in cache
        let metadata = CacheMetadata {
            file_path: PathBuf::from("src/main.csd"),
            last_modified: 1000,
            file_size: 2048,
            compiler_version: "0.1.0".to_string(),
            compilation_flags: vec!["--optimize".to_string()],
            source_hash: "main_hash".to_string(),
            dependency_hashes: HashMap::new(),
        };
        cache.store("main_target", CacheData::Ast("ast_data".to_string()), metadata).unwrap();
        
        // 3. Submit memory-aware tasks
        let task = create_memory_aware_task(
            "main_compilation".to_string(),
            "src/main.csd".to_string(),
            150.0,
            true,
        );
        memory_optimizer.submit_task(task).unwrap();
        
        // Allow some processing time
        std::thread::sleep(Duration::from_millis(100));
        
        // Stop systems and collect results
        let build_metrics = analytics.stop_build_monitoring().unwrap();
        let cache_stats = cache.get_statistics().unwrap();
        let memory_stats = memory_optimizer.get_statistics().unwrap();
        
        // Verify everything worked
        assert!(build_metrics.files_compiled > 0);
        assert!(cache_stats.total_entries > 0);
        assert!(memory_stats.current_usage_mb >= 0.0);
        
        memory_optimizer.stop().unwrap();
        
        println!("Integration test completed successfully!");
        println!("Build metrics: {} files compiled", build_metrics.files_compiled);
        println!("Cache stats: {} entries, {:.2} MB", cache_stats.total_entries, cache_stats.total_size_mb);
        println!("Memory stats: {:.2} MB current usage", memory_stats.current_usage_mb);
    }
}
