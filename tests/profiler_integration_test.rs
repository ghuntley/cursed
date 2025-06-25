/// Comprehensive integration tests for the CURSED profiler system
use std::time::Duration;
use cursed::stdlib::profiler::*;

#[test]
fn test_profiler_initialization() {
    // Test basic profiler initialization
    let result = initialize();
    assert!(result.is_ok(), "Profiler initialization should succeed");

    // Test shutdown
    let shutdown_result = shutdown();
    assert!(shutdown_result.is_ok(), "Profiler shutdown should succeed");
}

#[test]
fn test_quick_performance_check() {
    let stats = quick_performance_check();
    assert!(stats.is_ok(), "Quick performance check should succeed");

    let stats = stats.unwrap();
    assert!(stats.total_time_ns > 0, "Total time should be positive");
    assert!(stats.cpu_performance_ns > 0, "CPU performance time should be positive");
    assert!(stats.memory_performance_ns > 0, "Memory performance time should be positive");
    assert!(stats.dummy_computation_result != 0, "Computation should produce a result");
}

#[test]
fn test_profiling_overhead() {
    let overhead = get_profiling_overhead();
    assert!(overhead > 0, "Profiling overhead should be measurable");
    assert!(overhead < 10_000_000, "Profiling overhead should be reasonable (< 10ms)");
}

#[test]
fn test_cpu_profiler_config() {
    let config = ProfilerConfig::default();
    assert_eq!(config.sampling_frequency_hz, 100);
    assert_eq!(config.max_stack_depth, 32);
    assert_eq!(config.buffer_size, 10000);
    assert!(config.enable_call_graph);
    assert!(!config.track_allocations);
}

#[test]
fn test_cpu_profiler_creation_and_lifecycle() {
    let config = ProfilerConfig::default();
    let mut profiler = CpuProfiler::new(config);
    
    assert!(!profiler.is_running());
    
    // Start profiling
    let start_result = profiler.start();
    assert!(start_result.is_ok(), "CPU profiler should start successfully");
    assert!(profiler.is_running());
    
    // Record some function entries
    profiler.record_function_entry("test_function").unwrap();
    profiler.record_function_entry("another_function").unwrap();
    
    // Let it run for a short time
    std::thread::sleep(Duration::from_millis(100));
    
    // Stop profiling
    let profile_result = profiler.stop();
    assert!(profile_result.is_ok(), "CPU profiler should stop successfully");
    assert!(!profiler.is_running());
    
    let profile = profile_result.unwrap();
    assert!(profile.data.sample_count > 0, "Should have collected samples");
    assert!(profile.start_time <= profile.end_time.unwrap());
}

#[test]
fn test_global_cpu_profiling() {
    // Test global CPU profiling functions
    let start_result = start_cpu_profiling();
    if start_result.is_err() {
        // Might already be running, which is fine for this test
        assert!(matches!(start_result.unwrap_err(), ProfilerError::AlreadyRunning));
    }
    
    // Check if we can get current profile
    let current_profile = get_cpu_profile();
    assert!(current_profile.is_ok());
    
    // Stop profiling
    let stop_result = stop_cpu_profiling();
    assert!(stop_result.is_ok(), "Should be able to stop CPU profiling");
}

#[test]
fn test_memory_tracker() {
    let tracker = MemoryTracker::new();
    
    // Track an allocation
    let allocation_id = tracker.track_allocation(1024, "test_function", "test.rs", 42);
    assert!(allocation_id.is_ok());
    let allocation_id = allocation_id.unwrap();
    assert!(allocation_id > 0);
    
    // Check statistics
    let stats = tracker.get_stats().unwrap();
    assert_eq!(stats.allocation_count, 1);
    assert_eq!(stats.current_usage, 1024);
    assert_eq!(stats.live_objects, 1);
    
    // Track deallocation
    let dealloc_result = tracker.track_deallocation(allocation_id);
    assert!(dealloc_result.is_ok());
    
    // Check updated statistics
    let stats = tracker.get_stats().unwrap();
    assert_eq!(stats.deallocation_count, 1);
    assert_eq!(stats.current_usage, 0);
    assert_eq!(stats.live_objects, 0);
}

