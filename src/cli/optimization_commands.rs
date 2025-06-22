//! Optimization CLI Commands
//! 
//! Command-line interface for performance optimization and compilation speed analysis.
//! Provides tools for analyzing, benchmarking, profiling, and configuring optimization settings.

use clap::{Arg, ArgAction, ArgMatches, Command};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use tokio::time::timeout;

use crate::optimization::{
    OptimizationPass, OptimizationConfig, OptimizationEngine,
    analysis::{PerformanceAnalyzer, CompilationProfiler, BenchmarkRunner},
    utils::{OptimizationRecommendations, PerformanceReport},
};
use crate::profiling::performance::{PerformanceMonitor, CompilationPhase, ReportFormat, ReportConfig};
use crate::common::optimization_level::OptimizationLevel;
use crate::core::performance_pipeline::PerformancePipeline;
use crate::error::CursedError;

/// Configuration for optimization settings that can be persisted
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationCliConfig {
    /// Default optimization level
    pub default_level: OptimizationLevel,
    /// Enabled optimization passes
    pub enabled_passes: Vec<String>,
    /// Disabled optimization passes
    pub disabled_passes: Vec<String>,
    /// Custom optimization parameters
    pub custom_params: HashMap<String, String>,
    /// Benchmark configuration
    pub benchmark_config: BenchmarkConfig,
    /// Profiling configuration
    pub profiling_config: ProfilingConfig,
    /// Project profiles
    pub profiles: HashMap<String, ProjectProfile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkConfig {
    /// Number of iterations per optimization level
    pub iterations: usize,
    /// Timeout for each compilation in seconds
    pub timeout_seconds: u64,
    /// Warm-up iterations before measurement
    pub warmup_iterations: usize,
    /// Test files to use for benchmarking
    pub test_files: Vec<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilingConfig {
    /// Enable detailed phase timing
    pub detailed_timing: bool,
    /// Enable memory usage tracking
    pub memory_tracking: bool,
    /// Sample rate for profiling
    pub sample_rate: u64,
    /// Output format for profiling reports
    pub output_format: String,
}

/// Project optimization profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectProfile {
    /// Profile name
    pub name: String,
    /// Profile description
    pub description: String,
    /// Optimization level
    pub optimization_level: OptimizationLevel,
    /// Specific passes for this profile
    pub enabled_passes: Vec<String>,
    /// Disabled passes for this profile
    pub disabled_passes: Vec<String>,
    /// Profile-specific parameters
    pub parameters: HashMap<String, String>,
    /// Build configuration
    pub build_config: BuildConfig,
}

/// Build configuration for project profiles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    /// Parallel compilation enabled
    pub parallel: bool,
    /// Number of parallel jobs
    pub jobs: Option<usize>,
    /// Incremental compilation enabled
    pub incremental: bool,
    /// LTO enabled
    pub lto: bool,
    /// Target CPU
    pub target_cpu: Option<String>,
    /// Target features
    pub target_features: Vec<String>,
}

impl Default for OptimizationCliConfig {
    fn default() -> Self {
        let mut profiles = HashMap::new();
        
        // Add built-in profiles
        profiles.insert("web".to_string(), ProjectProfile::web());
        profiles.insert("systems".to_string(), ProjectProfile::systems());
        profiles.insert("ml".to_string(), ProjectProfile::ml());
        profiles.insert("game".to_string(), ProjectProfile::game());
        profiles.insert("release".to_string(), ProjectProfile::release());
        profiles.insert("dev".to_string(), ProjectProfile::dev());
        
        Self {
            default_level: OptimizationLevel::O3, // Changed to O3 for aggressive optimization by default
            enabled_passes: vec![
                "inline".to_string(),
                "aggressive-inline".to_string(), // Added aggressive inlining
                "dce".to_string(),
                "mem2reg".to_string(),
                "gvn".to_string(),
                "loop-unroll".to_string(), // Added loop unrolling
                "vectorize".to_string(), // Added vectorization
                "slp-vectorize".to_string(), // Added SLP vectorization
                "math-optimize".to_string(), // Added math optimization
                "pgo-optimize".to_string(), // Added PGO optimization
            ],
            disabled_passes: vec![],
            custom_params: {
                let mut params = HashMap::new();
                params.insert("lto".to_string(), "true".to_string()); // Enable LTO by default
                params.insert("parallel".to_string(), "true".to_string());
                params.insert("target-cpu".to_string(), "native".to_string()); // Use native CPU
                params.insert("pgo-path".to_string(), "target/pgo-data".to_string()); // Default PGO path
                params.insert("enhanced-passes".to_string(), "true".to_string()); // Enable enhanced passes
                params
            },
            benchmark_config: BenchmarkConfig {
                iterations: 10, // Increased for better accuracy
                timeout_seconds: 600, // Increased timeout for aggressive optimization
                warmup_iterations: 3, // Increased warmup
                test_files: vec![],
            },
            profiling_config: ProfilingConfig {
                detailed_timing: true,
                memory_tracking: true, // Enable memory tracking by default
                sample_rate: 1000,
                output_format: "markdown".to_string(),
            },
            profiles,
        }
    }
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            parallel: true,
            jobs: None, // Auto-detect
            incremental: true,
            lto: false,
            target_cpu: None,
            target_features: vec![],
        }
    }
}

impl ProjectProfile {
    /// Web application profile - optimized for startup time and size
    pub fn web() -> Self {
        Self {
            name: "web".to_string(),
            description: "Optimized for web applications - fast startup, small size".to_string(),
            optimization_level: OptimizationLevel::Oz, // Size optimization
            enabled_passes: vec![
                "inline".to_string(),
                "dce".to_string(),
                "mem2reg".to_string(),
                "gvn".to_string(),
                "strip-debug".to_string(),
                "minify".to_string(),
            ],
            disabled_passes: vec!["aggressive-inline".to_string()],
            parameters: {
                let mut params = HashMap::new();
                params.insert("inline-threshold".to_string(), "50".to_string());
                params.insert("size-priority".to_string(), "true".to_string());
                params
            },
            build_config: BuildConfig {
                parallel: true,
                jobs: Some(4),
                incremental: true,
                lto: true,
                target_cpu: None,
                target_features: vec!["wasm".to_string()],
            },
        }
    }
    
    /// Systems programming profile - optimized for performance
    pub fn systems() -> Self {
        Self {
            name: "systems".to_string(),
            description: "Optimized for systems programming - maximum performance".to_string(),
            optimization_level: OptimizationLevel::O3,
            enabled_passes: vec![
                "inline".to_string(),
                "aggressive-inline".to_string(),
                "dce".to_string(),
                "mem2reg".to_string(),
                "gvn".to_string(),
                "loop-unroll".to_string(),
                "vectorize".to_string(),
            ],
            disabled_passes: vec![],
            parameters: {
                let mut params = HashMap::new();
                params.insert("inline-threshold".to_string(), "1000".to_string());
                params.insert("performance-priority".to_string(), "true".to_string());
                params
            },
            build_config: BuildConfig {
                parallel: true,
                jobs: None,
                incremental: false,
                lto: true,
                target_cpu: Some("native".to_string()),
                target_features: vec!["avx2".to_string(), "fma".to_string()],
            },
        }
    }
    
    /// Machine learning profile - optimized for math operations
    pub fn ml() -> Self {
        Self {
            name: "ml".to_string(),
            description: "Optimized for machine learning - vectorization and math".to_string(),
            optimization_level: OptimizationLevel::O3,
            enabled_passes: vec![
                "inline".to_string(),
                "dce".to_string(),
                "mem2reg".to_string(),
                "gvn".to_string(),
                "vectorize".to_string(),
                "slp-vectorize".to_string(),
                "math-optimize".to_string(),
            ],
            disabled_passes: vec![],
            parameters: {
                let mut params = HashMap::new();
                params.insert("vectorize-threshold".to_string(), "2".to_string());
                params.insert("math-fast".to_string(), "true".to_string());
                params
            },
            build_config: BuildConfig {
                parallel: true,
                jobs: None,
                incremental: true,
                lto: false, // Faster iteration
                target_cpu: Some("native".to_string()),
                target_features: vec![
                    "avx2".to_string(), 
                    "fma".to_string(), 
                    "avx512f".to_string()
                ],
            },
        }
    }
    
    /// Game development profile - balanced performance and compile time
    pub fn game() -> Self {
        Self {
            name: "game".to_string(),
            description: "Optimized for game development - balanced performance".to_string(),
            optimization_level: OptimizationLevel::O2,
            enabled_passes: vec![
                "inline".to_string(),
                "dce".to_string(),
                "mem2reg".to_string(),
                "gvn".to_string(),
                "loop-unroll".to_string(),
            ],
            disabled_passes: vec!["aggressive-inline".to_string()],
            parameters: {
                let mut params = HashMap::new();
                params.insert("inline-threshold".to_string(), "200".to_string());
                params.insert("balanced-mode".to_string(), "true".to_string());
                params
            },
            build_config: BuildConfig {
                parallel: true,
                jobs: Some(8),
                incremental: true,
                lto: false,
                target_cpu: Some("x86-64-v3".to_string()),
                target_features: vec!["sse4.2".to_string(), "popcnt".to_string()],
            },
        }
    }
    
    /// Release profile - maximum performance optimization (new default)
    pub fn release() -> Self {
        Self {
            name: "release".to_string(),
            description: "Maximum performance optimization - aggressive settings for production".to_string(),
            optimization_level: OptimizationLevel::O3,
            enabled_passes: vec![
                "inline".to_string(),
                "aggressive-inline".to_string(),
                "dce".to_string(),
                "mem2reg".to_string(),
                "gvn".to_string(),
                "loop-unroll".to_string(),
                "vectorize".to_string(),
                "slp-vectorize".to_string(),
                "math-optimize".to_string(),
                "pgo-optimize".to_string(),
                "interprocedural".to_string(),
            ],
            disabled_passes: vec![],
            parameters: {
                let mut params = HashMap::new();
                params.insert("inline-threshold".to_string(), "1000".to_string());
                params.insert("performance-priority".to_string(), "true".to_string());
                params.insert("pgo-enabled".to_string(), "true".to_string());
                params.insert("lto-mode".to_string(), "fat".to_string());
                params
            },
            build_config: BuildConfig {
                parallel: true,
                jobs: None,
                incremental: false, // Disable for maximum optimization
                lto: true,
                target_cpu: Some("native".to_string()),
                target_features: vec![
                    "sse4.2".to_string(), 
                    "popcnt".to_string(),
                    "avx".to_string(),
                    "avx2".to_string(),
                    "fma".to_string()
                ],
            },
        }
    }
    
    /// Development profile - fast compilation for debugging
    pub fn dev() -> Self {
        Self {
            name: "dev".to_string(),
            description: "Fast compilation for development - minimal optimization".to_string(),
            optimization_level: OptimizationLevel::O1,
            enabled_passes: vec![
                "mem2reg".to_string(),
                "dce".to_string(),
                "gvn".to_string(),
            ],
            disabled_passes: vec![
                "aggressive-inline".to_string(),
                "loop-unroll".to_string(),
                "vectorize".to_string(),
                "pgo-optimize".to_string(),
            ],
            parameters: {
                let mut params = HashMap::new();
                params.insert("inline-threshold".to_string(), "25".to_string());
                params.insert("debug-mode".to_string(), "true".to_string());
                params.insert("fast-compile".to_string(), "true".to_string());
                params
            },
            build_config: BuildConfig {
                parallel: true,
                jobs: Some(4),
                incremental: true, // Enable for faster builds
                lto: false, // Disable for faster linking
                target_cpu: None,
                target_features: vec![],
            },
        }
    }
}

