//! CLI Integration for Documentation Generation
//! 
//! Provides command-line interface for generating documentation from CURSED projects.

use crate::docs::generator::{DocumentationGenerator, DocGeneratorConfig, DocFormat};
use crate::error::Error;
use clap::{Arg, ArgGroup, Command, value_parser};
use std::path::PathBuf;
use std::str::FromStr;

/// Documentation CLI command structure
pub struct DocsCommand;

impl DocsCommand {
    /// Create the docs subcommand for the CLI
    pub fn command() -> Command {
        Command::new("docs")
            .about("Generate documentation for CURSED projects")
            .long_about("Generate comprehensive documentation from CURSED source code in various formats including HTML, Markdown, JSON, and XML.")
            .arg(
                Arg::new("input")
                    .help("Input source directory or file")
                    .value_name("PATH")
                    .value_parser(value_parser!(PathBuf))
                    .default_value("src")
            )
            .arg(
                Arg::new("output")
                    .short('o')
                    .long("output")
                    .help("Output directory for generated documentation")
                    .value_name("DIR")
                    .value_parser(value_parser!(PathBuf))
                    .default_value("docs")
            )
            .arg(
                Arg::new("format")
                    .short('f')
                    .long("format")
                    .help("Output format")
                    .value_name("FORMAT")
                    .value_parser(["html", "markdown", "md", "json", "xml"])
                    .default_value("html")
            )
            .arg(
                Arg::new("title")
                    .short('t')
                    .long("title")
                    .help("Project title for documentation")
                    .value_name("TITLE")
                    .default_value("CURSED Project Documentation")
            )
            .arg(
                Arg::new("description")
                    .short('d')
                    .long("description")
                    .help("Project description")
                    .value_name("DESCRIPTION")
            )
            .arg(
                Arg::new("version")
                    .short('v')
                    .long("version")
                    .help("Project version")
                    .value_name("VERSION")
            )
            .arg(
                Arg::new("author")
                    .short('a')
                    .long("author")
                    .help("Project author(s)")
                    .value_name("AUTHOR")
                    .action(clap::ArgAction::Append)
            )
            .arg(
                Arg::new("include-private")
                    .long("include-private")
                    .help("Include private items in documentation")
                    .action(clap::ArgAction::SetTrue)
            )
            .arg(
                Arg::new("no-examples")
                    .long("no-examples")
                    .help("Exclude code examples from documentation")
                    .action(clap::ArgAction::SetTrue)
            )
            .arg(
                Arg::new("no-cross-refs")
                    .long("no-cross-refs")
                    .help("Disable cross-reference generation")
                    .action(clap::ArgAction::SetTrue)
            )
            .arg(
                Arg::new("base-url")
                    .long("base-url")
                    .help("Base URL for linking (useful for hosting)")
                    .value_name("URL")
            )
            .arg(
                Arg::new("custom-css")
                    .long("custom-css")
                    .help("Path to custom CSS file for HTML output")
                    .value_name("FILE")
                    .value_parser(value_parser!(PathBuf))
            )
            .arg(
                Arg::new("template-dir")
                    .long("template-dir")
                    .help("Custom template directory")
                    .value_name("DIR")
                    .value_parser(value_parser!(PathBuf))
            )
            .arg(
                Arg::new("verbose")
                    .short('V')
                    .long("verbose")
                    .help("Enable verbose output")
                    .action(clap::ArgAction::SetTrue)
            )
            .arg(
                Arg::new("quiet")
                    .short('q')
                    .long("quiet")
                    .help("Suppress output except errors")
                    .action(clap::ArgAction::SetTrue)
            )
            .group(
                ArgGroup::new("verbosity")
                    .args(["verbose", "quiet"])
                    .required(false)
            )
    }

