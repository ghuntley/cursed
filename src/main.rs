/// CURSED Programming Language - Modern CLI Interface
/// 
/// Unified command-line interface for the CURSED programming language
/// with comprehensive subcommands and advanced features.

use std::env;
use std::fs;
use std::process;
use std::path::Path;
use std::collections::HashMap;
use std::path::PathBuf;
use clap::{Arg, Command, ArgMatches, value_parser};
use colored::*;
use glob::glob;
use serde_json::json;
use cursed::{self, optimization::{OptimizationConfig, OptimizationLevel as OptOptimizationLevel}};
use cursed::package_manager::{PackageManager, PackageManagerConfig};
use cursed::tools::{CursedFormatter, CursedLinter, LinterConfig};
use cursed::repl::CursedRepl;

#[tokio::main]
async fn main() {
    // Initialize environment
    cursed::init();
    
    let args: Vec<String> = env::args().collect();
    
    // Check for backward compatibility: direct file execution
    if args.len() >= 2 && 
       !args[1].starts_with('-') && 
       args[1].ends_with(".csd") &&
       !matches!(args[1].as_str(), "compile" | "run" | "test" | "pkg" | "debug" | "lint" | "fmt" | "doc" | "build" | "clean" | "check") {
        // Direct file execution for backward compatibility
        let filename = &args[1];
        match run_file_direct(filename).await {
            Ok(_) => {},
            Err(e) => {
                eprintln!("{}: {}", "Error".red(), e);
                process::exit(1);
            }
        }
        return;
    }
    
    // Parse CLI arguments
    let matches = build_cli().get_matches();
    
    // Handle subcommands
    if let Err(e) = handle_command(matches).await {
        eprintln!("{}: {}", "Error".red(), e);
        process::exit(1);
    }
}

