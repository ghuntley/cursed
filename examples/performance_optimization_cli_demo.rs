/// Performance Optimization CLI Demo
/// 
/// Demonstrates how to integrate the performance optimization system with CLI commands
/// and build processes for maximum compilation and runtime performance.

use cursed::optimization::{
    performance_integration::{
        PerformanceIntegrationSystem, PerformanceIntegrationConfig, PerformanceTargets,
    },
    config::{OptimizationConfig, OptimizationProfile, OptimizationLevel},
    build_integration::{create_build_optimizer_from_args_with_performance, BuildContext},
};
use cursed::error::Result;
use std::path::PathBuf;
use std::time::Instant;
use clap::{Command, Arg, ArgMatches};

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    let matches = create_cli_app().get_matches();
    
    match matches.subcommand() {
        Some(("build", sub_matches)) => run_build_command(sub_matches),
        Some(("benchmark", sub_matches)) => run_benchmark_command(sub_matches),
        Some(("optimize", sub_matches)) => run_optimize_command(sub_matches),
        Some(("analyze", sub_matches)) => run_analyze_command(sub_matches),
        _ => {
            println!("Use --help to see available commands");
            Ok(())
        }
    }
}

fn create_cli_app() -> Command {
    Command::new("cursed-perf")
        .version("1.0.0")
        .about("CURSED Performance Optimization CLI Demo")
        .subcommand(
            Command::new("build")
                .about("Build with performance optimization")
                .arg(
                    Arg::new("source")
                        .help("Source files to compile")
                        .num_args(1..)
                        .required(true)
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("DIR")
                        .help("Output directory")
                )
                .arg(
                    Arg::new("release")
                        .long("release")
                        .help("Build in release mode with maximum optimization")
                        .action(clap::ArgAction::SetTrue)
                )
                .arg(
                    Arg::new("debug")
                        .long("debug")
                        .help("Build in debug mode with fast compilation")
                        .action(clap::ArgAction::SetTrue)
                )
                .arg(
                    Arg::new("profile")
                        .long("profile")
                        .value_name("PROFILE")
                        .help("Optimization profile (dev, release, debug, size, performance)")
                        )
                .arg(
                    Arg::new("parallel")
                        .short('j')
                        .long("parallel")
                        .value_name("N")
                        .help("Number of parallel workers")
                        )
                .arg(
                    Arg::new("adaptive")
                        .long("adaptive")
                        .help("Enable adaptive optimization")
                        .action(clap::ArgAction::SetTrue)
                )
                .arg(
                    Arg::new("verbose")
                        .short('v')
                        .long("verbose")
                        .help("Enable verbose output and detailed reports")
                        .action(clap::ArgAction::SetTrue)
                )
                .arg(
                    Arg::new("no-performance")
                        .long("no-performance")
                        .help("Disable performance integration system")
                        .action(clap::ArgAction::SetTrue)
                )
        )
        .subcommand(
            Command::new("benchmark")
                .about("Run performance benchmarks")
                .arg(
                    Arg::new("benchmark-name")
                        .help("Benchmark configuration name (quick, thorough)")
                        .default_value("quick")
                )
                .arg(
                    Arg::new("source")
                        .help("Source files for benchmarking")
                        .num_args(1..)
                        .required(true)
                )
                .arg(
                    Arg::new("iterations")
                        .short('i')
                        .long("iterations")
                        .value_name("N")
                        .help("Number of benchmark iterations")
                        )
        )
        .subcommand(
            Command::new("optimize")
                .about("Run performance optimization analysis")
                .arg(
                    Arg::new("source")
                        .help("Source files to analyze")
                        .num_args(1..)
                        .required(true)
                )
                .arg(
                    Arg::new("target-improvement")
                        .long("target")
                        .value_name("PERCENT")
                        .help("Target performance improvement percentage")
                        )
                .arg(
                    Arg::new("report")
                        .long("report")
                        .value_name("FILE")
                        .help("Output detailed report to file")
                        )
        )
        .subcommand(
            Command::new("analyze")
                .about("Analyze project characteristics for optimization")
                .arg(
                    Arg::new("project-root")
                        .help("Project root directory")
                        .default_value(".")
                )
                .arg(
                    Arg::new("output-format")
                        .long("format")
                        .value_name("FORMAT")
                        .help("Output format (json, yaml, table)")
                        .default_value("table")
                        )
        )
}

