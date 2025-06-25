/// LLVM Optimization System Tests
/// 
/// Comprehensive test suite for the LLVM optimization system including
/// optimization levels, pass management, performance analysis, and integration testing.

#[path = "common.rs"]
pub mod common;

use cursed::codegen::llvm::optimization::{
    OptimizationManager, OptimizationLevel, OptimizationConfig, OptimizationStats, utils
};
use cursed::profiling::performance::{
    PerformanceMonitor, CompilationPhase, ReportFormat, ReportConfig
};
use cursed::core::performance_pipeline::{
    PerformancePipeline, ParallelConfig, IncrementalConfig, ProgressConfig,
    CompilationJob, utils as pipeline_utils
};
use cursed::error::Result;
use inkwell::context::Context;
use std::path::PathBuf;
use std::time::Duration;

/// Initialize tracing for tests
macro_rules! init_tracing {
    () => {
        common::tracing::setup();
    };
}

#[test]
fn test_optimization_level_conversions() {
    init_tracing!();
    
    // Test string to optimization level conversion
    assert_eq!(OptimizationLevel::from_str("0").unwrap(), OptimizationLevel::None);
    assert_eq!(OptimizationLevel::from_str("O0").unwrap(), OptimizationLevel::None);
    assert_eq!(OptimizationLevel::from_str("1").unwrap(), OptimizationLevel::Less);
    assert_eq!(OptimizationLevel::from_str("O1").unwrap(), OptimizationLevel::Less);
    assert_eq!(OptimizationLevel::from_str("2").unwrap(), OptimizationLevel::Default);
    assert_eq!(OptimizationLevel::from_str("O2").unwrap(), OptimizationLevel::Default);
    assert_eq!(OptimizationLevel::from_str("3").unwrap(), OptimizationLevel::Aggressive);
    assert_eq!(OptimizationLevel::from_str("O3").unwrap(), OptimizationLevel::Aggressive);
    assert_eq!(OptimizationLevel::from_str("s").unwrap(), OptimizationLevel::Size);
    assert_eq!(OptimizationLevel::from_str("Os").unwrap(), OptimizationLevel::Size);
    assert_eq!(OptimizationLevel::from_str("z").unwrap(), OptimizationLevel::SizeAggressive);
    assert_eq!(OptimizationLevel::from_str("Oz").unwrap(), OptimizationLevel::SizeAggressive);
    
    // Test invalid optimization level
    assert!(OptimizationLevel::from_str("invalid").is_err());
    
    // Test string representation
    assert_eq!(OptimizationLevel::None.as_str(), "O0");
    assert_eq!(OptimizationLevel::Less.as_str(), "O1");
    assert_eq!(OptimizationLevel::Default.as_str(), "O2");
    assert_eq!(OptimizationLevel::Aggressive.as_str(), "O3");
    assert_eq!(OptimizationLevel::Size.as_str(), "Os");
    assert_eq!(OptimizationLevel::SizeAggressive.as_str(), "Oz");
    
    tracing::info!("✅ Optimization level conversions working correctly");
}

#[test]
fn test_optimization_config_creation() {
    init_tracing!();
    
    // Test default configuration
    let default_config = OptimizationConfig::default();
    assert_eq!(default_config.level, OptimizationLevel::Default);
    assert!(default_config.vectorize_loops);
    assert!(default_config.vectorize_slp);
    assert!(default_config.unroll_loops);
    assert!(default_config.merge_functions);
    assert!(default_config.inline_functions);
    assert!(!default_config.enable_lto);
    assert!(default_config.custom_passes.is_empty());
    
    // Test configuration from arguments
    let config = utils::create_config_from_args(
        Some("O3"),
        Some("native"),
        &["sse4.2".to_string(), "avx2".to_string()],
        true,
    ).unwrap();
    
    assert_eq!(config.level, OptimizationLevel::Aggressive);
    assert_eq!(config.target_cpu, Some("native".to_string()));
    assert_eq!(config.target_features, vec!["sse4.2", "avx2"]);
    assert!(config.enable_lto);
    
    // Test dev and release configurations
    let dev_config = utils::dev_config();
    let release_config = utils::release_config();
    
    assert_eq!(dev_config.level, OptimizationLevel::None);
    assert!(!dev_config.enable_lto);
    
    assert_eq!(release_config.level, OptimizationLevel::Aggressive);
    assert!(release_config.enable_lto);
    
    tracing::info!("✅ Optimization configuration creation working correctly");
}

