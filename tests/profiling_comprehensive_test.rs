// Comprehensive integration tests for CURSED profiling system

use std::time::Duration;
use std::collections::HashMap;

use cursed::profiling::{
    ProfilerBuilder, ProfilerConfig, ProfilerMode, OutputFormat,
    BenchmarkSuite, BenchmarkConfig, ReportGenerator, ReportConfig,
    VisualizationGenerator, VisualizationConfig, PerformanceAnalyzer, AnalysisConfig,
    BuildIntegration, BuildConfig,
};

#[path = "common.rs"]
mod common;

/// Test the complete profiling workflow
#[test]
fn test_complete_profiling_workflow() {
    // init_tracing!();
    common::tracing::setup();
    
    // Create profiler with comprehensive configuration
    let mut profiler = ProfilerBuilder::new()
        .with_modes(vec![
            ProfilerMode::Cpu,
            ProfilerMode::Memory,
            ProfilerMode::Concurrency,
            ProfilerMode::Io,
        ])
        .with_cpu_sampling(200)
        .with_memory_threshold(512)
        .with_output_dir("test_output".to_string())
        .with_format(OutputFormat::Json)
        .build();
    
    // Start profiling session
    let result = profiler.start_session("test_workflow".to_string());
    assert!(result.is_ok());
    assert!(profiler.is_active());
    
    // Simulate some work
    std::thread::sleep(Duration::from_millis(100));
    
    // Stop profiling session
    let profile_data = profiler.stop_session();
    assert!(profile_data.is_ok());
    assert!(!profiler.is_active());
    
    let data = profile_data.unwrap();
    assert_eq!(data.session_name, "test_workflow");
    assert!(data.session_duration > Duration::from_millis(50));
}

/// Test benchmarking framework
#[test]
fn test_benchmarking_framework() {
    // init_tracing!();
    common::tracing::setup();
    
    let config = BenchmarkConfig {
        warmup_iterations: 2,
        measurement_iterations: 5,
        enable_profiling: false,
        regression_threshold: 15.0,
        timeout: Duration::from_secs(30),
        memory_limit: None,
    };
    
    let mut suite = BenchmarkSuite::new("test_suite".to_string(), config);
    
    // Add simple benchmark
    let benchmark = cursed::profiling::benchmarking::Benchmark::new(
        "simple_computation".to_string(),
        || {
            // Simple computation benchmark
            let mut sum = 0;
            for i in 0..1000 {
                sum += i * i;
            }
            let _ = sum;
        },
    );
    
    suite.add_benchmark(benchmark);
    
    // Run benchmarks
    let results = suite.run_all();
    assert!(results.is_ok());
    
    let benchmark_results = results.unwrap();
    assert_eq!(benchmark_results.suite_name, "test_suite");
    assert_eq!(benchmark_results.results.len(), 1);
    
    let simple_result = benchmark_results.results.get("simple_computation");
    assert!(simple_result.is_some());
    
    let result = simple_result.unwrap();
    assert_eq!(result.measurements.len(), 5);
    assert!(result.statistics.mean > Duration::from_nanos(0));
}

/// Test memory profiling capabilities
#[test]
fn test_memory_profiling() {
    // init_tracing!();
    common::tracing::setup();
    
    let mut profiler = cursed::profiling::memory::MemoryProfiler::new(1024);
    
    // Start collection
    let result = profiler.start_collection();
    assert!(result.is_ok());
    assert!(profiler.is_collecting());
    
    // Simulate memory operations
    let result = profiler.track_allocation(
        2048,
        0x1000,
        vec!["test_function".to_string()],
    );
    assert!(result.is_ok());
    
    let result = profiler.track_deallocation(
        0x1000,
        vec!["test_function".to_string()],
    );
    assert!(result.is_ok());
    
    // Get memory usage snapshot
    let snapshot = profiler.get_memory_usage_snapshot();
    assert_eq!(snapshot.allocated_bytes, 0); // Allocated and deallocated
    
    // Stop collection
    let data = profiler.stop_collection();
    assert!(data.is_ok());
    assert!(!profiler.is_collecting());
}