fn run_build_command(matches: &ArgMatches) -> Result<()> {
    println!("🚀 Starting CURSED Performance-Optimized Build");
    
    let source_files: Vec<PathBuf> = matches.get_many::<String>("source")
        .unwrap()
        .map(|s| PathBuf::from(s))
        .collect();
    
    let output_dir = matches.get_one::<String>("output").map(PathBuf::from);
    let debug = matches.get_flag("debug");
    let release = matches.get_flag("release");
    let verbose = matches.get_flag("verbose");
    let enable_performance = !matches.get_flag("no-performance");
    
    println!("📁 Source files: {} files", source_files.len());
    println!("🔧 Build mode: {}", if debug { "Debug" } else if release { "Release" } else { "Default" });
    println!("⚡ Performance integration: {}", if enable_performance { "Enabled" } else { "Disabled" });
    
    let project_root = std::env::current_dir()?;
    
    // Create build optimizer with performance integration
    let mut optimizer = create_build_optimizer_from_args_with_performance(
        project_root,
        source_files,
        output_dir,
        None, // target
        debug,
        release,
        verbose,
        enable_performance,
    )?;
    
    let build_start = Instant::now();
    
    // Run optimized build
    let result = optimizer.optimize_build()?;
    
    let build_time = build_start.elapsed();
    
    // Display results
    println!("\n📊 Build Results:");
    println!("  ✅ Success: {}", result.success);
    println!("  ⏱️  Total time: {:.2?}", result.total_time);
    println!("  🔨 Compilation time: {:.2?}", result.compilation_time);
    println!("  ⚡ Optimization time: {:.2?}", result.optimization_time);
    println!("  📦 Files compiled: {}", result.files_compiled);
    println!("  💾 Files cached: {} ({:.1}% hit rate)", result.files_cached, result.cache_hit_rate * 100.0);
    println!("  🔀 Parallel efficiency: {:.1}%", result.parallel_efficiency * 100.0);
    println!("  📏 Size reduction: {} bytes", result.size_reduction_bytes);
    
    if let Some(profile) = &result.optimization_profile_used {
        println!("  🎯 Optimization profile: {:?}", profile);
    }
    
    if result.adaptive_optimization_enabled {
        println!("  🧠 Adaptive optimization: Enabled");
    }
    
    if let Some(improvements) = &result.performance_improvements {
        println!("\n📈 Performance Improvements:");
        println!("  ⏰ Compilation time saved: {:.2?}", improvements.compilation_time_saved);
        println!("  🏃 Runtime improvement estimate: {:.1}%", improvements.runtime_improvement_estimate);
        println!("  💾 Memory usage reduction: {:.1}%", improvements.memory_usage_reduction);
        println!("  📦 Binary size reduction: {:.1}%", improvements.binary_size_reduction);
    }
    
    if !result.optimization_recommendations.is_empty() {
        println!("\n💡 Optimization Recommendations:");
        for (i, rec) in result.optimization_recommendations.iter().enumerate() {
            println!("  {}. [{:?}] {} (Expected: {:.1}% improvement)",
                i + 1, rec.category, rec.description, rec.expected_improvement);
        }
    }
    
    if !result.warnings.is_empty() {
        println!("\n⚠️  Warnings:");
        for warning in &result.warnings {
            println!("  - {}", warning);
        }
    }
    
    if !result.errors.is_empty() {
        println!("\n❌ Errors:");
        for error in &result.errors {
            println!("  - {}", error);
        }
    }
    
    println!("\n{}", result.performance_summary);
    
    Ok(())
}

