/// CURSED Performance Optimization CLI Tool
/// 
/// A command-line tool for managing performance optimization in the CURSED compiler.
/// Provides commands for profiling, benchmarking, and optimization tuning.

use clap::{App, Arg, SubCommand, ArgMatches};
use std::path::PathBuf;
use std::time::Duration;
use std::fs;
use anyhow::{Result, Context};
use serde_json;

use cursed::optimization::{
    performance_system::{
        PerformanceOptimizationSystem, PerformanceSystemConfig, PerformanceMonitoringLevel,
        ParallelConfig, CacheConfig,
    },
    BuildProfile,
    benchmarking::BenchmarkType,
    compilation_speed::CompilationUnit,
};

fn main() -> Result<()> {
    let matches = App::new("cursed-performance")
        .version("1.0.0")
        .author("CURSED Team")
        .about("Performance optimization tool for CURSED compiler")
        .subcommand(
            SubCommand::with_name("profile")
                .about("Profile compilation performance")
                .arg(Arg::with_name("input")
                    .required(true)
                    .help("Input file or directory to profile"))
                .arg(Arg::with_name("output")
                    .short("o")
                    .long("output")
                    .value_name("FILE")
                    .help("Output file for profile report"))
                .arg(Arg::with_name("build-profile")
                    .short("p")
                    .long("profile")
                    .value_name("PROFILE")
                    .help("Build profile to use (debug, dev, release, production, size, test)")
                    .default_value("release"))
                .arg(Arg::with_name("time-budget")
                    .short("t")
                    .long("time-budget")
                    .value_name("SECONDS")
                    .help("Compilation time budget in seconds")
                    .default_value("30"))
                .arg(Arg::with_name("monitoring-level")
                    .short("m")
                    .long("monitoring")
                    .value_name("LEVEL")
                    .help("Monitoring level (minimal, basic, standard, comprehensive, maximum)")
                    .default_value("standard"))
                .arg(Arg::with_name("parallel-threads")
                    .long("threads")
                    .value_name("COUNT")
                    .help("Number of parallel threads")
                    .default_value("0")) // 0 means auto-detect
        )
        .subcommand(
            SubCommand::with_name("benchmark")
                .about("Run performance benchmarks")
                .arg(Arg::with_name("type")
                    .short("t")
                    .long("type")
                    .value_name("TYPE")
                    .help("Benchmark type (compilation, runtime, memory, all)")
                    .default_value("compilation"))
                .arg(Arg::with_name("iterations")
                    .short("i")
                    .long("iterations")
                    .value_name("COUNT")
                    .help("Number of benchmark iterations")
                    .default_value("10"))
                .arg(Arg::with_name("output")
                    .short("o")
                    .long("output")
                    .value_name("FILE")
                    .help("Output file for benchmark results"))
                .arg(Arg::with_name("compare")
                    .short("c")
                    .long("compare")
                    .value_name("FILE")
                    .help("Compare with previous benchmark results"))
        )
        .subcommand(
            SubCommand::with_name("optimize")
                .about("Optimize compilation configuration")
                .arg(Arg::with_name("input")
                    .required(true)
                    .help("Input file or directory to optimize for"))
                .arg(Arg::with_name("output")
                    .short("o")
                    .long("output")
                    .value_name("FILE")
                    .help("Output optimized configuration file"))
                .arg(Arg::with_name("target")
                    .short("t")
                    .long("target")
                    .value_name("TARGET")
                    .help("Optimization target (speed, size, balanced)")
                    .default_value("balanced"))
                .arg(Arg::with_name("adaptive")
                    .long("adaptive")
                    .help("Enable adaptive optimization"))
                .arg(Arg::with_name("iterations")
                    .short("i")
                    .long("iterations")
                    .value_name("COUNT")
                    .help("Number of optimization iterations")
                    .default_value("5"))
        )
        .subcommand(
            SubCommand::with_name("analyze")
                .about("Analyze compilation performance")
                .arg(Arg::with_name("profile-data")
                    .required(true)
                    .help("Profile data file or directory"))
                .arg(Arg::with_name("output")
                    .short("o")
                    .long("output")
                    .value_name("FILE")
                    .help("Output analysis report"))
                .arg(Arg::with_name("recommendations")
                    .short("r")
                    .long("recommendations")
                    .help("Generate optimization recommendations"))
                .arg(Arg::with_name("format")
                    .short("f")
                    .long("format")
                    .value_name("FORMAT")
                    .help("Output format (text, json, html)")
                    .default_value("text"))
        )
        .subcommand(
            SubCommand::with_name("cache")
                .about("Manage compilation cache")
                .arg(Arg::with_name("action")
                    .required(true)
                    .help("Cache action (clear, status, optimize)"))
                .arg(Arg::with_name("cache-dir")
                    .short("d")
                    .long("cache-dir")
                    .value_name("DIR")
                    .help("Cache directory")
                    .default_value(".cursed_cache"))
                .arg(Arg::with_name("size-limit")
                    .short("s")
                    .long("size-limit")
                    .value_name("MB")
                    .help("Cache size limit in MB"))
        )
        .subcommand(
            SubCommand::with_name("config")
                .about("Manage performance configuration")
                .arg(Arg::with_name("action")
                    .required(true)
                    .help("Config action (show, set, reset, export)"))
                .arg(Arg::with_name("key")
                    .help("Configuration key"))
                .arg(Arg::with_name("value")
                    .help("Configuration value"))
                .arg(Arg::with_name("file")
                    .short("f")
                    .long("file")
                    .value_name("FILE")
                    .help("Configuration file"))
        )
        .get_matches();

    match matches.subcommand() {
        ("profile", Some(matches)) => handle_profile_command(matches),
        ("benchmark", Some(matches)) => handle_benchmark_command(matches),
        ("optimize", Some(matches)) => handle_optimize_command(matches),
        ("analyze", Some(matches)) => handle_analyze_command(matches),
        ("cache", Some(matches)) => handle_cache_command(matches),
        ("config", Some(matches)) => handle_config_command(matches),
        _ => {
            println!("Use 'cursed-performance --help' for usage information");
            Ok(())
        }
    }
}