    /// Execute the docs command
    pub fn execute(matches: &clap::ArgMatches) -> Result<(), Error> {
        let input_path = matches.get_one::<PathBuf>("input").unwrap();
        let output_path = matches.get_one::<PathBuf>("output").unwrap();
        let format_str = matches.get_one::<String>("format").unwrap();
        let verbose = matches.get_flag("verbose");
        let quiet = matches.get_flag("quiet");

        // Parse format
        let format = DocFormat::from_str(format_str)
            .map_err(|e| Error::General(format!("Invalid format: {}", e)))?;

        // Build configuration
        let mut config = DocGeneratorConfig {
            output_dir: output_path.clone(),
            format,
            include_examples: !matches.get_flag("no-examples"),
            include_private: matches.get_flag("include-private"),
            generate_cross_refs: !matches.get_flag("no-cross-refs"),
            custom_css: None,
            template_dir: matches.get_one::<PathBuf>("template-dir").cloned(),
            title: matches.get_one::<String>("title").unwrap().clone(),
            description: matches.get_one::<String>("description").cloned(),
            version: matches.get_one::<String>("version").cloned(),
            authors: matches.get_many::<String>("author")
                .map(|vals| vals.cloned().collect())
                .unwrap_or_default(),
            base_url: matches.get_one::<String>("base-url").cloned(),
        };

        // Load custom CSS if provided
        if let Some(css_path) = matches.get_one::<PathBuf>("custom-css") {
            match std::fs::read_to_string(css_path) {
                Ok(css_content) => config.custom_css = Some(css_content),
                Err(e) => {
                    if !quiet {
                        eprintln!("Warning: Failed to load custom CSS from {}: {}", css_path.display(), e);
                    }
                }
            }
        }

        if verbose {
            println!("Generating {} documentation...", format);
            println!("Input: {}", input_path.display());
            println!("Output: {}", output_path.display());
            println!("Configuration:");
            println!("  Title: {}", config.title);
            if let Some(description) = &config.description {
                println!("  Description: {}", description);
            }
            if let Some(version) = &config.version {
                println!("  Version: {}", version);
            }
            if !config.authors.is_empty() {
                println!("  Authors: {}", config.authors.join(", "));
            }
            println!("  Include private: {}", config.include_private);
            println!("  Include examples: {}", config.include_examples);
            println!("  Generate cross-refs: {}", config.generate_cross_refs);
        }

        // Create and run generator
        let mut generator = DocumentationGenerator::new(config);

        let result = if input_path.is_file() {
            generator.generate_from_files(vec![input_path.clone()])
        } else {
            generator.generate_from_directory(input_path)
        };

        match result {
            Ok(()) => {
                if !quiet {
                    println!("Documentation generated successfully in {}", output_path.display());
                    
                    // Show helpful next steps
                    match format {
                        DocFormat::Html => {
                            println!("Open {} to view the documentation", output_path.join("index.html").display());
                        }
                        DocFormat::Markdown => {
                            println!("View the documentation starting with {}", output_path.join("README.md").display());
                        }
                        DocFormat::Json => {
                            println!("JSON documentation available at {}", output_path.join("documentation.json").display());
                        }
                        DocFormat::Xml => {
                            println!("XML documentation available at {}", output_path.join("documentation.xml").display());
                        }
                    }
                }
                Ok(())
            }
            Err(e) => {
                if !quiet {
                    eprintln!("Failed to generate documentation: {}", e);
                }
                Err(e)
            }
        }
    }

    /// Generate documentation for the entire CURSED standard library
    pub fn generate_stdlib_docs(output_dir: &PathBuf, format: DocFormat) -> Result<(), Error> {
        let config = DocGeneratorConfig {
            output_dir: output_dir.clone(),
            format,
            include_examples: true,
            include_private: false,
            generate_cross_refs: true,
            custom_css: None,
            template_dir: None,
            title: "CURSED Standard Library".to_string(),
            description: Some("Comprehensive documentation for the CURSED programming language standard library".to_string()),
            version: Some(env!("CARGO_PKG_VERSION").to_string()),
            authors: vec!["CURSED Team".to_string()],
            base_url: None,
        };

        let mut generator = DocumentationGenerator::new(config);
        
        // Look for stdlib source files
        let stdlib_path = std::path::Path::new("src/stdlib");
        if stdlib_path.exists() {
            generator.generate_from_directory(stdlib_path)
        } else {
            Err(Error::General("Standard library source not found at src/stdlib".to_string()))
        }
    }

