//! Comprehensive tests for the Performance Optimization System
//! 
//! This test suite validates the complete performance optimization infrastructure
//! including real optimization passes, resource monitoring, benchmarking, and reporting.

use cursed::optimization::{
    PerformanceOptimizationSystem, PerformanceConfig, OptimizationConfig, OptimizationLevel,
    EnhancedBuildProfiler, ProfilerConfig, ReportFormat,
    BenchmarkConfig, BenchmarkType, BenchmarkTestData, ComplexityLevel,
    CompilationUnit, ResourceMonitoringLevel,
};
use cursed::error::Result;
use std::time::Duration;
use std::path::PathBuf;

#[test]
fn test_performance_optimization_system_creation() {
    let perf_config = PerformanceConfig::default();
    let opt_config = OptimizationConfig::default();
    
    let system = PerformanceOptimizationSystem::new(perf_config, opt_config);
    assert!(system.is_ok(), "Should create performance optimization system successfully");
}

#[test]
fn test_performance_optimization_system_monitoring() {
    let perf_config = PerformanceConfig {
        enable_realtime_monitoring: true,
        monitoring_interval_ms: 50,
        ..Default::default()
    };
    let opt_config = OptimizationConfig::default();
    
    let system = PerformanceOptimizationSystem::new(perf_config, opt_config).unwrap();
    
    // Test monitoring start/stop
    assert!(system.start_monitoring().is_ok());
    assert!(system.stop_monitoring().is_ok());
}

#[test]
fn test_optimization_session_management() {
    let perf_config = PerformanceConfig::default();
    let opt_config = OptimizationConfig::default();
    
    let system = PerformanceOptimizationSystem::new(perf_config, opt_config).unwrap();
    
    // Create optimization session
    let session = system.create_session("test_session".to_string());
    assert_eq!(session.name, "test_session");
    assert!(session.id.starts_with("test_session_"));
}

#[test]
fn test_optimization_with_tracking() {
    let perf_config = PerformanceConfig {
        enable_realtime_monitoring: true,
        enable_benchmarking: false,
        monitoring_interval_ms: 10,
        ..Default::default()
    };
    let opt_config = OptimizationConfig::default();
    
    let system = PerformanceOptimizationSystem::new(perf_config, opt_config).unwrap();
    let session = system.create_session("tracking_test".to_string());
    
    // Create test compilation units
    let mut units = vec![
        CompilationUnit::new("test_unit_1".to_string()),
        CompilationUnit::new("test_unit_2".to_string()),
    ];
    
    // Add some test data
    units[0].source_files.push("test1.csd".to_string());
    units[1].source_files.push("test2.csd".to_string());
    
    // Run optimization with tracking
    let results = system.optimize_with_tracking(&mut units, &session);
    assert!(results.is_ok(), "Optimization with tracking should succeed");
    
    let results = results.unwrap();
    assert_eq!(results.unit_results.len(), 2);
    assert!(results.total_time > Duration::from_millis(0));
    assert_eq!(results.session_id, session.id);
}

#[test]
fn test_compilation_speed_benchmark() {
    let perf_config = PerformanceConfig {
        enable_benchmarking: true,
        max_benchmark_iterations: 5,
        ..Default::default()
    };
    let opt_config = OptimizationConfig::default();
    
    let system = PerformanceOptimizationSystem::new(perf_config, opt_config).unwrap();
    
    let benchmark_config = BenchmarkConfig {
        name: "test_compilation_speed".to_string(),
        benchmark_type: BenchmarkType::CompilationSpeed,
        iterations: 3,
        warmup_iterations: 1,
        test_data: BenchmarkTestData {
            unit_count: 10,
            complexity_level: ComplexityLevel::Simple,
            data_size_mb: 1.0,
        },
    };
    
    let results = system.run_benchmark(benchmark_config);
    assert!(results.is_ok(), "Compilation speed benchmark should succeed");
    
    let results = results.unwrap();
    assert_eq!(results.name, "test_compilation_speed");
    assert!(results.statistics.mean_time_ms > 0.0);
    assert!(results.statistics.throughput_ops_per_sec > 0.0);
}