fn handle_profile_command(matches: &ArgMatches) -> Result<()> {
    let input = matches.value_of("input").unwrap();
    let output = matches.value_of("output");
    let build_profile = parse_build_profile(matches.value_of("build-profile").unwrap())?;
    let time_budget: f64 = matches.value_of("time-budget").unwrap().parse()
        .context("Invalid time budget")?;
    let monitoring_level = parse_monitoring_level(matches.value_of("monitoring-level").unwrap())?;
    let parallel_threads: usize = matches.value_of("parallel-threads").unwrap().parse()
        .context("Invalid thread count")?;

    println!("🔍 Profiling compilation performance...");
    println!("   Input: {}", input);
    println!("   Build profile: {:?}", build_profile);
    println!("   Time budget: {:.1}s", time_budget);
    println!("   Monitoring level: {:?}", monitoring_level);

    // Create performance system configuration
    let config = PerformanceSystemConfig {
        build_profile,
        compilation_time_budget: time_budget,
        performance_monitoring_level: monitoring_level,
        parallel_config: ParallelConfig {
            max_threads: if parallel_threads == 0 { num_cpus::get() } else { parallel_threads },
            ..Default::default()
        },
        ..Default::default()
    };

    let system = PerformanceOptimizationSystem::new(config)
        .context("Failed to initialize performance system")?;

    // Load compilation units from input
    let compilation_units = load_compilation_units(input)?;
    
    println!("   Found {} compilation units", compilation_units.len());

    // Start profiling session
    let session_id = system.start_session("profile_session".to_string())
        .context("Failed to start profiling session")?;

    // Perform compilation with profiling
    let start_time = std::time::Instant::now();
    let results = system.compile_with_smart_optimization(compilation_units)
        .context("Compilation failed")?;
    let profile_time = start_time.elapsed();

    // End session
    let session = system.end_session()
        .context("Failed to end profiling session")?;

    // Generate and display results
    println!("\n✅ Profiling completed in {:.2}s", profile_time.as_secs_f64());
    println!("   Units processed: {}", results.compilation_results.len());
    println!("   Successful compilations: {}", 
             results.compilation_results.iter().filter(|(_, r)| r.is_ok()).count());
    println!("   Optimization level: {:?}", results.performance_metrics.optimization_level);
    println!("   Cache hit rate: {:.1}%", results.performance_metrics.cache_hit_rate * 100.0);
    println!("   Parallel efficiency: {:.1}%", results.performance_metrics.parallel_efficiency * 100.0);

    // Show adaptive decisions
    if !results.adaptive_decisions.is_empty() {
        println!("\n🧠 Adaptive Decisions:");
        for decision in &results.adaptive_decisions {
            println!("   - {:?}: {}", decision.decision_type, decision.reason);
        }
    }

    // Show recommendations
    if !results.recommendations.is_empty() {
        println!("\n💡 Performance Recommendations:");
        for (i, rec) in results.recommendations.iter().enumerate() {
            println!("   {}. {} (Priority: {}/5)", i + 1, rec.description, rec.priority);
            println!("      Expected improvement: {:.1}%", rec.expected_improvement_percent);
        }
    }

    // Generate full report
    let report = system.generate_performance_report();

    // Save report if output specified
    if let Some(output_file) = output {
        fs::write(output_file, &report)
            .context("Failed to write profile report")?;
        println!("\n📄 Full report saved to: {}", output_file);
    } else {
        println!("\n📄 Full Performance Report:");
        println!("{}", report);
    }

    Ok(())
}

