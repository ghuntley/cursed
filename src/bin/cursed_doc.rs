//! CURSED documentation generator CLI
//!
//! Comprehensive command-line interface for generating documentation in multiple formats
//! from CURSED source files with validation, server capabilities, and configuration support.

use clap::{Arg, ArgAction, Command, value_parser};
use cursed::docs::{
    DocConfig, DocumentationGenerator, DocumentationGenerationResult, DocResult, DocError,
    DocumentationValidationResult,
    config::{ConfigLoader, DocConfigFile},
    server::DocServer,
};
use serde_json;
use serde_yaml;
use toml;
use std::path::PathBuf;
use std::process;
use std::time::Duration;
use tracing::{debug, error, info, warn, Level};
use tracing_subscriber::{fmt, EnvFilter};

fn main() {
    // Initialize tracing
    init_tracing();

    // Parse command line arguments
    let matches = create_cli().get_matches();
    
    // Run the documentation generator
    if let Err(e) = run_doc_generator(&matches) {
        error!("Documentation generation failed: {}", e);
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

/// Initialize tracing subscriber
fn init_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info"))
        )
        .with_target(false)
        .init();
}

/// Create the CLI application
fn create_cli() -> Command {
    Command::new("cursed-doc")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Generate documentation for CURSED source files in multiple formats")
        .long_about("CURSED Documentation Generator

Generates comprehensive documentation from CURSED source files in multiple output formats:
- HTML: Professional web documentation with navigation and search
- Markdown: Structured text documentation for repositories
- JSON: Machine-readable documentation data

Features include function signatures, struct definitions, interface definitions,
documentation comments, cross-references, search functionality, validation,
and local development server with live reload.")
        .arg(
            Arg::new("source")
                .short('s')
                .long("source")
                .value_name("DIR")
                .help("Source directory to scan for CURSED files")
                .action(ArgAction::Append)
                .default_value("src")
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("DIR")
                .help("Output directory for generated documentation")
                .default_value("docs/html")
        )
        .arg(
            Arg::new("package-name")
                .long("package-name")
                .value_name("NAME")
                .help("Package name for documentation")
                .default_value("CURSED Package")
        )
        .arg(
            Arg::new("package-version")
                .long("package-version")
                .value_name("VERSION")
                .help("Package version")
                .default_value("1.0.0")
        )
        .arg(
            Arg::new("description")
                .long("description")
                .value_name("TEXT")
                .help("Package description")
        )
        .arg(
            Arg::new("include-private")
                .long("include-private")
                .help("Include private items in documentation")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("no-search")
                .long("no-search")
                .help("Disable search functionality")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("sitemap")
                .long("sitemap")
                .value_name("URL")
                .help("Generate sitemap with base URL")
        )
        .arg(
            Arg::new("custom-css")
                .long("custom-css")
                .value_name("FILE")
                .help("Custom CSS file to include")
        )
        .arg(
            Arg::new("custom-js")
                .long("custom-js")
                .value_name("FILE")
                .help("Custom JavaScript file to include")
        )
        .arg(
            Arg::new("max-depth")
                .long("max-depth")
                .value_name("NUM")
                .help("Maximum directory scanning depth")
                .value_parser(clap::value_parser!(usize))
        )
        .arg(
            Arg::new("exclude")
                .long("exclude")
                .value_name("PATTERN")
                .help("Exclude files matching pattern")
                .action(ArgAction::Append)
        )
        .arg(
            Arg::new("clean")
                .long("clean")
                .help("Clean output directory before generation")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose output")
                .action(ArgAction::Count)
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .help("Suppress non-error output")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("serve")
                .long("serve")
                .help("Serve documentation locally and watch for changes")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("port")
                .long("port")
                .value_name("PORT")
                .help("Port for documentation server")
                .value_parser(value_parser!(u16))
                .default_value("8080")
        )
        .arg(
            Arg::new("host")
                .long("host")
                .value_name("HOST")
                .help("Host for documentation server")
                .default_value("127.0.0.1")
        )
        .arg(
            Arg::new("watch")
                .long("watch")
                .help("Watch for file changes and regenerate documentation")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("html")
                .long("html")
                .help("Generate HTML documentation")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("markdown")
                .long("markdown")
                .help("Generate Markdown documentation")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("json")
                .long("json")
                .help("Generate JSON documentation data")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("check")
                .long("check")
                .help("Validate documentation completeness without generating")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("output-format")
                .long("output-format")
                .value_name("FORMAT")
                .help("Output format for documentation (fallback if no specific format flag)")
                .value_parser(["html", "json", "markdown"])
                .default_value("html")
        )
        .arg(
            Arg::new("config-file")
                .long("config-file")
                .value_name("FILE")
                .help("Configuration file (TOML, JSON, or YAML)")
        )
        .arg(
            Arg::new("generate-config")
                .long("generate-config")
                .value_name("FILE")
                .help("Generate default configuration file")
        )
        .arg(
            Arg::new("jobs")
                .long("jobs")
                .short('j')
                .value_name("NUM")
                .help("Number of parallel jobs (0 = auto)")
                .value_parser(value_parser!(usize))
                .default_value("0")
        )
        .arg(
            Arg::new("stats")
                .long("stats")
                .help("Show detailed generation statistics")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("open")
                .long("open")
                .help("Open documentation in browser after generation")
                .action(ArgAction::SetTrue)
        )
}

/// Run the documentation generator based on CLI arguments
fn run_doc_generator(matches: &clap::ArgMatches) -> DocResult<()> {
    // Handle config generation
    if let Some(config_path) = matches.get_one::<String>("generate-config") {
        return generate_default_config(config_path);
    }

    // Determine output formats
    let formats = determine_output_formats(matches);
    
    // Handle check mode
    if matches.get_flag("check") {
        return run_documentation_check(matches);
    }

    // Load configuration
    let mut config = load_configuration(matches)?;
    // Configure logging level based on verbosity
    if matches.get_flag("quiet") {
        // Already configured for errors only
    } else {
        match matches.get_count("verbose") {
            0 => { /* default info level */ }
            1 => {
                // Enable debug level
                tracing_subscriber::fmt()
                    .with_env_filter(EnvFilter::new("debug"))
                    .with_target(false)
                    .try_init()
                    .ok();
            }
            _ => {
                // Enable trace level
                tracing_subscriber::fmt()
                    .with_env_filter(EnvFilter::new("trace"))
                    .with_target(false)
                    .try_init()
                    .ok();
            }
        }
    }

    // Extract configuration from CLI arguments
    let source_dirs: Vec<PathBuf> = matches
        .get_many::<String>("source")
        .unwrap_or_default()
        .map(PathBuf::from)
        .collect();

    let output_dir = PathBuf::from(matches.get_one::<String>("output").unwrap());
    let package_name = matches.get_one::<String>("package-name").unwrap().clone();
    let package_version = matches.get_one::<String>("package-version").unwrap().clone();

    // Build configuration
    let mut config = DocConfig::new(package_name, package_version)
        .with_source_dirs(source_dirs)
        .with_output_dir(output_dir)
        .include_private(matches.get_flag("include-private"))
        .with_search(!matches.get_flag("no-search"));

    // Set optional configuration
    if let Some(description) = matches.get_one::<String>("description") {
        config = config.with_description(description.clone());
    }

    if let Some(base_url) = matches.get_one::<String>("sitemap") {
        config = config.with_sitemap(base_url.clone());
    }

    if let Some(css_path) = matches.get_one::<String>("custom-css") {
        config = config.with_custom_css(PathBuf::from(css_path));
    }

    if let Some(js_path) = matches.get_one::<String>("custom-js") {
        config = config.with_custom_js(PathBuf::from(js_path));
    }

    if let Some(max_depth) = matches.get_one::<usize>("max-depth") {
        config = config.with_max_depth(*max_depth);
    }

    if let Some(exclude_patterns) = matches.get_many::<String>("exclude") {
        let patterns: Vec<String> = exclude_patterns.cloned().collect();
        config = config.with_exclude_patterns(patterns);
    }

    // Create documentation generator
    let mut generator = DocumentationGenerator::new(config)?;

    // Clean output directory if requested
    if matches.get_flag("clean") {
        info!("Cleaning output directory");
        generator.clean_output()?;
    }

    // Generate documentation
    info!("Starting documentation generation");
    let start_time = std::time::Instant::now();
    
    let result = generator.generate()?;
    
    let duration = start_time.elapsed();
    
    // Handle serve mode
    if matches.get_flag("serve") {
        let port = *matches.get_one::<u16>("port").unwrap();
        let host = matches.get_one::<String>("host").unwrap();
        let watch = matches.get_flag("watch");
        
        info!("Starting documentation server on {}:{}", host, port);
        
        let mut server = DocServer::new(
            host.clone(),
            port,
            generator.config().output_dir.clone(),
        )?;
        
        if watch {
            server.enable_watch(generator.config().source_dirs.clone())?;
        }
        
        return server.serve();
    }

    // Print results
    if !matches.get_flag("quiet") {
        print_generation_results(&result, duration, matches)?;
    }

    // Show statistics if requested
    if matches.get_flag("stats") {
        print_detailed_statistics(&result)?;
    }

    // Open browser if requested
    if matches.get_flag("open") && !matches.get_flag("serve") {
        if let Some(index_path) = result.output_files.iter()
            .find(|p| p.file_name().and_then(|n| n.to_str()) == Some("index.html")) {
            open_browser(&format!("file://{}", index_path.display()))?;
        }
    }

    Ok(())
}

/// Determine output formats based on CLI flags
fn determine_output_formats(matches: &clap::ArgMatches) -> Vec<String> {
    let mut formats = Vec::new();
    
    if matches.get_flag("html") {
        formats.push("html".to_string());
    }
    if matches.get_flag("markdown") {
        formats.push("markdown".to_string());
    }
    if matches.get_flag("json") {
        formats.push("json".to_string());
    }
    
    // If no specific format flags are set, use the default format
    if formats.is_empty() {
        let default_format = matches.get_one::<String>("output-format").unwrap();
        formats.push(default_format.clone());
    }
    
    formats
}

/// Run documentation completeness check
fn run_documentation_check(matches: &clap::ArgMatches) -> DocResult<()> {
    info!("Running documentation completeness check");
    
    let config = load_configuration(matches)?;
    let generator = DocumentationGenerator::new(config)?;
    
    // Perform validation check
    let validation_result = generator.validate_documentation()?;
    
    if !matches.get_flag("quiet") {
        print_validation_results(&validation_result, matches)?;
    }
    
    if validation_result.has_errors() {
        process::exit(1);
    }
    
    Ok(())
}

/// Print validation results
fn print_validation_results(
    result: &DocumentationValidationResult,
    matches: &clap::ArgMatches,
) -> DocResult<()> {
    let output_format = matches.get_one::<String>("output-format").unwrap();
    
    match output_format.as_str() {
        "json" => {
            let output = serde_json::json!({
                "status": if result.has_errors() { "failed" } else { "passed" },
                "errors": result.errors,
                "warnings": result.warnings,
                "missing_docs": result.missing_documentation,
                "summary": result.summary()
            });
            println!("{}", serde_json::to_string_pretty(&output).unwrap());
        }
        "markdown" => {
            println!("# CURSED Documentation Validation\n");
            println!("**Status:** {}\n", if result.has_errors() { "❌ Failed" } else { "✅ Passed" });
            
            if !result.errors.is_empty() {
                println!("## Errors\n");
                for error in &result.errors {
                    println!("- {}", error);
                }
                println!();
            }
            
            if !result.warnings.is_empty() {
                println!("## Warnings\n");
                for warning in &result.warnings {
                    println!("- {}", warning);
                }
                println!();
            }
            
            if !result.missing_documentation.is_empty() {
                println!("## Missing Documentation\n");
                for missing in &result.missing_documentation {
                    println!("- {}", missing);
                }
                println!();
            }
        }
        _ => {
            println!("\n{}", "=".repeat(60));
            println!("CURSED Documentation Validation");
            println!("{}", "=".repeat(60));
            
            if result.has_errors() {
                println!("❌ Validation FAILED");
                println!("\nErrors found:");
                for error in &result.errors {
                    println!("  ❌ {}", error);
                }
            } else {
                println!("✅ Validation PASSED");
            }
            
            if !result.warnings.is_empty() {
                println!("\nWarnings:");
                for warning in &result.warnings {
                    println!("  ⚠️  {}", warning);
                }
            }
            
            if !result.missing_documentation.is_empty() {
                println!("\nMissing documentation:");
                for missing in &result.missing_documentation {
                    println!("  📝 {}", missing);
                }
            }
            
            println!("\n{}", result.summary());
        }
    }
    
    Ok(())
}

/// Load configuration from file and CLI arguments
fn load_configuration(matches: &clap::ArgMatches) -> DocResult<DocConfig> {
    let mut config = if let Some(config_file) = matches.get_one::<String>("config-file") {
        info!("Loading configuration from: {}", config_file);
        let loader = ConfigLoader::new();
        let file_config = loader.load_from_file(config_file)?;
        DocConfig::from_file_config(file_config)
    } else {
        // Check for default config files
        let loader = ConfigLoader::new();
        if let Ok(file_config) = loader.load_default() {
            info!("Using default configuration file");
            DocConfig::from_file_config(file_config)
        } else {
            // Use CLI arguments to build config
            build_config_from_cli(matches)?
        }
    };

    // Apply CLI overrides
    apply_cli_overrides(&mut config, matches)?;
    
    Ok(config)
}

/// Build configuration from CLI arguments
fn build_config_from_cli(matches: &clap::ArgMatches) -> DocResult<DocConfig> {
    let source_dirs: Vec<PathBuf> = matches
        .get_many::<String>("source")
        .unwrap_or_default()
        .map(PathBuf::from)
        .collect();

    let output_dir = PathBuf::from(matches.get_one::<String>("output").unwrap());
    let package_name = matches.get_one::<String>("package-name").unwrap().clone();
    let package_version = matches.get_one::<String>("package-version").unwrap().clone();

    let mut config = DocConfig::new(package_name, package_version)
        .with_source_dirs(source_dirs)
        .with_output_dir(output_dir)
        .include_private(matches.get_flag("include-private"))
        .with_search(!matches.get_flag("no-search"));

    // Set optional configuration
    if let Some(description) = matches.get_one::<String>("description") {
        config = config.with_description(description.clone());
    }

    if let Some(base_url) = matches.get_one::<String>("sitemap") {
        config = config.with_sitemap(base_url.clone());
    }

    if let Some(css_path) = matches.get_one::<String>("custom-css") {
        config = config.with_custom_css(PathBuf::from(css_path));
    }

    if let Some(js_path) = matches.get_one::<String>("custom-js") {
        config = config.with_custom_js(PathBuf::from(js_path));
    }

    if let Some(max_depth) = matches.get_one::<usize>("max-depth") {
        config = config.with_max_depth(*max_depth);
    }

    if let Some(exclude_patterns) = matches.get_many::<String>("exclude") {
        let patterns: Vec<String> = exclude_patterns.cloned().collect();
        config = config.with_exclude_patterns(patterns);
    }

    let jobs = *matches.get_one::<usize>("jobs").unwrap();
    if jobs > 0 {
        config = config.with_parallel_jobs(jobs);
    }

    Ok(config)
}

/// Apply CLI argument overrides to configuration
fn apply_cli_overrides(config: &mut DocConfig, matches: &clap::ArgMatches) -> DocResult<()> {
    // CLI arguments always override config file settings
    if matches.contains_id("source") {
        let source_dirs: Vec<PathBuf> = matches
            .get_many::<String>("source")
            .unwrap()
            .map(PathBuf::from)
            .collect();
        *config = config.clone().with_source_dirs(source_dirs);
    }

    if matches.contains_id("output") {
        let output_dir = PathBuf::from(matches.get_one::<String>("output").unwrap());
        *config = config.clone().with_output_dir(output_dir);
    }

    if matches.get_flag("include-private") {
        *config = config.clone().include_private(true);
    }

    if matches.get_flag("no-search") {
        *config = config.clone().with_search(false);
    }

    Ok(())
}

/// Generate default configuration file
fn generate_default_config(output_path: &str) -> DocResult<()> {
    let config = DocConfigFile::default();
    let extension = std::path::Path::new(output_path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("toml");

    let content = match extension {
        "json" => serde_json::to_string_pretty(&config)
            .map_err(|e| DocError::IoError(format!("Failed to serialize JSON: {}", e)))?,
        "yaml" | "yml" => serde_yaml::to_string(&config)
            .map_err(|e| DocError::IoError(format!("Failed to serialize YAML: {}", e)))?,
        _ => toml::to_string_pretty(&config)
            .map_err(|e| DocError::IoError(format!("Failed to serialize TOML: {}", e)))?,
    };

    std::fs::write(output_path, content)
        .map_err(|e| DocError::IoError(format!("Failed to write config file: {}", e)))?;

    println!("Generated default configuration at: {}", output_path);
    println!("Edit this file to customize documentation generation settings.");

    Ok(())
}

/// Print generation results
fn print_generation_results(
    result: &DocumentationGenerationResult,
    duration: Duration,
    matches: &clap::ArgMatches,
) -> DocResult<()> {
    let output_format = matches.get_one::<String>("output-format").unwrap();

    match output_format.as_str() {
        "json" => {
            let output = serde_json::json!({
                "status": "success",
                "duration_ms": duration.as_millis(),
                "summary": result.summary(),
                "output_files": result.output_files.iter()
                    .map(|p| p.to_string_lossy())
                    .collect::<Vec<_>>()
            });
            println!("{}", serde_json::to_string_pretty(&output).unwrap());
        }
        "markdown" => {
            println!("# CURSED Documentation Generation Complete\n");
            println!("**Status:** ✅ Success\n");
            println!("**Duration:** {:?}\n", duration);
            println!("## Summary\n");
            println!("{}\n", result.summary());
            println!("## Output Files\n");
            for file in &result.output_files {
                println!("- `{}`", file.display());
            }
        }
        _ => {
            println!("\n{}", "=".repeat(60));
            println!("CURSED Documentation Generation Complete!");
            println!("{}", "=".repeat(60));
            println!();
            println!("{}", result.summary());
            println!();
            println!("📁 Output files:");
            for file in &result.output_files {
                println!("   {}", file.display());
            }
            println!();
            println!("⏱️  Total time: {:?}", duration);
            println!("🎉 Documentation successfully generated!");

            // Show where to view the documentation
            let index_path = result.output_files.iter()
                .find(|p| p.file_name().and_then(|n| n.to_str()) == Some("index.html"));

            if let Some(index) = index_path {
                println!();
                println!("📖 View documentation at: file://{}", index.display());
            }
        }
    }

    Ok(())
}

/// Print detailed statistics
fn print_detailed_statistics(result: &DocumentationGenerationResult) -> DocResult<()> {
    println!("\n{}", "=".repeat(50));
    println!("Detailed Generation Statistics");
    println!("{}", "=".repeat(50));
    
    // This would display detailed stats from the result
    // Implementation depends on the DocumentationGenerationResult structure
    println!("Statistics feature coming soon...");
    
    Ok(())
}

/// Open URL in default browser
fn open_browser(url: &str) -> DocResult<()> {
    info!("Opening browser to: {}", url);
    
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(url)
            .spawn()
            .map_err(|e| DocError::IoError(format!("Failed to open browser: {}", e)))?;
    }
    
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(url)
            .spawn()
            .map_err(|e| DocError::IoError(format!("Failed to open browser: {}", e)))?;
    }
    
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(&["/C", "start", url])
            .spawn()
            .map_err(|e| DocError::IoError(format!("Failed to open browser: {}", e)))?;
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_cli_creation() {
        let cli = create_cli();
        assert_eq!(cli.get_name(), "cursed-doc");
    }

    #[test]
    fn test_default_arguments() {
        let cli = create_cli();
        let matches = cli.try_get_matches_from(&["cursed-doc"]).unwrap();
        
        assert_eq!(matches.get_one::<String>("output").unwrap(), "docs/html");
        assert_eq!(matches.get_one::<String>("package-name").unwrap(), "CURSED Package");
        assert_eq!(matches.get_one::<String>("package-version").unwrap(), "1.0.0");
        assert!(!matches.get_flag("include-private"));
        assert!(!matches.get_flag("no-search"));
    }

    #[test]
    fn test_verbose_flag() {
        let cli = create_cli();
        let matches = cli.try_get_matches_from(&["cursed-doc", "-vv"]).unwrap();
        
        assert_eq!(matches.get_count("verbose"), 2);
    }

    #[test]
    fn test_custom_config() {
        let cli = create_cli();
        let matches = cli.try_get_matches_from(&[
            "cursed-doc",
            "--source", "lib",
            "--source", "examples", 
            "--output", "generated_docs",
            "--package-name", "My Package",
            "--package-version", "2.0.0",
            "--description", "A test package",
            "--include-private",
            "--max-depth", "5",
            "--exclude", "test",
            "--exclude", "example"
        ]).unwrap();
        
        let sources: Vec<&String> = matches.get_many("source").unwrap().collect();
        assert_eq!(sources.len(), 2);
        assert!(sources.contains(&&"lib".to_string()));
        assert!(sources.contains(&&"examples".to_string()));
        
        assert_eq!(matches.get_one::<String>("output").unwrap(), "generated_docs");
        assert_eq!(matches.get_one::<String>("package-name").unwrap(), "My Package");
        assert_eq!(matches.get_one::<String>("package-version").unwrap(), "2.0.0");
        assert_eq!(matches.get_one::<String>("description").unwrap(), "A test package");
        assert!(matches.get_flag("include-private"));
        assert_eq!(matches.get_one::<usize>("max-depth").unwrap(), &5);
        
        let excludes: Vec<&String> = matches.get_many("exclude").unwrap().collect();
        assert_eq!(excludes.len(), 2);
        assert!(excludes.contains(&&"test".to_string()));
        assert!(excludes.contains(&&"example".to_string()));
    }
}