fn build_cli() -> Command {
    Command::new("cursed")
        .version(cursed::VERSION)
        .about("CURSED Programming Language - Gen Z slang meets Go-like grammar")
        .author("Geoffrey Huntley")
        .subcommand_required(false)
        .arg_required_else_help(true)
        .long_about(
            "CURSED is a modern programming language with Gen Z slang syntax and Go-like features.\n\
            It compiles to native code using LLVM and supports advanced features like goroutines,\n\
            channels, and comprehensive package management."
        )
        // Global flags
        .arg(Arg::new("verbose")
            .help("Enable verbose output")
            .short('v')
            .long("verbose")
            .action(clap::ArgAction::SetTrue)
            .global(true))
        .arg(Arg::new("quiet")
            .help("Suppress output")
            .short('q')
            .long("quiet")
            .action(clap::ArgAction::SetTrue)
            .global(true))
        .arg(Arg::new("color")
            .help("Control colored output")
            .long("color")
            .value_name("WHEN")
            .value_parser(["auto", "always", "never"])
            .default_value("auto")
            .global(true))
        // Optimization flags
        .arg(Arg::new("optimization")
            .help("Optimization level")
            .short('O')
            .long("optimization")
            .value_name("LEVEL")
            .value_parser(["0", "1", "2", "3", "s", "z", "debug", "release"])
            .default_value("2")
            .global(true))
        .arg(Arg::new("target")
            .help("Target architecture")
            .long("target")
            .value_name("TARGET")
            .global(true))
        .arg(Arg::new("profile")
            .help("Build profile")
            .long("profile")
            .value_name("PROFILE")
            .value_parser(["debug", "release", "test"])
            .default_value("release")
            .global(true))
        // File argument for backward compatibility
        .arg(Arg::new("file")
            .help("CURSED source file to run (backward compatibility)")
            .value_name("FILE")
            .index(1))
        
        // Error handling options
        .arg(Arg::new("explain")
            .help("Explain an error code (e.g., E0001)")
            .long("explain")
            .value_name("CODE")
            .global(true))
        .arg(Arg::new("list-error-codes")
            .help("List all available error codes")
            .long("list-error-codes")
            .action(clap::ArgAction::SetTrue)
            .global(true))
        .arg(Arg::new("max-errors")
            .help("Maximum number of errors to report")
            .long("max-errors")
            .value_name("COUNT")
            .value_parser(value_parser!(usize))
            .default_value("100")
            .global(true))
        .arg(Arg::new("json-errors")
            .help("Output errors in JSON format")
            .long("json-errors")
            .action(clap::ArgAction::SetTrue)
            .global(true))
        
        // Subcommands
        .subcommand(
            Command::new("compile")
                .about("Compile CURSED source to executable")
                .arg(Arg::new("input")
                    .help("Input CURSED source file")
                    .required_unless_present("check-deps")
                    .index(1))
                .arg(Arg::new("output")
                    .help("Output executable name")
                    .short('o')
                    .long("output")
                    .value_name("FILE"))
                .arg(Arg::new("emit-ir")
                    .help("Emit LLVM IR instead of executable")
                    .long("emit-ir")
                    .action(clap::ArgAction::SetTrue))
                .arg(Arg::new("emit-asm")
                    .help("Emit assembly instead of executable")
                    .long("emit-asm")
                    .action(clap::ArgAction::SetTrue))
                .arg(Arg::new("no-link")
                    .help("Compile to object file without linking")
                    .long("no-link")
                    .action(clap::ArgAction::SetTrue))
                .arg(Arg::new("native-only")
                    .help("Only try native compilation, fail if LLVM tools are missing")
                    .long("native-only")
                    .action(clap::ArgAction::SetTrue))
                .arg(Arg::new("check-deps")
                    .help("Check if compilation dependencies are available")
                    .long("check-deps")
                    .action(clap::ArgAction::SetTrue))
        )
        .subcommand(
            Command::new("run")
                .about("Execute CURSED source file")
                .arg(Arg::new("input")
                    .help("Input CURSED source file")
                    .required(true)
                    .index(1))

                .arg(Arg::new("jit")
                    .help("Use JIT compilation")
                    .long("jit")
                    .action(clap::ArgAction::SetTrue))
                .arg(Arg::new("interpreter")
                    .help("Use interpreter mode")
                    .long("interpreter")
                    .action(clap::ArgAction::SetTrue))
        )
        .subcommand(
            Command::new("test")
                .about("Run CURSED tests (.csd files)")
                .long_about("Run tests for CURSED stdlib and user code. Automatically discovers test_*.csd and *_test.csd files.")
                .arg(Arg::new("test_dir")
                    .help("Directory to search for tests")
                    .long("test-dir")
                    .value_name("DIR")
                    .default_value("stdlib"))
                .arg(Arg::new("pattern")
                    .help("Test file pattern")
                    .long("pattern")
                    .value_name("PATTERN")
                    .default_value("test_*.csd"))
                .arg(Arg::new("filter")
                    .help("Filter tests by name")
                    .short('f')
                    .long("filter")
                    .value_name("FILTER"))
                .arg(Arg::new("parallel")
                    .help("Run tests in parallel")
                    .short('p')
                    .long("parallel")
                    .action(clap::ArgAction::SetTrue))
                .arg(Arg::new("timeout")
                    .help("Test timeout in seconds")
                    .long("timeout")
                    .value_name("SECONDS")
                    .value_parser(value_parser!(u64))
                    .default_value("30"))
                .arg(Arg::new("fail_fast")
                    .help("Stop on first test failure")
                    .long("fail-fast")
                    .action(clap::ArgAction::SetTrue))
                .arg(Arg::new("output_format")
                    .help("Output format for test results")
                    .long("format")
                    .value_name("FORMAT")
                    .value_parser(["pretty", "json", "xml", "html"])
                    .default_value("pretty"))
                .arg(Arg::new("coverage")
                    .help("Generate coverage report")
                    .long("coverage")
                    .action(clap::ArgAction::SetTrue))
        )
        .subcommand(
            Command::new("pkg")
                .about("Package management")
                .subcommand_required(true)
                .subcommand(
                    Command::new("install")
                        .about("Install a package")
                        .arg(Arg::new("package")
                            .help("Package name to install")
                            .required(true)
                            .index(1))
                        .arg(Arg::new("version")
                            .help("Specific version to install")
                            .short('v')
                            .long("version")
                            .value_name("VERSION"))
                )
                .subcommand(
                    Command::new("uninstall")
                        .about("Uninstall a package")
                        .arg(Arg::new("package")
                            .help("Package name to uninstall")
                            .required(true)
                            .index(1))
                )
                .subcommand(
                    Command::new("list")
                        .about("List installed packages")
                )
                .subcommand(
                    Command::new("update")
                        .about("Update packages")
                        .arg(Arg::new("package")
                            .help("Specific package to update")
                            .index(1))
                )
                .subcommand(
                    Command::new("search")
                        .about("Search for packages")
                        .arg(Arg::new("query")
                            .help("Search query")
                            .required(true)
                            .index(1))
                )
                .subcommand(
                    Command::new("publish")
                        .about("Publish a package")
                        .arg(Arg::new("package")
                            .help("Package directory to publish")
                            .index(1)
                            .default_value("."))
                )
                .subcommand(
                    Command::new("init")
                        .about("Initialize a new package")
                        .arg(Arg::new("name")
                            .help("Package name")
                            .required(true)
                            .index(1))
                )
        )
        .subcommand(
            Command::new("debug")
                .about("Debug compilation and runtime")
                .arg(Arg::new("input")
                    .help("Input CURSED source file")
                    .required(true)
                    .index(1))
                .arg(Arg::new("breakpoints")
                    .help("Set breakpoints (line numbers)")
                    .short('b')
                    .long("breakpoints")
                    .value_name("LINES")
                    .num_args(1..))
                .arg(Arg::new("trace")
                    .help("Enable execution tracing")
                    .long("trace")
                    .action(clap::ArgAction::SetTrue))
                .arg(Arg::new("memory")
                    .help("Enable memory debugging")
                    .long("memory")
                    .action(clap::ArgAction::SetTrue))
                .arg(Arg::new("compile-only")
                    .help("Only compile with debug info, don't run")
                    .long("compile-only")
                    .action(clap::ArgAction::SetTrue))
        )
        .subcommand(
            Command::new("lint")
                .about("Lint CURSED source code")
                .arg(Arg::new("input")
                    .help("Input files or directories")
                    .required(true)
                    .num_args(1..))
                .arg(Arg::new("fix")
                    .help("Automatically fix issues")
                    .long("fix")
                    .action(clap::ArgAction::SetTrue))
                .arg(Arg::new("config")
                    .help("Linter configuration file")
                    .short('c')
                    .long("config")
                    .value_name("FILE"))
                .arg(Arg::new("rules")
                    .help("Enable specific rules")
                    .short('r')
                    .long("rules")
                    .value_name("RULES")
                    .num_args(1..))
        )
        .subcommand(
            Command::new("fmt")
                .about("Format CURSED source code")
                .arg(Arg::new("input")
                    .help("Input files or directories")
                    .required(true)
                    .num_args(1..))
                .arg(Arg::new("check")
                    .help("Check if files are formatted without modifying")
                    .long("check")
                    .action(clap::ArgAction::SetTrue))
                .arg(Arg::new("diff")
                    .help("Show formatting differences")
                    .long("diff")
                    .action(clap::ArgAction::SetTrue))
                .arg(Arg::new("config")
                    .help("Formatter configuration file")
                    .short('c')
                    .long("config")
                    .value_name("FILE"))
        )
        .subcommand(
            Command::new("doc")
                .about("Generate documentation")
                .arg(Arg::new("input")
                    .help("Input directory")
                    .index(1)
                    .default_value("."))
                .arg(Arg::new("output")
                    .help("Output directory")
                    .short('o')
                    .long("output")
                    .value_name("DIR")
                    .default_value("docs"))
                .arg(Arg::new("format")
                    .help("Documentation format")
                    .short('f')
                    .long("format")
                    .value_name("FORMAT")
                    .value_parser(["html", "markdown", "json"])
                    .default_value("html"))
                .arg(Arg::new("serve")
                    .help("Serve documentation locally")
                    .long("serve")
                    .action(clap::ArgAction::SetTrue))
                .arg(Arg::new("port")
                    .help("Server port")
                    .long("port")
                    .value_name("PORT")
                    .value_parser(value_parser!(u16))
                    .default_value("8080"))
        )
        .subcommand(
            Command::new("build")
                .about("Build project")
                .arg(Arg::new("release")
                    .help("Build in release mode")
                    .long("release")
                    .action(clap::ArgAction::SetTrue))
                .arg(Arg::new("output")
                    .help("Output directory")
                    .short('o')
                    .long("output")
                    .value_name("DIR")
                    .default_value("target"))
                .arg(Arg::new("jobs")
                    .help("Number of parallel jobs")
                    .short('j')
                    .long("jobs")
                    .value_name("N")
                    .value_parser(value_parser!(usize)))
        )
        .subcommand(
            Command::new("clean")
                .about("Clean build artifacts")
                .arg(Arg::new("all")
                    .help("Clean all artifacts including dependencies")
                    .long("all")
                    .action(clap::ArgAction::SetTrue))
                .arg(Arg::new("cache")
                    .help("Clean compilation cache")
                    .long("cache")
                    .action(clap::ArgAction::SetTrue))
        )
        .subcommand(
            Command::new("check")
                .about("Check source code without building")
                .arg(Arg::new("input")
                    .help("Input files or directories")
                    .required(true)
                    .num_args(1..))
                .arg(Arg::new("json")
                    .help("Output results in JSON format")
                    .long("json")
                    .action(clap::ArgAction::SetTrue))
        )
        .subcommand(
            Command::new("repl")
                .about("Start interactive REPL")
                .arg(Arg::new("no-history")
                    .help("Disable command history")
                    .long("no-history")
                    .action(clap::ArgAction::SetTrue))
                .arg(Arg::new("startup")
                    .help("Startup file to load")
                    .long("startup")
                    .value_name("FILE"))
        )
}

async fn handle_command(matches: ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    // Handle error explanation commands first
    if let Some(code) = matches.get_one::<String>("explain") {
        use cursed::error::cli::FileAwareErrorReporter;
        return match FileAwareErrorReporter::handle_explain_command(code) {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("{}: {}", "Error".red(), e);
                Err(e.into())
            }
        };
    }
    
    if matches.get_flag("list-error-codes") {
        use cursed::error::cli::FileAwareErrorReporter;
        FileAwareErrorReporter::list_error_codes();
        return Ok(());
    }
    
    // Set up logging based on verbosity
    if matches.get_flag("verbose") {
        std::env::set_var("RUST_LOG", "cursed=debug");
    } else if matches.get_flag("quiet") {
        std::env::set_var("RUST_LOG", "cursed=warn");
    }
    
    // Handle backward compatibility file argument
    if let Some(file) = matches.get_one::<String>("file") {
        return run_file_direct(file).await;
    }
    
    // Handle subcommands
    match matches.subcommand() {
        Some(("compile", sub_matches)) => handle_compile(sub_matches, &matches).await,
        Some(("run", sub_matches)) => handle_run(sub_matches, &matches).await,
        Some(("test", sub_matches)) => handle_test(sub_matches, &matches).await,
        Some(("pkg", sub_matches)) => handle_pkg(sub_matches, &matches).await,
        Some(("debug", sub_matches)) => handle_debug(sub_matches, &matches).await,
        Some(("lint", sub_matches)) => handle_lint(sub_matches, &matches).await,
        Some(("fmt", sub_matches)) => handle_fmt(sub_matches, &matches).await,
        Some(("doc", sub_matches)) => handle_doc(sub_matches, &matches).await,
        Some(("build", sub_matches)) => handle_build(sub_matches, &matches).await,
        Some(("clean", sub_matches)) => handle_clean(sub_matches, &matches).await,
        Some(("check", sub_matches)) => handle_check(sub_matches, &matches).await,
        Some(("repl", sub_matches)) => handle_repl(sub_matches, &matches).await,
        _ => {
            // This shouldn't happen due to arg_required_else_help
            eprintln!("No subcommand provided. Use --help for usage information.");
            process::exit(1);
        }
    }
}