#[test]
fn test_memory_profiler_lifecycle() {
    let mut profiler = MemoryProfiler::new();
    
    // Start profiling
    let start_result = profiler.start();
    assert!(start_result.is_ok());
    assert!(profiler.is_running());
    
    // Take a heap snapshot
    profiler.take_heap_snapshot().unwrap();
    
    // Record a GC event
    profiler.record_gc_event(
        "minor", 
        1000000, // 1ms
        512, 
        10, 
        2048, 
        1536, 
        "allocation_threshold"
    ).unwrap();
    
    // Stop profiling
    let profile_result = profiler.stop();
    assert!(profile_result.is_ok());
    assert!(!profiler.is_running());
    
    let profile = profile_result.unwrap();
    assert!(!profile.heap_profiles.is_empty());
    assert!(!profile.gc_profiles.is_empty());
}

#[test]
fn test_global_memory_profiling() {
    // Test global memory profiling functions
    let start_result = start_memory_profiling();
    if start_result.is_err() {
        // Might already be running
        assert!(matches!(start_result.unwrap_err(), ProfilerError::AlreadyRunning));
    }
    
    // Track some allocations
    let _ = track_allocation(512, "test", "test.rs", 10);
    let _ = track_allocation(1024, "test2", "test.rs", 20);
    
    // Get memory stats
    let stats = get_memory_stats();
    assert!(stats.is_ok());
    
    // Stop profiling
    let stop_result = stop_memory_profiling();
    assert!(stop_result.is_ok());
}

#[test]
fn test_benchmark_statistics() {
    let measurements = vec![
        Duration::from_nanos(100),
        Duration::from_nanos(200),
        Duration::from_nanos(150),
        Duration::from_nanos(180),
        Duration::from_nanos(120),
    ];
    
    let stats = BenchmarkStatistics::from_measurements(measurements);
    assert_eq!(stats.min, Duration::from_nanos(100));
    assert_eq!(stats.max, Duration::from_nanos(200));
    assert_eq!(stats.median, Duration::from_nanos(150));
    assert!(stats.mean > Duration::new(0, 0));
    assert!(stats.std_dev > Duration::new(0, 0));
    assert!(stats.coefficient_of_variation > 0.0);
}

#[test]
fn test_benchmark_function() {
    let result = benchmark_function("simple_test", || {
        // Simulate some work
        let mut sum = 0;
        for i in 0..1000 {
            sum += i;
        }
        Ok(())
    });
    
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.name, "simple_test");
    assert!(result.iterations_completed > 0);
    assert!(result.statistics.mean > Duration::new(0, 0));
    assert!(result.throughput() > 0.0);
}

#[test]
fn test_benchmark_with_setup() {
    let result = benchmark_with_setup(
        "test_with_setup",
        || Ok(()), // setup
        || {
            std::thread::sleep(Duration::from_nanos(100));
            Ok(())
        }, // benchmark
        || Ok(()), // teardown
    );
    
    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(result.statistics.mean >= Duration::from_nanos(100));
}

#[test]
fn test_benchmark_suite() {
    let config = BenchmarkConfig {
        iterations: 10,
        warmup_iterations: 2,
        min_duration: Duration::from_millis(10),
        max_duration: Duration::from_secs(1),
        ..BenchmarkConfig::default()
    };
    
    let mut suite = BenchmarkSuite::new("test_suite", config);
    
    suite.add_benchmark("fast_test", || {
        std::thread::sleep(Duration::from_nanos(50));
        Ok(())
    });
    
    suite.add_benchmark("slow_test", || {
        std::thread::sleep(Duration::from_nanos(100));
        Ok(())
    });
    
    let result = suite.run();
    assert!(result.is_ok());
    
    let result = result.unwrap();
    assert_eq!(result.benchmark_count, 2);
    assert_eq!(result.results.len(), 2);
    
    // Test suite analysis methods
    let fastest = result.fastest();
    assert!(fastest.is_some());
    
    let slowest = result.slowest();
    assert!(slowest.is_some());
    
    let avg_throughput = result.average_throughput();
    assert!(avg_throughput > 0.0);
}