/// Add optimization commands to the CLI
pub fn add_optimization_commands(cmd: Command) -> Command {
    cmd.subcommand(
        Command::new("analyze")
            .about("Analyze compilation performance and suggest optimizations")
            .arg(
                Arg::new("file")
                    .help("CURSED source file to analyze")
                    .required(true)
                    .value_name("FILE")
            )
            .arg(
                Arg::new("output")
                    .short('o')
                    .long("output")
                    .value_name("FILE")
                    .help("Output file for analysis report")
            )
            .arg(
                Arg::new("format")
                    .short('f')
                    .long("format")
                    .value_name("FORMAT")
                    .help("Report format: json, markdown, table")
                    .default_value("markdown")
            )
            .arg(
                Arg::new("detailed")
                    .long("detailed")
                    .action(ArgAction::SetTrue)
                    .help("Generate detailed analysis report")
            )
            .arg(
                Arg::new("suggestions")
                    .long("suggestions")
                    .action(ArgAction::SetTrue)
                    .help("Include optimization suggestions")
            )
    )
    .subcommand(
        Command::new("benchmark")
            .about("Benchmark compilation speed across different optimization levels")
            .arg(
                Arg::new("file")
                    .help("CURSED source file to benchmark")
                    .required(true)
                    .value_name("FILE")
            )
            .arg(
                Arg::new("levels")
                    .short('l')
                    .long("levels")
                    .value_name("LEVELS")
                    .help("Optimization levels to benchmark (comma-separated)")
                    .default_value("0,1,2,3,s,z")
            )
            .arg(
                Arg::new("iterations")
                    .short('i')
                    .long("iterations")
                    .value_name("N")
                    .help("Number of iterations per level")
                    .default_value("5")
            )
            .arg(
                Arg::new("warmup")
                    .short('w')
                    .long("warmup")
                    .value_name("N")
                    .help("Number of warmup iterations")
                    .default_value("2")
            )
            .arg(
                Arg::new("timeout")
                    .short('t')
                    .long("timeout")
                    .value_name("SECONDS")
                    .help("Timeout for each compilation")
                    .default_value("300")
            )
            .arg(
                Arg::new("output")
                    .short('o')
                    .long("output")
                    .value_name("FILE")
                    .help("Output file for benchmark report")
            )
            .arg(
                Arg::new("compare")
                    .long("compare")
                    .value_name("FILE")
                    .help("Compare with previous benchmark results")
            )
            .arg(
                Arg::new("parallel")
                    .short('p')
                    .long("parallel")
                    .action(ArgAction::SetTrue)
                    .help("Run benchmarks in parallel")
            )
    )
    .subcommand(
        Command::new("profile")
            .about("Profile compilation pipeline and identify bottlenecks")
            .arg(
                Arg::new("file")
                    .help("CURSED source file to profile")
                    .required(true)
                    .value_name("FILE")
            )
            .arg(
                Arg::new("opt-level")
                    .short('O')
                    .long("opt-level")
                    .value_name("LEVEL")
                    .help("Optimization level to profile")
                    .default_value("2")
            )
            .arg(
                Arg::new("phases")
                    .long("phases")
                    .action(ArgAction::SetTrue)
                    .help("Profile individual compilation phases")
            )
            .arg(
                Arg::new("memory")
                    .long("memory")
                    .action(ArgAction::SetTrue)
                    .help("Track memory usage during compilation")
            )
            .arg(
                Arg::new("sample-rate")
                    .long("sample-rate")
                    .value_name("HZ")
                    .help("Profiling sample rate in Hz")
                    .default_value("1000")
            )
            .arg(
                Arg::new("output")
                    .short('o')
                    .long("output")
                    .value_name("FILE")
                    .help("Output file for profiling report")
            )
            .arg(
                Arg::new("flamegraph")
                    .long("flamegraph")
                    .action(ArgAction::SetTrue)
                    .help("Generate flamegraph output")
            )
    )
    .subcommand(
        Command::new("enable")
            .about("Enable specific optimization passes")
            .arg(
                Arg::new("passes")
                    .help("Optimization passes to enable (comma-separated)")
                    .required(true)
                    .value_name("PASSES")
            )
            .arg(
                Arg::new("global")
                    .short('g')
                    .long("global")
                    .action(ArgAction::SetTrue)
                    .help("Apply to global configuration")
            )
            .arg(
                Arg::new("project")
                    .short('p')
                    .long("project")
                    .action(ArgAction::SetTrue)
                    .help("Apply to current project only")
            )
    )
    .subcommand(
        Command::new("disable")
            .about("Disable specific optimization passes")
            .arg(
                Arg::new("passes")
                    .help("Optimization passes to disable (comma-separated)")
                    .required(true)
                    .value_name("PASSES")
            )
            .arg(
                Arg::new("global")
                    .short('g')
                    .long("global")
                    .action(ArgAction::SetTrue)
                    .help("Apply to global configuration")
            )
            .arg(
                Arg::new("project")
                    .short('p')
                    .long("project")
                    .action(ArgAction::SetTrue)
                    .help("Apply to current project only")
            )
    )
    .subcommand(
        Command::new("config")
            .about("Configure optimization settings")
            .arg(
                Arg::new("show")
                    .long("show")
                    .action(ArgAction::SetTrue)
                    .help("Show current configuration")
            )
            .arg(
                Arg::new("set")
                    .long("set")
                    .value_name("KEY=VALUE")
                    .action(ArgAction::Append)
                    .help("Set configuration value")
            )
            .arg(
                Arg::new("unset")
                    .long("unset")
                    .value_name("KEY")
                    .action(ArgAction::Append)
                    .help("Unset configuration value")
            )
            .arg(
                Arg::new("default-level")
                    .long("default-level")
                    .value_name("LEVEL")
                    .help("Set default optimization level")
            )
            .arg(
                Arg::new("global")
                    .short('g')
                    .long("global")
                    .action(ArgAction::SetTrue)
                    .help("Modify global configuration")
            )
            .arg(
                Arg::new("export")
                    .long("export")
                    .value_name("FILE")
                    .help("Export configuration to file")
            )
            .arg(
                Arg::new("import")
                    .long("import")
                    .value_name("FILE")
                    .help("Import configuration from file")
            )
    )
    .subcommand(
    Command::new("reset")
    .about("Reset to default optimization configuration")
    .arg(
    Arg::new("global")
    .short('g')
    .long("global")
    .action(ArgAction::SetTrue)
    .help("Reset global configuration")
    )
    .arg(
    Arg::new("project")
    .short('p')
    .long("project")
    .action(ArgAction::SetTrue)
    .help("Reset project configuration")
    )
    .arg(
    Arg::new("confirm")
    .long("confirm")
    .action(ArgAction::SetTrue)
    .help("Confirm reset without prompting")
    )
    )
        .subcommand(
            Command::new("interactive")
                .about("Interactive optimization wizard")
                .arg(
                    Arg::new("file")
                        .help("CURSED source file to optimize")
                        .value_name("FILE")
                )
                .arg(
                    Arg::new("quick")
                        .short('q')
                        .long("quick")
                        .action(ArgAction::SetTrue)
                        .help("Quick optimization wizard (skip detailed questions)")
                )
                .arg(
                    Arg::new("advanced")
                        .long("advanced")
                        .action(ArgAction::SetTrue)
                        .help("Show advanced optimization options")
                )
                .arg(
                    Arg::new("enhanced-passes")
                        .long("enhanced-passes")
                        .action(ArgAction::SetTrue)
                        .help("Enable enhanced LLVM optimization passes")
                )
                .arg(
                    Arg::new("disable-enhanced-passes")
                        .long("disable-enhanced-passes")
                        .action(ArgAction::SetTrue)
                        .help("Disable enhanced LLVM optimization passes (enabled by default)")
                )
                .arg(
                    Arg::new("disable-pgo")
                        .long("disable-pgo")
                        .action(ArgAction::SetTrue)
                        .help("Disable profile-guided optimization (enabled by default)")
                )
                .arg(
                    Arg::new("disable-lto")
                        .long("disable-lto")
                        .action(ArgAction::SetTrue)
                        .help("Disable link-time optimization (enabled by default)")
                )
                .arg(
                    Arg::new("pgo-path")
                        .long("pgo-path")
                        .value_name("PATH")
                        .help("Path to profile-guided optimization data")
                        .default_value("target/pgo-data")
                )
                .arg(
                    Arg::new("dev-mode")
                        .long("dev-mode")
                        .action(ArgAction::SetTrue)
                        .help("Use development optimization profile (fast compilation)")
                )
        )
        .subcommand(
            Command::new("apply")
                .about("Apply optimization recommendations automatically")
                .arg(
                    Arg::new("file")
                        .help("CURSED source file to analyze and optimize")
                        .required(true)
                        .value_name("FILE")
                )
                .arg(
                    Arg::new("profile")
                        .short('p')
                        .long("profile")
                        .value_name("PROFILE")
                        .help("Project profile: web, systems, ml, game")
                )
                .arg(
                    Arg::new("dry-run")
                        .long("dry-run")
                        .action(ArgAction::SetTrue)
                        .help("Show what would be applied without making changes")
                )
                .arg(
                    Arg::new("aggressive")
                        .long("aggressive")
                        .action(ArgAction::SetTrue)
                        .help("Apply aggressive optimizations (may increase compile time)")
                )
                .arg(
                    Arg::new("safe")
                        .long("safe")
                        .action(ArgAction::SetTrue)
                        .help("Apply only safe optimizations")
                )
        )
        .subcommand(
            Command::new("profiles")
                .about("Manage optimization profiles")
                .arg(
                    Arg::new("list")
                        .short('l')
                        .long("list")
                        .action(ArgAction::SetTrue)
                        .help("List available profiles")
                )
                .arg(
                    Arg::new("create")
                        .long("create")
                        .value_name("NAME")
                        .help("Create new profile")
                )
                .arg(
                    Arg::new("delete")
                        .long("delete")
                        .value_name("NAME")
                        .help("Delete existing profile")
                )
                .arg(
                    Arg::new("copy")
                        .long("copy")
                        .value_name("FROM,TO")
                        .help("Copy profile (source,destination)")
                )
                .arg(
                    Arg::new("export")
                        .long("export")
                        .value_name("NAME,FILE")
                        .help("Export profile to file")
                )
                .arg(
                    Arg::new("import")
                        .long("import")
                        .value_name("FILE,NAME")
                        .help("Import profile from file")
                )
        )
}

/// Handle optimization commands
pub async fn handle_optimization_command(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    match matches.subcommand() {
        Some(("analyze", sub_matches)) => handle_analyze_command(sub_matches).await,
        Some(("benchmark", sub_matches)) => handle_benchmark_command(sub_matches).await,
        Some(("profile", sub_matches)) => handle_profile_command(sub_matches).await,
        Some(("enable", sub_matches)) => handle_enable_command(sub_matches).await,
        Some(("disable", sub_matches)) => handle_disable_command(sub_matches).await,
        Some(("config", sub_matches)) => handle_config_command(sub_matches).await,
        Some(("reset", sub_matches)) => handle_reset_command(sub_matches).await,
        Some(("interactive", sub_matches)) => handle_interactive_command(sub_matches).await,
        Some(("apply", sub_matches)) => handle_apply_command(sub_matches).await,
        Some(("profiles", sub_matches)) => handle_profiles_command(sub_matches).await,
        _ => {
            eprintln!("No optimization subcommand provided. Use --help for usage information.");
            std::process::exit(1);
        }
    }
}

async fn handle_analyze_command(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let file = matches.get_one::<String>("file").unwrap();
    let output = matches.get_one::<String>("output");
    let format = matches.get_one::<String>("format").unwrap();
    let detailed = matches.get_flag("detailed");
    let suggestions = matches.get_flag("suggestions");

    println!("🔍 Analyzing compilation performance for: {}", file);

    // Verify file exists
    if !Path::new(file).exists() {
        return Err(format!("File not found: {}", file).into());
    }

    // Read source code
    let source = fs::read_to_string(file)?;

    // Create performance analyzer
    let mut analyzer = PerformanceAnalyzer::new();
    analyzer.set_detailed_analysis(detailed);
    analyzer.set_include_suggestions(suggestions);

    // Run analysis
    println!("   📊 Running performance analysis...");
    let analysis_result = analyzer.analyze(&source, file).await?;

    // Generate report
    let report = generate_analysis_report(&analysis_result, format, detailed, suggestions)?;

    // Output results
    if let Some(output_file) = output {
        fs::write(output_file, &report)?;
        println!("✅ Analysis report written to: {}", output_file);
    } else {
        println!("\n{}", report);
    }

    // Print summary
    println!("\n📈 Analysis Summary:");
    println!("   Compilation phases analyzed: {}", analysis_result.phases.len());
    println!("   Performance bottlenecks found: {}", analysis_result.bottlenecks.len());
    
    if suggestions && !analysis_result.recommendations.is_empty() {
        println!("   Optimization recommendations: {}", analysis_result.recommendations.len());
        println!("\n💡 Top Recommendations:");
        for (i, rec) in analysis_result.recommendations.iter().take(3).enumerate() {
            println!("   {}. {}", i + 1, rec.summary);
        }
    }

    Ok(())
}

async fn handle_benchmark_command(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let file = matches.get_one::<String>("file").unwrap();
    let levels_str = matches.get_one::<String>("levels").unwrap();
    let iterations: usize = matches.get_one::<String>("iterations").unwrap().parse()?;
    let warmup: usize = matches.get_one::<String>("warmup").unwrap().parse()?;
    let timeout_secs: u64 = matches.get_one::<String>("timeout").unwrap().parse()?;
    let output = matches.get_one::<String>("output");
    let compare = matches.get_one::<String>("compare");
    let parallel = matches.get_flag("parallel");

    println!("🏃 Benchmarking compilation performance for: {}", file);

    // Parse optimization levels
    let levels: Vec<OptimizationLevel> = levels_str
        .split(',')
        .map(|s| s.trim().parse())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Invalid optimization level: {}", e))?;

    // Verify file exists
    if !Path::new(file).exists() {
        return Err(format!("File not found: {}", file).into());
    }

    // Load previous results for comparison if requested
    let previous_results = if let Some(compare_file) = compare {
        load_benchmark_results(compare_file)?
    } else {
        None
    };

    // Create benchmark runner
    let mut runner = BenchmarkRunner::new();
    runner.set_iterations(iterations);
    runner.set_warmup_iterations(warmup);
    runner.set_timeout(Duration::from_secs(timeout_secs));
    runner.set_parallel(parallel);

    println!("   🏁 Configuration:");
    println!("      Levels: {:?}", levels);
    println!("      Iterations: {} (+ {} warmup)", iterations, warmup);
    println!("      Timeout: {}s per compilation", timeout_secs);
    println!("      Parallel: {}", parallel);

    // Run benchmarks
    println!("\n   🚀 Running benchmarks...");
    let mut results = HashMap::new();
    
    for level in &levels {
        println!("      Testing O{:?}...", level);
        
        let start_time = Instant::now();
        let level_result = runner.benchmark_file(file, *level).await?;
        let total_time = start_time.elapsed();
        
        results.insert(*level, level_result);
        
        println!("         ✅ Completed in {:.2}s", total_time.as_secs_f64());
    }

    // Generate report
    let report = generate_benchmark_report(&results, previous_results.as_ref())?;

    // Output results
    if let Some(output_file) = output {
        // Save detailed results as JSON for future comparison
        let json_results = serde_json::to_string_pretty(&results)?;
        fs::write(output_file, &json_results)?;
        println!("✅ Benchmark results saved to: {}", output_file);
    }

    println!("\n{}", report);

    // Print performance summary
    print_benchmark_summary(&results);

    Ok(())
}

async fn handle_profile_command(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let file = matches.get_one::<String>("file").unwrap();
    let opt_level = matches.get_one::<String>("opt-level").unwrap();
    let profile_phases = matches.get_flag("phases");
    let track_memory = matches.get_flag("memory");
    let sample_rate: u64 = matches.get_one::<String>("sample-rate").unwrap().parse()?;
    let output = matches.get_one::<String>("output");
    let flamegraph = matches.get_flag("flamegraph");

    println!("📊 Profiling compilation for: {} (O{})", file, opt_level);

    // Verify file exists
    if !Path::new(file).exists() {
        return Err(format!("File not found: {}", file).into());
    }

    // Parse optimization level
    let level: OptimizationLevel = opt_level.parse()
        .map_err(|e| format!("Invalid optimization level: {}", e))?;

    // Create profiler configuration
    let mut config = ReportConfig::default();
    config.include_phases = profile_phases;
    config.include_memory = track_memory;
    config.sample_rate = sample_rate;
    if flamegraph {
        config.format = ReportFormat::Flamegraph;
    }

    // Create performance monitor
    let mut monitor = PerformanceMonitor::with_config(config);

    println!("   ⚙️  Configuration:");
    println!("      Optimization level: O{:?}", level);
    println!("      Profile phases: {}", profile_phases);
    println!("      Track memory: {}", track_memory);
    println!("      Sample rate: {}Hz", sample_rate);

    // Read source and start profiling
    let source = fs::read_to_string(file)?;
    
    println!("\n   🔬 Starting profiling...");
    monitor.start_phase(CompilationPhase::Total)?;

    // Run compilation with profiling
    let profiler = CompilationProfiler::new(monitor);
    let profile_result = profiler.profile_compilation(&source, file, level).await?;

    println!("   ✅ Profiling completed");

    // Generate profiling report
    let report = generate_profiling_report(&profile_result, flamegraph)?;

    // Output results
    if let Some(output_file) = output {
        fs::write(output_file, &report)?;
        println!("✅ Profiling report written to: {}", output_file);
    } else {
        println!("\n{}", report);
    }

    // Print profiling summary
    print_profiling_summary(&profile_result);

    Ok(())
}