/// Test concurrency profiling
#[test]
fn test_concurrency_profiling() {
    // init_tracing!();
    common::tracing::setup();
    
    let profiler = cursed::profiling::concurrency::ConcurrencyProfiler::new();
    
    // Track goroutine spawn
    let result = profiler.track_goroutine_spawn(
        1,
        None,
        vec!["main".to_string()],
    );
    assert!(result.is_ok());
    
    // Track channel operation
    let result = profiler.track_channel_operation(
        1,
        cursed::profiling::concurrency::ChannelOperation::Send("test".to_string()),
        1,
        Some(Duration::from_millis(10)),
    );
    assert!(result.is_ok());
    
    // Track goroutine completion
    let result = profiler.track_goroutine_completion(
        1,
        cursed::profiling::concurrency::GoroutineCompletionType::Normal,
    );
    assert!(result.is_ok());
    
    // Get timeline analysis
    let timeline = profiler.get_goroutine_timeline();
    assert!(!timeline.is_empty());
    
    let channel_analysis = profiler.get_channel_analysis();
    assert_eq!(channel_analysis.total_operations, 1);
}

/// Test I/O profiling
#[test]
fn test_io_profiling() {
    // init_tracing!();
    common::tracing::setup();
    
    let profiler = cursed::profiling::io::IoProfiler::new();
    
    // Track file operation
    let result = profiler.track_file_operation(
        cursed::profiling::io::FileOperation::Read,
        "/test/file.txt".to_string(),
        Some(1024),
        Duration::from_millis(5),
    );
    assert!(result.is_ok());
    
    // Track network operation
    let result = profiler.track_network_operation(
        cursed::profiling::io::NetworkOperation::HttpRequest,
        "http://example.com".to_string(),
        Some(512),
        Duration::from_millis(50),
    );
    assert!(result.is_ok());
    
    // Get I/O analysis
    let analysis = profiler.get_io_analysis();
    assert_eq!(analysis.file_stats.total_operations, 1);
    assert_eq!(analysis.network_stats.total_operations, 1);
    
    // Detect bottlenecks
    let bottlenecks = profiler.detect_io_bottlenecks();
    // Should not detect bottlenecks for such fast operations
    assert!(bottlenecks.is_empty());
}

/// Test report generation
#[test]
fn test_report_generation() {
    // init_tracing!();
    common::tracing::setup();
    
    let config = ReportConfig::default();
    let generator = ReportGenerator::new(config);
    
    // Create dummy profile data
    let mut profile_data = cursed::profiling::core::ProfileData::new("test_session".to_string());
    profile_data.session_duration = Duration::from_secs(10);
    
    let report = generator.generate_report(&profile_data);
    assert!(report.is_ok());
    
    let performance_report = report.unwrap();
    assert_eq!(performance_report.session_name, "test_session");
    assert!(performance_report.summary.overall_score >= 0.0);
    assert!(performance_report.summary.overall_score <= 100.0);
}

/// Test visualization generation
#[test]
fn test_visualization_generation() {
    // init_tracing!();
    common::tracing::setup();
    
    let config = VisualizationConfig::default();
    let generator = VisualizationGenerator::new(config);
    
    // Create sample CPU profile data
    let cpu_data = cursed::profiling::cpu::CpuProfileData::new();
    
    // Generate flame graph
    let result = generator.generate_flame_graph(&cpu_data);
    assert!(result.is_ok());
    
    let svg = result.unwrap();
    assert!(svg.contains("<svg"));
    assert!(svg.contains("</svg>"));
    
    // Generate call graph
    let result = generator.generate_call_graph(&cpu_data);
    assert!(result.is_ok());
    
    let dot = result.unwrap();
    assert!(dot.contains("digraph CallGraph"));
}

/// Test performance analysis
#[test]
fn test_performance_analysis() {
    // init_tracing!();
    common::tracing::setup();
    
    let config = AnalysisConfig::default();
    let analyzer = PerformanceAnalyzer::new(config);
    
    // Create dummy profile data
    let profile_data = cursed::profiling::core::ProfileData::new("test_analysis".to_string());
    
    let result = analyzer.analyze_performance(&profile_data);
    assert!(result.is_ok());
    
    let insights = result.unwrap();
    assert_eq!(insights.session_name, "test_analysis");
    assert!(insights.performance_score >= 0.0);
    assert!(insights.performance_score <= 100.0);
}