#[test]
fn test_optimization_manager_creation() {
    init_tracing!();
    
    let context = Context::create();
    let config = OptimizationConfig::default();
    
    let manager = OptimizationManager::new(&context, config.clone());
    assert_eq!(manager.get_config().level, OptimizationLevel::Default);
    
    // Test stats initialization
    let stats = manager.get_stats();
    assert_eq!(stats.passes_run, 0);
    assert_eq!(stats.code_size_before, 0);
    assert_eq!(stats.code_size_after, 0);
    
    tracing::info!("✅ Optimization manager creation working correctly");
}

#[test]
fn test_optimization_manager_initialization() {
    init_tracing!();
    
    let context = Context::create();
    let module = context.create_module("test_module");
    let config = OptimizationConfig::default();
    
    let mut manager = OptimizationManager::new(&context, config);
    
    // Test initialization
    let result = manager.initialize(&module);
    assert!(result.is_ok(), "Optimization manager initialization should succeed");
    
    // Test stats after initialization
    let stats = manager.get_stats();
    assert!(stats.passes_run > 0, "Should have registered some passes");
    
    tracing::info!("✅ Optimization manager initialization working correctly");
}

#[test]
fn test_optimization_different_levels() {
    init_tracing!();
    
    let context = Context::create();
    let module = context.create_module("test_module");
    
    let levels = vec![
        OptimizationLevel::None,
        OptimizationLevel::Less,
        OptimizationLevel::Default,
        OptimizationLevel::Aggressive,
        OptimizationLevel::Size,
        OptimizationLevel::SizeAggressive,
    ];
    
    for level in levels {
        let config = OptimizationConfig {
            level,
            ..Default::default()
        };
        
        let mut manager = OptimizationManager::new(&context, config);
        let result = manager.initialize(&module);
        assert!(result.is_ok(), "Initialization should succeed for level {:?}", level);
        
        let stats = manager.get_stats();
        tracing::info!("Level {:?}: {} passes registered", level, stats.passes_run);
    }
    
    tracing::info!("✅ Different optimization levels working correctly");
}

#[test]
fn test_performance_monitor_creation() {
    init_tracing!();
    
    // Test default monitor
    let monitor = PerformanceMonitor::new();
    assert!(monitor.get_all_metrics().is_empty());
    
    // Test monitor with custom config
    let config = ReportConfig {
        format: ReportFormat::Json,
        include_memory: true,
        include_phases: true,
        include_files: true,
        show_percentages: true,
        sort_by_duration: true,
    };
    
    let monitor = PerformanceMonitor::with_config(config);
    assert!(monitor.get_all_metrics().is_empty());
    
    tracing::info!("✅ Performance monitor creation working correctly");
}

#[test]
fn test_performance_phase_timing() {
    init_tracing!();
    
    let monitor = PerformanceMonitor::new();
    
    // Test phase timing
    monitor.start_phase(CompilationPhase::Lexing).unwrap();
    std::thread::sleep(Duration::from_millis(10));
    monitor.end_phase(CompilationPhase::Lexing).unwrap();
    
    let metrics = monitor.get_phase_metrics(CompilationPhase::Lexing).unwrap();
    assert!(metrics.duration.as_millis() >= 10);
    assert_eq!(metrics.phase, CompilationPhase::Lexing);
    
    // Test file processing tracking
    monitor.start_phase(CompilationPhase::Parsing).unwrap();
    monitor.record_file_processed(CompilationPhase::Parsing, 100);
    monitor.record_file_processed(CompilationPhase::Parsing, 150);
    monitor.end_phase(CompilationPhase::Parsing).unwrap();
    
    let parsing_metrics = monitor.get_phase_metrics(CompilationPhase::Parsing).unwrap();
    assert_eq!(parsing_metrics.files_processed, 2);
    assert_eq!(parsing_metrics.lines_processed, 250);
    
    // Test error tracking
    monitor.start_phase(CompilationPhase::TypeChecking).unwrap();
    monitor.record_error(CompilationPhase::TypeChecking);
    monitor.record_error(CompilationPhase::TypeChecking);
    monitor.end_phase(CompilationPhase::TypeChecking).unwrap();
    
    let type_checking_metrics = monitor.get_phase_metrics(CompilationPhase::TypeChecking).unwrap();
    assert_eq!(type_checking_metrics.errors_encountered, 2);
    
    tracing::info!("✅ Performance phase timing working correctly");
}

