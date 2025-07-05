//! Performance Baseline and Regression Analysis Demo
//! 
//! Demonstrates the basic performance benchmarking and analysis
//! system for the CURSED compiler.

use cursed::optimization::{
    BenchmarkConfig, BenchmarkResult, RegressionSeverity,
    BaselineComparator, BaselineComparisonConfig,
    BaselineMetadata, EnvironmentInfo,
};
use cursed::codegen::llvm::optimization::OptimizationLevel;
use cursed::error::{CursedError, Result};
use std::path::PathBuf;
use std::time::{Duration, SystemTime};
use std::collections::HashMap;

fn main() -> Result<()> {
    println!("🚀 CURSED Performance Baseline and Regression Analysis Demo");
    println!("============================================================\n");

    // 1. Basic benchmark configuration
    benchmark_config_demo()?;
    
    // 2. Benchmark result comparison
    benchmark_comparison_demo()?;
    
    // 3. Regression severity analysis
    regression_severity_demo()?;
    
    // 4. Performance metadata tracking
    baseline_metadata_demo()?;

    println!("\n✅ Demo completed successfully!");
    Ok(())
}

fn benchmark_config_demo() -> Result<()> {
    println!("📊 1. Benchmark Configuration Demo");
    println!("----------------------------------");

    // Create benchmark configuration
    let config = BenchmarkConfig {
        iterations: 50,
        warmup_iterations: 5,
        timeout: Duration::from_secs(30),
    };

    println!("✓ Created benchmark configuration:");
    println!("  - Iterations: {}", config.iterations);
    println!("  - Warmup iterations: {}", config.warmup_iterations);
    println!("  - Timeout: {:?}", config.timeout);

    // Create sample benchmark results matching the actual API
    let results = vec![
        BenchmarkResult {
            duration: Duration::from_millis(1500),
            throughput: 1000.0, // operations per second
            memory_usage: 2048, // KB
            cpu_usage: 85.5, // percentage
        },
        BenchmarkResult {
            duration: Duration::from_millis(3200),
            throughput: 750.0,
            memory_usage: 4096,
            cpu_usage: 92.1,
        },
        BenchmarkResult {
            duration: Duration::from_millis(2100),
            throughput: 850.0,
            memory_usage: 3072,
            cpu_usage: 78.3,
        },
    ];

    println!("\n✓ Sample benchmark results:");
    for (i, result) in results.iter().enumerate() {
        println!("  Benchmark {}: {:?} duration, {:.1} ops/sec, {} KB memory, {:.1}% CPU",
                 i + 1, result.duration, result.throughput, result.memory_usage, result.cpu_usage);
    }

    Ok(())
}

fn benchmark_comparison_demo() -> Result<()> {
    println!("\n🔍 2. Benchmark Comparison Demo");
    println!("-------------------------------");

    // Create baseline comparator
    let config = BaselineComparisonConfig {
        tolerance: 0.05, // 5% tolerance
        min_samples: 3,
        confidence_level: 0.95,
        regression_threshold_percent: 5.0,
        improvement_threshold_percent: 5.0,
        min_confidence_level: 0.8,
        max_baseline_age_days: 30,
        use_statistical_testing: true,
    };

    let comparator = BaselineComparator::new(config);
    println!("✓ Created baseline comparator with 5% tolerance and 95% confidence");

    // Create baseline results
    let baseline_results = vec![
        BenchmarkResult {
            duration: Duration::from_millis(1000),
            throughput: 1000.0,
            memory_usage: 2048,
            cpu_usage: 80.0,
        },
        BenchmarkResult {
            duration: Duration::from_millis(2000),
            throughput: 500.0,
            memory_usage: 4096,
            cpu_usage: 85.0,
        },
    ];

    // Create current results (some regression)
    let current_results = vec![
        BenchmarkResult {
            duration: Duration::from_millis(1200), // 20% slower
            throughput: 950.0, // 5% slower
            memory_usage: 2200, // 7% more memory
            cpu_usage: 88.0, // 10% more CPU
        },
        BenchmarkResult {
            duration: Duration::from_millis(1900), // 5% faster
            throughput: 520.0, // 4% faster
            memory_usage: 3900, // 5% less memory
            cpu_usage: 82.0, // 4% less CPU
        },
    ];

    println!("\n📊 Baseline vs Current Performance:");
    println!("  Baseline - Duration: {:?}, Throughput: {:.1} ops/sec", 
             baseline_results[0].duration, baseline_results[0].throughput);
    println!("  Current  - Duration: {:?}, Throughput: {:.1} ops/sec", 
             current_results[0].duration, current_results[0].throughput);

    // Calculate performance changes
    let duration_change = ((current_results[0].duration.as_millis() as f64 - 
                           baseline_results[0].duration.as_millis() as f64) / 
                          baseline_results[0].duration.as_millis() as f64) * 100.0;

    let throughput_change = ((current_results[0].throughput - baseline_results[0].throughput) / 
                            baseline_results[0].throughput) * 100.0;

    println!("  Changes  - Duration: {:.1}%, Throughput: {:.1}%", 
             duration_change, throughput_change);

    if duration_change > 5.0 {
        println!("  ⚠️  Performance regression detected: duration increased by {:.1}%", duration_change);
    } else {
        println!("  ✅ Performance within acceptable range");
    }

    Ok(())
}

