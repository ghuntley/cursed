//! CLI Integration for Documentation Generation
//! 
//! Command-line interface for the CURSED documentation generation system.
//! Provides comprehensive options for generating documentation in multiple formats.

use crate::documentation::{
    DocumentationSystem, DocumentationConfig, OutputFormat, ProjectMetadata, 
    DocOptions, StylingConfig, load_config, save_config, create_default_config
};
use crate::error::Error;
use clap::{Arg, ArgMatches, Command, ArgAction};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use tracing::{info, debug, error, instrument};

/// Add documentation commands to the CLI
pub fn add_documentation_commands(cmd: Command) -> Command {
    cmd.subcommand(
        Command::new("doc")
            .about("Generate comprehensive documentation for CURSED projects")
            .long_about("Generate comprehensive API documentation from CURSED source code. \
                         Supports multiple output formats including HTML, Markdown, and JSON \
                         with advanced features like cross-references, search indexing, and \
                         custom styling.")
            .arg(
                Arg::new("input")
                    .help("Input directory or file to document")
                    .value_name("INPUT")
                    .default_value(".")
                    .long_help("The source directory or file to generate documentation from. \
                                Can be a single .csd file or a directory containing CURSED source files.")
            )
            .arg(
                Arg::new("output")
                    .short('o')
                    .long("output")
                    .value_name("DIR")
                    .help("Output directory for generated documentation")
                    .default_value("docs")
                    .long_help("Directory where the generated documentation will be written. \
                                The directory will be created if it doesn't exist.")
            )
            .arg(
                Arg::new("format")
                    .short('f')
                    .long("format")
                    .value_name("FORMAT")
                    .help("Output format(s): html, markdown, json")
                    .action(ArgAction::Append)
                    .value_parser(["html", "markdown", "md", "json", "xml", "latex"])
                    .long_help("Output format for the documentation. Can be specified multiple times \
                                to generate documentation in multiple formats simultaneously.")
            )
            .arg(
                Arg::new("config")
                    .short('c')
                    .long("config")
                    .value_name("FILE")
                    .help("Configuration file (TOML or JSON)")
                    .long_help("Path to a configuration file containing documentation generation \
                                settings. Supports both TOML and JSON formats.")
            )
            .arg(
                Arg::new("title")
                    .long("title")
                    .value_name("TITLE")
                    .help("Documentation title")
                    .default_value("CURSED Documentation")
                    .long_help("The title to use for the generated documentation. \
                                Appears in HTML titles and headers.")
            )
            .arg(
                Arg::new("description")
                    .long("description")
                    .value_name("DESC")
                    .help("Project description")
                    .long_help("A brief description of the project that will appear \
                                in the documentation overview.")
            )
            .arg(
                Arg::new("version")
                    .long("version")
                    .value_name("VERSION")
                    .help("Project version")
                    .long_help("Version string for the project (e.g., '1.0.0')")
            )
            .arg(
                Arg::new("author")
                    .long("author")
                    .value_name("AUTHOR")
                    .action(ArgAction::Append)
                    .help("Project authors (can be used multiple times)")
                    .long_help("Author names for the project. Can be specified multiple times \
                                for projects with multiple authors.")
            )
            .arg(
                Arg::new("homepage")
                    .long("homepage")
                    .value_name("URL")
                    .help("Project homepage URL")
            )
            .arg(
                Arg::new("repository")
                    .long("repository")
                    .value_name("URL")
                    .help("Project repository URL")
            )
            .arg(
                Arg::new("license")
                    .long("license")
                    .value_name("LICENSE")
                    .help("Project license")
            )
            .arg(
                Arg::new("include-private")
                    .long("include-private")
                    .action(ArgAction::SetTrue)
                    .help("Include private items in documentation")
                    .long_help("Include private functions, types, and other items that are \
                                normally not exported. Useful for internal documentation.")
            )
            .arg(
                Arg::new("no-source")
                    .long("no-source")
                    .action(ArgAction::SetTrue)
                    .help("Exclude source code from documentation")
                    .long_help("Don't include source code snippets in the generated documentation. \
                                This can reduce file size for large projects.")
            )
            .arg(
                Arg::new("no-cross-refs")
                    .long("no-cross-refs")
                    .action(ArgAction::SetTrue)
                    .help("Disable cross-reference generation")
                    .long_help("Skip generating cross-references between documentation items. \
                                This can speed up generation for large projects.")
            )
            .arg(
                Arg::new("no-search")
                    .long("no-search")
                    .action(ArgAction::SetTrue)
                    .help("Disable search index generation")
                    .long_help("Don't generate a search index. This disables search functionality \
                                in HTML output but can speed up generation.")
            )
            .arg(
                Arg::new("no-examples")
                    .long("no-examples")
                    .action(ArgAction::SetTrue)
                    .help("Exclude code examples from documentation")
                    .long_help("Don't extract and include code examples from documentation comments.")
            )
            .arg(
                Arg::new("include-dependencies")
                    .long("include-dependencies")
                    .action(ArgAction::SetTrue)
                    .help("Include documentation for dependencies")
                    .long_help("Generate documentation for external dependencies in addition \
                                to the main project code.")
            )
            .arg(
                Arg::new("max-depth")
                    .long("max-depth")
                    .value_name("DEPTH")
                    .help("Maximum type recursion depth")
                    .default_value("10")
                    .value_parser(clap::value_parser!(usize))
                    .long_help("Maximum depth for recursive type documentation to prevent \
                                infinite recursion in complex type hierarchies.")
            )
            .arg(
                Arg::new("theme")
                    .long("theme")
                    .value_name("THEME")
                    .help("HTML theme: light, dark, auto")
                    .default_value("auto")
                    .value_parser(["light", "dark", "auto"])
                    .long_help("Color theme for HTML output. 'auto' uses system preference.")
            )
            .arg(
                Arg::new("custom-css")
                    .long("custom-css")
                    .value_name("FILE")
                    .action(ArgAction::Append)
                    .help("Custom CSS files for HTML output")
                    .long_help("Path to custom CSS files to include in HTML output. \
                                Can be specified multiple times.")
            )
            .arg(
                Arg::new("template-dir")
                    .long("template-dir")
                    .value_name("DIR")
                    .help("Custom template directory")
                    .long_help("Directory containing custom templates for HTML generation. \
                                Templates should follow the expected naming conventions.")
            )
            .arg(
                Arg::new("favicon")
                    .long("favicon")
                    .value_name("FILE")
                    .help("Custom favicon for HTML output")
            )
            .arg(
                Arg::new("logo")
                    .long("logo")
                    .value_name("FILE")
                    .help("Custom logo for documentation")
            )
            .arg(
                Arg::new("watch")
                    .short('w')
                    .long("watch")
                    .action(ArgAction::SetTrue)
                    .help("Watch files for changes and regenerate automatically")
                    .long_help("Monitor source files for changes and automatically regenerate \
                                documentation when files are modified.")
            )
            .arg(
                Arg::new("serve")
                    .long("serve")
                    .value_name("PORT")
                    .help("Serve documentation on local HTTP server")
                    .value_parser(clap::value_parser!(u16))
                    .long_help("Start a local HTTP server to serve the generated documentation. \
                                Useful for previewing HTML output.")
            )
            .arg(
                Arg::new("open")
                    .long("open")
                    .action(ArgAction::SetTrue)
                    .help("Open documentation in browser after generation")
                    .long_help("Automatically open the generated documentation in the default \
                                web browser after generation completes.")
            )
            .arg(
                Arg::new("quiet")
                    .short('q')
                    .long("quiet")
                    .action(ArgAction::SetTrue)
                    .help("Suppress output except for errors")
            )
            .arg(
                Arg::new("verbose")
                    .short('v')
                    .long("verbose")
                    .action(ArgAction::SetTrue)
                    .help("Enable verbose output")
                    .conflicts_with("quiet")
            )
            .arg(
                Arg::new("save-config")
                    .long("save-config")
                    .value_name("FILE")
                    .help("Save current configuration to file")
                    .long_help("Save the current configuration (including command-line options) \
                                to a file for future use.")
            )
            .arg(
                Arg::new("init-config")
                    .long("init-config")
                    .value_name("FILE")
                    .help("Create a default configuration file")
                    .long_help("Create a new configuration file with default settings \
                                that can be customized for the project.")
            )
    )
}