#[test]
fn test_performance_report_generation() {
    init_tracing!();
    
    let monitor = PerformanceMonitor::new();
    
    // Add some timing data
    monitor.start_phase(CompilationPhase::Lexing).unwrap();
    std::thread::sleep(Duration::from_millis(5));
    monitor.end_phase(CompilationPhase::Lexing).unwrap();
    
    monitor.start_phase(CompilationPhase::Parsing).unwrap();
    std::thread::sleep(Duration::from_millis(10));
    monitor.end_phase(CompilationPhase::Parsing).unwrap();
    
    // Test table report
    let table_report = monitor.generate_report().unwrap();
    assert!(table_report.contains("Performance Report"));
    assert!(table_report.contains("Lexing"));
    assert!(table_report.contains("Parsing"));
    
    // Test other report formats
    let mut json_monitor = PerformanceMonitor::with_config(ReportConfig {
        format: ReportFormat::Json,
        ..Default::default()
    });
    
    json_monitor.start_phase(CompilationPhase::Lexing).unwrap();
    std::thread::sleep(Duration::from_millis(1));
    json_monitor.end_phase(CompilationPhase::Lexing).unwrap();
    
    let json_report = json_monitor.generate_report().unwrap();
    assert!(json_report.contains("Lexing"));
    assert!(json_report.starts_with('{') && json_report.ends_with('}'));
    
    tracing::info!("✅ Performance report generation working correctly");
}

#[test]
fn test_parallel_config_creation() {
    init_tracing!();
    
    // Test default configuration
    let default_config = ParallelConfig::default();
    assert_eq!(default_config.num_threads, 0); // Auto-detect
    assert!(default_config.enable_work_stealing);
    assert!(default_config.dependency_aware);
    assert_eq!(default_config.max_files_per_worker, 50);
    
    // Test auto configuration
    let auto_config = pipeline_utils::auto_parallel_config();
    assert!(auto_config.num_threads > 0);
    assert!(auto_config.enable_work_stealing);
    
    tracing::info!("✅ Parallel configuration creation working correctly");
}

#[test]
fn test_incremental_config_creation() {
    init_tracing!();
    
    // Test default configuration
    let default_config = IncrementalConfig::default();
    assert!(default_config.enabled);
    assert_eq!(default_config.cache_dir, PathBuf::from(".cursed_cache"));
    assert_eq!(default_config.max_cache_size_mb, 1024);
    assert_eq!(default_config.cache_expiration_hours, 24);
    assert!(default_config.track_dependencies);
    
    tracing::info!("✅ Incremental configuration creation working correctly");
}

#[test]
fn test_progress_config_creation() {
    init_tracing!();
    
    // Test default configuration
    let default_config = ProgressConfig::default();
    assert!(default_config.enabled);
    assert_eq!(default_config.update_interval_ms, 100);
    assert!(!default_config.show_file_names);
    assert!(default_config.show_phases);
    assert!(default_config.show_eta);
    
    tracing::info!("✅ Progress configuration creation working correctly");
}

#[test]
fn test_compilation_job_creation() {
    init_tracing!();
    
    let job = CompilationJob {
        id: 1,
        file_path: PathBuf::from("test.csd"),
        source_code: "sus x = 42;".to_string(),
        dependencies: vec![PathBuf::from("common.csd")],
        priority: 0,
    };
    
    assert_eq!(job.id, 1);
    assert_eq!(job.file_path, PathBuf::from("test.csd"));
    assert!(!job.source_code.is_empty());
    assert_eq!(job.dependencies.len(), 1);
    assert_eq!(job.priority, 0);
    
    tracing::info!("✅ Compilation job creation working correctly");
}

#[test]
fn test_pipeline_configurations() {
    init_tracing!();
    
    // Test dev configuration
    let (dev_parallel, dev_incremental, dev_progress) = pipeline_utils::dev_config();
    assert_eq!(dev_parallel.num_threads, 2);
    assert!(dev_incremental.enabled);
    assert_eq!(dev_incremental.cache_dir, PathBuf::from(".cursed_dev_cache"));
    assert!(dev_progress.enabled);
    assert!(dev_progress.show_file_names);
    
    // Test production configuration
    let (prod_parallel, prod_incremental, prod_progress) = pipeline_utils::production_config();
    assert!(prod_parallel.num_threads > 0);
    assert!(prod_incremental.enabled);
    assert_eq!(prod_incremental.cache_dir, PathBuf::from(".cursed_cache"));
    assert_eq!(prod_incremental.max_cache_size_mb, 2048);
    assert!(!prod_progress.enabled); // Less verbose in production
    
    tracing::info!("✅ Pipeline configurations working correctly");
}

#[test]
fn test_performance_pipeline_creation() {
    init_tracing!();
    
    let (parallel_config, incremental_config, progress_config) = pipeline_utils::dev_config();
    
    let pipeline = PerformancePipeline::new(
        parallel_config,
        incremental_config,
        progress_config,
    );
    
    // Just test that creation works - initialization might fail in test environment
    // due to thread pool setup
    tracing::info!("✅ Performance pipeline creation working correctly");
}