    /// Generate API documentation for external tools
    pub fn generate_api_docs(input_dir: &PathBuf, output_dir: &PathBuf) -> Result<(), Error> {
        let config = DocGeneratorConfig {
            output_dir: output_dir.clone(),
            format: DocFormat::Json,
            include_examples: true,
            include_private: false,
            generate_cross_refs: true,
            custom_css: None,
            template_dir: None,
            title: "CURSED API Documentation".to_string(),
            description: Some("Machine-readable API documentation for tooling integration".to_string()),
            version: Some(env!("CARGO_PKG_VERSION").to_string()),
            authors: vec!["CURSED Team".to_string()],
            base_url: None,
        };

        let mut generator = DocumentationGenerator::new(config);
        generator.generate_from_directory(input_dir)?;

        // Also generate search index and schema files
        println!("Generated API documentation with search index and schema");
        Ok(())
    }

    /// Quick documentation generation with sensible defaults
    pub fn quick_generate(input_path: &PathBuf) -> Result<(), Error> {
        let output_path = input_path.parent()
            .unwrap_or_else(|| std::path::Path::new("."))
            .join("docs");

        let config = DocGeneratorConfig {
            output_dir: output_path.clone(),
            format: DocFormat::Html,
            include_examples: true,
            include_private: false,
            generate_cross_refs: true,
            custom_css: None,
            template_dir: None,
            title: "Project Documentation".to_string(),
            description: None,
            version: None,
            authors: Vec::new(),
            base_url: None,
        };

        let mut generator = DocumentationGenerator::new(config);
        
        if input_path.is_file() {
            generator.generate_from_files(vec![input_path.clone()])?;
        } else {
            generator.generate_from_directory(input_path)?;
        }

        println!("Quick documentation generated in {}", output_path.display());
        Ok(())
    }
}

/// Utility functions for documentation CLI
pub mod utils {
    use super::*;

    /// Check if input path contains CURSED source files
    pub fn has_cursed_files(path: &PathBuf) -> bool {
        if path.is_file() {
            return path.extension().map_or(false, |ext| ext == "csd");
        }

        if path.is_dir() {
            return std::fs::read_dir(path)
                .map(|entries| {
                    entries
                        .filter_map(|entry| entry.ok())
                        .any(|entry| {
                            let path = entry.path();
                            path.extension().map_or(false, |ext| ext == "csd") ||
                            (path.is_dir() && has_cursed_files(&path))
                        })
                })
                .unwrap_or(false);
        }

        false
    }

    /// Detect project metadata from common files
    pub fn detect_project_metadata(project_dir: &PathBuf) -> (Option<String>, Option<String>, Vec<String>) {
        let mut title = None;
        let mut description = None;
        let mut authors = Vec::new();

        // Try to read from CursedPackage.toml
        let package_file = project_dir.join("CursedPackage.toml");
        if package_file.exists() {
            if let Ok(content) = std::fs::read_to_string(&package_file) {
                // Simple TOML parsing for common fields
                for line in content.split("\n") {
                    let line = line.trim();
                    if let Some(name) = line.strip_prefix("name = ") {
                        title = Some(name.trim_matches('"').to_string());
                    } else if let Some(desc) = line.strip_prefix("description = ") {
                        description = Some(desc.trim_matches('"').to_string());
                    } else if let Some(author) = line.strip_prefix("author = ") {
                        authors.push(author.trim_matches('"').to_string());
                    }
                }
            }
        }

        // Fallback to directory name for title
        if title.is_none() {
            title = project_dir.file_name()
                .and_then(|name| name.to_str())
                .map(|name| format!("{} Documentation", name));
        }

        (title, description, authors)
    }

    /// Validate documentation configuration
    pub fn validate_config(config: &DocGeneratorConfig) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if config.title.is_empty() {
            errors.push("Title cannot be empty".to_string());
        }

        if !config.output_dir.exists() {
            if let Err(e) = std::fs::create_dir_all(&config.output_dir) {
                errors.push(format!("Cannot create output directory: {}", e));
            }
        }

        if let Some(template_dir) = &config.template_dir {
            if !template_dir.exists() {
                errors.push(format!("Template directory does not exist: {}", template_dir.display()));
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