#[test]
fn test_benchmark_comparison() {
    let fast_result = BenchmarkResult {
        name: "fast".to_string(),
        config: BenchmarkConfig::default(),
        statistics: BenchmarkStatistics {
            mean: Duration::from_nanos(100),
            coefficient_of_variation: 0.1,
            ..BenchmarkStatistics::default()
        },
        iterations_completed: 100,
        total_time: Duration::from_secs(1),
        warmup_time: Duration::from_millis(10),
        memory_usage: None,
        cpu_usage: None,
        metadata: std::collections::HashMap::new(),
        raw_measurements: vec![],
    };
    
    let slow_result = BenchmarkResult {
        name: "slow".to_string(),
        config: BenchmarkConfig::default(),
        statistics: BenchmarkStatistics {
            mean: Duration::from_nanos(200),
            coefficient_of_variation: 0.1,
            ..BenchmarkStatistics::default()
        },
        iterations_completed: 100,
        total_time: Duration::from_secs(2),
        warmup_time: Duration::from_millis(10),
        memory_usage: None,
        cpu_usage: None,
        metadata: std::collections::HashMap::new(),
        raw_measurements: vec![],
    };
    
    let comparison = compare_benchmarks(&slow_result, &fast_result);
    assert!(comparison.is_faster);
    assert_eq!(comparison.speedup_factor, 2.0);
    assert_eq!(comparison.baseline_name, "slow");
    assert_eq!(comparison.comparison_name, "fast");
}

#[test]
fn test_counter_metric() {
    let counter = CounterMetric::new("test_counter");
    
    assert_eq!(counter.get(), 0);
    
    counter.increment();
    assert_eq!(counter.get(), 1);
    
    counter.add(5);
    assert_eq!(counter.get(), 6);
    
    counter.reset();
    assert_eq!(counter.get(), 0);
    
    let metric = counter.to_metric();
    assert_eq!(metric.name, "test_counter");
    assert_eq!(metric.metric_type, MetricType::Counter);
}

#[test]
fn test_gauge_metric() {
    let gauge = GaugeMetric::new("test_gauge");
    
    gauge.set(25);
    assert_eq!(gauge.get(), 25);
    
    gauge.increment();
    assert_eq!(gauge.get(), 26);
    
    gauge.decrement();
    assert_eq!(gauge.get(), 25);
    
    gauge.add(-5);
    assert_eq!(gauge.get(), 20);
    
    let metric = gauge.to_metric();
    assert_eq!(metric.name, "test_gauge");
    assert_eq!(metric.metric_type, MetricType::Gauge);
}

#[test]
fn test_histogram_metric() {
    let histogram = HistogramMetric::new("test_histogram");
    
    histogram.observe(0.1);
    histogram.observe(0.2);
    histogram.observe(0.15);
    
    let summary = histogram.get_summary();
    assert_eq!(summary.count, 3);
    assert!((summary.mean - 0.15).abs() < 0.001);
    assert_eq!(summary.min, 0.1);
    assert_eq!(summary.max, 0.2);
    
    let metric = histogram.to_metric();
    assert_eq!(metric.metric_type, MetricType::Histogram);
}

#[test]
fn test_timer_metric() {
    let timer = TimerMetric::new("test_timer");
    
    let result = timer.time(|| {
        std::thread::sleep(Duration::from_millis(1));
        42
    });
    
    assert_eq!(result, 42);
    
    let summary = timer.get_summary();
    assert!(summary.count > 0);
    assert!(summary.mean > 0.0);
    
    let metric = timer.to_metric();
    assert_eq!(metric.metric_type, MetricType::Timer);
}

#[test]
fn test_metrics_collection() {
    let metrics = collect_metrics();
    assert!(metrics.is_ok());
    
    let metrics = metrics.unwrap();
    assert!(metrics.total_metrics > 0);
    assert!(metrics.collection_duration > Duration::new(0, 0));
    
    // Test metric access
    let cpu_metric = metrics.get_metric("cpu_usage_percent");
    assert!(cpu_metric.is_some());
    
    let gauge_metrics = metrics.get_metrics_by_type(MetricType::Gauge);
    assert!(!gauge_metrics.is_empty());
}

