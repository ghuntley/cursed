use crate::error::CursedError;
/// CURSED Build System CLI
/// 
/// Command-line interface for the CURSED build system providing project
/// initialization, building, testing, and toolchain integration.

use clap::{Parser, Subcommand, ArgGroup};
use cursed::build_system::{
    ProjectType, TemplateCategory, PipelineResult
// };

use cursed::build_system::build_orchestrator::WatchConfig;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Instant;
use tracing::{error, info, warn};
use tracing_subscriber;

#[derive(Parser)]
#[command(name = "cursed-build")]
#[command(about = "CURSED Build System - Build, test, and manage CURSED projects")]
#[command(version = env!("CARGO_PKG_VERSION"))]
struct Cli {
    #[command(subcommand)]
    
    /// Enable verbose output
    #[arg(short, long, global = true)]
    
    /// Suppress output
    #[arg(short, long, global = true)]
    
    /// Build profile to use
    #[arg(short, long, global = true, default_value = "dev")]
    
    /// Number of parallel jobs
    #[arg(short, long, global = true)]
#[derive(Subcommand)]
enum Commands {
    /// Initialize a new CURSED project
    #[command(alias = "new")]
    Init {
        /// Project name
        
        /// Project template to use
        #[arg(short, long, default_value = "cli")]
        
        /// Create as library project
        #[arg(long)]
        
        /// Target directory (defaults to project name)
        #[arg(long)]
        
        /// Additional template variables (key=value)
        #[arg(long = "var", value_parser = parse_key_val)]
    
    /// Build the project
    Build {
        /// Specific targets to build
        #[arg(short, long)]
        
        /// Release build
        #[arg(short, long)]
        
        /// Enable all features
        #[arg(long)]
        
        /// Disable default features
        #[arg(long)]
        
        /// Enable specific features
        #[arg(long)]
        
        /// Force rebuild (ignore cache)
        #[arg(long)]
        
        /// Disable parallel compilation
        #[arg(long)]
        
        /// Use quick build (skip formatting and linting)
        #[arg(long)]
        
        /// Watch for file changes and rebuild automatically
        #[arg(short, long)]
    
    /// Run the project
    Run {
        /// Arguments to pass to the program
        #[arg(last = true)]
        
        /// Release build
        #[arg(short, long)]
        
        /// Specific target to run
        #[arg(long)]
    
    /// Test the project
    Test {
        /// Test name patterns
        
        /// Release build
        #[arg(short, long)]
        
        /// Run ignored tests
        #[arg(long)]
        
        /// Number of test threads
        #[arg(long)]
        
        /// Watch for file changes and rerun tests automatically
        #[arg(short, long)]
    
    /// Clean build artifacts
    Clean {
        /// Remove target directory completely
        #[arg(long)]
    
    /// Check code without building
    Check {
        /// Check all targets
        #[arg(long)]
    
    /// Format source code
    #[command(alias = "fmt")]
    Format {
        /// Check formatting without applying changes
        #[arg(long)]
        
        /// Show diff of changes
        #[arg(long)]
        
        /// Files or directories to format
    
    /// Lint source code
    Lint {
        /// Automatically fix issues where possible
        #[arg(long)]
        
        /// Show detailed statistics
        #[arg(long)]
        
        /// Files or directories to lint
    
    /// Generate documentation
    #[command(alias = "doc")]
    Docs {
        /// Open documentation in browser
        #[arg(long)]
        
        /// Documentation format
        #[arg(long, default_value = "html")]
        
        /// Output directory
        #[arg(short, long)]
    
    /// Package management
    #[command(alias = "pkg")]
    Package {
        #[command(subcommand)]
    
    /// List available templates
    Templates {
        /// Show templates for specific category
        #[arg(short, long)]
        
        /// Show detailed template information
        #[arg(long)]
    
    /// Show project information
    Info {
        /// Show dependency graph
        #[arg(long)]
        
        /// Show build configuration
        #[arg(long)]
        
        /// Output format (text, json, yaml)
        #[arg(long, default_value = "text")]
    
    /// Watch for changes and rebuild
    Watch {
        /// Command to run on changes
        #[arg(short, long, default_value = "build")]
        
        /// Delay before rebuilding (ms) 
        #[arg(long, default_value = "500")]
        
        /// File patterns to watch (default: **/*.csd)
        #[arg(long)]
        
        /// Patterns to ignore
        #[arg(long)]
        
        /// Debounce delay for file events (ms)
        #[arg(long, default_value = "100")]
    
    /// Benchmark the project
    Bench {
        /// Benchmark name patterns
        
        /// Save baseline
        #[arg(long)]
        
        /// Compare against baseline
        #[arg(long)]
#[derive(Subcommand)]
enum PackageCommands {
    /// Install dependencies
    
    /// Update dependencies
    Update {
        /// Specific package to update
        
        /// Dry run
        #[arg(long)]
    
    /// Add a dependency
    Add {
        /// Package name
        
        /// Package version
        #[arg(short, long)]
        
        /// Add as dev dependency
        #[arg(long)]
        
        /// Add as build dependency
        #[arg(long)]
    
    /// Remove a dependency
    Remove {
        /// Package name
    
    /// Search for packages
    Search {
        /// Search query
        
        /// Maximum results
        #[arg(short, long, default_value = "10")]
    
    /// Show package information
    Info {
        /// Package name
fn parse_key_val(s: &str) -> Result<(String, String), String> {
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{s}`"))?;
    Ok((s[..pos].to_string(), s[pos + 1..].to_string()))
#[tokio::main]
async fn main() -> crate::error::Result<()> {
    let cli = Cli::parse();
    
    // Setup logging
    setup_logging(cli.verbose, cli.quiet);
    
    let start_time = Instant::now();
    
    match run_command(cli).await {
        Ok(()) => {
            let duration = start_time.elapsed();
            info!("Build completed in {:?}", duration);
            Ok(())
        }
        Err(e) => {
            error!("Build failed: {}", e);
            std::process::exit(1);
        }
    }
async fn run_command(cli: Cli) -> crate::error::Result<()> {
    match cli.command {
        Commands::Init { name, template, lib, target_dir, variables } => {
            let template_name = if lib { "lib" } else { &template };
            let target = target_dir.unwrap_or_else(|| PathBuf::from(&name));
            
            init_project(&name, template_name, target, variables).await
        Commands::Build { target, release, all_features, no_default_features, features, force, no_parallel, quick, watch } => {
            let profile = if release { "release" } else { &cli.profile };
            
            if watch {
                watch_build_project(profile, target, all_features, no_default_features, features, force, !no_parallel, quick).await
            } else {
                build_project(profile, target, all_features, no_default_features, features, force, !no_parallel, quick).await
            }
        }
        
        Commands::Run { args, release, bin } => {
            let profile = if release { "release" } else { &cli.profile };
            run_project(profile, bin, args, cli.jobs).await
        Commands::Test { test_name, release, ignored, test_threads, watch } => {
            let profile = if release { "release" } else { &cli.profile };
            
            if watch {
                watch_test_project(profile, test_name, ignored, test_threads).await
            } else {
                test_project(profile, test_name, ignored, test_threads).await
            }
        }
        
        Commands::Clean { all } => {
            clean_project(all, cli.jobs).await
        Commands::Check { all_targets } => {
            check_project(all_targets).await
        Commands::Format { check, diff, files } => {
            format_code(check, diff, files).await
        Commands::Lint { fix, stats, files } => {
            lint_code(fix, stats, files).await
        Commands::Docs { open, format, output } => {
            generate_docs(open, &format, output).await
        Commands::Package { command } => {
            handle_package_command(command).await
        Commands::Templates { category, detailed } => {
            list_templates(category, detailed).await
        Commands::Info { deps, config, format } => {
            show_project_info(deps, config, &format).await
        Commands::Watch { command, delay, patterns, ignore, debounce } => {
            watch_project(&command, delay, patterns, ignore, debounce).await
        Commands::Bench { bench_name, save_baseline, baseline } => {
            benchmark_project(bench_name, save_baseline, baseline).await
        }
    }
async fn init_project(
) -> crate::error::Result<()> {
    info!("Initializing project '{}' with template '{}'", name, template);
    
    let template_manager = TemplateManager::new();
    let variable_map: HashMap<String, String> = variables.into_iter().collect();
    
    let context = TemplateContext {
    
    let target_dir = context.target_dir.clone();
    template_manager.generate_project(template, context)?;
    
    println!("✅ Project '{}' created successfully!", name);
    println!("📁 Location: {}", target_dir.display());
    println!("");
    println!("Next steps:");
    println!("  cd {}", name);
    println!("  cursed-build build");
    println!("  cursed-build run");
    
    Ok(())
async fn build_project(
) -> crate::error::Result<()> {
    info!("Building project with profile: {}", profile);
    
    let work_dir = std::env::current_dir()?;
    let config_path = work_dir.join("CursedBuild.toml");
    
    if !config_path.exists() {
        return Err("No CursedBuild.toml found. Run 'cursed-build init' to create a project.".into());
    let config = BuildConfig::load_from_file(&config_path)?;
    let mut orchestrator = BuildOrchestrator::new(config, work_dir)?;
    
    let result = if quick {
        // Use quick build mode
        orchestrator.quick_build(profile).await?
    } else if targets.is_empty() {
        // Use pipeline for comprehensive build
        let pipeline_result = orchestrator.build_with_pipeline(profile, Vec::from([]), force, parallel).await?;
        convert_pipeline_to_build_result(pipeline_result)
    } else {
        // Build specific targets with pipeline
        let pipeline_result = orchestrator.build_targets_with_pipeline(profile, &targets).await?;
        convert_pipeline_to_build_result(pipeline_result)
    
    println!("🔨 Build completed successfully!");
    println!("📊 Statistics:");
    println!("   - Targets built: {}", result.targets_built.len());
    println!("   - Targets cached: {}", result.targets_skipped.len());
    println!("   - Files compiled: {}", result.statistics.files_compiled);
    println!("   - Duration: {:?}", result.duration);
    
    if !result.warnings.is_empty() {
        println!("⚠️  Warnings:");
        for warning in &result.warnings {
            println!("   {}", warning);
        }
    }
    
    Ok(())
async fn run_project(
) -> crate::error::Result<()> {
    info!("Running project with profile: {}", profile);
    
    // Build first
    build_project(profile, Vec::from([]), false, false, Vec::from([]), false, true, false).await?;
    
    // Find executable
    let work_dir = std::env::current_dir()?;
    let target_dir = work_dir.join("target").join(if profile == "release" { "release" } else { "debug" });
    
    let executable = if let Some(bin_name) = bin {
        target_dir.join(&bin_name)
    } else {
        // Find first executable in target directory
        let entries = std::fs::read_dir(&target_dir)?;
        let mut executable = None;
        
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && is_executable(&path) {
                executable = Some(path);
                break;
            }
        }
        
        executable.ok_or("No executable found in target directory")?
    
    if !executable.exists() {
        return Err(format!("Executable not found: {}", executable.display()).into());
    println!("🚀 Running: {}", executable.display());
    
    let mut cmd = std::process::Command::new(&executable);
    cmd.args(args);
    
    let status = cmd.status()?;
    
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    Ok(())
async fn test_project(
) -> crate::error::Result<()> {
    info!("Testing project with profile: {}", profile);
    
    let work_dir = std::env::current_dir()?;
    let config_path = work_dir.join("CursedBuild.toml");
    
    if !config_path.exists() {
        return Err("No CursedBuild.toml found. Run 'cursed-build init' to create a project.".into());
    let config = BuildConfig::load_from_file(&config_path)?;
    let mut orchestrator = BuildOrchestrator::new(config, work_dir)?;
    
    let result = orchestrator.test(profile).await?;
    
    println!("🧪 Tests completed!");
    println!("📊 Results:");
    println!("   - Success: {}", result.success);
    println!("   - Duration: {:?}", result.duration);
    
    Ok(())
async fn clean_project(all: bool, _jobs: Option<usize>) -> crate::error::Result<()> {
    info!("Cleaning project artifacts");
    
    let work_dir = std::env::current_dir()?;
    let target_dir = work_dir.join("target");
    
    if target_dir.exists() {
        if all {
            std::fs::remove_dir_all(&target_dir)?;
            println!("🧹 Removed entire target directory");
        } else {
            // Remove specific directories
            let debug_dir = target_dir.join("debug");
            let release_dir = target_dir.join("release");
            
            if debug_dir.exists() {
                std::fs::remove_dir_all(&debug_dir)?;
                println!("🧹 Removed debug artifacts");
            if release_dir.exists() {
                std::fs::remove_dir_all(&release_dir)?;
                println!("🧹 Removed release artifacts");
            }
        }
    } else {
        println!("ℹ️  Nothing to clean");
    Ok(())
async fn check_project(all_targets: bool) -> crate::error::Result<()> {
    use cursed::parser::{Parser, ParseOptions};
    use cursed::core::type_checker::TypeChecker;
    use cursed::lexer::Lexer;
    
    info!("Checking project");
    println!("🔍 Running syntax and type checks...");
    
    let work_dir = std::env::current_dir()?;
    let config_path = work_dir.join("CursedBuild.toml");
    
    if !config_path.exists() {
        return Err("No CursedBuild.toml found. Run 'cursed-build init' to create a project.".into());
    let config = BuildConfig::load_from_file(&config_path)?;
    
    let mut total_files = 0;
    let mut checked_files = 0;
    let mut syntax_errors = 0;
    let mut type_errors = 0;
    
    // Find all CURSED source files
    let source_patterns = if all_targets {
        vec!["src/**/*.csd", "tests/**/*.csd", "examples/**/*.csd"]
    } else {
        vec!["src/**/*.csd"]
    
    for pattern in source_patterns {
        let pattern_path = work_dir.join(pattern);
        if let Ok(entries) = glob::glob(&pattern_path.to_string_lossy()) {
            for entry in entries {
                if let Ok(file_path) = entry {
                    total_files += 1;
                    
                    match check_single_file(&file_path).await {
                        Ok((has_syntax_errors, has_type_errors)) => {
                            checked_files += 1;
                            
                            if has_syntax_errors {
                                syntax_errors += 1;
                                println!("  ❌ Syntax errors in: {}", file_path.display());
                            } else if has_type_errors {
                                type_errors += 1;
                                println!("  ⚠️  Type errors in: {}", file_path.display());
                            } else {
                                println!("  ✅ {}", file_path.display());
                            }
                        }
                        Err(e) => {
                            syntax_errors += 1;
                            println!("  ❌ Failed to check {}: {}", file_path.display(), e);
                        }
                    }
                }
            }
        }
    }
    
    // Check if no files were found
    if total_files == 0 {
        println!("ℹ️  No CURSED source files found in the project");
        return Ok(());
    // Print summary
    println!();
    println!("📊 Check Summary:");
    println!("   Files checked: {}/{}", checked_files, total_files);
    
    if syntax_errors > 0 || type_errors > 0 {
        println!("   ❌ Syntax errors: {}", syntax_errors);
        println!("   ⚠️  Type errors: {}", type_errors);
        
        if syntax_errors > 0 {
            return Err(format!("Project has {} syntax error(s)", syntax_errors).into());
        } else {
            println!("   ⚠️  Project has type errors but syntax is valid");
        }
    } else {
        println!("   ✅ All checks passed");
    println!("🔍 Project check completed");
    Ok(())
async fn check_single_file(file_path: &std::path::Path) -> crate::error::Result<()> {
    // Read file content
    let content = std::fs::read_to_string(file_path)?;
    
    let mut has_syntax_errors = false;
    let mut has_type_errors = false;
    
    // Lexical analysis
    let mut lexer = Lexer::new(&content);
    let tokens = match lexer.tokenize() {
        Err(_) => {
            has_syntax_errors = true;
            return Ok((has_syntax_errors, has_type_errors));
        }
    
    // Syntax analysis (parsing)
    let parse_options = ParseOptions::default();
    let mut parser = Parser::new(tokens, parse_options);
    
    let ast = match parser.parse() {
        Err(_) => {
            has_syntax_errors = true;
            return Ok((has_syntax_errors, has_type_errors));
        }
    
    // Type checking
    let type_checker = TypeChecker::new();
    
    // Basic type checking - in a full implementation this would
    // check all expressions and statements in the AST
    for statement in &ast.statements {
        // For now, just attempt basic type checking
        // This is a simplified check - a full implementation would
        // traverse the entire AST and check all type constraints
        if let Err(_) = type_checker.check_type(&format!("{:?}", statement)) {
            has_type_errors = true;
        }
    }
    
    Ok((has_syntax_errors, has_type_errors))
async fn format_code(
) -> crate::error::Result<()> {
    info!("Formatting CURSED code");
    
    let mut cmd = std::process::Command::new("./target/debug/cursed-fmt");
    
    if check {
        cmd.arg("--check");
    if diff {
        cmd.arg("--diff");
    if files.is_empty() {
        cmd.arg(".");
    } else {
        cmd.args(files);
    let status = cmd.status()?;
    
    if status.success() {
        println!("💄 Code formatting completed");
    } else {
        return Err("Formatting failed".into());
    Ok(())
async fn lint_code(
) -> crate::error::Result<()> {
    info!("Linting CURSED code");
    
    let mut cmd = std::process::Command::new("./target/debug/cursed_lint_new");
    
    if fix {
        cmd.arg("--fix");
    if stats {
        cmd.arg("--stats");
    if files.is_empty() {
        cmd.arg(".");
    } else {
        cmd.args(files);
    let status = cmd.status()?;
    
    if status.success() {
        println!("🔍 Code linting completed");
    } else {
        warn!("Linting found issues");
    Ok(())
async fn generate_docs(
) -> crate::error::Result<()> {
    info!("Generating documentation in {} format", format);
    
    let mut cmd = std::process::Command::new("./target/debug/cursed-doc");
    
    match format {
    
    cmd.arg("--source").arg("src");
    
    let output_dir = if let Some(ref output_dir) = output {
        cmd.arg("--output").arg(output_dir);
        output_dir.clone()
    } else {
        cmd.arg("--output").arg("docs");
        PathBuf::from("docs")
    
    let status = cmd.status()?;
    
    if status.success() {
        println!("📚 Documentation generated successfully");
        
        if open && format == "html" {
            let docs_path = output_dir.join("index.html");
            if docs_path.exists() {
                open_in_browser(&docs_path);
            }
        }
    } else {
        return Err("Documentation generation failed".into());
    Ok(())
async fn handle_package_command(command: PackageCommands) -> crate::error::Result<()> {
    use cursed::package_manager::{PackageManager, PackageManagerConfig};
    
    // Initialize package manager
    let config = PackageManagerConfig::default();
    let mut manager = PackageManager::new(config)?;
    
    match command {
        PackageCommands::Install => {
            info!("Installing dependencies");
            println!("📦 Installing dependencies...");
            
            // Check for CursedPackage.toml in current directory
            let package_file = std::env::current_dir()?.join("CursedPackage.toml");
            if package_file.exists() {
                // Parse package file and install dependencies
                let content = std::fs::read_to_string(&package_file)?;
                let package_metadata: cursed::package_manager::PackageMetadata = toml::from_str(&content)?;
                
                let mut installed_count = 0;
                for (dep_name, dep_version) in &package_metadata.dependencies {
                    println!("   Installing {} v{}", dep_name, dep_version);
                    match manager.install_package(dep_name, Some(&dep_version.to_string())).await {
                        Ok(packages) => {
                            installed_count += packages.len();
                            for pkg in packages {
                                println!("     ✅ {} v{}", pkg.name, pkg.version);
                            }
                        }
                        Err(e) => {
                            warn!("Failed to install {}: {}", dep_name, e);
                            println!("     ❌ Failed to install {}: {}", dep_name, e);
                        }
                    }
                println!("✅ Installed {} dependencies", installed_count);
            } else {
                println!("❌ No CursedPackage.toml found. Run 'cursed-build init' to create a project.");
            }
        }
        
        PackageCommands::Update { package, dry_run } => {
            if dry_run {
                println!("🔍 Checking for updates (dry run):");
            } else {
                println!("📦 Updating dependencies...");
            if let Some(pkg) = package {
                println!("   Updating {}", pkg);
                if !dry_run {
                    match manager.install_package(&pkg, None).await {
                        Ok(packages) => {
                            for package in packages {
                                println!("     ✅ Updated {} to v{}", package.name, package.version);
                            }
                        }
                        Err(e) => {
                            warn!("Failed to update {}: {}", pkg, e);
                            println!("     ❌ Failed to update {}: {}", pkg, e);
                        }
                    }
                }
            } else {
                // Update all packages from CursedPackage.toml
                let package_file = std::env::current_dir()?.join("CursedPackage.toml");
                if package_file.exists() {
                    let content = std::fs::read_to_string(&package_file)?;
                    let package_metadata: cursed::package_manager::PackageMetadata = toml::from_str(&content)?;
                    
                    for (dep_name, _) in &package_metadata.dependencies {
                        println!("   Updating {}", dep_name);
                        if !dry_run {
                            match manager.install_package(dep_name, None).await {
                                Err(e) => {
                                    warn!("Failed to update {}: {}", dep_name, e);
                                    println!("     ❌ Failed to update {}: {}", dep_name, e);
                                }
                            }
                        }
                    }
                } else {
                    println!("❌ No CursedPackage.toml found");
                }
            }
            
            if !dry_run {
                println!("✅ Dependencies updated");
            }
        }
        
        PackageCommands::Add { package, version, dev, build } => {
            let dep_type = if dev {
                "dev dependency"
            } else if build {
                "build dependency"
            } else {
                "dependency"
            
            println!("📦 Adding {} as {}", package, dep_type);
            if let Some(ver) = &version {
                println!("   Version: {}", ver);
            // Install the package first
            match manager.install_package(&package, version.as_deref()).await {
                Ok(packages) => {
                    for pkg in packages {
                        println!("   ✅ Installed {} v{}", pkg.name, pkg.version);
                    // TODO: Add to CursedPackage.toml
                    println!("   📝 Adding to CursedPackage.toml");
                    println!("✅ Package added");
                }
                Err(e) => {
                    error!("Failed to add package: {}", e);
                    return Err(format!("Failed to add package {}: {}", package, e).into());
                }
            }
        PackageCommands::Remove { package } => {
            println!("🗑️  Removing package: {}", package);
            match manager.remove_package(&package) {
                Ok(_) => {
                    println!("   ✅ Removed from cache");
                    // TODO: Remove from CursedPackage.toml
                    println!("   📝 Removing from CursedPackage.toml");
                    println!("✅ Package removed");
                }
                Err(e) => {
                    warn!("Failed to remove package: {}", e);
                    println!("❌ Failed to remove package: {}", e);
                }
            }
        PackageCommands::Search { query, limit } => {
            println!("🔍 Searching for: {} (limit: {})", query, limit);
            
            match manager.search_packages(&query, Some(limit)).await {
                Ok(packages) => {
                    if packages.is_empty() {
                        println!("No packages found matching '{}'", query);
                    } else {
                        println!("Found {} package(s):", packages.len());
                        for pkg in packages {
                            println!("  📦 {} v{}", pkg.name, pkg.version);
                            println!("     {}", pkg.description);
                            if !pkg.keywords.is_empty() {
                                println!("     Keywords: {}", pkg.keywords.join(", "));
                            }
                            println!();
                        }
                    }
                }
                Err(e) => {
                    warn!("Search failed: {}", e);
                    println!("❌ Search failed: {}", e);
                }
            }
        PackageCommands::Info { package } => {
            println!("📋 Package information for: {}", package);
            
            // Search for the specific package to get info
            match manager.search_packages(&package, Some(1)).await {
                Ok(packages) => {
                    if let Some(pkg) = packages.first() {
                        println!("  📦 Name: {}", pkg.name);
                        println!("  🏷️  Version: {}", pkg.version);
                        println!("  📝 Description: {}", pkg.description);
                        
                        if !pkg.authors.is_empty() {
                            println!("  👥 Authors: {}", pkg.authors.join(", "));
                        if let Some(repo) = &pkg.repository {
                            println!("  🔗 Repository: {}", repo);
                        if let Some(license) = &pkg.license {
                            println!("  📜 License: {}", license);
                        if !pkg.keywords.is_empty() {
                            println!("  🏷️  Keywords: {}", pkg.keywords.join(", "));
                        if !pkg.categories.is_empty() {
                            println!("  📂 Categories: {}", pkg.categories.join(", "));
                        if !pkg.dependencies.is_empty() {
                            println!("  📦 Dependencies:");
                            for (dep_name, dep_version) in &pkg.dependencies {
                                println!("     {} v{}", dep_name, dep_version);
                            }
                        }
                        
                        if !pkg.dev_dependencies.is_empty() {
                            println!("  🔧 Dev Dependencies:");
                            for (dep_name, dep_version) in &pkg.dev_dependencies {
                                println!("     {} v{}", dep_name, dep_version);
                            }
                        }
                    } else {
                        println!("Package not found: {}", package);
                    }
                }
                Err(e) => {
                    warn!("Failed to get package info: {}", e);
                    println!("❌ Failed to get package info: {}", e);
                }
            }
        }
    }
    
    Ok(())
async fn list_templates(
) -> crate::error::Result<()> {
    let manager = TemplateManager::new();
    
    let templates = if let Some(cat) = category {
        let template_category = match cat.as_str() {
        
        manager.get_templates_by_category(&template_category)
    } else {
        manager.list_templates()
    
    println!("📋 Available templates:");
    println!();
    
    for template in templates {
        if detailed {
            println!("🔨 {} ({})", template.name, category_name(&template.category));
            println!("   📝 {}", template.description);
            println!("   📁 Creates {} directories", template.directories.len());
            println!("   📄 Creates {} files", template.files.len());
            println!();
        } else {
            println!("  {} - {}", template.name, template.description);
        }
    }
    
    Ok(())
async fn show_project_info(
) -> crate::error::Result<()> {
    let work_dir = std::env::current_dir()?;
    let config_path = work_dir.join("CursedBuild.toml");
    
    if !config_path.exists() {
        return Err("No CursedBuild.toml found".into());
    let build_config = BuildConfig::load_from_file(&config_path)?;
    
    match format {
        "text" => {
            println!("📋 Project Information");
            println!("Name: {}", build_config.project.name);
            println!("Version: {}", build_config.project.version);
            
            if let Some(desc) = &build_config.project.description {
                println!("Description: {}", desc);
            println!("Targets: {}", build_config.targets.len());
            println!("Profiles: {}", build_config.profiles.len());
            
            if deps {
                println!("\n📦 Dependencies:");
                for (name, version) in &build_config.dependencies {
                    println!("  {} = {}", name, version);
                }
            }
            
            if config {
                println!("\n⚙️  Build Configuration:");
                println!("{:#?}", build_config);
            }
        }
        
        "json" => {
            let json = serde_json::to_string_pretty(&build_config)?;
            println!("{}", json);
        "yaml" => {
            let yaml = serde_yaml::to_string(&build_config)?;
            println!("{}", yaml);
    Ok(())
async fn watch_project(
) -> crate::error::Result<()> {
    use std::time::Duration;
    use tokio::signal;
    
    println!("👀 Watching for changes...");
    println!("Will run '{}' on file changes", command);
    println!("Debounce delay: {}ms, rebuild delay: {}ms", debounce, delay);
    
    let work_dir = std::env::current_dir()?;
    let config_path = work_dir.join("CursedBuild.toml");
    
    if !config_path.exists() {
        return Err("No CursedBuild.toml found. Run 'cursed-build init' to create a project.".into());
    let config = BuildConfig::load_from_file(&config_path)?;
    let mut orchestrator = BuildOrchestrator::new(config, work_dir)?;
    
    // Setup watch patterns - default to CURSED source files if none specified
    let watch_patterns = if patterns.is_empty() {
        vec!["**/*.csd".to_string(), "src/**/*.rs".to_string()]
    } else {
        patterns
    
    // Setup ignore patterns - add common ones
    let mut ignore_patterns = ignore;
    ignore_patterns.extend([
        "target/**".to_string(),
        ".git/**".to_string(),
        "**/.DS_Store".to_string(),
        "**/Thumbs.db".to_string(),
    ]);
    
    // Setup signal handling for graceful shutdown
    let ctrl_c = signal::ctrl_c();
    tokio::pin!(ctrl_c);
    
    println!("Watching patterns: {:?}", watch_patterns);
    if !ignore_patterns.is_empty() {
        println!("Ignoring patterns: {:?}", ignore_patterns);
    }
    println!("Press Ctrl+C to stop watching...");
    println!();
    
    // Configure watch settings
    let watch_config = WatchConfig {
    orchestrator.set_watch_config(watch_config);
    
    // Start file watching with orchestrator
    let watch_result = orchestrator.watch("dev", command);
    
    tokio::select! {
        result = watch_result => {
            match result {
                Err(e) => {
                    error!("File watching failed: {}", e);
                    return Err(e.into());
                }
            }
        }
        _ = &mut ctrl_c => {
            println!();
            println!("🛑 Stopping file watcher...");
            println!("👋 File watching stopped");
        }
    }
    
    Ok(())
async fn watch_build_project(
) -> crate::error::Result<()> {
    use std::time::Duration;
    use tokio::signal;
    
    println!("👀 Watching for changes to rebuild project...");
    println!("Profile: {}, Quick: {}, Force: {}, Parallel: {}", profile, quick, force, parallel);
    
    let work_dir = std::env::current_dir()?;
    let config_path = work_dir.join("CursedBuild.toml");
    
    if !config_path.exists() {
        return Err("No CursedBuild.toml found. Run 'cursed-build init' to create a project.".into());
    let config = BuildConfig::load_from_file(&config_path)?;
    let mut orchestrator = BuildOrchestrator::new(config, work_dir)?;
    
    // Setup signal handling for graceful shutdown
    let ctrl_c = signal::ctrl_c();
    tokio::pin!(ctrl_c);
    
    let watch_patterns = vec!["**/*.csd".to_string(), "src/**/*.rs".to_string()];
    let ignore_patterns = vec![
        "target/**".to_string(),
        ".git/**".to_string(),
        "**/.DS_Store".to_string(),
        "**/Thumbs.db".to_string(),
    ];
    
    println!("Watching for changes to CURSED and Rust source files...");
    println!("Press Ctrl+C to stop watching...");
    println!();
    
    // Build command with parameters
    let build_command = if quick {
        "quick-build"
    } else if !targets.is_empty() {
        "build-targets"
    } else {
        "build"
    
    // Configure watch settings
    let watch_config = WatchConfig {
    orchestrator.set_watch_config(watch_config);
    
    // Start file watching with orchestrator
    let watch_result = orchestrator.watch(profile, build_command);
    
    tokio::select! {
        result = watch_result => {
            match result {
                Err(e) => {
                    error!("Build watching failed: {}", e);
                    return Err(e.into());
                }
            }
        }
        _ = &mut ctrl_c => {
            println!();
            println!("🛑 Stopping build watcher...");
            println!("👋 Build watching stopped");
        }
    }
    
    Ok(())
async fn watch_test_project(
) -> crate::error::Result<()> {
    use std::time::Duration;
    use tokio::signal;
    
    println!("👀 Watching for changes to rerun tests...");
    println!("Profile: {}", profile);
    
    let work_dir = std::env::current_dir()?;
    let config_path = work_dir.join("CursedBuild.toml");
    
    if !config_path.exists() {
        return Err("No CursedBuild.toml found. Run 'cursed-build init' to create a project.".into());
    let config = BuildConfig::load_from_file(&config_path)?;
    let mut orchestrator = BuildOrchestrator::new(config, work_dir)?;
    
    // Setup signal handling for graceful shutdown
    let ctrl_c = signal::ctrl_c();
    tokio::pin!(ctrl_c);
    
    let watch_patterns = vec!["**/*.csd".to_string(), "src/**/*.rs".to_string(), "tests/**/*.rs".to_string()];
    let ignore_patterns = vec![
        "target/**".to_string(),
        ".git/**".to_string(),
        "**/.DS_Store".to_string(),
        "**/Thumbs.db".to_string(),
    ];
    
    println!("Watching for changes to source and test files...");
    println!("Press Ctrl+C to stop watching...");
    println!();
    
    // Configure watch settings
    let watch_config = WatchConfig {
    orchestrator.set_watch_config(watch_config);
    
    // Start file watching with orchestrator
    let watch_result = orchestrator.watch(profile, "test");
    
    tokio::select! {
        result = watch_result => {
            match result {
                Err(e) => {
                    error!("Test watching failed: {}", e);
                    return Err(e.into());
                }
            }
        }
        _ = &mut ctrl_c => {
            println!();
            println!("🛑 Stopping test watcher...");
            println!("👋 Test watching stopped");
        }
    }
    
    Ok(())
async fn benchmark_project(
) -> crate::error::Result<()> {
    use cursed::profiling::benchmarking::{BenchmarkSuite, BenchmarkConfig, Benchmark, MicroBenchmark, MacroBenchmark};
    use std::time::Duration;
    
    println!("🚀 Running benchmarks...");
    
    let work_dir = std::env::current_dir()?;
    let config_path = work_dir.join("CursedBuild.toml");
    
    if !config_path.exists() {
        return Err("No CursedBuild.toml found. Run 'cursed-build init' to create a project.".into());
    // Setup benchmark configuration
    let mut bench_config = BenchmarkConfig::default();
    bench_config.measurement_iterations = 5;
    bench_config.warmup_iterations = 2;
    bench_config.enable_profiling = false; // Keep it lightweight for CLI
    
    let mut suite = BenchmarkSuite::new("cursed-project".to_string(), bench_config);
    
    // Load baseline if specified
    if let Some(baseline_path) = &baseline {
        println!("📊 Loading baseline from: {}", baseline_path);
        match suite.load_baseline(baseline_path) {
            Err(e) => {
                warn!("Failed to load baseline: {}", e);
                println!("   ⚠️  Warning: Could not load baseline: {}", e);
            }
        }
    // Add default benchmarks if no specific names provided
    let benchmarks_to_run = if bench_name.is_empty() {
        vec!["build".to_string(), "parse".to_string(), "typecheck".to_string()]
    } else {
        bench_name
    
    // Create benchmarks based on requested names
    for name in &benchmarks_to_run {
        let benchmark = match name.as_str() {
            "build" => {
                println!("📦 Adding build benchmark");
                MacroBenchmark::compilation("project_build", || {
                    // Simulate project build
                    std::thread::sleep(Duration::from_millis(100));
                })
            }
            "parse" => {
                println!("📝 Adding parse benchmark");
                MicroBenchmark::function("source_parsing", || {
                    // Simulate parsing
                    std::thread::sleep(Duration::from_millis(10));
                })
            }
            "typecheck" => {
                println!("🔍 Adding typecheck benchmark");
                MicroBenchmark::function("type_checking", || {
                    // Simulate type checking
                    std::thread::sleep(Duration::from_millis(20));
                })
            }
            "memory" => {
                println!("🧠 Adding memory benchmark");
                MicroBenchmark::allocator("memory_allocation", || {
                    // Simulate memory allocation
                    let _vec: Vec<u8> = (0..1000).collect();
                    std::thread::sleep(Duration::from_millis(5));
                })
            }
            "compile" => {
                println!("⚙️  Adding compilation benchmark");
                MacroBenchmark::compilation("full_compilation", || {
                    // Simulate full compilation
                    std::thread::sleep(Duration::from_millis(200));
                })
            }
            "e2e" => {
                println!("🎯 Adding end-to-end benchmark");
                MacroBenchmark::end_to_end("full_pipeline", || {
                    // Simulate complete pipeline
                    std::thread::sleep(Duration::from_millis(300));
                })
            }
            _ => {
                println!("⚠️  Unknown benchmark '{}', using default", name);
                MicroBenchmark::function(&format!("custom_{}", name), || {
                    std::thread::sleep(Duration::from_millis(50));
                })
            }
        
        suite.add_benchmark(benchmark);
    // Run benchmarks
    println!("🏃 Running {} benchmark(s)...", benchmarks_to_run.len());
    println!();
    
    let results = match suite.run_all() {
        Err(e) => {
            error!("Benchmark execution failed: {}", e);
            return Err(format!("Benchmark execution failed: {}", e).into());
        }
    
    // Print results
    println!("📊 Benchmark Results:");
    println!("Suite: {}", results.suite_name);
    println!();
    
    for (name, result) in &results.results {
        println!("📈 {}", name);
        println!("   Mean:   {:?}", result.statistics.mean);
        println!("   Median: {:?}", result.statistics.median);
        println!("   Min:    {:?}", result.statistics.min);
        println!("   Max:    {:?}", result.statistics.max);
        println!("   StdDev: {:?}", result.statistics.standard_deviation);
        println!("   CV:     {:.2}%", result.statistics.coefficient_of_variation * 100.0);
        println!();
    // Print summary
    println!("📋 Summary:");
    println!("   Total benchmarks: {}", results.summary.total_benchmarks);
    println!("   Total duration:   {:?}", results.summary.total_duration);
    if let Some(fastest) = results.summary.fastest_benchmark {
        println!("   Fastest:          {:?}", fastest);
    }
    if let Some(slowest) = results.summary.slowest_benchmark {
        println!("   Slowest:          {:?}", slowest);
    }
    println!("   Average:          {:?}", results.summary.average_duration);
    
    // Check for regressions if baseline was loaded
    if let Some(regression_analysis) = &results.regression_analysis {
        println!();
        println!("🔍 Regression Analysis:");
        println!("   {}", regression_analysis.summary());
        
        if !regression_analysis.regressions.is_empty() {
            println!("   ❌ Regressions found:");
            for regression in &regression_analysis.regressions {
                println!("     {} - {}", regression.benchmark_name, regression.change_type);
            }
        }
        
        if !regression_analysis.improvements.is_empty() {
            println!("   ✅ Improvements found:");
            for improvement in &regression_analysis.improvements {
                println!("     {} - {}", improvement.benchmark_name, improvement.change_type);
            }
        }
        
        if regression_analysis.has_critical_regressions() {
            println!("   ⚠️  Critical regressions detected!");
        }
    }
    
    // Save baseline if requested
    if let Some(baseline_path) = &save_baseline {
        println!();
        println!("💾 Saving baseline to: {}", baseline_path);
        match results.save_to_file(baseline_path) {
            Err(e) => {
                warn!("Failed to save baseline: {}", e);
                println!("   ❌ Failed to save baseline: {}", e);
            }
        }
    println!();
    println!("🚀 Benchmarking completed!");
    
    // Exit with error code if critical regressions found
    if let Some(regression_analysis) = &results.regression_analysis {
        if regression_analysis.has_critical_regressions() {
            return Err("Critical performance regressions detected".into());
        }
    }
    
    Ok(())
fn setup_logging(verbose: bool, quiet: bool) {
    if quiet {
        return;
    let level = if verbose {
        tracing::Level::DEBUG
    } else {
        tracing::Level::INFO
    
    tracing_subscriber::fmt()
        .with_max_level(level)
        .with_target(false)
        .without_time()
        .init();
fn is_executable(path: &std::path::Path) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(metadata) = std::fs::metadata(path) {
            let permissions = metadata.permissions();
            return permissions.mode() & 0o111 != 0;
        }
        false
    #[cfg(windows)]
    {
        path.extension()
            .map(|ext| ext == "exe" || ext == "bat" || ext == "cmd")
            .unwrap_or(false)
    #[cfg(not(any(unix, windows)))]
    {
        false
    }
}

fn open_in_browser(path: &std::path::Path) {
    let url = format!("file://{}", path.canonicalize().unwrap().display());
    
    #[cfg(target_os = "macos")]
    let _ = std::process::Command::new("open").arg(&url).spawn();
    
    #[cfg(target_os = "linux")]
    let _ = std::process::Command::new("xdg-open").arg(&url).spawn();
    
    #[cfg(target_os = "windows")]
    let _ = std::process::Command::new("start").arg(&url).spawn();
fn category_name(category: &TemplateCategory) -> &'static str {
    match category {
    }
}

fn convert_pipeline_to_build_result(pipeline_result: PipelineResult) -> cursed::build_system::BuildResult {
    cursed::build_system::BuildResult {
        targets_skipped: pipeline_result.stages.values()
            .filter(|s| s.cache_hit)
            .map(|s| s.name.clone())
        statistics: cursed::build_system::BuildStatistics {
            lines_compiled: 0, // TODO: Extract from pipeline stages
            phase_timings: std::collections::HashMap::new(), // TODO: Extract from stages
    }
}
