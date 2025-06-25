/// CLI Commands for JIT Compilation
/// 
/// Provides command-line interface integration for JIT compilation features
/// including compilation options, performance monitoring, and configuration.

use crate::error::CursedError;
use crate::config::{JitConfig, parse_optimization_level};
use crate::codegen::llvm::{LlvmCodeGenerator, CursedJitEngine, JitCompilationInterface, create_optimized_jit_interface, create_debug_jit_interface};
use crate::runtime::{Runtime, JitRuntime};

use clap::{Command, Arg, ArgMatches};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;
use inkwell::context::Context;

/// Add JIT-related commands to the CLI
pub fn add_jit_commands(app: Command) -> Command {
    app.subcommand(
        Command::new("jit")
            .about("JIT compilation commands")
            .subcommand(
                Command::new("compile")
                    .about("Compile CURSED code with JIT")
                    .arg(
                        Arg::new("input")
                            .help("Input CURSED source file")
                            .required(true)
                            .index(1)
                    )
                    .arg(
                        Arg::new("optimization")
                            .long("optimization")
                            .short("O")
                            .help("Optimization level: none, less, default, aggressive")
                            .takes_value(true)
                            .default_value("default")
                    )
                    .arg(
                        Arg::with_name("hot-path-threshold")
                            .long("hot-path-threshold")
                            .help("Execution count threshold for hot path optimization")
                            .takes_value(true)
                            .default_value("100")
                    )
                    .arg(
                        Arg::with_name("enable-cache")
                            .long("enable-cache")
                            .help("Enable function compilation cache")
                    )
                    .arg(
                        Arg::with_name("max-memory")
                            .long("max-memory")
                            .help("Maximum JIT memory usage in MB")
                            .takes_value(true)
                            .default_value("100")
                    )
                    .arg(
                        Arg::with_name("profile")
                            .long("profile")
                            .help("Enable performance profiling")
                    )
                    .arg(
                        Arg::with_name("debug")
                            .long("debug")
                            .help("Enable debug information in JIT code")
                    )
            )
            .subcommand(
                Command::new("execute")
                    .about("Execute JIT-compiled CURSED code")
                    .arg(
                        Arg::with_name("input")
                            .help("Input CURSED source file")
                            .required(true)
                            .index(1)
                    )
                    .arg(
                        Arg::with_name("function")
                            .long("function")
                            .short("f")
                            .help("Function to execute (default: main)")
                            .takes_value(true)
                            .default_value("main")
                    )
                    .arg(
                        Arg::with_name("iterations")
                            .long("iterations")
                            .short("n")
                            .help("Number of times to execute the function")
                            .takes_value(true)
                            .default_value("1")
                    )
                    .arg(
                        Arg::with_name("profile")
                            .long("profile")
                            .help("Profile function execution")
                    )
                    .arg(
                        Arg::with_name("optimize")
                            .long("optimize")
                            .help("Enable hot path optimization")
                    )
            )
            .subcommand(
                Command::new("benchmark")
                    .about("Benchmark JIT compilation and execution performance")
                    .arg(
                        Arg::with_name("input")
                            .help("Input CURSED source file")
                            .required(true)
                            .index(1)
                    )
                    .arg(
                        Arg::with_name("function")
                            .long("function")
                            .short("f")
                            .help("Function to benchmark (default: main)")
                            .takes_value(true)
                            .default_value("main")
                    )
                    .arg(
                        Arg::with_name("warmup")
                            .long("warmup")
                            .help("Number of warmup iterations")
                            .takes_value(true)
                            .default_value("100")
                    )
                    .arg(
                        Arg::with_name("iterations")
                            .long("iterations")
                            .help("Number of benchmark iterations")
                            .takes_value(true)
                            .default_value("1000")
                    )
                    .arg(
                        Arg::with_name("compare-optimization")
                            .long("compare-optimization")
                            .help("Compare different optimization levels")
                    )
            )
            .subcommand(
                Command::new("config")
                    .about("JIT configuration management")
                    .subcommand(
                        Command::new("show")
                            .about("Show current JIT configuration")
                            .arg(
                                Arg::with_name("format")
                                    .long("format")
                                    .help("Output format: summary, json, toml")
                                    .takes_value(true)
                                    .default_value("summary")
                            )
                    )
                    .subcommand(
                        Command::new("create")
                            .about("Create JIT configuration file")
                            .arg(
                                Arg::with_name("output")
                                    .long("output")
                                    .short("o")
                                    .help("Output configuration file")
                                    .takes_value(true)
                                    .required(true)
                            )
                            .arg(
                                Arg::with_name("template")
                                    .long("template")
                                    .help("Configuration template: default, development, production, benchmarking")
                                    .takes_value(true)
                                    .default_value("default")
                            )
                    )
                    .subcommand(
                        Command::new("validate")
                            .about("Validate JIT configuration file")
                            .arg(
                                Arg::with_name("config")
                                    .help("Configuration file to validate")
                                    .required(true)
                                    .index(1)
                            )
                    )
            )
            .subcommand(
                Command::new("stats")
                    .about("Show JIT compilation and execution statistics")
                    .arg(
                        Arg::with_name("input")
                            .help("Input CURSED source file")
                            .required(true)
                            .index(1)
                    )
                    .arg(
                        Arg::with_name("detailed")
                            .long("detailed")
                            .help("Show detailed statistics")
                    )
                    .arg(
                        Arg::with_name("reset")
                            .long("reset")
                            .help("Reset statistics after showing")
                    )
            )
    )
/// Handle JIT-related CLI commands
pub fn handle_jit_command(matches: &ArgMatches) -> crate::error::Result<()> {
    match matches.subcommand() {
        _ => {
            eprintln!("Unknown JIT command. Use 'cursed jit --help' for available commands.");
            Ok(())
        }
    }
/// Handle JIT compile command
fn handle_jit_compile_command(matches: &ArgMatches) -> crate::error::Result<()> {
    let input_file = matches.value_of("input").unwrap();
    let optimization_level = matches.value_of("optimization").unwrap();
    let hot_path_threshold: u64 = matches.value_of("hot-path-threshold").unwrap().parse()
        .map_err(|_| CursedError::from_str("Invalid hot path threshold"))?;
    let max_memory_mb: usize = matches.value_of("max-memory").unwrap().parse()
        .map_err(|_| CursedError::from_str("Invalid max memory value"))?;
    
    let enable_cache = matches.is_present("enable-cache");
    let enable_profile = matches.is_present("profile");
    let enable_debug = matches.is_present("debug");

    println!("Compiling {} with JIT...", input_file);

    // Read source file
    let source = std::fs::read_to_string(input_file)
        .map_err(|e| CursedError::from_str(&format!("Failed to read input file: {}", e)))?;

    // Create JIT compilation interface
    let context = Context::create();
    let mut jit_interface = if enable_debug {
        create_debug_jit_interface(&context)?
    } else {
        create_optimized_jit_interface(&context)?

    // Update configuration based on CLI arguments
    let mut config = jit_interface.get_config().clone();
    config.hot_path_threshold = hot_path_threshold;
    
    jit_interface.update_config(config);

    let start_time = Instant::now();

    // Compile the source
    let function_name = "main"; // Assume main function for now
    jit_interface.compile_function(function_name, &source)?;

    let compilation_time = start_time.elapsed();

    println!("Compilation completed successfully!");
    println!("Time: {:?}", compilation_time);

    if enable_profile {
        let stats = jit_interface.get_stats();
        println!("Compilation statistics:");
        println!("  Functions compiled: {}", stats.total_jit_compilations);
        println!("  Total compilation time: {:?}", stats.total_compilation_time);
        println!("  Average compilation time: {:?}", stats.avg_compilation_time);
    Ok(())
/// Handle JIT execute command
fn handle_jit_execute_command(matches: &ArgMatches) -> crate::error::Result<()> {
    let input_file = matches.value_of("input").unwrap();
    let function_name = matches.value_of("function").unwrap();
    let iterations: u32 = matches.value_of("iterations").unwrap().parse()
        .map_err(|_| CursedError::from_str("Invalid iterations count"))?;
    
    let enable_profile = matches.is_present("profile");
    let enable_optimize = matches.is_present("optimize");

    println!("Executing {} function {} from {}...", function_name, iterations, input_file);

    // Read source file
    let source = std::fs::read_to_string(input_file)
        .map_err(|e| CursedError::from_str(&format!("Failed to read input file: {}", e)))?;

    // Create JIT runtime
    let context = Context::create();
    let jit_interface = create_optimized_jit_interface(&context)?;
    let runtime = Arc::new(Runtime::new());
    let mut jit_runtime = JitRuntime::new_with_default_config(jit_interface, runtime);
    jit_runtime.initialize()?;

    // Compile function
    jit_runtime.compile_function(function_name, &source)?;

    let start_time = Instant::now();
    let mut results = Vec::new();

    // Execute function multiple times
    for i in 0..iterations {
        let exec_start = Instant::now();
        let result = jit_runtime.execute_function(function_name)?;
        let exec_time = exec_start.elapsed();
        
        results.push((result, exec_time));
        
        if enable_profile && (i + 1) % 100 == 0 {
            println!("Completed {} iterations...", i + 1);
        }
    }

    let total_time = start_time.elapsed();

    // Optimize hot paths if requested
    if enable_optimize {
        println!("Optimizing hot paths...");
        let optimized_count = jit_runtime.optimize_hot_paths()?;
        println!("Optimized {} functions", optimized_count);
    println!("Execution completed!");
    println!("Total time: {:?}", total_time);
    println!("Average time per execution: {:?}", total_time / iterations);

    if enable_profile {
        let stats = jit_runtime.get_stats();
        println!("Execution statistics:");
        println!("  Total executions: {}", stats.total_jit_executions);
        println!("  Total execution time: {:?}", stats.total_execution_time);
        println!("  Average execution time: {:?}", stats.avg_execution_time);
        
        // Show result summary
        let last_result = results.last().unwrap().0;
        println!("Final result: {}", last_result);
        
        let min_time = results.iter().map(|(_, time)| *time).min().unwrap();
        let max_time = results.iter().map(|(_, time)| *time).max().unwrap();
        println!("Execution time range: {:?} - {:?}", min_time, max_time);
    jit_runtime.shutdown()?;
    Ok(())
/// Handle JIT benchmark command
fn handle_jit_benchmark_command(matches: &ArgMatches) -> crate::error::Result<()> {
    let input_file = matches.value_of("input").unwrap();
    let function_name = matches.value_of("function").unwrap();
    let warmup_iterations: u32 = matches.value_of("warmup").unwrap().parse()
        .map_err(|_| CursedError::from_str("Invalid warmup iterations"))?;
    let benchmark_iterations: u32 = matches.value_of("iterations").unwrap().parse()
        .map_err(|_| CursedError::from_str("Invalid benchmark iterations"))?;
    
    let compare_optimization = matches.is_present("compare-optimization");

    println!("Benchmarking function {} from {}...", function_name, input_file);

    // Read source file
    let source = std::fs::read_to_string(input_file)
        .map_err(|e| CursedError::from_str(&format!("Failed to read input file: {}", e)))?;

    if compare_optimization {
        benchmark_optimization_levels(&source, function_name, warmup_iterations, benchmark_iterations)?;
    } else {
        benchmark_single_configuration(&source, function_name, warmup_iterations, benchmark_iterations)?;
    Ok(())
/// Benchmark single configuration
fn benchmark_single_configuration(
) -> crate::error::Result<()> {
    let context = Context::create();
    let jit_interface = create_optimized_jit_interface(&context)?;
    let runtime = Arc::new(Runtime::new());
    let mut jit_runtime = JitRuntime::new_with_default_config(jit_interface, runtime);
    jit_runtime.initialize()?;

    // Compile function
    let compile_start = Instant::now();
    jit_runtime.compile_function(function_name, source)?;
    let compile_time = compile_start.elapsed();

    println!("Compilation time: {:?}", compile_time);

    // Warmup phase
    println!("Warming up with {} iterations...", warmup_iterations);
    for _ in 0..warmup_iterations {
        jit_runtime.execute_function(function_name)?;
    // Benchmark phase
    println!("Benchmarking with {} iterations...", benchmark_iterations);
    let benchmark_start = Instant::now();
    
    for _ in 0..benchmark_iterations {
        jit_runtime.execute_function(function_name)?;
    let benchmark_time = benchmark_start.elapsed();

    println!("=== Benchmark Results ===");
    println!("Compilation time: {:?}", compile_time);
    println!("Benchmark time: {:?}", benchmark_time);
    println!("Average execution time: {:?}", benchmark_time / benchmark_iterations);
    println!("Executions per second: {:.0}", benchmark_iterations as f64 / benchmark_time.as_secs_f64());

    // Show detailed statistics
    let stats = jit_runtime.get_stats();
    println!("=== Detailed Statistics ===");
    println!("Total executions: {}", stats.total_jit_executions);
    println!("Total execution time: {:?}", stats.total_execution_time);
    println!("Average execution time: {:?}", stats.avg_execution_time);

    jit_runtime.shutdown()?;
    Ok(())
/// Benchmark different optimization levels
fn benchmark_optimization_levels(
) -> crate::error::Result<()> {
    let optimization_levels = ["none", "less", "default", "aggressive"];
    
    println!("=== Optimization Level Comparison ===");
    
    for opt_level in &optimization_levels {
        println!("\nTesting optimization level: {}", opt_level);
        
        let context = Context::create();
        let jit_interface = if *opt_level == "none" {
            create_debug_jit_interface(&context)?
        } else {
            create_optimized_jit_interface(&context)?
        
        let runtime = Arc::new(Runtime::new());
        let mut jit_runtime = JitRuntime::new_with_default_config(jit_interface, runtime);
        jit_runtime.initialize()?;

        // Compile function
        let compile_start = Instant::now();
        jit_runtime.compile_function(function_name, source)?;
        let compile_time = compile_start.elapsed();

        // Warmup
        for _ in 0..warmup_iterations {
            jit_runtime.execute_function(function_name)?;
        // Benchmark
        let benchmark_start = Instant::now();
        for _ in 0..benchmark_iterations {
            jit_runtime.execute_function(function_name)?;
        }
        let benchmark_time = benchmark_start.elapsed();

        println!("  Compilation time: {:?}", compile_time);
        println!("  Execution time: {:?}", benchmark_time);
        println!("  Average execution: {:?}", benchmark_time / benchmark_iterations);
        println!("  Executions/sec: {:.0}", benchmark_iterations as f64 / benchmark_time.as_secs_f64());

        jit_runtime.shutdown()?;
    Ok(())
/// Handle JIT config command
fn handle_jit_config_command(matches: &ArgMatches) -> crate::error::Result<()> {
    match matches.subcommand() {
        ("show", Some(show_matches)) => {
            let format = show_matches.value_of("format").unwrap();
            show_jit_config(format)
        }
        ("create", Some(create_matches)) => {
            let output_file = create_matches.value_of("output").unwrap();
            let template = create_matches.value_of("template").unwrap();
            create_jit_config(output_file, template)
        }
        ("validate", Some(validate_matches)) => {
            let config_file = validate_matches.value_of("config").unwrap();
            validate_jit_config(config_file)
        }
        _ => {
            eprintln!("Unknown config command. Use 'cursed jit config --help' for available commands.");
            Ok(())
        }
    }
/// Show JIT configuration
fn show_jit_config(format: &str) -> crate::error::Result<()> {
    // Try to load from environment, fallback to default
    let config = JitConfig::from_env().unwrap_or_else(|_| JitConfig::default());

    match format {
        "summary" => {
            println!("{}", config.summary());
        }
        "json" => {
            let json = serde_json::to_string_pretty(&config)
                .map_err(|e| CursedError::from_str(&format!("Failed to serialize config to JSON: {}", e)))?;
            println!("{}", json);
        }
        "toml" => {
            let toml = toml::to_string_pretty(&config)
                .map_err(|e| CursedError::from_str(&format!("Failed to serialize config to TOML: {}", e)))?;
            println!("{}", toml);
        }
        _ => {
            return Err(CursedError::from_str(&format!("Unknown format: {}. Use: summary, json, toml", format)));
        }
    }

    Ok(())
/// Create JIT configuration file
fn create_jit_config(output_file: &str, template: &str) -> crate::error::Result<()> {
    let config = match template {

    let output_path = PathBuf::from(output_file);
    let extension = output_path.extension().and_then(|ext| ext.to_str()).unwrap_or("toml");

    match extension {
        _ => {
            // Default to TOML
            config.save_to_toml_file(&output_path)?;
        }
    }

    println!("Created {} configuration file: {}", template, output_file);
    Ok(())
/// Validate JIT configuration file
fn validate_jit_config(config_file: &str) -> crate::error::Result<()> {
    let config_path = PathBuf::from(config_file);
    let extension = config_path.extension().and_then(|ext| ext.to_str()).unwrap_or("toml");

    let config = match extension {

    match config.validate() {
        Ok(()) => {
            println!("Configuration is valid!");
            println!("{}", config.summary());
        }
        Err(e) => {
            println!("Configuration validation failed: {}", e);
            return Err(e);
        }
    }

    Ok(())
/// Handle JIT stats command
fn handle_jit_stats_command(matches: &ArgMatches) -> crate::error::Result<()> {
    let input_file = matches.value_of("input").unwrap();
    let detailed = matches.is_present("detailed");
    let reset = matches.is_present("reset");

    println!("Collecting JIT statistics for {}...", input_file);

    // Read source file
    let source = std::fs::read_to_string(input_file)
        .map_err(|e| CursedError::from_str(&format!("Failed to read input file: {}", e)))?;

    // Create JIT runtime
    let context = Context::create();
    let jit_interface = create_optimized_jit_interface(&context)?;
    let runtime = Arc::new(Runtime::new());
    let mut jit_runtime = JitRuntime::new_with_default_config(jit_interface, runtime);
    jit_runtime.initialize()?;

    // Compile and execute to generate some statistics
    jit_runtime.compile_function("main", &source)?;
    
    // Execute a few times to generate execution statistics
    for _ in 0..10 {
        jit_runtime.execute_function("main")?;
    // Show statistics
    let runtime_stats = jit_runtime.get_stats();
    let jit_stats = jit_runtime.get_jit_stats();

    println!("=== JIT Runtime Statistics ===");
    println!("Total executions: {}", runtime_stats.total_jit_executions);
    println!("Total execution time: {:?}", runtime_stats.total_execution_time);
    println!("Average execution time: {:?}", runtime_stats.avg_execution_time);
    println!("JIT memory usage: {} bytes", runtime_stats.jit_memory_usage);

    println!("\n=== JIT Compilation Statistics ===");
    println!("Total compilations: {}", jit_stats.total_jit_compilations);
    println!("Total compilation time: {:?}", jit_stats.total_compilation_time);
    println!("Average compilation time: {:?}", jit_stats.avg_compilation_time);
    println!("Hot path optimizations: {}", jit_stats.hot_path_optimizations);

    if detailed {
        println!("\n=== Detailed Statistics ===");
        println!("GC safe points: {}", runtime_stats.gc_safe_points);
        println!("Goroutine yields: {}", runtime_stats.goroutine_yields);
        println!("Panics recovered: {}", runtime_stats.panics_recovered);
        println!("Background optimizations: {}", runtime_stats.background_optimizations);
        println!("Performance improvement: {:.2}%", runtime_stats.performance_improvement);
        
        println!("Background compilations: {}", jit_stats.background_compilations);
        println!("Compilation timeouts: {}", jit_stats.compilation_timeouts);
        println!("Compilation failures: {}", jit_stats.compilation_failures);
    if reset {
        jit_runtime.reset_stats();
        println!("\nStatistics have been reset.");
    jit_runtime.shutdown()?;
    Ok(())