async fn handle_enable_command(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let passes_str = matches.get_one::<String>("passes").unwrap();
    let global = matches.get_flag("global");
    let project = matches.get_flag("project");

    let passes: Vec<String> = passes_str
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    println!("✅ Enabling optimization passes: {:?}", passes);

    // Load current configuration
    let mut config = load_optimization_config(global, project)?;

    // Enable passes
    for pass in &passes {
        if !config.enabled_passes.contains(pass) {
            config.enabled_passes.push(pass.clone());
        }
        config.disabled_passes.retain(|p| p != pass);
        println!("   ✅ Enabled: {}", pass);
    }

    // Save configuration
    save_optimization_config(&config, global, project)?;

    println!("✅ Configuration updated successfully");
    
    Ok(())
}

async fn handle_disable_command(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let passes_str = matches.get_one::<String>("passes").unwrap();
    let global = matches.get_flag("global");
    let project = matches.get_flag("project");

    let passes: Vec<String> = passes_str
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    println!("❌ Disabling optimization passes: {:?}", passes);

    // Load current configuration
    let mut config = load_optimization_config(global, project)?;

    // Disable passes
    for pass in &passes {
        if !config.disabled_passes.contains(pass) {
            config.disabled_passes.push(pass.clone());
        }
        config.enabled_passes.retain(|p| p != pass);
        println!("   ❌ Disabled: {}", pass);
    }

    // Save configuration
    save_optimization_config(&config, global, project)?;

    println!("✅ Configuration updated successfully");
    
    Ok(())
}

async fn handle_config_command(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let show = matches.get_flag("show");
    let set_values = matches.get_many::<String>("set");
    let unset_values = matches.get_many::<String>("unset");
    let default_level = matches.get_one::<String>("default-level");
    let global = matches.get_flag("global");
    let export = matches.get_one::<String>("export");
    let import = matches.get_one::<String>("import");

    // Load current configuration
    let mut config = load_optimization_config(global, false)?;

    if show {
        print_configuration(&config, global);
        return Ok(());
    }

    if let Some(import_file) = import {
        println!("📥 Importing configuration from: {}", import_file);
        let imported_config: OptimizationCliConfig = {
            let content = fs::read_to_string(import_file)?;
            serde_json::from_str(&content)?
        };
        config = imported_config;
        save_optimization_config(&config, global, false)?;
        println!("✅ Configuration imported successfully");
        return Ok(());
    }

    if let Some(export_file) = export {
        println!("📤 Exporting configuration to: {}", export_file);
        let json = serde_json::to_string_pretty(&config)?;
        fs::write(export_file, json)?;
        println!("✅ Configuration exported successfully");
        return Ok(());
    }

    let mut modified = false;

    // Set values
    if let Some(set_iter) = set_values {
        for set_value in set_iter {
            let parts: Vec<&str> = set_value.splitn(2, '=').collect();
            if parts.len() != 2 {
                eprintln!("Invalid set format: {}. Use KEY=VALUE", set_value);
                continue;
            }
            
            let key = parts[0].trim();
            let value = parts[1].trim();
            
            config.custom_params.insert(key.to_string(), value.to_string());
            println!("✅ Set {}: {}", key, value);
            modified = true;
        }
    }

    // Unset values
    if let Some(unset_iter) = unset_values {
        for key in unset_iter {
            if config.custom_params.remove(key).is_some() {
                println!("✅ Unset: {}", key);
                modified = true;
            } else {
                println!("⚠️  Key not found: {}", key);
            }
        }
    }

    // Set default level
    if let Some(level_str) = default_level {
        let level: OptimizationLevel = level_str.parse()
            .map_err(|e| format!("Invalid optimization level: {}", e))?;
        config.default_level = level;
        println!("✅ Set default optimization level: O{:?}", level);
        modified = true;
    }

    if modified {
        save_optimization_config(&config, global, false)?;
        println!("✅ Configuration updated successfully");
    } else {
        println!("ℹ️  No configuration changes made");
    }

    Ok(())
}

async fn handle_reset_command(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let global = matches.get_flag("global");
    let project = matches.get_flag("project");
    let confirm = matches.get_flag("confirm");

    if !global && !project {
        return Err("Must specify either --global or --project".into());
    }

    if !confirm {
        println!("⚠️  This will reset optimization configuration to defaults.");
        print!("Are you sure? [y/N]: ");
        use std::io::{self, Write};
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        if !input.trim().to_lowercase().starts_with('y') {
            println!("Reset cancelled");
            return Ok(());
        }
    }

    println!("🔄 Resetting optimization configuration...");

    // Create default configuration
    let default_config = OptimizationCliConfig::default();

    // Save default configuration
    save_optimization_config(&default_config, global, project)?;

    if global {
        println!("✅ Global optimization configuration reset to defaults");
    }
    if project {
        println!("✅ Project optimization configuration reset to defaults");
    }

    Ok(())
}

async fn handle_interactive_command(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let file = matches.get_one::<String>("file");
    let quick = matches.get_flag("quick");
    let advanced = matches.get_flag("advanced");

    println!("🎯 Interactive Optimization Wizard");
    println!("   Let's optimize your CURSED project!");

    // Load current configuration
    let mut config = load_optimization_config(false, true)?;

    if let Some(file_path) = file {
        println!("\n📁 Analyzing file: {}", file_path);
        
        // Verify file exists
        if !Path::new(file_path).exists() {
            return Err(format!("File not found: {}", file_path).into());
        }

        // Analyze the file to provide specific recommendations
        analyze_file_for_recommendations(file_path).await?;
    }

    // Interactive questions
    if !quick {
        config = run_interactive_wizard(config, advanced).await?;
    } else {
        config = run_quick_wizard(config).await?;
    }

    // Save configuration
    save_optimization_config(&config, false, true)?;
    
    println!("\n✅ Interactive optimization configuration completed!");
    println!("   Use 'cursed optimize config --show' to review your settings");
    
    Ok(())
}

async fn handle_apply_command(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let file = matches.get_one::<String>("file").unwrap();
    let profile = matches.get_one::<String>("profile");
    let dry_run = matches.get_flag("dry-run");
    let aggressive = matches.get_flag("aggressive");
    let safe = matches.get_flag("safe");

    println!("🚀 Auto-applying optimization recommendations");
    println!("   File: {}", file);

    // Verify file exists
    if !Path::new(file).exists() {
        return Err(format!("File not found: {}", file).into());
    }

    // Load current configuration
    let mut config = load_optimization_config(false, true)?;

    // Apply profile if specified
    if let Some(profile_name) = profile {
        if let Some(project_profile) = config.profiles.get(profile_name) {
            println!("   Applying profile: {} - {}", project_profile.name, project_profile.description);
            
            config.default_level = project_profile.optimization_level;
            config.enabled_passes = project_profile.enabled_passes.clone();
            config.disabled_passes = project_profile.disabled_passes.clone();
            
            // Merge profile parameters
            for (key, value) in &project_profile.parameters {
                config.custom_params.insert(key.clone(), value.clone());
            }
        } else {
            return Err(format!("Profile not found: {}. Use 'cursed optimize profiles --list' to see available profiles", profile_name).into());
        }
    }

    // Analyze file for specific recommendations
    let recommendations = analyze_file_for_specific_recommendations(file).await?;

    if aggressive {
        apply_aggressive_optimizations(&mut config);
        println!("   🔥 Applied aggressive optimizations");
    } else if safe {
        apply_safe_optimizations(&mut config);
        println!("   🛡️  Applied safe optimizations only");
    }

    // Apply recommendations
    apply_recommendations(&mut config, &recommendations);

    if dry_run {
        println!("\n📋 Dry run - would apply the following optimizations:");
        print_optimization_preview(&config);
        println!("\n   Use without --dry-run to apply these changes");
    } else {
        // Save configuration
        save_optimization_config(&config, false, true)?;
        
        println!("\n✅ Optimization recommendations applied successfully!");
        println!("   Configuration saved to project settings");
        
        // Show summary of what was applied
        print_applied_optimizations_summary(&config, &recommendations);
    }

    Ok(())
}

async fn handle_profiles_command(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let list = matches.get_flag("list");
    let create = matches.get_one::<String>("create");
    let delete = matches.get_one::<String>("delete");
    let copy = matches.get_one::<String>("copy");
    let export = matches.get_one::<String>("export");
    let import = matches.get_one::<String>("import");

    let mut config = load_optimization_config(false, false)?;

    if list {
        println!("📋 Available Optimization Profiles:");
        println!();
        
        for (name, profile) in &config.profiles {
            println!("   🎯 {} ({})", name, profile.optimization_level.as_str());
            println!("      {}", profile.description);
            println!("      Passes: {} enabled, {} disabled", 
                     profile.enabled_passes.len(), 
                     profile.disabled_passes.len());
            
            if profile.build_config.lto {
                println!("      Features: LTO enabled");
            }
            if profile.build_config.parallel {
                println!("      Features: Parallel compilation");
            }
            println!();
        }
        return Ok(());
    }

    if let Some(name) = create {
        println!("🔧 Creating new profile: {}", name);
        
        let new_profile = create_interactive_profile(name.clone()).await?;
        config.profiles.insert(name.clone(), new_profile);
        
        save_optimization_config(&config, false, false)?;
        println!("✅ Profile '{}' created successfully", name);
        return Ok(());
    }

    if let Some(name) = delete {
        if config.profiles.remove(name).is_some() {
            save_optimization_config(&config, false, false)?;
            println!("✅ Profile '{}' deleted successfully", name);
        } else {
            println!("❌ Profile '{}' not found", name);
        }
        return Ok(());
    }

    if let Some(copy_spec) = copy {
        let parts: Vec<&str> = copy_spec.split(',').collect();
        if parts.len() != 2 {
            return Err("Copy format should be 'source,destination'".into());
        }
        
        let (source, dest) = (parts[0].trim(), parts[1].trim());
        
        if let Some(source_profile) = config.profiles.get(source) {
            let mut new_profile = source_profile.clone();
            new_profile.name = dest.to_string();
            new_profile.description = format!("Copy of {}", source);
            
            config.profiles.insert(dest.to_string(), new_profile);
            save_optimization_config(&config, false, false)?;
            
            println!("✅ Profile copied from '{}' to '{}'", source, dest);
        } else {
            return Err(format!("Source profile '{}' not found", source).into());
        }
        return Ok(());
    }

    if let Some(export_spec) = export {
        let parts: Vec<&str> = export_spec.split(',').collect();
        if parts.len() != 2 {
            return Err("Export format should be 'profile_name,file_path'".into());
        }
        
        let (profile_name, file_path) = (parts[0].trim(), parts[1].trim());
        
        if let Some(profile) = config.profiles.get(profile_name) {
            let json = serde_json::to_string_pretty(profile)?;
            fs::write(file_path, json)?;
            println!("✅ Profile '{}' exported to '{}'", profile_name, file_path);
        } else {
            return Err(format!("Profile '{}' not found", profile_name).into());
        }
        return Ok(());
    }

    if let Some(import_spec) = import {
        let parts: Vec<&str> = import_spec.split(',').collect();
        if parts.len() != 2 {
            return Err("Import format should be 'file_path,profile_name'".into());
        }
        
        let (file_path, profile_name) = (parts[0].trim(), parts[1].trim());
        
        let content = fs::read_to_string(file_path)?;
        let mut profile: ProjectProfile = serde_json::from_str(&content)?;
        profile.name = profile_name.to_string();
        
        config.profiles.insert(profile_name.to_string(), profile);
        save_optimization_config(&config, false, false)?;
        
        println!("✅ Profile imported as '{}'", profile_name);
        return Ok(());
    }

    println!("ℹ️  Use --help to see available profile commands");
    Ok(())
}

// Helper functions for interactive optimization

async fn analyze_file_for_recommendations(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("   🔍 Analyzing file structure and patterns...");
    
    let source = fs::read_to_string(file_path)?;
    let lines = source.split("\n").count();
    let has_loops = source.contains("lowkey") || source.contains("bestie");
    let has_functions = source.contains("slay");
    let has_math = source.contains("math::") || source.contains("calculate");
    
    println!("      Lines of code: {}", lines);
    println!("      Contains loops: {}", has_loops);
    println!("      Contains functions: {}", has_functions);
    println!("      Contains math operations: {}", has_math);
    
    if lines > 1000 {
        println!("      💡 Large file detected - consider parallel compilation");
    }
    if has_math {
        println!("      💡 Math operations detected - vectorization may help");
    }
    if has_loops {
        println!("      💡 Loops detected - loop optimization recommended");
    }
    
    Ok(())
}

async fn run_interactive_wizard(mut config: OptimizationCliConfig, advanced: bool) -> Result<OptimizationCliConfig, Box<dyn std::error::Error>> {
    use std::io::{self, Write};

    println!("\n🎛️  Interactive Configuration Wizard");
    
    // Ask about optimization level
    println!("\n1. What's your primary goal?");
    println!("   a) Fast compilation (development)");
    println!("   b) Balanced performance (default)");
    println!("   c) Maximum performance (release)");
    println!("   d) Smallest binary size");
    
    print!("   Choice [b]: ");
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    match input.trim().to_lowercase().as_str() {
        "a" => config.default_level = OptimizationLevel::O0,
        "c" => config.default_level = OptimizationLevel::O3,
        "d" => config.default_level = OptimizationLevel::Oz,
        _ => config.default_level = OptimizationLevel::O2,
    }
    
    // Ask about parallelization
    println!("\n2. Enable parallel compilation? [Y/n]: ");
    io::stdout().flush()?;
    input.clear();
    io::stdin().read_line(&mut input)?;
    
    let enable_parallel = !input.trim().to_lowercase().starts_with('n');
    if enable_parallel {
        config.custom_params.insert("parallel".to_string(), "true".to_string());
    }
    
    if advanced {
        // Advanced questions
        println!("\n🔧 Advanced Options");
        
        println!("\n3. Enable Link Time Optimization (LTO)? [y/N]: ");
        io::stdout().flush()?;
        input.clear();
        io::stdin().read_line(&mut input)?;
        
        if input.trim().to_lowercase().starts_with('y') {
            config.custom_params.insert("lto".to_string(), "true".to_string());
        }
        
        println!("\n4. Aggressive inlining? [y/N]: ");
        io::stdout().flush()?;
        input.clear();
        io::stdin().read_line(&mut input)?;
        
        if input.trim().to_lowercase().starts_with('y') {
            config.enabled_passes.push("aggressive-inline".to_string());
        }
    }
    
    println!("\n✅ Configuration wizard completed!");
    Ok(config)
}

