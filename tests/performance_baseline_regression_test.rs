//! Comprehensive tests for Performance Baseline and Regression Analysis System

use cursed::optimization::{
    BaselineStorage, BaselineStorageConfig, BaselineType, PerformanceBaseline,
    RegressionAnalyzer, RegressionAnalysisConfig, RegressionType, RegressionSeverity,
    BenchmarkRunner, BenchmarkConfig, BenchmarkResult, BenchmarkSuiteResult,
    BenchmarkStatistics, PerformanceThresholds,
};
use cursed::codegen::llvm::optimization::OptimizationLevel;
use cursed::error::Result;
use std::time::Duration;
use tempfile::TempDir;
use chrono::Utc;

/// Create a test benchmark result
fn create_test_benchmark_result(
    name: &str,
    compile_time_ms: u64,
    binary_size: usize,
    memory_usage: usize,
) -> BenchmarkResult {
    BenchmarkResult {
        name: name.to_string(),
        optimization_level: OptimizationLevel::Default,
        compile_time: Duration::from_millis(compile_time_ms),
        runtime_performance: Some(Duration::from_millis(500)),
        binary_size,
        peak_memory_usage: memory_usage,
        optimization_passes: 10,
        success: true,
        error_message: None,
    }
}

/// Create a test benchmark suite result
fn create_test_suite_result(name: &str, results: Vec<BenchmarkResult>) -> BenchmarkSuiteResult {
    let successful_benchmarks = results.iter().filter(|r| r.success).count();
    let avg_compile_time = if !results.is_empty() {
        Duration::from_nanos(
            results.iter()
                .map(|r| r.compile_time.as_nanos())
                .sum::<u128>() / results.len() as u128
        )
    } else {
        Duration::from_secs(0)
    };

    BenchmarkSuiteResult {
        suite_name: name.to_string(),
        timestamp: Utc::now(),
        results,
        statistics: BenchmarkStatistics {
            total_benchmarks: results.len(),
            successful_benchmarks,
            avg_compile_time,
            avg_performance_improvement: 15.0,
            avg_size_change: -5.0,
            best_optimization_level: OptimizationLevel::Default,
        },
        regression_analysis: None,
    }
}

#[test]
fn test_baseline_storage_creation_and_persistence() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let config = BaselineStorageConfig {
        storage_dir: temp_dir.path().to_path_buf(),
        max_baselines: 10,
        auto_cleanup: true,
        min_confidence_level: 0.7,
    };

    // Create baseline storage
    let mut storage = BaselineStorage::new(config)?;

    // Create test benchmark results
    let results = vec![
        create_test_benchmark_result("fibonacci", 2000, 1024, 8192),
        create_test_benchmark_result("matrix_multiply", 3000, 2048, 16384),
    ];
    let suite_result = create_test_suite_result("test_suite", results);

    // Create a baseline
    let baseline_id = storage.create_baseline(
        "v1.0.0".to_string(),
        BaselineType::Release,
        &suite_result,
        Some("abc123".to_string()),
        Some("v1.0.0".to_string()),
    )?;

    // Verify baseline was created
    let loaded_baseline = storage.load_baseline(&baseline_id)?;
    assert!(loaded_baseline.is_some());

    let baseline = loaded_baseline.unwrap();
    assert_eq!(baseline.name, "v1.0.0");
    assert_eq!(baseline.baseline_type, BaselineType::Release);
    assert_eq!(baseline.benchmarks.len(), 2);
    assert!(baseline.benchmarks.contains_key("fibonacci"));
    assert!(baseline.benchmarks.contains_key("matrix_multiply"));

    // Test that baseline persists after recreating storage
    drop(storage);
    let config = BaselineStorageConfig {
        storage_dir: temp_dir.path().to_path_buf(),
        max_baselines: 10,
        auto_cleanup: true,
        min_confidence_level: 0.7,
    };
    let mut new_storage = BaselineStorage::new(config)?;
    let reloaded_baseline = new_storage.load_baseline(&baseline_id)?;
    assert!(reloaded_baseline.is_some());

    Ok(())
}