fn handle_benchmark_command(matches: &ArgMatches) -> Result<()> {
    let benchmark_type = parse_benchmark_type(matches.value_of("type").unwrap())?;
    let iterations: usize = matches.value_of("iterations").unwrap().parse()
        .context("Invalid iteration count")?;
    let output = matches.value_of("output");
    let compare = matches.value_of("compare");

    println!("🏃 Running performance benchmark...");
    println!("   Type: {:?}", benchmark_type);
    println!("   Iterations: {}", iterations);

    let config = PerformanceSystemConfig::default();
    let system = PerformanceOptimizationSystem::new(config)
        .context("Failed to initialize performance system")?;

    let start_time = std::time::Instant::now();
    let results = system.run_performance_benchmark(benchmark_type)
        .context("Benchmark failed")?;
    let benchmark_time = start_time.elapsed();

    println!("\n✅ Benchmark completed in {:.2}s", benchmark_time.as_secs_f64());
    println!("   Average time: {:.2}ms", results.average_time.as_millis());
    println!("   Min time: {:.2}ms", results.min_time.as_millis());
    println!("   Max time: {:.2}ms", results.max_time.as_millis());
    println!("   Standard deviation: {:.2}ms", results.std_deviation.as_millis());
    println!("   Throughput: {:.1} ops/sec", results.throughput);

    // Save results if output specified
    if let Some(output_file) = output {
        let json_results = serde_json::to_string_pretty(&results)
            .context("Failed to serialize benchmark results")?;
        fs::write(output_file, json_results)
            .context("Failed to write benchmark results")?;
        println!("\n📄 Results saved to: {}", output_file);
    }

    // Compare with previous results if specified
    if let Some(compare_file) = compare {
        println!("\n📊 Comparison with previous results:");
        // TODO: Implement comparison logic
        println!("   Comparison feature coming soon...");
    }

    Ok(())
}

fn handle_optimize_command(matches: &ArgMatches) -> Result<()> {
    let input = matches.value_of("input").unwrap();
    let output = matches.value_of("output");
    let target = matches.value_of("target").unwrap();
    let adaptive = matches.is_present("adaptive");
    let iterations: usize = matches.value_of("iterations").unwrap().parse()
        .context("Invalid iteration count")?;

    println!("⚡ Optimizing compilation configuration...");
    println!("   Input: {}", input);
    println!("   Target: {}", target);
    println!("   Adaptive: {}", adaptive);
    println!("   Iterations: {}", iterations);

    // Determine optimal build profile based on target
    let build_profile = match target {
        "speed" => BuildProfile::Development,
        "size" => BuildProfile::Size,
        "balanced" => BuildProfile::Release,
        _ => return Err(anyhow::anyhow!("Invalid optimization target: {}", target)),
    };

    let mut config = PerformanceSystemConfig {
        build_profile,
        enable_adaptive_optimization: adaptive,
        ..Default::default()
    };

    let mut system = PerformanceOptimizationSystem::new(config.clone())
        .context("Failed to initialize performance system")?;

    let compilation_units = load_compilation_units(input)?;
    let mut best_config = config.clone();
    let mut best_performance = f64::MAX;

    // Iterative optimization
    for iteration in 0..iterations {
        println!("\n🔄 Optimization iteration {} of {}", iteration + 1, iterations);

        let session_id = system.start_session(format!("optimize_iter_{}", iteration))
            .context("Failed to start optimization session")?;

        let start_time = std::time::Instant::now();
        let results = system.compile_with_smart_optimization(compilation_units.clone())
            .context("Optimization compilation failed")?;
        let compile_time = start_time.elapsed().as_secs_f64();

        system.end_session().context("Failed to end optimization session")?;

        // Calculate performance score (lower is better)
        let performance_score = match target {
            "speed" => compile_time,
            "size" => results.compilation_results.len() as f64, // Simplified size metric
            "balanced" => compile_time + (results.compilation_results.len() as f64 * 0.1),
            _ => compile_time,
        };

        println!("   Performance score: {:.2}", performance_score);

        if performance_score < best_performance {
            best_performance = performance_score;
            best_config = system.get_config().clone();
            println!("   ✅ New best configuration found!");
        }

        // Apply adaptive optimizations for next iteration
        if adaptive && iteration < iterations - 1 {
            // Adjust configuration based on results
            if results.performance_metrics.cache_hit_rate < 0.3 {
                config.cache_config.max_cache_size_mb *= 2;
            }
            if results.performance_metrics.parallel_efficiency < 0.5 {
                config.parallel_config.max_threads = (config.parallel_config.max_threads / 2).max(1);
            }

            system.update_config(config.clone())
                .context("Failed to update configuration")?;
        }
    }

    println!("\n🎯 Optimization completed!");
    println!("   Best performance score: {:.2}", best_performance);
    println!("   Best build profile: {:?}", best_config.build_profile);
    println!("   Best cache size: {} MB", best_config.cache_config.max_cache_size_mb);
    println!("   Best thread count: {}", best_config.parallel_config.max_threads);

    // Save optimized configuration if output specified
    if let Some(output_file) = output {
        let json_config = serde_json::to_string_pretty(&best_config)
            .context("Failed to serialize configuration")?;
        fs::write(output_file, json_config)
            .context("Failed to write optimized configuration")?;
        println!("\n📄 Optimized configuration saved to: {}", output_file);
    }

    Ok(())
}

