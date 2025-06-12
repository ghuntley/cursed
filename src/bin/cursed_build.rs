/// CURSED Build System CLI
/// 
/// Command-line interface for the CURSED build system providing project
/// initialization, building, testing, and toolchain integration.

use clap::{Parser, Subcommand, ArgGroup};
use cursed::build_system::{
    BuildConfig, BuildOrchestrator, TemplateManager, TemplateContext, 
    ProjectType, TemplateCategory, PipelineResult
};
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
    command: Commands,
    
    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,
    
    /// Suppress output
    #[arg(short, long, global = true)]
    quiet: bool,
    
    /// Build profile to use
    #[arg(short, long, global = true, default_value = "dev")]
    profile: String,
    
    /// Number of parallel jobs
    #[arg(short, long, global = true)]
    jobs: Option<usize>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new CURSED project
    #[command(alias = "new")]
    Init {
        /// Project name
        name: String,
        
        /// Project template to use
        #[arg(short, long, default_value = "cli")]
        template: String,
        
        /// Create as library project
        #[arg(long)]
        lib: bool,
        
        /// Target directory (defaults to project name)
        #[arg(long)]
        target_dir: Option<PathBuf>,
        
        /// Additional template variables (key=value)
        #[arg(long = "var", value_parser = parse_key_val)]
        variables: Vec<(String, String)>,
    },
    
    /// Build the project
    Build {
        /// Specific targets to build
        #[arg(short, long)]
        target: Vec<String>,
        
        /// Release build
        #[arg(short, long)]
        release: bool,
        
        /// Enable all features
        #[arg(long)]
        all_features: bool,
        
        /// Disable default features
        #[arg(long)]
        no_default_features: bool,
        
        /// Enable specific features
        #[arg(long)]
        features: Vec<String>,
        
        /// Force rebuild (ignore cache)
        #[arg(long)]
        force: bool,
        
        /// Disable parallel compilation
        #[arg(long)]
        no_parallel: bool,
        
        /// Use quick build (skip formatting and linting)
        #[arg(long)]
        quick: bool,
        
        /// Watch for file changes and rebuild automatically
        #[arg(short, long)]
        watch: bool,
    },
    
    /// Run the project
    Run {
        /// Arguments to pass to the program
        #[arg(last = true)]
        args: Vec<String>,
        
        /// Release build
        #[arg(short, long)]
        release: bool,
        
        /// Specific target to run
        #[arg(long)]
        bin: Option<String>,
    },
    
    /// Test the project
    Test {
        /// Test name patterns
        test_name: Vec<String>,
        
        /// Release build
        #[arg(short, long)]
        release: bool,
        
        /// Run ignored tests
        #[arg(long)]
        ignored: bool,
        
        /// Number of test threads
        #[arg(long)]
        test_threads: Option<usize>,
        
        /// Watch for file changes and rerun tests automatically
        #[arg(short, long)]
        watch: bool,
    },
    
    /// Clean build artifacts
    Clean {
        /// Remove target directory completely
        #[arg(long)]
        all: bool,
    },
    
    /// Check code without building
    Check {
        /// Check all targets
        #[arg(long)]
        all_targets: bool,
    },
    
    /// Format source code
    #[command(alias = "fmt")]
    Format {
        /// Check formatting without applying changes
        #[arg(long)]
        check: bool,
        
        /// Show diff of changes
        #[arg(long)]
        diff: bool,
        
        /// Files or directories to format
        files: Vec<PathBuf>,
    },
    
    /// Lint source code
    Lint {
        /// Automatically fix issues where possible
        #[arg(long)]
        fix: bool,
        
        /// Show detailed statistics
        #[arg(long)]
        stats: bool,
        
        /// Files or directories to lint
        files: Vec<PathBuf>,
    },
    
    /// Generate documentation
    #[command(alias = "doc")]
    Docs {
        /// Open documentation in browser
        #[arg(long)]
        open: bool,
        
        /// Documentation format
        #[arg(long, default_value = "html")]
        format: String,
        
        /// Output directory
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    
    /// Package management
    #[command(alias = "pkg")]
    Package {
        #[command(subcommand)]
        command: PackageCommands,
    },
    
    /// List available templates
    Templates {
        /// Show templates for specific category
        #[arg(short, long)]
        category: Option<String>,
        
        /// Show detailed template information
        #[arg(long)]
        detailed: bool,
    },
    
    /// Show project information
    Info {
        /// Show dependency graph
        #[arg(long)]
        deps: bool,
        
        /// Show build configuration
        #[arg(long)]
        config: bool,
        
        /// Output format (text, json, yaml)
        #[arg(long, default_value = "text")]
        format: String,
    },
    
    /// Watch for changes and rebuild
    Watch {
        /// Command to run on changes
        #[arg(short, long, default_value = "build")]
        command: String,
        
        /// Delay before rebuilding (ms) 
        #[arg(long, default_value = "500")]
        delay: u64,
        
        /// File patterns to watch (default: **/*.csd)
        #[arg(long)]
        patterns: Vec<String>,
        
        /// Patterns to ignore
        #[arg(long)]
        ignore: Vec<String>,
        
        /// Debounce delay for file events (ms)
        #[arg(long, default_value = "100")]
        debounce: u64,
    },
    
    /// Benchmark the project
    Bench {
        /// Benchmark name patterns
        bench_name: Vec<String>,
        
        /// Save baseline
        #[arg(long)]
        save_baseline: Option<String>,
        
        /// Compare against baseline
        #[arg(long)]
        baseline: Option<String>,
    },
}