/// Handle documentation command execution
#[instrument(skip(matches))]
pub async fn handle_documentation_command(matches: &ArgMatches) -> Result<(), Error> {
    // Handle configuration initialization
    if let Some(config_file) = matches.get_one::<String>("init-config") {
        return handle_init_config(config_file);
    }

    // Set up logging level based on verbosity
    let quiet = matches.get_flag("quiet");
    let verbose = matches.get_flag("verbose");
    
    if !quiet {
        if verbose {
            info!("Starting CURSED documentation generation (verbose mode)");
        } else {
            println!("🚀 Generating CURSED documentation...");
        }
    }

    // Load or build configuration
    let mut config = if let Some(config_file) = matches.get_one::<String>("config") {
        load_config(Path::new(config_file))?
    } else {
        build_config_from_args(matches)?
    };

    // Override config with command-line arguments
    override_config_with_args(&mut config, matches)?;

    // Save configuration if requested
    if let Some(save_path) = matches.get_one::<String>("save-config") {
        save_config(&config, Path::new(save_path))?;
        if !quiet {
            println!("✅ Configuration saved to: {}", save_path);
        }
    }

    // Create documentation system
    let mut doc_system = DocumentationSystem::new(config.clone())?;

    // Handle watch mode
    if matches.get_flag("watch") {
        return handle_watch_mode(doc_system, matches).await;
    }

    // Generate documentation
    let result = doc_system.generate_all().await?;

    // Report results
    if !quiet {
        println!("✅ Documentation generation completed!");
        println!("   📁 Files processed: {}", result.files_processed);
        println!("   📝 Items documented: {}", result.items_documented);
        println!("   📄 Output files: {}", result.output_files.len());
        println!("   ⏱️  Processing time: {}ms", result.processing_time_ms);

        if !result.warnings.is_empty() {
            println!("   ⚠️  Warnings: {}", result.warnings.len());
            for warning in &result.warnings {
                println!("      - {}", warning);
            }
        }

        println!("   📂 Output directory: {}", config.output_dir.display());
    }

    // Serve documentation if requested
    if let Some(port) = matches.get_one::<u16>("serve") {
        handle_serve_mode(&config.output_dir, *port, matches.get_flag("open")).await?;
    } else if matches.get_flag("open") {
        open_documentation(&config.output_dir)?;
    }

    Ok(())
}