#[test]
fn test_memory_usage_benchmark() {
    let perf_config = PerformanceConfig {
        enable_benchmarking: true,
        max_benchmark_iterations: 5,
        ..Default::default()
    };
    let opt_config = OptimizationConfig::default();
    
    let system = PerformanceOptimizationSystem::new(perf_config, opt_config).unwrap();
    
    let benchmark_config = BenchmarkConfig {
        name: "test_memory_usage".to_string(),
        benchmark_type: BenchmarkType::MemoryUsage,
        iterations: 3,
        warmup_iterations: 0,
        test_data: BenchmarkTestData {
            unit_count: 50,
            complexity_level: ComplexityLevel::Medium,
            data_size_mb: 5.0,
        },
    };
    
    let results = system.run_benchmark(benchmark_config);
    assert!(results.is_ok(), "Memory usage benchmark should succeed");
    
    let results = results.unwrap();
    assert_eq!(results.name, "test_memory_usage");
    assert!(results.statistics.mean_memory_delta_mb >= 0.0);
}

#[test]
fn test_optimization_effectiveness_benchmark() {
    let perf_config = PerformanceConfig {
        enable_benchmarking: true,
        ..Default::default()
    };
    let opt_config = OptimizationConfig::default();
    
    let system = PerformanceOptimizationSystem::new(perf_config, opt_config).unwrap();
    
    let benchmark_config = BenchmarkConfig {
        name: "test_optimization_effectiveness".to_string(),
        benchmark_type: BenchmarkType::OptimizationEffectiveness,
        iterations: 2,
        warmup_iterations: 1,
        test_data: BenchmarkTestData {
            unit_count: 20,
            complexity_level: ComplexityLevel::Complex,
            data_size_mb: 2.0,
        },
    };
    
    let results = system.run_benchmark(benchmark_config);
    assert!(results.is_ok(), "Optimization effectiveness benchmark should succeed");
    
    let results = results.unwrap();
    assert_eq!(results.name, "test_optimization_effectiveness");
}

#[test]
fn test_cache_performance_benchmark() {
    let perf_config = PerformanceConfig {
        enable_benchmarking: true,
        ..Default::default()
    };
    let opt_config = OptimizationConfig::default();
    
    let system = PerformanceOptimizationSystem::new(perf_config, opt_config).unwrap();
    
    let benchmark_config = BenchmarkConfig {
        name: "test_cache_performance".to_string(),
        benchmark_type: BenchmarkType::CachePerformance,
        iterations: 3,
        warmup_iterations: 1,
        test_data: BenchmarkTestData {
            unit_count: 30,
            complexity_level: ComplexityLevel::Medium,
            data_size_mb: 3.0,
        },
    };
    
    let results = system.run_benchmark(benchmark_config);
    assert!(results.is_ok(), "Cache performance benchmark should succeed");
    
    let results = results.unwrap();
    assert_eq!(results.name, "test_cache_performance");
}

#[test]
fn test_enhanced_build_profiler_creation() {
    let config = ProfilerConfig::default();
    let profiler = EnhancedBuildProfiler::new(config);
    assert!(profiler.is_ok(), "Should create enhanced build profiler successfully");
}

#[test]
fn test_enhanced_build_profiler_session() {
    let config = ProfilerConfig {
        enable_realtime_monitoring: true,
        enable_memory_profiling: true,
        enable_cpu_profiling: true,
        enable_io_profiling: true,
        monitoring_interval_ms: 10,
        ..Default::default()
    };
    
    let profiler = EnhancedBuildProfiler::new(config).unwrap();
    
    // Start profiling session
    let session = profiler.start_build_session("test_profiling_session".to_string());
    assert!(session.is_ok(), "Should start profiling session successfully");
    
    let session = session.unwrap();
    assert_eq!(session.name, "test_profiling_session");
    assert!(session.id.contains("test_profiling_session"));
    
    // End profiling session
    let report = profiler.end_build_session(session);
    assert!(report.is_ok(), "Should end profiling session successfully");
    
    let report = report.unwrap();
    assert!(report.total_duration > Duration::from_millis(0));
    assert!(report.performance_summary.overall_performance_score >= 0.0);
}

#[test]
fn test_build_profiler_compilation_unit_profiling() {
    let config = ProfilerConfig {
        enable_memory_profiling: true,
        enable_cpu_profiling: true,
        enable_io_profiling: true,
        ..Default::default()
    };
    
    let profiler = EnhancedBuildProfiler::new(config).unwrap();
    let session = profiler.start_build_session("unit_profiling_test".to_string()).unwrap();
    
    // Create test compilation unit
    let mut unit = CompilationUnit::new("test_unit".to_string());
    unit.source_files.push("test.csd".to_string());
    
    // Profile compilation unit
    let result = profiler.profile_compilation_unit(&unit, &session);
    assert!(result.is_ok(), "Should profile compilation unit successfully");
    
    let result = result.unwrap();
    assert_eq!(result.unit_name, "test_unit");
    assert!(result.compilation_time > Duration::from_millis(0));
    assert!(result.peak_memory_mb >= 0.0);
}

