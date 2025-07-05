/// CURSED Programming Language - Modern CLI Interface
/// 
/// Unified command-line interface for the CURSED programming language
/// with comprehensive subcommands and advanced features.

use std::env;
use std::fs;
use std::process;
use std::path::Path;
use clap::{Arg, Command, ArgMatches, value_parser};
use colored::*;
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
                    .required(true)
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
                .about("Run tests")
                .arg(Arg::new("pattern")
                    .help("Test file pattern")
                    .index(1)
                    .default_value("**/*.test.csd"))
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

async fn handle_compile(matches: &ArgMatches, global_matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
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
        // TODO: Implement assembly generation
        println!("{} Assembly generation not yet implemented", "Warning".yellow().bold());
    } else {
        cursed::compile(input, output)?;
        println!("{} executable: {}", "Generated".green().bold(), output);
    }
    
    Ok(())
}

async fn handle_run(matches: &ArgMatches, global_matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let input = matches.get_one::<String>("input").unwrap();
    
    if matches.get_flag("verbose") || global_matches.get_flag("verbose") {
        println!("{} {}", "Running".green().bold(), input);
    }
    
    if matches.get_flag("jit") {
        // TODO: Implement JIT execution
        println!("{} JIT execution not yet implemented, using interpreter", "Warning".yellow().bold());
    }
    
    if matches.get_flag("interpreter") {
        cursed::run_file(input)?;
    } else {
        // Default: run file directly for simplicity
        cursed::run_file(input)?;
    }
    
    Ok(())
}

async fn handle_test(matches: &ArgMatches, global_matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let pattern = matches.get_one::<String>("pattern").unwrap();
    let filter = matches.get_one::<String>("filter").map(|s| s.as_str());
    let parallel = matches.get_flag("parallel");
    let timeout = matches.get_one::<u64>("timeout").copied().unwrap_or(30);
    let coverage = matches.get_flag("coverage");
    
    if matches.get_flag("verbose") || global_matches.get_flag("verbose") {
        println!("{} tests with pattern: {}", "Running".green().bold(), pattern);
    }
    
    // TODO: Implement comprehensive test runner
    // For now, use a simple approach
    println!("{} Test runner implementation in progress", "Info".blue().bold());
    
    // Find test files
    let test_files = find_test_files(pattern)?;
    
    if test_files.is_empty() {
        println!("{} No test files found matching pattern: {}", "Warning".yellow().bold(), pattern);
        return Ok(());
    }
    
    println!("Found {} test files", test_files.len());
    
    let mut passed = 0;
    let mut failed = 0;
    
    for test_file in test_files {
        if let Some(f) = filter {
            if !test_file.contains(f) {
                continue;
            }
        }
        
        println!("Running test: {}", test_file);
        
        // Run the test file
        match cursed::run_file(&test_file) {
            Ok(_) => {
                println!("  {} PASSED", "✓".green().bold());
                passed += 1;
            }
            Err(e) => {
                println!("  {} FAILED: {}", "✗".red().bold(), e);
                failed += 1;
            }
        }
    }
    
    println!();
    println!("Test results: {} passed, {} failed", 
             passed.to_string().green().bold(), 
             failed.to_string().red().bold());
    
    if failed > 0 {
        process::exit(1);
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
            
            // TODO: Implement init functionality
            println!("{} Package initialization not yet implemented", "Warning".yellow().bold());
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
    
    // TODO: Implement documentation generation
    println!("{} Documentation generation not yet implemented", "Warning".yellow().bold());
    
    if serve {
        println!("{} documentation server on port {}", "Starting".blue().bold(), port);
        // TODO: Implement documentation server
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
        // TODO: Implement startup file loading
        println!("{} Startup file loading not yet implemented", "Warning".yellow().bold());
    }
    
    // TODO: Implement proper REPL loop
    println!("{} REPL loop not yet implemented", "Warning".yellow().bold());
    
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
        return Err(format!("Parse errors: {}", errors.join(", ")).into());
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