fn run_benchmark_command(matches: &ArgMatches) -> Result<()> {
    println!("🏁 Running Performance Benchmarks");
    
    let benchmark_name = matches.get_one::<String>("benchmark-name").unwrap();
    let source_files: Vec<PathBuf> = matches.get_many::<String>("source")
        .unwrap()
        .map(|s| PathBuf::from(s))
        .collect();
    
    println!("📋 Benchmark: {}", benchmark_name);
    println!("📁 Source files: {} files", source_files.len());
    
    // Create performance integration system
    let config = PerformanceIntegrationConfig::default();
    let optimization_config = OptimizationConfig::default();
    let system = PerformanceIntegrationSystem::new(config, optimization_config)?;
    
    // Run benchmark
    let benchmark_start = Instant::now();
    let results = system.run_performance_benchmarks(benchmark_name)?;
    let benchmark_time = benchmark_start.elapsed();
    
    // Display results
    println!("\n📊 Benchmark Results:");
    println!("  ⏱️  Total benchmark time: {:.2?}", benchmark_time);
    println!("  🔄 Iterations: {}", results.iterations);
    println!("  📊 Average time: {:.2?}", results.average_time);
    println!("  ⚡ Best time: {:.2?}", results.min_time);
    println!("  🐌 Worst time: {:.2?}", results.max_time);
    println!("  📏 Standard deviation: {:.2?}", results.std_deviation);
    
    if results.iterations > 1 {
        println!("\n📈 Individual Results:");
        for i in 0..results.iterations {
            println!("  Run {}: {:.2?}", i + 1, results.avg_time_per_iteration);
        }
    }
    
    // Performance analysis
    let efficiency_score = calculate_efficiency_score(&results);
    println!("\n🎯 Performance Score: {:.1}/100", efficiency_score);
    
    if efficiency_score < 70.0 {
        println!("💡 Consider enabling adaptive optimization or using release mode for better performance");
    }
    
    Ok(())
}

fn run_optimize_command(matches: &ArgMatches) -> Result<()> {
    println!("🔍 Running Performance Optimization Analysis");
    
    let source_files: Vec<PathBuf> = matches.get_many::<String>("source")
        .unwrap()
        .map(|s| PathBuf::from(s))
        .collect();
    
    let target_improvement = matches.get_one::<String>("target-improvement")
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(25.0);
    
    println!("📁 Source files: {} files", source_files.len());
    println!("🎯 Target improvement: {:.1}%", target_improvement);
    
    // Create performance integration system with custom targets
    let config = PerformanceIntegrationConfig {
        target_improvements: PerformanceTargets {
            compilation_time_reduction: target_improvement,
            runtime_performance_improvement: target_improvement,
            memory_usage_reduction: target_improvement * 0.6,
            binary_size_reduction: target_improvement * 0.4,
            compilation_time_target_ms: 5000,
            runtime_improvement_target: target_improvement,
            memory_reduction_target: target_improvement * 0.6,
            binary_size_reduction_target: target_improvement * 0.4,
        },
        enable_automatic_reporting: true,
        ..Default::default()
    };
    
    let optimization_config = OptimizationConfig {
        optimization_level: OptimizationLevel::Aggressive,
        enable_parallel: true,
        enable_incremental: true,
        profile_guided: true,
        ..Default::default()
    };
    
    let mut system = PerformanceIntegrationSystem::new(config, optimization_config)?;
    
    // Run optimization analysis
    let output_path = std::env::temp_dir().join("cursed_optimization_analysis");
    let results = system.optimize_project(&source_files, &output_path)?;
    
    // Display analysis results
    println!("\n📊 Optimization Analysis Results:");
    println!("  🎯 Profile used: {:?}", results.optimization_profile);
    println!("  ⏱️  Compilation time: {:.2?}", results.compilation_time);
    println!("  🔀 Parallel efficiency: {:.1}%", results.parallel_efficiency * 100.0);
    println!("  💾 Cache hit rate: {:.1}%", results.cache_hit_rate * 100.0);
    
    println!("\n📈 Performance Improvements:");
    let improvements = &results.performance_improvements;
    println!("  ⏰ Compilation time saved: {:.2?}", improvements.compilation_time_saved);
    println!("  🏃 Runtime improvement: {:.1}%", improvements.runtime_improvement_estimate);
    println!("  💾 Memory reduction: {:.1}%", improvements.memory_usage_reduction);
    println!("  📦 Size reduction: {:.1}%", improvements.binary_size_reduction);
    
    // Check if targets were met
    println!("\n🎯 Target Achievement:");
    let runtime_met = improvements.runtime_improvement_estimate >= target_improvement;
    let compile_time_met = improvements.compilation_time_saved.as_secs_f64() >= target_improvement / 100.0 * 10.0; // Rough estimate
    
    println!("  🏃 Runtime target: {} ({:.1}% vs {:.1}%)", 
        if runtime_met { "✅ Met" } else { "❌ Not met" },
        improvements.runtime_improvement_estimate, target_improvement);
    println!("  ⏰ Compile time target: {}", 
        if compile_time_met { "✅ Met" } else { "❌ Not met" });
    
    if !results.recommendations.is_empty() {
        println!("\n💡 Optimization Recommendations:");
        for (i, rec) in results.recommendations.iter().enumerate() {
            let effort_icon = match rec.implementation_effort {
                cursed::optimization::performance_integration::ImplementationEffort::Low => "🟢",
                cursed::optimization::performance_integration::ImplementationEffort::Medium => "🟡",
                cursed::optimization::performance_integration::ImplementationEffort::High => "🔴",
            };
            println!("  {}. {} [{:?}] {} (Expected: {:.1}% improvement)",
                i + 1, effort_icon, rec.category, rec.description, rec.expected_improvement);
        }
    }
    
    // Save detailed report if requested
    if let Some(report_file) = matches.get_one::<String>("report") {
        save_optimization_report(&results, report_file)?;
        println!("\n📄 Detailed report saved to: {}", report_file);
    }
    
    Ok(())
}

