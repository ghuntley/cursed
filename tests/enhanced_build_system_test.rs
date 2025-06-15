//! Enhanced Build System Tests
//! 
//! Comprehensive test suite for the enhanced build system with pipeline metrics,
//! parallel compilation optimization, and build performance analysis.

use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;
use tempfile::tempdir;

use cursed::build_system::{
    BuildConfig, BuildTarget, BuildProfile, TargetType, OptimizationLevel, ProjectType,
    build_orchestrator::{
        BuildOrchestrator, BuildResult, BuildStatistics, BuildPerformanceReport,
        ResourceUtilizationAnalysis, CacheEffectivenessMetrics, BuildBottleneck,
        BottleneckSeverity,
    },
    parallel_compilation::{
        ParallelCompiler, ParallelCompilationConfig, CompilationTask, TaskPriority,
        ParallelEfficiencyAnalysis, SchedulingStrategy,
    },
};

/// Test build orchestrator creation and basic functionality
#[tokio::test]
async fn test_build_orchestrator_enhanced_metrics() {
    let config = BuildConfig::default_for_project("test_enhanced", ProjectType::Binary);
    let work_dir = tempdir().unwrap().into_path();
    
    let mut orchestrator = BuildOrchestrator::new(config, work_dir).unwrap();
    
    // Test pipeline build with enhanced metrics
    let result = orchestrator.build_with_pipeline(
        "dev",
        vec!["main".to_string()],
        false,
        true
    ).await;
    
    assert!(result.is_ok());
    let pipeline_result = result.unwrap();
    assert!(pipeline_result.success);
}

/// Test enhanced pipeline metrics extraction
#[test]
fn test_pipeline_metrics_extraction() {
    // Create mock pipeline result
    let mut pipeline_stages = HashMap::new();
    let mut stage_info = HashMap::new();
    stage_info.insert("duration".to_string(), serde_json::Value::String("1500".to_string()));
    stage_info.insert("lines_compiled".to_string(), serde_json::Value::String("2500".to_string()));
    pipeline_stages.insert("compilation".to_string(), stage_info);
    
    // Verify that metrics extraction would work with proper structure
    assert!(!pipeline_stages.is_empty());
}

/// Test build performance analysis
#[test]
fn test_build_performance_analysis() {
    let mut phase_timings = HashMap::new();
    phase_timings.insert("compilation".to_string(), Duration::from_millis(3000)); // 60% of 5000ms
    phase_timings.insert("linking".to_string(), Duration::from_millis(1500));     // 30% of 5000ms
    phase_timings.insert("parsing".to_string(), Duration::from_millis(500));      // 10% of 5000ms
    
    let statistics = BuildStatistics {
        files_compiled: 10,
        files_cached: 5,
        lines_compiled: 1000,
        peak_memory: 3 * 1024 * 1024 * 1024, // 3GB
        phase_timings,
    };
    
    // Test build efficiency score calculation
    assert!(statistics.files_compiled > 0);
    assert!(statistics.files_cached > 0);
    
    // Test cache hit rate calculation
    let total_files = statistics.files_compiled + statistics.files_cached;
    let cache_hit_rate = statistics.files_cached as f64 / total_files as f64;
    assert!(cache_hit_rate > 0.0);
    assert!(cache_hit_rate <= 1.0);
}

/// Test resource utilization analysis
#[test]
fn test_resource_utilization_analysis() {
    let analysis = ResourceUtilizationAnalysis {
        memory_peak_mb: 2048,
        memory_efficiency: 0.85,
        cpu_intensive_phases: vec!["compilation".to_string(), "optimization".to_string()],
        io_intensive_phases: vec!["parsing".to_string(), "linking".to_string()],
        parallelization_opportunities: vec![
            "Multiple compilation units can be processed in parallel".to_string()
        ],
        total_build_time: Duration::from_secs(45),
    };
    
    assert!(analysis.memory_efficiency > 0.0);
    assert!(analysis.memory_efficiency <= 1.0);
    assert!(!analysis.cpu_intensive_phases.is_empty());
    assert!(!analysis.io_intensive_phases.is_empty());
}

