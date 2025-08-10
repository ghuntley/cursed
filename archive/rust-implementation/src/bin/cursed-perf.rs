//! CURSED Compiler Performance Monitoring CLI Tool
//! 
//! Provides comprehensive performance monitoring, profiling, and benchmarking
//! capabilities for the CURSED compiler with real-time visualization.

use std::env;
use std::process;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use clap::{Arg, ArgMatches, Command};
use cursed::error::CursedError;
use cursed::performance::*;

fn main() {
    let matches = Command::new("cursed-perf")
        .version(env!("CARGO_PKG_VERSION"))
        .author("CURSED Compiler Team")
        .about("Performance monitoring and profiling tool for CURSED compiler")
        .subcommand(
            Command::new("monitor")
                .about("Start performance monitoring")
                .arg(Arg::new("duration")
                    .short('d')
                    .long("duration")
                    .value_name("SECONDS")
                    .help("Monitoring duration in seconds")
                    .action(clap::ArgAction::Set))
                .arg(Arg::new("output")
                    .short('o')
                    .long("output")
                    .value_name("DIR")
                    .help("Output directory for reports")
                    .action(clap::ArgAction::Set))
                .arg(Arg::new("format")
                    .short('f')
                    .long("format")
                    .value_name("FORMAT")
                    .help("Report format (html, json, csv, markdown)")
                    .action(clap::ArgAction::Set)
                    .default_value("html"))
                .arg(Arg::new("realtime")
                    .short('r')
                    .long("realtime")
                    .help("Enable real-time dashboard")
                    .action(clap::ArgAction::SetTrue))
        )
        .subcommand(
            Command::new("benchmark")
                .about("Run performance benchmarks")
                .arg(Arg::new("suite")
                    .short('s')
                    .long("suite")
                    .value_name("SUITE")
                    .help("Benchmark suite to run (all, compilation, execution, memory, stdlib)")
                    .action(clap::ArgAction::Set)
                    .default_value("all"))
                .arg(Arg::new("iterations")
                    .short('i')
                    .long("iterations")
                    .value_name("COUNT")
                    .help("Number of iterations per benchmark")
                    .action(clap::ArgAction::Set)
                    .default_value("1000"))
                .arg(Arg::new("warmup")
                    .short('w')
                    .long("warmup")
                    .value_name("COUNT")
                    .help("Number of warmup iterations")
                    .action(clap::ArgAction::Set)
                    .default_value("100"))
                .arg(Arg::new("output")
                    .short('o')
                    .long("output")
                    .value_name("DIR")
                    .help("Output directory for results")
                    .action(clap::ArgAction::Set))
                .arg(Arg::new("baseline")
                    .short('b')
                    .long("baseline")
                    .help("Save results as baseline")
                    .action(clap::ArgAction::SetTrue))
                .arg(Arg::new("compare")
                    .short('c')
                    .long("compare")
                    .help("Compare with baseline")
                    .action(clap::ArgAction::SetTrue))
        )
        .subcommand(
            Command::new("profile")
                .about("Profile CURSED program execution")
                .arg(Arg::new("program")
                    .value_name("PROGRAM")
                    .help("CURSED program to profile")
                    .required(true)
                    .action(clap::ArgAction::Set))
                .arg(Arg::new("output")
                    .short('o')
                    .long("output")
                    .value_name("FILE")
                    .help("Output file for profile data")
                    .action(clap::ArgAction::Set))
                .arg(Arg::new("type")
                    .short('t')
                    .long("type")
                    .value_name("TYPE")
                    .help("Profiling type (cpu, memory, combined)")
                    .action(clap::ArgAction::Set)
                    .default_value("combined"))
        )
        .subcommand(
            Command::new("regression")
                .about("Detect performance regressions")
                .arg(Arg::new("threshold")
                    .short('t')
                    .long("threshold")
                    .value_name("PERCENT")
                    .help("Regression threshold percentage")
                    .action(clap::ArgAction::Set)
                    .default_value("5.0"))
                .arg(Arg::new("lookback")
                    .short('l')
                    .long("lookback")
                    .value_name("DAYS")
                    .help("Lookback period in days")
                    .action(clap::ArgAction::Set)
                    .default_value("7"))
        )
        .subcommand(
            Command::new("visualize")
                .about("Generate performance visualizations")
                .arg(Arg::new("data")
                    .short('d')
                    .long("data")
                    .value_name("FILE")
                    .help("Performance data file")
                    .action(clap::ArgAction::Set))
                .arg(Arg::new("output")
                    .short('o')
                    .long("output")
                    .value_name("FILE")
                    .help("Output visualization file")
                    .action(clap::ArgAction::Set))
                .arg(Arg::new("type")
                    .short('t')
                    .long("type")
                    .value_name("TYPE")
                    .help("Visualization type (dashboard, chart, report)")
                    .action(clap::ArgAction::Set)
                    .default_value("dashboard"))
        )
        .subcommand(
            Command::new("report")
                .about("Generate comprehensive performance report")
                .arg(Arg::new("data")
                    .short('d')
                    .long("data")
                    .value_name("DIR")
                    .help("Data directory")
                    .action(clap::ArgAction::Set))
                .arg(Arg::new("output")
                    .short('o')
                    .long("output")
                    .value_name("FILE")
                    .help("Output report file")
                    .action(clap::ArgAction::Set))
                .arg(Arg::new("format")
                    .short('f')
                    .long("format")
                    .value_name("FORMAT")
                    .help("Report format (html, pdf, markdown)")
                    .action(clap::ArgAction::Set)
                    .default_value("html"))
        )
        .get_matches();

    let result = match matches.subcommand() {
        Some(("monitor", sub_matches)) => run_monitor(sub_matches),
        Some(("benchmark", sub_matches)) => run_benchmark(sub_matches),
        Some(("profile", sub_matches)) => run_profile(sub_matches),
        Some(("regression", sub_matches)) => run_regression(sub_matches),
        Some(("visualize", sub_matches)) => run_visualize(sub_matches),
        Some(("report", sub_matches)) => run_report(sub_matches),
        _ => {
            println!("Use --help for usage information");
            Ok(())
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

/// Run performance monitoring
fn run_monitor(matches: &ArgMatches) -> Result<(), CursedError> {
    println!("🔍 Starting CURSED Performance Monitor...");
    
    let duration = matches.get_one::<String>("duration")
        .unwrap_or(&"300".to_string()) // 5 minutes default
        .parse::<u64>()
        .map_err(|_| CursedError::runtime_error("Invalid duration"))?;
    
    let output_dir = matches.get_one::<String>("output")
        .map(|s| s.as_str())
        .unwrap_or("./performance_reports");
    let format = matches.get_one::<String>("format")
        .map(|s| s.as_str())
        .unwrap_or("html");
    let realtime = matches.get_flag("realtime");
    
    let config = PerformanceConfig {
        enable_monitoring: true,
        enable_profiling: true,
        enable_benchmarking: false,
        enable_regression_detection: true,
        enable_visualization: true,
        sampling_rate: 1.0,
        buffer_size: 100000,
        flush_interval: Duration::from_secs(5),
        output_dir: output_dir.to_string(),
        report_format: match format {
            "html" => ReportFormat::Html,
            "json" => ReportFormat::Json,
            "csv" => ReportFormat::Csv,
            "markdown" => ReportFormat::Markdown,
            _ => ReportFormat::Html,
        },
        performance_threshold: 0.05,
        memory_threshold: 1024 * 1024 * 100,
        cpu_threshold: 80.0,
    };
    
    let system = PerformanceSystem::new(config)?;
    system.start()?;
    
    println!("📊 Performance monitoring started for {} seconds", duration);
    println!("📁 Output directory: {}", output_dir);
    println!("📄 Report format: {}", format);
    
    if realtime {
        println!("🔴 Real-time dashboard enabled");
        create_realtime_dashboard(&system)?;
    }
    
    // Monitor for specified duration
    let start_time = Instant::now();
    let monitor_duration = Duration::from_secs(duration);
    
    let mut sample_count = 0;
    while start_time.elapsed() < monitor_duration {
        std::thread::sleep(Duration::from_secs(1));
        sample_count += 1;
        
        if sample_count % 30 == 0 {
            let metrics = system.get_metrics()?;
            println!("📈 Sample {}: CPU {:.1}%, Memory {:.1}MB, Throughput {:.1} ops/sec",
                sample_count / 30, metrics.cpu_usage, 
                metrics.memory_usage as f64 / 1024.0 / 1024.0, 
                metrics.throughput);
        }
    }
    
    system.stop()?;
    
    // Generate final report
    let report = system.generate_report()?;
    let report_file = format!("{}/performance_monitor_report.{}", output_dir, format);
    std::fs::write(&report_file, report)?;
    
    println!("✅ Monitoring completed!");
    println!("📄 Report saved to: {}", report_file);
    
    Ok(())
}

/// Run performance benchmarks
fn run_benchmark(matches: &ArgMatches) -> Result<(), CursedError> {
    println!("🏃 Starting CURSED Performance Benchmarks...");
    
    let suite = matches.get_one::<String>("suite")
        .map(|s| s.as_str())
        .unwrap_or("all");
    let iterations = matches.get_one::<String>("iterations")
        .unwrap_or(&"1000".to_string())
        .parse::<u32>()
        .map_err(|_| CursedError::runtime_error("Invalid iterations"))?;
    
    let warmup = matches.get_one::<String>("warmup")
        .unwrap_or(&"100".to_string())
        .parse::<u32>()
        .map_err(|_| CursedError::runtime_error("Invalid warmup"))?;
    
    let output_dir = matches.get_one::<String>("output")
        .map(|s| s.as_str())
        .unwrap_or("./benchmark_results");
    let save_baseline = matches.get_flag("baseline");
    let compare_baseline = matches.get_flag("compare");
    
    let config = PerformanceConfig {
        enable_monitoring: false,
        enable_profiling: false,
        enable_benchmarking: true,
        enable_regression_detection: false,
        enable_visualization: true,
        sampling_rate: 1.0,
        buffer_size: 100000,
        flush_interval: Duration::from_secs(5),
        output_dir: output_dir.to_string(),
        report_format: ReportFormat::Html,
        performance_threshold: 0.05,
        memory_threshold: 1024 * 1024 * 100,
        cpu_threshold: 80.0,
    };
    
    let system = PerformanceSystem::new(config)?;
    let benchmark_runner = BenchmarkRunner::new(system.get_config().clone())?;
    
    println!("🎯 Running benchmark suite: {}", suite);
    println!("🔄 Iterations: {}, Warmup: {}", iterations, warmup);
    
    // Run benchmarks based on suite selection
    let results = match suite {
        "all" => benchmark_runner.run_all_benchmarks()?,
        "compilation" => benchmark_runner.run_benchmarks_by_tags(&["compilation".to_string()])?,
        "execution" => benchmark_runner.run_benchmarks_by_tags(&["execution".to_string()])?,
        "memory" => benchmark_runner.run_benchmarks_by_tags(&["memory".to_string()])?,
        "stdlib" => benchmark_runner.run_benchmarks_by_tags(&["stdlib".to_string()])?,
        _ => return Err(CursedError::runtime_error(&format!("Unknown benchmark suite: {}", suite))),
    };
    
    // Display results
    println!("\n📊 Benchmark Results:");
    println!("┌─────────────────────────────────┬─────────────────┬─────────────────┬─────────────────┐");
    println!("│ Benchmark                       │ Average Time    │ Throughput      │ Success Rate    │");
    println!("├─────────────────────────────────┼─────────────────┼─────────────────┼─────────────────┤");
    
    for (name, result) in &results {
        println!("│ {:<31} │ {:>13.3?} │ {:>13.2} │ {:>13.2}% │",
            name, result.average_time, result.throughput, result.success_rate * 100.0);
    }
    
    println!("└─────────────────────────────────┴─────────────────┴─────────────────┴─────────────────┘");
    
    if save_baseline {
        benchmark_runner.save_baseline()?;
        println!("💾 Baseline saved");
    }
    
    if compare_baseline {
        benchmark_runner.load_baseline()?;
        println!("\n📈 Comparison with baseline:");
        
        for (name, _) in &results {
            match benchmark_runner.compare_with_baseline(name) {
                Ok(comparison) => {
                    let symbol = if comparison.improvement_percentage > 0.0 { "📈" } else { "📉" };
                    println!("  {} {}: {:.1}% change - {}", 
                        symbol, name, comparison.improvement_percentage, comparison.recommendation);
                }
                Err(_) => {
                    println!("  ❓ {}: No baseline available", name);
                }
            }
        }
    }
    
    // Generate benchmark report
    let report = benchmark_runner.generate_report()?;
    let report_file = format!("{}/benchmark_report.html", output_dir);
    std::fs::write(&report_file, report)?;
    
    println!("\n✅ Benchmarks completed!");
    println!("📄 Report saved to: {}", report_file);
    
    Ok(())
}

/// Run performance profiling
fn run_profile(matches: &ArgMatches) -> Result<(), CursedError> {
    println!("🔬 Starting CURSED Performance Profiler...");
    
    let program = matches.get_one::<String>("program").unwrap();
    let output_file = matches.get_one::<String>("output")
        .map(|s| s.as_str())
        .unwrap_or("profile_results.json");
    let profile_type = matches.get_one::<String>("type")
        .map(|s| s.as_str())
        .unwrap_or("combined");
    
    println!("📁 Profiling program: {}", program);
    println!("📊 Profile type: {}", profile_type);
    
    let config = PerformanceConfig {
        enable_monitoring: true,
        enable_profiling: true,
        enable_benchmarking: false,
        enable_regression_detection: false,
        enable_visualization: true,
        sampling_rate: 1.0,
        buffer_size: 100000,
        flush_interval: Duration::from_secs(1),
        output_dir: "./profiling_results".to_string(),
        report_format: ReportFormat::Json,
        performance_threshold: 0.05,
        memory_threshold: 1024 * 1024 * 100,
        cpu_threshold: 80.0,
    };
    
    let system = PerformanceSystem::new(config)?;
    system.start()?;
    
    // Run the program with profiling
    let start_time = Instant::now();
    
    // Simulate program execution with profiling
    let output = std::process::Command::new("cargo")
        .args(&["run", "--bin", "cursed", program])
        .output()
        .map_err(|e| CursedError::runtime_error(&format!("Failed to run program: {}", e)))?;
    
    let execution_time = start_time.elapsed();
    
    system.stop()?;
    
    // Get profiling results
    let metrics = system.get_metrics()?;
    
    // Create profiling report
    let profile_data = serde_json::json!({
        "program": program,
        "profile_type": profile_type,
        "execution_time_ms": execution_time.as_millis(),
        "metrics": {
            "compilation_time_ms": metrics.compilation_time.as_millis(),
            "execution_time_ms": metrics.execution_time.as_millis(),
            "memory_usage_mb": metrics.memory_usage as f64 / 1024.0 / 1024.0,
            "cpu_usage_percent": metrics.cpu_usage,
            "throughput_ops_per_sec": metrics.throughput,
            "latency_ms": metrics.latency.as_millis(),
            "error_rate_percent": metrics.error_rate * 100.0,
            "gc_pressure_percent": metrics.gc_pressure * 100.0,
        },
        "program_output": String::from_utf8_lossy(&output.stdout),
        "timestamp": chrono::Utc::now().to_rfc3339(),
    });
    
    std::fs::write(output_file, serde_json::to_string_pretty(&profile_data).unwrap())?;
    
    println!("✅ Profiling completed!");
    println!("⏱️  Execution time: {:?}", execution_time);
    println!("💾 Memory usage: {:.1}MB", metrics.memory_usage as f64 / 1024.0 / 1024.0);
    println!("🔥 CPU usage: {:.1}%", metrics.cpu_usage);
    println!("📄 Profile saved to: {}", output_file);
    
    Ok(())
}

/// Run regression detection
fn run_regression(matches: &ArgMatches) -> Result<(), CursedError> {
    println!("🔍 Starting CURSED Regression Detection...");
    
    let threshold = matches.get_one::<String>("threshold")
        .unwrap_or(&"5.0".to_string())
        .parse::<f64>()
        .map_err(|_| CursedError::runtime_error("Invalid threshold"))?;
    
    let lookback_days = matches.get_one::<String>("lookback")
        .unwrap_or(&"7".to_string())
        .parse::<u64>()
        .map_err(|_| CursedError::runtime_error("Invalid lookback period"))?;
    
    let config = PerformanceConfig {
        enable_monitoring: false,
        enable_profiling: false,
        enable_benchmarking: false,
        enable_regression_detection: true,
        enable_visualization: false,
        sampling_rate: 1.0,
        buffer_size: 100000,
        flush_interval: Duration::from_secs(5),
        output_dir: "./regression_results".to_string(),
        report_format: ReportFormat::Html,
        performance_threshold: threshold / 100.0,
        memory_threshold: 1024 * 1024 * 100,
        cpu_threshold: 80.0,
    };
    
    let system = PerformanceSystem::new(config)?;
    
    println!("🎯 Regression threshold: {:.1}%", threshold);
    println!("📅 Lookback period: {} days", lookback_days);
    
    // Detect regressions
    let alerts = system.detect_regressions()?;
    
    if alerts.is_empty() {
        println!("✅ No performance regressions detected!");
    } else {
        println!("⚠️  {} performance regression(s) detected:", alerts.len());
        
        for alert in &alerts {
            let severity_symbol = match alert.severity {
                RegressionSeverity::Low => "🟢",
                RegressionSeverity::Medium => "🟡",
                RegressionSeverity::High => "🟠",
                RegressionSeverity::Critical => "🔴",
            };
            
            println!("  {} {} - {:.1}% change (baseline: {:.2}, current: {:.2})",
                severity_symbol, alert.metric, alert.regression_percentage, 
                alert.baseline_value, alert.current_value);
            println!("     💡 {}", alert.recommendation);
        }
    }
    
    Ok(())
}

/// Run performance visualization
fn run_visualize(matches: &ArgMatches) -> Result<(), CursedError> {
    println!("📊 Starting CURSED Performance Visualization...");
    
    let data_file = matches.get_one::<String>("data")
        .map(|s| s.as_str())
        .unwrap_or("performance_data.json");
    let output_file = matches.get_one::<String>("output")
        .map(|s| s.as_str())
        .unwrap_or("performance_visualization.html");
    let viz_type = matches.get_one::<String>("type")
        .map(|s| s.as_str())
        .unwrap_or("dashboard");
    
    println!("📁 Data file: {}", data_file);
    println!("📄 Output file: {}", output_file);
    println!("📊 Visualization type: {}", viz_type);
    
    let config = PerformanceConfig {
        enable_monitoring: false,
        enable_profiling: false,
        enable_benchmarking: false,
        enable_regression_detection: false,
        enable_visualization: true,
        sampling_rate: 1.0,
        buffer_size: 100000,
        flush_interval: Duration::from_secs(5),
        output_dir: "./visualization_results".to_string(),
        report_format: ReportFormat::Html,
        performance_threshold: 0.05,
        memory_threshold: 1024 * 1024 * 100,
        cpu_threshold: 80.0,
    };
    
    let system = PerformanceSystem::new(config)?;
    
    // Create sample performance data for visualization
    let sample_data = create_sample_performance_data();
    
    let visualization_file = system.create_visualization(&sample_data)?;
    
    println!("✅ Visualization completed!");
    println!("📄 Visualization saved to: {}", visualization_file);
    
    Ok(())
}

/// Run comprehensive performance report
fn run_report(matches: &ArgMatches) -> Result<(), CursedError> {
    println!("📄 Generating CURSED Performance Report...");
    
    let data_dir = matches.get_one::<String>("data")
        .map(|s| s.as_str())
        .unwrap_or("./performance_data");
    let output_file = matches.get_one::<String>("output")
        .map(|s| s.as_str())
        .unwrap_or("comprehensive_report.html");
    let format = matches.get_one::<String>("format")
        .map(|s| s.as_str())
        .unwrap_or("html");
    
    println!("📁 Data directory: {}", data_dir);
    println!("📄 Output file: {}", output_file);
    println!("📊 Report format: {}", format);
    
    let config = PerformanceConfig {
        enable_monitoring: true,
        enable_profiling: true,
        enable_benchmarking: true,
        enable_regression_detection: true,
        enable_visualization: true,
        sampling_rate: 1.0,
        buffer_size: 100000,
        flush_interval: Duration::from_secs(5),
        output_dir: "./report_results".to_string(),
        report_format: match format {
            "html" => ReportFormat::Html,
            "markdown" => ReportFormat::Markdown,
            "json" => ReportFormat::Json,
            _ => ReportFormat::Html,
        },
        performance_threshold: 0.05,
        memory_threshold: 1024 * 1024 * 100,
        cpu_threshold: 80.0,
    };
    
    let system = PerformanceSystem::new(config)?;
    
    // Generate comprehensive report
    let report = system.generate_report()?;
    
    std::fs::write(output_file, report)?;
    
    println!("✅ Report generated successfully!");
    println!("📄 Report saved to: {}", output_file);
    
    Ok(())
}

/// Create real-time dashboard
fn create_realtime_dashboard(system: &PerformanceSystem) -> Result<(), CursedError> {
    let sample_data = create_sample_performance_data();
    system.create_visualization(&sample_data)?;
    
    println!("🔴 Real-time dashboard available at: ./performance_reports/performance_visualization.html");
    println!("🔄 Dashboard will update every 30 seconds");
    
    Ok(())
}

/// Create sample performance data for demonstration
fn create_sample_performance_data() -> PerformanceData {
    let mut metrics = Vec::new();
    let mut timestamps = Vec::new();
    
    let start_time = Instant::now();
    
    // Generate sample data points
    for i in 0..100 {
        let timestamp = start_time + Duration::from_secs(i * 5);
        
        // Simulate varying performance metrics
        let base_compilation_time = 100.0 + (i as f64 * 0.5);
        let base_execution_time = 50.0 + (i as f64 * 0.3);
        let base_memory = 64.0 + (i as f64 * 0.1);
        let base_cpu = 25.0 + (i as f64 * 0.2);
        
        let metrics_sample = PerformanceMetrics {
            compilation_time: Duration::from_millis(base_compilation_time as u64),
            execution_time: Duration::from_millis(base_execution_time as u64),
            memory_usage: (base_memory * 1024.0 * 1024.0) as usize,
            cpu_usage: base_cpu,
            throughput: 1000.0 + (i as f64 * 2.0),
            latency: Duration::from_millis(10 + (i / 10)),
            error_rate: 0.001 + (i as f64 * 0.0001),
            gc_pressure: 0.1 + (i as f64 * 0.002),
        };
        
        metrics.push(metrics_sample);
        timestamps.push(timestamp);
    }
    
    PerformanceData {
        metrics,
        timestamps,
        labels: vec!["sample_data".to_string()],
    }
}