fn run_analyze_command(matches: &ArgMatches) -> Result<()> {
    println!("📊 Analyzing Project Characteristics");
    
    let project_root = PathBuf::from(matches.get_one::<String>("project-root").unwrap());
    let output_format = matches.get_one::<String>("output-format").unwrap();
    
    println!("📁 Project root: {}", project_root.display());
    
    // Discover source files
    let source_files = discover_cursed_files(&project_root)?;
    println!("🔍 Found {} CURSED source files", source_files.len());
    
    // Analyze project characteristics
    let characteristics = analyze_project_characteristics(&source_files)?;
    
    match output_format.as_str() {
        "json" => print_characteristics_json(&characteristics),
        "yaml" => print_characteristics_yaml(&characteristics),
        _ => print_characteristics_table(&characteristics),
    }
    
    // Provide optimization recommendations based on characteristics
    provide_project_recommendations(&characteristics);
    
    Ok(())
}

// Helper functions

fn calculate_efficiency_score(results: &cursed::optimization::benchmarking::BenchmarkResults) -> f64 {
    let base_score = 100.0;
    let avg_ms = results.average_time.as_millis() as f64;
    
    // Simple scoring: faster is better
    if avg_ms < 1000.0 {
        base_score
    } else if avg_ms < 5000.0 {
        base_score - (avg_ms - 1000.0) / 100.0
    } else {
        50.0 // Cap at 50 for very slow builds
    }
}

fn save_optimization_report(
    results: &cursed::optimization::performance_integration::IntegratedOptimizationResults,
    filename: &str,
) -> Result<()> {
    let report = serde_json::json!({
        "optimization_profile": format!("{:?}", results.optimization_profile),
        "compilation_time_ms": results.compilation_time.as_millis(),
        "parallel_efficiency": results.parallel_efficiency,
        "cache_hit_rate": results.cache_hit_rate,
        "performance_improvements": {
            "compilation_time_saved_ms": results.performance_improvements.compilation_time_saved.as_millis(),
            "runtime_improvement_percent": results.performance_improvements.runtime_improvement_estimate,
            "memory_reduction_percent": results.performance_improvements.memory_usage_reduction,
            "size_reduction_percent": results.performance_improvements.binary_size_reduction,
        },
        "recommendations": results.recommendations.iter().map(|r| {
            serde_json::json!({
                "category": format!("{:?}", r.category),
                "description": r.description,
                "expected_improvement": r.expected_improvement,
                "implementation_effort": format!("{:?}", r.implementation_effort),
            })
        }).collect::<Vec<_>>(),
        "timestamp": chrono::Utc::now().to_rfc3339(),
    });
    
    std::fs::write(filename, serde_json::to_string_pretty(&report).map_err(|e| cursed::error::Error::Other(e.to_string()))?)?;
    Ok(())
}

fn discover_cursed_files(root: &PathBuf) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    
    fn visit_dir(dir: &PathBuf, files: &mut Vec<PathBuf>) -> Result<()> {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                visit_dir(&path, files)?;
            } else if path.extension().and_then(|s| s.to_str()) == Some("csd") {
                files.push(path);
            }
        }
        Ok(())
    }
    
    visit_dir(root, &mut files)?;
    Ok(files)
}