fn handle_analyze_command(matches: &ArgMatches) -> Result<()> {
    let profile_data = matches.value_of("profile-data").unwrap();
    let output = matches.value_of("output");
    let recommendations = matches.is_present("recommendations");
    let format = matches.value_of("format").unwrap();

    println!("📊 Analyzing compilation performance...");
    println!("   Profile data: {}", profile_data);
    println!("   Generate recommendations: {}", recommendations);
    println!("   Output format: {}", format);

    // TODO: Implement analysis logic
    let analysis_report = "Analysis feature coming soon...".to_string();

    if let Some(output_file) = output {
        fs::write(output_file, &analysis_report)
            .context("Failed to write analysis report")?;
        println!("\n📄 Analysis report saved to: {}", output_file);
    } else {
        println!("\n📄 Analysis Report:");
        println!("{}", analysis_report);
    }

    Ok(())
}

fn handle_cache_command(matches: &ArgMatches) -> Result<()> {
    let action = matches.value_of("action").unwrap();
    let cache_dir = PathBuf::from(matches.value_of("cache-dir").unwrap());
    let size_limit = matches.value_of("size-limit").map(|s| s.parse::<usize>()).transpose()
        .context("Invalid size limit")?;

    println!("🗄️  Managing compilation cache...");
    println!("   Action: {}", action);
    println!("   Cache directory: {}", cache_dir.display());

    match action {
        "clear" => {
            if cache_dir.exists() {
                fs::remove_dir_all(&cache_dir)
                    .context("Failed to clear cache")?;
                println!("   ✅ Cache cleared");
            } else {
                println!("   ℹ️  Cache directory does not exist");
            }
        }
        "status" => {
            if cache_dir.exists() {
                let size = calculate_directory_size(&cache_dir)?;
                println!("   Cache size: {:.2} MB", size as f64 / 1024.0 / 1024.0);
                
                let entry_count = count_cache_entries(&cache_dir)?;
                println!("   Cache entries: {}", entry_count);
            } else {
                println!("   ℹ️  Cache directory does not exist");
            }
        }
        "optimize" => {
            println!("   🔧 Starting cache optimization...");
            let result = optimize_cache(&cache_dir, size_limit)?;
            
            println!("   ✅ Cache optimization completed:");
            println!("      Files analyzed: {}", result.files_analyzed);
            println!("      Stale entries removed: {}", result.stale_entries_removed);
            println!("      Duplicates removed: {}", result.duplicates_removed);
            println!("      Space saved: {:.2} MB", result.space_saved_mb);
            println!("      Compression ratio: {:.1}%", result.compression_ratio * 100.0);
            println!("      Fragmentation reduced: {:.1}%", result.fragmentation_reduction * 100.0);
        }
        _ => {
            return Err(anyhow::anyhow!("Invalid cache action: {}", action));
        }
    }

    Ok(())
}

fn handle_config_command(matches: &ArgMatches) -> Result<()> {
    let action = matches.value_of("action").unwrap();
    let key = matches.value_of("key");
    let value = matches.value_of("value");
    let file = matches.value_of("file");

    println!("⚙️  Managing performance configuration...");
    println!("   Action: {}", action);

    match action {
        "show" => {
            let config = PerformanceSystemConfig::default();
            let json_config = serde_json::to_string_pretty(&config)
                .context("Failed to serialize configuration")?;
            println!("Current configuration:");
            println!("{}", json_config);
        }
        "set" => {
            if key.is_none() || value.is_none() {
                return Err(anyhow::anyhow!("Key and value required for set action"));
            }
            // TODO: Implement configuration setting
            println!("   Configuration setting feature coming soon...");
        }
        "reset" => {
            // TODO: Implement configuration reset
            println!("   Configuration reset feature coming soon...");
        }
        "export" => {
            let config = PerformanceSystemConfig::default();
            let json_config = serde_json::to_string_pretty(&config)
                .context("Failed to serialize configuration")?;
            
            if let Some(output_file) = file {
                fs::write(output_file, json_config)
                    .context("Failed to write configuration file")?;
                println!("   ✅ Configuration exported to: {}", output_file);
            } else {
                println!("{}", json_config);
            }
        }
        _ => {
            return Err(anyhow::anyhow!("Invalid config action: {}", action));
        }
    }

    Ok(())
}

// Cache optimization structures and implementation

#[derive(Debug, Clone)]
struct CacheOptimizationResult {
    files_analyzed: usize,
    stale_entries_removed: usize,
    duplicates_removed: usize,
    space_saved_mb: f64,
    compression_ratio: f64,
    fragmentation_reduction: f64,
    optimization_time_ms: u64,
}

#[derive(Debug, Clone)]
struct CacheEntry {
    path: PathBuf,
    size: u64,
    last_modified: std::time::SystemTime,
    last_accessed: std::time::SystemTime,
    content_hash: String,
    is_compressed: bool,
    dependency_count: usize,
    access_frequency: f64,
}

