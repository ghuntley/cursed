//! Performance Baseline and Regression Analysis Demo
//! 
//! Demonstrates the comprehensive performance baseline and regression analysis
//! system for the CURSED compiler.

use cursed::optimization::{
    BaselineStorage, BaselineStorageConfig, BaselineType,
    RegressionAnalyzer, RegressionAnalysisConfig,
    BenchmarkRunner, BenchmarkConfig, BenchmarkResult, BenchmarkSuiteResult,
    BenchmarkStatistics, PerformanceThresholds,
};
use cursed::codegen::llvm::optimization::OptimizationLevel;
use cursed::error::Result;
use std::path::PathBuf;
use std::time::Duration;
use chrono::Utc;

fn main() -> Result<()> {
    println!("🚀 CURSED Performance Baseline and Regression Analysis Demo");
    println!("============================================================\n");

    // 1. Setup Baseline Storage
    setup_baseline_storage_demo()?;
    
    // 2. Demonstrate Regression Analysis
    regression_analysis_demo()?;
    
    // 3. Show Benchmark Runner Integration
    benchmark_runner_integration_demo()?;
    
    // 4. Advanced Features Demo
    advanced_features_demo()?;

    println!("\n✅ Demo completed successfully!");
    Ok(())
}

fn setup_baseline_storage_demo() -> Result<()> {
    println!("📊 1. Baseline Storage Demo");
    println!("---------------------------");

    // Create temporary storage for demo
    let storage_config = BaselineStorageConfig {
        storage_dir: PathBuf::from(".cursed/demo_baselines"),
        max_baselines: 10,
        auto_cleanup: true,
        min_confidence_level: 0.8,
    };

    let mut storage = BaselineStorage::new(storage_config)?;
    println!("✓ Created baseline storage in .cursed/demo_baselines");

    // Create mock benchmark results
    let benchmark_results = vec![
        BenchmarkResult {
            name: "fibonacci_recursive".to_string(),
            optimization_level: OptimizationLevel::Default,
            compile_time: Duration::from_millis(1500),
            runtime_performance: Some(Duration::from_millis(850)),
            binary_size: 2048,
            peak_memory_usage: 16384,
            optimization_passes: 12,
            success: true,
            error_message: None,
        },
        BenchmarkResult {
            name: "matrix_multiplication".to_string(),
            optimization_level: OptimizationLevel::Default,
            compile_time: Duration::from_millis(3200),
            runtime_performance: Some(Duration::from_millis(425)),
            binary_size: 4096,
            peak_memory_usage: 32768,
            optimization_passes: 15,
            success: true,
            error_message: None,
        },
        BenchmarkResult {
            name: "string_processing".to_string(),
            optimization_level: OptimizationLevel::Default,
            compile_time: Duration::from_millis(2100),
            runtime_performance: Some(Duration::from_millis(320)),
            binary_size: 3072,
            peak_memory_usage: 24576,
            optimization_passes: 18,
            success: true,
            error_message: None,
        },
    ];

    let suite_result = BenchmarkSuiteResult {
        suite_name: "demo_suite".to_string(),
        timestamp: Utc::now(),
        results: benchmark_results,
        statistics: BenchmarkStatistics {
            total_benchmarks: 3,
            successful_benchmarks: 3,
            avg_compile_time: Duration::from_millis(2267),
            avg_performance_improvement: 15.3,
            avg_size_change: -8.2,
            best_optimization_level: OptimizationLevel::Default,
        },
        regression_analysis: None,
    };

    // Create release baseline
    let baseline_id = storage.create_baseline(
        "v1.0.0".to_string(),
        BaselineType::Release,
        &suite_result,
        Some("a1b2c3d4e5f6".to_string()),
        Some("v1.0.0".to_string()),
    )?;

    println!("✓ Created release baseline: {}", baseline_id);
    println!("  - 3 benchmarks included");
    println!("  - Average compile time: 2.267s");
    println!("  - Linked to git commit: a1b2c3d4e5f6");

    // Set as default baseline
    storage.set_default_baseline(baseline_id.clone())?;
    println!("✓ Set as default baseline for comparisons");

    // Create development baseline with different performance
    let dev_results = vec![
        BenchmarkResult {
            name: "fibonacci_recursive".to_string(),
            optimization_level: OptimizationLevel::Default,
            compile_time: Duration::from_millis(1800), // 20% slower
            runtime_performance: Some(Duration::from_millis(720)), // 15% faster
            binary_size: 1950, // 5% smaller
            peak_memory_usage: 17000, // slightly more memory
            optimization_passes: 12,
            success: true,
            error_message: None,
        },
        BenchmarkResult {
            name: "matrix_multiplication".to_string(),
            optimization_level: OptimizationLevel::Default,
            compile_time: Duration::from_millis(3100), // slightly faster
            runtime_performance: Some(Duration::from_millis(400)), // 6% faster
            binary_size: 4200, // slightly larger
            peak_memory_usage: 31000, // less memory
            optimization_passes: 15,
            success: true,
            error_message: None,
        },
    ];

    let dev_suite = BenchmarkSuiteResult {
        suite_name: "dev_suite".to_string(),
        timestamp: Utc::now(),
        results: dev_results,
        statistics: BenchmarkStatistics {
            total_benchmarks: 2,
            successful_benchmarks: 2,
            avg_compile_time: Duration::from_millis(2450),
            avg_performance_improvement: 12.5,
            avg_size_change: 1.2,
            best_optimization_level: OptimizationLevel::Default,
        },
        regression_analysis: None,
    };

    let dev_baseline_id = storage.create_baseline(
        "dev_branch_optimization".to_string(),
        BaselineType::Development,
        &dev_suite,
        Some("b2c3d4e5f6a1".to_string()),
        None,
    )?;

    println!("✓ Created development baseline: {}", dev_baseline_id);

    // List all baselines
    let all_baselines = storage.list_baselines();
    println!("📋 Available baselines:");
    for baseline in &all_baselines {
        let loaded = storage.load_baseline(baseline)?.unwrap();
        println!("  - {} ({:?}, {} benchmarks)", 
                 loaded.name, loaded.baseline_type, loaded.benchmarks.len());
    }

    Ok(())
}

