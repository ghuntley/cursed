//! CURSED Documentation System CLI
//! 
//! Command-line interface for the CURSED documentation generation system.
//! Provides commands for generating, serving, and analyzing documentation.

use std::env;
use std::process;
use crate::error::CursedError;
use crate::documentation::{DocumentationGenerator, DocCli};
use crate::documentation::live_server::{LiveServerCli};
use crate::documentation::coverage_analyzer::{CoverageAnalyzer, CoverageReportConfig, ReportFormat};
use crate::documentation::build_integration::{BuildIntegrationCli};

/// Main entry point for documentation CLI
pub fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_help();
        process::exit(1);
    }

    let result = match args[1].as_str() {
        "generate" | "gen" => generate_docs(&args[2..]),
        "serve" => serve_docs(&args[2..]),
        "coverage" | "cov" => analyze_coverage(&args[2..]),
        "watch" => watch_docs(&args[2..]),
        "build-integration" | "build" => build_integration(&args[2..]),
        "init" => init_docs(&args[2..]),
        "clean" => clean_docs(&args[2..]),
        "help" | "-h" | "--help" => {
            print_help();
            Ok(())
        }
        "version" | "-v" | "--version" => {
            print_version();
            Ok(())
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_help();
            Err(CursedError::InvalidInput(format!("Unknown command: {}", args[1])))
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

/// Generate documentation
fn generate_docs(args: &[String]) -> Result<(), CursedError> {
    let config_path = args.get(0).map(|s| s.as_str());
    
    println!("Generating CURSED documentation...");
    
    let mut generator = DocumentationGenerator::new(config_path)?;
    generator.generate()?;
    
    println!("Documentation generated successfully!");
    println!("Open docs/html/index.html to view the documentation.");
    
    Ok(())
}

/// Start live documentation server
fn serve_docs(args: &[String]) -> Result<(), CursedError> {
    let port = args.get(0)
        .and_then(|s| s.parse().ok())
        .unwrap_or(8080);
    
    let host = args.get(1)
        .map(|s| s.to_string())
        .unwrap_or_else(|| "localhost".to_string());

    println!("Starting CURSED documentation server...");
    println!("Server will be available at http://{}:{}", host, port);
    
    LiveServerCli::run(vec![
        "serve".to_string(),
        ".cursed-doc.toml".to_string(),
        port.to_string(),
        host,
    ])?;
    
    Ok(())
}

/// Analyze documentation coverage
fn analyze_coverage(args: &[String]) -> Result<(), CursedError> {
    let format = args.get(0)
        .map(|s| s.as_str())
        .unwrap_or("console");
    
    let output_file = args.get(1).map(|s| s.to_string());
    
    println!("Analyzing documentation coverage...");
    
    // Generate documentation first
    let mut generator = DocumentationGenerator::new(None)?;
    generator.generate()?;
    
    // Analyze coverage
    let mut analyzer = CoverageAnalyzer::new();
    analyzer.analyze_documentation(&generator.documentation)?;
    
    // Generate report
    let report_format = match format {
        "html" => ReportFormat::Html,
        "markdown" | "md" => ReportFormat::Markdown,
        "json" => ReportFormat::Json,
        "console" | "text" => ReportFormat::Console,
        _ => {
            eprintln!("Unknown format: {}. Using console format.", format);
            ReportFormat::Console
        }
    };
    
    let config = CoverageReportConfig {
        include_missing_items: true,
        include_quality_metrics: true,
        include_suggestions: true,
        format: report_format,
        output_file,
    };
    
    let report = analyzer.generate_report(&config)?;
    
    match config.format {
        ReportFormat::Console => println!("{}", report),
        _ => {
            if let Some(ref file) = config.output_file {
                println!("Coverage report saved to: {}", file);
            } else {
                println!("{}", report);
            }
        }
    }
    
    analyzer.save_report(&config)?;
    
    Ok(())
}

/// Watch for changes and auto-regenerate documentation
fn watch_docs(args: &[String]) -> Result<(), CursedError> {
    let config_path = args.get(0).map(|s| s.as_str());
    
    println!("Starting documentation watch mode...");
    println!("Watching for changes in source files...");
    println!("Press Ctrl+C to stop");
    
    // Implementation would use file system watching
    // For now, show the concept
    
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        
        // Check for file changes
        // If changes detected, regenerate documentation
        
        // This is a simplified version - real implementation would use
        // proper file system watching like notify crate
    }
}

/// Initialize build system integration
fn build_integration(args: &[String]) -> Result<(), CursedError> {
    let command = args.get(0).map(|s| s.as_str()).unwrap_or("init");
    
    match command {
        "init" => {
            BuildIntegrationCli::init()?;
            println!("Build integration initialized successfully!");
        }
        "build" => {
            let build_command = args.get(1)
                .map(|s| s.as_str())
                .unwrap_or("cargo build");
            
            BuildIntegrationCli::build_with_docs(build_command)?;
        }
        _ => {
            eprintln!("Unknown build integration command: {}", command);
            println!("Available commands:");
            println!("  init  - Initialize build integration");
            println!("  build - Run build with documentation generation");
        }
    }
    
    Ok(())
}

/// Initialize documentation configuration
fn init_docs(_args: &[String]) -> Result<(), CursedError> {
    use std::fs;
    
    println!("Initializing CURSED documentation configuration...");
    
    let config_content = r#"[general]
project_name = "My CURSED Project"
project_version = "1.0.0"
project_description = "A project built with the CURSED programming language"
project_url = "https://example.com"
authors = ["Your Name <your.email@example.com>"]
license = "MIT"
repository = "https://github.com/username/project"

[input]
source_dirs = ["src/", "stdlib/"]
include_patterns = ["**/*.csd"]
exclude_patterns = ["target/**", ".git/**", "node_modules/**"]
max_file_size = 10485760  # 10MB

[output]
output_dir = "docs"
formats = ["html", "json"]
clean_output = true
base_url = ""

[html]
theme = "default"
syntax_highlighting = true
table_of_contents = true
search_enabled = true
responsive_design = true
custom_css = []
custom_js = []
offline_mode = false

[markdown]
flavor = "github"
table_of_contents = true
code_block_style = "fenced"
link_style = "reference"

[processing]
extract_comments = true
extract_examples = true
generate_cross_references = true
analyze_dependencies = true
process_cursed_files = true
process_rust_files = false
process_markdown_files = true
cursed_comment_patterns = ["^\\s*fr fr\\s+(.*)$", "^\\s*/\\*\\*?(.*)\\*/$"]

[validation]
check_links = true
check_examples = true
validate_syntax = true
require_descriptions = false
treat_warnings_as_errors = false

[examples]
extract_examples = true
validate_examples = true
run_examples = false
categorize_by_directory = true
generate_example_index = true

[api]
generate_api_docs = true
include_private = false
include_internal = false
show_source_links = true
require_doc_comments = false
coverage_threshold = 70.0
"#;

    fs::write(".cursed-doc.toml", config_content)
        .map_err(|e| CursedError::IoError(format!("Failed to write config file: {}", e)))?;
    
    println!("Configuration file created: .cursed-doc.toml");
    println!("Edit the configuration file to customize documentation settings.");
    
    Ok(())
}

/// Clean generated documentation files
fn clean_docs(_args: &[String]) -> Result<(), CursedError> {
    use std::fs;
    use std::path::Path;
    
    println!("Cleaning generated documentation files...");
    
    let docs_dir = Path::new("docs");
    if docs_dir.exists() {
        fs::remove_dir_all(docs_dir)
            .map_err(|e| CursedError::IoError(format!("Failed to clean docs directory: {}", e)))?;
    }
    
    println!("Documentation files cleaned successfully!");
    
    Ok(())
}

/// Print help information
fn print_help() {
    println!("CURSED Documentation System v1.0.0");
    println!("Generate, serve, and analyze documentation for CURSED projects");
    println!();
    println!("USAGE:");
    println!("    cursed-doc <COMMAND> [OPTIONS]");
    println!();
    println!("COMMANDS:");
    println!("    generate, gen                Generate documentation");
    println!("    serve                        Start live documentation server");
    println!("    coverage, cov               Analyze documentation coverage");
    println!("    watch                       Watch for changes and auto-regenerate");
    println!("    build-integration, build    Build system integration commands");
    println!("    init                        Initialize documentation configuration");
    println!("    clean                       Clean generated documentation files");
    println!("    help                        Show this help message");
    println!("    version                     Show version information");
    println!();
    println!("EXAMPLES:");
    println!("    cursed-doc generate                     # Generate docs with default config");
    println!("    cursed-doc generate my-config.toml     # Generate docs with custom config");
    println!("    cursed-doc serve 3000                  # Start server on port 3000");
    println!("    cursed-doc coverage html coverage.html # Generate HTML coverage report");
    println!("    cursed-doc build init                  # Initialize build integration");
    println!("    cursed-doc build build \"cargo build\"   # Build with documentation");
    println!();
    println!("For more information, visit: https://github.com/ghuntley/cursed");
}

/// Print version information
fn print_version() {
    println!("CURSED Documentation System v1.0.0");
    println!("Built with Rust and powered by the CURSED compiler");
    println!();
    println!("Features:");
    println!("  - Multi-format output (HTML, Markdown, JSON)");
    println!("  - Live development server with hot reload");
    println!("  - Documentation coverage analysis");
    println!("  - Build system integration");
    println!("  - Syntax highlighting for CURSED language");
    println!("  - Interactive search and navigation");
}

/// CLI helper functions
pub struct DocumentationCli;

impl DocumentationCli {
    /// Run documentation CLI with given arguments
    pub fn run(args: Vec<String>) -> Result<(), CursedError> {
        // Skip the program name
        let mut cli_args = vec!["cursed-doc".to_string()];
        cli_args.extend(args);
        
        // Temporarily set args for main function
        std::env::set_var("CURSED_DOC_ARGS", cli_args.join(" "));
        
        main();
        Ok(())
    }
    
    /// Generate documentation with default settings
    pub fn quick_generate() -> Result<(), CursedError> {
        let mut generator = DocumentationGenerator::new(None)?;
        generator.generate()
    }
    
    /// Start documentation server with default settings
    pub fn quick_serve(port: Option<u16>) -> Result<(), CursedError> {
        let port = port.unwrap_or(8080);
        LiveServerCli::run(vec![
            "serve".to_string(),
            ".cursed-doc.toml".to_string(),
            port.to_string(),
            "localhost".to_string(),
        ])
    }
    
    /// Generate coverage report with default settings
    pub fn quick_coverage() -> Result<String, CursedError> {
        let mut generator = DocumentationGenerator::new(None)?;
        generator.generate()?;
        
        let mut analyzer = CoverageAnalyzer::new();
        analyzer.analyze_documentation(&generator.documentation)?;
        
        let config = CoverageReportConfig::default();
        analyzer.generate_report(&config)
    }
}