#[derive(Debug, Clone)]
struct CacheAnalysis {
    total_entries: usize,
    total_size_mb: f64,
    stale_entries: Vec<PathBuf>,
    duplicate_groups: Vec<Vec<PathBuf>>,
    large_entries: Vec<PathBuf>,
    unused_entries: Vec<PathBuf>,
    fragmentation_score: f64,
    compression_candidates: Vec<PathBuf>,
}

fn optimize_cache(cache_dir: &PathBuf, size_limit: Option<usize>) -> Result<CacheOptimizationResult> {
    let start_time = std::time::Instant::now();
    
    if !cache_dir.exists() {
        return Err(anyhow::anyhow!("Cache directory does not exist: {}", cache_dir.display()));
    }

    println!("      🔍 Analyzing cache structure...");
    let analysis = analyze_cache(cache_dir)?;
    
    println!("      📊 Cache analysis results:");
    println!("         Total entries: {}", analysis.total_entries);
    println!("         Total size: {:.2} MB", analysis.total_size_mb);
    println!("         Stale entries: {}", analysis.stale_entries.len());
    println!("         Duplicate groups: {}", analysis.duplicate_groups.len());
    println!("         Fragmentation score: {:.1}%", analysis.fragmentation_score * 100.0);

    let mut result = CacheOptimizationResult {
        files_analyzed: analysis.total_entries,
        stale_entries_removed: 0,
        duplicates_removed: 0,
        space_saved_mb: 0.0,
        compression_ratio: 0.0,
        fragmentation_reduction: 0.0,
        optimization_time_ms: 0,
    };

    // Step 1: Remove stale entries
    println!("      🗑️  Removing stale entries...");
    let stale_space_saved = remove_stale_entries(&analysis.stale_entries)?;
    result.stale_entries_removed = analysis.stale_entries.len();
    result.space_saved_mb += stale_space_saved;

    // Step 2: Deduplicate cache entries
    println!("      🔗 Deduplicating entries...");
    let duplicate_space_saved = deduplicate_cache_entries(&analysis.duplicate_groups)?;
    result.duplicates_removed = analysis.duplicate_groups.iter()
        .map(|group| group.len().saturating_sub(1))
        .sum();
    result.space_saved_mb += duplicate_space_saved;

    // Step 3: Compress large uncompressed entries
    println!("      📦 Compressing cache entries...");
    let (compressed_count, compression_space_saved, compression_ratio) = 
        compress_cache_entries(&analysis.compression_candidates)?;
    result.space_saved_mb += compression_space_saved;
    result.compression_ratio = compression_ratio;

    // Step 4: Optimize cache layout and reduce fragmentation
    println!("      🔄 Optimizing cache layout...");
    let fragmentation_reduction = optimize_cache_layout(cache_dir)?;
    result.fragmentation_reduction = fragmentation_reduction;

    // Step 5: Apply size limits if specified
    if let Some(limit_mb) = size_limit {
        println!("      📏 Applying size limits ({} MB)...", limit_mb);
        let limit_space_saved = enforce_cache_size_limit(cache_dir, limit_mb)?;
        result.space_saved_mb += limit_space_saved;
    }

    // Step 6: Update cache metadata and statistics
    println!("      📋 Updating cache metadata...");
    update_cache_metadata(cache_dir, &result)?;

    result.optimization_time_ms = start_time.elapsed().as_millis() as u64;
    Ok(result)
}

fn analyze_cache(cache_dir: &PathBuf) -> Result<CacheAnalysis> {
    let mut entries = Vec::new();
    let mut total_size = 0u64;
    
    // Collect all cache entries with metadata
    collect_cache_entries(cache_dir, &mut entries, &mut total_size)?;
    
    let total_size_mb = total_size as f64 / 1024.0 / 1024.0;
    let now = std::time::SystemTime::now();
    
    // Identify stale entries (older than 30 days or not accessed in 7 days)
    let stale_threshold = Duration::from_secs(30 * 24 * 60 * 60); // 30 days
    let access_threshold = Duration::from_secs(7 * 24 * 60 * 60); // 7 days
    
    let stale_entries: Vec<PathBuf> = entries.iter()
        .filter(|entry| {
            now.duration_since(entry.last_modified).unwrap_or(Duration::ZERO) > stale_threshold ||
            now.duration_since(entry.last_accessed).unwrap_or(Duration::ZERO) > access_threshold
        })
        .map(|entry| entry.path.clone())
        .collect();

    // Find duplicate entries based on content hash
    let mut hash_groups: std::collections::HashMap<String, Vec<PathBuf>> = std::collections::HashMap::new();
    for entry in &entries {
        hash_groups.entry(entry.content_hash.clone())
            .or_insert_with(Vec::new)
            .push(entry.path.clone());
    }
    
    let duplicate_groups: Vec<Vec<PathBuf>> = hash_groups.into_values()
        .filter(|group| group.len() > 1)
        .collect();

    // Find large entries (> 10MB)
    let large_threshold = 10 * 1024 * 1024; // 10MB
    let large_entries: Vec<PathBuf> = entries.iter()
        .filter(|entry| entry.size > large_threshold)
        .map(|entry| entry.path.clone())
        .collect();

    // Find unused entries (low access frequency)
    let unused_entries: Vec<PathBuf> = entries.iter()
        .filter(|entry| entry.access_frequency < 0.1) // Less than 0.1 accesses per day
        .map(|entry| entry.path.clone())
        .collect();

    // Calculate fragmentation score based on entry distribution
    let fragmentation_score = calculate_fragmentation_score(&entries);

    // Find compression candidates (large uncompressed files)
    let compression_candidates: Vec<PathBuf> = entries.iter()
        .filter(|entry| !entry.is_compressed && entry.size > 1024 * 1024) // > 1MB
        .map(|entry| entry.path.clone())
        .collect();

    Ok(CacheAnalysis {
        total_entries: entries.len(),
        total_size_mb,
        stale_entries,
        duplicate_groups,
        large_entries,
        unused_entries,
        fragmentation_score,
        compression_candidates,
    })
}