fn regression_severity_demo() -> Result<()> {
    println!("\n🚨 3. Regression Severity Analysis Demo");
    println!("--------------------------------------");

    // Demonstrate different severity levels
    let severity_cases = vec![
        (RegressionSeverity::Critical, "50% performance drop", "System unusable"),
        (RegressionSeverity::Major, "25% performance drop", "Significant impact"),
        (RegressionSeverity::Minor, "10% performance drop", "Noticeable but manageable"),
        (RegressionSeverity::Warning, "5% performance drop", "Within normal variation"),
    ];

    println!("✓ Regression severity levels:");
    for (severity, description, impact) in severity_cases {
        let icon = match severity {
            RegressionSeverity::Critical => "🔴",
            RegressionSeverity::Major => "🟠",
            RegressionSeverity::Minor => "🟡",
            RegressionSeverity::Warning => "🔵",
        };
        
        println!("  {} {:?}: {} - {}", icon, severity, description, impact);
    }

    // Simulate regression detection
    let performance_drops = vec![
        ("fibonacci_test", 55.0, RegressionSeverity::Critical),
        ("matrix_mult", 30.0, RegressionSeverity::Major),
        ("string_ops", 12.0, RegressionSeverity::Minor),
        ("basic_math", 3.0, RegressionSeverity::Warning),
    ];

    println!("\n📈 Simulated regression analysis:");
    for (test_name, drop_percent, severity) in performance_drops {
        let icon = match severity {
            RegressionSeverity::Critical => "🔴",
            RegressionSeverity::Major => "🟠",
            RegressionSeverity::Minor => "🟡",
            RegressionSeverity::Warning => "🔵",
        };
        
        println!("  {} {}: {:.1}% performance drop - {:?}", 
                 icon, test_name, drop_percent, severity);
    }

    Ok(())
}

fn baseline_metadata_demo() -> Result<()> {
    println!("\n📁 4. Baseline Metadata Tracking Demo");
    println!("------------------------------------");

    // Create environment info
    let env_info = EnvironmentInfo {
        os: "Linux".to_string(),
        arch: "x86_64".to_string(),
        cpu_count: 8,
        memory_gb: 16.0,
        cpu_cores: 8,
        memory_mb: 16384,
    };

    // Create baseline metadata
    let metadata = BaselineMetadata {
        version: "v1.0.0".to_string(),
        timestamp: SystemTime::now(),
        environment: env_info.clone(),
        benchmark_results: Vec::new(),
        metadata: cursed::optimization::BaselineInfo {
            version: "v1.0.0".to_string(),
            timestamp: SystemTime::now(),
            environment: env_info,
        },
    };

    println!("✓ Created baseline metadata:");
    println!("  - Version: {}", metadata.version);
    println!("  - Timestamp: {:?}", metadata.timestamp);
    println!("  - Environment:");
    println!("    - OS: {}", metadata.environment.os);
    println!("    - Architecture: {}", metadata.environment.arch);
    println!("    - CPU cores: {}", metadata.environment.cpu_count);
    println!("    - Memory: {:.1} GB", metadata.environment.memory_gb);

    // Demonstrate optimization level tracking
    println!("\n🔧 Optimization Level Tracking:");
    let optimization_levels = vec![
        (OptimizationLevel::O0, "No optimization - fastest compile"),
        (OptimizationLevel::O1, "Basic optimization - balanced"),
        (OptimizationLevel::O2, "Standard optimization - good performance"),
        (OptimizationLevel::O3, "Aggressive optimization - best performance"),
        (OptimizationLevel::Default, "Default optimization - alias for O2"),
    ];

    for (level, description) in optimization_levels {
        println!("  {:?}: {}", level, description);
    }

    // Show performance tracking best practices
    println!("\n💡 Performance Tracking Best Practices:");
    println!("  1. Track multiple metrics (duration, throughput, memory, CPU)");
    println!("  2. Use consistent test environments");
    println!("  3. Set appropriate regression thresholds");
    println!("  4. Monitor trends over time");
    println!("  5. Document baseline conditions");
    println!("  6. Test with different optimization levels");

    Ok(())
}
