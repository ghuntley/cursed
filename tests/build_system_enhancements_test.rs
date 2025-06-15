//! Build System Enhancements Integration Test
//! 
//! Tests all the enhanced functionality implemented across the build system modules

#[cfg(test)]
mod tests {
    use cursed::build_system::{
        analytics::BuildAnalytics,
        advanced_cache::{AdvancedCache, AdvancedCacheConfig, CacheData, CacheMetadata},
        incremental_cache::IncrementalCache,
        parallel_compilation::{ParallelCompiler, ParallelCompilationConfig},
        memory_optimizer::{MemoryOptimizer, MemoryOptimizerConfig, create_memory_aware_task},
        test_executor::{TestExecutor, TestExecutionConfig},
        project_template::{TemplateManager, TemplateContext},
    };
    use std::collections::HashMap;
    use std::path::PathBuf;
    use std::time::Duration;
    use tempfile::tempdir;

    #[test]
    fn test_build_analytics_dependency_analysis() {
        let dir = tempdir().unwrap();
        let config = cursed::build_system::analytics::BuildAnalyticsConfig {
            analytics_data_path: dir.path().to_path_buf(),
            ..Default::default()
        };
        
        let analytics = BuildAnalytics::new(config).expect("Failed to create analytics");
        
        // Test that analytics can be created and basic operations work
        let result = analytics.analyze_bottlenecks();
        assert!(result.is_ok());
        
        let bottlenecks = result.unwrap();
        // The analysis should work even with no data
        assert!(bottlenecks.longest_dependencies.is_empty());
        assert!(bottlenecks.slowest_files.is_empty());
    }

