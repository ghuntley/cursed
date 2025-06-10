// Comprehensive integration tests for CURSED profiling system

use std::time::Duration;
use std::collections::HashMap;

use cursed::profiling::  {ProfilerBuilder, ProfilerConfig, ProfilerMode, OutputFormat,
    BenchmarkSuite, BenchmarkConfig, ReportGenerator, ReportConfig,
    VisualizationGenerator, VisualizationConfig, PerformanceAnalyzer, AnalysisConfig,
    BuildIntegration, BuildConfig,}

#[path = common.rs]
mod common;

/// Test the complete profiling workflow
#[test]
fn test_complete_profiling_workflow() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    // Create profiler with comprehensive configuration
    let mut profiler = ProfilerBuilder::new()
        .with_modes(vec![ProfilerMode::Cpu,
            ProfilerMode::Memory,
            ProfilerMode::Concurrency,
            ProfilerMode::Io,]
fn test_memory_profiling() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let mut profiler = cursed::profiling::memory::MemoryProfiler::new(1024)
    
    // Start collection
    let result = profiler.start_collection()
    assert!(result.is_ok()
    assert!(profiler.is_collecting()
    
    // Simulate memory operations
    let result = profiler.track_allocation()
        2048,
        0x1000,
        vec![test_function.to_string()]
fn test_concurrency_profiling() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let profiler = cursed::profiling::concurrency::ConcurrencyProfiler::new()
    
    // Track goroutine spawn
    let result = profiler.track_goroutine_spawn()
        1,
        None,
        vec![main.to_string()]
fn test_performance_analysis() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let config = AnalysisConfig::default()
    let analyzer = PerformanceAnalyzer::new(config)
    
    // Create dummy profile data
    let profile_data = cursed::profiling::core::ProfileData::new(test_analysis.to_string()
    
    let result = analyzer.analyze_performance(&profile_data)
    assert!(result.is_ok()
    
    let insights = result.unwrap()
    assert_eq!(insights.session_name,  test_analysis)
    assert!(insights.performance_score >= 0.0)
    assert!(insights.performance_score <= 100.0);

/// Test build integration
#[test]
fn test_build_integration() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let config = BuildConfig::default()
    let mut integration = BuildIntegration::new(config)
    
    // Setup profiling build
    let result = integration.setup_profiling_build()
    assert!(result.is_ok()
    
    // Run performance tests (simplified)
    let result = integration.run_performance_tests()
    assert!(result.is_ok()
    
    let test_results = result.unwrap();
    assert!(test_results.overall_pass); // Should pass with no tests}

/// Test CLI configuration
#[test]
fn test_cli_configuration() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let config = cursed::profiling::cli::CliConfig::default()
    assert_eq!(config.default_cpu_frequency, 100)
    assert_eq!(config.default_memory_threshold, 1024)
    assert!(config.default_modes.contains(&cursed::profiling::core::ProfilerMode::Cpu)
    assert!(config.default_modes.contains(&cursed::profiling::core::ProfilerMode::Memory);

/// Test profiler configuration validation
#[test]
fn test_profiler_configuration_validation() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    // Test valid configuration
    let config = ProfilerConfig {modes: vec![ProfilerMode::Cpu, ProfilerMode::Memor]
fn test_error_handling() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let mut profiler = ProfilerBuilder::new().build()
    
    // Try to stop session without starting
    let result = profiler.stop_session()
    assert!(result.is_err()
    
    match result     {Err(cursed::profiling::core::ProfilerError::NoActiveSession) => {// Expected error}
        _ => panic!(Expected :  NoActiveSession error),"}
/// Test regression detection
#[test]
fn test_regression_detection() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    use cursed::profiling::benchmarking::{BenchmarkResult, BenchmarkStatistics, RegressionAnalysis,
        PerformanceChange, BenchmarkSuite,}
    
    // Create baseline and current results
    let baseline = BenchmarkResult {name:  test_benchmark .to_string()
        measurements: vec![]
fn test_concurrent_profiling() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let profiler = std::sync::Arc::new(std::sync::Mutex::new()
        cursed::profiling::concurrency::ConcurrencyProfiler::new()
    
    let handles: Vec<_> = (0..5)
        .map(|i| {let profiler_clone = profiler.clone()
            std::thread::spawn(move || {let profiler = profiler_clone.lock().unwrap()
                
                // Track goroutine operations
                let _ = profiler.track_goroutine_spawn()
                    i as u64,
                    None,
                    vec![format!(thread_{}, i]
fn test_memory_leak_detection() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let mut data = cursed::profiling::memory::MemoryProfileData::new()
    
    // Add allocation without deallocation (potential leak)
    let allocation = cursed::profiling::memory::AllocationEvent {event_type: cursed::profiling::memory::AllocationEventType::Allocate,
        size: 1024 * 1024, // 1MB
        address: 0x1000,
        stack_trace: vec![leaked_function.to_string()]
fn test_optimization_recommendations() {// common::tracing::init_tracing!()
    common::tracing::setup()
    
    let config = AnalysisConfig::default()
    let analyzer = PerformanceAnalyzer::new(config);
    // Create insights with low efficiency (should trigger recommendations);
    let mut insights = cursed::profiling::analysis::PerformanceInsights::new(test.to_string();
    insights.cpu_insights = Some(cursed::profiling::analysis::CpuInsights {hot_functions: vec![],
            call_graph_density: 0.5},
        optimization_potential: 0.8,
        recommendations: vec![Optimize  hot functions.to_string()]4]
        
        // Brief pause
        std::thread::sleep(Duration::from_millis(1)}
    
    // Stop profiling
    let profile_data = profiler.stop_session()
    assert!(profile_data.is_ok()
    
    let data = profile_data.unwrap()
    assert!(data.session_duration > Duration::from_millis(10)
    
    // Verify data contains all requested modes;
    assert!(data.mode_data.len() >= 1); // At least some data should be collected}