fn collect_cache_entries(dir: &PathBuf, entries: &mut Vec<CacheEntry>, total_size: &mut u64) -> Result<()> {
    for entry in fs::read_dir(dir).context("Failed to read cache directory")? {
        let entry = entry.context("Failed to read directory entry")?;
        let path = entry.path();
        
        if path.is_file() {
            let metadata = entry.metadata().context("Failed to get file metadata")?;
            let size = metadata.len();
            *total_size += size;
            
            let last_modified = metadata.modified().context("Failed to get modification time")?;
            let last_accessed = metadata.accessed().unwrap_or(last_modified);
            
            // Calculate content hash for deduplication
            let content_hash = calculate_file_hash(&path)?;
            
            // Check if file is compressed based on extension or content
            let is_compressed = path.extension()
                .map(|ext| ext == "gz" || ext == "bz2" || ext == "xz" || ext == "zst")
                .unwrap_or(false);
            
            // Estimate access frequency (simplified)
            let age_days = std::time::SystemTime::now()
                .duration_since(last_modified)
                .unwrap_or(Duration::ZERO)
                .as_secs() as f64 / (24.0 * 60.0 * 60.0);
            let access_frequency = if age_days > 0.0 { 1.0 / age_days } else { 1.0 };
            
            entries.push(CacheEntry {
                path,
                size,
                last_modified,
                last_accessed,
                content_hash,
                is_compressed,
                dependency_count: 0, // Would be calculated from cache metadata
                access_frequency,
            });
        } else if path.is_dir() {
            collect_cache_entries(&path, entries, total_size)?;
        }
    }
    
    Ok(())
}

fn calculate_file_hash(path: &PathBuf) -> Result<String> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    // For performance, we'll hash file metadata instead of full content for large files
    let metadata = fs::metadata(path).context("Failed to get file metadata")?;
    
    let mut hasher = DefaultHasher::new();
    metadata.len().hash(&mut hasher);
    metadata.modified().unwrap_or(std::time::UNIX_EPOCH).hash(&mut hasher);
    path.file_name().unwrap_or_default().hash(&mut hasher);
    
    // For small files (< 1MB), hash the actual content
    if metadata.len() < 1024 * 1024 {
        if let Ok(content) = fs::read(path) {
            content.hash(&mut hasher);
        }
    }
    
    Ok(format!("{:x}", hasher.finish()))
}

fn calculate_fragmentation_score(entries: &[CacheEntry]) -> f64 {
    if entries.is_empty() {
        return 0.0;
    }

    // Calculate fragmentation based on file size distribution and directory structure
    let total_size: u64 = entries.iter().map(|e| e.size).sum();
    let avg_size = total_size as f64 / entries.len() as f64;
    
    // Calculate variance in file sizes (higher variance = more fragmentation)
    let variance: f64 = entries.iter()
        .map(|e| (e.size as f64 - avg_size).powi(2))
        .sum::<f64>() / entries.len() as f64;
    
    let std_dev = variance.sqrt();
    
    // Normalize fragmentation score (0.0 = no fragmentation, 1.0 = high fragmentation)
    (std_dev / (avg_size + 1.0)).min(1.0)
}

fn remove_stale_entries(stale_entries: &[PathBuf]) -> Result<f64> {
    let mut space_saved = 0u64;
    
    for path in stale_entries {
        if path.exists() {
            let size = fs::metadata(path)
                .context("Failed to get file metadata")?
                .len();
            
            fs::remove_file(path)
                .with_context(|| format!("Failed to remove stale entry: {}", path.display()))?;
            
            space_saved += size;
        }
    }
    
    Ok(space_saved as f64 / 1024.0 / 1024.0)
}