async fn run_file_direct(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("{} {}", "Running".green().bold(), filename);
    
    match cursed::run_file(filename) {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e))
    }
}

fn handle_jit_execution(input: &str, global_matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    use std::time::Instant;
    use cursed::execution::{JitExecutor, JitExecutorConfig};
    use cursed::runtime::jit_runtime::OptimizationLevel;
    
    println!("{} {} with JIT compilation", "Running".green().bold(), input);
    
    let start_time = Instant::now();
    
    // Configure JIT executor based on CLI options
    let mut config = JitExecutorConfig::default();
    
    // Set optimization level based on global options
    if let Some(opt_level) = global_matches.get_one::<String>("optimization") {
        config.initial_optimization = match opt_level.as_str() {
            "0" => OptimizationLevel::None,
            "1" => OptimizationLevel::Basic,
            "2" => OptimizationLevel::Standard,
            "3" | "s" | "z" => OptimizationLevel::Aggressive,
            _ => OptimizationLevel::Standard,
        };
    }
    
    // Enable profiling in verbose mode
    config.enable_profiling = global_matches.get_flag("verbose");
    
    println!("{} Initializing JIT compiler with optimization level: {:?}", 
             "Info".blue().bold(), config.initial_optimization);
    
    // Create JIT executor
    let mut jit_executor = JitExecutor::with_config(config)
        .map_err(|e| format!("Failed to create JIT executor: {}", e))?;
    
    // Read source file
    let source = std::fs::read_to_string(input)
        .map_err(|e| format!("Failed to read source file: {}", e))?;
    
    // Execute with JIT compilation
    let result = jit_executor.execute(&source)
        .map_err(|e| format!("JIT execution failed: {}", e));
    
    match result {
        Ok(_value) => {
            let execution_time = start_time.elapsed();
            
            // Show performance metrics
            if global_matches.get_flag("verbose") {
                display_jit_performance_metrics(&jit_executor, execution_time)?;
            } else {
                println!("{} JIT compilation and execution completed in {:.2}ms", 
                         "✓".green().bold(), 
                         execution_time.as_millis());
            }
            
            Ok(())
        }
        Err(e) => {
            eprintln!("{} JIT execution failed: {}", "Error".red().bold(), e);
            
            // Fallback to standard interpreter mode
            println!("{} Falling back to standard interpreter mode", "Warning".yellow().bold());
            cursed::run_file(input)?;
            
            Ok(())
        }
    }
}

fn display_simple_performance_metrics(
    execution_time: std::time::Duration
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n{} JIT Performance Metrics:", "📊".bold());
    println!("  Execution time: {:.2}ms", execution_time.as_millis());
    println!("  Engine: Enhanced CURSED interpreter with JIT optimizations");
    println!("  Features: Advanced AST optimization, hot path detection");
    println!("  Memory: Stack-based execution with garbage collection");
    Ok(())
}

fn display_jit_performance_metrics(
    jit_executor: &cursed::execution::JitExecutor, 
    execution_time: std::time::Duration
) -> Result<(), Box<dyn std::error::Error>> {
    let stats = jit_executor.get_stats()
        .map_err(|e| format!("Failed to get JIT statistics: {}", e))?;
    
    println!("\n{} JIT Performance Metrics:", "📊".bold());
    println!("  Execution time: {:.2}ms", execution_time.as_millis());
    println!("  Total compiled functions: {}", stats.functions_compiled);
    println!("  Total executions: {}", stats.total_executions);
    println!("  Total compilation time: {:.2}ms", stats.total_compilation_time.as_millis());
    println!("  Total execution time: {:.2}ms", stats.total_execution_time.as_millis());
    
    if stats.total_executions > 0 {
        println!("  Average execution time: {:.2}ms", 
                 stats.total_execution_time.as_millis() as f64 / stats.total_executions as f64);
    }
    
    println!("  JIT compilation ratio: {:.1}%", stats.jit_ratio * 100.0);
    println!("  Cache hit ratio: {:.1}%", stats.cache_hit_ratio * 100.0);
    println!("  Tier-up events: {}", stats.tier_up_events);
    
    println!("  Engine: LLVM JIT with real-time compilation");
    println!("  Features: Background compilation, hot path detection, tier-up optimization");
    
    Ok(())
}

async fn handle_compile(matches: &ArgMatches, global_matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    // Handle dependency check first (doesn't need input file)
    if matches.get_flag("check-deps") {
        return check_compilation_dependencies().await;
    }
    
    let input = matches.get_one::<String>("input").unwrap();
    let output = matches.get_one::<String>("output")
        .map(|s| s.as_str())
        .unwrap_or_else(|| {
            Path::new(input)
                .file_stem()
                .and_then(|stem| stem.to_str())
                .unwrap_or("output")
        });
    
    if matches.get_flag("verbose") || global_matches.get_flag("verbose") {
        println!("{} {} to {}", "Compiling".green().bold(), input, output);
    }
    
    if matches.get_flag("emit-ir") {
        let ir = cursed::compile_to_ir_with_optimization(
            &fs::read_to_string(input)?,
            global_matches.get_one::<String>("optimization").map(|s| s.as_str())
        )?;
        fs::write(format!("{}.ll", output), ir)?;
        println!("{} LLVM IR to {}.ll", "Generated".green().bold(), output);
    } else if matches.get_flag("emit-asm") {
        let source = fs::read_to_string(input)?;
        let assembly = cursed::compile_to_assembly(&source)?;
        fs::write(format!("{}.s", output), assembly)?;
        println!("{} Assembly to {}.s", "Generated".green().bold(), output);
    } else {
        // Handle native-only flag
        if matches.get_flag("native-only") {
            cursed::compile_native_only(input, output).await?;
            println!("{} native executable: {}", "Generated".green().bold(), output);
        } else {
            cursed::compile(input, output).await?;
            println!("{} executable: {}", "Generated".green().bold(), output);
        }
    }
    
    Ok(())
}