#[test]
fn test_baseline_default_and_comparison() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let config = BaselineStorageConfig {
        storage_dir: temp_dir.path().to_path_buf(),
        ..Default::default()
    };

    let mut storage = BaselineStorage::new(config)?;

    // Create multiple baselines
    let results1 = vec![create_test_benchmark_result("test", 1000, 1000, 5000)];
    let suite1 = create_test_suite_result("suite1", results1);
    
    let results2 = vec![create_test_benchmark_result("test", 1500, 1200, 6000)];
    let suite2 = create_test_suite_result("suite2", results2);

    let baseline1_id = storage.create_baseline(
        "release_v1".to_string(),
        BaselineType::Release,
        &suite1,
        None,
        Some("v1.0.0".to_string()),
    )?;

    let baseline2_id = storage.create_baseline(
        "dev_branch".to_string(),
        BaselineType::Development,
        &suite2,
        None,
        None,
    )?;

    // Test default baseline (should be the release baseline)
    let default_baseline = storage.get_default_baseline();
    assert!(default_baseline.is_some());
    assert_eq!(default_baseline.unwrap().baseline_id, baseline1_id);

    // Test setting custom default
    storage.set_default_baseline(baseline2_id.clone())?;
    let new_default = storage.get_default_baseline();
    assert_eq!(new_default.unwrap().baseline_id, baseline2_id);

    // Test comparison baseline finding
    let comparison_baseline = storage.find_comparison_baseline("test");
    assert!(comparison_baseline.is_some());
    assert!(comparison_baseline.unwrap().benchmarks.contains_key("test"));

    Ok(())
}

#[test]
fn test_regression_analysis_comprehensive() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let config = BaselineStorageConfig {
        storage_dir: temp_dir.path().to_path_buf(),
        ..Default::default()
    };

    let mut storage = BaselineStorage::new(config)?;

    // Create baseline with good performance
    let baseline_results = vec![
        create_test_benchmark_result("compile_test", 1000, 1000, 5000),
        create_test_benchmark_result("runtime_test", 800, 800, 4000),
    ];
    let baseline_suite = create_test_suite_result("baseline", baseline_results);

    let baseline_id = storage.create_baseline(
        "baseline".to_string(),
        BaselineType::Manual,
        &baseline_suite,
        None,
        None,
    )?;

    let baseline = storage.load_baseline(&baseline_id)?.unwrap();

    // Create regression analyzer
    let regression_config = RegressionAnalysisConfig {
        thresholds: PerformanceThresholds {
            max_compile_time_increase: 50.0,
            min_runtime_improvement: 10.0,
            max_size_increase: 20.0,
            max_memory_increase: 30.0,
        },
        confidence_level: 0.95,
        min_sample_size: 3,
        enable_trend_analysis: true,
        severity_mode: cursed::optimization::regression_analyzer::SeverityCalculationMode::Adaptive,
    };

    let mut analyzer = RegressionAnalyzer::new(regression_config);

    // Test with regression results (performance got worse)
    let regression_results = vec![
        create_test_benchmark_result("compile_test", 2000, 1500, 8000), // 100% compile time increase, 50% size increase, 60% memory increase
        create_test_benchmark_result("runtime_test", 900, 900, 4500),   // 12.5% compile time increase, 12.5% size increase, 12.5% memory increase
    ];

    let analysis = analyzer.analyze_regressions(&regression_results, Some(baseline))?;

    // Should detect multiple regressions
    assert!(!analysis.regressions.is_empty());
    assert!(analysis.has_critical_regressions || analysis.regressions.len() > 1);

    // Check for specific regression types
    let compile_regressions: Vec<_> = analysis.regressions.iter()
        .filter(|r| r.regression_type == RegressionType::CompileTime)
        .collect();
    assert!(!compile_regressions.is_empty());

    let size_regressions: Vec<_> = analysis.regressions.iter()
        .filter(|r| r.regression_type == RegressionType::BinarySize)
        .collect();
    assert!(!size_regressions.is_empty());

    let memory_regressions: Vec<_> = analysis.regressions.iter()
        .filter(|r| r.regression_type == RegressionType::MemoryUsage)
        .collect();
    assert!(!memory_regressions.is_empty());

    // Check severity assignment
    let critical_regressions: Vec<_> = analysis.regressions.iter()
        .filter(|r| matches!(r.severity, RegressionSeverity::Critical))
        .collect();
    assert!(!critical_regressions.is_empty()); // 100% increase should be critical

    // Check baseline comparison
    assert!(analysis.baseline_comparison.is_some());
    let comparison = analysis.baseline_comparison.unwrap();
    assert!(!comparison.improvements.is_empty());

    // Check recommendations
    assert!(!analysis.recommendations.is_empty());
    
    Ok(())
}