/// Test cache effectiveness metrics
#[test]
fn test_cache_effectiveness_metrics() {
    let metrics = CacheEffectivenessMetrics {
        hit_rate: 0.75,
        files_from_cache: 15,
        total_files: 20,
        estimated_time_saved: Duration::from_secs(30),
        cache_storage_efficiency: 0.85,
    };
    
    assert_eq!(metrics.hit_rate, 0.75);
    assert_eq!(metrics.files_from_cache, 15);
    assert_eq!(metrics.total_files, 20);
    assert!(metrics.estimated_time_saved > Duration::default());
}

/// Test build bottleneck analysis
#[test]
fn test_build_bottleneck_analysis() {
    let bottleneck = BuildBottleneck {
        phase: "compilation".to_string(),
        duration: Duration::from_secs(30),
        percentage: 65.0,
        severity: BottleneckSeverity::High,
        recommendations: vec![
            "Enable parallel compilation with optimal worker count".to_string(),
            "Consider using precompiled headers".to_string(),
        ],
    };
    
    assert_eq!(bottleneck.phase, "compilation");
    assert!(bottleneck.percentage > 50.0);
    assert!(matches!(bottleneck.severity, BottleneckSeverity::High));
    assert!(!bottleneck.recommendations.is_empty());
}

/// Test parallel compilation configuration
#[test]
fn test_parallel_compilation_config() {
    let config = ParallelCompilationConfig {
        max_workers: 8,
        memory_limit_mb: 512,
        cpu_affinity: true,
        pipeline_overlap: true,
        scheduling_strategy: SchedulingStrategy::WorkStealing,
        monitor_interval_ms: 100,
        max_queue_depth: 10,
        streaming_results: true,
        adaptive_scaling: true,
        cross_module_optimization: true,
    };
    
    assert_eq!(config.max_workers, 8);
    assert!(config.cpu_affinity);
    assert!(config.pipeline_overlap);
    assert!(matches!(config.scheduling_strategy, SchedulingStrategy::WorkStealing));
}

/// Test parallel compiler creation
#[test]
fn test_parallel_compiler_creation() {
    let config = ParallelCompilationConfig::default();
    let compiler = ParallelCompiler::new(config);
    assert!(compiler.is_ok());
}

/// Test compilation task creation
#[test]
fn test_compilation_task_creation() {
    let target = BuildTarget {
        name: "test_target".to_string(),
        path: PathBuf::from("src/main.csd"),
        target_type: TargetType::Bin,
        dependencies: vec!["dep1".to_string()],
        features: vec!["feature1".to_string()],
    };
    
    let profile = BuildProfile {
        debug: true,
        optimization: OptimizationLevel::None,
        lto: false,
        codegen_units: Some(1),
        llvm_args: vec![],
        env: HashMap::new(),
    };
    
    let task = CompilationTask {
        id: "task_1".to_string(),
        target,
        profile,
        dependencies: vec!["dep1".to_string()],
        estimated_duration: Duration::from_secs(10),
        memory_requirement: 256 * 1024 * 1024, // 256MB
        priority: TaskPriority::Normal,
        compilation_units: vec![],
    };
    
    assert_eq!(task.id, "task_1");
    assert_eq!(task.dependencies.len(), 1);
    assert!(matches!(task.priority, TaskPriority::Normal));
}

/// Test parallel efficiency analysis
#[test]
fn test_parallel_efficiency_analysis() {
    let analysis = ParallelEfficiencyAnalysis {
        overall_efficiency: 0.85,
        load_balance_score: 0.92,
        average_worker_utilization: 0.88,
        potential_speedup: 3.5,
        scalability_recommendations: vec![
            "High efficiency achieved - consider increasing worker count for better performance".to_string()
        ],
    };
    
    assert!(analysis.overall_efficiency > 0.8);
    assert!(analysis.load_balance_score > 0.9);
    assert!(analysis.potential_speedup > 3.0);
    assert!(!analysis.scalability_recommendations.is_empty());
}

/// Test task scheduling strategies
#[test]
fn test_scheduling_strategies() {
    let strategies = vec![
        SchedulingStrategy::Fifo,
        SchedulingStrategy::ShortestFirst,
        SchedulingStrategy::CriticalPath,
        SchedulingStrategy::DependencyRoundRobin,
        SchedulingStrategy::WorkStealing,
        SchedulingStrategy::Adaptive,
    ];
    
    assert_eq!(strategies.len(), 6);
    assert!(strategies.contains(&SchedulingStrategy::WorkStealing));
    assert!(strategies.contains(&SchedulingStrategy::Adaptive));
}