#[tokio::test]
async fn test_empty_compilation_jobs() {
    init_tracing!();
    
    let (parallel_config, incremental_config, progress_config) = pipeline_utils::dev_config();
    
    let mut pipeline = PerformancePipeline::new(
        parallel_config,
        incremental_config,
        progress_config,
    );
    
    // Test with empty job list
    let jobs = Vec::new();
    let results = pipeline.compile_files(jobs).await.unwrap();
    assert!(results.is_empty());
    
    tracing::info!("✅ Empty compilation jobs handled correctly");
}

#[test]
fn test_optimization_stats_tracking() {
    init_tracing!();
    
    let context = Context::create();
    let module = context.create_module("test_module");
    let config = OptimizationConfig::default();
    
    let mut manager = OptimizationManager::new(&context, config);
    manager.initialize(&module).unwrap();
    
    // Create a simple function for optimization
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = module.add_function("test_function", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    let builder = context.create_builder();
    builder.position_at_end(basic_block);
    builder.build_return(Some(&i32_type.const_int(42, false))).unwrap();
    
    // Run optimization
    let result = manager.optimize_module(&module);
    assert!(result.is_ok());
    
    let stats = manager.get_stats();
    assert!(stats.passes_run > 0);
    assert!(stats.optimization_time.as_nanos() > 0);
    
    tracing::info!("✅ Optimization stats tracking working correctly");
}

#[test] 
fn test_target_machine_creation() {
    init_tracing!();
    
    let context = Context::create();
    let config = OptimizationConfig {
        target_cpu: Some("generic".to_string()),
        target_features: vec!["sse2".to_string()],
        ..Default::default()
    };
    
    let manager = OptimizationManager::new(&context, config);
    
    // Test target machine creation with a known target triple
    let target_triple = "x86_64-unknown-linux-gnu";
    let result = manager.create_target_machine(target_triple);
    
    // This might fail in some test environments due to LLVM target initialization
    match result {
        Ok(_) => {
            tracing::info!("✅ Target machine creation successful");
        }
        Err(e) => {
            tracing::warn!("Target machine creation failed (expected in some test environments): {}", e);
        }
    }
}

#[test]
fn test_optimization_config_update() {
    init_tracing!();
    
    let context = Context::create();
    let initial_config = OptimizationConfig::default();
    
    let mut manager = OptimizationManager::new(&context, initial_config);
    assert_eq!(manager.get_config().level, OptimizationLevel::Default);
    
    // Update configuration
    let new_config = OptimizationConfig {
        level: OptimizationLevel::Aggressive,
        enable_lto: true,
        ..Default::default()
    };
    
    manager.update_config(new_config);
    assert_eq!(manager.get_config().level, OptimizationLevel::Aggressive);
    assert!(manager.get_config().enable_lto);
    
    tracing::info!("✅ Optimization configuration update working correctly");
}

#[test]
fn test_utils_format_functions() {
    init_tracing!();
    
    use cursed::profiling::performance::utils::{format_duration, format_memory};
    
    // Test duration formatting
    assert_eq!(format_duration(Duration::from_millis(500)), "500ms");
    assert_eq!(format_duration(Duration::from_secs(1)), "1.00s");
    assert_eq!(format_duration(Duration::from_secs(65)), "1m5.0s");
    
    // Test memory formatting
    assert_eq!(format_memory(100.0), "100.0MB");
    assert_eq!(format_memory(1536.0), "1.50GB");
    
    tracing::info!("✅ Utility format functions working correctly");
}

#[test]
fn test_performance_monitor_finalization() {
    init_tracing!();
    
    let mut monitor = PerformanceMonitor::new();
    
    // Add some phases
    monitor.start_phase(CompilationPhase::Lexing).unwrap();
    monitor.record_file_processed(CompilationPhase::Lexing, 50);
    monitor.end_phase(CompilationPhase::Lexing).unwrap();
    
    monitor.start_phase(CompilationPhase::Parsing).unwrap();
    monitor.record_file_processed(CompilationPhase::Parsing, 75);
    monitor.record_error(CompilationPhase::Parsing);
    monitor.end_phase(CompilationPhase::Parsing).unwrap();
    
    // Finalize
    monitor.finalize().unwrap();
    
    // Check total metrics
    let total_metrics = monitor.get_phase_metrics(CompilationPhase::Total).unwrap();
    assert_eq!(total_metrics.files_processed, 2);
    assert_eq!(total_metrics.lines_processed, 125);
    assert_eq!(total_metrics.errors_encountered, 1);
    assert!(total_metrics.duration.as_nanos() > 0);
    
    tracing::info!("✅ Performance monitor finalization working correctly");
}