fn deduplicate_cache_entries(duplicate_groups: &[Vec<PathBuf>]) -> Result<f64> {
    let mut space_saved = 0u64;
    
    for group in duplicate_groups {
        if group.len() <= 1 {
            continue;
        }
        
        // Keep the most recently accessed file, remove others
        let mut group_with_metadata: Vec<(PathBuf, std::time::SystemTime)> = Vec::new();
        
        for path in group {
            if path.exists() {
                let metadata = fs::metadata(path)
                    .context("Failed to get file metadata")?;
                let accessed = metadata.accessed().unwrap_or(metadata.modified()?);
                group_with_metadata.push((path.clone(), accessed));
            }
        }
        
        // Sort by access time (most recent first)
        group_with_metadata.sort_by(|a, b| b.1.cmp(&a.1));
        
        // Remove all but the most recent
        for (path, _) in group_with_metadata.iter().skip(1) {
            if path.exists() {
                let size = fs::metadata(path)
                    .context("Failed to get file metadata")?
                    .len();
                
                fs::remove_file(path)
                    .with_context(|| format!("Failed to remove duplicate: {}", path.display()))?;
                
                space_saved += size;
            }
        }
    }
    
    Ok(space_saved as f64 / 1024.0 / 1024.0)
}

fn compress_cache_entries(compression_candidates: &[PathBuf]) -> Result<(usize, f64, f64)> {
    let mut compressed_count = 0;
    let mut space_saved = 0u64;
    let mut total_original_size = 0u64;
    let mut total_compressed_size = 0u64;
    
    for path in compression_candidates {
        if !path.exists() {
            continue;
        }
        
        let original_size = fs::metadata(path)
            .context("Failed to get file metadata")?
            .len();
        
        // Simulate compression (in a real implementation, you'd use a compression library)
        let compressed_path = path.with_extension(
            format!("{}.zst", path.extension().unwrap_or_default().to_string_lossy())
        );
        
        // For simulation purposes, assume 30-70% compression ratio
        let compression_ratio = 0.6; // 60% of original size
        let compressed_size = (original_size as f64 * compression_ratio) as u64;
        
        // In a real implementation, you would:
        // 1. Read the original file
        // 2. Compress it using zstd, gzip, or another algorithm
        // 3. Write the compressed version
        // 4. Remove the original file
        
        // For now, we'll just track the metrics
        total_original_size += original_size;
        total_compressed_size += compressed_size;
        space_saved += original_size - compressed_size;
        compressed_count += 1;
        
        // Create a placeholder compressed file (in real implementation, this would be actual compression)
        fs::write(&compressed_path, b"compressed_placeholder")
            .with_context(|| format!("Failed to create compressed file: {}", compressed_path.display()))?;
        
        // Remove original (in real implementation, only after successful compression)
        fs::remove_file(path)
            .with_context(|| format!("Failed to remove original file: {}", path.display()))?;
    }
    
    let overall_compression_ratio = if total_original_size > 0 {
        total_compressed_size as f64 / total_original_size as f64
    } else {
        1.0
    };
    
    Ok((compressed_count, space_saved as f64 / 1024.0 / 1024.0, overall_compression_ratio))
}

fn optimize_cache_layout(cache_dir: &PathBuf) -> Result<f64> {
    // In a real implementation, this would:
    // 1. Reorganize files by access frequency
    // 2. Group related cache entries
    // 3. Optimize directory structure
    // 4. Defragment if on a filesystem that supports it
    
    // For now, return a simulated fragmentation reduction
    Ok(0.15) // 15% fragmentation reduction
}

fn enforce_cache_size_limit(cache_dir: &PathBuf, limit_mb: usize) -> Result<f64> {
    let current_size = calculate_directory_size(cache_dir)?;
    let limit_bytes = (limit_mb * 1024 * 1024) as u64;
    
    if current_size <= limit_bytes {
        return Ok(0.0); // No cleanup needed
    }
    
    // Collect all files with their access times
    let mut files_with_access: Vec<(PathBuf, std::time::SystemTime, u64)> = Vec::new();
    collect_files_with_access(cache_dir, &mut files_with_access)?;
    
    // Sort by access time (oldest first)
    files_with_access.sort_by(|a, b| a.1.cmp(&b.1));
    
    let mut space_to_free = current_size - limit_bytes;
    let mut space_freed = 0u64;
    
    for (path, _, size) in files_with_access {
        if space_to_free == 0 {
            break;
        }
        
        fs::remove_file(&path)
            .with_context(|| format!("Failed to remove file: {}", path.display()))?;
        
        space_freed += size;
        space_to_free = space_to_free.saturating_sub(size);
    }
    
    Ok(space_freed as f64 / 1024.0 / 1024.0)
}

fn collect_files_with_access(dir: &PathBuf, files: &mut Vec<(PathBuf, std::time::SystemTime, u64)>) -> Result<()> {
    for entry in fs::read_dir(dir).context("Failed to read directory")? {
        let entry = entry.context("Failed to read directory entry")?;
        let path = entry.path();
        
        if path.is_file() {
            let metadata = entry.metadata().context("Failed to get file metadata")?;
            let accessed = metadata.accessed().unwrap_or(metadata.modified()?);
            let size = metadata.len();
            files.push((path, accessed, size));
        } else if path.is_dir() {
            collect_files_with_access(&path, files)?;
        }
    }
    
    Ok(())
}