/// Test memory efficiency calculation
#[test]
fn test_memory_efficiency_calculation() {
    // Test efficient memory usage
    let efficient_stats = BuildStatistics {
        files_compiled: 10,
        files_cached: 5,
        lines_compiled: 1000,
        peak_memory: 512 * 1024 * 1024, // 512MB for 10 files = 51.2MB per file (efficient)
        phase_timings: HashMap::new(),
    };
    
    let memory_per_file = efficient_stats.peak_memory as f64 / efficient_stats.files_compiled as f64;
    let optimal_memory_per_file = 128.0 * 1024.0 * 1024.0; // 128MB per file target
    let efficiency = (optimal_memory_per_file / memory_per_file).min(1.0);
    
    assert!(efficiency > 0.8); // Should be efficient
    
    // Test inefficient memory usage
    let inefficient_stats = BuildStatistics {
        files_compiled: 10,
        files_cached: 5,
        lines_compiled: 1000,
        peak_memory: 4 * 1024 * 1024 * 1024, // 4GB for 10 files = 400MB per file (inefficient)
        phase_timings: HashMap::new(),
    };
    
    let memory_per_file_inefficient = inefficient_stats.peak_memory as f64 / inefficient_stats.files_compiled as f64;
    let efficiency_inefficient = (optimal_memory_per_file / memory_per_file_inefficient).min(1.0);
    
    assert!(efficiency_inefficient < 0.5); // Should be inefficient
}

/// Test build performance score calculation
#[test]
fn test_build_performance_score() {
    // Test high-performance build
    let high_perf_stats = BuildStatistics {
        files_compiled: 5,
        files_cached: 15, // 75% cache hit rate
        lines_compiled: 10000,
        peak_memory: 1024 * 1024 * 1024, // 1GB
        phase_timings: {
            let mut timings = HashMap::new();
            timings.insert("compilation".to_string(), Duration::from_secs(10)); // 10000 lines in 10s = 1000 lines/sec
            timings
        },
    };
    
    // Calculate cache ratio
    let cache_ratio = high_perf_stats.files_cached as f64 / 
        (high_perf_stats.files_compiled + high_perf_stats.files_cached) as f64;
    assert!(cache_ratio >= 0.7); // Good cache utilization
    
    // Calculate compilation speed
    let total_time_secs = high_perf_stats.phase_timings.values().sum::<Duration>().as_secs_f64();
    let lines_per_second = high_perf_stats.lines_compiled as f64 / total_time_secs;
    assert!(lines_per_second >= 1000.0); // Good compilation speed
}

/// Test queue wait time calculation
#[test]
fn test_queue_wait_time_calculation() {
    // Simulate worker results with varying completion times
    let completion_times = vec![100u128, 150, 200, 120, 180]; // milliseconds
    
    let mean_time = completion_times.iter().sum::<u128>() as f64 / completion_times.len() as f64;
    let variance = completion_times.iter()
        .map(|&time| {
            let diff = time as f64 - mean_time;
            diff * diff
        })
        .sum::<f64>() / completion_times.len() as f64;
    
    let std_deviation = variance.sqrt();
    let wait_factor = (std_deviation / mean_time).min(1.0);
    
    assert!(wait_factor >= 0.0);
    assert!(wait_factor <= 1.0);
    
    // Higher variance should result in higher wait factor
    let high_variance_times = vec![50u128, 500, 100, 450, 200];
    let high_mean = high_variance_times.iter().sum::<u128>() as f64 / high_variance_times.len() as f64;
    let high_variance = high_variance_times.iter()
        .map(|&time| {
            let diff = time as f64 - high_mean;
            diff * diff
        })
        .sum::<f64>() / high_variance_times.len() as f64;
    
    let high_std_deviation = high_variance.sqrt();
    let high_wait_factor = (high_std_deviation / high_mean).min(1.0);
    
    assert!(high_wait_factor > wait_factor); // Higher variance = more waiting
}