fn regression_analysis_demo() -> Result<()> {
    println!("\n🔍 2. Regression Analysis Demo");
    println!("------------------------------");

    // Setup storage and load baseline
    let storage_config = BaselineStorageConfig {
        storage_dir: PathBuf::from(".cursed/demo_baselines"),
        ..Default::default()
    };

    let mut storage = BaselineStorage::new(storage_config)?;
    let baseline = storage.get_default_baseline()
        .ok_or_else(|| cursed::error::Error::Other("No default baseline found".to_string()))?;

    println!("✓ Loaded baseline: {} (created {})", 
             baseline.name, baseline.created_at.format("%Y-%m-%d %H:%M:%S"));

    // Create regression analyzer with strict thresholds
    let regression_config = RegressionAnalysisConfig {
        thresholds: PerformanceThresholds {
            max_compile_time_increase: 25.0,  // Only 25% increase allowed
            min_runtime_improvement: 5.0,     // Expect at least 5% runtime improvement
            max_size_increase: 15.0,          // Max 15% size increase
            max_memory_increase: 20.0,        // Max 20% memory increase
        },
        confidence_level: 0.95,
        min_sample_size: 3,
        enable_trend_analysis: true,
        severity_mode: cursed::optimization::regression_analyzer::SeverityCalculationMode::Adaptive,
    };

    let mut analyzer = RegressionAnalyzer::new(regression_config);
    println!("✓ Configured regression analyzer with strict thresholds");

    // Test with regressed performance
    let regressed_results = vec![
        BenchmarkResult {
            name: "fibonacci_recursive".to_string(),
            optimization_level: OptimizationLevel::Default,
            compile_time: Duration::from_millis(2400), // 60% increase - REGRESSION!
            runtime_performance: Some(Duration::from_millis(950)), // 12% worse - REGRESSION!
            binary_size: 2600, // 27% increase - REGRESSION!
            peak_memory_usage: 22000, // 34% increase - REGRESSION!
            optimization_passes: 12,
            success: true,
            error_message: None,
        },
        BenchmarkResult {
            name: "matrix_multiplication".to_string(),
            optimization_level: OptimizationLevel::Default,
            compile_time: Duration::from_millis(3800), // 19% increase - acceptable
            runtime_performance: Some(Duration::from_millis(480)), // 13% worse - REGRESSION!
            binary_size: 4500, // 10% increase - acceptable
            peak_memory_usage: 35000, // 7% increase - acceptable
            optimization_passes: 15,
            success: true,
            error_message: None,
        },
    ];

    println!("\n🔬 Analyzing performance results...");
    let analysis = analyzer.analyze_regressions(&regressed_results, Some(baseline))?;

    println!("\n📈 Regression Analysis Results:");
    println!("==============================");

    if analysis.has_critical_regressions {
        println!("🚨 CRITICAL REGRESSIONS DETECTED!");
    } else if !analysis.regressions.is_empty() {
        println!("⚠️  Performance regressions detected");
    } else {
        println!("✅ No performance regressions detected");
    }

    println!("\n📊 Detected Regressions ({}):", analysis.regressions.len());
    for regression in &analysis.regressions {
        let severity_icon = match regression.severity {
            cursed::optimization::RegressionSeverity::Critical => "🔴",
            cursed::optimization::RegressionSeverity::Major => "🟠",
            cursed::optimization::RegressionSeverity::Minor => "🟡",
            cursed::optimization::RegressionSeverity::Warning => "🔵",
        };
        
        println!("  {} {:?} in {}: {:.1}% (threshold: {:.1}%)",
                 severity_icon,
                 regression.regression_type,
                 regression.benchmark_name,
                 regression.actual_value,
                 regression.expected_value);
        println!("    └─ {}", regression.description);
    }

    // Show baseline comparison
    if let Some(ref comparison) = analysis.baseline_comparison {
        println!("\n📊 Baseline Comparison:");
        println!("  Overall change: {:.1}%", comparison.overall_improvement);
        println!("  Individual improvements:");
        for improvement in &comparison.improvements {
            let direction = if improvement.improvement_percentage > 0.0 { "↗️" } else { "↘️" };
            println!("    {} {}: {:.1}% ({:?})",
                     direction,
                     improvement.benchmark_name,
                     improvement.improvement_percentage,
                     improvement.improvement_category);
        }
    }

    // Show recommendations
    println!("\n💡 Recommendations ({}):", analysis.recommendations.len());
    for (i, rec) in analysis.recommendations.iter().enumerate() {
        let priority_icon = match rec.priority {
            1 => "🔥",
            2 => "⚡",
            3 => "💡",
            _ => "💭",
        };
        
        println!("  {}. {} Priority {}: {}",
                 i + 1,
                 priority_icon,
                 rec.priority,
                 rec.recommendation);
        println!("     Effort: {:?}, Expected Impact: {:?}",
                 rec.estimated_effort,
                 rec.expected_impact);
    }

    Ok(())
}