/// Build configuration from command-line arguments
fn build_config_from_args(matches: &ArgMatches) -> Result<DocumentationConfig, Error> {
    let input = matches.get_one::<String>("input").unwrap();
    let output = matches.get_one::<String>("output").unwrap();

    // Parse output formats
    let output_formats = if let Some(formats) = matches.get_many::<String>("format") {
        formats.map(|f| OutputFormat::from_str(f))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| Error::ConfigurationError(format!("Invalid output format: {}", e)))?
    } else {
        vec![OutputFormat::Html] // Default to HTML
    };

    // Build project metadata
    let project = ProjectMetadata {
        name: matches.get_one::<String>("title").unwrap().clone(),
        version: matches.get_one::<String>("version")
            .unwrap_or(&"0.1.0".to_string()).clone(),
        description: matches.get_one::<String>("description").cloned(),
        authors: matches.get_many::<String>("author")
            .map(|v| v.cloned().collect())
            .unwrap_or_default(),
        homepage: matches.get_one::<String>("homepage").cloned(),
        repository: matches.get_one::<String>("repository").cloned(),
        license: matches.get_one::<String>("license").cloned(),
    };

    // Build documentation options
    let options = DocOptions {
        include_private: matches.get_flag("include-private"),
        include_source: !matches.get_flag("no-source"),
        generate_cross_refs: !matches.get_flag("no-cross-refs"),
        generate_search_index: !matches.get_flag("no-search"),
        include_examples: !matches.get_flag("no-examples"),
        max_type_depth: *matches.get_one::<usize>("max-depth").unwrap(),
        include_dependencies: matches.get_flag("include-dependencies"),
    };

    // Build styling configuration
    let styling = StylingConfig {
        custom_css: matches.get_many::<String>("custom-css")
            .map(|v| v.map(PathBuf::from).collect())
            .unwrap_or_default(),
        template_dir: matches.get_one::<String>("template-dir")
            .map(PathBuf::from),
        theme: matches.get_one::<String>("theme").unwrap().clone(),
        colors: None, // TODO: Support color overrides
        favicon: matches.get_one::<String>("favicon").map(PathBuf::from),
        logo: matches.get_one::<String>("logo").map(PathBuf::from),
    };

    // Determine source directories
    let source_dirs = if Path::new(input).is_file() {
        vec![Path::new(input).parent().unwrap_or(Path::new(".")).to_path_buf()]
    } else {
        vec![PathBuf::from(input)]
    };

    Ok(DocumentationConfig {
        source_dirs,
        output_dir: PathBuf::from(output),
        output_formats,
        project,
        options,
        styling,
    })
}

