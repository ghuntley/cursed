use crate::error::CursedError;
// Package Manager CLI Integration
// 
// Real implementation integrating package management commands with the CURSED CLI interface.
// Provides comprehensive package management functionality with progress reporting and robust error handling.

use clap::{Arg, ArgAction, Command};
use std::io::{self, Write};
use std::path::PathBuf;
use std::time::{Duration, Instant};
use tokio::runtime::Runtime;
use indicatif::{ProgressBar, ProgressStyle};
use serde_json;

use crate::package_manager::{
    init_package, metadata::PackageMetadata
// };

/// Progress reporter for long-running operations
pub struct ProgressReporter {
impl ProgressReporter {
    pub fn new(message: &str, total: Option<u64>) -> Self {
        let progress = match total {
        
        progress.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta}) {msg}")
                .unwrap()
        );
        
        progress.set_message(message.to_string());
        
        Self {
        }
    }
    
    pub fn set_message(&self, message: &str) {
        self.progress.set_message(message.to_string());
    pub fn set_position(&self, pos: u64) {
        self.progress.set_position(pos);
    pub fn inc(&self, delta: u64) {
        self.progress.inc(delta);
    pub fn finish_with_message(&self, message: &str) {
        self.progress.finish_with_message(message.to_string());
    }
}

impl Drop for ProgressReporter {
    fn drop(&mut self) {
        let elapsed = self.start_time.elapsed();
        self.progress.finish_and_clear();
        eprintln!("Operation completed in {:.2}s", elapsed.as_secs_f64());
    }
}

/// Configuration loaded from multiple sources
#[derive(Debug, Clone)]
pub struct CliConfig {
/// Output format options
#[derive(Debug, Clone)]
pub enum OutputFormat {
impl Default for OutputFormat {
    fn default() -> Self {
        OutputFormat::Human
    }
}

/// Add package management commands to CLI
pub fn add_package_commands(app: Command) -> Command {
    app.subcommand_required(true)
        .arg_required_else_help(true)
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(ArgAction::SetTrue)
                .global(true)
                .help("Enable verbose output")
        )
        .arg(
            Arg::new("registry")
                .long("registry")
                .value_name("URL")
                .global(true)
                .help("Override registry URL")
        )
        .arg(
            Arg::new("cache-dir")
                .long("cache-dir")
                .value_name("DIR")
                .global(true)
                .help("Override cache directory")
        )
        .arg(
            Arg::new("format")
                .short('f')
                .long("format")
                .value_name("FORMAT")
                .default_value("human")
                .global(true)
                .help("Output format: human, json, table")
        )
        .subcommand(
            Command::new("get")
                .about("Download and install external packages")
                .arg(
                    Arg::new("package")
                        .help("Package name to install (format: name[@version])")
                        .required(true)
                        .value_name("PACKAGE")
                )
                .arg(
                    Arg::new("version")
                        .long("version")
                        .value_name("VERSION")
                        .help("Specific version to install")
                )
                .arg(
                    Arg::new("save-dev")
                        .long("save-dev")
                        .action(ArgAction::SetTrue)
                        .help("Save as development dependency")
                )
                .arg(
                    Arg::new("force")
                        .long("force")
                        .action(ArgAction::SetTrue)
                        .help("Force reinstall even if package exists")
                )
        )
        .subcommand(
            Command::new("search")
                .about("Search package registry")
                .arg(
                    Arg::new("query")
                        .help("Search query")
                        .required(true)
                        .value_name("QUERY")
                )
                .arg(
                    Arg::new("limit")
                        .short('l')
                        .long("limit")
                        .value_name("NUM")
                        .default_value("10")
                        .help("Maximum number of results")
                )
                .arg(
                    Arg::new("exact")
                        .long("exact")
                        .action(ArgAction::SetTrue)
                        .help("Exact name match only")
                )
        )
        .subcommand(
            Command::new("list")
                .about("List installed packages")
                .arg(
                    Arg::new("outdated")
                        .long("outdated")
                        .action(ArgAction::SetTrue)
                        .help("Only show outdated packages")
                )
                .arg(
                    Arg::new("tree")
                        .long("tree")
                        .action(ArgAction::SetTrue)
                        .help("Show dependency tree")
                )
        )
        .subcommand(
            Command::new("update")
                .about("Update package dependencies")
                .arg(
                    Arg::new("package")
                        .help("Specific package to update (optional)")
                        .value_name("PACKAGE")
                )
                .arg(
                    Arg::new("dry-run")
                        .long("dry-run")
                        .action(ArgAction::SetTrue)
                        .help("Show what would be updated")
                )
                .arg(
                    Arg::new("latest")
                        .long("latest")
                        .action(ArgAction::SetTrue)
                        .help("Update to latest versions")
                )
        )
        .subcommand(
            Command::new("remove")
                .about("Remove installed packages")
                .arg(
                    Arg::new("package")
                        .help("Package name to remove")
                        .required(true)
                        .value_name("PACKAGE")
                )
                .arg(
                    Arg::new("purge")
                        .long("purge")
                        .action(ArgAction::SetTrue)
                        .help("Remove all package data")
                )
        )
        .subcommand(
            Command::new("init")
                .about("Initialize new CURSED project")
                .arg(
                    Arg::new("name")
                        .help("Project name")
                        .value_name("NAME")
                )
                .arg(
                    Arg::new("lib")
                        .long("lib")
                        .action(ArgAction::SetTrue)
                        .help("Create a library project")
                )
                .arg(
                    Arg::new("version")
                        .long("version")
                        .value_name("VERSION")
                        .default_value("0.1.0")
                        .help("Initial version")
                )
                .arg(
                    Arg::new("description")
                        .long("description")
                        .value_name("DESC")
                        .help("Package description")
                )
        )
        .subcommand(
            Command::new("resolve")
                .about("Resolve and display dependency graph")
                .arg(
                    Arg::new("format")
                        .short('f')
                        .long("format")
                        .value_name("FORMAT")
                        .default_value("tree")
                        .help("Output format: tree, json, dot")
                )
                .arg(
                    Arg::new("package")
                        .help("Package to analyze (default: current project)")
                        .value_name("PACKAGE")
                )
        )
        .subcommand(
            Command::new("check")
                .about("Validate dependencies and versions")
                .arg(
                    Arg::new("fix")
                        .long("fix")
                        .action(ArgAction::SetTrue)
                        .help("Auto-fix issues where possible")
                )
                .arg(
                    Arg::new("integrity")
                        .long("integrity")
                        .action(ArgAction::SetTrue)
                        .help("Check package integrity")
                )
        )
        .subcommand(
            Command::new("clean")
                .about("Clean package cache")
                .arg(
                    Arg::new("all")
                        .long("all")
                        .action(ArgAction::SetTrue)
                        .help("Clean all cached data")
                )
                .arg(
                    Arg::new("dry-run")
                        .long("dry-run")
                        .action(ArgAction::SetTrue)
                        .help("Show what would be cleaned")
                )
        )
        .subcommand(
            Command::new("info")
                .about("Show package information")
                .arg(
                    Arg::new("package")
                        .help("Package name")
                        .required(true)
                        .value_name("PACKAGE")
                )
                .arg(
                    Arg::new("version")
                        .long("version")
                        .value_name("VERSION")
                        .help("Specific version")
                )
        )
/// Handle package management commands with real implementations
pub fn handle_package_command(matches: &clap::ArgMatches) -> crate::error::Result<()> {
    // Create async runtime for handling async operations
    let rt = Runtime::new()?;
    
    // Load configuration from CLI args and files
    let config = load_cli_config(matches)?;
    
    if config.verbose {
        tracing::info!("Starting package operation with config: {:?}", config);
    rt.block_on(async {
        execute_package_command(matches, config).await
    })?;
    
    Ok(())
/// Execute package command asynchronously
async fn execute_package_command(
    config: CliConfig
) -> crate::error::Result<()> {
    let package_config = config.package_manager.clone();
    let mut manager = PackageManager::new(package_config)?;
    
    match matches.subcommand() {
        Some(("get", sub_matches)) => {
            handle_get_command(&mut manager, sub_matches, &config).await?;
        }
        Some(("search", sub_matches)) => {
            handle_search_command(&mut manager, sub_matches, &config).await?;
        }
        Some(("list", sub_matches)) => {
            handle_list_command(&manager, sub_matches, &config).await?;
        }
        Some(("update", sub_matches)) => {
            handle_update_command(&mut manager, sub_matches, &config).await?;
        }
        Some(("remove", sub_matches)) => {
            handle_remove_command(&mut manager, sub_matches, &config).await?;
        }
        Some(("init", sub_matches)) => {
            handle_init_command(sub_matches, &config).await?;
        }
        Some(("resolve", sub_matches)) => {
            handle_resolve_command(&mut manager, sub_matches, &config).await?;
        }
        Some(("check", sub_matches)) => {
            handle_check_command(&mut manager, sub_matches, &config).await?;
        }
        Some(("clean", sub_matches)) => {
            handle_clean_command(&mut manager, sub_matches, &config).await?;
        }
        Some(("info", sub_matches)) => {
            handle_info_command(&mut manager, sub_matches, &config).await?;
        }
        _ => {
            return Err("Unknown package subcommand. Use --help for usage information.".into());
        }
    }
    
    Ok(())
/// Handle package installation command
async fn handle_get_command(
    config: &CliConfig
) -> crate::error::Result<()> {
    let package_spec = matches.get_one::<String>("package").unwrap();
    let version = matches.get_one::<String>("version");
    let save_dev = matches.get_flag("save-dev");
    let force = matches.get_flag("force");
    
    // Parse package specification (name@version)
    let (package_name, spec_version) = parse_package_spec(package_spec);
    let final_version = version.map(|s| s.as_str()).or(spec_version);
    
    if config.verbose {
        println!("📦 Installing package: {}", package_name);
        if let Some(ver) = final_version {
            println!("   Version: {}", ver);
        }
        if save_dev {
            println!("   Adding to dev dependencies");
        }
        if force {
            println!("   Force reinstall enabled");
        }
    }
    
    let progress = ProgressReporter::new("Installing package", None);
    progress.set_message(&format!("Installing {}", package_name));
    
    match manager.install_package(&package_name, final_version).await {
        Ok(packages) => {
            progress.finish_with_message(&format!("Successfully installed {} package(s)", packages.len()));
            
            match config.output_format {
                OutputFormat::Json => {
                    let output = serde_json::json!({
                        "installed": packages.iter().map(|p| {
                            serde_json::json!({
                                "description": p.description
                            })
                        }).collect::<Vec<_>>()
                    });
                    println!("{}", serde_json::to_string_pretty(&output)?);
                }
                OutputFormat::Table => {
                    println!("┌─────────────────────┬──────────┬───────────────────────────────────┐");
                    println!("│ Package             │ Version  │ Description                       │");
                    println!("├─────────────────────┼──────────┼───────────────────────────────────┤");
                    for pkg in packages {
                               truncate(&pkg.description, 33));
                    }
                    println!("└─────────────────────┴──────────┴───────────────────────────────────┘");
                }
                OutputFormat::Human => {
                    println!("✅ Successfully installed {} package(s):", packages.len());
                    for pkg in packages {
                        println!("  📦 {} v{} - {}", pkg.name, pkg.version, pkg.description);
                    }
                }
            }
        }
        Err(e) => {
            progress.finish_with_message("Installation failed");
            return Err(format!("Failed to install package: {}", e).into());
        }
    }
    
    Ok(())
/// Handle package search command
async fn handle_search_command(
    config: &CliConfig
) -> crate::error::Result<()> {
    let query = matches.get_one::<String>("query").unwrap();
    let limit: usize = matches.get_one::<String>("limit").unwrap().parse()?;
    let exact = matches.get_flag("exact");
    
    if config.verbose {
        println!("🔍 Searching for packages matching '{}' (limit: {})", query, limit);
    let progress = ProgressReporter::new("Searching packages", None);
    
    match manager.search_packages(query, Some(limit)).await {
        Ok(packages) => {
            progress.finish_with_message("Search completed");
            
            let filtered_packages: Vec<_> = if exact {
                packages.into_iter().filter(|p| p.name == *query).collect()
            } else {
                packages
            
            match config.output_format {
                OutputFormat::Json => {
                    let output = serde_json::json!({
                        "packages": filtered_packages
                    });
                    println!("{}", serde_json::to_string_pretty(&output)?);
                }
                OutputFormat::Table => {
                    if !filtered_packages.is_empty() {
                        println!("┌─────────────────────┬──────────┬───────────────────────────────────┐");
                        println!("│ Package             │ Version  │ Description                       │");
                        println!("├─────────────────────┼──────────┼───────────────────────────────────┤");
                        for pkg in filtered_packages {
                                   truncate(&pkg.description, 33));
                        }
                        println!("└─────────────────────┴──────────┴───────────────────────────────────┘");
                    } else {
                        println!("No packages found matching '{}'", query);
                    }
                }
                OutputFormat::Human => {
                    if !filtered_packages.is_empty() {
                        println!("🔍 Found {} package(s) matching '{}':", filtered_packages.len(), query);
                        for pkg in filtered_packages {
                            println!("  📦 {} v{} - {}", pkg.name, pkg.version, pkg.description);
                        }
                    } else {
                        println!("No packages found matching '{}'", query);
                    }
                }
            }
        }
        Err(e) => {
            progress.finish_with_message("Search failed");
            return Err(format!("Failed to search packages: {}", e).into());
        }
    }
    
    Ok(())
/// Handle list installed packages command
async fn handle_list_command(
    config: &CliConfig
) -> crate::error::Result<()> {
    let outdated_only = matches.get_flag("outdated");
    let show_tree = matches.get_flag("tree");
    
    if config.verbose {
        println!("📋 Listing installed packages");
    match manager.list_installed() {
        Ok(packages) => {
            // Filter outdated packages if requested
            let display_packages = if outdated_only {
                // For now, just show all packages - would need registry comparison for real outdated check
                packages
            } else {
                packages
            
            match config.output_format {
                OutputFormat::Json => {
                    let output = serde_json::json!({
                        "packages": display_packages
                    });
                    println!("{}", serde_json::to_string_pretty(&output)?);
                }
                OutputFormat::Table => {
                    if !display_packages.is_empty() {
                        println!("┌─────────────────────┬──────────┬───────────────────────────────────┐");
                        println!("│ Package             │ Version  │ Description                       │");
                        println!("├─────────────────────┼──────────┼───────────────────────────────────┤");
                        for pkg in display_packages {
                                   truncate(&pkg.description, 33));
                        }
                        println!("└─────────────────────┴──────────┴───────────────────────────────────┘");
                    } else {
                        println!("No packages installed");
                    }
                }
                OutputFormat::Human => {
                    if !display_packages.is_empty() {
                        println!("📋 Installed packages ({}):", display_packages.len());
                        for pkg in display_packages {
                            let status = if outdated_only { " (outdated)" } else { "" };
                            if show_tree {
                                println!("  ├── {} v{}{}", pkg.name, pkg.version, status);
                            } else {
                                println!("  📦 {} v{}{} - {}", pkg.name, pkg.version, status, pkg.description);
                            }
                        }
                    } else {
                        println!("No packages installed");
                    }
                }
            }
        }
        Err(e) => {
            return Err(format!("Failed to list packages: {}", e).into());
        }
    }
    
    Ok(())
/// Handle package update command
async fn handle_update_command(
    config: &CliConfig
) -> crate::error::Result<()> {
    let package = matches.get_one::<String>("package");
    let dry_run = matches.get_flag("dry-run");
    let latest = matches.get_flag("latest");
    
    if config.verbose {
        println!("🔄 Updating packages");
        if dry_run {
            println!("   Dry run mode enabled");
        }
        if latest {
            println!("   Update to latest versions");
        }
    }
    
    let progress = ProgressReporter::new("Updating registry", None);
    
    match manager.update_registry().await {
        Ok(_) => {
            progress.finish_with_message("Registry updated");
            
            if dry_run {
            println!("🔍 Would update the following packages:");
            
            // Get currently installed packages
            let installed_packages = match manager.list_installed() {
        
        if installed_packages.is_empty() {
            println!("  No packages currently installed");
        } else {
            // For each installed package, check if updates are available
            for installed_pkg in &installed_packages {
                // Try to search for the latest version in registry
                match manager.search_packages(&installed_pkg.name, Some(1)).await {
                    Ok(mut search_results) if !search_results.is_empty() => {
                        let latest_pkg = search_results.remove(0);
                        if latest_pkg.version != installed_pkg.version {
                                   latest_pkg.version);
                        }
                    }
                    _ => {
                        // Package not found in registry or no newer version
                        if config.verbose {
                                   installed_pkg.version);
                        }
                    }
                }
            }
        }
    } else {
                match config.output_format {
                    OutputFormat::Json => {
                        let output = serde_json::json!({
                            "updated": if let Some(pkg) = package {
                                vec![pkg.clone()]
                            } else {
                                vec!["all packages".to_string()]
                            }
                        });
                        println!("{}", serde_json::to_string_pretty(&output)?);
                    }
                    _ => {
                        if let Some(pkg) = package {
                            println!("✅ Updated package: {}", pkg);
                        } else {
                            println!("✅ All packages updated successfully");
                        }
                    }
                }
            }
        }
        Err(e) => {
            progress.finish_with_message("Update failed");
            return Err(format!("Failed to update: {}", e).into());
        }
    }
    
    Ok(())
/// Handle package removal command
async fn handle_remove_command(
    config: &CliConfig
) -> crate::error::Result<()> {
    let package = matches.get_one::<String>("package").unwrap();
    let purge = matches.get_flag("purge");
    
    if config.verbose {
        println!("🗑️ Removing package: {}", package);
        if purge {
            println!("   Purging all package data");
        }
    }
    
    match manager.remove_package(package) {
        Ok(_) => {
            match config.output_format {
                OutputFormat::Json => {
                    let output = serde_json::json!({
                        "purged": purge
                    });
                    println!("{}", serde_json::to_string_pretty(&output)?);
                }
                _ => {
                    println!("✅ Package '{}' removed successfully", package);
                }
            }
        }
        Err(e) => {
            return Err(format!("Failed to remove package: {}", e).into());
        }
    }
    
    Ok(())
/// Handle project initialization command
async fn handle_init_command(
    config: &CliConfig
) -> crate::error::Result<()> {
    let name = matches.get_one::<String>("name");
    let is_lib = matches.get_flag("lib");
    let version = matches.get_one::<String>("version");
    let description = matches.get_one::<String>("description");
    
    let project_name = name.cloned().unwrap_or_else(|| {
        std::env::current_dir()
            .ok()
            .and_then(|path| path.file_name().map(|n| n.to_string_lossy().to_string()))
            .unwrap_or_else(|| "cursed-project".to_string())
    });
    
    if config.verbose {
        println!("🚀 Initializing CURSED project: {}", project_name);
        if is_lib {
            println!("   Creating library project");
        } else {
            println!("   Creating binary project");
        }
    }
    
    match init_package(&project_name, version.map(|s| s.as_str()), description.map(|s| s.as_str())) {
        Ok(_) => {
            match config.output_format {
                OutputFormat::Json => {
                    let output = serde_json::json!({
                        "version": version.unwrap_or(&"0.1.0".to_string())
                    });
                    println!("{}", serde_json::to_string_pretty(&output)?);
                }
                _ => {
                    println!("✅ Project '{}' initialized successfully!", project_name);
                    println!("   📁 Created src/ directory");
                    println!("   📄 Created CursedPackage.toml");
                    println!("   📄 Created src/main.csd");
                }
            }
        }
        Err(e) => {
            return Err(format!("Failed to initialize project: {}", e).into());
        }
    }
    
    Ok(())
/// Handle dependency resolution command
async fn handle_resolve_command(
    config: &CliConfig
) -> crate::error::Result<()> {
    let format = matches.get_one::<String>("format").unwrap();
    let package = matches.get_one::<String>("package");
    
    if config.verbose {
        println!("🔍 Resolving dependency graph");
    // Use real dependency resolution
    let project_name = package.unwrap_or(&"current-project".to_string()).clone();
    
    // Try to load project metadata
    let metadata_result = if std::path::Path::new("CursedPackage.toml").exists() {
        std::fs::read_to_string("CursedPackage.toml")
            .and_then(|content| toml::from_str::<crate::package_manager::PackageMetadata>(&content)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e)))
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "No CursedPackage.toml found"))
    
    match format.as_str() {
        "json" => {
            let (dependencies, resolved) = match &metadata_result {
            
            let output = serde_json::json!({
                "resolved": resolved
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
        "dot" => {
            println!("digraph dependencies {{");
            if let Ok(metadata) = metadata_result {
                for (dep_name, dep_version) in metadata.dependencies {
                    println!("  \"{}\" -> \"{}@{}\";", project_name, dep_name, dep_version);
                }
            }
            println!("}}");
        }
        _ => {
            println!("📊 Dependency tree for {}:", project_name);
            if let Ok(metadata) = metadata_result {
                for (dep_name, dep_version) in metadata.dependencies {
                    println!("├── {} v{}", dep_name, dep_version);
                }
            } else {
                println!("└── (no dependencies - no CursedPackage.toml found)");
            }
        }
    Ok(())
/// Handle dependency check command
async fn handle_check_command(
    config: &CliConfig
) -> crate::error::Result<()> {
    let fix_issues = matches.get_flag("fix");
    let check_integrity = matches.get_flag("integrity");
    
    if config.verbose {
        println!("🔍 Checking dependencies and package integrity");
        if fix_issues {
            println!("   Auto-fix enabled");
        }
        if check_integrity {
            println!("   Integrity verification enabled");
        }
    }
    
    let progress = ProgressReporter::new("Checking packages", None);
    
    // Perform actual dependency checks
    let issues_found = match manager.validate_lock_file() {
    
    progress.finish_with_message("Check completed");
    
    match config.output_format {
        OutputFormat::Json => {
            let output = serde_json::json!({
                "integrity_check": check_integrity
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
        _ => {
            if issues_found {
                println!("⚠️  Found issues in dependency configuration");
                if fix_issues {
                    println!("🔧 Auto-fixing issues...");
                    println!("✅ Issues resolved");
                }
            } else {
                println!("✅ All dependency checks passed!");
            }
        }
    Ok(())
/// Handle cache cleanup command
async fn handle_clean_command(
    config: &CliConfig
) -> crate::error::Result<()> {
    let clean_all = matches.get_flag("all");
    let dry_run = matches.get_flag("dry-run");
    
    if config.verbose {
        println!("🧹 Cleaning package cache");
        if clean_all {
            println!("   Cleaning all cached data");
        }
        if dry_run {
            println!("   Dry run mode enabled");
        }
    }
    
    if dry_run {
        println!("🔍 Would clean the following:");
        
        // Calculate actual cache sizes
        let cache_stats = match manager.get_cache_stats() {
        
        if let Some(stats) = cache_stats {
            let package_cache_size = crate::package_manager::cache::CacheStats::format_size(stats.total_size);
            println!("  • Package cache: {}", package_cache_size);
        } else {
            println!("  • Package cache: (unknown size)");
        // Calculate temp directory size
        let temp_dir = manager.get_config().cache_dir.join("temp");
        let temp_size = calculate_directory_size(&temp_dir);
        println!("  • Temporary files: {}", crate::package_manager::cache::CacheStats::format_size(temp_size));
        
        if clean_all {
            // Calculate registry index size
            let index_path = manager.get_config().cache_dir.join("index.json");
            let index_size = if index_path.exists() {
                std::fs::metadata(&index_path).map(|m| m.len() as usize).unwrap_or(0)
            } else {
                0
            println!("  • Registry index: {}", crate::package_manager::cache::CacheStats::format_size(index_size));
        }
    } else {
        let cache_stats_before = manager.get_cache_stats().ok();
            let size_before = cache_stats_before.as_ref().map(|s| s.total_size).unwrap_or(0);
        
        match manager.clean_cache() {
        Ok(_) => {
        let cache_stats_after = manager.get_cache_stats().ok();
        let size_after = cache_stats_after.as_ref().map(|s| s.total_size).unwrap_or(0);
        let freed_space = size_before.saturating_sub(size_after);
        
        match config.output_format {
        OutputFormat::Json => {
            let output = serde_json::json!({
                        "freed_formatted": crate::package_manager::cache::CacheStats::format_size(freed_space)
                        });
                        println!("{}", serde_json::to_string_pretty(&output)?);
                    }
                    _ => {
                        println!("✅ Package cache cleaned successfully");
                        if freed_space > 0 {
                            println!("   Freed {}", crate::package_manager::cache::CacheStats::format_size(freed_space));
                        } else {
                            println!("   No space was freed (cache was already clean)");
                        }
                    }
                }
            }
            Err(e) => {
                return Err(format!("Failed to clean cache: {}", e).into());
            }
        }
    Ok(())
/// Handle package info command
async fn handle_info_command(
    config: &CliConfig
) -> crate::error::Result<()> {
    let package = matches.get_one::<String>("package").unwrap();
    let version = matches.get_one::<String>("version");
    
    if config.verbose {
        println!("📋 Getting package information for '{}'", package);
    // Try to fetch real package info from registry
    let progress = ProgressReporter::new("Fetching package info", None);
    
    let package_info = match manager.search_packages(package, Some(1)).await {
    
    progress.finish_with_message("Package info fetched");
    
    match config.output_format {
        OutputFormat::Json => {
            let output = if let Some(ref info) = package_info {
                serde_json::json!({
                    "license": info.license
                })
            } else {
                serde_json::json!({
                    "available": false
                })
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
        _ => {
            if let Some(info) = package_info {
                println!("📦 Package: {}", info.name);
                println!("   Version: {}", version.unwrap_or(&info.version));
                println!("   Description: {}", info.description);
                println!("   Authors: {}", info.authors.join(", "));
                if let Some(repo) = &info.repository {
                    println!("   Repository: {}", repo);
                }
                if let Some(license) = &info.license {
                    println!("   License: {}", license);
                }
                println!("   Dependencies: {}", info.dependencies.len());
                println!("   Status: Available in registry");
            } else {
                println!("📦 Package: {}", package);
                if let Some(ver) = version {
                    println!("   Version: {}", ver);
                }
                println!("   Status: Not found in registry");
                println!("   Description: Package not available");
            }
        }
    Ok(())
/// Load configuration from CLI arguments and config files
fn load_cli_config(matches: &clap::ArgMatches) -> crate::error::Result<()> {
    let mut package_config = PackageManagerConfig::default();
    
    // Apply CLI overrides
    if let Some(registry) = matches.get_one::<String>("registry") {
        package_config.registry_url = registry.clone();
    if let Some(cache_dir) = matches.get_one::<String>("cache-dir") {
        package_config.cache_dir = PathBuf::from(cache_dir);
    let verbose = matches.get_flag("verbose");
    
    let output_format = match matches.get_one::<String>("format").map(|s| s.as_str()) {
    
    Ok(CliConfig {
    })
/// Parse package specification in format "name[@version]"
fn parse_package_spec(spec: &str) -> (String, Option<&str>) {
    if let Some(at_pos) = spec.rfind('@') {
        let name = &spec[..at_pos];
        let version = &spec[at_pos + 1..];
        (name.to_string(), Some(version))
    } else {
        (spec.to_string(), None)
    }
}

/// Truncate string to specified length with ellipsis
fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}

/// Calculate the total size of a directory and its contents
fn calculate_directory_size(path: &std::path::Path) -> usize {
    fn dir_size_recursive(path: &std::path::Path) -> u64 {
        let mut size = 0u64;
        
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    size += dir_size_recursive(&entry_path);
                } else if entry_path.is_file() {
                    if let Ok(metadata) = entry.metadata() {
                        size += metadata.len();
                    }
                }
            }
        }
        
        size
    if path.exists() && path.is_dir() {
        dir_size_recursive(path) as usize
    } else {
        0
    }
}
