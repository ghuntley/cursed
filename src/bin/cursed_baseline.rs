//! CURSED Performance Baseline Management CLI Tool
//! 
//! Command-line interface for managing performance baselines, running regression
//! analysis, and monitoring performance trends in the CURSED compiler.

use clap::{Parser, Subcommand};
use cursed::optimization::{
    BaselineStorage, BaselineStorageConfig, BaselineType,
    RegressionAnalyzer, RegressionAnalysisConfig,
    BenchmarkRunner, BenchmarkConfig, create_default_benchmarks,
    PerformanceThresholds,
};
use cursed::error::Result;
use std::path::PathBuf;
use std::process;

/// CURSED Performance Baseline Management Tool
#[derive(Parser)]
#[command(name = "cursed-baseline")]
#[command(about = "Manage performance baselines and regression analysis for CURSED compiler")]
#[command(version = "1.0.0")]
#[command(long_about = None)]
struct Cli {
    /// Baseline storage directory
    #[arg(long, default_value = ".cursed/baselines")]
    storage_dir: PathBuf,
    
    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
    
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all available baselines
    List {
        /// Show detailed information
        #[arg(short, long)]
        detailed: bool,
    },
    /// Create a new baseline
    Create {
        /// Baseline name
        name: String,
        /// Baseline type
        #[arg(short, long, value_enum, default_value = "manual")]
        baseline_type: BaselineTypeArg,
        /// Git commit hash
        #[arg(long)]
        commit: Option<String>,
        /// Version tag
        #[arg(long)]
        version: Option<String>,
        /// Compiler path
        #[arg(long, default_value = "cursed")]
        compiler: PathBuf,
        /// Work directory for benchmarks
        #[arg(long, default_value = ".cursed/benchmark_work")]
        work_dir: PathBuf,
    },
    /// Set default baseline for comparisons
    SetDefault {
        /// Baseline ID to set as default
        baseline_id: String,
    },
    /// Run regression analysis
    Analyze {
        /// Baseline ID to compare against (uses default if not specified)
        #[arg(long)]
        baseline: Option<String>,
        /// Compiler path
        #[arg(long, default_value = "cursed")]
        compiler: PathBuf,
        /// Work directory for benchmarks
        #[arg(long, default_value = ".cursed/benchmark_work")]
        work_dir: PathBuf,
        /// Output format
        #[arg(short, long, value_enum, default_value = "text")]
        format: OutputFormat,
        /// Output file (stdout if not specified)
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Maximum compile time increase threshold (%)
        #[arg(long, default_value = "50.0")]
        max_compile_time_increase: f64,
        /// Minimum runtime improvement threshold (%)
        #[arg(long, default_value = "10.0")]
        min_runtime_improvement: f64,
        /// Maximum binary size increase threshold (%)
        #[arg(long, default_value = "20.0")]
        max_size_increase: f64,
        /// Maximum memory usage increase threshold (%)
        #[arg(long, default_value = "30.0")]
        max_memory_increase: f64,
    },
    /// Export baselines to file
    Export {
        /// Export file path
        output: PathBuf,
        /// Specific baseline IDs to export (all if not specified)
        #[arg(long)]
        baselines: Option<Vec<String>>,
    },
    /// Import baselines from file
    Import {
        /// Import file path
        input: PathBuf,
        /// Overwrite existing baselines
        #[arg(long)]
        overwrite: bool,
    },
    /// Delete a baseline
    Delete {
        /// Baseline ID to delete
        baseline_id: String,
        /// Force deletion without confirmation
        #[arg(short, long)]
        force: bool,
    },
    /// Show baseline information
    Show {
        /// Baseline ID to show
        baseline_id: String,
        /// Show detailed benchmark information
        #[arg(short, long)]
        detailed: bool,
    },
    /// Clean up old baselines
    Cleanup {
        /// Maximum number of baselines to keep
        #[arg(long, default_value = "20")]
        max_baselines: usize,
        /// Dry run - show what would be deleted
        #[arg(long)]
        dry_run: bool,
    },
}