#[test]
fn test_regression_analysis_no_regressions() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let config = BaselineStorageConfig {
        storage_dir: temp_dir.path().to_path_buf(),
        ..Default::default()
    };

    let mut storage = BaselineStorage::new(config)?;

    // Create baseline
    let baseline_results = vec![
        create_test_benchmark_result("test", 1000, 1000, 5000),
    ];
    let baseline_suite = create_test_suite_result("baseline", baseline_results);

    let baseline_id = storage.create_baseline(
        "baseline".to_string(),
        BaselineType::Manual,
        &baseline_suite,
        None,
        None,
    )?;

    let baseline = storage.load_baseline(&baseline_id)?.unwrap();

    // Create regression analyzer
    let mut analyzer = RegressionAnalyzer::new(RegressionAnalysisConfig::default());

    // Test with improved results (no regressions)
    let improved_results = vec![
        create_test_benchmark_result("test", 900, 950, 4800), // Slight improvements
    ];

    let analysis = analyzer.analyze_regressions(&improved_results, Some(baseline))?;

    // Should not detect regressions
    assert!(analysis.regressions.is_empty());
    assert!(!analysis.has_critical_regressions);

    // Should have baseline comparison showing improvements
    assert!(analysis.baseline_comparison.is_some());
    let comparison = analysis.baseline_comparison.unwrap();
    assert!(comparison.overall_improvement > 0.0);

    Ok(())
}

#[test]
fn test_benchmark_runner_with_baseline_integration() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let work_dir = TempDir::new().unwrap();

    // Create benchmark runner with baseline storage
    let storage_config = BaselineStorageConfig {
        storage_dir: temp_dir.path().to_path_buf(),
        ..Default::default()
    };

    let mut runner = BenchmarkRunner::new(
        "mock_compiler".into(),
        work_dir.path().to_path_buf(),
    ).with_baseline_storage(storage_config)?;

    // Test baseline operations
    let baselines = runner.list_baselines();
    assert!(baselines.is_empty()); // Should start empty

    // Create mock benchmark results
    let results = vec![
        create_test_benchmark_result("test1", 1000, 1000, 5000),
        create_test_benchmark_result("test2", 1500, 1200, 6000),
    ];
    let suite_result = create_test_suite_result("integration_test", results);

    // Create baseline from results
    let baseline_id = runner.create_baseline(
        "integration_baseline".to_string(),
        BaselineType::Manual,
        &suite_result,
        Some("integration_commit".to_string()),
        Some("v1.0.0".to_string()),
    )?;

    assert!(baseline_id.is_some());
    let baseline_id = baseline_id.unwrap();

    // Verify baseline was created
    let loaded = runner.load_baseline(&baseline_id)?;
    assert!(loaded);

    let baselines = runner.list_baselines();
    assert_eq!(baselines.len(), 1);
    assert!(baselines.contains(&baseline_id));

    // Test setting default baseline
    runner.set_default_baseline(baseline_id.clone())?;

    Ok(())
}