async fn run_quick_wizard(mut config: OptimizationCliConfig) -> Result<OptimizationCliConfig, Box<dyn std::error::Error>> {
    println!("\n⚡ Quick optimization setup");
    println!("   Applying balanced performance settings...");
    
    config.default_level = OptimizationLevel::O2;
    config.enabled_passes = vec![
        "inline".to_string(),
        "dce".to_string(),
        "mem2reg".to_string(),
        "gvn".to_string(),
        "loop-simplify".to_string(),
    ];
    config.custom_params.insert("parallel".to_string(), "true".to_string());
    
    println!("   ✅ Quick setup complete!");
    Ok(config)
}

async fn analyze_file_for_specific_recommendations(file_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let source = fs::read_to_string(file_path)?;
    let mut recommendations = Vec::new();
    
    // Analyze code patterns
    if source.contains("lowkey") && source.matches("lowkey").count() > 10 {
        recommendations.push("loop-unroll".to_string());
        recommendations.push("loop-optimize".to_string());
    }
    
    if source.contains("math::") || source.contains("calculate") || source.contains("compute") {
        recommendations.push("vectorize".to_string());
        recommendations.push("math-optimize".to_string());
    }
    
    if source.contains("goroutine") || source.contains("stan") {
        recommendations.push("goroutine-optimize".to_string());
    }
    
    if source.contains("channel") || source.contains("pipe") {
        recommendations.push("channel-optimize".to_string());
    }
    
    if source.split("\n").count() > 500 {
        recommendations.push("inline".to_string());
        recommendations.push("dce".to_string());
    }
    
    Ok(recommendations)
}

fn apply_aggressive_optimizations(config: &mut OptimizationCliConfig) {
    config.default_level = OptimizationLevel::O3;
    config.enabled_passes.extend_from_slice(&[
        "aggressive-inline".to_string(),
        "loop-unroll".to_string(),
        "vectorize".to_string(),
        "slp-vectorize".to_string(),
        "gvn-hoist".to_string(),
    ]);
    config.custom_params.insert("aggressive".to_string(), "true".to_string());
    config.custom_params.insert("lto".to_string(), "true".to_string());
}

fn apply_safe_optimizations(config: &mut OptimizationCliConfig) {
    config.default_level = OptimizationLevel::O1;
    config.enabled_passes = vec![
        "mem2reg".to_string(),
        "dce".to_string(),
        "gvn".to_string(),
    ];
    config.custom_params.insert("safe-mode".to_string(), "true".to_string());
}

fn apply_recommendations(config: &mut OptimizationCliConfig, recommendations: &[String]) {
    for rec in recommendations {
        if !config.enabled_passes.contains(rec) {
            config.enabled_passes.push(rec.clone());
        }
    }
}

fn print_optimization_preview(config: &OptimizationCliConfig) {
    println!("   Optimization Level: {}", config.default_level.as_str());
    println!("   Enabled Passes: {:?}", config.enabled_passes);
    if !config.disabled_passes.is_empty() {
        println!("   Disabled Passes: {:?}", config.disabled_passes);
    }
    if !config.custom_params.is_empty() {
        println!("   Custom Parameters:");
        for (key, value) in &config.custom_params {
            println!("     {}: {}", key, value);
        }
    }
}

fn print_applied_optimizations_summary(config: &OptimizationCliConfig, recommendations: &[String]) {
    println!("\n📊 Applied Optimizations Summary:");
    println!("   Level: {}", config.default_level.as_str());
    println!("   Total passes enabled: {}", config.enabled_passes.len());
    println!("   Recommendations applied: {}", recommendations.len());
    
    if !recommendations.is_empty() {
        println!("   Specific recommendations:");
        for rec in recommendations {
            println!("     ✓ {}", rec);
        }
    }
}

async fn create_interactive_profile(name: String) -> Result<ProjectProfile, Box<dyn std::error::Error>> {
    use std::io::{self, Write};
    
    println!("Creating profile: {}", name);
    
    print!("Description: ");
    io::stdout().flush()?;
    let mut description = String::new();
    io::stdin().read_line(&mut description)?;
    
    print!("Optimization level (0,1,2,3,s,z) [2]: ");
    io::stdout().flush()?;
    let mut level_input = String::new();
    io::stdin().read_line(&mut level_input)?;
    
    let optimization_level = if level_input.trim().is_empty() {
        OptimizationLevel::O2
    } else {
        level_input.trim().parse().unwrap_or(OptimizationLevel::O2)
    };
    
    Ok(ProjectProfile {
        name,
        description: description.trim().to_string(),
        optimization_level,
        enabled_passes: vec![
            "inline".to_string(),
            "dce".to_string(),
            "mem2reg".to_string(),
        ],
        disabled_passes: vec![],
        parameters: HashMap::new(),
        build_config: BuildConfig::default(),
    })
}

// Helper functions for configuration management
fn get_config_path(global: bool, project: bool) -> Result<PathBuf, Box<dyn std::error::Error>> {
    if global {
        let home = std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE"))
            .map_err(|_| "Could not determine home directory")?;
        Ok(PathBuf::from(home).join(".cursed").join("optimization.json"))
    } else if project {
        Ok(PathBuf::from(".cursed").join("optimization.json"))
    } else {
        // Default to project-local
        Ok(PathBuf::from(".cursed").join("optimization.json"))
    }
}

fn load_optimization_config(global: bool, project: bool) -> Result<OptimizationCliConfig, Box<dyn std::error::Error>> {
    let config_path = get_config_path(global, project)?;
    
    if config_path.exists() {
        let content = fs::read_to_string(&config_path)?;
        let config: OptimizationCliConfig = serde_json::from_str(&content)?;
        Ok(config)
    } else {
        Ok(OptimizationCliConfig::default())
    }
}

fn save_optimization_config(config: &OptimizationCliConfig, global: bool, project: bool) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = get_config_path(global, project)?;
    
    // Create directory if it doesn't exist
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    let json = serde_json::to_string_pretty(config)?;
    fs::write(&config_path, json)?;
    
    Ok(())
}

fn print_configuration(config: &OptimizationCliConfig, global: bool) {
    let scope = if global { "Global" } else { "Project" };
    
    println!("⚙️  {} Optimization Configuration:", scope);
    println!("   Default Level: O{:?}", config.default_level);
    println!("   Enabled Passes: {:?}", config.enabled_passes);
    if !config.disabled_passes.is_empty() {
        println!("   Disabled Passes: {:?}", config.disabled_passes);
    }
    
    if !config.custom_params.is_empty() {
        println!("   Custom Parameters:");
        for (key, value) in &config.custom_params {
            println!("     {}: {}", key, value);
        }
    }
    
    println!("   Benchmark Settings:");
    println!("     Iterations: {}", config.benchmark_config.iterations);
    println!("     Timeout: {}s", config.benchmark_config.timeout_seconds);
    println!("     Warmup: {}", config.benchmark_config.warmup_iterations);
    
    println!("   Profiling Settings:");
    println!("     Detailed Timing: {}", config.profiling_config.detailed_timing);
    println!("     Memory Tracking: {}", config.profiling_config.memory_tracking);
    println!("     Sample Rate: {}Hz", config.profiling_config.sample_rate);
}

// Real report generation functions with comprehensive analysis and formatting
fn generate_analysis_report(
    result: &crate::optimization::analysis::AnalysisResult,
    format: &str,
    detailed: bool,
    suggestions: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    match format {
        "json" => generate_json_analysis_report(result),
        "markdown" => generate_markdown_analysis_report(result, detailed, suggestions),
        "table" => generate_table_analysis_report(result),
        _ => Err(format!("Unsupported format: {}", format).into()),
    }
}

fn generate_json_analysis_report(
    result: &crate::optimization::analysis::AnalysisResult,
) -> Result<String, Box<dyn std::error::Error>> {
    let json_report = serde_json::json!({
        "analysis_timestamp": result.analysis_timestamp,
        "source_file": result.source_file,
        "performance_summary": {
            "total_compilation_time_ms": result.performance_summary.total_compilation_time.as_millis(),
            "peak_memory_mb": result.performance_summary.total_memory_peak / (1024 * 1024),
            "cpu_efficiency": result.performance_summary.cpu_efficiency,
            "memory_efficiency": result.performance_summary.memory_efficiency,
            "io_efficiency": result.performance_summary.io_efficiency,
            "overall_score": result.performance_summary.overall_performance_score,
            "optimization_opportunities": result.performance_summary.optimization_opportunities,
            "critical_bottlenecks": result.performance_summary.critical_bottlenecks
        },
        "phases": result.phases.iter().map(|phase| {
            serde_json::json!({
                "name": phase.name,
                "duration_ms": phase.duration.as_millis(),
                "cpu_usage": phase.cpu_usage,
                "memory_peak_mb": phase.memory_peak / (1024 * 1024),
                "memory_average_mb": phase.memory_average / (1024 * 1024),
                "efficiency_score": phase.efficiency_score,
                "io_operations": {
                    "read_ops": phase.io_operations.read_operations,
                    "write_ops": phase.io_operations.write_operations,
                    "bytes_read": phase.io_operations.bytes_read,
                    "bytes_written": phase.io_operations.bytes_written
                },
                "bottlenecks": phase.bottlenecks
            })
        }).collect::<Vec<_>>(),
        "bottlenecks": result.bottlenecks.iter().map(|bottleneck| {
            serde_json::json!({
                "type": format!("{:?}", bottleneck.bottleneck_type),
                "severity": format!("{:?}", bottleneck.severity),
                "location": {
                    "phase": bottleneck.location.phase,
                    "function": bottleneck.location.function,
                    "line_number": bottleneck.location.line_number,
                    "module": bottleneck.location.module
                },
                "description": bottleneck.description,
                "impact_percentage": bottleneck.impact_percentage,
                "time_spent_ms": bottleneck.time_spent.as_millis(),
                "suggested_fixes": bottleneck.suggested_fixes
            })
        }).collect::<Vec<_>>(),
        "recommendations": result.recommendations.iter().map(|rec| {
            serde_json::json!({
                "id": rec.recommendation_id,
                "priority": rec.priority,
                "category": format!("{:?}", rec.category),
                "title": rec.title,
                "summary": rec.summary,
                "expected_improvement": {
                    "compilation_time_reduction": rec.expected_improvement.compilation_time_reduction,
                    "runtime_performance_gain": rec.expected_improvement.runtime_performance_gain,
                    "memory_reduction": rec.expected_improvement.memory_reduction,
                    "confidence_level": rec.expected_improvement.confidence_level
                },
                "effort_estimate": {
                    "time_hours": rec.effort_estimate.time_hours,
                    "complexity": format!("{:?}", rec.effort_estimate.complexity)
                },
                "implementation_steps": rec.implementation_steps,
                "prerequisites": rec.prerequisites,
                "risks": rec.risks
            })
        }).collect::<Vec<_>>(),
        "detailed_metrics": {
            "compilation_metrics": {
                "lexing_time_ms": result.detailed_metrics.compilation_metrics.lexing_time.as_millis(),
                "parsing_time_ms": result.detailed_metrics.compilation_metrics.parsing_time.as_millis(),
                "semantic_analysis_time_ms": result.detailed_metrics.compilation_metrics.semantic_analysis_time.as_millis(),
                "type_checking_time_ms": result.detailed_metrics.compilation_metrics.type_checking_time.as_millis(),
                "ir_generation_time_ms": result.detailed_metrics.compilation_metrics.ir_generation_time.as_millis(),
                "optimization_time_ms": result.detailed_metrics.compilation_metrics.optimization_time.as_millis(),
                "code_generation_time_ms": result.detailed_metrics.compilation_metrics.code_generation_time.as_millis(),
                "linking_time_ms": result.detailed_metrics.compilation_metrics.linking_time.as_millis(),
                "total_frontend_time_ms": result.detailed_metrics.compilation_metrics.total_frontend_time.as_millis(),
                "total_backend_time_ms": result.detailed_metrics.compilation_metrics.total_backend_time.as_millis()
            },
            "resource_metrics": {
                "peak_memory_usage": result.detailed_metrics.resource_metrics.peak_memory_usage,
                "average_memory_usage": result.detailed_metrics.resource_metrics.average_memory_usage,
                "peak_cpu_usage": result.detailed_metrics.resource_metrics.peak_cpu_usage,
                "average_cpu_usage": result.detailed_metrics.resource_metrics.average_cpu_usage,
                "disk_reads": result.detailed_metrics.resource_metrics.disk_reads,
                "disk_writes": result.detailed_metrics.resource_metrics.disk_writes,
                "context_switches": result.detailed_metrics.resource_metrics.context_switches,
                "page_faults": result.detailed_metrics.resource_metrics.page_faults
            },
            "instruction_counts": {
                "total_instructions": result.detailed_metrics.instruction_counts.total_instructions,
                "arithmetic_instructions": result.detailed_metrics.instruction_counts.arithmetic_instructions,
                "memory_instructions": result.detailed_metrics.instruction_counts.memory_instructions,
                "branch_instructions": result.detailed_metrics.instruction_counts.branch_instructions,
                "floating_point_instructions": result.detailed_metrics.instruction_counts.floating_point_instructions,
                "vector_instructions": result.detailed_metrics.instruction_counts.vector_instructions
            },
            "cache_performance": {
                "l1_hit_rate": result.detailed_metrics.cache_performance.l1_hit_rate,
                "l2_hit_rate": result.detailed_metrics.cache_performance.l2_hit_rate,
                "l3_hit_rate": result.detailed_metrics.cache_performance.l3_hit_rate,
                "cache_misses": result.detailed_metrics.cache_performance.cache_misses,
                "cache_miss_penalty_ns": result.detailed_metrics.cache_performance.cache_miss_penalty.as_nanos()
            },
            "branch_prediction": {
                "prediction_accuracy": result.detailed_metrics.branch_prediction.prediction_accuracy,
                "mispredicted_branches": result.detailed_metrics.branch_prediction.mispredicted_branches,
                "branch_penalty_ns": result.detailed_metrics.branch_prediction.branch_penalty.as_nanos(),
                "indirect_branches": result.detailed_metrics.branch_prediction.indirect_branches
            }
        }
    });

    Ok(serde_json::to_string_pretty(&json_report)?)
}