#[derive(clap::ValueEnum, Clone)]
enum BaselineTypeArg {
    Manual,
    Release,
    Development,
    CI,
    GitCommit,
}

impl From<BaselineTypeArg> for BaselineType {
    fn from(arg: BaselineTypeArg) -> Self {
        match arg {
            BaselineTypeArg::Manual => BaselineType::Manual,
            BaselineTypeArg::Release => BaselineType::Release,
            BaselineTypeArg::Development => BaselineType::Development,
            BaselineTypeArg::CI => BaselineType::CI,
            BaselineTypeArg::GitCommit => BaselineType::GitCommit,
        }
    }
}

#[derive(clap::ValueEnum, Clone)]
enum OutputFormat {
    Text,
    Json,
    Markdown,
}

fn main() {
    let cli = Cli::parse();
    
    if let Err(e) = run_cli(&cli) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run_cli(cli: &Cli) -> Result<()> {
    let storage_config = BaselineStorageConfig {
        storage_dir: cli.storage_dir.clone(),
        max_baselines: 50,
        auto_cleanup: true,
        min_confidence_level: 0.7,
    };

    match &cli.command {
        Commands::List { detailed } => {
            let storage = BaselineStorage::new(storage_config)?;
            list_baselines(&storage, *detailed)
        },
        Commands::Create { name, baseline_type, commit, version, compiler, work_dir } => {
            let mut storage = BaselineStorage::new(storage_config)?;
            create_baseline(&mut storage, name, baseline_type.clone().into(), commit, version, compiler, work_dir)
        },
        Commands::SetDefault { baseline_id } => {
            let mut storage = BaselineStorage::new(storage_config)?;
            storage.set_default_baseline(baseline_id.clone())?;
            println!("✓ Set default baseline to: {}", baseline_id);
            Ok(())
        },
        Commands::Analyze { baseline, compiler, work_dir, format, output, 
                           max_compile_time_increase, min_runtime_improvement, 
                           max_size_increase, max_memory_increase } => {
            let storage = BaselineStorage::new(storage_config)?;
            let thresholds = PerformanceThresholds {
                max_compile_time_increase: *max_compile_time_increase,
                min_runtime_improvement: *min_runtime_improvement,
                max_size_increase: *max_size_increase,
                max_memory_increase: *max_memory_increase,
            };
            run_regression_analysis(&storage, baseline, compiler, work_dir, format, output, &thresholds)
        },
        Commands::Export { output, baselines } => {
            let storage = BaselineStorage::new(storage_config)?;
            storage.export_baselines(output, baselines.clone())?;
            println!("✓ Exported baselines to: {}", output.display());
            Ok(())
        },
        Commands::Import { input, overwrite } => {
            let mut storage = BaselineStorage::new(storage_config)?;
            let count = storage.import_baselines(input, *overwrite)?;
            println!("✓ Imported {} baselines from: {}", count, input.display());
            Ok(())
        },
        Commands::Delete { baseline_id, force } => {
            delete_baseline(&storage_config, baseline_id, *force)
        },
        Commands::Show { baseline_id, detailed } => {
            let mut storage = BaselineStorage::new(storage_config)?;
            show_baseline(&mut storage, baseline_id, *detailed)
        },
        Commands::Cleanup { max_baselines, dry_run } => {
            cleanup_baselines(&storage_config, *max_baselines, *dry_run)
        },
    }
}

fn list_baselines(storage: &BaselineStorage, detailed: bool) -> Result<()> {
    let baselines = storage.list_baselines();
    
    if baselines.is_empty() {
        println!("No baselines found.");
        return Ok(());
    }

    println!("Available baselines ({}):", baselines.len());
    println!();

    let default_baseline = storage.get_default_baseline();
    let default_id = default_baseline.map(|b| &b.baseline_id);

    for baseline in baselines {
        let is_default = default_id == Some(&baseline.baseline_id);
        let default_marker = if is_default { " (default)" } else { "" };
        
        println!("📊 {}{}", baseline.name, default_marker);
        println!("   ID: {}", baseline.baseline_id);
        println!("   Type: {:?}", baseline.baseline_type);
        println!("   Created: {}", baseline.created_at.format("%Y-%m-%d %H:%M:%S UTC"));
        
        if let Some(ref commit) = baseline.git_commit {
            println!("   Git commit: {}", commit);
        }
        
        if let Some(ref version) = baseline.version {
            println!("   Version: {}", version);
        }
        
        println!("   Benchmarks: {}", baseline.benchmarks.len());
        println!("   Confidence: {:.1}%", baseline.confidence_level * 100.0);

        if detailed {
            println!("   Benchmark details:");
            for (name, benchmark) in &baseline.benchmarks {
                println!("     • {}: {:.3}s compile, {} bytes binary, {} bytes memory",
                         name,
                         benchmark.compile_time_metrics.mean.as_secs_f64(),
                         benchmark.binary_size,
                         benchmark.peak_memory_usage);
            }
        }
        
        println!();
    }

    Ok(())
}

fn create_baseline(
    storage: &mut BaselineStorage,
    name: &str,
    baseline_type: BaselineType,
    commit: &Option<String>,
    version: &Option<String>,
    compiler: &PathBuf,
    work_dir: &PathBuf,
) -> Result<()> {
    println!("Creating baseline '{}'...", name);
    
    // Create benchmark runner
    let storage_config = BaselineStorageConfig {
        storage_dir: storage.storage_dir.clone(),
        ..Default::default()
    };
    
    let mut runner = BenchmarkRunner::new(compiler.clone(), work_dir.clone())
        .with_baseline_storage(storage_config)?;
    
    println!("Running benchmarks...");
    
    // Run default benchmarks
    let configs = create_default_benchmarks();
    let suite_result = runner.run_benchmark_suite("baseline_creation", &configs).await?;
    
    println!("✓ Completed {} benchmarks", suite_result.results.len());
    
    // Create baseline
    let baseline_id = storage.create_baseline(
        name.to_string(),
        baseline_type,
        &suite_result,
        commit.clone(),
        version.clone(),
    )?;
    
    println!("✓ Created baseline: {}", baseline_id);
    println!("  Average compile time: {:?}", suite_result.statistics.avg_compile_time);
    println!("  Success rate: {}/{}", 
             suite_result.statistics.successful_benchmarks,
             suite_result.statistics.total_benchmarks);
    
    Ok(())
}

fn run_regression_analysis(
    storage: &BaselineStorage,
    baseline_id: &Option<String>,
    compiler: &PathBuf,
    work_dir: &PathBuf,
    format: &OutputFormat,
    output: &Option<PathBuf>,
    thresholds: &PerformanceThresholds,
) -> Result<()> {
    println!("Running regression analysis...");
    
    // Get baseline for comparison
    let baseline = if let Some(id) = baseline_id {
        let mut storage_mut = BaselineStorage::new(BaselineStorageConfig {
            storage_dir: storage.storage_dir.clone(),
            ..Default::default()
        })?;
        storage_mut.load_baseline(id)?
            .ok_or_else(|| cursed::error::Error::General(format!("Baseline not found: {}", id)))?
    } else {
        storage.get_default_baseline()
            .ok_or_else(|| cursed::error::Error::General("No default baseline available".to_string()))?
    };
    
    println!("✓ Using baseline: {} ({})", baseline.name, baseline.baseline_id);
    
    // Run current benchmarks
    let storage_config = BaselineStorageConfig {
        storage_dir: storage.storage_dir.clone(),
        ..Default::default()
    };
    
    let mut runner = BenchmarkRunner::new(compiler.clone(), work_dir.clone())
        .with_baseline_storage(storage_config)?;
    
    let configs = create_default_benchmarks();
    let current_results = runner.run_benchmark_suite("regression_analysis", &configs).await?;
    
    println!("✓ Completed current benchmarks");
    
    // Run regression analysis
    let regression_config = RegressionAnalysisConfig {
        thresholds: thresholds.clone(),
        confidence_level: 0.95,
        min_sample_size: 1,
        enable_trend_analysis: true,
        severity_mode: cursed::optimization::regression_analyzer::SeverityCalculationMode::Adaptive,
    };
    
    let mut analyzer = RegressionAnalyzer::new(regression_config);
    let analysis = analyzer.analyze_regressions(&current_results.results, Some(baseline))?;
    
    // Generate output
    let output_text = match format {
        OutputFormat::Text => generate_text_report(&analysis, &current_results, baseline),
        OutputFormat::Json => generate_json_report(&analysis, &current_results)?,
        OutputFormat::Markdown => generate_markdown_report(&analysis, &current_results, baseline),
    };
    
    // Write output
    if let Some(output_path) = output {
        std::fs::write(output_path, &output_text)?;
        println!("✓ Report written to: {}", output_path.display());
    } else {
        println!("{}", output_text);
    }
    
    // Exit with error code if regressions found
    if analysis.has_critical_regressions {
        process::exit(2); // Critical regressions
    } else if !analysis.regressions.is_empty() {
        process::exit(1); // Minor regressions
    }
    
    Ok(())
}

fn show_baseline(storage: &mut BaselineStorage, baseline_id: &str, detailed: bool) -> Result<()> {
    let baseline = storage.load_baseline(baseline_id)?
        .ok_or_else(|| cursed::error::Error::General(format!("Baseline not found: {}", baseline_id)))?;
    
    println!("Baseline: {}", baseline.name);
    println!("=================={}", "=".repeat(baseline.name.len()));
    println!();
    println!("ID: {}", baseline.baseline_id);
    println!("Type: {:?}", baseline.baseline_type);
    println!("Created: {}", baseline.created_at.format("%Y-%m-%d %H:%M:%S UTC"));
    println!("Confidence Level: {:.1}%", baseline.confidence_level * 100.0);
    
    if let Some(ref commit) = baseline.git_commit {
        println!("Git Commit: {}", commit);
    }
    
    if let Some(ref version) = baseline.version {
        println!("Version: {}", version);
    }
    
    println!();
    println!("Metadata:");
    for (key, value) in &baseline.metadata {
        println!("  {}: {}", key, value);
    }
    
    println!();
    println!("Benchmarks ({}):", baseline.benchmarks.len());
    
    for (name, benchmark) in &baseline.benchmarks {
        println!("  📊 {}", name);
        println!("     Compile Time: {:.3}s (±{:.3}s)", 
                 benchmark.compile_time_metrics.mean.as_secs_f64(),
                 benchmark.compile_time_metrics.std_dev.as_secs_f64());
        
        if let Some(ref runtime) = benchmark.runtime_metrics {
            println!("     Runtime: {:.3}s (±{:.3}s)",
                     runtime.mean.as_secs_f64(),
                     runtime.std_dev.as_secs_f64());
        }
        
        println!("     Binary Size: {} bytes", benchmark.binary_size);
        println!("     Memory Usage: {} bytes", benchmark.peak_memory_usage);
        println!("     Optimization Passes: {}", benchmark.optimization_passes);
        
        if detailed && !benchmark.custom_metrics.is_empty() {
            println!("     Custom Metrics:");
            for (metric, value) in &benchmark.custom_metrics {
                println!("       {}: {:.3}", metric, value);
            }
        }
        
        println!();
    }
    
    Ok(())
}

fn delete_baseline(storage_config: &BaselineStorageConfig, baseline_id: &str, force: bool) -> Result<()> {
    let mut storage = BaselineStorage::new(storage_config.clone())?;
    
    // Check if baseline exists
    let baseline = storage.load_baseline(baseline_id)?
        .ok_or_else(|| cursed::error::Error::General(format!("Baseline not found: {}", baseline_id)))?;
    
    if !force {
        println!("Are you sure you want to delete baseline '{}'? (y/N)", baseline.name);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        
        if !input.trim().to_lowercase().starts_with('y') {
            println!("Deletion cancelled.");
            return Ok(());
        }
    }
    
    // Remove baseline file
    let baseline_path = storage_config.storage_dir.join(format!("{}.json", baseline_id));
    if baseline_path.exists() {
        std::fs::remove_file(&baseline_path)?;
        println!("✓ Deleted baseline: {}", baseline.name);
    } else {
        println!("Baseline file not found: {}", baseline_path.display());
    }
    
    Ok(())
}

fn cleanup_baselines(storage_config: &BaselineStorageConfig, max_baselines: usize, dry_run: bool) -> Result<()> {
    let storage = BaselineStorage::new(storage_config.clone())?;
    let baselines = storage.list_baselines();
    
    if baselines.len() <= max_baselines {
        println!("No cleanup needed. Current: {} baselines, Limit: {}", baselines.len(), max_baselines);
        return Ok(());
    }
    
    // Sort by creation time, keeping the most recent
    let mut sorted_baselines = baselines.clone();
    sorted_baselines.sort_by_key(|b| b.created_at);
    
    let to_remove = sorted_baselines.len() - max_baselines;
    let candidates: Vec<_> = sorted_baselines.iter()
        .take(to_remove)
        .filter(|b| b.baseline_type != BaselineType::Release) // Don't remove release baselines
        .collect();
    
    if candidates.is_empty() {
        println!("No baselines can be safely removed (release baselines are protected)");
        return Ok(());
    }
    
    if dry_run {
        println!("Would remove {} baselines:", candidates.len());
        for baseline in &candidates {
            println!("  - {} ({})", baseline.name, baseline.baseline_id);
        }
    } else {
        println!("Removing {} old baselines...", candidates.len());
        for baseline in &candidates {
            let baseline_path = storage_config.storage_dir.join(format!("{}.json", baseline.baseline_id));
            if baseline_path.exists() {
                std::fs::remove_file(&baseline_path)?;
                println!("  ✓ Removed: {}", baseline.name);
            }
        }
    }
    
    Ok(())
}

fn generate_text_report(
    analysis: &cursed::optimization::regression_analyzer::DetailedRegressionAnalysis,
    current_results: &cursed::optimization::BenchmarkSuiteResult,
    baseline: &cursed::optimization::PerformanceBaseline,
) -> String {
    let mut report = String::new();
    
    report.push_str("CURSED Performance Regression Analysis Report\n");
    report.push_str("=============================================\n\n");
    
    report.push_str(&format!("Baseline: {} ({})\n", baseline.name, baseline.baseline_id));
    report.push_str(&format!("Current Suite: {}\n", current_results.suite_name));
    report.push_str(&format!("Analysis Date: {}\n\n", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));
    
    // Summary
    if analysis.has_critical_regressions {
        report.push_str("🚨 CRITICAL REGRESSIONS DETECTED!\n\n");
    } else if !analysis.regressions.is_empty() {
        report.push_str("⚠️  Performance regressions detected\n\n");
    } else {
        report.push_str("✅ No performance regressions detected\n\n");
    }
    
    // Regressions
    if !analysis.regressions.is_empty() {
        report.push_str(&format!("Detected Regressions ({}):\n", analysis.regressions.len()));
        for regression in &analysis.regressions {
            let severity_icon = match regression.severity {
                cursed::optimization::RegressionSeverity::Critical => "🔴",
                cursed::optimization::RegressionSeverity::Major => "🟠",
                cursed::optimization::RegressionSeverity::Minor => "🟡",
                cursed::optimization::RegressionSeverity::Warning => "🔵",
            };
            
            report.push_str(&format!("  {} {:?} in {}: {:.1}% (threshold: {:.1}%)\n",
                                   severity_icon,
                                   regression.regression_type,
                                   regression.benchmark_name,
                                   regression.actual_value,
                                   regression.expected_value));
            report.push_str(&format!("    └─ {}\n", regression.description));
        }
        report.push('\n');
    }
    
    // Recommendations
    if !analysis.recommendations.is_empty() {
        report.push_str(&format!("Recommendations ({}):\n", analysis.recommendations.len()));
        for (i, rec) in analysis.recommendations.iter().enumerate() {
            report.push_str(&format!("  {}. Priority {}: {}\n",
                                   i + 1,
                                   rec.priority,
                                   rec.recommendation));
        }
    }
    
    report
}

fn generate_json_report(
    analysis: &cursed::optimization::regression_analyzer::DetailedRegressionAnalysis,
    current_results: &cursed::optimization::BenchmarkSuiteResult,
) -> Result<String> {
    let json_data = serde_json::json!({
        "analysis_date": chrono::Utc::now().to_rfc3339(),
        "current_suite": current_results.suite_name,
        "has_critical_regressions": analysis.has_critical_regressions,
        "regressions": analysis.regressions,
        "recommendations": analysis.recommendations,
        "statistical_analysis": analysis.statistical_analysis,
        "baseline_comparison": analysis.baseline_comparison,
    });
    
    Ok(serde_json::to_string_pretty(&json_data)?)
}

fn generate_markdown_report(
    analysis: &cursed::optimization::regression_analyzer::DetailedRegressionAnalysis,
    current_results: &cursed::optimization::BenchmarkSuiteResult,
    baseline: &cursed::optimization::PerformanceBaseline,
) -> String {
    let mut report = String::new();
    
    report.push_str("# CURSED Performance Regression Analysis Report\n\n");
    
    report.push_str(&format!("**Baseline:** {} ({})\n", baseline.name, baseline.baseline_id));
    report.push_str(&format!("**Current Suite:** {}\n", current_results.suite_name));
    report.push_str(&format!("**Analysis Date:** {}\n\n", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));
    
    // Summary
    report.push_str("## Summary\n\n");
    if analysis.has_critical_regressions {
        report.push_str("🚨 **CRITICAL REGRESSIONS DETECTED!**\n\n");
    } else if !analysis.regressions.is_empty() {
        report.push_str("⚠️  **Performance regressions detected**\n\n");
    } else {
        report.push_str("✅ **No performance regressions detected**\n\n");
    }
    
    // Regressions table
    if !analysis.regressions.is_empty() {
        report.push_str("## Detected Regressions\n\n");
        report.push_str("| Severity | Type | Benchmark | Actual | Threshold | Description |\n");
        report.push_str("|----------|------|-----------|--------|-----------|-------------|\n");
        
        for regression in &analysis.regressions {
            let severity_icon = match regression.severity {
                cursed::optimization::RegressionSeverity::Critical => "🔴",
                cursed::optimization::RegressionSeverity::Major => "🟠",
                cursed::optimization::RegressionSeverity::Minor => "🟡",
                cursed::optimization::RegressionSeverity::Warning => "🔵",
            };
            
            report.push_str(&format!("| {} | {:?} | {} | {:.1}% | {:.1}% | {} |\n",
                                   severity_icon,
                                   regression.regression_type,
                                   regression.benchmark_name,
                                   regression.actual_value,
                                   regression.expected_value,
                                   regression.description));
        }
        report.push_str("\n");
    }
    
    // Recommendations
    if !analysis.recommendations.is_empty() {
        report.push_str("## Recommendations\n\n");
        for (i, rec) in analysis.recommendations.iter().enumerate() {
            report.push_str(&format!("{}. **Priority {}:** {}\n",
                                   i + 1,
                                   rec.priority,
                                   rec.recommendation));
            report.push_str(&format!("   - Effort: {:?}\n", rec.estimated_effort));
            report.push_str(&format!("   - Expected Impact: {:?}\n\n", rec.expected_impact));
        }
    }
    
    report
}
