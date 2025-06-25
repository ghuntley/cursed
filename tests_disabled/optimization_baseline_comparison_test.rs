//! Tests for the baseline comparison system

use cursed::optimization::{
    BaselineComparator, BaselineComparisonConfig, BaselineMetadata, EnvironmentInfo,
    BenchmarkResult, BenchmarkSuiteResult,
};
use cursed::error::Result;
use std::time::Duration;
use tempfile::tempdir;

#[test]
fn test_baseline_comparator_creation() {
    let temp_dir = tempdir().unwrap();
    let config = BaselineComparisonConfig::default();
    let _comparator = BaselineComparator::new(temp_dir.path(), config);
}

#[test]
fn test_create_baseline_from_benchmark_results() -> Result<()> {
    let temp_dir = tempdir().unwrap();
    let config = BaselineComparisonConfig::default();
    let comparator = BaselineComparator::new(temp_dir.path(), config);

    let benchmark_results = create_sample_benchmark_results();
    let metadata = BaselineMetadata {
        commit_hash: Some("abc123".to_string()),
        environment: EnvironmentInfo {
            os: "linux".to_string(),
            arch: "x86_64".to_string(),
            cpu_cores: 8,
            memory_mb: 16384,
        },
        compiler_config: "release".to_string(),
        notes: Some("Initial baseline".to_string()),
    };

    let baseline = comparator.create_baseline(&benchmark_results, "1.0.0".to_string(), metadata)?;
    
    assert_eq!(baseline.version, "1.0.0");
    assert_eq!(baseline.benchmark_results.len(), 3);
    assert!(baseline.benchmark_results.contains_key("compile_speed"));
    
    Ok(())
}

#[test]
fn test_baseline_comparison_no_regression() -> Result<()> {
    let temp_dir = tempdir().unwrap();
    let config = BaselineComparisonConfig {
        regression_threshold_percent: 10.0,
        improvement_threshold_percent: 10.0,
        ..Default::default()
    };
    let comparator = BaselineComparator::new(temp_dir.path(), config);

    let baseline_results = create_sample_benchmark_results();
    let current_results = create_sample_benchmark_results(); // Same as baseline

    let metadata = create_sample_metadata();
    let baseline = comparator.create_baseline(&baseline_results, "1.0.0".to_string(), metadata)?;
    
    let comparison = comparator.compare_against_baseline(&current_results, &baseline)?;
    
    assert!(!comparison.has_regressions);
    assert!(!comparison.has_improvements);
    assert!(comparison.overall_change_percent.abs() < 5.0);
    
    Ok(())
}

#[test]
fn test_baseline_comparison_with_regression() -> Result<()> {
    let temp_dir = tempdir().unwrap();
    let config = BaselineComparisonConfig {
        regression_threshold_percent: 5.0,
        improvement_threshold_percent: 5.0,
        ..Default::default()
    };
    let comparator = BaselineComparator::new(temp_dir.path(), config);

    let baseline_results = create_sample_benchmark_results();
    let mut current_results = create_sample_benchmark_results();
    
    // Make current results slower (regression)
    for result in &mut current_results.results {
        result.execution_times = result.execution_times.iter()
            .map(|&time| time + Duration::from_millis(200)) // 20% slower
            .collect();
    }

    let metadata = create_sample_metadata();
    let baseline = comparator.create_baseline(&baseline_results, "1.0.0".to_string(), metadata)?;
    
    let comparison = comparator.compare_against_baseline(&current_results, &baseline)?;
    
    assert!(comparison.has_regressions);
    assert!(comparison.overall_change_percent < -5.0); // Negative means regression
    assert!(comparison.summary.contains("REGRESSION"));
    
    Ok(())
}

#[test]
fn test_baseline_comparison_with_improvement() -> Result<()> {
    let temp_dir = tempdir().unwrap();
    let config = BaselineComparisonConfig {
        regression_threshold_percent: 5.0,
        improvement_threshold_percent: 5.0,
        ..Default::default()
    };
    let comparator = BaselineComparator::new(temp_dir.path(), config);

    let baseline_results = create_sample_benchmark_results();
    let mut current_results = create_sample_benchmark_results();
    
    // Make current results faster (improvement)
    for result in &mut current_results.results {
        result.execution_times = result.execution_times.iter()
            .map(|&time| time.saturating_sub(Duration::from_millis(200))) // 20% faster
            .collect();
    }

    let metadata = create_sample_metadata();
    let baseline = comparator.create_baseline(&baseline_results, "1.0.0".to_string(), metadata)?;
    
    let comparison = comparator.compare_against_baseline(&current_results, &baseline)?;
    
    assert!(comparison.has_improvements);
    assert!(comparison.overall_change_percent > 5.0); // Positive means improvement
    assert!(comparison.summary.contains("improvement"));
    
    Ok(())
}