fn analyze_project_characteristics(source_files: &[PathBuf]) -> Result<cursed::optimization::performance_integration::ProjectCharacteristics> {
    let mut total_lines = 0;
    let mut total_bytes = 0;
    let mut has_heavy_computation = false;
    let mut has_many_generics = false;
    
    for file_path in source_files {
        if let Ok(content) = std::fs::read_to_string(file_path) {
            total_lines += content.lines().count();
            total_bytes += content.len();
            
            // Heuristics for complexity analysis
            if content.contains("lowkey") && content.matches("lowkey").count() > 10 {
                has_heavy_computation = true;
            }
            if content.contains("<") && content.matches("<").count() > 20 {
                has_many_generics = true;
            }
        }
    }
    
    let average_file_size = if source_files.len() > 0 {
        total_bytes / source_files.len()
    } else {
        0
    };
    
    Ok(cursed::optimization::performance_integration::ProjectCharacteristics {
        total_source_files: source_files.len(),
        total_lines_of_code: total_lines,
        average_file_size,
        dependency_count: 0, // Would need proper dependency analysis
        has_heavy_computation,
        has_many_generics,
        typical_build_time_seconds: estimate_build_time(total_lines),
        // Add missing fields
        total_loc: total_lines,
        function_count: total_lines / 10, // Rough estimate
        module_count: source_files.len(),
        complexity_score: (total_lines as f64 / 1000.0).min(10.0),
        size_category: cursed::optimization::performance_integration::ProjectSize::Medium,
        optimization_patterns: Vec::new(),
        recommended_level: cursed::optimization::config::OptimizationLevel::Default,
        estimated_build_time: std::time::Duration::from_secs_f64(estimate_build_time(total_lines)),
        memory_usage_estimate: total_bytes,
    })
}

fn estimate_build_time(lines_of_code: usize) -> f64 {
    // Simple heuristic: ~1000 lines per second compilation
    (lines_of_code as f64 / 1000.0).max(1.0)
}

fn print_characteristics_table(characteristics: &cursed::optimization::performance_integration::ProjectCharacteristics) {
    println!("\n📊 Project Characteristics:");
    println!("  📁 Source files: {}", characteristics.total_source_files);
    println!("  📝 Lines of code: {}", characteristics.total_lines_of_code);
    println!("  📏 Average file size: {} bytes", characteristics.average_file_size);
    println!("  🔗 Dependencies: {}", characteristics.dependency_count);
    println!("  🧮 Heavy computation: {}", if characteristics.has_heavy_computation { "Yes" } else { "No" });
    println!("  🔧 Many generics: {}", if characteristics.has_many_generics { "Yes" } else { "No" });
    println!("  ⏱️  Estimated build time: {:.1}s", characteristics.typical_build_time_seconds);
}

fn print_characteristics_json(characteristics: &cursed::optimization::performance_integration::ProjectCharacteristics) {
    let json = serde_json::json!({
        "total_source_files": characteristics.total_source_files,
        "total_lines_of_code": characteristics.total_lines_of_code,
        "average_file_size": characteristics.average_file_size,
        "dependency_count": characteristics.dependency_count,
        "has_heavy_computation": characteristics.has_heavy_computation,
        "has_many_generics": characteristics.has_many_generics,
        "typical_build_time_seconds": characteristics.typical_build_time_seconds,
    });
    println!("{}", serde_json::to_string_pretty(&json).unwrap());
}

fn print_characteristics_yaml(_characteristics: &cursed::optimization::performance_integration::ProjectCharacteristics) {
    // Placeholder for YAML output
    println!("YAML output not implemented in this demo");
}

fn provide_project_recommendations(characteristics: &cursed::optimization::performance_integration::ProjectCharacteristics) {
    println!("\n💡 Optimization Recommendations:");
    
    if characteristics.total_source_files > 100 {
        println!("  🔀 Large project detected - Enable aggressive parallel compilation");
        println!("  💾 Consider increasing cache size to 4GB+");
    }
    
    if characteristics.has_heavy_computation {
        println!("  ⚡ Heavy computation detected - Enable profile-guided optimization");
        println!("  🎯 Use performance optimization profile for builds");
    }
    
    if characteristics.has_many_generics {
        println!("  🔧 Many generics detected - Enable incremental compilation");
        println!("  📦 Consider template specialization optimizations");
    }
    
    if characteristics.total_lines_of_code > 50000 {
        println!("  🏭 Large codebase - Enable distributed compilation if available");
        println!("  📊 Regular performance monitoring recommended");
    }
    
    if characteristics.typical_build_time_seconds > 60.0 {
        println!("  ⏰ Long build times - Focus on compilation speed optimizations");
        println!("  🔄 Enable adaptive optimization to balance speed vs performance");
    }
}