async fn check_compilation_dependencies() -> Result<(), Box<dyn std::error::Error>> {
    use std::process::Command;
    
    println!("{} compilation dependencies...", "Checking".blue().bold());
    
    let mut all_available = true;
    
    // Check for LLVM tools
    println!("🔍 Checking for LLVM tools:");
    let llc_locations = vec![
        "llc",
        "/nix/store/013b6qj9g2n2pmxcllnch9drrf9m0zwf-llvm-17.0.6/bin/llc",
        "/nix/store/s5a4igx64mngxrz3d4s2mxz6764mdv47-llvm-17.0.6/bin/llc",
        "/nix/store/8qpf7pp0a71psdngm5nxc64jahw0vlwl-llvm-19.1.7/bin/llc",
        "/nix/store/vnxd8nqfibccfbczxwd9li5hw42k5kmw-llvm-19.1.6/bin/llc",
        "/usr/bin/llc",
        "/usr/local/bin/llc",
    ];
    
    let mut llc_found = false;
    for location in &llc_locations {
        if let Ok(output) = Command::new(location).arg("--version").output() {
            if output.status.success() {
                println!("  {} llc found at: {}", "✓".green(), location);
                llc_found = true;
                break;
            }
        }
    }
    
    if !llc_found {
        println!("  {} llc not found", "✗".red());
        all_available = false;
    }
    
    // Check for linkers
    println!("🔗 Checking for linkers:");
    let linkers = ["clang", "gcc", "ld"];
    let mut linker_found = false;
    
    for linker in &linkers {
        if let Ok(output) = Command::new(linker).arg("--version").output() {
            if output.status.success() {
                println!("  {} {} found", "✓".green(), linker);
                linker_found = true;
                break;
            }
        }
    }
    
    if !linker_found {
        println!("  {} No linkers found", "✗".red());
        all_available = false;
    }
    
    // Check for CURSED runtime library
    println!("📚 Checking for CURSED runtime:");
    let runtime_paths = vec![
        format!("{}/libcursed_runtime.a", env!("OUT_DIR")),
        "./libcursed_runtime.a".to_string(),
        "/usr/lib/libcursed_runtime.a".to_string(),
        "/usr/local/lib/libcursed_runtime.a".to_string(),
    ];
    
    let mut runtime_found = false;
    for path in &runtime_paths {
        if std::path::Path::new(path).exists() {
            println!("  {} Runtime library found at: {}", "✓".green(), path);
            runtime_found = true;
            break;
        }
    }
    
    if !runtime_found {
        println!("  {} Runtime library not found (optional)", "⚠️".yellow());
    }
    
    println!();
    
    if all_available {
        println!("{} All compilation dependencies are available!", "🎉".green());
        println!("You can use native compilation with: cursed compile program.csd");
    } else {
        println!("{} Some compilation dependencies are missing", "⚠️".yellow());
        println!("Install missing dependencies:");
        
        if !llc_found {
            println!("  LLVM tools:");
            println!("    Ubuntu/Debian: sudo apt install llvm");
            println!("    macOS: brew install llvm");
            println!("    devenv: direnv allow");
        }
        
        if !linker_found {
            println!("  C compiler/linker:");
            println!("    Ubuntu/Debian: sudo apt install clang gcc");
            println!("    macOS: xcode-select --install");
        }
        
        println!();
        println!("💡 You can still use interpretation mode:");
        println!("  cursed run program.csd");
        println!("  cursed compile program.csd  # Will fall back to interpretation wrapper");
    }
    
    Ok(())
}

async fn handle_run(matches: &ArgMatches, global_matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let input = matches.get_one::<String>("input").unwrap();
    
    if matches.get_flag("verbose") || global_matches.get_flag("verbose") {
        println!("{} {}", "Running".green().bold(), input);
    }
    
    if matches.get_flag("jit") {
        handle_jit_execution(input, global_matches)?;
    } else if matches.get_flag("interpreter") {
        cursed::run_file(input)?;
    } else {
        // Default: run file directly for simplicity
        cursed::run_file(input)?;
    }
    
    Ok(())
}

async fn handle_test(matches: &ArgMatches, _global_matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    use std::path::Path;
    use std::fs;
    use std::process::Command;
    use glob::glob;
    
    // Parse command line arguments
    let test_dir = matches.get_one::<String>("test_dir").unwrap();
    let pattern = matches.get_one::<String>("pattern").unwrap();
    let filter = matches.get_one::<String>("filter");
    let parallel = matches.get_flag("parallel");
    let timeout = matches.get_one::<u64>("timeout").unwrap();
    let fail_fast = matches.get_flag("fail_fast");
    let output_format = matches.get_one::<String>("output_format").unwrap();
    
    println!("{}", "🧪 CURSED Test Runner".bold().cyan());
    println!("Test directory: {}", test_dir);
    println!("Pattern: {}", pattern);
    if let Some(f) = filter {
        println!("Filter: {}", f);
    }
    println!("Format: {}", output_format);
    println!();
    
    // Discover test files
    let search_pattern = Path::new(test_dir).join(pattern);
    let mut test_files = Vec::new();
    
    for entry in glob(&search_pattern.to_string_lossy()).map_err(|e| format!("Glob error: {}", e))? {
        let path = entry.map_err(|e| format!("Path error: {}", e))?;
        if path.is_file() {
            if let Some(filter_str) = filter {
                if !path.to_string_lossy().contains(filter_str) {
                    continue;
                }
            }
            test_files.push(path);
        }
    }
    
    // Also look for alternative patterns
    let alt_pattern = Path::new(test_dir).join("*_test.csd");
    for entry in glob(&alt_pattern.to_string_lossy()).map_err(|e| format!("Glob error: {}", e))? {
        let path = entry.map_err(|e| format!("Path error: {}", e))?;
        if path.is_file() && !test_files.contains(&path) {
            if let Some(filter_str) = filter {
                if !path.to_string_lossy().contains(filter_str) {
                    continue;
                }
            }
            test_files.push(path);
        }
    }
    
    test_files.sort();
    
    if test_files.is_empty() {
        println!("{}", "No test files found".yellow());
        return Ok(());
    }
    
    println!("Found {} test file(s):", test_files.len());
    for test_file in &test_files {
        println!("  {}", test_file.display());
    }
    println!();
    
    // Run tests
    let mut results = Vec::new();
    let mut passed = 0;
    let mut failed = 0;
    
    for (i, test_file) in test_files.iter().enumerate() {
        println!("[{}/{}] Running: {}", i + 1, test_files.len(), test_file.display());
        
        let start_time = std::time::Instant::now();
        
        // Use cargo run to execute the test file
        let output = Command::new("cargo")
            .args(&["run", "--bin", "cursed", "--"])
            .arg(test_file)
            .output();
        
        let duration = start_time.elapsed();
        
        match output {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                
                if output.status.success() {
                    println!("  {} {} ({:?})", "✓".green(), "PASSED".green(), duration);
                    passed += 1;
                } else {
                    println!("  {} {} ({:?})", "✗".red(), "FAILED".red(), duration);
                    if !stderr.is_empty() {
                        println!("    Error: {}", stderr.lines().next().unwrap_or("Unknown error"));
                    }
                    failed += 1;
                    
                    if fail_fast {
                        break;
                    }
                }
                
                // Store result for potential JSON/XML output
                results.push((test_file.clone(), output.status.success(), duration, stdout.to_string(), stderr.to_string()));
            }
            Err(e) => {
                println!("  {} {} ({:?})", "✗".red(), "ERROR".red(), duration);
                println!("    Failed to execute: {}", e);
                failed += 1;
                
                if fail_fast {
                    break;
                }
            }
        }
        
        // Apply timeout if specified
        if duration.as_secs() > *timeout {
            println!("    Warning: Test took longer than {} seconds", timeout);
        }
        
        println!();
    }
    
    // Print summary
    println!("{}", "=== TEST SUMMARY ===".bold().underline());
    println!("Total tests: {}", test_files.len());
    println!("{} {}", "Passed:".green(), passed.to_string().green());
    if failed > 0 {
        println!("{} {}", "Failed:".red(), failed.to_string().red());
    }
    
    if failed == 0 {
        println!("{}", "🎉 ALL TESTS PASSED! 🎉".green().bold());
    } else {
        println!("{}", "❌ Some tests failed".red().bold());
    }
    
    // Handle different output formats
    match output_format.as_str() {
        "json" => {
            let json_results = serde_json::json!({
                "summary": {
                    "total": test_files.len(),
                    "passed": passed,
                    "failed": failed
                },
                "tests": results.iter().map(|(path, success, duration, stdout, stderr)| {
                    serde_json::json!({
                        "name": path.file_name().unwrap().to_string_lossy(),
                        "path": path.to_string_lossy(),
                        "passed": success,
                        "duration_ms": duration.as_millis(),
                        "stdout": stdout,
                        "stderr": stderr
                    })
                }).collect::<Vec<_>>()
            });
            println!("\nJSON Output:");
            println!("{}", serde_json::to_string_pretty(&json_results).unwrap());
        }
        "xml" => {
            println!("\nXML Output:");
            println!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
            println!("<testsuites>");
            println!("  <testsuite name=\"cursed-tests\" tests=\"{}\" failures=\"{}\" time=\"{:.3}\">", 
                     test_files.len(), failed, results.iter().map(|(_, _, d, _, _)| d.as_secs_f64()).sum::<f64>());
            for (path, success, duration, stdout, stderr) in &results {
                println!("    <testcase name=\"{}\" time=\"{:.3}\">", 
                         path.file_name().unwrap().to_string_lossy(), duration.as_secs_f64());
                if !success {
                    println!("      <failure message=\"Test failed\">{}</failure>", stderr);
                }
                println!("    </testcase>");
            }
            println!("  </testsuite>");
            println!("</testsuites>");
        }
        "html" => {
            println!("\nHTML Output:");
            println!("<!DOCTYPE html>");
            println!("<html><head><title>CURSED Test Results</title></head><body>");
            println!("<h1>CURSED Test Results</h1>");
            println!("<p>Total: {}, Passed: {}, Failed: {}</p>", test_files.len(), passed, failed);
            println!("<table border=\"1\">");
            println!("<tr><th>Test</th><th>Status</th><th>Duration</th></tr>");
            for (path, success, duration, _, _) in &results {
                let status = if *success { "PASSED" } else { "FAILED" };
                let color = if *success { "green" } else { "red" };
                println!("<tr><td>{}</td><td style=\"color: {}\">{}</td><td>{:.3}s</td></tr>", 
                         path.file_name().unwrap().to_string_lossy(), color, status, duration.as_secs_f64());
            }
            println!("</table></body></html>");
        }
        _ => {} // "pretty" format already handled above
    }
    
    if failed > 0 {
        std::process::exit(1);
    }
    
    Ok(())
}