fn benchmark_runner_integration_demo() -> Result<()> {
    println!("\n🏃 3. Benchmark Runner Integration Demo");
    println!("---------------------------------------");

    // Create benchmark runner with baseline integration
    let storage_config = BaselineStorageConfig {
        storage_dir: PathBuf::from(".cursed/demo_baselines"),
        ..Default::default()
    };

    let mut runner = BenchmarkRunner::new(
        PathBuf::from("cursed"), // Assuming cursed compiler in PATH
        PathBuf::from(".cursed/demo_work"),
    ).with_baseline_storage(storage_config)?;

    println!("✓ Created benchmark runner with baseline integration");

    // List available baselines
    let baselines = runner.list_baselines();
    println!("📋 Available baselines: {}", baselines.len());
    for baseline_id in &baselines {
        println!("  - {}", baseline_id);
    }

    // Mock creating a new baseline from benchmark results
    let new_results = vec![
        BenchmarkResult {
            name: "optimized_fibonacci".to_string(),
            optimization_level: OptimizationLevel::Aggressive,
            compile_time: Duration::from_millis(2800), // Longer compile time but...
            runtime_performance: Some(Duration::from_millis(650)), // Much better runtime!
            binary_size: 1800, // Smaller binary
            peak_memory_usage: 15000, // Less memory
            optimization_passes: 25,
            success: true,
            error_message: None,
        },
    ];

    let new_suite = BenchmarkSuiteResult {
        suite_name: "aggressive_optimization_test".to_string(),
        timestamp: Utc::now(),
        results: new_results,
        statistics: BenchmarkStatistics {
            total_benchmarks: 1,
            successful_benchmarks: 1,
            avg_compile_time: Duration::from_millis(2800),
            avg_performance_improvement: 23.5,
            avg_size_change: -12.2,
            best_optimization_level: OptimizationLevel::Aggressive,
        },
        regression_analysis: None,
    };

    // Create new baseline
    if let Some(new_baseline_id) = runner.create_baseline(
        "aggressive_optimization_v1".to_string(),
        BaselineType::Manual,
        &new_suite,
        Some("c3d4e5f6a1b2".to_string()),
        Some("v1.1.0-aggressive".to_string()),
    )? {
        println!("✓ Created new baseline: {}", new_baseline_id);
        
        // Set as new default
        runner.set_default_baseline(new_baseline_id)?;
        println!("✓ Set as new default baseline");
    }

    // Demonstrate export/import
    let export_path = PathBuf::from(".cursed/baselines_backup.json");
    runner.export_baselines(&export_path, None)?;
    println!("✓ Exported all baselines to: {}", export_path.display());

    Ok(())
}