/// Override configuration with command-line arguments
fn override_config_with_args(config: &mut DocumentationConfig, matches: &ArgMatches) -> Result<(), Error> {
    // Override output directory
    if let Some(output) = matches.get_one::<String>("output") {
        config.output_dir = PathBuf::from(output);
    }

    // Override output formats if specified
    if let Some(formats) = matches.get_many::<String>("format") {
        config.output_formats = formats.map(|f| OutputFormat::from_str(f))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| Error::ConfigurationError(format!("Invalid output format: {}", e)))?;
    }

    // Override project metadata
    if let Some(title) = matches.get_one::<String>("title") {
        config.project.name = title.clone();
    }
    
    if let Some(description) = matches.get_one::<String>("description") {
        config.project.description = Some(description.clone());
    }
    
    if let Some(version) = matches.get_one::<String>("version") {
        config.project.version = version.clone();
    }
    
    if let Some(authors) = matches.get_many::<String>("author") {
        config.project.authors = authors.cloned().collect();
    }

    // Override documentation options
    if matches.get_flag("include-private") {
        config.options.include_private = true;
    }
    
    if matches.get_flag("no-source") {
        config.options.include_source = false;
    }
    
    if matches.get_flag("no-cross-refs") {
        config.options.generate_cross_refs = false;
    }
    
    if matches.get_flag("no-search") {
        config.options.generate_search_index = false;
    }
    
    if matches.get_flag("no-examples") {
        config.options.include_examples = false;
    }

    Ok(())
}

/// Handle configuration file initialization
fn handle_init_config(config_file: &str) -> Result<(), Error> {
    let config_path = Path::new(config_file);
    
    if config_path.exists() {
        return Err(Error::ConfigurationError(
            format!("Configuration file already exists: {}", config_file)
        ));
    }

    create_default_config(config_path)?;
    
    println!("✅ Created default configuration file: {}", config_file);
    println!("   Edit this file to customize documentation generation settings.");
    
    Ok(())
}

/// Handle watch mode for automatic regeneration
async fn handle_watch_mode(
    mut doc_system: DocumentationSystem,
    _matches: &ArgMatches,
) -> Result<(), Error> {
    println!("👀 Watching for file changes...");
    println!("   Press Ctrl+C to stop watching");

    // TODO: Implement file watching using notify crate
    // For now, just generate once and show a message
    let _result = doc_system.generate_all().await?;
    
    println!("✅ Initial documentation generated");
    println!("🔧 File watching not yet implemented - run command again to regenerate");
    
    Ok(())
}