async fn handle_pkg(matches: &ArgMatches, _global_matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    // Create package manager
    let config = PackageManagerConfig::default();
    let mut pkg_manager = PackageManager::new(config)?;
    
    match matches.subcommand() {
        Some(("install", sub_matches)) => {
            let package = sub_matches.get_one::<String>("package").unwrap();
            let version = sub_matches.get_one::<String>("version").map(|s| s.as_str());
            
            println!("{} package: {}", "Installing".green().bold(), package);
            
            match version {
                Some(v) => { pkg_manager.install_package(package, Some(v)).await?; },
                None => { pkg_manager.install_package(package, None).await?; },
            }
            
            println!("{} Successfully installed {}", "✓".green().bold(), package);
        }
        Some(("uninstall", sub_matches)) => {
            let package = sub_matches.get_one::<String>("package").unwrap();
            
            println!("{} package: {}", "Uninstalling".yellow().bold(), package);
            pkg_manager.uninstall_package(package).await?;
            println!("{} Successfully uninstalled {}", "✓".green().bold(), package);
        }
        Some(("list", _)) => {
            println!("{} packages:", "Installed".blue().bold());
            let packages = pkg_manager.list_installed();
            for package in packages {
                println!("  {} {}", package.name, package.version);
            }
        }
        Some(("update", sub_matches)) => {
            if let Some(package) = sub_matches.get_one::<String>("package") {
                println!("{} package: {}", "Updating".blue().bold(), package);
                pkg_manager.update_package(package).await?;
            } else {
                println!("{} all packages", "Updating".blue().bold());
                pkg_manager.update_all().await?;
            }
            println!("{} Update completed", "✓".green().bold());
        }
        Some(("search", sub_matches)) => {
            let query = sub_matches.get_one::<String>("query").unwrap();
            println!("{} for: {}", "Searching".blue().bold(), query);
            
            // TODO: Implement search functionality
            println!("{} Search functionality not yet implemented", "Warning".yellow().bold());
        }
        Some(("publish", sub_matches)) => {
            let package_dir = sub_matches.get_one::<String>("package").unwrap();
            println!("{} package from: {}", "Publishing".blue().bold(), package_dir);
            
            // TODO: Implement publish functionality
            println!("{} Publish functionality not yet implemented", "Warning".yellow().bold());
        }
        Some(("init", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name").unwrap();
            println!("{} new package: {}", "Initializing".blue().bold(), name);
            
            // Create package directory structure
            std::fs::create_dir_all(name)?;
            std::fs::create_dir_all(&format!("{}/src", name))?;
            std::fs::create_dir_all(&format!("{}/tests", name))?;
            
            // Create package.toml
            let package_toml = format!(
                r#"[package]
name = "{}"
version = "0.1.0"
description = "A new CURSED package"
authors = ["Your Name <your.email@example.com>"]
license = "MIT"
edition = "2024"

[dependencies]
# Add dependencies here

[dev-dependencies]
# Add development dependencies here

[build]
# Build configuration
optimization = "2"
target = "native"

[features]
# Feature flags
default = []
"#, name);
            
            std::fs::write(&format!("{}/package.toml", name), package_toml)?;
            
            // Create main.csd
            let main_csd = format!(
                r#"// {} - Main entry point
//
// This is the main module for your CURSED package.
// You can import other modules and define your program logic here.

func main() {{
    puts("Hello from {}! 🚀");
}}
"#, name, name);
            
            std::fs::write(&format!("{}/src/main.csd", name), main_csd)?;
            
            // Create lib.csd
            let lib_csd = format!(
                r#"// {} - Library module
//
// This is the main library module for your CURSED package.
// Export public functions and types here.

// Example public function
pub func greet(name: string) -> string {{
    return "Hello, " + name + "!";
}}

// Example public interface
pub interface Greeter {{
    func greet(name: string) -> string;
}}

// Example struct implementing the interface
pub struct SimpleGreeter {{}}

impl Greeter for SimpleGreeter {{
    func greet(name: string) -> string {{
        return "Hello, " + name + " from SimpleGreeter!";
    }}
}}
"#, name);
            
            std::fs::write(&format!("{}/src/lib.csd", name), lib_csd)?;
            
            // Create example test
            let test_csd = format!(
                r#"// {} - Test module
//
// This file contains tests for your CURSED package.
// Run tests with: cursed test

import "{}/src/lib" as lib;

func test_greet() {{
    let result = lib.greet("World");
    assert_eq(result, "Hello, World!");
    puts("✓ test_greet passed");
}}

func test_greeter_interface() {{
    let greeter = lib.SimpleGreeter{{}};
    let result = greeter.greet("CURSED");
    assert_eq(result, "Hello, CURSED from SimpleGreeter!");
    puts("✓ test_greeter_interface passed");
}}

func main() {{
    puts("Running tests for {}...");
    
    test_greet();
    test_greeter_interface();
    
    puts("All tests passed! 🎉");
}}
"#, name, name, name);
            
            std::fs::write(&format!("{}/tests/lib_test.csd", name), test_csd)?;
            
            // Create README.md
            let readme_md = format!(
                r#"# {}

A CURSED programming language package.

## Getting Started

### Building

```bash
cd {}
cursed build
```

### Running

```bash
cursed run src/main.csd
```

### Testing

```bash
cursed test
```

## Development

### Project Structure

```
{}/
├── src/
│   ├── main.csd      # Main entry point
│   └── lib.csd       # Library module
├── tests/
│   └── lib_test.csd  # Test module
├── package.toml      # Package configuration
└── README.md         # This file
```

### Adding Dependencies

Add dependencies to `package.toml`:

```toml
[dependencies]
some-package = "1.0.0"
```

### Publishing

```bash
cursed pkg publish
```

## License

MIT License
"#, name, name, name);
            
            std::fs::write(&format!("{}/README.md", name), readme_md)?;
            
            // Create .gitignore
            let gitignore = r#"# CURSED build artifacts
target/
*.ll
*.s
*.o
*.out

# IDE files
.vscode/
.idea/
*.swp
*.swo

# OS files
.DS_Store
Thumbs.db

# Temporary files
*.tmp
*.temp
"#;
            
            std::fs::write(&format!("{}/.gitignore", name), gitignore)?;
            
            println!("{} Successfully initialized package '{}'", "✅".green().bold(), name);
            println!();
            println!("Next steps:");
            println!("  cd {}", name);
            println!("  cursed build              # Build the package");
            println!("  cursed run src/main.csd   # Run the main program");
            println!("  cursed test               # Run tests");
            println!("  cursed pkg publish        # Publish to registry");
        }
        _ => {
            eprintln!("Unknown package subcommand");
            process::exit(1);
        }
    }
    
    Ok(())
}

async fn handle_debug(matches: &ArgMatches, global_matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let input = matches.get_one::<String>("input").unwrap();
    let compile_only = matches.get_flag("compile-only");
    
    println!("{} {} with debug information", "Compiling".green().bold(), input);
    
    // Read source file
    let source = fs::read_to_string(input)?;
    
    // Compile with debug optimization
    let ir = cursed::compile_to_ir_with_optimization(&source, Some("debug"))?;
    
    if compile_only {
        let output = format!("{}.debug.ll", 
                           Path::new(input).file_stem().unwrap().to_str().unwrap());
        fs::write(&output, ir)?;
        println!("{} debug IR to: {}", "Generated".green().bold(), output);
    } else {
        println!("{} Debug execution not yet fully implemented", "Warning".yellow().bold());
        // TODO: Implement debug execution with breakpoints and tracing
    }
    
    Ok(())
}

async fn handle_lint(matches: &ArgMatches, _global_matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let inputs: Vec<&String> = matches.get_many::<String>("input").unwrap().collect();
    let fix = matches.get_flag("fix");
    
    println!("{} {} files", "Linting".blue().bold(), inputs.len());
    
    let mut linter = CursedLinter::new();
    
    let mut total_issues = 0;
    
    for input in inputs {
        if Path::new(input).is_file() {
            let source = fs::read_to_string(input)?;
            let results = linter.lint_source(&source)?;
            
            if !results.issues.is_empty() {
                println!("\n{}: {} issues found", input, results.issues.len());
                for issue in &results.issues {
                    println!("  {}:{}: {} {}", 
                    issue.line, issue.column, 
                    issue.severity.as_str().yellow().bold(), 
                    issue.message);
                }
                total_issues += results.issues.len();
            }
        } else {
            // Handle directory
            for entry in glob::glob(&format!("{}/**/*.csd", input))? {
                let path = entry?;
                let source = fs::read_to_string(&path)?;
                let results = linter.lint_source(&source)?;
                
                if !results.issues.is_empty() {
                    println!("\n{}: {} issues found", path.display(), results.issues.len());
                    for issue in &results.issues {
                        println!("  {}:{}: {} {}", 
                        issue.line, issue.column, 
                        issue.severity.as_str().yellow().bold(), 
                        issue.message);
                    }
                    total_issues += results.issues.len();
                }
            }
        }
    }
    
    if total_issues == 0 {
        println!("{} No issues found", "✓".green().bold());
    } else {
        println!("\n{} {} issues found", "Summary:".bold(), total_issues);
        if fix {
            println!("{} Auto-fixing not yet implemented", "Warning".yellow().bold());
        }
    }
    
    Ok(())
}

async fn handle_fmt(matches: &ArgMatches, _global_matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let inputs: Vec<&String> = matches.get_many::<String>("input").unwrap().collect();
    let check = matches.get_flag("check");
    let diff = matches.get_flag("diff");
    
    println!("{} {} files", "Formatting".blue().bold(), inputs.len());
    
    let mut formatter = CursedFormatter::default();
    let mut changed_files = 0;
    
    for input in inputs {
        if Path::new(input).is_file() {
            let source = fs::read_to_string(input)?;
            let formatted = formatter.format(&source)?;
            
            if source != formatted {
                if check {
                    println!("{}: {} needs formatting", input, "✗".red().bold());
                    changed_files += 1;
                } else if diff {
                    println!("\n{}: formatting differences", input);
                    // TODO: Show diff
                    changed_files += 1;
                } else {
                    fs::write(input, formatted)?;
                    println!("{}: {} formatted", input, "✓".green().bold());
                    changed_files += 1;
                }
            } else {
                println!("{}: {} already formatted", input, "✓".green().bold());
            }
        } else {
            // Handle directory
            for entry in glob::glob(&format!("{}/**/*.csd", input))? {
                let path = entry?;
                let source = fs::read_to_string(&path)?;
                let formatted = formatter.format(&source)?;
                
                if source != formatted {
                    if check {
                        println!("{}: {} needs formatting", path.display(), "✗".red().bold());
                        changed_files += 1;
                    } else if diff {
                        println!("\n{}: formatting differences", path.display());
                        // TODO: Show diff
                        changed_files += 1;
                    } else {
                        fs::write(&path, formatted)?;
                        println!("{}: {} formatted", path.display(), "✓".green().bold());
                        changed_files += 1;
                    }
                } else {
                    println!("{}: {} already formatted", path.display(), "✓".green().bold());
                }
            }
        }
    }
    
    if check && changed_files > 0 {
        println!("\n{} {} files need formatting", "Summary:".bold(), changed_files);
        process::exit(1);
    }
    
    Ok(())
}

async fn handle_doc(matches: &ArgMatches, _global_matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let input = matches.get_one::<String>("input").unwrap();
    let output = matches.get_one::<String>("output").unwrap();
    let format = matches.get_one::<String>("format").unwrap();
    let serve = matches.get_flag("serve");
    let port = matches.get_one::<u16>("port").copied().unwrap_or(8080);
    
    println!("{} documentation from {}", "Generating".blue().bold(), input);
    
    // Generate documentation using integrated system
    generate_documentation(input, output, format).await?;
    
    if serve {
        println!("{} documentation server on port {}", "Starting".blue().bold(), port);
        serve_documentation(output, port).await?;
    }
    
    Ok(())
}

async fn handle_build(matches: &ArgMatches, global_matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let release = matches.get_flag("release");
    let output = matches.get_one::<String>("output").unwrap();
    let jobs = matches.get_one::<usize>("jobs").copied();
    
    println!("{} project in {} mode", 
             "Building".green().bold(), 
             if release { "release" } else { "debug" });
    
    // Use the new build system
    use cursed::build_system::{BuildOrchestrator, BuildStrategy};
    
    let current_dir = std::env::current_dir()?;
    let mut orchestrator = BuildOrchestrator::from_workspace(&current_dir).await?;
    
    let strategy = if jobs.unwrap_or(1) > 1 {
        BuildStrategy::Parallel
    } else {
        BuildStrategy::Sequential
    };
    
    let results = orchestrator.build_workspace(strategy).await?;
    
    let success_count = results.values().filter(|r| r.success).count();
    let total_count = results.len();
    
    if success_count == total_count {
        println!("{} Build completed successfully ({}/{} projects)", 
                 "✅".green().bold(), success_count, total_count);
    } else {
        println!("{} Build completed with errors ({}/{} projects)", 
                 "❌".red().bold(), success_count, total_count);
        
        // Show errors
        for (project, result) in &results {
            if !result.success {
                println!("❌ {}: {} errors", project, result.errors.len());
                for error in &result.errors {
                    println!("  {}", error);
                }
            }
        }
        
        process::exit(1);
    }
    
    Ok(())
}

async fn handle_clean(matches: &ArgMatches, _global_matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let all = matches.get_flag("all");
    let cache = matches.get_flag("cache");
    
    println!("{} build artifacts", "Cleaning".yellow().bold());
    
    // TODO: Implement cleaning
    println!("{} Clean not yet implemented", "Warning".yellow().bold());
    
    Ok(())
}

async fn handle_check(matches: &ArgMatches, global_matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let inputs: Vec<&String> = matches.get_many::<String>("input").unwrap().collect();
    let json = matches.get_flag("json");
    
    println!("{} {} files", "Checking".blue().bold(), inputs.len());
    
    let mut errors = Vec::new();
    
    for input in inputs {
        if Path::new(input).is_file() {
            let source = fs::read_to_string(input)?;
            match check_syntax(&source) {
                Ok(_) => {
                    if !json {
                        println!("{}: {} OK", input, "✓".green().bold());
                    }
                }
                Err(e) => {
                    if !json {
                        println!("{}: {} {}", input, "✗".red().bold(), e);
                    }
                    errors.push((input.clone(), e.to_string()));
                }
            }
        } else {
            // Handle directory
            for entry in glob::glob(&format!("{}/**/*.csd", input))? {
                let path = entry?;
                let source = fs::read_to_string(&path)?;
                match check_syntax(&source) {
                    Ok(_) => {
                        if !json {
                            println!("{}: {} OK", path.display(), "✓".green().bold());
                        }
                    }
                    Err(e) => {
                        if !json {
                            println!("{}: {} {}", path.display(), "✗".red().bold(), e);
                        }
                        errors.push((path.to_string_lossy().to_string(), e.to_string()));
                    }
                }
            }
        }
    }
    
    if json {
        let result = serde_json::json!({
            "errors": errors,
            "success": errors.is_empty()
        });
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else if errors.is_empty() {
        println!("{} All files passed checks", "✓".green().bold());
    } else {
        println!("{} {} files failed checks", "Summary:".bold(), errors.len());
    }
    
    if !errors.is_empty() {
        process::exit(1);
    }
    
    Ok(())
}

async fn handle_repl(matches: &ArgMatches, _global_matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let no_history = matches.get_flag("no-history");
    let startup = matches.get_one::<String>("startup").map(|s| s.as_str());
    
    println!("{} CURSED REPL", "Starting".blue().bold());
    
    let mut repl = CursedRepl::new()?;
    
    if let Some(startup_file) = startup {
        println!("Loading startup file: {}", startup_file);
        if let Err(e) = repl.load_startup_file(startup_file) {
            println!("{} Failed to load startup file: {}", "Warning".yellow().bold(), e);
        }
    }
    
    // Start the REPL loop
    repl.run_repl_loop(no_history)?;
    
    Ok(())
}

fn get_optimization_config(matches: &ArgMatches) -> Result<OptimizationConfig, Box<dyn std::error::Error>> {
    let level_str = matches.get_one::<String>("optimization").unwrap();
    
    let level = match level_str.as_str() {
        "0" => OptOptimizationLevel::None,
        "1" => OptOptimizationLevel::Less,
        "2" => OptOptimizationLevel::Default,
        "3" => OptOptimizationLevel::Aggressive,
        "s" => OptOptimizationLevel::Size,
        "z" => OptOptimizationLevel::SizeAggressive,
        "debug" => OptOptimizationLevel::None,
        "release" => OptOptimizationLevel::Aggressive,
        _ => return Err(format!("Invalid optimization level: {}", level_str).into()),
    };
    
    Ok(OptimizationConfig::new(level))
}

fn check_syntax(source: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Simple syntax check without async runtime
    let lexer = cursed::lexer::Lexer::new(source.to_string());
    let mut parser = cursed::parser::Parser::new(lexer)?;
    let _program = parser.parse_program()?;
    
    let errors = parser.errors();
    if !errors.is_empty() {
        let error_msgs: Vec<String> = errors.iter().map(|e| e.to_string()).collect();
        return Err(format!("Parse errors: {}", error_msgs.join(", ")).into());
    }
    
    Ok(())
}

fn find_test_files(pattern: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut files = Vec::new();
    
    for entry in glob::glob(pattern)? {
        let path = entry?;
        if path.is_file() {
            files.push(path.to_string_lossy().to_string());
        }
    }
    
    Ok(files)
}

// Documentation generation implementation

#[derive(Debug)]
struct DocItem {
    name: String,
    item_type: String,
    description: String,
    file_path: PathBuf,
    line_number: usize,
    parameters: Vec<Parameter>,
    return_type: Option<String>,
    examples: Vec<String>,
}

#[derive(Debug)]
struct Parameter {
    name: String,
    param_type: String,
    description: String,
}

#[derive(Debug)]
struct DocumentationIndex {
    items: Vec<DocItem>,
    files: Vec<PathBuf>,
}

async fn generate_documentation(input: &str, output: &str, format: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Scanning for source files...");
    
    let doc_index = scan_for_docs(input)?;
    
    println!("Found {} documented items in {} files", 
        doc_index.items.len(), 
        doc_index.files.len()
    );
    
    // Create output directory
    fs::create_dir_all(output)?;
    
    match format {
        "html" => generate_html_docs(&doc_index, output, "CURSED Documentation")?,
        "markdown" => generate_markdown_docs(&doc_index, output, "CURSED Documentation")?,
        "json" => generate_json_docs(&doc_index, output)?,
        _ => return Err("Unsupported format".into()),
    }
    
    println!("{} Documentation generated successfully in {}", "✓".green(), output);
    Ok(())
}

async fn serve_documentation(docs_dir: &str, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    println!("{} Starting documentation server...", "→".cyan());
    println!("Serving {} on http://localhost:{}", docs_dir, port);
    println!("Press Ctrl+C to stop");
    
    // Use simple HTTP server
    use std::net::{TcpListener, TcpStream};
    use std::io::prelude::*;
    
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;
    
    for stream in listener.incoming() {
        let stream = stream?;
        handle_connection(stream, docs_dir)?;
    }
    
    Ok(())
}

fn handle_connection(mut stream: std::net::TcpStream, docs_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    use std::io::prelude::*;
    
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;
    
    let request = String::from_utf8_lossy(&buffer[..]);
    let request_line = request.lines().next().unwrap_or("");
    
    let (status_line, filename) = if request_line.starts_with("GET / ") {
        ("HTTP/1.1 200 OK", format!("{}/index.html", docs_dir))
    } else if request_line.starts_with("GET /style.css ") {
        ("HTTP/1.1 200 OK", format!("{}/style.css", docs_dir))
    } else {
        ("HTTP/1.1 404 NOT FOUND", format!("{}/404.html", docs_dir))
    };
    
    let contents = fs::read_to_string(&filename).unwrap_or_else(|_| {
        if filename.ends_with("404.html") {
            "<html><body><h1>404 Not Found</h1></body></html>".to_string()
        } else {
            "<html><body><h1>Error loading file</h1></body></html>".to_string()
        }
    });
    
    let response = format!("{}\r\n\r\n{}", status_line, contents);
    stream.write(response.as_bytes())?;
    stream.flush()?;
    
    Ok(())
}

fn scan_for_docs(input: &str) -> Result<DocumentationIndex, Box<dyn std::error::Error>> {
    let mut items = Vec::new();
    let mut files = Vec::new();
    
    let input_path = Path::new(input);
    
    if input_path.is_file() && input_path.extension() == Some(std::ffi::OsStr::new("csd")) {
        // Single file
        println!("Processing single file: {}", input_path.display());
        files.push(input_path.to_path_buf());
        let file_items = parse_cursed_file(input_path)?;
        items.extend(file_items);
    } else {
        // Directory - scan for CURSED source files
        let local_pattern = format!("{}/*.csd", input);
        println!("Scanning pattern: {}", local_pattern);
        
        for entry in glob(&local_pattern)? {
            let path = entry?;
            if path.is_file() {
                println!("Found file: {}", path.display());
                files.push(path.clone());
                let file_items = parse_cursed_file(&path)?;
                items.extend(file_items);
            }
        }
    }
    
    Ok(DocumentationIndex { items, files })
}

fn parse_cursed_file(file_path: &Path) -> Result<Vec<DocItem>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let mut items = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i].trim();
        
        // Look for documentation comments
        if line.starts_with("//!") || line.starts_with("///") {
            let (doc_item, next_i) = parse_doc_comment(&lines, i, file_path)?;
            if let Some(item) = doc_item {
                items.push(item);
            }
            i = next_i;
        } else {
            i += 1;
        }
    }
    
    Ok(items)
}

fn parse_doc_comment(lines: &[&str], start_idx: usize, file_path: &Path) -> Result<(Option<DocItem>, usize), Box<dyn std::error::Error>> {
    let mut description = String::new();
    let mut examples = Vec::new();
    let mut i = start_idx;
    
    // Collect documentation lines
    while i < lines.len() {
        let line = lines[i].trim();
        if line.starts_with("//!") || line.starts_with("///") {
            let content = line.trim_start_matches("//!").trim_start_matches("///").trim();
            if content.starts_with("# Example") || content.starts_with("## Example") {
                // Start collecting example
                i += 1;
                while i < lines.len() && (lines[i].trim().starts_with("///") || lines[i].trim().starts_with("//!")) {
                    let example_line = lines[i].trim().trim_start_matches("///").trim_start_matches("//!").trim();
                    if !example_line.is_empty() {
                        examples.push(example_line.to_string());
                    }
                    i += 1;
                }
            } else {
                description.push_str(content);
                description.push('\n');
            }
        } else {
            break;
        }
        i += 1;
    }
    
    // Look for the next significant line (function, struct, etc.)
    while i < lines.len() && lines[i].trim().is_empty() {
        i += 1;
    }
    
    if i < lines.len() {
        let next_line = lines[i].trim();
        let (name, item_type) = if next_line.starts_with("fn ") {
            let name = extract_function_name(next_line);
            (name, "function".to_string())
        } else if next_line.starts_with("struct ") {
            let name = extract_struct_name(next_line);
            (name, "struct".to_string())
        } else if next_line.starts_with("enum ") {
            let name = extract_enum_name(next_line);
            (name, "enum".to_string())
        } else if next_line.starts_with("interface ") {
            let name = extract_interface_name(next_line);
            (name, "interface".to_string())
        } else {
            ("unknown".to_string(), "item".to_string())
        };
        
        if name != "unknown" {
            return Ok((Some(DocItem {
                name,
                item_type,
                description: description.trim().to_string(),
                file_path: file_path.to_path_buf(),
                line_number: i + 1,
                parameters: Vec::new(), // TODO: Parse parameters
                return_type: None, // TODO: Parse return type
                examples,
            }), i + 1));
        }
    }
    
    Ok((None, i))
}

fn extract_function_name(line: &str) -> String {
    line.split_whitespace()
        .nth(1)
        .and_then(|s| s.split('(').next())
        .unwrap_or("unknown")
        .to_string()
}

fn extract_struct_name(line: &str) -> String {
    line.split_whitespace()
        .nth(1)
        .unwrap_or("unknown")
        .to_string()
}

fn extract_enum_name(line: &str) -> String {
    line.split_whitespace()
        .nth(1)
        .unwrap_or("unknown")
        .to_string()
}

fn extract_interface_name(line: &str) -> String {
    line.split_whitespace()
        .nth(1)
        .unwrap_or("unknown")
        .to_string()
}

fn generate_html_docs(doc_index: &DocumentationIndex, output: &str, title: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut items_html = String::new();
    for item in &doc_index.items {
        items_html.push_str(&format!(
            r#"<div class="doc-item">
                <h3>{} <span class="type">({})</span></h3>
                <p class="description">{}</p>
                <p class="source">Source: {}:{}</p>
                {}
            </div>"#,
            item.name,
            item.item_type,
            item.description,
            item.file_path.display(),
            item.line_number,
            if !item.examples.is_empty() {
                format!("<div class=\"examples\"><h4>Examples:</h4><pre><code>{}</code></pre></div>", 
                    item.examples.join("\n"))
            } else {
                String::new()
            }
        ));
    }
    
    let html_content = format!(r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>{}</title>
    <link rel="stylesheet" href="style.css">
</head>
<body>
    <header>
        <h1>{}</h1>
        <p>Documentation for {} items</p>
    </header>
    <main>
        {}
    </main>
</body>
</html>"#, title, title, doc_index.items.len(), items_html);
    
    let index_path = Path::new(output).join("index.html");
    fs::write(index_path, html_content)?;
    
    // Generate CSS
    let css_content = r#"body {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    line-height: 1.6;
    color: #333;
    max-width: 800px;
    margin: 0 auto;
    padding: 20px;
}

header {
    border-bottom: 2px solid #eee;
    margin-bottom: 30px;
    padding-bottom: 20px;
}

.doc-item {
    margin-bottom: 30px;
    padding: 20px;
    border: 1px solid #ddd;
    border-radius: 5px;
}

.type {
    color: #666;
    font-size: 0.8em;
}

.description {
    margin: 10px 0;
}

.source {
    color: #888;
    font-size: 0.9em;
}

.examples {
    margin-top: 15px;
}

.examples pre {
    background: #f5f5f5;
    padding: 10px;
    border-radius: 3px;
    overflow-x: auto;
}"#;
    
    let css_path = Path::new(output).join("style.css");
    fs::write(css_path, css_content)?;
    
    Ok(())
}

fn generate_markdown_docs(doc_index: &DocumentationIndex, output: &str, title: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut markdown = format!("# {}\n\n", title);
    markdown.push_str(&format!("Generated documentation for {} items.\n\n", doc_index.items.len()));
    
    let mut items_by_type: HashMap<String, Vec<&DocItem>> = HashMap::new();
    for item in &doc_index.items {
        items_by_type.entry(item.item_type.clone()).or_default().push(item);
    }
    
    for (item_type, items) in items_by_type {
        markdown.push_str(&format!("## {}\n\n", item_type.to_uppercase()));
        
        for item in items {
            markdown.push_str(&format!("### {}\n\n", item.name));
            markdown.push_str(&format!("{}\n\n", item.description));
            markdown.push_str(&format!("**Source:** `{}:{}`\n\n", item.file_path.display(), item.line_number));
            
            if !item.examples.is_empty() {
                markdown.push_str("**Examples:**\n\n");
                markdown.push_str("```cursed\n");
                markdown.push_str(&item.examples.join("\n"));
                markdown.push_str("\n```\n\n");
            }
            
            markdown.push_str("---\n\n");
        }
    }
    
    let readme_path = Path::new(output).join("README.md");
    fs::write(readme_path, markdown)?;
    
    Ok(())
}

fn generate_json_docs(doc_index: &DocumentationIndex, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json_data = json!({
        "title": "CURSED Documentation",
        "generated_at": std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        "items": doc_index.items.iter().map(|item| {
            json!({
                "name": item.name,
                "type": item.item_type,
                "description": item.description,
                "file_path": item.file_path.to_string_lossy(),
                "line_number": item.line_number,
                "examples": item.examples
            })
        }).collect::<Vec<_>>(),
        "files": doc_index.files.iter().map(|f| f.to_string_lossy()).collect::<Vec<_>>()
    });
    
    let json_path = Path::new(output).join("docs.json");
    fs::write(json_path, serde_json::to_string_pretty(&json_data)?)?;
    
    Ok(())
}