/// Test build integration
#[test]
fn test_build_integration() {
    // init_tracing!();
    common::tracing::setup();
    
    let config = BuildConfig::default();
    let mut integration = BuildIntegration::new(config);
    
    // Setup profiling build
    let result = integration.setup_profiling_build();
    assert!(result.is_ok());
    
    // Run performance tests (simplified)
    let result = integration.run_performance_tests();
    assert!(result.is_ok());
    
    let test_results = result.unwrap();
    assert!(test_results.overall_pass); // Should pass with no tests
}

/// Test CLI configuration
#[test]
fn test_cli_configuration() {
    // init_tracing!();
    common::tracing::setup();
    
    let config = cursed::profiling::cli::CliConfig::default();
    assert_eq!(config.default_cpu_frequency, 100);
    assert_eq!(config.default_memory_threshold, 1024);
    assert!(config.default_modes.contains(&cursed::profiling::core::ProfilerMode::Cpu));
    assert!(config.default_modes.contains(&cursed::profiling::core::ProfilerMode::Memory));
}

/// Test profiler configuration validation
#[test]
fn test_profiler_configuration_validation() {
    // init_tracing!();
    common::tracing::setup();
    
    // Test valid configuration
    let config = ProfilerConfig {
        modes: vec![ProfilerMode::Cpu, ProfilerMode::Memory],
        cpu_sampling_frequency: 100,
        memory_tracking_threshold: 1024,
        max_stack_depth: 64,
        track_goroutines: true,
        track_io_operations: true,
        output_directory: "test_output".to_string(),
        max_session_duration: Duration::from_secs(300),
        output_format: OutputFormat::Json,
        regression_threshold: 10.0,
    };
    
    let profiler = cursed::profiling::core::CursedProfiler::new(config);
    assert!(!profiler.is_active());
    
    let stats = profiler.get_stats();
    assert_eq!(stats.sessions_started, 0);
    assert_eq!(stats.sessions_completed, 0);
}

/// Test error handling
#[test]
fn test_error_handling() {
    // init_tracing!();
    common::tracing::setup();
    
    let mut profiler = ProfilerBuilder::new().build();
    
    // Try to stop session without starting
    let result = profiler.stop_session();
    assert!(result.is_err());
    
    match result {
        Err(cursed::profiling::core::ProfilerError::NoActiveSession) => {
            // Expected error
        }
        _ => panic!("Expected NoActiveSession error"),
    }
    
    // Start session
    let result = profiler.start_session("test".to_string());
    assert!(result.is_ok());
    
    // Try to start another session
    let result = profiler.start_session("test2".to_string());
    assert!(result.is_err());
    
    match result {
        Err(cursed::profiling::core::ProfilerError::SessionAlreadyActive) => {
            // Expected error
        }
        _ => panic!("Expected SessionAlreadyActive error"),
    }
}

/// Test regression detection
#[test]
fn test_regression_detection() {
    // init_tracing!();
    common::tracing::setup();
    
    use cursed::profiling::benchmarking::{
        BenchmarkResult, BenchmarkStatistics, RegressionAnalysis,
        PerformanceChange, BenchmarkSuite,
    };
    
    // Create baseline and current results
    let baseline = BenchmarkResult {
        name: "test_benchmark".to_string(),
        measurements: vec![],
        statistics: BenchmarkStatistics {
            mean: Duration::from_millis(100),
            ..Default::default()
        },
        timestamp: std::time::SystemTime::now(),
    };
    
    let current = BenchmarkResult {
        name: "test_benchmark".to_string(),
        measurements: vec![],
        statistics: BenchmarkStatistics {
            mean: Duration::from_millis(130), // 30% slower
            ..Default::default()
        },
        timestamp: std::time::SystemTime::now(),
    };
    
    // Calculate performance change
    let change = BenchmarkSuite::calculate_performance_change(&baseline, &current);
    
    match change {
        PerformanceChange::Regression { percentage, .. } => {
            assert!((percentage - 30.0).abs() < 0.1);
        }
        _ => panic!("Expected regression"),
    }
}