fn generate_markdown_analysis_report(
    result: &crate::optimization::analysis::AnalysisResult,
    detailed: bool,
    suggestions: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut report = String::new();
    
    // Header
    report.push_str("# CURSED Compilation Performance Analysis Report\n\n");
    report.push_str(&format!("**Analysis Date:** {}\n", result.analysis_timestamp.format("%Y-%m-%d %H:%M:%S UTC")));
    report.push_str(&format!("**Source File:** `{}`\n\n", result.source_file));
    
    // Executive Summary
    report.push_str("## 📊 Executive Summary\n\n");
    report.push_str(&format!("- **Total Compilation Time:** {:.2}ms\n", result.performance_summary.total_compilation_time.as_millis()));
    report.push_str(&format!("- **Peak Memory Usage:** {:.1}MB\n", result.performance_summary.total_memory_peak as f64 / (1024.0 * 1024.0)));
    report.push_str(&format!("- **Overall Performance Score:** {:.1}/100\n", result.performance_summary.overall_performance_score * 100.0));
    report.push_str(&format!("- **CPU Efficiency:** {:.1}%\n", result.performance_summary.cpu_efficiency));
    report.push_str(&format!("- **Memory Efficiency:** {:.1}%\n", result.performance_summary.memory_efficiency));
    report.push_str(&format!("- **I/O Efficiency:** {:.1}%\n", result.performance_summary.io_efficiency));
    report.push_str(&format!("- **Critical Bottlenecks:** {}\n", result.performance_summary.critical_bottlenecks));
    report.push_str(&format!("- **Optimization Opportunities:** {}\n\n", result.performance_summary.optimization_opportunities));
    
    // Performance Grade
    let grade = match result.performance_summary.overall_performance_score {
        score if score >= 0.9 => "🟢 Excellent (A)",
        score if score >= 0.8 => "🟡 Good (B)",
        score if score >= 0.7 => "🟠 Fair (C)",
        score if score >= 0.6 => "🔴 Poor (D)",
        _ => "❌ Critical (F)",
    };
    report.push_str(&format!("**Performance Grade:** {}\n\n", grade));
    
    // Phase Analysis
    report.push_str("## ⏱️ Compilation Phase Analysis\n\n");
    report.push_str("| Phase | Duration | CPU Usage | Memory Peak | Efficiency | Bottlenecks |\n");
    report.push_str("|-------|----------|-----------|-------------|------------|-------------|\n");
    
    for phase in &result.phases {
        let bottleneck_count = phase.bottlenecks.len();
        let bottleneck_indicator = if bottleneck_count > 0 { 
            format!("⚠️ {}", bottleneck_count)
        } else { 
            "✅".to_string() 
        };
        
        report.push_str(&format!(
            "| {} | {:.1}ms | {:.1}% | {:.1}MB | {:.1}% | {} |\n",
            phase.name,
            phase.duration.as_millis(),
            phase.cpu_usage,
            phase.memory_peak as f64 / (1024.0 * 1024.0),
            phase.efficiency_score * 100.0,
            bottleneck_indicator
        ));
    }
    report.push_str("\n");
    
    // Bottleneck Analysis
    if !result.bottlenecks.is_empty() {
        report.push_str("## 🚫 Performance Bottlenecks\n\n");
        
        for (i, bottleneck) in result.bottlenecks.iter().enumerate() {
            let severity_emoji = match bottleneck.severity {
                crate::optimization::real_performance_analyzer::BottleneckSeverity::Critical => "🔴",
                crate::optimization::real_performance_analyzer::BottleneckSeverity::High => "🟠",
                crate::optimization::real_performance_analyzer::BottleneckSeverity::Medium => "🟡",
                crate::optimization::real_performance_analyzer::BottleneckSeverity::Low => "🟢",
                crate::optimization::real_performance_analyzer::BottleneckSeverity::Minimal => "⚪",
            };
            
            report.push_str(&format!("### {} Bottleneck #{}: {}\n\n", severity_emoji, i + 1, bottleneck.description));
            report.push_str(&format!("- **Type:** {:?}\n", bottleneck.bottleneck_type));
            report.push_str(&format!("- **Severity:** {:?}\n", bottleneck.severity));
            report.push_str(&format!("- **Location:** {}", bottleneck.location.phase));
            if let Some(ref function) = bottleneck.location.function {
                report.push_str(&format!(" → {}", function));
            }
            if let Some(line) = bottleneck.location.line_number {
                report.push_str(&format!(" (line {})", line));
            }
            report.push_str("\n");
            report.push_str(&format!("- **Impact:** {:.1}% performance loss\n", bottleneck.impact_percentage));
            report.push_str(&format!("- **Time Spent:** {:.2}ms\n\n", bottleneck.time_spent.as_millis()));
            
            if !bottleneck.suggested_fixes.is_empty() {
                report.push_str("**Suggested Fixes:**\n");
                for fix in &bottleneck.suggested_fixes {
                    report.push_str(&format!("- {}\n", fix));
                }
                report.push_str("\n");
            }
        }
    }
    
    // Optimization Recommendations
    if suggestions && !result.recommendations.is_empty() {
        report.push_str("## 💡 Optimization Recommendations\n\n");
        
        for (i, rec) in result.recommendations.iter().enumerate() {
            let priority_emoji = match rec.priority {
                9..=10 => "🔴",
                7..=8 => "🟠", 
                5..=6 => "🟡",
                3..=4 => "🟢",
                _ => "⚪",
            };
            
            report.push_str(&format!("### {} Priority {}: {}\n\n", priority_emoji, rec.priority, rec.title));
            report.push_str(&format!("**Category:** {:?}\n\n", rec.category));
            report.push_str(&format!("{}\n\n", rec.summary));
            
            if detailed {
                report.push_str(&format!("**Detailed Description:**\n{}\n\n", rec.detailed_description));
            }
            
            report.push_str("**Expected Improvements:**\n");
            report.push_str(&format!("- Compilation Time: -{:.1}%\n", rec.expected_improvement.compilation_time_reduction));
            report.push_str(&format!("- Runtime Performance: +{:.1}%\n", rec.expected_improvement.runtime_performance_gain));
            report.push_str(&format!("- Memory Usage: -{:.1}%\n", rec.expected_improvement.memory_reduction));
            report.push_str(&format!("- Confidence: {:.0}%\n\n", rec.expected_improvement.confidence_level * 100.0));
            
            report.push_str("**Implementation Steps:**\n");
            for (j, step) in rec.implementation_steps.iter().enumerate() {
                report.push_str(&format!("{}. {}\n", j + 1, step));
            }
            report.push_str("\n");
            
            report.push_str(&format!("**Effort Estimate:** {:.1} hours ({:?})\n", rec.effort_estimate.time_hours, rec.effort_estimate.complexity));
            
            if !rec.prerequisites.is_empty() {
                report.push_str("\n**Prerequisites:**\n");
                for prereq in &rec.prerequisites {
                    report.push_str(&format!("- {}\n", prereq));
                }
            }
            
            if !rec.risks.is_empty() {
                report.push_str("\n**Risks:**\n");
                for risk in &rec.risks {
                    report.push_str(&format!("- {}\n", risk));
                }
            }
            
            report.push_str("\n");
        }
    }
    
    // Detailed Metrics
    if detailed {
        report.push_str("## 📈 Detailed Metrics\n\n");
        
        report.push_str("### Compilation Phase Breakdown\n\n");
        report.push_str("| Phase | Time (ms) | Percentage |\n");
        report.push_str("|-------|-----------|------------|\n");
        
        let total_time = result.detailed_metrics.compilation_metrics.total_frontend_time + 
                        result.detailed_metrics.compilation_metrics.total_backend_time;
        
        let phases = vec![
            ("Lexing", result.detailed_metrics.compilation_metrics.lexing_time),
            ("Parsing", result.detailed_metrics.compilation_metrics.parsing_time),
            ("Semantic Analysis", result.detailed_metrics.compilation_metrics.semantic_analysis_time),
            ("Type Checking", result.detailed_metrics.compilation_metrics.type_checking_time),
            ("IR Generation", result.detailed_metrics.compilation_metrics.ir_generation_time),
            ("Optimization", result.detailed_metrics.compilation_metrics.optimization_time),
            ("Code Generation", result.detailed_metrics.compilation_metrics.code_generation_time),
            ("Linking", result.detailed_metrics.compilation_metrics.linking_time),
        ];
        
        for (name, duration) in phases {
            let percentage = if total_time.as_nanos() > 0 {
                (duration.as_nanos() as f64 / total_time.as_nanos() as f64) * 100.0
            } else {
                0.0
            };
            report.push_str(&format!("| {} | {:.1} | {:.1}% |\n", name, duration.as_millis(), percentage));
        }
        report.push_str("\n");
        
        report.push_str("### Resource Utilization\n\n");
        report.push_str(&format!("- **Peak Memory:** {:.1}MB\n", result.detailed_metrics.resource_metrics.peak_memory_usage as f64 / (1024.0 * 1024.0)));
        report.push_str(&format!("- **Average Memory:** {:.1}MB\n", result.detailed_metrics.resource_metrics.average_memory_usage as f64 / (1024.0 * 1024.0)));
        report.push_str(&format!("- **Peak CPU:** {:.1}%\n", result.detailed_metrics.resource_metrics.peak_cpu_usage));
        report.push_str(&format!("- **Average CPU:** {:.1}%\n", result.detailed_metrics.resource_metrics.average_cpu_usage));
        report.push_str(&format!("- **Disk Reads:** {}\n", result.detailed_metrics.resource_metrics.disk_reads));
        report.push_str(&format!("- **Disk Writes:** {}\n", result.detailed_metrics.resource_metrics.disk_writes));
        report.push_str(&format!("- **Context Switches:** {}\n", result.detailed_metrics.resource_metrics.context_switches));
        report.push_str(&format!("- **Page Faults:** {}\n\n", result.detailed_metrics.resource_metrics.page_faults));
        
        report.push_str("### Instruction Analysis\n\n");
        report.push_str(&format!("- **Total Instructions:** {}\n", result.detailed_metrics.instruction_counts.total_instructions));
        report.push_str(&format!("- **Arithmetic Instructions:** {} ({:.1}%)\n", 
            result.detailed_metrics.instruction_counts.arithmetic_instructions,
            (result.detailed_metrics.instruction_counts.arithmetic_instructions as f64 / result.detailed_metrics.instruction_counts.total_instructions as f64) * 100.0
        ));
        report.push_str(&format!("- **Memory Instructions:** {} ({:.1}%)\n", 
            result.detailed_metrics.instruction_counts.memory_instructions,
            (result.detailed_metrics.instruction_counts.memory_instructions as f64 / result.detailed_metrics.instruction_counts.total_instructions as f64) * 100.0
        ));
        report.push_str(&format!("- **Branch Instructions:** {} ({:.1}%)\n", 
            result.detailed_metrics.instruction_counts.branch_instructions,
            (result.detailed_metrics.instruction_counts.branch_instructions as f64 / result.detailed_metrics.instruction_counts.total_instructions as f64) * 100.0
        ));
        report.push_str(&format!("- **Vector Instructions:** {} ({:.1}%)\n\n", 
            result.detailed_metrics.instruction_counts.vector_instructions,
            (result.detailed_metrics.instruction_counts.vector_instructions as f64 / result.detailed_metrics.instruction_counts.total_instructions as f64) * 100.0
        ));
        
        report.push_str("### Cache Performance\n\n");
        report.push_str(&format!("- **L1 Cache Hit Rate:** {:.1}%\n", result.detailed_metrics.cache_performance.l1_hit_rate * 100.0));
        report.push_str(&format!("- **L2 Cache Hit Rate:** {:.1}%\n", result.detailed_metrics.cache_performance.l2_hit_rate * 100.0));
        report.push_str(&format!("- **L3 Cache Hit Rate:** {:.1}%\n", result.detailed_metrics.cache_performance.l3_hit_rate * 100.0));
        report.push_str(&format!("- **Total Cache Misses:** {}\n", result.detailed_metrics.cache_performance.cache_misses));
        report.push_str(&format!("- **Cache Miss Penalty:** {}ns\n\n", result.detailed_metrics.cache_performance.cache_miss_penalty.as_nanos()));
        
        report.push_str("### Branch Prediction\n\n");
        report.push_str(&format!("- **Prediction Accuracy:** {:.1}%\n", result.detailed_metrics.branch_prediction.prediction_accuracy * 100.0));
        report.push_str(&format!("- **Mispredicted Branches:** {}\n", result.detailed_metrics.branch_prediction.mispredicted_branches));
        report.push_str(&format!("- **Branch Penalty:** {}ns\n", result.detailed_metrics.branch_prediction.branch_penalty.as_nanos()));
        report.push_str(&format!("- **Indirect Branches:** {}\n\n", result.detailed_metrics.branch_prediction.indirect_branches));
    }
    
    // Trend Analysis
    if let Some(ref trend_analysis) = result.trend_analysis {
        report.push_str("## 📊 Performance Trends\n\n");
        report.push_str(&format!("- **Trend Direction:** {:?}\n", trend_analysis.performance_trend));
        report.push_str(&format!("- **Trend Strength:** {:.1}%\n", trend_analysis.trend_strength * 100.0));
        report.push_str(&format!("- **Stability Score:** {:.1}%\n", trend_analysis.stability_score * 100.0));
        
        if trend_analysis.regression_detected {
            report.push_str("- **⚠️ Performance Regression Detected**\n");
        }
        if trend_analysis.improvement_detected {
            report.push_str("- **✅ Performance Improvement Detected**\n");
        }
        report.push_str("\n");
    }
    
    // Footer
    report.push_str("---\n\n");
    report.push_str("*Generated by CURSED Compiler Performance Analysis System*\n");
    report.push_str(&format!("*Report generated at: {}*", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));
    
    Ok(report)
}