#[test]
fn test_metrics_export() {
    let mut metrics = PerformanceMetrics::new();
    
    let metric = Metric::new("test_metric", MetricType::Counter, MetricValue::Integer(42))
        .with_description("Test metric for export")
        .with_label("service", "test");
    
    metrics.add_metric(metric);
    
    // Test Prometheus export
    let prometheus_output = metrics.export_prometheus();
    assert!(prometheus_output.contains("# HELP test_metric Test metric for export"));
    assert!(prometheus_output.contains("# TYPE test_metric counter"));
    assert!(prometheus_output.contains("test_metric{service=\"test\"} 42"));
    
    // Test JSON export
    let json_output = metrics.export_json();
    assert!(json_output.is_ok());
    let json_output = json_output.unwrap();
    assert!(json_output.contains("test_metric"));
    assert!(json_output.contains("42"));
}

#[test]
fn test_global_metrics_functions() {
    // Test export functions
    let prometheus_export = export_metrics("prometheus");
    assert!(prometheus_export.is_ok());
    
    let json_export = export_metrics("json");
    assert!(json_export.is_ok());
    
    // Test unsupported format
    let invalid_export = export_metrics("xml");
    assert!(invalid_export.is_err());
    
    // Test metrics count
    let count = get_metrics_count();
    assert!(count >= 0);
}

#[test]
fn test_metrics_collector_lifecycle() {
    let mut collector = MetricsCollector::new();
    
    // Start collection
    let start_result = collector.start();
    assert!(start_result.is_ok());
    assert!(collector.is_collecting());
    
    // Record some metrics
    let metric1 = Metric::new("test1", MetricType::Counter, MetricValue::Integer(1));
    let metric2 = Metric::new("test2", MetricType::Gauge, MetricValue::Float(2.5));
    
    collector.record_metric(metric1).unwrap();
    collector.record_metric(metric2).unwrap();
    
    // Get current snapshot
    let current = collector.get_current_metrics();
    assert!(current.is_ok());
    let current = current.unwrap();
    assert_eq!(current.total_metrics, 2);
    
    // Stop collection
    let final_metrics = collector.stop();
    assert!(final_metrics.is_ok());
    assert!(!collector.is_collecting());
    
    let final_metrics = final_metrics.unwrap();
    assert_eq!(final_metrics.total_metrics, 2);
}

#[test]
fn test_runtime_integration_config() {
    let config = IntegrationConfig::default();
    assert!(config.enable_gc_integration);
    assert!(config.enable_goroutine_integration);
    assert!(config.enable_jit_integration);
    assert_eq!(config.sampling_frequency_hz, 100);
    assert!(config.memory_tracking);
    assert!(config.cpu_profiling);
    assert!(config.metrics_collection);
    assert!(!config.benchmarking); // Disabled by default
}

#[test]
fn test_runtime_profiler() {
    let config = IntegrationConfig::default();
    let mut profiler = RuntimeProfiler::new(config);
    
    assert!(!profiler.is_active());
    
    // Start profiling
    let start_result = profiler.start();
    assert!(start_result.is_ok());
    assert!(profiler.is_active());
    
    // Let it run briefly
    std::thread::sleep(Duration::from_millis(50));
    
    // Stop profiling
    let result = profiler.stop();
    assert!(result.is_ok());
    assert!(!profiler.is_active());
    
    let result = result.unwrap();
    assert!(result.duration > Duration::new(0, 0));
    assert!(result.total_samples >= 0);
}

#[test]
fn test_profiler_runtime() {
    let config = IntegrationConfig::default();
    let runtime = ProfilerRuntime::new(config);
    
    assert!(!runtime.is_initialized());
    
    // Initialize
    let init_result = runtime.initialize();
    assert!(init_result.is_ok());
    assert!(runtime.is_initialized());
    
    // Start profiling
    let start_result = runtime.start_profiling();
    assert!(start_result.is_ok());
    
    // Stop profiling
    let stop_result = runtime.stop_profiling();
    assert!(stop_result.is_ok());
    
    // Shutdown
    let shutdown_result = runtime.shutdown();
    assert!(shutdown_result.is_ok());
    assert!(!runtime.is_initialized());
}