/// Test concurrent profiling
#[test]
fn test_concurrent_profiling() {
    // init_tracing!();
    common::tracing::setup();
    
    let profiler = std::sync::Arc::new(std::sync::Mutex::new(
        cursed::profiling::concurrency::ConcurrencyProfiler::new()
    ));
    
    let handles: Vec<_> = (0..5)
        .map(|i| {
            let profiler_clone = profiler.clone();
            std::thread::spawn(move || {
                let profiler = profiler_clone.lock().unwrap();
                
                // Track goroutine operations
                let _ = profiler.track_goroutine_spawn(
                    i as u64,
                    None,
                    vec![format!("thread_{}", i)],
                );
                
                std::thread::sleep(Duration::from_millis(10));
                
                let _ = profiler.track_goroutine_completion(
                    i as u64,
                    cursed::profiling::concurrency::GoroutineCompletionType::Normal,
                );
            })
        })
        .collect();
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    let profiler = profiler.lock().unwrap();
    let timeline = profiler.get_goroutine_timeline();
    assert_eq!(timeline.len(), 5);
}

/// Test memory leak detection
#[test]
fn test_memory_leak_detection() {
    // init_tracing!();
    common::tracing::setup();
    
    let mut data = cursed::profiling::memory::MemoryProfileData::new();
    
    // Add allocation without deallocation (potential leak)
    let allocation = cursed::profiling::memory::AllocationEvent {
        event_type: cursed::profiling::memory::AllocationEventType::Allocate,
        size: 1024 * 1024, // 1MB
        address: 0x1000,
        stack_trace: vec!["leaked_function".to_string()],
        timestamp: std::time::Instant::now() - Duration::from_secs(30), // Old allocation
        thread_id: 1,
    };
    
    data.add_allocation_event(allocation);
    
    let leaks = data.detect_leaks();
    assert!(!leaks.is_empty());
    
    let leak = &leaks[0];
    assert_eq!(leak.size, 1024 * 1024);
    assert_eq!(leak.address, 0x1000);
    assert!(leak.age > Duration::from_secs(20));
}

/// Test performance optimization recommendations
#[test]
fn test_optimization_recommendations() {
    // init_tracing!();
    common::tracing::setup();
    
    let config = AnalysisConfig::default();
    let analyzer = PerformanceAnalyzer::new(config);
    
    // Create insights with low efficiency (should trigger recommendations)
    let mut insights = cursed::profiling::analysis::PerformanceInsights::new("test".to_string());
    insights.cpu_insights = Some(cursed::profiling::analysis::CpuInsights {
        hot_functions: vec![],
        cpu_efficiency: 0.5, // Low efficiency
        function_complexity: cursed::profiling::analysis::ComplexityAnalysis::default(),
        call_patterns: cursed::profiling::analysis::CallPatternAnalysis {
            recursive_functions: vec![],
            deep_call_chains: vec![],
            frequent_calls: vec![],
            call_graph_density: 0.5,
        },
        optimization_potential: 0.8,
        recommendations: vec!["Optimize hot functions".to_string()],
    });
    
    let opportunities = analyzer.identify_optimization_opportunities(&insights);
    assert!(!opportunities.is_empty());
    
    let cpu_opportunity = opportunities.iter()
        .find(|op| matches!(op.category, cursed::profiling::analysis::OptimizationCategory::Cpu));
    assert!(cpu_opportunity.is_some());
}

/// Integration test with multiple profiling modes
#[test]
fn test_multi_mode_profiling() {
    // init_tracing!();
    common::tracing::setup();
    
    let _timer = common::timing::Timer::new("multi_mode_profiling");
    
    let mut profiler = ProfilerBuilder::new()
        .with_modes(vec![
            ProfilerMode::Cpu,
            ProfilerMode::Memory,
            ProfilerMode::Concurrency,
        ])
        .build();
    
    // Start comprehensive profiling
    assert!(profiler.start_session("multi_mode_test".to_string()).is_ok());
    
    // Simulate complex workload
    for i in 0..10 {
        // CPU work
        let mut sum = 0;
        for j in 0..1000 {
            sum += i * j;
        }
        
        // Memory allocations (simulated)
        let _data = vec![0u8; 1024];
        
        // Brief pause
        std::thread::sleep(Duration::from_millis(1));
    }
    
    // Stop profiling
    let profile_data = profiler.stop_session();
    assert!(profile_data.is_ok());
    
    let data = profile_data.unwrap();
    assert!(data.session_duration > Duration::from_millis(10));
    
    // Verify data contains all requested modes
    assert!(data.mode_data.len() >= 1); // At least some data should be collected
}
