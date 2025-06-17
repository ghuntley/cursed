/// Fast Compilation CLI Tool for CURSED
/// 
/// Demonstrates the compilation speed improvements including:
/// - Incremental compilation with caching
/// - Parallel processing
/// - Performance monitoring and reporting

use std::path::PathBuf;
use std::time::Instant;
use std::fs;
use clap::{Arg, Command};
use tracing::{info, error, debug};

use cursed::optimization::{OptimizationConfig, compilation_speed::*};
use cursed::error::Result;

fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let matches = Command::new("cursed-compile-fast")
        .version("0.1.0")
        .about("Fast compilation tool for CURSED with caching and parallelization")
        .arg(
            Arg::new("input")
                .help("Input CURSED files or directories")
                .value_name("FILES")
                .num_args(1..)
                .required(true),
        )
        .arg(
            Arg::new("jobs")
                .short('j')
                .long("jobs")
                .value_name("N")
                .help("Number of parallel compilation threads")
                .default_value("auto"),
        )
        .arg(
            Arg::new("cache-dir")
                .long("cache-dir")
                .value_name("DIR")
                .help("Cache directory for incremental compilation")
                .default_value(".cursed_cache"),
        )
        .arg(
            Arg::new("no-cache")
                .long("no-cache")
                .help("Disable incremental compilation cache")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("no-parallel")
                .long("no-parallel")
                .help("Disable parallel compilation")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("clear-cache")
                .long("clear-cache")
                .help("Clear compilation cache before building")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("report")
                .long("report")
                .value_name("FILE")
                .help("Generate performance report to file"),
        )
        .arg(
            Arg::new("benchmark")
                .long("benchmark")
                .help("Run compilation multiple times for benchmarking")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose output")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let input_paths: Vec<PathBuf> = matches
        .get_many::<String>("input")
        .unwrap()
        .map(PathBuf::from)
        .collect();

    let jobs = match matches.get_one::<String>("jobs").unwrap().as_str() {
        "auto" => num_cpus::get(),
        n => n.parse().unwrap_or(1),
    };

    let cache_dir = PathBuf::from(matches.get_one::<String>("cache-dir").unwrap());
    let no_cache = matches.get_flag("no-cache");
    let no_parallel = matches.get_flag("no-parallel");
    let clear_cache = matches.get_flag("clear-cache");
    let report_file = matches.get_one::<String>("report");
    let benchmark = matches.get_flag("benchmark");
    let verbose = matches.get_flag("verbose");

    if verbose {
        println!("CURSED Fast Compilation Tool");
        println!("Input files: {} paths", input_paths.len());
        println!("Parallel threads: {}", jobs);
        println!("Cache directory: {}", cache_dir.display());
        println!("Incremental compilation: {}", !no_cache);
        println!("Parallel compilation: {}", !no_parallel);
        println!();
    }

    // Create optimization configuration
    let config = OptimizationConfig {
        enable_parallel_compilation: !no_parallel,
        enable_incremental_compilation: !no_cache,
        max_parallel_threads: jobs,
        ..Default::default()
    };

    // Create compilation speed optimizer
    let optimizer = CompilationSpeedOptimizer::new(&config)?;

    // Clear cache if requested
    if clear_cache {
        info!("Clearing compilation cache...");
        optimizer.clear_caches()?;
    }

    // Discover CURSED files
    let cursed_files = discover_cursed_files(&input_paths)?;
    
    if verbose {
        println!("Found {} CURSED files to compile", cursed_files.len());
    }

    // Create compilation units
    let units = create_compilation_units(&cursed_files)?;

    if benchmark {
        run_benchmark(&optimizer, units, verbose)?;
    } else {
        run_single_compilation(&optimizer, units, verbose)?;
    }

    // Generate performance report
    let report = optimizer.generate_performance_report();
    
    if let Some(report_path) = report_file {
        fs::write(report_path, &report)?;
        println!("Performance report written to {}", report_path);
    } else if verbose {
        println!("\n{}", report);
    }

    // Display summary statistics
    let stats = optimizer.get_statistics();
    println!("\nCompilation Summary:");
    println!("  Total units: {}", stats.total_units);
    println!("  Completed: {}", stats.completed_units);
    println!("  Failed: {}", stats.failed_units);
    println!("  Cached: {}", stats.cached_units);
    println!("  Total time: {}ms", stats.total_compilation_time.as_millis());
    println!("  Cache hit rate: {:.1}%", stats.cache_hit_rate * 100.0);
    
    if stats.total_units > 0 && stats.total_compilation_time.as_millis() > 0 {
        let units_per_second = (stats.total_units as f64 * 1000.0) / stats.total_compilation_time.as_millis() as f64;
        println!("  Compilation speed: {:.1} units/second", units_per_second);
    }

    Ok(())
}