fn advanced_features_demo() -> Result<()> {
    println!("\n🎯 4. Advanced Features Demo");
    println!("----------------------------");

    // Demonstrate trend analysis
    println!("📈 Trend Analysis:");
    
    let mut analyzer = RegressionAnalyzer::new(RegressionAnalysisConfig::default());
    
    // Simulate a series of measurements showing gradual performance degradation
    let trend_data = vec![
        ("Week 1", 1000, 1000, 5000),
        ("Week 2", 1050, 1020, 5100),
        ("Week 3", 1100, 1040, 5200),
        ("Week 4", 1180, 1080, 5350),
        ("Week 5", 1250, 1120, 5500),
    ];

    for (week, compile_ms, size, memory) in trend_data {
        let result = vec![BenchmarkResult {
            name: "trending_benchmark".to_string(),
            optimization_level: OptimizationLevel::Default,
            compile_time: Duration::from_millis(compile_ms),
            runtime_performance: Some(Duration::from_millis(400)),
            binary_size: size,
            peak_memory_usage: memory,
            optimization_passes: 12,
            success: true,
            error_message: None,
        }];

        let analysis = analyzer.analyze_regressions(&result, None)?;
        
        if !analysis.statistical_analysis.is_empty() {
            if let Some(stats) = analysis.statistical_analysis.get("trending_benchmark") {
                println!("  {}: Compile time trend direction: {:.3} (samples: {})",
                         week,
                         stats.trend_direction,
                         stats.sample_count);
            }
        }
    }

    // Show confidence intervals and statistical significance
    println!("\n🎲 Statistical Analysis:");
    println!("  - Trend detection uses linear regression on historical data");
    println!("  - Confidence intervals help determine measurement reliability");
    println!("  - Significance levels indicate whether changes are meaningful");

    // Performance prediction example
    println!("\n🔮 Performance Prediction:");
    println!("  - Based on current trends, next week's compile time estimate: 1320ms ± 50ms");
    println!("  - Prediction confidence: 87%");
    println!("  - Recommended action: Investigate optimization regression");

    println!("\n📁 Baseline Management Best Practices:");
    println!("  1. Create release baselines for each version");
    println!("  2. Use development baselines for feature branches");
    println!("  3. Set up CI/CD to automatically detect regressions");
    println!("  4. Export baselines regularly for backup");
    println!("  5. Review trend analysis weekly");

    Ok(())
}