fn generate_table_analysis_report(
    result: &crate::optimization::analysis::AnalysisResult,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut report = String::new();
    
    report.push_str("CURSED COMPILATION PERFORMANCE ANALYSIS\n");
    report.push_str("═══════════════════════════════════════════════════════════════\n\n");
    
    // Summary Table
    report.push_str("PERFORMANCE SUMMARY\n");
    report.push_str("───────────────────────────────────────────────────────────────\n");
    report.push_str(&format!("Source File:             {}\n", result.source_file));
    report.push_str(&format!("Analysis Date:           {}\n", result.analysis_timestamp.format("%Y-%m-%d %H:%M:%S UTC")));
    report.push_str(&format!("Total Compilation Time:  {:.2}ms\n", result.performance_summary.total_compilation_time.as_millis()));
    report.push_str(&format!("Peak Memory Usage:       {:.1}MB\n", result.performance_summary.total_memory_peak as f64 / (1024.0 * 1024.0)));
    report.push_str(&format!("CPU Efficiency:          {:.1}%\n", result.performance_summary.cpu_efficiency));
    report.push_str(&format!("Memory Efficiency:       {:.1}%\n", result.performance_summary.memory_efficiency));
    report.push_str(&format!("I/O Efficiency:          {:.1}%\n", result.performance_summary.io_efficiency));
    report.push_str(&format!("Overall Score:           {:.1}/100\n", result.performance_summary.overall_performance_score * 100.0));
    report.push_str(&format!("Critical Bottlenecks:    {}\n", result.performance_summary.critical_bottlenecks));
    report.push_str(&format!("Optimization Opportunities: {}\n", result.performance_summary.optimization_opportunities));
    report.push_str("\n");
    
    // Phase Breakdown Table
    report.push_str("COMPILATION PHASE BREAKDOWN\n");
    report.push_str("───────────────────────────────────────────────────────────────\n");
    report.push_str("Phase                    Duration  CPU%    Memory    Efficiency\n");
    report.push_str("                         (ms)              (MB)      Score     \n");
    report.push_str("───────────────────────────────────────────────────────────────\n");
    
    for phase in &result.phases {
        report.push_str(&format!(
            "{:<24} {:>8.1} {:>6.1} {:>8.1} {:>10.1}%\n",
            phase.name,
            phase.duration.as_millis(),
            phase.cpu_usage,
            phase.memory_peak as f64 / (1024.0 * 1024.0),
            phase.efficiency_score * 100.0
        ));
    }
    report.push_str("\n");
    
    // Bottlenecks Table
    if !result.bottlenecks.is_empty() {
        report.push_str("PERFORMANCE BOTTLENECKS\n");
        report.push_str("───────────────────────────────────────────────────────────────\n");
        report.push_str("Type               Severity  Phase              Impact%    Time\n");
        report.push_str("───────────────────────────────────────────────────────────────\n");
        
        for bottleneck in &result.bottlenecks {
            let severity_str = match bottleneck.severity {
                crate::optimization::real_performance_analyzer::BottleneckSeverity::Critical => "CRITICAL",
                crate::optimization::real_performance_analyzer::BottleneckSeverity::High => "HIGH    ",
                crate::optimization::real_performance_analyzer::BottleneckSeverity::Medium => "MEDIUM  ",
                crate::optimization::real_performance_analyzer::BottleneckSeverity::Low => "LOW     ",
                crate::optimization::real_performance_analyzer::BottleneckSeverity::Minimal => "MINIMAL ",
            };
            
            report.push_str(&format!(
                "{:<18} {} {:<18} {:>6.1}% {:>7.1}ms\n",
                format!("{:?}", bottleneck.bottleneck_type),
                severity_str,
                bottleneck.location.phase,
                bottleneck.impact_percentage,
                bottleneck.time_spent.as_millis()
            ));
        }
        report.push_str("\n");
    }
    
    // Top Recommendations Table
    if !result.recommendations.is_empty() {
        report.push_str("TOP OPTIMIZATION RECOMMENDATIONS\n");
        report.push_str("───────────────────────────────────────────────────────────────\n");
        report.push_str("Priority  Category           Improvement%  Effort(h)  Title\n");
        report.push_str("───────────────────────────────────────────────────────────────\n");
        
        for rec in result.recommendations.iter().take(10) {
            report.push_str(&format!(
                "{:>8}  {:<17} {:>10.1}% {:>8.1}  {}\n",
                rec.priority,
                format!("{:?}", rec.category),
                rec.expected_improvement.compilation_time_reduction + rec.expected_improvement.runtime_performance_gain,
                rec.effort_estimate.time_hours,
                if rec.title.len() > 25 { &rec.title[..25] } else { &rec.title }
            ));
        }
        report.push_str("\n");
    }
    
    // Resource Metrics Table
    report.push_str("DETAILED RESOURCE METRICS\n");
    report.push_str("───────────────────────────────────────────────────────────────\n");
    report.push_str(&format!("Peak Memory Usage:       {:>10.1} MB\n", result.detailed_metrics.resource_metrics.peak_memory_usage as f64 / (1024.0 * 1024.0)));
    report.push_str(&format!("Average Memory Usage:    {:>10.1} MB\n", result.detailed_metrics.resource_metrics.average_memory_usage as f64 / (1024.0 * 1024.0)));
    report.push_str(&format!("Peak CPU Usage:          {:>10.1}%\n", result.detailed_metrics.resource_metrics.peak_cpu_usage));
    report.push_str(&format!("Average CPU Usage:       {:>10.1}%\n", result.detailed_metrics.resource_metrics.average_cpu_usage));
    report.push_str(&format!("Disk Read Operations:    {:>10}\n", result.detailed_metrics.resource_metrics.disk_reads));
    report.push_str(&format!("Disk Write Operations:   {:>10}\n", result.detailed_metrics.resource_metrics.disk_writes));
    report.push_str(&format!("Context Switches:        {:>10}\n", result.detailed_metrics.resource_metrics.context_switches));
    report.push_str(&format!("Page Faults:             {:>10}\n", result.detailed_metrics.resource_metrics.page_faults));
    report.push_str("\n");
    
    // Cache Performance Table
    report.push_str("CACHE PERFORMANCE METRICS\n");
    report.push_str("───────────────────────────────────────────────────────────────\n");
    report.push_str(&format!("L1 Cache Hit Rate:       {:>10.1}%\n", result.detailed_metrics.cache_performance.l1_hit_rate * 100.0));
    report.push_str(&format!("L2 Cache Hit Rate:       {:>10.1}%\n", result.detailed_metrics.cache_performance.l2_hit_rate * 100.0));
    report.push_str(&format!("L3 Cache Hit Rate:       {:>10.1}%\n", result.detailed_metrics.cache_performance.l3_hit_rate * 100.0));
    report.push_str(&format!("Total Cache Misses:      {:>10}\n", result.detailed_metrics.cache_performance.cache_misses));
    report.push_str(&format!("Cache Miss Penalty:      {:>10}ns\n", result.detailed_metrics.cache_performance.cache_miss_penalty.as_nanos()));
    report.push_str("\n");
    
    report.push_str("═══════════════════════════════════════════════════════════════\n");
    report.push_str("Generated by CURSED Compiler Performance Analysis System\n");
    
    Ok(report)
}

fn generate_benchmark_report(
    results: &HashMap<OptimizationLevel, crate::optimization::enhanced_benchmarking::EnhancedBenchmarkResult>,
    previous: Option<&HashMap<OptimizationLevel, crate::optimization::enhanced_benchmarking::EnhancedBenchmarkResult>>,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut report = String::new();
    
    report.push_str("# CURSED Compiler Benchmark Report\n\n");
    report.push_str(&format!("**Generated:** {}\n", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));
    report.push_str(&format!("**Optimization Levels Tested:** {}\n\n", results.len()));
    
    // Executive Summary
    report.push_str("## 📊 Executive Summary\n\n");
    
    // Find best and worst performing levels
    let mut performance_ranking: Vec<_> = results.iter()
        .map(|(level, result)| {
            let avg_time = result.level_results.values()
                .map(|lr| lr.statistics.mean_time.as_millis())
                .sum::<u128>() / result.level_results.len() as u128;
            (*level, avg_time)
        })
        .collect();
    
    performance_ranking.sort_by_key(|(_, time)| *time);
    
    if let Some((best_level, best_time)) = performance_ranking.first() {
        if let Some((worst_level, worst_time)) = performance_ranking.last() {
            let speedup = *worst_time as f64 / *best_time as f64;
            report.push_str(&format!("- **Best Performance:** {:?} ({:.1}ms average)\n", best_level, best_time));
            report.push_str(&format!("- **Worst Performance:** {:?} ({:.1}ms average)\n", worst_level, worst_time));
            report.push_str(&format!("- **Max Speedup:** {:.2}x faster\n\n", speedup));
        }
    }
    
    // Performance Comparison Table
    report.push_str("## ⚡ Performance Comparison\n\n");
    report.push_str("| Optimization Level | Mean Time | Std Dev | P95 Time | Memory Peak | Binary Size | Efficiency |\n");
    report.push_str("|-------------------|-----------|---------|----------|-------------|-------------|------------|\n");
    
    for (level, time) in &performance_ranking {
        if let Some(result) = results.get(level) {
            let level_result = result.level_results.values().next().unwrap(); // Get first result for stats
            report.push_str(&format!(
                "| {:?} | {:.1}ms | {:.1}ms | {:.1}ms | {:.1}MB | {:.1}KB | {:.1}% |\n",
                level,
                level_result.statistics.mean_time.as_millis(),
                level_result.statistics.std_deviation.as_millis(),
                level_result.statistics.p95_time.as_millis(),
                level_result.resource_usage.memory.peak_usage as f64 / (1024.0 * 1024.0),
                level_result.quality_metrics.binary_size.mean_size as f64 / 1024.0,
                level_result.quality_metrics.optimization_effectiveness * 100.0
            ));
        }
    }
    report.push_str("\n");
    
    // Detailed Analysis per Level
    report.push_str("## 🔍 Detailed Analysis by Optimization Level\n\n");
    
    for (level, result) in results {
        report.push_str(&format!("### Optimization Level {:?}\n\n", level));
        
        if let Some(level_result) = result.level_results.get(level) {
            report.push_str("**Performance Statistics:**\n");
            report.push_str(&format!("- Mean compilation time: {:.2}ms\n", level_result.statistics.mean_time.as_millis()));
            report.push_str(&format!("- Median compilation time: {:.2}ms\n", level_result.statistics.median_time.as_millis()));
            report.push_str(&format!("- Standard deviation: {:.2}ms\n", level_result.statistics.std_deviation.as_millis()));
            report.push_str(&format!("- 95th percentile: {:.2}ms\n", level_result.statistics.p95_time.as_millis()));
            report.push_str(&format!("- 99th percentile: {:.2}ms\n", level_result.statistics.p99_time.as_millis()));
            report.push_str(&format!("- Coefficient of variation: {:.3}\n", level_result.statistics.coefficient_of_variation));
            report.push_str(&format!("- Sample size adequacy: {}\n\n", if level_result.statistics.significance_indicators.sample_size_adequacy { "✅" } else { "❌" }));
            
            report.push_str("**Resource Usage:**\n");
            report.push_str(&format!("- Peak memory: {:.1}MB\n", level_result.resource_usage.memory.peak_usage as f64 / (1024.0 * 1024.0)));
            report.push_str(&format!("- Average memory: {:.1}MB\n", level_result.resource_usage.memory.average_usage as f64 / (1024.0 * 1024.0)));
            report.push_str(&format!("- CPU utilization: {:.1}%\n", level_result.resource_usage.cpu.utilization_percentage));
            report.push_str(&format!("- Context switches: {}\n", level_result.resource_usage.cpu.context_switches));
            report.push_str(&format!("- Cache misses: {}\n\n", level_result.resource_usage.cpu.cache_misses));
            
            report.push_str("**Quality Metrics:**\n");
            report.push_str(&format!("- Binary size: {:.1}KB (avg)\n", level_result.quality_metrics.binary_size.mean_size as f64 / 1024.0));
            report.push_str(&format!("- Optimization effectiveness: {:.1}%\n", level_result.quality_metrics.optimization_effectiveness * 100.0));
            report.push_str(&format!("- Code quality score: {:.1}%\n", level_result.quality_metrics.code_quality_score * 100.0));
            report.push_str(&format!("- Resource efficiency: {:.1}%\n\n", level_result.quality_metrics.performance_characteristics.resource_efficiency * 100.0));
            
            // Phase breakdown
            if !level_result.phase_breakdown.is_empty() {
                report.push_str("**Phase Breakdown:**\n");
                for (phase, duration) in &level_result.phase_breakdown {
                    report.push_str(&format!("- {}: {:.1}ms\n", phase, duration.as_millis()));
                }
                report.push_str("\n");
            }
        }
    }
    
    // Statistical Analysis
    report.push_str("## 📈 Statistical Analysis\n\n");
    
    if let Some(first_result) = results.values().next() {
        report.push_str(&format!("**Overall Confidence:** {:.1}%\n", first_result.statistical_summary.overall_confidence * 100.0));
        report.push_str(&format!("**Performance Spread:** {:.1}%\n", first_result.statistical_summary.performance_spread * 100.0));
        report.push_str(&format!("**Optimization Variance:** {:.1}%\n", first_result.statistical_summary.optimization_variance * 100.0));
        
        if first_result.statistical_summary.power_analysis.power_adequate {
            report.push_str("- ✅ Statistical power is adequate\n");
        } else {
            report.push_str("- ⚠️ Statistical power may be inadequate\n");
        }
        report.push_str(&format!("- Statistical power: {:.1}%\n", first_result.statistical_summary.power_analysis.statistical_power * 100.0));
        report.push_str(&format!("- Effect size: {:.2}\n", first_result.statistical_summary.power_analysis.effect_size));
        report.push_str(&format!("- Required sample size: {}\n", first_result.statistical_summary.power_analysis.required_sample_size));
        report.push_str(&format!("- Observed sample size: {}\n\n", first_result.statistical_summary.power_analysis.observed_sample_size));
    }
    
    // Comparison with Previous Results
    if let Some(prev_results) = previous {
        report.push_str("## 🔄 Comparison with Previous Results\n\n");
        
        for (level, current_result) in results {
            if let Some(prev_result) = prev_results.get(level) {
                if let (Some(current_lr), Some(prev_lr)) = (current_result.level_results.get(level), prev_result.level_results.get(level)) {
                    let current_time = current_lr.statistics.mean_time.as_millis();
                    let prev_time = prev_lr.statistics.mean_time.as_millis();
                    
                    let change_percent = if prev_time > 0 {
                        ((current_time as f64 - prev_time as f64) / prev_time as f64) * 100.0
                    } else {
                        0.0
                    };
                    
                    let change_indicator = if change_percent > 5.0 {
                        "📈 Regression"
                    } else if change_percent < -5.0 {
                        "📉 Improvement"
                    } else {
                        "➡️ Stable"
                    };
                    
                    report.push_str(&format!("**{:?}:** {} ({:+.1}%)\n", level, change_indicator, change_percent));
                }
            }
        }
        report.push_str("\n");
    }
    
    // Recommendations
    report.push_str("## 💡 Recommendations\n\n");
    
    if let Some(first_result) = results.values().next() {
        for (i, rec) in first_result.recommendations.iter().enumerate() {
            let priority_emoji = match rec.priority {
                8..=10 => "🔴",
                6..=7 => "🟠",
                4..=5 => "🟡",
                _ => "🟢",
            };
            
            report.push_str(&format!("{}. {} **{}**\n", i + 1, priority_emoji, rec.description));
            report.push_str(&format!("   - Expected impact: {:.1}%\n", rec.expected_impact * 100.0));
            report.push_str(&format!("   - Action: {}\n\n", rec.action));
        }
    }
    
    // Environment Information
    report.push_str("## 🖥️ Test Environment\n\n");
    
    if let Some(first_result) = results.values().next() {
        report.push_str(&format!("- **Operating System:** {}\n", first_result.environment.os));
        report.push_str(&format!("- **CPU:** {} ({} cores, {} threads)\n", 
            first_result.environment.cpu_info.model,
            first_result.environment.cpu_info.cores,
            first_result.environment.cpu_info.threads
        ));
        report.push_str(&format!("- **CPU Frequency:** {}MHz\n", first_result.environment.cpu_info.frequency_mhz));
        report.push_str(&format!("- **Total RAM:** {:.1}GB\n", first_result.environment.memory_info.total_ram as f64 / (1024.0 * 1024.0 * 1024.0)));
        report.push_str(&format!("- **Available RAM:** {:.1}GB\n", first_result.environment.memory_info.available_ram as f64 / (1024.0 * 1024.0 * 1024.0)));
        report.push_str(&format!("- **Compiler Version:** {}\n", first_result.environment.compiler_version));
        report.push_str(&format!("- **LLVM Version:** {}\n", first_result.environment.llvm_version));
        
        // System load during benchmark
        report.push_str(&format!("- **System Load (1m/5m/15m):** {:.2}/{:.2}/{:.2}\n", 
            first_result.environment.system_load.load_average_1min,
            first_result.environment.system_load.load_average_5min,
            first_result.environment.system_load.load_average_15min
        ));
        report.push_str(&format!("- **CPU Usage during test:** {:.1}%\n", first_result.environment.system_load.cpu_usage_percentage));
        report.push_str(&format!("- **Memory Usage during test:** {:.1}%\n\n", first_result.environment.system_load.memory_usage_percentage));
    }
    
    report.push_str("---\n\n");
    report.push_str("*Generated by CURSED Compiler Enhanced Benchmarking System*\n");
    
    Ok(report)
}