    #[test]
    fn test_advanced_cache_statistics_updates() {
        let dir = tempdir().unwrap();
        let config = AdvancedCacheConfig {
            cache_directory: dir.path().to_path_buf(),
            enable_distributed_cache: false,
            ..Default::default()
        };
        
        let cache = AdvancedCache::new(config).expect("Failed to create cache");
        
        // Test cache statistics
        let stats = cache.get_statistics().expect("Failed to get statistics");
        assert_eq!(stats.total_entries, 0);
        assert_eq!(stats.hit_rate, 0.0);
        
        // Test precomputation capability
        let result = cache.warm_cache(&["test.csd".to_string()]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_incremental_cache_profile_detection() {
        let dir = tempdir().unwrap();
        let cache_dir = dir.path().join("cache");
        
        let cache = IncrementalCache::new(cache_dir).expect("Failed to create incremental cache");
        
        // Test cache validation
        let stats = cache.get_statistics();
        assert_eq!(stats.entry_count, 0);
        
        // Test that cache can be created and operations work
        let needs_rebuild = cache.needs_rebuild("test", &[]);
        assert!(needs_rebuild.is_ok());
    }

    #[test]
    fn test_parallel_compilation_critical_path() {
        let config = ParallelCompilationConfig {
            max_workers: 2,
            scheduling_strategy: cursed::build_system::parallel_compilation::SchedulingStrategy::CriticalPath,
            ..Default::default()
        };
        
        let compiler = ParallelCompiler::new(config);
        assert!(compiler.is_ok());
    }

    #[test]
    fn test_memory_optimizer_chunk_execution() {
        let config = MemoryOptimizerConfig {
            max_memory_mb: 1024.0,
            enable_streaming: true,
            ..Default::default()
        };
        
        let optimizer = MemoryOptimizer::new(config).expect("Failed to create memory optimizer");
        
        // Test memory monitoring
        let stats = optimizer.get_statistics().expect("Failed to get statistics");
        assert_eq!(stats.current_usage_mb, 0.0);
        
        // Test task creation
        let task = create_memory_aware_task(
            "test_task".to_string(),
            "test.csd".to_string(),
            50.0, // 50MB estimated memory
            true, // Can stream
        );
        
        assert_eq!(task.id, "test_task");
        assert_eq!(task.estimated_memory_mb, 50.0);
        assert!(task.can_stream);
    }

    #[test]
    fn test_test_executor_compilation_time_extraction() {
        let config = TestExecutionConfig {
            parallel_threads: 1,
            capture_output: true,
            ..Default::default()
        };
        
        let executor = TestExecutor::new(config);
        
        // Test output parser creation
        let parser = executor.output_parser;
        
        // Test compilation time extraction with mock cargo output
        let mock_output = "Finished dev [unoptimized + debuginfo] target(s) in 2.34s";
        let duration = parser.extract_compilation_time(mock_output);
        assert!(duration.is_some());
        assert_eq!(duration.unwrap().as_secs(), 2);
    }

    #[test]
    fn test_project_template_enhanced_servers() {
        let manager = TemplateManager::new();
        
        // Test that templates are available
        let templates = manager.list_templates();
        assert!(!templates.is_empty());
        
        // Find web template
        let web_template = templates.iter().find(|t| t.name == "web");
        assert!(web_template.is_some());
        
        // Find API template
        let api_template = templates.iter().find(|t| t.name == "api");
        assert!(api_template.is_some());
        
        // Test template generation
        let dir = tempdir().unwrap();
        let mut variables = HashMap::new();
        variables.insert("description".to_string(), "Test web app".to_string());
        variables.insert("port".to_string(), "3000".to_string());
        
        let context = TemplateContext {
            project_name: "test-web-app".to_string(),
            target_dir: dir.path().to_path_buf(),
            variables,
        };
        
        let result = manager.generate_project("web", context);
        assert!(result.is_ok());
        
        // Verify files were created
        assert!(dir.path().join("src").exists());
        assert!(dir.path().join("src/main.csd").exists());
        assert!(dir.path().join("src/server.csd").exists());
        assert!(dir.path().join("static/index.html").exists());
        
        // Verify content was processed
        let server_content = std::fs::read_to_string(dir.path().join("src/server.csd")).unwrap();
        assert!(server_content.contains("test-web-app"));
        assert!(server_content.contains("WebServer"));
        assert!(server_content.contains("handle_connection"));
    }

    #[test]
    fn test_memory_optimizer_adaptive_decisions() {
        let config = MemoryOptimizerConfig {
            max_memory_mb: 1024.0,
            enable_adaptive_scheduling: true,
            memory_strategy: cursed::build_system::memory_optimizer::MemoryStrategy::Adaptive,
            ..Default::default()
        };
        
        let optimizer = MemoryOptimizer::new(config).expect("Failed to create optimizer");
        
        let task = create_memory_aware_task(
            "large_task".to_string(),
            "large_file.csd".to_string(),
            200.0, // 200MB - large task
            true,
        );
        
        // Test scheduling decision
        let decision = optimizer.make_scheduling_decision(&task);
        assert!(decision.is_ok());
        
        let decision = decision.unwrap();
        // Should recommend streaming for large tasks
        assert!(matches!(decision.action, cursed::build_system::memory_optimizer::SchedulingAction::Stream));
    }

    #[test]
    fn test_advanced_cache_precomputation() {
        let dir = tempdir().unwrap();
        let config = AdvancedCacheConfig {
            cache_directory: dir.path().to_path_buf(),
            enable_streaming: true,
            precomputation_enabled: true,
            ..Default::default()
        };
        
        let cache = AdvancedCache::new(config).expect("Failed to create cache");
        
        // Create a test file
        let test_file = dir.path().join("test.csd");
        std::fs::write(&test_file, "// Test CURSED file\nslay main() {\n    io::println(\"Hello\");\n}").unwrap();
        
        // Test precomputation
        let result = cache.warm_cache(&[test_file.to_string_lossy().to_string()]);
        assert!(result.is_ok());
        
        let warmed = result.unwrap();
        assert_eq!(warmed, 1); // Should have warmed 1 file
    }

    #[test]
    fn test_build_system_integration() {
        // Test that all components can work together
        let dir = tempdir().unwrap();
        
        // Create analytics
        let analytics_config = cursed::build_system::analytics::BuildAnalyticsConfig {
            analytics_data_path: dir.path().join("analytics"),
            ..Default::default()
        };
        let analytics = BuildAnalytics::new(analytics_config).expect("Analytics creation failed");
        
        // Create cache
        let cache_config = AdvancedCacheConfig {
            cache_directory: dir.path().join("cache"),
            ..Default::default()
        };
        let cache = AdvancedCache::new(cache_config).expect("Cache creation failed");
        
        // Create memory optimizer
        let memory_config = MemoryOptimizerConfig::default();
        let memory_optimizer = MemoryOptimizer::new(memory_config).expect("Memory optimizer creation failed");
        
        // Test that they can all be created together without conflicts
        assert!(analytics.get_statistics().is_ok());
        assert!(cache.get_statistics().is_ok());
        assert!(memory_optimizer.get_statistics().is_ok());
    }
}
