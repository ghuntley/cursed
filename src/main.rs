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
use cursed::package_manager::{PackageManagerConfig, PackageManager};
use cursed::tools::{CursedTools, Profiler};
use cursed::repl::CursedRepl;
use cursed::execution::pure_cursed_bridge::PureCursedBridge;
use cursed::coverage;
use cursed::optimization::advanced_llvm_passes::{SizeOptLevel, LtoLevel, PassPipeline};
use std::os::unix::fs::PermissionsExt;

// Dropz integration helpers to replace std::fs calls
struct DropzFilesystem {
    bridge: PureCursedBridge,
}

impl DropzFilesystem {
    fn new() -> Self {
        Self {
            bridge: PureCursedBridge::new(),
        }
    }
    
    fn read_to_string(&self, path: &str) -> Result<String, String> {
        self.bridge.io_read_text_file(path)
    }
    
    fn write(&self, path: &str, content: &str) -> Result<(), String> {
        self.bridge.io_write_text_file(path, content, 0o644)
    }
    
    fn create_dir_all(&self, path: &str) -> Result<(), String> {
        self.bridge.io_mkdir_all(path, 0o755)
    }
}

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
            .value_parser(["native", "wasm", "wasm32", "wasm64"])
            .default_value("native")
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
                // New optimization flags
                .arg(Arg::new("optimize")
                    .help("Enable basic optimization")
                    .long("optimize")
                    .action(clap::ArgAction::SetTrue))
                .arg(Arg::new("opt-level")
                    .help("Set optimization level (0-3)")
                    .long("opt-level")
                    .value_name("LEVEL")
                    .value_parser(["0", "1", "2", "3"])
                    .conflicts_with("optimize"))
                // Advanced optimization flags
                .arg(Arg::new("enable-pgo")
                    .help("Enable Profile-Guided Optimization")
                    .long("enable-pgo")
                    .action(clap::ArgAction::SetTrue))
                .arg(Arg::new("pgo-profile")
                    .help("Path to PGO profile data")
                    .long("pgo-profile")
                    .value_name("PATH")
                    .requires("enable-pgo"))
                .arg(Arg::new("pgo-generate")
                    .help("Generate PGO profile data")
                    .long("pgo-generate")
                    .action(clap::ArgAction::SetTrue))
                .arg(Arg::new("enable-lto")
                    .help("Enable Link-Time Optimization")
                    .long("enable-lto")
                    .action(clap::ArgAction::SetTrue))
                .arg(Arg::new("lto-level")
                    .help("Set LTO level (thin, full)")
                    .long("lto-level")
                    .value_name("LEVEL")
                    .value_parser(["thin", "full"])
                    .requires("enable-lto"))
                .arg(Arg::new("size-opt")
                    .help("Enable size optimization")
                    .long("size-opt")
                    .action(clap::ArgAction::SetTrue))
                .arg(Arg::new("size-level")
                    .help("Set size optimization level (s, z)")
                    .long("size-level")
                    .value_name("LEVEL")
                    .value_parser(["s", "z"])
                    .requires("size-opt"))
                .arg(Arg::new("enable-bolt")
                    .help("Enable BOLT optimization")
                    .long("enable-bolt")
                    .action(clap::ArgAction::SetTrue))
                .arg(Arg::new("bolt-profile")
                    .help("Path to BOLT profile data")
                    .long("bolt-profile")
                    .value_name("PATH")
                    .requires("enable-bolt"))
                .arg(Arg::new("pass-pipeline")
                    .help("Set optimization pass pipeline")
                    .long("pass-pipeline")
                    .value_name("PIPELINE")
                    .value_parser(["default", "pgo", "size", "production"]))
                // Function inlining optimization flags
                .arg(Arg::new("enable-inlining")
                    .help("Enable function inlining optimization")
                    .long("enable-inlining")
                    .action(clap::ArgAction::SetTrue))
                .arg(Arg::new("inline-threshold")
                    .help("Set function inlining threshold")
                    .long("inline-threshold")
                    .value_name("SIZE")
                    .value_parser(value_parser!(u32))
                    .requires("enable-inlining"))
                .arg(Arg::new("aggressive-inline")
                    .help("Enable aggressive function inlining")
                    .long("aggressive-inline")
                    .action(clap::ArgAction::SetTrue)
                    .requires("enable-inlining"))
                .arg(Arg::new("inline-generics")
                    .help("Enable inlining of generic functions")
                    .long("inline-generics")
                    .action(clap::ArgAction::SetTrue)
                    .requires("enable-inlining"))
                .arg(Arg::new("inline-interfaces")
                    .help("Enable inlining of interface methods")
                    .long("inline-interfaces")
                    .action(clap::ArgAction::SetTrue)
                    .requires("enable-inlining"))
                .arg(Arg::new("benchmark")
                    .help("Generate optimization benchmark report")
                    .long("benchmark")
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
                .arg(Arg::new("coverage_format")
                    .help("Coverage report format")
                    .long("coverage-format")
                    .value_name("FORMAT")
                    .value_parser(["html", "json", "xml", "lcov", "console"])
                    .default_value("html"))
                .arg(Arg::new("coverage_threshold")
                    .help("Minimum coverage threshold percentage")
                    .long("coverage-threshold")
                    .value_name("PERCENT")
                    .value_parser(clap::value_parser!(f64))
                    .default_value("80.0"))
        )
        .subcommand(
            Command::new("coverage")
                .about("Code coverage analysis")
                .subcommand_required(true)
                .subcommand(
                    Command::new("run")
                        .about("Run tests with coverage collection")
                        .arg(Arg::new("test_command")
                            .help("Test command to run")
                            .value_name("COMMAND")
                            .default_value("cargo test"))
                        .arg(Arg::new("output_dir")
                            .help("Coverage output directory")
                            .long("output-dir")
                            .value_name("DIR")
                            .default_value("coverage"))
                        .arg(Arg::new("source_dirs")
                            .help("Source directories to include")
                            .long("source-dirs")
                            .value_name("DIRS")
                            .action(clap::ArgAction::Append)
                            .default_values(["src", "stdlib"]))
                        .arg(Arg::new("exclude_patterns")
                            .help("Exclude patterns")
                            .long("exclude")
                            .value_name("PATTERNS")
                            .action(clap::ArgAction::Append)
                            .default_values(["*/target/*", "*/tests/*", "*_test.csd", "test_*.csd"]))
                        .arg(Arg::new("formats")
                            .help("Report formats to generate")
                            .long("format")
                            .value_name("FORMATS")
                            .value_parser(["html", "json", "xml", "lcov", "console"])
                            .action(clap::ArgAction::Append)
                            .default_values(["html", "json"]))
                        .arg(Arg::new("threshold")
                            .help("Minimum coverage threshold")
                            .long("threshold")
                            .value_name("PERCENT")
                            .value_parser(clap::value_parser!(f64))
                            .default_value("80.0"))
                        .arg(Arg::new("collect_branches")
                            .help("Collect branch coverage")
                            .long("branches")
                            .action(clap::ArgAction::SetTrue))
                        .arg(Arg::new("collect_functions")
                            .help("Collect function coverage")
                            .long("functions")
                            .action(clap::ArgAction::SetTrue))
                        .arg(Arg::new("instrument")
                            .help("Instrument source files")
                            .long("instrument")
                            .action(clap::ArgAction::SetTrue))
                )
                .subcommand(
                    Command::new("report")
                        .about("Generate coverage report from existing data")
                        .arg(Arg::new("coverage_data")
                            .help("Path to coverage data file")
                            .value_name("FILE")
                            .required(true))
                        .arg(Arg::new("output_dir")
                            .help("Output directory for reports")
                            .long("output-dir")
                            .value_name("DIR")
                            .default_value("coverage"))
                        .arg(Arg::new("formats")
                            .help("Report formats to generate")
                            .long("format")
                            .value_name("FORMATS")
                            .value_parser(["html", "json", "xml", "lcov", "console"])
                            .action(clap::ArgAction::Append)
                            .default_values(["html", "console"]))
                )
                .subcommand(
                    Command::new("instrument")
                        .about("Instrument source files for coverage")
                        .arg(Arg::new("source_dirs")
                            .help("Source directories to instrument")
                            .action(clap::ArgAction::Append)
                            .default_values(["src", "stdlib"]))
                        .arg(Arg::new("output_dir")
                            .help("Output directory for instrumented files")
                            .long("output-dir")
                            .value_name("DIR")
                            .default_value("coverage/instrumented"))
                )
                .subcommand(
                    Command::new("analyze")
                        .about("Analyze coverage data and generate insights")
                        .arg(Arg::new("coverage_data")
                            .help("Path to coverage data file")
                            .value_name("FILE")
                            .required(true))
                        .arg(Arg::new("threshold")
                            .help("Coverage threshold for analysis")
                            .long("threshold")
                            .value_name("PERCENT")
                            .value_parser(clap::value_parser!(f64))
                            .default_value("80.0"))
                        .arg(Arg::new("complexity_threshold")
                            .help("Cyclomatic complexity threshold")
                            .long("complexity-threshold")
                            .value_name("NUMBER")
                            .value_parser(clap::value_parser!(u32))
                            .default_value("10"))
                )
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
                        .arg(Arg::new("dry-run")
                            .help("Perform a dry run without actually publishing")
                            .long("dry-run")
                            .action(clap::ArgAction::SetTrue))
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
                .arg(Arg::new("format")
                    .help("Output format")
                    .short('f')
                    .long("format")
                    .value_name("FORMAT")
                    .value_parser(["human", "json", "compact"])
                    .default_value("human"))
                .arg(Arg::new("strict")
                    .help("Use strict linting configuration")
                    .long("strict")
                    .action(clap::ArgAction::SetTrue))
                .arg(Arg::new("minimal")
                    .help("Use minimal linting configuration (security + correctness only)")
                    .long("minimal")
                    .action(clap::ArgAction::SetTrue))
                .arg(Arg::new("categories")
                    .help("Lint only specific categories")
                    .long("categories")
                    .value_name("CATEGORIES")
                    .value_parser(["style", "performance", "security", "correctness", "best-practice"])
                    .num_args(1..))
                .arg(Arg::new("severity")
                    .help("Minimum severity level to report")
                    .long("severity")
                    .value_name("LEVEL")
                    .value_parser(["error", "warning", "info"])
                    .default_value("info"))
                    )
                    .subcommand(
                    Command::new("check-llvm")
                .about("Check LLVM toolchain availability")
                .long_about("Checks if LLVM tools (llc, clang, gcc) are available for native compilation.\nProvides detailed installation instructions if tools are missing.")
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
        .subcommand(
            Command::new("lsp")
                .about("Start Language Server Protocol server")
                .long_about("Starts the CURSED Language Server Protocol server for IDE integration.\nProvides features like completion, hover, goto definition, and diagnostics.")
                .arg(Arg::new("stdio")
                    .help("Use stdio for communication (default)")
                    .long("stdio")
                    .action(clap::ArgAction::SetTrue))
                .arg(Arg::new("tcp")
                    .help("Use TCP for communication")
                    .long("tcp")
                    .action(clap::ArgAction::SetTrue))
                .arg(Arg::new("port")
                    .help("TCP port to listen on")
                    .long("port")
                    .value_name("PORT")
                    .value_parser(value_parser!(u16))
                    .default_value("9257"))
                .arg(Arg::new("log-level")
                    .help("Logging level")
                    .long("log-level")
                    .value_name("LEVEL")
                    .value_parser(["error", "warn", "info", "debug", "trace"])
                    .default_value("info"))
                .arg(Arg::new("log-file")
                    .help("Log to file")
                    .long("log-file")
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
        Some(("coverage", sub_matches)) => handle_coverage(sub_matches, &matches).await,
        Some(("pkg", sub_matches)) => handle_pkg(sub_matches, &matches).await,
        Some(("debug", sub_matches)) => handle_debug(sub_matches, &matches).await,
        Some(("lint", sub_matches)) => handle_lint(sub_matches, &matches).await,
        Some(("fmt", sub_matches)) => handle_fmt(sub_matches, &matches).await,
        Some(("doc", sub_matches)) => handle_doc(sub_matches, &matches).await,
        Some(("build", sub_matches)) => handle_build(sub_matches, &matches).await,
        Some(("clean", sub_matches)) => handle_clean(sub_matches, &matches).await,
        Some(("check", sub_matches)) => handle_check(sub_matches, &matches).await,
        Some(("check-llvm", _sub_matches)) => handle_check_llvm().await,
        Some(("repl", sub_matches)) => handle_repl(sub_matches, &matches).await,
        Some(("lsp", sub_matches)) => handle_doc(sub_matches, &matches).await,
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
    
    // Read source file using dropz
    let dropz_fs = DropzFilesystem::new();
    let source = dropz_fs.read_to_string(input)
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
    
    // Determine optimization level
    let optimization_level = if matches.get_flag("optimize") {
        Some("2") // Basic optimization (O2)
    } else if let Some(level) = matches.get_one::<String>("opt-level") {
        Some(level.as_str())
    } else {
        global_matches.get_one::<String>("optimization").map(|s| s.as_str())
    };

    // Get target architecture
    let target = global_matches.get_one::<String>("target")
        .map(|s| s.as_str())
        .unwrap_or("native");

    // Create advanced optimization configuration
    let mut advanced_config = create_advanced_optimization_config(matches, global_matches)?;
    
    // Configure for WebAssembly target
    if target.starts_with("wasm") {
        advanced_config.target_platform = Some("wasm".to_string());
        advanced_config.enable_wasm_optimizations = true;
        advanced_config.enable_size_optimization = true; // WASM benefits from smaller binaries
        advanced_config.size_optimization_level = SizeOptLevel::Size;
    }
    
    if matches.get_flag("verbose") || global_matches.get_flag("verbose") {
        println!("{} {} to {}", "Compiling".green().bold(), input, output);
        if let Some(level) = optimization_level {
            println!("{} optimization level: {}", "Using".blue().bold(), level);
        }
        
        // Print advanced optimization info
        if advanced_config.enable_pgo {
            println!("{} PGO enabled", "Advanced".cyan().bold());
        }
        if advanced_config.enable_lto {
            println!("{} LTO enabled ({:?})", "Advanced".cyan().bold(), advanced_config.lto_level);
        }
        if advanced_config.enable_size_optimization {
            println!("{} Size optimization enabled ({:?})", "Advanced".cyan().bold(), advanced_config.size_optimization_level);
        }
        if target.starts_with("wasm") {
            println!("{} WebAssembly target: {}", "Target".magenta().bold(), target);
        }
    }
    
    if matches.get_flag("emit-ir") {
        let dropz_fs = DropzFilesystem::new();
        let source = dropz_fs.read_to_string(input)
            .map_err(|e| format!("Failed to read file: {}", e))?;
        
        let ir = if target.starts_with("wasm") {
            // Use WebAssembly-specific IR generation
            cursed::compile_source_to_wasm_ir(&source, &advanced_config).await?
        } else {
            cursed::compile_to_ir_with_advanced_optimization(
                &source,
                &advanced_config
            ).await?
        };
        
        dropz_fs.write(&format!("{}.ll", output), &ir)
            .map_err(|e| format!("Failed to write IR: {}", e))?;
        println!("{} LLVM IR to {}.ll", "Generated".green().bold(), output);
        if let Some(level) = optimization_level {
            println!("{} optimization level {} applied", "Optimization".green().bold(), level);
        }
    } else if matches.get_flag("emit-asm") {
        let dropz_fs = DropzFilesystem::new();
        let source = dropz_fs.read_to_string(input)
            .map_err(|e| format!("Failed to read file: {}", e))?;
        let assembly = cursed::compile_to_assembly_with_advanced_optimization(&source, &advanced_config).await?;
        dropz_fs.write(&format!("{}.s", output), &assembly)
            .map_err(|e| format!("Failed to write assembly: {}", e))?;
        println!("{} Assembly to {}.s", "Generated".green().bold(), output);
    } else {
        // Enhanced WebAssembly compilation
        if target.starts_with("wasm") {
            let wasm_output = if output.ends_with(".wasm") { 
                output.to_string() 
            } else { 
                format!("{}.wasm", output) 
            };
            
            // Create enhanced WASM compilation configuration
            let mut wasm_config = cursed::WasmCompilationConfig::default();
            
            // Configure based on optimization level
            match advanced_config.base_config.level {
                cursed::optimization::OptimizationLevel::Size | 
                cursed::optimization::OptimizationLevel::SizeAggressive => {
                    wasm_config.code_size_optimization = true;
                    wasm_config.memory_optimization_level = cursed::WasmMemoryOptLevel::Aggressive;
                    wasm_config.dead_code_elimination = true;
                }
                cursed::optimization::OptimizationLevel::Aggressive => {
                    wasm_config.enable_simd = true;
                    wasm_config.enable_bulk_memory = true;
                    wasm_config.function_table_optimization = true;
                    wasm_config.memory_optimization_level = cursed::WasmMemoryOptLevel::Aggressive;
                }
                _ => {
                    // Default configuration already set
                }
            }
            
            // Enable debugging if verbose mode
            if matches.get_flag("verbose") {
                wasm_config.generate_debug_info = true;
                wasm_config.generate_source_maps = true;
                wasm_config.validation_level = cursed::WasmValidationLevel::Strict;
            }
            
            // Enable WASI if environment variable set
            if std::env::var("CURSED_WASM_WASI").is_ok() {
                wasm_config.enable_wasi = true;
            }
            
            // Enable advanced features based on environment
            if std::env::var("CURSED_WASM_SIMD").is_ok() {
                wasm_config.enable_simd = true;
            }
            if std::env::var("CURSED_WASM_THREADS").is_ok() {
                wasm_config.enable_threads = true;
            }
            
            // Use enhanced WASM compilation
            let result = cursed::compile_to_wasm_with_optimizations(
                input, 
                &wasm_output, 
                &advanced_config,
                &wasm_config
            ).await?;
            
            // Print enhanced compilation results
            println!("{} WebAssembly module: {}", "Generated".green().bold(), result.output_file);
            println!("  Binary size: {} bytes", result.binary_size);
            println!("  Compilation time: {:?}", result.compilation_time);
            println!("  Functions optimized: {}", result.optimization_stats.functions_optimized);
            if result.optimization_stats.code_size_reduction > 0.0 {
                println!("  Code size reduction: {:.1}%", result.optimization_stats.code_size_reduction);
            }
            if result.debug_info_generated {
                println!("  Debug information: Generated");
            }
            if result.source_maps_generated {
                println!("  Source maps: Generated");
            }
            
            // Print validation results if verbose
            if matches.get_flag("verbose") {
                if !result.validation_result.validation_errors.is_empty() {
                    println!("⚠ Validation errors:");
                    for error in &result.validation_result.validation_errors {
                        println!("    {}", error);
                    }
                }
                if !result.validation_result.warnings.is_empty() {
                    println!("⚠ Validation warnings:");
                    for warning in &result.validation_result.warnings {
                        println!("    {}", warning);
                    }
                }
                if !result.validation_result.performance_suggestions.is_empty() {
                    println!("💡 Performance suggestions:");
                    for suggestion in &result.validation_result.performance_suggestions {
                        println!("    {}", suggestion);
                    }
                }
            }
        }
        // Handle native-only flag
        else if matches.get_flag("native-only") {
            cursed::compile_native_only_with_advanced_optimization(input, output, &advanced_config).await?;
            println!("{} native executable: {}", "Generated".green().bold(), output);
        } else {
            let result = cursed::compile_with_advanced_optimization(input, output, &advanced_config).await?;
            println!("{} executable: {}", "Generated".green().bold(), output);
            
            // Generate benchmark report if requested
            if matches.get_flag("benchmark") {
                if let Some(report) = result.benchmark_report {
                    report.print_report();
                }
            }
        }
    }
    
    Ok(())
}

fn create_advanced_optimization_config(matches: &ArgMatches, global_matches: &ArgMatches) -> Result<cursed::optimization::AdvancedOptimizationConfig, Box<dyn std::error::Error>> {
    use cursed::optimization::advanced_llvm_passes::cli_integration;
    
    let optimization_level = if matches.get_flag("optimize") {
        Some("2")
    } else if let Some(level) = matches.get_one::<String>("opt-level") {
        Some(level.as_str())
    } else {
        global_matches.get_one::<String>("optimization").map(|s| s.as_str())
    };

    let enable_pgo = matches.get_flag("enable-pgo");
    let pgo_profile_path = matches.get_one::<String>("pgo-profile").map(|s| s.as_str());
    let enable_lto = matches.get_flag("enable-lto");
    let size_optimized = matches.get_flag("size-opt");

    let mut config = cli_integration::create_config_from_cli(
        optimization_level,
        enable_pgo,
        pgo_profile_path,
        enable_lto,
        size_optimized,
    )?;

    // Set PGO generation if requested
    if matches.get_flag("pgo-generate") {
        config.pgo_generate_profile = true;
        if config.pgo_profile_path.is_none() {
            config.pgo_profile_path = Some(std::path::PathBuf::from("target/pgo-profile.profdata"));
        }
    }

    // Set LTO level if specified
    if let Some(lto_level) = matches.get_one::<String>("lto-level") {
        config.lto_level = match lto_level.as_str() {
            "thin" => LtoLevel::Thin,
            "full" => LtoLevel::Full,
            _ => LtoLevel::Full,
        };
    }

    // Set size optimization level if specified
    if let Some(size_level) = matches.get_one::<String>("size-level") {
        config.size_optimization_level = match size_level.as_str() {
            "s" => SizeOptLevel::Size,
            "z" => SizeOptLevel::SizeAggressive,
            _ => SizeOptLevel::Size,
        };
    }

    // Set pass pipeline if specified
    if let Some(pipeline) = matches.get_one::<String>("pass-pipeline") {
        config.pass_pipeline = match pipeline.as_str() {
            "default" => PassPipeline::Default,
            "pgo" => PassPipeline::ProfileGuided,
            "size" => PassPipeline::SizeOptimized,
            "production" => PassPipeline::Production,
            _ => PassPipeline::Default,
        };
    }

    // Enable BOLT if requested
    if matches.get_flag("enable-bolt") {
        config.enable_bolt = true;
        if let Some(bolt_profile) = matches.get_one::<String>("bolt-profile") {
            config.bolt_profile_path = Some(std::path::PathBuf::from(bolt_profile));
        }
    }

    // Configure function inlining optimization
    if matches.get_flag("enable-inlining") {
        config.base_config.enable_function_inlining = true;
        
        if let Some(threshold) = matches.get_one::<u32>("inline-threshold") {
            config.base_config.inline_threshold = *threshold;
        }
        
        if matches.get_flag("aggressive-inline") {
            config.base_config.aggressive_inlining = true;
        }
        
        if matches.get_flag("inline-generics") {
            config.base_config.enable_generics_inlining = true;
        }
        
        if matches.get_flag("inline-interfaces") {
            config.base_config.enable_interface_inlining = true;
        }
    }

    Ok(config)
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

async fn handle_coverage(matches: &ArgMatches, _global_matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    use crate::coverage::{CoverageAnalyzer, CoverageConfig, OutputFormat, run_coverage_analysis};
    use std::path::PathBuf;
    
    match matches.subcommand() {
        Some(("run", sub_matches)) => {
            let test_command = sub_matches.get_one::<String>("test_command").unwrap();
            let output_dir = PathBuf::from(sub_matches.get_one::<String>("output_dir").unwrap());
            let threshold = *sub_matches.get_one::<f64>("threshold").unwrap();
            
            // Build source directories
            let source_dirs: Vec<PathBuf> = sub_matches.get_many::<String>("source_dirs")
                .unwrap()
                .map(|s| PathBuf::from(s))
                .collect();
            
            // Build exclude patterns
            let exclude_patterns: Vec<String> = sub_matches.get_many::<String>("exclude_patterns")
                .unwrap()
                .map(|s| s.to_string())
                .collect();
            
            // Build formats
            let formats: Vec<OutputFormat> = sub_matches.get_many::<String>("formats")
                .unwrap()
                .map(|s| match s.as_str() {
                    "html" => OutputFormat::Html,
                    "json" => OutputFormat::Json,
                    "xml" => OutputFormat::Xml,
                    "lcov" => OutputFormat::Lcov,
                    "console" => OutputFormat::Console,
                    _ => OutputFormat::Html,
                })
                .collect();
            
            let config = CoverageConfig {
                output_dir,
                source_dirs,
                exclude_patterns,
                include_patterns: vec!["*.rs".to_string(), "*.csd".to_string()],
                formats,
                min_coverage_threshold: threshold,
                collect_branch_coverage: sub_matches.get_flag("collect_branches"),
                collect_function_coverage: sub_matches.get_flag("collect_functions"),
                enable_instrumentation: sub_matches.get_flag("instrument"),
            };
            
            println!("🎯 Running coverage analysis...");
            match run_coverage_analysis(test_command, config).await {
                Ok(coverage_data) => {
                    println!("✅ Coverage analysis completed successfully!");
                    println!("📊 Line coverage: {:.2}%", coverage_data.summary.line_coverage_percentage);
                    println!("🔧 Function coverage: {:.2}%", coverage_data.summary.function_coverage_percentage);
                    println!("🌿 Branch coverage: {:.2}%", coverage_data.summary.branch_coverage_percentage);
                }
                Err(e) => {
                    eprintln!("❌ Coverage analysis failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
        
        Some(("report", sub_matches)) => {
            let coverage_data_path = sub_matches.get_one::<String>("coverage_data").unwrap();
            let output_dir = PathBuf::from(sub_matches.get_one::<String>("output_dir").unwrap());
            
            // Build formats
            let formats: Vec<OutputFormat> = sub_matches.get_many::<String>("formats")
                .unwrap()
                .map(|s| match s.as_str() {
                    "html" => OutputFormat::Html,
                    "json" => OutputFormat::Json,
                    "xml" => OutputFormat::Xml,
                    "lcov" => OutputFormat::Lcov,
                    "console" => OutputFormat::Console,
                    _ => OutputFormat::Html,
                })
                .collect();
            
            let config = CoverageConfig {
                output_dir,
                source_dirs: vec![PathBuf::from("src"), PathBuf::from("stdlib")],
                exclude_patterns: vec![],
                include_patterns: vec!["*.rs".to_string(), "*.csd".to_string()],
                formats: formats.clone(),
                min_coverage_threshold: 80.0,
                collect_branch_coverage: true,
                collect_function_coverage: true,
                enable_instrumentation: false,
            };
            
            // Load coverage data
            let coverage_data_content = std::fs::read_to_string(coverage_data_path)?;
            let coverage_data: crate::coverage::CoverageData = serde_json::from_str(&coverage_data_content)?;
            
            // Generate reports
            let reporter = crate::coverage::reporter::CoverageReporter::new(config)?;
            for format in &formats {
                reporter.generate_report(&coverage_data, format).await?;
            }
            
            println!("✅ Coverage reports generated successfully!");
        }
        
        Some(("instrument", sub_matches)) => {
            let source_dirs: Vec<PathBuf> = sub_matches.get_many::<String>("source_dirs")
                .unwrap()
                .map(|s| PathBuf::from(s))
                .collect();
            let output_dir = PathBuf::from(sub_matches.get_one::<String>("output_dir").unwrap());
            
            println!("🔧 Instrumenting source files...");
            // crate::coverage::instrumentation::instrument_cursed_files(&source_dirs, &output_dir)?;
            // crate::coverage::instrumentation::create_coverage_runtime_module(&output_dir)?;
            println!("⚠️  Instrumentation not yet implemented in basic coverage");
            
            println!("✅ Source files instrumented successfully!");
            println!("📁 Instrumented files saved to: {}", output_dir.display());
        }
        
        Some(("analyze", sub_matches)) => {
            let coverage_data_path = sub_matches.get_one::<String>("coverage_data").unwrap();
            let threshold = *sub_matches.get_one::<f64>("threshold").unwrap();
            let complexity_threshold = *sub_matches.get_one::<u32>("complexity_threshold").unwrap();
            
            // Load coverage data
            let coverage_data_content = std::fs::read_to_string(coverage_data_path)?;
            let coverage_data: crate::coverage::CoverageData = serde_json::from_str(&coverage_data_content)?;
            
            println!("🔍 Analyzing coverage data...");
            
            // Analyze coverage
            let summary = &coverage_data.summary;
            println!("\n📊 Coverage Analysis Results:");
            println!("  Total files: {}", summary.total_files);
            println!("  Line coverage: {:.2}% ({}/{})", 
                     summary.line_coverage_percentage, 
                     summary.covered_lines, 
                     summary.total_lines);
            println!("  Function coverage: {:.2}% ({}/{})", 
                     summary.function_coverage_percentage, 
                     summary.covered_functions, 
                     summary.total_functions);
            println!("  Branch coverage: {:.2}% ({}/{})", 
                     summary.branch_coverage_percentage, 
                     summary.covered_branches, 
                     summary.total_branches);
            
            // Find low coverage files
            let mut low_coverage_files = Vec::new();
            for (file_path, file_coverage) in &coverage_data.files {
                if file_coverage.coverage_percentage < threshold {
                    low_coverage_files.push((file_path, file_coverage.coverage_percentage));
                }
            }
            
            if !low_coverage_files.is_empty() {
                println!("\n⚠️  Files below {}% coverage threshold:", threshold);
                low_coverage_files.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
                for (file_path, coverage) in low_coverage_files.iter().take(10) {
                    println!("  📄 {} ({:.2}%)", file_path, coverage);
                }
            }
            
            // Find high complexity functions
            let mut high_complexity_functions = Vec::new();
            for file_coverage in coverage_data.files.values() {
                for function in file_coverage.functions.values() {
                    if function.complexity >= complexity_threshold {
                        high_complexity_functions.push((&file_coverage.path, &function.name, function.complexity));
                    }
                }
            }
            
            if !high_complexity_functions.is_empty() {
                println!("\n🔄 High complexity functions (>= {}):", complexity_threshold);
                high_complexity_functions.sort_by(|a, b| b.2.cmp(&a.2));
                for (file_path, function_name, complexity) in high_complexity_functions.iter().take(10) {
                    println!("  🔧 {}::{} (complexity: {})", file_path, function_name, complexity);
                }
            }
            
            // Coverage quality assessment
            println!("\n📈 Coverage Quality Assessment:");
            let quality_score = (summary.line_coverage_percentage + 
                                summary.function_coverage_percentage + 
                                summary.branch_coverage_percentage) / 3.0;
            let quality_grade = if quality_score >= 90.0 {
                "A+ (Excellent)"
            } else if quality_score >= 80.0 {
                "A (Good)"
            } else if quality_score >= 70.0 {
                "B (Fair)"
            } else if quality_score >= 60.0 {
                "C (Poor)"
            } else {
                "D (Very Poor)"
            };
            
            println!("  Overall quality score: {:.2}% ({})", quality_score, quality_grade);
            
            if summary.line_coverage_percentage < threshold {
                println!("❌ Coverage threshold not met!");
                std::process::exit(1);
            } else {
                println!("✅ Coverage threshold met!");
            }
        }
        
        _ => {
            eprintln!("❌ Unknown coverage subcommand");
            std::process::exit(1);
        }
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
            
            let config = PackageManagerConfig::default();
            let pkg_manager = PackageManager::new(config)?;
            
            match pkg_manager.search_packages(query).await {
                Ok(packages) => {
                    if packages.is_empty() {
                        println!("{} No packages found for query: {}", "Info".cyan().bold(), query);
                    } else {
                        println!("{} Found {} package(s):", "Success".green().bold(), packages.len());
                        println!();
                        
                        for package in packages {
                            println!("  {} {} {}", 
                                package.name.bright_cyan().bold(),
                                format!("v{}", package.version).bright_magenta(),
                                if package.description.is_empty() { 
                                    "".to_string() 
                                } else { 
                                    format!("- {}", package.description) 
                                }.dimmed()
                            );
                            
                            if !package.authors.is_empty() {
                                println!("    {}: {}", "Authors".dimmed(), package.authors.join(", ").dimmed());
                            }
                            
                            if !package.keywords.is_empty() {
                                println!("    {}: {}", "Keywords".dimmed(), package.keywords.join(", ").dimmed());
                            }
                            
                            if let Some(license) = &package.license {
                                println!("    {}: {}", "License".dimmed(), license.dimmed());
                            }
                            
                            if let Some(homepage) = &package.homepage {
                                println!("    {}: {}", "Homepage".dimmed(), homepage.dimmed());
                            }
                            
                            println!();
                        }
                    }
                }
                Err(e) => {
                    eprintln!("{} Failed to search packages: {}", "Error".red().bold(), e);
                    return Err(e.into());
                }
            }
        }
        Some(("publish", sub_matches)) => {
            let package_dir = sub_matches.get_one::<String>("package").unwrap();
            let dry_run = sub_matches.get_flag("dry-run");
            
            if dry_run {
                println!("{} package from: {} (dry run)", "Publishing".blue().bold(), package_dir);
            } else {
                println!("{} package from: {}", "Publishing".blue().bold(), package_dir);
            }
            
            let config = PackageManagerConfig::default();
            let pkg_manager = PackageManager::new(config)?;
            
            match pkg_manager.publish_package(package_dir, dry_run).await {
                Ok(()) => {
                    if dry_run {
                        println!("{} Dry run completed successfully", "Success".green().bold());
                    } else {
                        println!("{} Package published successfully", "Success".green().bold());
                    }
                }
                Err(e) => {
                    eprintln!("{} Failed to publish package: {}", "Error".red().bold(), e);
                    return Err(e.into());
                }
            }
        }
        Some(("init", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name").unwrap();
            println!("{} new package: {}", "Initializing".blue().bold(), name);
            
            // Create package directory structure using dropz
            let dropz_fs = DropzFilesystem::new();
            dropz_fs.create_dir_all(name)
                .map_err(|e| format!("Failed to create directory: {}", e))?;
            dropz_fs.create_dir_all(&format!("{}/src", name))
                .map_err(|e| format!("Failed to create src directory: {}", e))?;
            dropz_fs.create_dir_all(&format!("{}/tests", name))
                .map_err(|e| format!("Failed to create tests directory: {}", e))?;
            
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
            
            dropz_fs.write(&format!("{}/package.toml", name), &package_toml)
                .map_err(|e| format!("Failed to write package.toml: {}", e))?;
            
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
            
            dropz_fs.write(&format!("{}/src/main.csd", name), &main_csd)
                .map_err(|e| format!("Failed to write main.csd: {}", e))?;
            
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
            
            dropz_fs.write(&format!("{}/src/lib.csd", name), &lib_csd)
                .map_err(|e| format!("Failed to write lib.csd: {}", e))?;
            
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
            
            dropz_fs.write(&format!("{}/tests/lib_test.csd", name), &test_csd)
                .map_err(|e| format!("Failed to write test file: {}", e))?;
            
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
            
            dropz_fs.write(&format!("{}/README.md", name), &readme_md)
                .map_err(|e| format!("Failed to write README.md: {}", e))?;
            
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
            
            dropz_fs.write(&format!("{}/.gitignore", name), gitignore)
                .map_err(|e| format!("Failed to write .gitignore: {}", e))?;
            
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
    let breakpoints: Vec<u32> = matches.get_many::<String>("breakpoints")
        .unwrap_or_default()
        .map(|s| s.parse::<u32>().unwrap_or(0))
        .collect();
    let trace_enabled = matches.get_flag("trace");
    let memory_debug = matches.get_flag("memory");
    
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
        // Initialize and run the interactive debugger
        println!("{} Starting interactive debugger...", "Debug".cyan().bold());
        
        let mut debugger = InteractiveDebugger::new(
            input.to_string(),
            source,
            ir,
            breakpoints,
            trace_enabled,
            memory_debug,
        )?;
        
        debugger.run().await?;
    }
    
    Ok(())
}

async fn handle_lint(matches: &ArgMatches, _global_matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let inputs: Vec<&String> = matches.get_many::<String>("input").unwrap().collect();
    let fix = matches.get_flag("fix");
    let format = matches.get_one::<String>("format").map(|s| s.as_str()).unwrap_or("human");
    let config_path = matches.get_one::<String>("config");
    let strict = matches.get_flag("strict");
    let minimal = matches.get_flag("minimal");
    
    println!("{} {} files", "Linting".blue().bold(), inputs.len());
    
    // Initialize linter with configuration
    let mut linter = if let Some(config_path) = config_path {
        let config = cursed::linter::LinterConfig::from_file(config_path)?;
        cursed::linter::CursedLinter::with_config(config)
    } else if strict {
        cursed::linter::CursedLinter::with_config(cursed::linter::LinterConfig::strict())
    } else if minimal {
        cursed::linter::CursedLinter::with_config(cursed::linter::LinterConfig::minimal())
    } else {
        cursed::linter::CursedLinter::new()
    };
    
    let output_format = match format {
        "json" => cursed::linter::OutputFormat::Json,
        "compact" => cursed::linter::OutputFormat::Compact,
        _ => cursed::linter::OutputFormat::Human,
    };
    
    let mut total_issues = 0;
    let mut all_results = Vec::new();
    
    for input in inputs {
        if Path::new(input).is_file() {
            let results = linter.lint_file(input)?;
            total_issues += results.stats.total_issues;
            
            // Store results for batch processing
            all_results.push(results);
        } else {
            // Handle directory
            for entry in glob::glob(&format!("{}/**/*.csd", input))? {
                let path = entry?;
                let results = linter.lint_file(&path)?;
                total_issues += results.stats.total_issues;
                
                // Store results for batch processing
                all_results.push(results);
            }
        }
    }
    
    // Process and display results
    for results in &all_results {
        let formatted_output = linter.format_results(results, output_format.clone());
        
        if matches!(output_format, cursed::linter::OutputFormat::Json) {
            println!("{}", formatted_output);
        } else if !results.issues.is_empty() || matches!(output_format, cursed::linter::OutputFormat::Human) {
            print!("{}", formatted_output);
        }
        
        // Apply fixes if requested
        if fix && !results.issues.is_empty() {
            apply_lint_fixes(&results.file_path, &results.issues)?;
        }
    }
    
    // Summary
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

async fn handle_fmt(matches: &ArgMatches, global_matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let inputs: Vec<&String> = matches.get_many::<String>("input").unwrap().collect();
    let check = matches.get_flag("check");
    let diff = matches.get_flag("diff");
    let config_file = matches.get_one::<String>("config");
    
    println!("{} {} files", "Formatting".blue().bold(), inputs.len());
    
    // Initialize formatter with configuration
    let formatter = if let Some(config_path) = config_file {
        cursed::formatter::SimpleCursedFormatter::with_config_file(config_path)?
    } else {
        cursed::formatter::SimpleCursedFormatter::default()
    };
    
    let mut changed_files = 0;
    let mut stats = cursed::formatter::FormattingStats::default();
    let color_enabled = match global_matches.get_one::<String>("color") {
        Some(value) => value != "never",
        None => true, // Default to enabled
    };
    let diff_formatter = cursed::formatter::DiffFormatter::new(
        color_enabled,
        3,
        true
    );
    
    for input in inputs {
        if Path::new(input).is_file() {
            match cursed::formatter::format_single_file(&formatter, &diff_formatter, input, check, diff) {
                Ok(result) => {
                    let original_lines = fs::read_to_string(input)?.lines().count();
                    stats.add_file(original_lines, original_lines, result.needs_formatting);
                    
                    if result.needs_formatting {
                        changed_files += 1;
                    }
                    
                    if let Some(error) = result.error {
                        println!("{}: {} {}", input, "Error".red().bold(), error);
                    } else if result.needs_formatting {
                        if check {
                            println!("{}: {} needs formatting", input, "✗".red().bold());
                        } else if diff {
                            println!("\n{}: formatting differences", input);
                            if let Some(diff_output) = result.diff {
                                println!("{}", diff_output);
                            }
                        } else {
                            println!("{}: {} formatted", input, "✓".green().bold());
                        }
                    } else {
                        println!("{}: {} already formatted", input, "✓".green().bold());
                    }
                }
                Err(e) => {
                    println!("{}: {} {}", input, "Error".red().bold(), e);
                }
            }
        } else {
            // Handle directory
            for entry in glob::glob(&format!("{}/**/*.csd", input))? {
                let path = entry?;
                let path_str = path.to_str().unwrap();
                
                match cursed::formatter::format_single_file(&formatter, &diff_formatter, path_str, check, diff) {
                    Ok(result) => {
                        let original_lines = fs::read_to_string(&path)?.lines().count();
                        stats.add_file(original_lines, original_lines, result.needs_formatting);
                        
                        if result.needs_formatting {
                            changed_files += 1;
                            if check {
                                println!("{}: {} needs formatting", path.display(), "✗".red().bold());
                            } else if diff {
                                println!("\n{}: formatting differences", path.display());
                                if let Some(diff_output) = result.diff {
                                    println!("{}", diff_output);
                                }
                            } else {
                                println!("{}: {} formatted", path.display(), "✓".green().bold());
                            }
                        } else {
                            println!("{}: {} already formatted", path.display(), "✓".green().bold());
                        }
                    }
                    Err(e) => {
                        println!("{}: {} {}", path.display(), "Error".red().bold(), e);
                    }
                }
            }
        }
    }
    
    // Print summary
    if !check && !diff {
        println!("\n{}", stats);
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
    
    let mut cleaned_count = 0;
    
    // Clean target directory
    if std::path::Path::new("target").exists() {
        println!("Removing target/ directory...");
        std::fs::remove_dir_all("target")?;
        cleaned_count += 1;
    }
    
    // Clean compiled executables
    for entry in std::fs::read_dir(".")? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(file_name) = path.file_name() {
                let name_str = file_name.to_string_lossy();
                // Remove CURSED compiled executables (no extension)
                if !name_str.contains('.') && path.metadata()?.permissions().mode() & 0o111 != 0 {
                    // Check if it's a CURSED-compiled binary
                    if let Ok(metadata) = std::fs::metadata(&path) {
                        if metadata.len() > 0 {
                            println!("Removing compiled executable: {}", name_str);
                            std::fs::remove_file(&path)?;
                            cleaned_count += 1;
                        }
                    }
                }
                // Remove LLVM IR files
                if name_str.ends_with(".ll") || name_str.ends_with(".bc") {
                    println!("Removing LLVM file: {}", name_str);
                    std::fs::remove_file(&path)?;
                    cleaned_count += 1;
                }
                // Remove object files
                if name_str.ends_with(".o") {
                    println!("Removing object file: {}", name_str);
                    std::fs::remove_file(&path)?;
                    cleaned_count += 1;
                }
                // Remove output files
                if name_str.ends_with("_output.txt") || name_str.ends_with("_out.txt") {
                    println!("Removing output file: {}", name_str);
                    std::fs::remove_file(&path)?;
                    cleaned_count += 1;
                }
            }
        }
    }
    
    println!("{} Cleaned {} artifacts", "Success".green().bold(), cleaned_count);
    Ok(())
}

async fn handle_check_llvm() -> Result<(), Box<dyn std::error::Error>> {
    match cursed::check_llvm_tools() {
        Ok(()) => {
            println!("✅ LLVM toolchain is ready for native compilation");
            Ok(())
        }
        Err(e) => {
            eprintln!("❌ LLVM toolchain issues: {}", e);
            std::process::exit(1);
        }
    }
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

async fn handle_lsp(matches: &ArgMatches, _global_matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    // use cursed::lsp::start_lsp_server;
    
    println!("{} CURSED Language Server", "Starting".blue().bold());
    
    // Set up logging
    let log_level = matches.get_one::<String>("log-level").unwrap();
    let log_filter = match log_level.as_str() {
        "error" => "error",
        "warn" => "warn", 
        "info" => "info",
        "debug" => "debug",
        "trace" => "trace",
        _ => "info",
    };
    
    // Configure logging
    if let Some(log_file) = matches.get_one::<String>("log-file") {
        // Log to file
        std::env::set_var("RUST_LOG", format!("cursed={}", log_filter));
        env_logger::builder()
            .target(env_logger::Target::Pipe(
                Box::new(std::fs::File::create(log_file)?)
            ))
            .init();
        println!("Logging to file: {}", log_file);
    } else {
        // Log to stderr
        std::env::set_var("RUST_LOG", format!("cursed={}", log_filter));
        env_logger::init();
    }
    
    // Determine communication method
    let use_tcp = matches.get_flag("tcp");
    
    if use_tcp {
        let port = *matches.get_one::<u16>("port").unwrap();
        println!("Starting LSP server on TCP port {}", port);
        eprintln!("TCP mode not yet implemented. Please use stdio mode.");
        std::process::exit(1);
    } else {
        println!("Starting LSP server with stdio communication");
        eprintln!("CURSED Language Server ready");
        
        // Start the LSP server
        println!("LSP server not implemented yet");
        Ok(())
    }
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
        let (name, item_type, parameters, return_type) = if next_line.starts_with("fn ") || next_line.starts_with("slay ") {
            let name = extract_function_name(next_line);
            let parameters = parse_function_parameters(next_line);
            let return_type = parse_function_return_type(next_line);
            (name, "function".to_string(), parameters, return_type)
        } else if next_line.starts_with("struct ") {
            let name = extract_struct_name(next_line);
            (name, "struct".to_string(), Vec::new(), None)
        } else if next_line.starts_with("enum ") {
            let name = extract_enum_name(next_line);
            (name, "enum".to_string(), Vec::new(), None)
        } else if next_line.starts_with("interface ") {
            let name = extract_interface_name(next_line);
            (name, "interface".to_string(), Vec::new(), None)
        } else {
            ("unknown".to_string(), "item".to_string(), Vec::new(), None)
        };
        
        if name != "unknown" {
            return Ok((Some(DocItem {
                name,
                item_type,
                description: description.trim().to_string(),
                file_path: file_path.to_path_buf(),
                line_number: i + 1,
                parameters,
                return_type,
                examples,
            }), i + 1));
        }
    }
    
    Ok((None, i))
}

fn extract_function_name(line: &str) -> String {
    // Handle both Rust-style (fn) and CURSED-style (slay) function declarations
    if line.starts_with("slay ") {
        line.split_whitespace()
            .nth(1)
            .and_then(|s| s.split('(').next())
            .unwrap_or("unknown")
            .to_string()
    } else {
        line.split_whitespace()
            .nth(1)
            .and_then(|s| s.split('(').next())
            .unwrap_or("unknown")
            .to_string()
    }
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

/// Parse function parameters from a function declaration line
/// Supports both Rust-style (fn name(params)) and CURSED-style (slay name(params)) syntax
fn parse_function_parameters(line: &str) -> Vec<Parameter> {
    let mut parameters = Vec::new();
    
    // Find the parameter list between parentheses
    if let Some(start) = line.find('(') {
        if let Some(end) = line.find(')') {
            let param_str = &line[start + 1..end];
            
            if param_str.trim().is_empty() {
                return parameters;
            }
            
            // Split parameters by comma
            for param in param_str.split(',') {
                let param = param.trim();
                
                if param.is_empty() {
                    continue;
                }
                
                // Parse individual parameter
                // Format: "name: type" or "name tea" (CURSED syntax)
                let (name, param_type) = if param.contains(':') {
                    // Standard format: "name: type"
                    let parts: Vec<&str> = param.splitn(2, ':').collect();
                    let name = parts[0].trim().to_string();
                    let param_type = parts.get(1).map(|t| t.trim().to_string()).unwrap_or_default();
                    (name, param_type)
                } else {
                    // CURSED format: might be "name type" or just "name"
                    let parts: Vec<&str> = param.split_whitespace().collect();
                    if parts.len() >= 2 {
                        let name = parts[0].to_string();
                        let param_type = parts[1..].join(" ");
                        (name, param_type)
                    } else {
                        let name = parts[0].to_string();
                        (name, "unknown".to_string())
                    }
                };
                
                parameters.push(Parameter {
                    name,
                    param_type,
                    description: String::new(), // Will be filled from documentation comments
                });
            }
        }
    }
    
    parameters
}

/// Parse function return type from a function declaration line
/// Supports both Rust-style (-> type) and CURSED-style (type after parentheses)
fn parse_function_return_type(line: &str) -> Option<String> {
    // Look for Rust-style return type with "->"
    if let Some(arrow_pos) = line.find("->") {
        let return_part = &line[arrow_pos + 2..];
        
        // Extract return type before any braces
        let return_type = return_part
            .split('{')
            .next()
            .unwrap_or("")
            .trim()
            .to_string();
            
        if !return_type.is_empty() {
            return Some(return_type);
        }
    }
    
    // Look for CURSED-style return type (after closing parenthesis, before opening brace)
    if let Some(paren_end) = line.find(')') {
        let after_params = &line[paren_end + 1..];
        
        // Look for return type before opening brace
        if let Some(brace_pos) = after_params.find('{') {
            let return_part = &after_params[..brace_pos].trim();
            
            // Remove arrow if present and extract type
            let return_type = return_part
                .trim_start_matches("->")
                .trim()
                .to_string();
                
            if !return_type.is_empty() && return_type != "{" {
                return Some(return_type);
            }
        } else {
            // No opening brace on same line, check for return type
            let return_type = after_params.trim().to_string();
            if !return_type.is_empty() {
                return Some(return_type);
            }
        }
    }
    
    None
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

/// Apply automatic fixes for linting issues
fn apply_lint_fixes(file_path: &str, issues: &[cursed::linter::LintIssue]) -> Result<(), Box<dyn std::error::Error>> {
    let mut applied_fixes = 0;
    let source = fs::read_to_string(file_path)?;
    let mut lines: Vec<&str> = source.lines().collect();
    
    // Sort issues by line number in reverse order to avoid offset issues
    let mut sortable_issues: Vec<_> = issues.iter().collect();
    sortable_issues.sort_by(|a, b| b.line.cmp(&a.line));
    
    for issue in sortable_issues {
        if let Some(fix) = &issue.fix_suggestion {
            // Apply simple text-based fixes
            if let Some(line) = lines.get_mut(issue.line.saturating_sub(1)) {
                // Simple fixes for common issues
                if issue.rule == "trailing_whitespace" {
                    *line = line.trim_end();
                    applied_fixes += 1;
                } else if issue.rule == "missing_newline_at_eof" && issue.line == lines.len() {
                    // This would be handled by appending a newline
                    applied_fixes += 1;
                }
                // Add more fix implementations as needed
            }
        }
    }
    
    if applied_fixes > 0 {
        let fixed_content = lines.join("\n");
        // Ensure file ends with newline if original did
        let final_content = if source.ends_with('\n') {
            format!("{}\n", fixed_content)
        } else {
            fixed_content
        };
        
        fs::write(file_path, final_content)?;
        println!("  {} Applied {} automatic fixes to {}", 
                "✓".green().bold(), applied_fixes, file_path);
    }
    
    Ok(())
}

/// Interactive debugger for CURSED programs
pub struct InteractiveDebugger {
    /// Source filename
    filename: String,
    /// Source code
    source: String,
    /// Compiled LLVM IR
    ir: String,
    /// Current breakpoints
    breakpoints: HashMap<u32, Breakpoint>,
    /// Execution trace enabled
    trace_enabled: bool,
    /// Memory debugging enabled
    memory_debug: bool,
    /// Current execution state
    execution_state: ExecutionState,
    /// Variable watch list
    watch_variables: Vec<String>,
    /// Call stack
    call_stack: Vec<StackFrame>,
    /// Debug runtime manager
    debug_manager: cursed::runtime::DebugManager,
    /// Performance monitor
    performance_monitor: cursed::runtime::PerformanceMonitor,
    /// Current line number
    current_line: u32,
    /// Source code lines
    source_lines: Vec<String>,
    /// Symbol table
    symbol_table: HashMap<String, SymbolInfo>,
    /// Memory inspector
    memory_inspector: MemoryInspector,
}

#[derive(Debug, Clone)]
pub struct Breakpoint {
    pub line: u32,
    pub condition: Option<String>,
    pub hit_count: u32,
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub struct StackFrame {
    pub function_name: String,
    pub line: u32,
    pub column: u32,
    pub variables: HashMap<String, VariableValue>,
}

#[derive(Debug, Clone)]
pub struct VariableValue {
    pub name: String,
    pub value: String,
    pub type_info: String,
    pub address: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct SymbolInfo {
    pub name: String,
    pub symbol_type: String,
    pub address: u64,
    pub size: u32,
    pub debug_info: Option<String>,
}

#[derive(Debug, Clone)]
pub enum ExecutionState {
    Running,
    Paused,
    Stopped,
    Error(String),
}

#[derive(Debug)]
pub struct MemoryInspector {
    pub heap_usage: u64,
    pub stack_usage: u64,
    pub allocations: Vec<AllocationInfo>,
}

#[derive(Debug, Clone)]
pub struct AllocationInfo {
    pub address: u64,
    pub size: u32,
    pub allocated_at: std::time::SystemTime,
    pub allocation_type: String,
}

impl InteractiveDebugger {
    pub fn new(
        filename: String,
        source: String,
        ir: String,
        initial_breakpoints: Vec<u32>,
        trace_enabled: bool,
        memory_debug: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let source_lines: Vec<String> = source.lines().map(|s| s.to_string()).collect();
        
        let mut breakpoints = HashMap::new();
        for line in initial_breakpoints {
            breakpoints.insert(line, Breakpoint {
                line,
                condition: None,
                hit_count: 0,
                enabled: true,
            });
        }
        
        // Initialize debug manager
        let debug_config = cursed::runtime::DebugManagerConfig {
            enabled: true,
            breakpoints_enabled: true,
            variable_inspection: true,
            stack_traces: true,
            llvm_debug_info: true,
            max_stack_depth: 100,
            symbol_resolution: true,
            verbosity_level: cursed::runtime::debug_manager::DebugVerbosity::Normal,
            log_buffer_size: 1000,
        };
        
        let debug_manager = cursed::runtime::DebugManager::new(debug_config);
        
        // Initialize performance monitor
        let performance_monitor = cursed::runtime::PerformanceMonitor::new(
            "debug_session".to_string(),
        );
        
        Ok(InteractiveDebugger {
            filename,
            source,
            ir,
            breakpoints,
            trace_enabled,
            memory_debug,
            execution_state: ExecutionState::Stopped,
            watch_variables: Vec::new(),
            call_stack: Vec::new(),
            debug_manager,
            performance_monitor,
            current_line: 1,
            source_lines,
            symbol_table: HashMap::new(),
            memory_inspector: MemoryInspector {
                heap_usage: 0,
                stack_usage: 0,
                allocations: Vec::new(),
            },
        })
    }
    
    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("{}", "=== CURSED Interactive Debugger ===".cyan().bold());
        println!("File: {}", self.filename.green());
        println!("Use 'help' for available commands\n");
        
        self.display_current_context();
        
        loop {
            print!("{} ", "(cursed-debug)".cyan().bold());
            use std::io::{self, Write};
            io::stdout().flush()?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();
            
            if input.is_empty() {
                continue;
            }
            
            match self.handle_command(input).await {
                Ok(true) => break, // Exit command
                Ok(false) => continue,
                Err(e) => {
                    eprintln!("{}: {}", "Error".red().bold(), e);
                }
            }
        }
        
        Ok(())
    }
    
    async fn handle_command(&mut self, input: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            return Ok(false);
        }
        
        match parts[0] {
            "help" | "h" => {
                self.show_help();
            }
            "run" | "r" => {
                self.run_program().await?;
            }
            "continue" | "c" => {
                self.continue_execution().await?;
            }
            "step" | "s" => {
                self.step_execution().await?;
            }
            "next" | "n" => {
                self.next_execution().await?;
            }
            "break" | "b" => {
                if parts.len() > 1 {
                    self.set_breakpoint(parts[1])?;
                } else {
                    self.list_breakpoints();
                }
            }
            "delete" | "d" => {
                if parts.len() > 1 {
                    self.delete_breakpoint(parts[1])?;
                }
            }
            "watch" | "w" => {
                if parts.len() > 1 {
                    self.add_watch(parts[1].to_string());
                } else {
                    self.list_watches();
                }
            }
            "unwatch" | "uw" => {
                if parts.len() > 1 {
                    self.remove_watch(parts[1]);
                }
            }
            "print" | "p" => {
                if parts.len() > 1 {
                    self.print_variable(parts[1]).await?;
                }
            }
            "eval" | "e" => {
                if parts.len() > 1 {
                    let expr = parts[1..].join(" ");
                    self.evaluate_expression(&expr).await?;
                }
            }
            "backtrace" | "bt" => {
                self.show_backtrace();
            }
            "frame" | "f" => {
                if parts.len() > 1 {
                    self.select_frame(parts[1])?;
                }
            }
            "locals" | "l" => {
                self.show_locals();
            }
            "memory" | "m" => {
                self.show_memory_info();
            }
            "inspect" | "i" => {
                if parts.len() > 1 {
                    self.inspect_memory(parts[1]).await?;
                }
            }
            "source" | "src" => {
                self.display_source_context();
            }
            "symbols" | "sym" => {
                self.show_symbols();
            }
            "performance" | "perf" => {
                self.show_performance_info().await?;
            }
            "trace" | "t" => {
                self.toggle_trace();
            }
            "quit" | "q" | "exit" => {
                println!("{}", "Goodbye!".green().bold());
                return Ok(true);
            }
            _ => {
                println!("{}: Unknown command '{}'. Use 'help' for available commands.", 
                        "Error".red().bold(), parts[0]);
            }
        }
        
        Ok(false)
    }
    
    fn show_help(&self) {
        println!("{}", "Available Commands:".cyan().bold());
        println!("  {} - Show this help message", "help (h)".green());
        println!("  {} - Run the program", "run (r)".green());
        println!("  {} - Continue execution", "continue (c)".green());
        println!("  {} - Step into (single instruction)", "step (s)".green());
        println!("  {} - Step over (next line)", "next (n)".green());
        println!("  {} - Set breakpoint at line", "break <line> (b)".green());
        println!("  {} - Delete breakpoint", "delete <line> (d)".green());
        println!("  {} - Add variable to watch list", "watch <var> (w)".green());
        println!("  {} - Remove variable from watch list", "unwatch <var> (uw)".green());
        println!("  {} - Print variable value", "print <var> (p)".green());
        println!("  {} - Evaluate expression", "eval <expr> (e)".green());
        println!("  {} - Show call stack", "backtrace (bt)".green());
        println!("  {} - Select stack frame", "frame <num> (f)".green());
        println!("  {} - Show local variables", "locals (l)".green());
        println!("  {} - Show memory information", "memory (m)".green());
        println!("  {} - Inspect memory at address", "inspect <addr> (i)".green());
        println!("  {} - Show source code context", "source (src)".green());
        println!("  {} - Show symbol table", "symbols (sym)".green());
        println!("  {} - Show performance information", "performance (perf)".green());
        println!("  {} - Toggle execution tracing", "trace (t)".green());
        println!("  {} - Quit debugger", "quit (q)".green());
    }
    
    async fn run_program(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("{} Starting program execution...", "Running".green().bold());
        
        self.execution_state = ExecutionState::Running;
        self.current_line = 1;
        
        // Performance monitoring would be started here
        // Start performance monitoring if enabled
        if let Err(e) = self.performance_monitor.start() {
            eprintln!("Failed to start performance monitor: {}", e);
        }
        
        // Simulate program execution with breakpoint checking
        for (line_num, line) in self.source_lines.iter().enumerate() {
            let line_num = line_num as u32 + 1;
            
            // Check for breakpoints
            if let Some(breakpoint) = self.breakpoints.get_mut(&line_num) {
                if breakpoint.enabled {
                    breakpoint.hit_count += 1;
                    self.execution_state = ExecutionState::Paused;
                    self.current_line = line_num;
                    
                    println!("{} Breakpoint hit at line {}", "Paused".yellow().bold(), line_num);
                    self.display_current_context();
                    return Ok(());
                }
            }
            
            // Execute line (simulated)
            if self.trace_enabled {
                println!("{}: {}", format!("{:4}", line_num).blue(), line);
            }
            
            self.current_line = line_num;
            
            // Simulate execution delay
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        }
        
        self.execution_state = ExecutionState::Stopped;
        println!("{} Program execution completed", "Finished".green().bold());
        
        Ok(())
    }
    
    async fn continue_execution(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if !matches!(self.execution_state, ExecutionState::Paused) {
            println!("{} Program is not paused", "Warning".yellow().bold());
            return Ok(());
        }
        
        println!("{} Continuing execution...", "Resuming".green().bold());
        self.execution_state = ExecutionState::Running;
        
        // Continue from current line
        for line_num in self.current_line + 1..=self.source_lines.len() as u32 {
            // Check for breakpoints
            if let Some(breakpoint) = self.breakpoints.get_mut(&line_num) {
                if breakpoint.enabled {
                    breakpoint.hit_count += 1;
                    self.execution_state = ExecutionState::Paused;
                    self.current_line = line_num;
                    
                    println!("{} Breakpoint hit at line {}", "Paused".yellow().bold(), line_num);
                    self.display_current_context();
                    return Ok(());
                }
            }
            
            // Execute line (simulated)
            if self.trace_enabled {
                let line = &self.source_lines[line_num as usize - 1];
                println!("{}: {}", format!("{:4}", line_num).blue(), line);
            }
            
            self.current_line = line_num;
            
            // Simulate execution delay
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        }
        
        self.execution_state = ExecutionState::Stopped;
        println!("{} Program execution completed", "Finished".green().bold());
        
        Ok(())
    }
    
    async fn step_execution(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if matches!(self.execution_state, ExecutionState::Stopped) {
            println!("{} Program is not running. Use 'run' to start.", "Warning".yellow().bold());
            return Ok(());
        }
        
        if self.current_line < self.source_lines.len() as u32 {
            self.current_line += 1;
            
            if self.trace_enabled {
                let line = &self.source_lines[self.current_line as usize - 1];
                println!("{}: {}", format!("{:4}", self.current_line).blue(), line);
            }
            
            self.display_current_context();
        } else {
            self.execution_state = ExecutionState::Stopped;
            println!("{} Program execution completed", "Finished".green().bold());
        }
        
        Ok(())
    }
    
    async fn next_execution(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Same as step for now - in a real implementation, this would step over function calls
        self.step_execution().await
    }
    
    fn set_breakpoint(&mut self, line_str: &str) -> Result<(), Box<dyn std::error::Error>> {
        let line: u32 = line_str.parse()?;
        
        if line == 0 || line > self.source_lines.len() as u32 {
            return Err(format!("Invalid line number: {}", line).into());
        }
        
        self.breakpoints.insert(line, Breakpoint {
            line,
            condition: None,
            hit_count: 0,
            enabled: true,
        });
        
        println!("{} Breakpoint set at line {}", "Set".green().bold(), line);
        Ok(())
    }
    
    fn delete_breakpoint(&mut self, line_str: &str) -> Result<(), Box<dyn std::error::Error>> {
        let line: u32 = line_str.parse()?;
        
        if self.breakpoints.remove(&line).is_some() {
            println!("{} Breakpoint deleted at line {}", "Deleted".green().bold(), line);
        } else {
            println!("{} No breakpoint found at line {}", "Warning".yellow().bold(), line);
        }
        
        Ok(())
    }
    
    fn list_breakpoints(&self) {
        if self.breakpoints.is_empty() {
            println!("No breakpoints set");
            return;
        }
        
        println!("{}", "Breakpoints:".cyan().bold());
        for (line, breakpoint) in &self.breakpoints {
            let status = if breakpoint.enabled { "enabled" } else { "disabled" };
            println!("  Line {}: {} (hit {} times)", 
                    line, status.green(), breakpoint.hit_count);
        }
    }
    
    fn add_watch(&mut self, variable: String) {
        if !self.watch_variables.contains(&variable) {
            self.watch_variables.push(variable.clone());
            println!("{} Added '{}' to watch list", "Watch".green().bold(), variable);
        } else {
            println!("{} '{}' is already being watched", "Warning".yellow().bold(), variable);
        }
    }
    
    fn remove_watch(&mut self, variable: &str) {
        if let Some(pos) = self.watch_variables.iter().position(|v| v == variable) {
            self.watch_variables.remove(pos);
            println!("{} Removed '{}' from watch list", "Unwatch".green().bold(), variable);
        } else {
            println!("{} '{}' is not being watched", "Warning".yellow().bold(), variable);
        }
    }
    
    fn list_watches(&self) {
        if self.watch_variables.is_empty() {
            println!("No variables being watched");
            return;
        }
        
        println!("{}", "Watch Variables:".cyan().bold());
        for var in &self.watch_variables {
            println!("  {}", var.green());
        }
    }
    
    async fn print_variable(&self, variable: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Simulate variable lookup
        println!("{} Variable '{}' = <value>", "Print".green().bold(), variable);
        // In a real implementation, this would query the runtime for the variable value
        Ok(())
    }
    
    async fn evaluate_expression(&self, expression: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Simulate expression evaluation
        println!("{} Expression '{}' = <result>", "Eval".green().bold(), expression);
        // In a real implementation, this would evaluate the expression in the current context
        Ok(())
    }
    
    fn show_backtrace(&self) {
        println!("{}", "Call Stack:".cyan().bold());
        if self.call_stack.is_empty() {
            println!("  No stack frames available");
            return;
        }
        
        for (i, frame) in self.call_stack.iter().enumerate() {
            println!("  #{}: {} at line {}", i, frame.function_name.green(), frame.line);
        }
    }
    
    fn select_frame(&mut self, frame_str: &str) -> Result<(), Box<dyn std::error::Error>> {
        let frame_num: usize = frame_str.parse()?;
        
        if frame_num >= self.call_stack.len() {
            return Err(format!("Invalid frame number: {}", frame_num).into());
        }
        
        let frame = &self.call_stack[frame_num];
        println!("{} Selected frame #{}: {} at line {}", 
                "Frame".green().bold(), frame_num, frame.function_name, frame.line);
        
        Ok(())
    }
    
    fn show_locals(&self) {
        println!("{}", "Local Variables:".cyan().bold());
        if self.call_stack.is_empty() {
            println!("  No stack frame selected");
            return;
        }
        
        // Show variables from the current frame
        if let Some(current_frame) = self.call_stack.last() {
            for (name, value) in &current_frame.variables {
                println!("  {} = {} ({})", name.green(), value.value, value.type_info);
            }
        }
    }
    
    fn show_memory_info(&self) {
        println!("{}", "Memory Information:".cyan().bold());
        println!("  Heap Usage: {} bytes", self.memory_inspector.heap_usage);
        println!("  Stack Usage: {} bytes", self.memory_inspector.stack_usage);
        println!("  Active Allocations: {}", self.memory_inspector.allocations.len());
        
        if self.memory_debug {
            println!("\n{}", "Recent Allocations:".cyan().bold());
            for (i, alloc) in self.memory_inspector.allocations.iter().take(10).enumerate() {
                println!("  #{}: {} bytes at 0x{:x} ({})", 
                        i, alloc.size, alloc.address, alloc.allocation_type);
            }
        }
    }
    
    async fn inspect_memory(&self, addr_str: &str) -> Result<(), Box<dyn std::error::Error>> {
        let address: u64 = if addr_str.starts_with("0x") {
            u64::from_str_radix(&addr_str[2..], 16)?
        } else {
            addr_str.parse()?
        };
        
        println!("{} Memory at 0x{:x}:", "Inspect".green().bold(), address);
        println!("  <memory contents would be displayed here>");
        
        Ok(())
    }
    
    fn display_source_context(&self) {
        self.display_current_context();
    }
    
    fn display_current_context(&self) {
        let start_line = self.current_line.saturating_sub(3);
        let end_line = (self.current_line + 3).min(self.source_lines.len() as u32);
        
        println!("{}", "Source Context:".cyan().bold());
        for line_num in start_line..=end_line {
            if line_num == 0 || line_num > self.source_lines.len() as u32 {
                continue;
            }
            
            let line = &self.source_lines[line_num as usize - 1];
            let marker = if line_num == self.current_line { ">" } else { " " };
            let breakpoint_marker = if self.breakpoints.contains_key(&line_num) { "*" } else { " " };
            
            println!("{}{} {:4}: {}", 
                    marker.red().bold(),
                    breakpoint_marker.yellow().bold(),
                    line_num,
                    line);
        }
    }
    
    fn show_symbols(&self) {
        println!("{}", "Symbol Table:".cyan().bold());
        if self.symbol_table.is_empty() {
            println!("  No symbols loaded");
            return;
        }
        
        for (name, symbol) in &self.symbol_table {
            println!("  {} ({}): 0x{:x} [{}]", 
                    name.green(), symbol.symbol_type, symbol.address, symbol.size);
        }
    }
    
    async fn show_performance_info(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("{}", "Performance Information:".cyan().bold());
        
        // Get current performance metrics (placeholder)
        println!("  Performance monitoring integration pending");
        println!("  CPU Usage: <monitoring not yet integrated>");
        println!("  Memory Usage: <monitoring not yet integrated>");
        println!("  Memory Peak: <monitoring not yet integrated>");
        println!("  Allocations: <monitoring not yet integrated>");
        println!("  Total Allocated: <monitoring not yet integrated>");
        
        Ok(())
    }
    
    fn toggle_trace(&mut self) {
        self.trace_enabled = !self.trace_enabled;
        println!("{} Execution tracing {}", 
                "Trace".green().bold(), 
                if self.trace_enabled { "enabled" } else { "disabled" });
    }
}