#[test]
fn test_profiler_report_generation() {
    let config = ProfilerConfig {
        report_format: ReportFormat::Json,
        output_directory: Some(PathBuf::from("/tmp")),
        ..Default::default()
    };
    
    let profiler = EnhancedBuildProfiler::new(config).unwrap();
    let session = profiler.start_build_session("report_test".to_string()).unwrap();
    
    // Generate report
    let report = profiler.end_build_session(session).unwrap();
    
    // Test report content
    assert!(!report.session_id.is_empty());
    assert!(!report.session_name.is_empty());
    assert!(report.total_duration >= Duration::from_millis(0));
    assert!(report.performance_summary.overall_performance_score >= 0.0);
    assert!(report.system_metrics.peak_memory_mb >= 0.0);
    assert!(report.system_metrics.average_cpu_percent >= 0.0);
}

#[test]
fn test_profiler_report_export_json() {
    let config = ProfilerConfig::default();
    let profiler = EnhancedBuildProfiler::new(config).unwrap();
    let session = profiler.start_build_session("export_test".to_string()).unwrap();
    
    let report = profiler.end_build_session(session).unwrap();
    
    // Test JSON export
    let temp_path = PathBuf::from("/tmp/test_report.json");
    let result = profiler.export_report(&report, ReportFormat::Json, temp_path.clone());
    assert!(result.is_ok(), "Should export JSON report successfully");
    
    // Clean up
    let _ = std::fs::remove_file(temp_path);
}

#[test]
fn test_profiler_report_export_html() {
    let config = ProfilerConfig::default();
    let profiler = EnhancedBuildProfiler::new(config).unwrap();
    let session = profiler.start_build_session("html_export_test".to_string()).unwrap();
    
    let report = profiler.end_build_session(session).unwrap();
    
    // Test HTML export
    let temp_path = PathBuf::from("/tmp/test_report.html");
    let result = profiler.export_report(&report, ReportFormat::Html, temp_path.clone());
    assert!(result.is_ok(), "Should export HTML report successfully");
    
    // Clean up
    let _ = std::fs::remove_file(temp_path);
}

#[test]
fn test_profiler_report_export_markdown() {
    let config = ProfilerConfig::default();
    let profiler = EnhancedBuildProfiler::new(config).unwrap();
    let session = profiler.start_build_session("markdown_export_test".to_string()).unwrap();
    
    let report = profiler.end_build_session(session).unwrap();
    
    // Test Markdown export
    let temp_path = PathBuf::from("/tmp/test_report.md");
    let result = profiler.export_report(&report, ReportFormat::Markdown, temp_path.clone());
    assert!(result.is_ok(), "Should export Markdown report successfully");
    
    // Clean up
    let _ = std::fs::remove_file(temp_path);
}

#[test]
fn test_performance_config_validation() {
    // Test default configuration
    let config = PerformanceConfig::default();
    assert!(config.enable_realtime_monitoring);
    assert!(config.enable_benchmarking);
    assert_eq!(config.monitoring_interval_ms, 100);
    assert_eq!(config.max_benchmark_iterations, 10);
    assert_eq!(config.max_performance_entries, 10000);
    
    // Test custom configuration
    let custom_config = PerformanceConfig {
        enable_realtime_monitoring: false,
        enable_benchmarking: true,
        monitoring_interval_ms: 50,
        max_benchmark_iterations: 5,
        resource_monitoring_level: ResourceMonitoringLevel::Comprehensive,
        ..Default::default()
    };
    
    assert!(!custom_config.enable_realtime_monitoring);
    assert_eq!(custom_config.monitoring_interval_ms, 50);
    assert_eq!(custom_config.max_benchmark_iterations, 5);
}

#[test]
fn test_profiler_config_validation() {
    // Test default configuration
    let config = ProfilerConfig::default();
    assert!(config.enable_realtime_monitoring);
    assert!(config.enable_memory_profiling);
    assert!(config.enable_cpu_profiling);
    assert!(config.enable_io_profiling);
    assert_eq!(config.monitoring_interval_ms, 100);
    assert_eq!(config.max_profile_entries, 1000);
    
    // Test custom configuration
    let custom_config = ProfilerConfig {
        enable_realtime_monitoring: false,
        enable_memory_profiling: true,
        enable_cpu_profiling: false,
        enable_io_profiling: true,
        monitoring_interval_ms: 200,
        report_format: ReportFormat::Interactive,
        output_directory: Some(PathBuf::from("/custom/output")),
        ..Default::default()
    };
    
    assert!(!custom_config.enable_realtime_monitoring);
    assert!(!custom_config.enable_cpu_profiling);
    assert_eq!(custom_config.monitoring_interval_ms, 200);
    assert!(matches!(custom_config.report_format, ReportFormat::Interactive));
}