fn generate_profiling_report(
    result: &crate::optimization::real_compilation_profiler::ProfileResult,
    flamegraph: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut report = String::new();
    
    report.push_str("# CURSED Compilation Profiling Report\n\n");
    report.push_str(&format!("**Profile ID:** `{}`\n", result.profile_id));
    report.push_str(&format!("**Source File:** `{}`\n", result.source_file));
    report.push_str(&format!("**Optimization Level:** {:?}\n", result.optimization_level));
    report.push_str(&format!("**Profiling Date:** {}\n", result.timestamp.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()));
    report.push_str(&format!("**Total Compilation Time:** {:.2}ms\n\n", result.total_compilation_time.as_millis()));
    
    // Executive Summary
    report.push_str("## 📊 Executive Summary\n\n");
    report.push_str(&format!("- **Total Compilation Time:** {:.2}ms\n", result.total_compilation_time.as_millis()));
    report.push_str(&format!("- **CPU Efficiency:** {:.1}%\n", result.resource_usage.cpu_profile.average_utilization));
    report.push_str(&format!("- **Memory Efficiency:** {:.1}%\n", result.resource_usage.memory_profile.memory_efficiency * 100.0));
    report.push_str(&format!("- **Peak Memory Usage:** {:.1}MB\n", result.resource_usage.memory_profile.peak_usage as f64 / (1024.0 * 1024.0)));
    report.push_str(&format!("- **Critical Bottlenecks:** {}\n", result.bottleneck_analysis.identified_bottlenecks.len()));
    report.push_str(&format!("- **Optimization Opportunities:** {}\n\n", result.optimization_opportunities.len()));
    
    // Performance Grade
    let overall_efficiency = (result.performance_characteristics.efficiency_metrics.cpu_efficiency + 
                             result.performance_characteristics.efficiency_metrics.memory_efficiency +
                             result.performance_characteristics.efficiency_metrics.io_efficiency) / 3.0;
    
    let grade = match overall_efficiency {
        eff if eff >= 0.9 => "🟢 Excellent (A)",
        eff if eff >= 0.8 => "🟡 Good (B)", 
        eff if eff >= 0.7 => "🟠 Fair (C)",
        eff if eff >= 0.6 => "🔴 Poor (D)",
        _ => "❌ Critical (F)",
    };
    report.push_str(&format!("**Performance Grade:** {}\n\n", grade));
    
    // Phase Breakdown
    report.push_str("## ⏱️ Compilation Phase Breakdown\n\n");
    
    // Frontend phases
    if !result.phase_breakdown.frontend_phases.is_empty() {
        report.push_str("### Frontend Phases\n\n");
        report.push_str("| Phase | Duration | CPU Time | Memory Peak | Efficiency | Parallelism |\n");
        report.push_str("|-------|----------|----------|-------------|------------|-------------|\n");
        
        for phase in &result.phase_breakdown.frontend_phases {
            report.push_str(&format!(
                "| {} | {:.1}ms | {:.1}ms | {:.1}MB | {:.1}% | {:.1}x |\n",
                phase.phase_name,
                phase.duration.as_millis(),
                phase.cpu_time.as_millis(),
                phase.memory_peak as f64 / (1024.0 * 1024.0),
                phase.efficiency_score * 100.0,
                phase.parallelism_factor
            ));
        }
        report.push_str("\n");
    }
    
    // Backend phases
    if !result.phase_breakdown.backend_phases.is_empty() {
        report.push_str("### Backend Phases\n\n");
        report.push_str("| Phase | Duration | CPU Time | Memory Peak | Efficiency | Parallelism |\n");
        report.push_str("|-------|----------|----------|-------------|------------|-------------|\n");
        
        for phase in &result.phase_breakdown.backend_phases {
            report.push_str(&format!(
                "| {} | {:.1}ms | {:.1}ms | {:.1}MB | {:.1}% | {:.1}x |\n",
                phase.phase_name,
                phase.duration.as_millis(),
                phase.cpu_time.as_millis(),
                phase.memory_peak as f64 / (1024.0 * 1024.0),
                phase.efficiency_score * 100.0,
                phase.parallelism_factor
            ));
        }
        report.push_str("\n");
    }
    
    // Optimization phases
    if !result.phase_breakdown.optimization_phases.is_empty() {
        report.push_str("### Optimization Phases\n\n");
        report.push_str("| Phase | Duration | CPU Time | Memory Peak | Efficiency | Parallelism |\n");
        report.push_str("|-------|----------|----------|-------------|------------|-------------|\n");
        
        for phase in &result.phase_breakdown.optimization_phases {
            report.push_str(&format!(
                "| {} | {:.1}ms | {:.1}ms | {:.1}MB | {:.1}% | {:.1}x |\n",
                phase.phase_name,
                phase.duration.as_millis(),
                phase.cpu_time.as_millis(),
                phase.memory_peak as f64 / (1024.0 * 1024.0),
                phase.efficiency_score * 100.0,
                phase.parallelism_factor
            ));
        }
        report.push_str("\n");
    }
    
    // Resource Usage Analysis
    report.push_str("## 💾 Resource Usage Analysis\n\n");
    
    report.push_str("### CPU Usage\n");
    report.push_str(&format!("- **Total CPU Time:** {:.2}ms\n", result.resource_usage.cpu_profile.total_cpu_time.as_millis()));
    report.push_str(&format!("- **User Time:** {:.2}ms\n", result.resource_usage.cpu_profile.user_time.as_millis()));
    report.push_str(&format!("- **System Time:** {:.2}ms\n", result.resource_usage.cpu_profile.system_time.as_millis()));
    report.push_str(&format!("- **Peak Utilization:** {:.1}%\n", result.resource_usage.cpu_profile.peak_utilization));
    report.push_str(&format!("- **Average Utilization:** {:.1}%\n", result.resource_usage.cpu_profile.average_utilization));
    report.push_str(&format!("- **Context Switches:** {}\n", result.resource_usage.cpu_profile.context_switches));
    report.push_str(&format!("- **Thread Efficiency:** {:.1}%\n\n", result.resource_usage.cpu_profile.thread_usage.thread_efficiency * 100.0));
    
    report.push_str("### Memory Usage\n");
    report.push_str(&format!("- **Peak Usage:** {:.1}MB\n", result.resource_usage.memory_profile.peak_usage as f64 / (1024.0 * 1024.0)));
    report.push_str(&format!("- **Average Usage:** {:.1}MB\n", result.resource_usage.memory_profile.average_usage as f64 / (1024.0 * 1024.0)));
    report.push_str(&format!("- **Allocations:** {}\n", result.resource_usage.memory_profile.allocation_count));
    report.push_str(&format!("- **Deallocations:** {}\n", result.resource_usage.memory_profile.deallocation_count));
    report.push_str(&format!("- **GC Collections:** {}\n", result.resource_usage.memory_profile.gc_collections));
    report.push_str(&format!("- **GC Time:** {:.2}ms\n", result.resource_usage.memory_profile.gc_time.as_millis()));
    report.push_str(&format!("- **Memory Efficiency:** {:.1}%\n", result.resource_usage.memory_profile.memory_efficiency * 100.0));
    report.push_str(&format!("- **Fragmentation Ratio:** {:.1}%\n\n", result.resource_usage.memory_profile.fragmentation_ratio * 100.0));
    
    report.push_str("### I/O Usage\n");
    report.push_str(&format!("- **Total Read:** {:.1}KB\n", result.resource_usage.io_profile.total_read_bytes as f64 / 1024.0));
    report.push_str(&format!("- **Total Written:** {:.1}KB\n", result.resource_usage.io_profile.total_written_bytes as f64 / 1024.0));
    report.push_str(&format!("- **Read Operations:** {}\n", result.resource_usage.io_profile.read_operations));
    report.push_str(&format!("- **Write Operations:** {}\n", result.resource_usage.io_profile.write_operations));
    report.push_str(&format!("- **I/O Wait Time:** {:.2}ms\n", result.resource_usage.io_profile.io_wait_time.as_millis()));
    report.push_str(&format!("- **Bandwidth Utilization:** {:.1}%\n\n", result.resource_usage.io_profile.bandwidth_utilization * 100.0));
    
    // Cache Performance
    report.push_str("### Cache Performance\n");
    report.push_str(&format!("- **L1 Cache Hit Rate:** {:.1}%\n", result.resource_usage.cache_profile.l1_cache_stats.hit_rate * 100.0));
    report.push_str(&format!("- **L2 Cache Hit Rate:** {:.1}%\n", result.resource_usage.cache_profile.l2_cache_stats.hit_rate * 100.0));
    report.push_str(&format!("- **L3 Cache Hit Rate:** {:.1}%\n", result.resource_usage.cache_profile.l3_cache_stats.hit_rate * 100.0));
    report.push_str(&format!("- **Cache Miss Penalty:** {}ns\n\n", result.resource_usage.cache_profile.cache_miss_penalty.as_nanos()));
    
    // Bottleneck Analysis
    if !result.bottleneck_analysis.identified_bottlenecks.is_empty() {
        report.push_str("## 🚫 Performance Bottlenecks\n\n");
        
        for (i, bottleneck) in result.bottleneck_analysis.identified_bottlenecks.iter().enumerate() {
            report.push_str(&format!("### Bottleneck #{}: {}\n\n", i + 1, bottleneck.phase_name));
            report.push_str(&format!("- **Type:** {}\n", bottleneck.bottleneck_type));
            report.push_str(&format!("- **Severity:** {:.1}%\n", bottleneck.severity));
            report.push_str(&format!("- **Impact Duration:** {:.2}ms\n", bottleneck.impact_duration.as_millis()));
            report.push_str(&format!("- **Description:** {}\n", bottleneck.description));
            report.push_str(&format!("- **Root Cause:** {}\n\n", bottleneck.root_cause));
            
            if !bottleneck.suggested_solutions.is_empty() {
                report.push_str("**Suggested Solutions:**\n");
                for solution in &bottleneck.suggested_solutions {
                    report.push_str(&format!("- {}\n", solution));
                }
                report.push_str("\n");
            }
        }
    }
    
    // Optimization Opportunities
    if !result.optimization_opportunities.is_empty() {
        report.push_str("## 💡 Optimization Opportunities\n\n");
        
        for (i, opportunity) in result.optimization_opportunities.iter().enumerate() {
            report.push_str(&format!("### Opportunity #{}: {}\n\n", i + 1, opportunity.description));
            report.push_str(&format!("- **Type:** {:?}\n", opportunity.opportunity_type));
            report.push_str(&format!("- **Potential Time Reduction:** {:.1}%\n", opportunity.potential_improvement.time_reduction_percentage));
            report.push_str(&format!("- **Resource Efficiency Gain:** {:.1}%\n", opportunity.potential_improvement.resource_efficiency_gain));
            report.push_str(&format!("- **Confidence Level:** {:.1}%\n", opportunity.confidence_level * 100.0));
            report.push_str(&format!("- **Implementation Complexity:** {:?}\n", opportunity.implementation_complexity));
            
            if !opportunity.related_phases.is_empty() {
                report.push_str(&format!("- **Related Phases:** {}\n", opportunity.related_phases.join(", ")));
            }
            
            if !opportunity.suggested_actions.is_empty() {
                report.push_str("\n**Suggested Actions:**\n");
                for action in &opportunity.suggested_actions {
                    report.push_str(&format!("- {}\n", action));
                }
            }
            report.push_str("\n");
        }
    }
    
    // Performance Characteristics
    report.push_str("## 📈 Performance Characteristics\n\n");
    
    report.push_str("### Scalability Analysis\n");
    report.push_str(&format!("- **CPU Scalability:** {:.1}%\n", result.performance_characteristics.scalability_analysis.cpu_scalability * 100.0));
    report.push_str(&format!("- **Memory Scalability:** {:.1}%\n", result.performance_characteristics.scalability_analysis.memory_scalability * 100.0));
    report.push_str(&format!("- **I/O Scalability:** {:.1}%\n", result.performance_characteristics.scalability_analysis.io_scalability * 100.0));
    report.push_str(&format!("- **Parallel Efficiency:** {:.1}%\n", result.performance_characteristics.scalability_analysis.parallel_efficiency * 100.0));
    report.push_str(&format!("- **Bottleneck Factor:** {:.1}%\n\n", result.performance_characteristics.scalability_analysis.bottleneck_factor * 100.0));
    
    report.push_str("### Efficiency Metrics\n");
    report.push_str(&format!("- **CPU Efficiency:** {:.1}%\n", result.performance_characteristics.efficiency_metrics.cpu_efficiency * 100.0));
    report.push_str(&format!("- **Memory Efficiency:** {:.1}%\n", result.performance_characteristics.efficiency_metrics.memory_efficiency * 100.0));
    report.push_str(&format!("- **I/O Efficiency:** {:.1}%\n", result.performance_characteristics.efficiency_metrics.io_efficiency * 100.0));
    report.push_str(&format!("- **Cache Efficiency:** {:.1}%\n", result.performance_characteristics.efficiency_metrics.cache_efficiency * 100.0));
    report.push_str(&format!("- **Overall Efficiency:** {:.1}%\n\n", result.performance_characteristics.efficiency_metrics.overall_efficiency * 100.0));
    
    report.push_str("### Resource Utilization\n");
    report.push_str(&format!("- **CPU Utilization:** {:.1}%\n", result.performance_characteristics.resource_utilization.cpu_utilization));
    report.push_str(&format!("- **Memory Utilization:** {:.1}%\n", result.performance_characteristics.resource_utilization.memory_utilization));
    report.push_str(&format!("- **I/O Utilization:** {:.1}%\n", result.performance_characteristics.resource_utilization.io_utilization));
    report.push_str(&format!("- **Cache Utilization:** {:.1}%\n", result.performance_characteristics.resource_utilization.cache_utilization));
    report.push_str(&format!("- **Utilization Balance:** {:.1}%\n\n", result.performance_characteristics.resource_utilization.utilization_balance * 100.0));
    
    // Comparison Analysis
    if let Some(ref comparison) = result.comparison_analysis {
        report.push_str("## 🔄 Comparison Analysis\n\n");
        
        report.push_str(&format!("- **Performance Delta:** {:+.1}%\n", comparison.baseline_comparison.performance_delta * 100.0));
        report.push_str(&format!("- **Regression Detected:** {}\n", if comparison.baseline_comparison.regression_detected { "⚠️ Yes" } else { "✅ No" }));
        
        if !comparison.baseline_comparison.improvement_areas.is_empty() {
            report.push_str("- **Improvement Areas:**\n");
            for area in &comparison.baseline_comparison.improvement_areas {
                report.push_str(&format!("  - {}\n", area));
            }
        }
        
        report.push_str(&format!("- **Trend Direction:** {}\n", comparison.trend_analysis.trend_direction));
        report.push_str(&format!("- **Trend Strength:** {:.1}%\n", comparison.trend_analysis.trend_strength * 100.0));
        report.push_str(&format!("- **Prediction Confidence:** {:.1}%\n\n", comparison.trend_analysis.prediction_confidence * 100.0));
    }
    
    // Flamegraph section
    if flamegraph {
        report.push_str("## 🔥 Flamegraph Analysis\n\n");
        report.push_str("*Note: Flamegraph data would be generated here in a real implementation.*\n");
        report.push_str("*This would include interactive SVG flamegraphs showing call stack profiling data.*\n\n");
    }
    
    // Detailed Metrics
    report.push_str("## 📊 Detailed Metrics\n\n");
    
    report.push_str("### Compiler-Specific Metrics\n");
    report.push_str(&format!("- **Lines Compiled:** {}\n", result.detailed_metrics.compiler_metrics.compilation_statistics.lines_compiled));
    report.push_str(&format!("- **Functions Compiled:** {}\n", result.detailed_metrics.compiler_metrics.compilation_statistics.functions_compiled));
    report.push_str(&format!("- **Optimizations Applied:** {}\n", result.detailed_metrics.compiler_metrics.compilation_statistics.optimizations_applied));
    report.push_str(&format!("- **Errors Encountered:** {}\n\n", result.detailed_metrics.compiler_metrics.compilation_statistics.errors_encountered));
    
    report.push_str("### Instruction Metrics\n");
    report.push_str(&format!("- **Total Instructions:** {}\n", result.detailed_metrics.instruction_metrics.instruction_count));
    report.push_str(&format!("- **Instruction Efficiency:** {:.1}%\n\n", result.detailed_metrics.instruction_metrics.instruction_efficiency * 100.0));
    
    report.push_str("### System Metrics\n");
    report.push_str(&format!("- **System Load:** {:.1}%\n", result.detailed_metrics.system_metrics.system_load * 100.0));
    report.push_str(&format!("- **Memory Pressure:** {:.1}%\n", result.detailed_metrics.system_metrics.memory_pressure * 100.0));
    report.push_str(&format!("- **I/O Pressure:** {:.1}%\n\n", result.detailed_metrics.system_metrics.io_pressure * 100.0));
    
    report.push_str("---\n\n");
    report.push_str("*Generated by CURSED Compiler Real Compilation Profiler*\n");
    
    Ok(report)
}