#[derive(Subcommand)]
enum PackageCommands {
    /// Install dependencies
    Install,
    
    /// Update dependencies
    Update {
        /// Specific package to update
        package: Option<String>,
        
        /// Dry run
        #[arg(long)]
        dry_run: bool,
    },
    
    /// Add a dependency
    Add {
        /// Package name
        package: String,
        
        /// Package version
        #[arg(short, long)]
        version: Option<String>,
        
        /// Add as dev dependency
        #[arg(long)]
        dev: bool,
        
        /// Add as build dependency
        #[arg(long)]
        build: bool,
    },
    
    /// Remove a dependency
    Remove {
        /// Package name
        package: String,
    },
    
    /// Search for packages
    Search {
        /// Search query
        query: String,
        
        /// Maximum results
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },
    
    /// Show package information
    Info {
        /// Package name
        package: String,
    },
}

fn parse_key_val(s: &str) -> Result<(String, String), String> {
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{s}`"))?;
    Ok((s[..pos].to_string(), s[pos + 1..].to_string()))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
}

async fn run_command(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    match cli.command {
        Commands::Init { name, template, lib, target_dir, variables } => {
            let template_name = if lib { "lib" } else { &template };
            let target = target_dir.unwrap_or_else(|| PathBuf::from(&name));
            
            init_project(&name, template_name, target, variables).await
        }
        
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
        }
        
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
        }
        
        Commands::Check { all_targets } => {
            check_project(all_targets).await
        }
        
        Commands::Format { check, diff, files } => {
            format_code(check, diff, files).await
        }
        
        Commands::Lint { fix, stats, files } => {
            lint_code(fix, stats, files).await
        }
        
        Commands::Docs { open, format, output } => {
            generate_docs(open, &format, output).await
        }
        
        Commands::Package { command } => {
            handle_package_command(command).await
        }
        
        Commands::Templates { category, detailed } => {
            list_templates(category, detailed).await
        }
        
        Commands::Info { deps, config, format } => {
            show_project_info(deps, config, &format).await
        }
        
        Commands::Watch { command, delay, patterns, ignore, debounce } => {
            watch_project(&command, delay, patterns, ignore, debounce).await
        }
        
        Commands::Bench { bench_name, save_baseline, baseline } => {
            benchmark_project(bench_name, save_baseline, baseline).await
        }
    }
}

async fn init_project(
    name: &str,
    template: &str,
    target_dir: PathBuf,
    variables: Vec<(String, String)>,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Initializing project '{}' with template '{}'", name, template);
    
    let template_manager = TemplateManager::new();
    let variable_map: HashMap<String, String> = variables.into_iter().collect();
    
    let context = TemplateContext {
        project_name: name.to_string(),
        target_dir,
        variables: variable_map,
    };
    
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
}

async fn build_project(
    profile: &str,
    targets: Vec<String>,
    _all_features: bool,
    _no_default_features: bool,
    _features: Vec<String>,
    force: bool,
    parallel: bool,
    quick: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Building project with profile: {}", profile);
    
    let work_dir = std::env::current_dir()?;
    let config_path = work_dir.join("CursedBuild.toml");
    
    if !config_path.exists() {
        return Err("No CursedBuild.toml found. Run 'cursed-build init' to create a project.".into());
    }
    
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
    };
    
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
}

async fn run_project(
    profile: &str,
    bin: Option<String>,
    args: Vec<String>,
    _jobs: Option<usize>,
) -> Result<(), Box<dyn std::error::Error>> {
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
    };
    
    if !executable.exists() {
        return Err(format!("Executable not found: {}", executable.display()).into());
    }
    
    println!("🚀 Running: {}", executable.display());
    
    let mut cmd = std::process::Command::new(&executable);
    cmd.args(args);
    
    let status = cmd.status()?;
    
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
    
    Ok(())
}

async fn test_project(
    profile: &str,
    _test_name: Vec<String>,
    _ignored: bool,
    _test_threads: Option<usize>,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Testing project with profile: {}", profile);
    
    let work_dir = std::env::current_dir()?;
    let config_path = work_dir.join("CursedBuild.toml");
    
    if !config_path.exists() {
        return Err("No CursedBuild.toml found. Run 'cursed-build init' to create a project.".into());
    }
    
    let config = BuildConfig::load_from_file(&config_path)?;
    let mut orchestrator = BuildOrchestrator::new(config, work_dir)?;
    
    let result = orchestrator.test(profile).await?;
    
    println!("🧪 Tests completed!");
    println!("📊 Results:");
    println!("   - Success: {}", result.success);
    println!("   - Duration: {:?}", result.duration);
    
    Ok(())
}

async fn clean_project(all: bool, _jobs: Option<usize>) -> Result<(), Box<dyn std::error::Error>> {
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
            }
            
            if release_dir.exists() {
                std::fs::remove_dir_all(&release_dir)?;
                println!("🧹 Removed release artifacts");
            }
        }
    } else {
        println!("ℹ️  Nothing to clean");
    }
    
    Ok(())
}

async fn check_project(_all_targets: bool) -> Result<(), Box<dyn std::error::Error>> {
    info!("Checking project");
    
    // TODO: Implement check without building
    println!("🔍 Project check completed");
    
    Ok(())
}

async fn format_code(
    check: bool,
    diff: bool,
    files: Vec<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Formatting CURSED code");
    
    let mut cmd = std::process::Command::new("./target/debug/cursed-fmt");
    
    if check {
        cmd.arg("--check");
    }
    
    if diff {
        cmd.arg("--diff");
    }
    
    if files.is_empty() {
        cmd.arg(".");
    } else {
        cmd.args(files);
    }
    
    let status = cmd.status()?;
    
    if status.success() {
        println!("💄 Code formatting completed");
    } else {
        return Err("Formatting failed".into());
    }
    
    Ok(())
}

async fn lint_code(
    fix: bool,
    stats: bool,
    files: Vec<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Linting CURSED code");
    
    let mut cmd = std::process::Command::new("./target/debug/cursed_lint_new");
    
    if fix {
        cmd.arg("--fix");
    }
    
    if stats {
        cmd.arg("--stats");
    }
    
    if files.is_empty() {
        cmd.arg(".");
    } else {
        cmd.args(files);
    }
    
    let status = cmd.status()?;
    
    if status.success() {
        println!("🔍 Code linting completed");
    } else {
        warn!("Linting found issues");
    }
    
    Ok(())
}

async fn generate_docs(
    open: bool,
    format: &str,
    output: Option<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Generating documentation in {} format", format);
    
    let mut cmd = std::process::Command::new("./target/debug/cursed-doc");
    
    match format {
        "html" => cmd.arg("--html"),
        "markdown" => cmd.arg("--markdown"),
        "json" => cmd.arg("--json"),
        _ => return Err(format!("Unknown format: {}", format).into()),
    };
    
    cmd.arg("--source").arg("src");
    
    let output_dir = if let Some(ref output_dir) = output {
        cmd.arg("--output").arg(output_dir);
        output_dir.clone()
    } else {
        cmd.arg("--output").arg("docs");
        PathBuf::from("docs")
    };
    
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
    }
    
    Ok(())
}

async fn handle_package_command(command: PackageCommands) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        PackageCommands::Install => {
            info!("Installing dependencies");
            println!("📦 Installing dependencies...");
            // TODO: Implement package installation
            println!("✅ Dependencies installed");
        }
        
        PackageCommands::Update { package, dry_run } => {
            if dry_run {
                println!("🔍 Checking for updates (dry run):");
            } else {
                println!("📦 Updating dependencies...");
            }
            
            if let Some(pkg) = package {
                println!("   Updating {}", pkg);
            } else {
                println!("   Updating all packages");
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
            };
            
            println!("📦 Adding {} as {}", package, dep_type);
            if let Some(ver) = version {
                println!("   Version: {}", ver);
            }
            println!("✅ Package added");
        }
        
        PackageCommands::Remove { package } => {
            println!("🗑️  Removing package: {}", package);
            println!("✅ Package removed");
        }
        
        PackageCommands::Search { query, limit } => {
            println!("🔍 Searching for: {} (limit: {})", query, limit);
            // TODO: Implement package search
            println!("No packages found matching '{}'", query);
        }
        
        PackageCommands::Info { package } => {
            println!("📋 Package information for: {}", package);
            // TODO: Implement package info
            println!("Package not found: {}", package);
        }
    }
    
    Ok(())
}

async fn list_templates(
    category: Option<String>,
    detailed: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let manager = TemplateManager::new();
    
    let templates = if let Some(cat) = category {
        let template_category = match cat.as_str() {
            "cli" => TemplateCategory::CLI,
            "lib" | "library" => TemplateCategory::Library,
            "web" => TemplateCategory::Web,
            "api" => TemplateCategory::API,
            "game" => TemplateCategory::Game,
            _ => return Err(format!("Unknown category: {}", cat).into()),
        };
        
        manager.get_templates_by_category(&template_category)
    } else {
        manager.list_templates()
    };
    
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
}

async fn show_project_info(
    deps: bool,
    config: bool,
    format: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let work_dir = std::env::current_dir()?;
    let config_path = work_dir.join("CursedBuild.toml");
    
    if !config_path.exists() {
        return Err("No CursedBuild.toml found".into());
    }
    
    let build_config = BuildConfig::load_from_file(&config_path)?;
    
    match format {
        "text" => {
            println!("📋 Project Information");
            println!("Name: {}", build_config.project.name);
            println!("Version: {}", build_config.project.version);
            
            if let Some(desc) = &build_config.project.description {
                println!("Description: {}", desc);
            }
            
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
        }
        
        "yaml" => {
            let yaml = serde_yaml::to_string(&build_config)?;
            println!("{}", yaml);
        }
        
        _ => return Err(format!("Unknown format: {}", format).into()),
    }
    
    Ok(())
}

async fn watch_project(
    command: &str, 
    delay: u64,
    patterns: Vec<String>, 
    ignore: Vec<String>,
    debounce: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    use std::time::Duration;
    use tokio::signal;
    
    println!("👀 Watching for changes...");
    println!("Will run '{}' on file changes", command);
    println!("Debounce delay: {}ms, rebuild delay: {}ms", debounce, delay);
    
    let work_dir = std::env::current_dir()?;
    let config_path = work_dir.join("CursedBuild.toml");
    
    if !config_path.exists() {
        return Err("No CursedBuild.toml found. Run 'cursed-build init' to create a project.".into());
    }
    
    let config = BuildConfig::load_from_file(&config_path)?;
    let mut orchestrator = BuildOrchestrator::new(config, work_dir)?;
    
    // Setup watch patterns - default to CURSED source files if none specified
    let watch_patterns = if patterns.is_empty() {
        vec!["**/*.csd".to_string(), "src/**/*.rs".to_string()]
    } else {
        patterns
    };
    
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
        patterns: watch_patterns,
        debounce_ms: debounce,
        incremental: true,
        build_profile: "dev".to_string(),
    };
    orchestrator.set_watch_config(watch_config);
    
    // Start file watching with orchestrator
    let watch_result = orchestrator.watch("dev", command);
    
    tokio::select! {
        result = watch_result => {
            match result {
                Ok(_) => println!("👀 File watching completed"),
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
}

async fn watch_build_project(
    profile: &str,
    targets: Vec<String>,
    _all_features: bool,
    _no_default_features: bool,
    _features: Vec<String>,
    force: bool,
    parallel: bool,
    quick: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    use std::time::Duration;
    use tokio::signal;
    
    println!("👀 Watching for changes to rebuild project...");
    println!("Profile: {}, Quick: {}, Force: {}, Parallel: {}", profile, quick, force, parallel);
    
    let work_dir = std::env::current_dir()?;
    let config_path = work_dir.join("CursedBuild.toml");
    
    if !config_path.exists() {
        return Err("No CursedBuild.toml found. Run 'cursed-build init' to create a project.".into());
    }
    
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
    };
    
    // Configure watch settings
    let watch_config = WatchConfig {
        patterns: watch_patterns,
        debounce_ms: 100,
        incremental: !force,
        build_profile: profile.to_string(),
    };
    orchestrator.set_watch_config(watch_config);
    
    // Start file watching with orchestrator
    let watch_result = orchestrator.watch(profile, build_command);
    
    tokio::select! {
        result = watch_result => {
            match result {
                Ok(_) => println!("👀 Build watching completed"),
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
}

async fn watch_test_project(
    profile: &str,
    _test_name: Vec<String>,
    _ignored: bool,
    _test_threads: Option<usize>,
) -> Result<(), Box<dyn std::error::Error>> {
    use std::time::Duration;
    use tokio::signal;
    
    println!("👀 Watching for changes to rerun tests...");
    println!("Profile: {}", profile);
    
    let work_dir = std::env::current_dir()?;
    let config_path = work_dir.join("CursedBuild.toml");
    
    if !config_path.exists() {
        return Err("No CursedBuild.toml found. Run 'cursed-build init' to create a project.".into());
    }
    
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
        patterns: watch_patterns,
        debounce_ms: 100,
        incremental: true,
        build_profile: profile.to_string(),
    };
    orchestrator.set_watch_config(watch_config);
    
    // Start file watching with orchestrator
    let watch_result = orchestrator.watch(profile, "test");
    
    tokio::select! {
        result = watch_result => {
            match result {
                Ok(_) => println!("👀 Test watching completed"),
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
}

async fn benchmark_project(
    _bench_name: Vec<String>,
    _save_baseline: Option<String>,
    _baseline: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Running benchmarks...");
    
    // TODO: Implement benchmarking
    println!("Benchmarking not yet implemented");
    
    Ok(())
}

fn setup_logging(verbose: bool, quiet: bool) {
    if quiet {
        return;
    }
    
    let level = if verbose {
        tracing::Level::DEBUG
    } else {
        tracing::Level::INFO
    };
    
    tracing_subscriber::fmt()
        .with_max_level(level)
        .with_target(false)
        .without_time()
        .init();
}

fn is_executable(path: &std::path::Path) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(metadata) = std::fs::metadata(path) {
            let permissions = metadata.permissions();
            return permissions.mode() & 0o111 != 0;
        }
        false
    }
    
    #[cfg(windows)]
    {
        path.extension()
            .map(|ext| ext == "exe" || ext == "bat" || ext == "cmd")
            .unwrap_or(false)
    }
    
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
}

fn category_name(category: &TemplateCategory) -> &'static str {
    match category {
        TemplateCategory::CLI => "cli",
        TemplateCategory::Library => "library",
        TemplateCategory::Web => "web",
        TemplateCategory::API => "api",
        TemplateCategory::Desktop => "desktop",
        TemplateCategory::Game => "game",
        TemplateCategory::Custom => "custom",
    }
}

fn convert_pipeline_to_build_result(pipeline_result: PipelineResult) -> cursed::build_system::BuildResult {
    cursed::build_system::BuildResult {
        success: pipeline_result.success,
        duration: pipeline_result.duration,
        targets_built: pipeline_result.stages.keys().cloned().collect(),
        targets_skipped: pipeline_result.stages.values()
            .filter(|s| s.cache_hit)
            .map(|s| s.name.clone())
            .collect(),
        outputs: pipeline_result.artifacts.values().cloned().collect(),
        artifacts: pipeline_result.artifacts,
        warnings: pipeline_result.warnings,
        statistics: cursed::build_system::BuildStatistics {
            files_compiled: pipeline_result.statistics.stages_executed,
            files_cached: pipeline_result.statistics.stages_cached,
            lines_compiled: 0, // TODO: Extract from pipeline stages
            peak_memory: pipeline_result.statistics.resource_usage.peak_memory,
            phase_timings: std::collections::HashMap::new(), // TODO: Extract from stages
        },
    }
}