#[test]
fn test_benchmark_test_data_creation() {
    let test_data = BenchmarkTestData {
        unit_count: 100,
        complexity_level: ComplexityLevel::Complex,
        data_size_mb: 50.0,
    };
    
    assert_eq!(test_data.unit_count, 100);
    assert!(matches!(test_data.complexity_level, ComplexityLevel::Complex));
    assert_eq!(test_data.data_size_mb, 50.0);
}

#[test]
fn test_system_statistics_tracking() {
    let perf_config = PerformanceConfig::default();
    let opt_config = OptimizationConfig::default();
    
    let system = PerformanceOptimizationSystem::new(perf_config, opt_config).unwrap();
    
    // Initial statistics should be zero
    let stats = system.get_system_statistics();
    assert_eq!(stats.optimizations_completed, 0);
    assert_eq!(stats.total_units_optimized, 0);
    assert_eq!(stats.benchmark_runs, 0);
    assert_eq!(stats.cache_hits, 0);
    assert_eq!(stats.cache_misses, 0);
    assert_eq!(stats.errors_encountered, 0);
}

#[test]
fn test_resource_statistics() {
    let perf_config = PerformanceConfig {
        enable_realtime_monitoring: true,
        resource_monitoring_level: ResourceMonitoringLevel::Detailed,
        ..Default::default()
    };
    let opt_config = OptimizationConfig::default();
    
    let system = PerformanceOptimizationSystem::new(perf_config, opt_config).unwrap();
    
    let stats = system.get_resource_statistics();
    assert!(stats.is_ok(), "Should get resource statistics successfully");
    
    let stats = stats.unwrap();
    assert!(stats.peak_memory_mb >= 0.0);
    assert!(stats.average_memory_mb >= 0.0);
    assert!(stats.peak_cpu_percent >= 0.0);
    assert!(stats.average_cpu_percent >= 0.0);
    assert!(stats.total_io_operations >= 0);
    assert!(stats.monitoring_uptime >= Duration::from_millis(0));
}

#[test]
fn test_performance_analysis_generation() {
    let perf_config = PerformanceConfig {
        enable_prediction: true,
        ..Default::default()
    };
    let opt_config = OptimizationConfig::default();
    
    let system = PerformanceOptimizationSystem::new(perf_config, opt_config).unwrap();
    
    // Test performance analysis for recent optimizations
    let analysis = system.get_performance_analysis(Duration::from_secs(3600));
    assert!(analysis.is_ok(), "Should get performance analysis successfully");
}

#[test]
fn test_multiple_benchmark_runs() {
    let perf_config = PerformanceConfig {
        enable_benchmarking: true,
        max_benchmark_iterations: 3,
        ..Default::default()
    };
    let opt_config = OptimizationConfig::default();
    
    let system = PerformanceOptimizationSystem::new(perf_config, opt_config).unwrap();
    
    // Run multiple benchmarks
    let benchmark_types = vec![
        BenchmarkType::CompilationSpeed,
        BenchmarkType::MemoryUsage,
        BenchmarkType::CachePerformance,
    ];
    
    for (i, benchmark_type) in benchmark_types.into_iter().enumerate() {
        let benchmark_config = BenchmarkConfig {
            name: format!("multi_benchmark_{}", i),
            benchmark_type,
            iterations: 2,
            warmup_iterations: 1,
            test_data: BenchmarkTestData {
                unit_count: 10,
                complexity_level: ComplexityLevel::Simple,
                data_size_mb: 1.0,
            },
        };
        
        let results = system.run_benchmark(benchmark_config);
        assert!(results.is_ok(), "Each benchmark should succeed");
        
        let results = results.unwrap();
        assert!(results.statistics.mean_time_ms >= 0.0);
        assert!(results.statistics.throughput_ops_per_sec >= 0.0);
    }
}