#[test]
fn test_baseline_save_and_load() -> Result<()> {
    let temp_dir = tempdir().unwrap();
    let config = BaselineComparisonConfig::default();
    let comparator = BaselineComparator::new(temp_dir.path(), config);

    // Create and save baseline
    let benchmark_results = create_sample_benchmark_results();
    let metadata = create_sample_metadata();
    let _baseline = comparator.create_baseline(&benchmark_results, "1.0.0".to_string(), metadata)?;

    // Load baseline
    let loaded_baseline = comparator.load_latest_baseline()?;
    
    assert!(loaded_baseline.is_some());
    let baseline = loaded_baseline.unwrap();
    assert_eq!(baseline.version, "1.0.0");
    assert_eq!(baseline.benchmark_results.len(), 3);
    
    Ok(())
}

#[test]
fn test_confidence_level_calculation() -> Result<()> {
    let temp_dir = tempdir().unwrap();
    let config = BaselineComparisonConfig {
        min_confidence_level: 0.7,
        ..Default::default()
    };
    let comparator = BaselineComparator::new(temp_dir.path(), config);

    let baseline_results = create_sample_benchmark_results();
    let current_results = create_sample_benchmark_results();

    let metadata = create_sample_metadata();
    let baseline = comparator.create_baseline(&baseline_results, "1.0.0".to_string(), metadata)?;
    
    let comparison = comparator.compare_against_baseline(&current_results, &baseline)?;
    
    // Check that confidence levels are calculated
    for (_, benchmark_comparison) in &comparison.benchmark_comparisons {
        assert!(benchmark_comparison.confidence_level >= 0.0);
        assert!(benchmark_comparison.confidence_level <= 1.0);
    }
    
    Ok(())
}

#[test]
fn test_environment_info_collection() {
    let env_info = BaselineComparator::get_current_environment();
    
    assert!(!env_info.os.is_empty());
    assert!(!env_info.arch.is_empty());
    assert!(env_info.cpu_cores > 0);
    assert!(env_info.memory_mb > 0);
}

#[test]
fn test_statistical_significance() -> Result<()> {
    let temp_dir = tempdir().unwrap();
    let config = BaselineComparisonConfig {
        use_statistical_testing: true,
        min_confidence_level: 0.8,
        ..Default::default()
    };
    let comparator = BaselineComparator::new(temp_dir.path(), config);

    let baseline_results = create_sample_benchmark_results();
    let mut current_results = create_sample_benchmark_results();
    
    // Create a small change that might not be statistically significant
    for result in &mut current_results.results {
        result.execution_times = result.execution_times.iter()
            .map(|&time| time + Duration::from_millis(5)) // Very small change
            .collect();
    }

    let metadata = create_sample_metadata();
    let baseline = comparator.create_baseline(&baseline_results, "1.0.0".to_string(), metadata)?;
    
    let comparison = comparator.compare_against_baseline(&current_results, &baseline)?;
    
    // Small changes should not be flagged as regressions due to low confidence
    assert!(!comparison.has_regressions);
    
    Ok(())
}

// Helper functions

fn create_sample_benchmark_results() -> BenchmarkSuiteResult {
    BenchmarkSuiteResult {
        suite_name: "test_suite".to_string(),
        results: vec![
            BenchmarkResult {
                benchmark_name: "compile_speed".to_string(),
                execution_times: vec![
                    Duration::from_millis(1000),
                    Duration::from_millis(1100),
                    Duration::from_millis(900),
                    Duration::from_millis(1050),
                    Duration::from_millis(950),
                ],
                memory_usage_mb: Some(256.0),
                cpu_utilization: Some(75.0),
                success: true,
                error_message: None,
            },
            BenchmarkResult {
                benchmark_name: "memory_usage".to_string(),
                execution_times: vec![
                    Duration::from_millis(500),
                    Duration::from_millis(520),
                    Duration::from_millis(480),
                    Duration::from_millis(510),
                    Duration::from_millis(490),
                ],
                memory_usage_mb: Some(128.0),
                cpu_utilization: Some(60.0),
                success: true,
                error_message: None,
            },
            BenchmarkResult {
                benchmark_name: "optimization_effectiveness".to_string(),
                execution_times: vec![
                    Duration::from_millis(2000),
                    Duration::from_millis(2100),
                    Duration::from_millis(1900),
                    Duration::from_millis(2050),
                    Duration::from_millis(1950),
                ],
                memory_usage_mb: Some(512.0),
                cpu_utilization: Some(85.0),
                success: true,
                error_message: None,
            },
        ],
        total_time: Duration::from_secs(10),
        successful_benchmarks: 3,
        failed_benchmarks: 0,
        regression_analysis: None,
    }
}

fn create_sample_metadata() -> BaselineMetadata {
    BaselineMetadata {
        commit_hash: Some("abc123def456".to_string()),
        environment: EnvironmentInfo {
            os: "linux".to_string(),
            arch: "x86_64".to_string(),
            cpu_cores: 8,
            memory_mb: 16384,
        },
        compiler_config: "OptimizationLevel::Release".to_string(),
        notes: Some("Test baseline for integration testing".to_string()),
    }
}