fn discover_cursed_files(paths: &[PathBuf]) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    
    for path in paths {
        if path.is_file() {
            if path.extension().and_then(|ext| ext.to_str()) == Some("csd") {
                files.push(path.clone());
            }
        } else if path.is_dir() {
            let dir_files = fs::read_dir(path)?;
            for entry in dir_files {
                let entry = entry?;
                let file_path = entry.path();
                if file_path.extension().and_then(|ext| ext.to_str()) == Some("csd") {
                    files.push(file_path);
                }
            }
        }
    }
    
    Ok(files)
}

fn create_compilation_units(files: &[PathBuf]) -> Result<Vec<CompilationUnit>> {
    let mut units = Vec::new();
    
    for (index, file_path) in files.iter().enumerate() {
        let source_code = fs::read_to_string(file_path)?;
        let module_name = file_path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or(&format!("module_{}", index))
            .to_string();
        
        // Simple dependency analysis (look for import statements)
        let dependencies = extract_dependencies(&source_code);
        
        let metadata = fs::metadata(file_path)?;
        let last_modified = metadata.modified().unwrap_or(std::time::SystemTime::now());
        
        let mut unit = CompilationUnit {
            id: format!("{}_{}", module_name, index),
            source_path: file_path.clone(),
            module_name,
            source_code,
            dependencies,
            last_modified,
            status: CompilationStatus::Pending,
            priority: 1,
            content_hash: String::new(),
        };
        
        unit.content_hash = AstCache::calculate_content_hash(&unit);
        units.push(unit);
    }
    
    Ok(units)
}

fn extract_dependencies(source_code: &str) -> Vec<String> {
    let mut dependencies = Vec::new();
    
    // Simple pattern matching for import statements
    for line in source_code.split("\n") {
        let line = line.trim();
        if line.starts_with("import ") {
            // Extract module name from import statement
            if let Some(quote_start) = line.find('"') {
                if let Some(quote_end) = line[quote_start + 1..].find('"') {
                    let module_path = &line[quote_start + 1..quote_start + 1 + quote_end];
                    // Convert path to module name
                    let module_name = module_path
                        .split("::")
                        .last()
                        .unwrap_or(module_path)
                        .to_string();
                    dependencies.push(module_name);
                }
            }
        }
    }
    
    dependencies
}

fn run_single_compilation(
    optimizer: &CompilationSpeedOptimizer,
    units: Vec<CompilationUnit>,
    verbose: bool,
) -> Result<()> {
    info!("Starting compilation of {} units", units.len());
    
    let start_time = Instant::now();
    let results = optimizer.compile_incremental(units)?;
    let compilation_time = start_time.elapsed();
    
    let successful = results.iter().filter(|(_, result)| result.is_ok()).count();
    let failed = results.iter().filter(|(_, result)| result.is_err()).count();
    
    if verbose {
        println!("Compilation completed in {}ms", compilation_time.as_millis());
        println!("  Successful: {}", successful);
        println!("  Failed: {}", failed);
        
        // Show failures
        for (unit_id, result) in &results {
            if let Err(error) = result {
                error!("Failed to compile {}: {}", unit_id, error);
            }
        }
    }
    
    Ok(())
}

fn run_benchmark(
    optimizer: &CompilationSpeedOptimizer,
    units: Vec<CompilationUnit>,
    verbose: bool,
) -> Result<()> {
    const BENCHMARK_RUNS: usize = 3;
    
    println!("Running compilation benchmark ({} runs)...", BENCHMARK_RUNS);
    
    let mut run_times = Vec::new();
    
    for run in 1..=BENCHMARK_RUNS {
        if verbose {
            println!("Benchmark run {}/{}", run, BENCHMARK_RUNS);
        }
        
        // Clear cache for consistent benchmark
        if run > 1 {
            optimizer.clear_caches()?;
        }
        
        let start_time = Instant::now();
        let results = optimizer.compile_incremental(units.clone())?;
        let run_time = start_time.elapsed();
        
        run_times.push(run_time);
        
        let successful = results.iter().filter(|(_, result)| result.is_ok()).count();
        
        if verbose {
            println!("  Run {}: {}ms ({} successful)", run, run_time.as_millis(), successful);
        }
    }
    
    // Calculate statistics
    let total_time: std::time::Duration = run_times.iter().sum();
    let avg_time = total_time / BENCHMARK_RUNS as u32;
    let min_time = run_times.iter().min().cloned().unwrap_or_default();
    let max_time = run_times.iter().max().cloned().unwrap_or_default();
    
    println!("\nBenchmark Results:");
    println!("  Average time: {}ms", avg_time.as_millis());
    println!("  Min time: {}ms", min_time.as_millis());
    println!("  Max time: {}ms", max_time.as_millis());
    
    if units.len() > 0 && avg_time.as_millis() > 0 {
        let units_per_second = (units.len() as f64 * 1000.0) / avg_time.as_millis() as f64;
        println!("  Average speed: {:.1} units/second", units_per_second);
    }
    
    Ok(())
}