/// Handle serve mode for local HTTP server
async fn handle_serve_mode(
    output_dir: &Path,
    port: u16,
    open_browser: bool,
) -> Result<(), Error> {
    println!("🌐 Starting local documentation server...");
    println!("   URL: http://localhost:{}", port);
    println!("   Directory: {}", output_dir.display());
    
    if open_browser {
        let url = format!("http://localhost:{}", port);
        if let Err(e) = open::that(&url) {
            eprintln!("Failed to open browser: {}", e);
        } else {
            println!("🔗 Opened documentation in browser");
        }
    }
    
    // TODO: Implement HTTP server using warp or similar
    println!("🔧 HTTP server not yet implemented");
    println!("   You can serve the documentation using any static file server:");
    println!("   cd {} && python -m http.server {}", output_dir.display(), port);
    
    Ok(())
}

/// Open documentation in the default browser
fn open_documentation(output_dir: &Path) -> Result<(), Error> {
    let index_file = output_dir.join("index.html");
    
    if index_file.exists() {
        if let Err(e) = open::that(&index_file) {
            eprintln!("Failed to open documentation: {}", e);
        } else {
            println!("🔗 Opened documentation in browser");
        }
    } else {
        println!("📁 Documentation generated in: {}", output_dir.display());
    }
    
    Ok(())
}

impl FromStr for OutputFormat {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "html" => Ok(OutputFormat::Html),
            "markdown" | "md" => Ok(OutputFormat::Markdown),
            "json" => Ok(OutputFormat::Json),
            "xml" => Ok(OutputFormat::Xml),
            "latex" => Ok(OutputFormat::LaTeX),
            _ => Err(format!("Unsupported output format: {}", s)),
        }
    }
}

/// Generate sample configuration for documentation
pub fn generate_sample_config() -> DocumentationConfig {
    let mut config = DocumentationConfig::default();
    
    // Set example project metadata
    config.project.name = "My CURSED Project".to_string();
    config.project.version = "1.0.0".to_string();
    config.project.description = Some("An example CURSED project with comprehensive documentation".to_string());
    config.project.authors = vec!["John Doe <john@example.com>".to_string()];
    config.project.homepage = Some("https://example.com".to_string());
    config.project.repository = Some("https://github.com/user/repo".to_string());
    config.project.license = Some("MIT".to_string());
    
    // Enable all features by default
    config.options.include_private = false;
    config.options.include_source = true;
    config.options.generate_cross_refs = true;
    config.options.generate_search_index = true;
    config.options.include_examples = true;
    config.options.include_dependencies = false;
    
    // Set up styling
    config.styling.theme = "auto".to_string();
    
    config
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Command;

    #[test]
    fn test_add_documentation_commands() {
        let app = Command::new("test");
        let app_with_docs = add_documentation_commands(app);
        
        // Test that the doc subcommand was added
        let matches = app_with_docs.try_get_matches_from(vec!["test", "doc", "--help"]);
        assert!(matches.is_err()); // Help causes early exit, but command exists
    }

    #[test]
    fn test_build_config_from_args() {
        let app = Command::new("test");
        let app_with_docs = add_documentation_commands(app);
        
        let matches = app_with_docs.try_get_matches_from(vec![
            "test", "doc", 
            "--title", "Test Project",
            "--format", "html",
            "--format", "markdown"
        ]).unwrap();
        
        let doc_matches = matches.subcommand_matches("doc").unwrap();
        let config = build_config_from_args(doc_matches).unwrap();
        
        assert_eq!(config.project.name, "Test Project");
        assert_eq!(config.output_formats.len(), 2);
    }

    #[test]
    fn test_generate_sample_config() {
        let config = generate_sample_config();
        
        assert!(!config.project.name.is_empty());
        assert!(config.project.description.is_some());
        assert!(!config.project.authors.is_empty());
        assert!(config.options.include_source);
        assert!(config.options.generate_cross_refs);
    }
}