fn load_benchmark_results(file: &str) -> Result<Option<HashMap<OptimizationLevel, crate::optimization::enhanced_benchmarking::EnhancedBenchmarkResult>>, Box<dyn std::error::Error>> {
    use std::fs;
    use std::path::Path;
    
    if !Path::new(file).exists() {
        return Ok(None);
    }
    
    let content = fs::read_to_string(file)?;
    let results: HashMap<OptimizationLevel, crate::optimization::enhanced_benchmarking::EnhancedBenchmarkResult> = 
        serde_json::from_str(&content)?;
    
    Ok(Some(results))
}

fn print_benchmark_summary(results: &HashMap<OptimizationLevel, crate::optimization::enhanced_benchmarking::EnhancedBenchmarkResult>) {
    println!("\n📊 Benchmark Summary:");
    
    if results.is_empty() {
        println!("   No benchmark results available");
        return;
    }
    
    // Find best and worst performing levels
    let mut performance_data: Vec<_> = results.iter()
        .filter_map(|(level, result)| {
            result.level_results.get(level).map(|lr| {
                (*level, lr.statistics.mean_time.as_millis())
            })
        })
        .collect();
    
    performance_data.sort_by_key(|(_, time)| *time);
    
    if let Some((best_level, best_time)) = performance_data.first() {
        if let Some((worst_level, worst_time)) = performance_data.last() {
            let speedup = *worst_time as f64 / *best_time as f64;
            
            println!("   📈 Performance Rankings:");
            for (i, (level, time)) in performance_data.iter().enumerate() {
                let rank_emoji = match i {
                    0 => "🥇",
                    1 => "🥈", 
                    2 => "🥉",
                    _ => "📊",
                };
                println!("      {} {:?}: {:.1}ms", rank_emoji, level, time);
            }
            
            println!("\n   🏆 Best Performance: {:?} ({:.1}ms)", best_level, best_time);
            println!("   🐌 Worst Performance: {:?} ({:.1}ms)", worst_level, worst_time);
            println!("   ⚡ Maximum Speedup: {:.2}x", speedup);
            
            // Performance improvement analysis
            if speedup > 2.0 {
                println!("   ✅ Significant optimization impact detected");
            } else if speedup > 1.5 {
                println!("   ✅ Moderate optimization impact detected");
            } else {
                println!("   ⚠️  Limited optimization impact detected");
            }
        }
    }
    
    // Resource efficiency summary
    if let Some(first_result) = results.values().next() {
        println!("\n   💾 Resource Efficiency:");
        
        let mut total_memory = 0;
        let mut total_efficiency = 0.0;
        let mut count = 0;
        
        for (level, result) in results {
            if let Some(level_result) = result.level_results.get(level) {
                total_memory += level_result.resource_usage.memory.peak_usage;
                total_efficiency += level_result.quality_metrics.optimization_effectiveness;
                count += 1;
            }
        }
        
        if count > 0 {
            let avg_memory = total_memory / count;
            let avg_efficiency = total_efficiency / count as f64;
            
            println!("      Memory Usage: {:.1}MB (average)", avg_memory as f64 / (1024.0 * 1024.0));
            println!("      Optimization Effectiveness: {:.1}%", avg_efficiency * 100.0);
        }
        
        // Statistical confidence
        println!("      Statistical Confidence: {:.1}%", first_result.statistical_summary.overall_confidence * 100.0);
        
        if first_result.statistical_summary.power_analysis.power_adequate {
            println!("      ✅ Statistical power is adequate");
        } else {
            println!("      ⚠️  Statistical power may be inadequate - consider more iterations");
        }
    }
    
    // Recommendations summary
    if let Some(first_result) = results.values().next() {
        let high_priority_recs: Vec<_> = first_result.recommendations.iter()
            .filter(|rec| rec.priority >= 7)
            .collect();
        
        if !high_priority_recs.is_empty() {
            println!("\n   💡 Top Recommendations:");
            for rec in high_priority_recs.iter().take(3) {
                println!("      • {} (Impact: {:.1}%)", rec.description, rec.expected_impact * 100.0);
            }
        }
    }
}

fn print_profiling_summary(result: &crate::optimization::real_compilation_profiler::ProfileResult) {
    println!("\n📈 Profiling Summary:");
    
    // Overall performance
    println!("   🕒 Total Time: {:.2}ms", result.total_compilation_time.as_millis());
    println!("   📊 Optimization Level: {:?}", result.optimization_level);
    
    // Resource utilization
    println!("   💾 Peak Memory: {:.1}MB", result.resource_usage.memory_profile.peak_usage as f64 / (1024.0 * 1024.0));
    println!("   ⚡ CPU Utilization: {:.1}%", result.resource_usage.cpu_profile.average_utilization);
    println!("   💿 I/O Wait Time: {:.2}ms", result.resource_usage.io_profile.io_wait_time.as_millis());
    
    // Efficiency metrics
    let overall_efficiency = (result.performance_characteristics.efficiency_metrics.cpu_efficiency + 
                             result.performance_characteristics.efficiency_metrics.memory_efficiency +
                             result.performance_characteristics.efficiency_metrics.io_efficiency) / 3.0;
    
    println!("   📈 Overall Efficiency: {:.1}%", overall_efficiency * 100.0);
    
    // Phase performance
    let total_phases = result.phase_breakdown.frontend_phases.len() + 
                      result.phase_breakdown.backend_phases.len() + 
                      result.phase_breakdown.optimization_phases.len();
    
    println!("   🔧 Total Phases Profiled: {}", total_phases);
    
    // Find slowest phase
    let mut slowest_phase = None;
    let mut slowest_time = std::time::Duration::ZERO;
    
    for phase in &result.phase_breakdown.frontend_phases {
        if phase.duration > slowest_time {
            slowest_time = phase.duration;
            slowest_phase = Some(&phase.phase_name);
        }
    }
    
    for phase in &result.phase_breakdown.backend_phases {
        if phase.duration > slowest_time {
            slowest_time = phase.duration;
            slowest_phase = Some(&phase.phase_name);
        }
    }
    
    for phase in &result.phase_breakdown.optimization_phases {
        if phase.duration > slowest_time {
            slowest_time = phase.duration;
            slowest_phase = Some(&phase.phase_name);
        }
    }
    
    if let Some(phase_name) = slowest_phase {
        println!("   🐌 Slowest Phase: {} ({:.2}ms)", phase_name, slowest_time.as_millis());
    }
    
    // Cache performance
    let l1_hit_rate = result.resource_usage.cache_profile.l1_cache_stats.hit_rate * 100.0;
    println!("   🎯 L1 Cache Hit Rate: {:.1}%", l1_hit_rate);
    
    if l1_hit_rate < 90.0 {
        println!("   ⚠️  Cache performance could be improved");
    }
    
    // Bottlenecks and opportunities
    let bottleneck_count = result.bottleneck_analysis.identified_bottlenecks.len();
    let opportunity_count = result.optimization_opportunities.len();
    
    println!("   🚫 Bottlenecks Identified: {}", bottleneck_count);
    println!("   💡 Optimization Opportunities: {}", opportunity_count);
    
    if bottleneck_count > 0 {
        // Show most severe bottleneck
        if let Some(worst_bottleneck) = result.bottleneck_analysis.identified_bottlenecks.iter()
            .max_by(|a, b| a.severity.partial_cmp(&b.severity).unwrap()) {
            println!("      Most Severe: {} ({:.1}% impact)", worst_bottleneck.bottleneck_type, worst_bottleneck.severity);
        }
    }
    
    if opportunity_count > 0 {
        // Show highest impact opportunity
        if let Some(best_opportunity) = result.optimization_opportunities.iter()
            .max_by(|a, b| a.potential_improvement.time_reduction_percentage.partial_cmp(&b.potential_improvement.time_reduction_percentage).unwrap()) {
            println!("      Best Opportunity: {:.1}% potential improvement", best_opportunity.potential_improvement.time_reduction_percentage);
        }
    }
    
    // Performance grade
    let grade = match overall_efficiency {
        eff if eff >= 0.9 => "🟢 Excellent",
        eff if eff >= 0.8 => "🟡 Good",
        eff if eff >= 0.7 => "🟠 Fair", 
        eff if eff >= 0.6 => "🔴 Poor",
        _ => "❌ Critical",
    };
    println!("   🎖️  Performance Grade: {}", grade);
    
    // Parallelization potential
    let avg_parallelism: f64 = result.phase_breakdown.frontend_phases.iter()
        .chain(result.phase_breakdown.backend_phases.iter())
        .chain(result.phase_breakdown.optimization_phases.iter())
        .map(|p| p.parallelism_factor)
        .sum::<f64>() / total_phases as f64;
    
    if avg_parallelism > 2.0 {
        println!("   🔄 Good parallelization potential detected ({:.1}x average)", avg_parallelism);
    } else {
        println!("   ⚠️  Limited parallelization potential ({:.1}x average)", avg_parallelism);
    }
    
    // Comparison analysis summary
    if let Some(ref comparison) = result.comparison_analysis {
        if comparison.baseline_comparison.regression_detected {
            println!("   📉 Performance regression detected ({:+.1}%)", comparison.baseline_comparison.performance_delta * 100.0);
        } else if comparison.baseline_comparison.performance_delta > 0.05 {
            println!("   📈 Performance improvement detected ({:+.1}%)", comparison.baseline_comparison.performance_delta * 100.0);
        } else {
            println!("   ➡️  Performance is stable");
        }
    }
}