#[test]
fn test_global_profiler_runtime() {
    // Test global profiler runtime functions
    let init_result = initialize_profiler();
    if init_result.is_err() {
        // Might already be initialized
        assert!(matches!(init_result.unwrap_err(), ProfilerError::AlreadyRunning));
    }
    
    // Get runtime
    let runtime_result = get_profiler_runtime();
    if runtime_result.is_ok() {
        let runtime = runtime_result.unwrap();
        assert!(runtime.is_initialized());
        
        // Test integrations
        let gc_integration = integrate_with_gc();
        assert!(gc_integration.is_ok() || matches!(gc_integration.unwrap_err(), ProfilerError::NotInitialized));
        
        let goroutine_integration = integrate_with_goroutines();
        assert!(goroutine_integration.is_ok() || matches!(goroutine_integration.unwrap_err(), ProfilerError::NotInitialized));
        
        let jit_integration = integrate_with_jit();
        assert!(jit_integration.is_ok() || matches!(jit_integration.unwrap_err(), ProfilerError::NotInitialized));
    }
    
    // Shutdown
    let shutdown_result = shutdown_profiler();
    assert!(shutdown_result.is_ok());
}

#[test]
fn test_error_handling() {
    // Test various error conditions
    
    // Invalid configuration
    let error = ProfilerError::InvalidConfig("test error".to_string());
    assert_eq!(error.to_string(), "Invalid profiler configuration: test error");
    
    // Not initialized
    let error = ProfilerError::NotInitialized;
    assert_eq!(error.to_string(), "Profiler is not initialized");
    
    // Already running
    let error = ProfilerError::AlreadyRunning;
    assert_eq!(error.to_string(), "Profiler is already running");
    
    // Helper functions
    let custom_error = crate::stdlib::profiler::error::profiler_error("custom message");
    match custom_error {
        ProfilerError::General(msg) => assert_eq!(msg, "custom message"),
        _ => panic!("Expected General error"),
    }
}

#[test]
fn test_profiler_statistics() {
    // Test global profiler statistics
    let stats = get_statistics();
    assert!(stats.is_ok());
    
    let stats = stats.unwrap();
    assert!(stats.cpu_profiles_created >= 0);
    assert!(stats.memory_profiles_created >= 0);
    assert!(stats.benchmarks_run >= 0);
    assert!(stats.metrics_collected >= 0);
    assert!(stats.total_samples >= 0);
    assert!(stats.profiling_overhead_ns > 0);
}

#[test]
fn test_performance_metrics_analysis() {
    let mut metrics = PerformanceMetrics::new();
    
    // Add various metrics
    metrics.add_metric(Metric::new("counter1", MetricType::Counter, MetricValue::Integer(100)));
    metrics.add_metric(Metric::new("gauge1", MetricType::Gauge, MetricValue::Float(75.5)));
    metrics.add_metric(Metric::new("histogram1", MetricType::Histogram, MetricValue::Float(0.25)));
    
    assert_eq!(metrics.total_metrics, 3);
    
    // Test type filtering
    let counters = metrics.get_metrics_by_type(MetricType::Counter);
    assert_eq!(counters.len(), 1);
    
    let gauges = metrics.get_metrics_by_type(MetricType::Gauge);
    assert_eq!(gauges.len(), 1);
    
    let histograms = metrics.get_metrics_by_type(MetricType::Histogram);
    assert_eq!(histograms.len(), 1);
}

// Helper function for tests that need Default implementation
impl Default for BenchmarkResult {
    fn default() -> Self {
        Self {
            name: String::new(),
            config: BenchmarkConfig::default(),
            statistics: BenchmarkStatistics::default(),
            iterations_completed: 0,
            total_time: Duration::new(0, 0),
            warmup_time: Duration::new(0, 0),
            memory_usage: None,
            cpu_usage: None,
            metadata: std::collections::HashMap::new(),
            raw_measurements: vec![],
        }
    }
}