#[test]
fn test_baseline_export_import() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let export_dir = TempDir::new().unwrap();

    let config = BaselineStorageConfig {
        storage_dir: temp_dir.path().to_path_buf(),
        ..Default::default()
    };

    let mut storage = BaselineStorage::new(config)?;

    // Create multiple baselines
    let results1 = vec![create_test_benchmark_result("test1", 1000, 1000, 5000)];
    let suite1 = create_test_suite_result("suite1", results1);
    
    let results2 = vec![create_test_benchmark_result("test2", 1500, 1200, 6000)];
    let suite2 = create_test_suite_result("suite2", results2);

    let baseline1_id = storage.create_baseline(
        "baseline1".to_string(),
        BaselineType::Release,
        &suite1,
        None,
        Some("v1.0.0".to_string()),
    )?;

    let baseline2_id = storage.create_baseline(
        "baseline2".to_string(),
        BaselineType::Development,
        &suite2,
        None,
        None,
    )?;

    // Export all baselines
    let export_path = export_dir.path().join("baselines_export.json");
    storage.export_baselines(&export_path, None)?;
    assert!(export_path.exists());

    // Export specific baseline
    let export_path_specific = export_dir.path().join("baseline_specific.json");
    storage.export_baselines(&export_path_specific, Some(vec![baseline1_id.clone()]))?;
    assert!(export_path_specific.exists());

    // Create new storage and import
    let import_dir = TempDir::new().unwrap();
    let import_config = BaselineStorageConfig {
        storage_dir: import_dir.path().to_path_buf(),
        ..Default::default()
    };

    let mut import_storage = BaselineStorage::new(import_config)?;
    
    // Should start empty
    assert!(import_storage.list_baselines().is_empty());

    // Import baselines
    let imported_count = import_storage.import_baselines(&export_path, false)?;
    assert_eq!(imported_count, 2);

    // Verify imported baselines
    let imported_baselines = import_storage.list_baselines();
    assert_eq!(imported_baselines.len(), 2);

    let loaded_baseline1 = import_storage.load_baseline(&baseline1_id)?;
    assert!(loaded_baseline1.is_some());

    let loaded_baseline2 = import_storage.load_baseline(&baseline2_id)?;
    assert!(loaded_baseline2.is_some());

    Ok(())
}

#[test]
fn test_statistical_analysis_and_trends() -> Result<()> {
    let mut analyzer = RegressionAnalyzer::new(RegressionAnalysisConfig::default());

    // Simulate multiple benchmark runs with varying performance
    let benchmark_series = vec![
        create_test_benchmark_result("trending_test", 1000, 1000, 5000),
        create_test_benchmark_result("trending_test", 1100, 1050, 5200),
        create_test_benchmark_result("trending_test", 1200, 1100, 5400),
        create_test_benchmark_result("trending_test", 1300, 1150, 5600),
        create_test_benchmark_result("trending_test", 1400, 1200, 5800),
    ];

    // Analyze trends over time (simulating degrading performance)
    for (i, result) in benchmark_series.iter().enumerate() {
        let single_result = vec![result.clone()];
        let analysis = analyzer.analyze_regressions(&single_result, None)?;
        
        if i >= 2 { // After a few measurements, should have statistical data
            assert!(!analysis.statistical_analysis.is_empty());
            
            if let Some(stats) = analysis.statistical_analysis.get("trending_test") {
                assert!(stats.sample_count > 0);
                assert!(stats.confidence_interval.0 < stats.confidence_interval.1);
                
                // Later in the series, trend should be degrading (positive trend direction)
                if i >= 4 {
                    assert!(stats.trend_direction > 0.0);
                }
            }
        }
    }

    Ok(())
}

#[test]
fn test_baseline_cleanup_and_limits() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let config = BaselineStorageConfig {
        storage_dir: temp_dir.path().to_path_buf(),
        max_baselines: 3, // Small limit for testing
        auto_cleanup: true,
        min_confidence_level: 0.7,
    };

    let mut storage = BaselineStorage::new(config)?;

    // Create more baselines than the limit
    for i in 0..5 {
        let results = vec![create_test_benchmark_result(&format!("test{}", i), 1000, 1000, 5000)];
        let suite_result = create_test_suite_result(&format!("suite{}", i), results);

        storage.create_baseline(
            format!("baseline_{}", i),
            if i == 0 { BaselineType::Release } else { BaselineType::Development },
            &suite_result,
            None,
            None,
        )?;
        
        // Allow some time for different timestamps
        std::thread::sleep(Duration::from_millis(10));
    }

    // Should not exceed the maximum (but release baselines are preserved)
    let baselines = storage.list_baselines();
    assert!(baselines.len() <= 4); // 3 recent + 1 release baseline protected

    Ok(())
}