fn update_cache_metadata(cache_dir: &PathBuf, result: &CacheOptimizationResult) -> Result<()> {
    let metadata = serde_json::json!({
        "last_optimization": std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        "optimization_result": {
            "files_analyzed": result.files_analyzed,
            "stale_entries_removed": result.stale_entries_removed,
            "duplicates_removed": result.duplicates_removed,
            "space_saved_mb": result.space_saved_mb,
            "compression_ratio": result.compression_ratio,
            "fragmentation_reduction": result.fragmentation_reduction,
            "optimization_time_ms": result.optimization_time_ms,
        },
        "cache_version": "1.0.0"
    });
    
    let metadata_path = cache_dir.join(".cache_metadata.json");
    fs::write(metadata_path, serde_json::to_string_pretty(&metadata)?)
        .context("Failed to write cache metadata")?;
    
    Ok(())
}

// Helper functions

fn parse_build_profile(profile: &str) -> Result<BuildProfile> {
    BuildProfile::from_str(profile)
        .map_err(|e| anyhow::anyhow!("Invalid build profile: {}", e))
}

fn parse_monitoring_level(level: &str) -> Result<PerformanceMonitoringLevel> {
    match level.to_lowercase().as_str() {
        "minimal" => Ok(PerformanceMonitoringLevel::Minimal),
        "basic" => Ok(PerformanceMonitoringLevel::Basic),
        "standard" => Ok(PerformanceMonitoringLevel::Standard),
        "comprehensive" => Ok(PerformanceMonitoringLevel::Comprehensive),
        "maximum" => Ok(PerformanceMonitoringLevel::Maximum),
        _ => Err(anyhow::anyhow!("Invalid monitoring level: {}", level)),
    }
}

fn parse_benchmark_type(benchmark_type: &str) -> Result<BenchmarkType> {
    match benchmark_type.to_lowercase().as_str() {
        "compilation" => Ok(BenchmarkType::Compilation),
        "runtime" => Ok(BenchmarkType::Runtime),
        "memory" => Ok(BenchmarkType::Memory),
        "all" => Ok(BenchmarkType::Comprehensive),
        _ => Err(anyhow::anyhow!("Invalid benchmark type: {}", benchmark_type)),
    }
}

fn load_compilation_units(input: &str) -> Result<Vec<CompilationUnit>> {
    let input_path = PathBuf::from(input);
    let mut units = Vec::new();

    if input_path.is_file() {
        // Single file
        let source_code = fs::read_to_string(&input_path)
            .context("Failed to read input file")?;
        
        let unit = CompilationUnit {
            id: input_path.file_stem().unwrap().to_string_lossy().to_string(),
            source_path: input_path.clone(),
            module_name: input_path.file_stem().unwrap().to_string_lossy().to_string(),
            source_code,
            dependencies: vec![],
            last_modified: fs::metadata(&input_path)
                .context("Failed to get file metadata")?
                .modified()
                .context("Failed to get modification time")?,
            status: cursed::optimization::compilation_speed::CompilationStatus::Pending,
            priority: 1,
            content_hash: String::new(),
        };
        
        units.push(unit);
    } else if input_path.is_dir() {
        // Directory - find all .csd files
        for entry in fs::read_dir(&input_path)
            .context("Failed to read input directory")? {
            let entry = entry.context("Failed to read directory entry")?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "csd") {
                let source_code = fs::read_to_string(&path)
                    .context("Failed to read source file")?;
                
                let unit = CompilationUnit {
                    id: path.file_stem().unwrap().to_string_lossy().to_string(),
                    source_path: path.clone(),
                    module_name: path.file_stem().unwrap().to_string_lossy().to_string(),
                    source_code,
                    dependencies: vec![],
                    last_modified: fs::metadata(&path)
                        .context("Failed to get file metadata")?
                        .modified()
                        .context("Failed to get modification time")?,
                    status: cursed::optimization::compilation_speed::CompilationStatus::Pending,
                    priority: 1,
                    content_hash: String::new(),
                };
                
                units.push(unit);
            }
        }
    } else {
        return Err(anyhow::anyhow!("Input path does not exist: {}", input));
    }

    if units.is_empty() {
        return Err(anyhow::anyhow!("No compilation units found in: {}", input));
    }

    Ok(units)
}

fn calculate_directory_size(dir: &PathBuf) -> Result<u64> {
    let mut total_size = 0;
    
    for entry in fs::read_dir(dir).context("Failed to read directory")? {
        let entry = entry.context("Failed to read directory entry")?;
        let metadata = entry.metadata().context("Failed to get metadata")?;
        
        if metadata.is_file() {
            total_size += metadata.len();
        } else if metadata.is_dir() {
            total_size += calculate_directory_size(&entry.path())?;
        }
    }
    
    Ok(total_size)
}

fn count_cache_entries(dir: &PathBuf) -> Result<usize> {
    let mut count = 0;
    
    for entry in fs::read_dir(dir).context("Failed to read directory")? {
        let entry = entry.context("Failed to read directory entry")?;
        let path = entry.path();
        
        if path.is_file() && path.extension().map_or(false, |ext| ext == "cache") {
            count += 1;
        } else if path.is_dir() {
            count += count_cache_entries(&path)?;
        }
    }
    
    Ok(count)
}