#[test]
fn test_performance_system_configuration_update() {
    let initial_config = PerformanceConfig {
        enable_realtime_monitoring: false,
        monitoring_interval_ms: 100,
        ..Default::default()
    };
    let opt_config = OptimizationConfig::default();
    
    let mut system = PerformanceOptimizationSystem::new(initial_config, opt_config).unwrap();
    
    // Update configuration
    let new_config = PerformanceConfig {
        enable_realtime_monitoring: true,
        monitoring_interval_ms: 50,
        enable_prediction: true,
        ..Default::default()
    };
    
    let result = system.update_config(new_config);
    assert!(result.is_ok(), "Should update configuration successfully");
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_end_to_end_optimization_with_profiling() {
        // Create comprehensive system
        let perf_config = PerformanceConfig {
            enable_realtime_monitoring: true,
            enable_benchmarking: true,
            enable_prediction: false,
            monitoring_interval_ms: 10,
            max_benchmark_iterations: 3,
            resource_monitoring_level: ResourceMonitoringLevel::Comprehensive,
            ..Default::default()
        };
        let opt_config = OptimizationConfig {
            optimization_level: OptimizationLevel::Default,
            enable_parallel: true,
            enable_profiling: true,
            ..Default::default()
        };
        
        let system = PerformanceOptimizationSystem::new(perf_config, opt_config).unwrap();
        
        // Start monitoring
        assert!(system.start_monitoring().is_ok());
        
        // Create optimization session
        let session = system.create_session("integration_test".to_string());
        
        // Create test compilation units
        let mut units = vec![
            CompilationUnit::new("integration_unit_1".to_string()),
            CompilationUnit::new("integration_unit_2".to_string()),
            CompilationUnit::new("integration_unit_3".to_string()),
        ];
        
        for (i, unit) in units.iter_mut().enumerate() {
            unit.source_files.push(format!("integration_test_{}.csd", i));
            unit.dependencies.push("stdlib".to_string());
        }
        
        // Run optimization with tracking
        let optimization_results = system.optimize_with_tracking(&mut units, &session).unwrap();
        
        // Validate optimization results
        assert_eq!(optimization_results.unit_results.len(), 3);
        assert!(optimization_results.total_time > Duration::from_millis(0));
        assert!(optimization_results.performance_analysis.is_some());
        
        let analysis = optimization_results.performance_analysis.unwrap();
        assert_eq!(analysis.units_optimized, 3);
        assert!(analysis.optimization_efficiency >= 0.0);
        
        // Run benchmark
        let benchmark_config = BenchmarkConfig {
            name: "integration_benchmark".to_string(),
            benchmark_type: BenchmarkType::CompilationSpeed,
            iterations: 3,
            warmup_iterations: 1,
            test_data: BenchmarkTestData {
                unit_count: 10,
                complexity_level: ComplexityLevel::Medium,
                data_size_mb: 5.0,
            },
        };
        
        let benchmark_results = system.run_benchmark(benchmark_config).unwrap();
        assert!(benchmark_results.statistics.mean_time_ms > 0.0);
        assert!(benchmark_results.statistics.throughput_ops_per_sec > 0.0);
        
        // Get system statistics
        let system_stats = system.get_system_statistics();
        assert!(system_stats.optimizations_completed > 0);
        assert!(system_stats.benchmark_runs > 0);
        
        // Stop monitoring
        assert!(system.stop_monitoring().is_ok());
    }
    
    #[test]
    fn test_build_profiler_with_performance_system_integration() {
        // Create profiler
        let profiler_config = ProfilerConfig {
            enable_realtime_monitoring: true,
            enable_memory_profiling: true,
            enable_cpu_profiling: true,
            enable_io_profiling: true,
            monitoring_interval_ms: 10,
            ..Default::default()
        };
        
        let profiler = EnhancedBuildProfiler::new(profiler_config).unwrap();
        
        // Start profiling session
        let session = profiler.start_build_session("integration_profiling".to_string()).unwrap();
        
        // Create and profile multiple compilation units
        let units = vec![
            CompilationUnit::new("profiled_unit_1".to_string()),
            CompilationUnit::new("profiled_unit_2".to_string()),
        ];
        
        let mut unit_results = Vec::new();
        for unit in &units {
            let result = profiler.profile_compilation_unit(unit, &session).unwrap();
            unit_results.push(result);
        }
        
        // End session and generate report
        let report = profiler.end_build_session(session).unwrap();
        
        // Validate comprehensive report
        assert!(!report.session_id.is_empty());
        assert_eq!(report.session_name, "integration_profiling");
        assert!(report.total_duration > Duration::from_millis(0));
        assert!(report.performance_summary.overall_performance_score >= 0.0);
        assert!(report.system_metrics.peak_memory_mb >= 0.0);
        
        // Test report export
        let temp_dir = std::env::temp_dir();
        let report_path = temp_dir.join("integration_report.html");
        
        assert!(profiler.export_report(&report, ReportFormat::Html, report_path.clone()).is_ok());
        
        // Verify file was created
        assert!(report_path.exists());
        
        // Clean up
        let _ = std::fs::remove_file(report_path);
    }
}