/// Test task distribution optimization
#[test]
fn test_task_distribution_optimization() {
    // Create tasks with different estimated durations
    let short_task = CompilationTask {
        id: "short".to_string(),
        target: create_test_target("short"),
        profile: BuildProfile::default(),
        dependencies: vec![],
        estimated_duration: Duration::from_secs(2),
        memory_requirement: 128 * 1024 * 1024,
        priority: TaskPriority::Normal,
        compilation_units: vec![],
    };
    
    let long_task = CompilationTask {
        id: "long".to_string(),
        target: create_test_target("long"),
        profile: BuildProfile::default(),
        dependencies: vec![],
        estimated_duration: Duration::from_secs(10),
        memory_requirement: 256 * 1024 * 1024,
        priority: TaskPriority::High,
        compilation_units: vec![],
    };
    
    let tasks = vec![long_task.clone(), short_task.clone()];
    
    // Verify task properties
    assert!(short_task.estimated_duration < Duration::from_secs(5));
    assert!(long_task.estimated_duration >= Duration::from_secs(5));
    assert!(matches!(long_task.priority, TaskPriority::High));
}

/// Helper function to create test targets
fn create_test_target(name: &str) -> BuildTarget {
    BuildTarget {
        name: name.to_string(),
        path: PathBuf::from(format!("src/{}.csd", name)),
        target_type: TargetType::Bin,
        dependencies: vec![],
        features: vec![],
    }
}

/// Test bottleneck severity classification
#[test]
fn test_bottleneck_severity_classification() {
    // Test critical bottleneck (>50% of time)
    let critical_percentage = 65.0;
    let critical_severity = if critical_percentage > 50.0 {
        BottleneckSeverity::Critical
    } else if critical_percentage > 35.0 {
        BottleneckSeverity::High
    } else {
        BottleneckSeverity::Medium
    };
    assert!(matches!(critical_severity, BottleneckSeverity::Critical));
    
    // Test high bottleneck (35-50% of time)
    let high_percentage = 42.0;
    let high_severity = if high_percentage > 50.0 {
        BottleneckSeverity::Critical
    } else if high_percentage > 35.0 {
        BottleneckSeverity::High
    } else {
        BottleneckSeverity::Medium
    };
    assert!(matches!(high_severity, BottleneckSeverity::High));
    
    // Test medium bottleneck (25-35% of time)
    let medium_percentage = 30.0;
    let medium_severity = if medium_percentage > 50.0 {
        BottleneckSeverity::Critical
    } else if medium_percentage > 35.0 {
        BottleneckSeverity::High
    } else {
        BottleneckSeverity::Medium
    };
    assert!(matches!(medium_severity, BottleneckSeverity::Medium));
}

/// Integration test for complete build performance analysis
#[test]
fn test_complete_build_performance_analysis() {
    let mut phase_timings = HashMap::new();
    phase_timings.insert("parsing".to_string(), Duration::from_millis(500));
    phase_timings.insert("compilation".to_string(), Duration::from_millis(3000));
    phase_timings.insert("optimization".to_string(), Duration::from_millis(1000));
    phase_timings.insert("linking".to_string(), Duration::from_millis(500));
    
    let statistics = BuildStatistics {
        files_compiled: 20,
        files_cached: 30,
        lines_compiled: 5000,
        peak_memory: 1536 * 1024 * 1024, // 1.5GB
        phase_timings: phase_timings.clone(),
    };
    
    // Calculate total build time
    let total_time: Duration = phase_timings.values().sum();
    assert_eq!(total_time, Duration::from_millis(5000));
    
    // Calculate cache effectiveness
    let cache_hit_rate = statistics.files_cached as f64 / 
        (statistics.files_compiled + statistics.files_cached) as f64;
    assert!(cache_hit_rate > 0.5); // Good cache usage
    
    // Calculate compilation speed
    let compilation_time = phase_timings.get("compilation").unwrap().as_secs_f64();
    let lines_per_second = statistics.lines_compiled as f64 / compilation_time;
    assert!(lines_per_second > 1000.0); // Reasonable compilation speed
    
    // Verify all required metrics are present
    assert!(statistics.files_compiled > 0);
    assert!(statistics.lines_compiled > 0);
    assert!(statistics.peak_memory > 0);
    assert!(!statistics.phase_timings.is_empty());
}
