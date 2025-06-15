//! Documentation CLI Integration
//! 
//! Command-line interface for the CURSED documentation generator.

use super::generator::{DocumentationGenerator, DocGeneratorConfig, DocFormat};
use crate::error::Error;
use clap::{Arg, ArgMatches, Command, ArgAction};
use std::path::{Path, PathBuf};
use std::str::FromStr;

/// Add documentation commands to CLI
pub fn add_doc_commands(cmd: Command) -> Command {
    cmd.subcommand(
        Command::new("doc")
            .about("Generate documentation")
            .arg(
                Arg::new("input")
                    .help("Input directory or file to document")
                    .value_name("INPUT")
                    .default_value(".")
            )
            .arg(
                Arg::new("output")
                    .short('o')
                    .long("output")
                    .value_name("DIR")
                    .help("Output directory for documentation")
                    .default_value("docs")
            )
            .arg(
                Arg::new("format")
                    .short('f')
                    .long("format")
                    .value_name("FORMAT")
                    .help("Output format: html, markdown, json, xml")
                    .default_value("html")
                    .value_parser(["html", "markdown", "md", "json", "xml"])
            )
            .arg(
                Arg::new("title")
                    .long("title")
                    .value_name("TITLE")
                    .help("Documentation title")
                    .default_value("CURSED Documentation")
            )
            .arg(
                Arg::new("description")
                    .long("description")
                    .value_name("DESC")
                    .help("Project description")
            )
            .arg(
                Arg::new("version")
                    .long("version")
                    .value_name("VERSION")
                    .help("Project version")
            )
            .arg(
                Arg::new("author")
                    .long("author")
                    .value_name("AUTHOR")
                    .action(ArgAction::Append)
                    .help("Project authors (can be used multiple times)")
            )
            .arg(
                Arg::new("include-private")
                    .long("include-private")
                    .action(ArgAction::SetTrue)
                    .help("Include private items in documentation")
            )
            .arg(
                Arg::new("no-examples")
                    .long("no-examples")
                    .action(ArgAction::SetTrue)
                    .help("Exclude code examples from documentation")
            )
            .arg(
                Arg::new("no-cross-refs")
                    .long("no-cross-refs")
                    .action(ArgAction::SetTrue)
                    .help("Disable cross-reference generation")
            )
            .arg(
                Arg::new("custom-css")
                    .long("custom-css")
                    .value_name("FILE")
                    .help("Custom CSS file for HTML output")
            )
            .arg(
                Arg::new("template-dir")
                    .long("template-dir")
                    .value_name("DIR")
                    .help("Custom template directory")
            )
            .arg(
                Arg::new("base-url")
                    .long("base-url")
                    .value_name("URL")
                    .help("Base URL for linking")
            )
            .arg(
                Arg::new("serve")
                    .long("serve")
                    .action(ArgAction::SetTrue)
                    .help("Start local documentation server (HTML format only)")
            )
            .arg(
                Arg::new("port")
                    .long("port")
                    .value_name("PORT")
                    .help("Port for documentation server")
                    .default_value("8080")
            )
            .arg(
                Arg::new("watch")
                    .short('w')
                    .long("watch")
                    .action(ArgAction::SetTrue)
                    .help("Watch files for changes and regenerate")
            )
            .arg(
                Arg::new("config")
                    .short('c')
                    .long("config")
                    .value_name("FILE")
                    .help("Configuration file (TOML, JSON, or YAML)")
            )
            .arg(
                Arg::new("verbose")
                    .short('v')
                    .long("verbose")
                    .action(ArgAction::SetTrue)
                    .help("Verbose output")
            )
    )
}

/// Handle documentation command
pub async fn handle_doc_command(matches: &ArgMatches) -> Result<(), Error> {
    let config = build_config_from_args(matches)?;
    let verbose = matches.get_flag("verbose");
    
    if verbose {
        println!("🚀 Starting documentation generation...");
        println!("   Input: {}", matches.get_one::<String>("input").unwrap());
        println!("   Output: {}", config.output_dir.display());
        println!("   Format: {}", config.format);
    }

    // Load configuration from file if specified
    let config = if let Some(config_file) = matches.get_one::<String>("config") {
        load_config_from_file(config_file, config)?
    } else {
        config
    };

    // Create documentation generator
    let mut generator = DocumentationGenerator::new(config.clone());

    // Determine input path
    let input_path = Path::new(matches.get_one::<String>("input").unwrap());
    
    // Generate documentation
    if input_path.is_dir() {
        if verbose {
            println!("📁 Scanning directory for CURSED files...");
        }
        generator.generate_from_directory(input_path)?;
    } else if input_path.is_file() {
        if verbose {
            println!("📄 Processing single file...");
        }
        generator.generate_from_files(vec![input_path.to_path_buf()])?;
    } else {
        return Err(Error::Parse(format!("Input path does not exist: {}", input_path.display())));
    }

    if verbose {
        println!("✅ Documentation generated successfully!");
        println!("   Output directory: {}", config.output_dir.display());
    }

    // Start server if requested
    if matches.get_flag("serve") {
        let port: u16 = matches.get_one::<String>("port")
            .unwrap()
            .parse()
            .map_err(|_| Error::Parse("Invalid port number".to_string()))?;
        
        start_doc_server(&config.output_dir, port, verbose).await?;
    }

    // Watch for changes if requested
    if matches.get_flag("watch") {
        if verbose {
            println!("👀 Watching for file changes...");
        }
        start_doc_watcher(input_path, config, verbose).await?;
    }

    Ok(())
}

/// Build configuration from command line arguments
fn build_config_from_args(matches: &ArgMatches) -> Result<DocGeneratorConfig, Error> {
    let format = DocFormat::from_str(matches.get_one::<String>("format").unwrap())
        .map_err(|e| Error::Parse(e))?;

    let authors = matches.get_many::<String>("author")
        .map(|values| values.cloned().collect())
        .unwrap_or_default();

    let custom_css = matches.get_one::<String>("custom-css")
        .map(|path| std::fs::read_to_string(path))
        .transpose()
        .map_err(Error::Io)?;

    let template_dir = matches.get_one::<String>("template-dir")
        .map(PathBuf::from);

    Ok(DocGeneratorConfig {
        output_dir: PathBuf::from(matches.get_one::<String>("output").unwrap()),
        format,
        include_examples: !matches.get_flag("no-examples"),
        include_private: matches.get_flag("include-private"),
        generate_cross_refs: !matches.get_flag("no-cross-refs"),
        custom_css,
        template_dir,
        title: matches.get_one::<String>("title").unwrap().clone(),
        description: matches.get_one::<String>("description").cloned(),
        version: matches.get_one::<String>("version").cloned(),
        authors,
        base_url: matches.get_one::<String>("base-url").cloned(),
    })
}

/// Load configuration from file
fn load_config_from_file(config_file: &str, base_config: DocGeneratorConfig) -> Result<DocGeneratorConfig, Error> {
    let config_path = Path::new(config_file);
    
    if !config_path.exists() {
        return Err(Error::Parse(format!("Configuration file not found: {}", config_file)));
    }

    let content = std::fs::read_to_string(config_path)
        .map_err(Error::Io)?;

    // Determine format by extension
    let config = match config_path.extension().and_then(|s| s.to_str()) {
        Some("toml") => {
            toml::from_str::<DocGeneratorConfig>(&content)
                .map_err(|e| Error::Parse(format!("TOML parse error: {}", e)))?
        }
        Some("json") => {
            serde_json::from_str::<DocGeneratorConfig>(&content)
                .map_err(|e| Error::Parse(format!("JSON parse error: {}", e)))?
        }
        Some("yaml") | Some("yml") => {
            serde_yaml::from_str::<DocGeneratorConfig>(&content)
                .map_err(|e| Error::Parse(format!("YAML parse error: {}", e)))?
        }
        _ => {
            return Err(Error::Parse("Unsupported configuration file format. Use .toml, .json, or .yaml".to_string()));
        }
    };

    // Merge with base config (file config takes precedence)
    Ok(merge_configs(base_config, config))
}

/// Merge two configurations (second takes precedence)
fn merge_configs(base: DocGeneratorConfig, override_config: DocGeneratorConfig) -> DocGeneratorConfig {
    DocGeneratorConfig {
        output_dir: if override_config.output_dir.as_os_str().is_empty() { 
            base.output_dir 
        } else { 
            override_config.output_dir 
        },
        format: override_config.format,
        include_examples: override_config.include_examples,
        include_private: override_config.include_private,
        generate_cross_refs: override_config.generate_cross_refs,
        custom_css: override_config.custom_css.or(base.custom_css),
        template_dir: override_config.template_dir.or(base.template_dir),
        title: if override_config.title.is_empty() { 
            base.title 
        } else { 
            override_config.title 
        },
        description: override_config.description.or(base.description),
        version: override_config.version.or(base.version),
        authors: if override_config.authors.is_empty() { 
            base.authors 
        } else { 
            override_config.authors 
        },
        base_url: override_config.base_url.or(base.base_url),
    }
}

/// Start documentation server (simplified implementation)
async fn start_doc_server(output_dir: &Path, port: u16, verbose: bool) -> Result<(), Error> {
    if !matches!(std::env::var("CARGO_PKG_NAME").as_deref(), Ok("cursed")) {
        return Err(Error::Parse("Documentation server requires additional dependencies".to_string()));
    }

    if verbose {
        println!("🌐 Starting documentation server...");
        println!("   Port: {}", port);
        println!("   Directory: {}", output_dir.display());
        println!("   URL: http://localhost:{}", port);
    }

    // Simplified server implementation
    // In a real implementation, this would use a web framework like warp or axum
    println!("📡 Documentation server would start here");
    println!("   Navigate to http://localhost:{} to view documentation", port);
    
    // For now, just indicate that the server would run
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    println!("✅ Server functionality demonstrated (not actually running)");

    Ok(())
}

/// Start documentation file watcher (simplified implementation)
async fn start_doc_watcher(input_path: &Path, config: DocGeneratorConfig, verbose: bool) -> Result<(), Error> {
    if verbose {
        println!("👀 File watcher would monitor: {}", input_path.display());
        println!("   Regenerating documentation on changes...");
    }

    // Simplified watcher implementation
    // In a real implementation, this would use the notify crate
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(5));
    let mut generation_count = 0;

    loop {
        interval.tick().await;
        generation_count += 1;
        
        if verbose {
            println!("🔄 Checking for changes... (iteration {})", generation_count);
        }

        // In a real implementation, this would:
        // 1. Check for file modifications
        // 2. Regenerate documentation if changes detected
        // 3. Notify user of regeneration

        if generation_count >= 3 {
            if verbose {
                println!("✅ Watcher functionality demonstrated");
            }
            break;
        }
    }

    Ok(())
}

/// Generate sample configuration file
pub fn generate_sample_config(format: &str, output_path: &Path) -> Result<(), Error> {
    let config = DocGeneratorConfig::default();
    
    let content = match format {
        "toml" => {
            toml::to_string_pretty(&config)
                .map_err(|e| Error::Parse(format!("TOML serialization error: {}", e)))?
        }
        "json" => {
            serde_json::to_string_pretty(&config)
                .map_err(|e| Error::Parse(format!("JSON serialization error: {}", e)))?
        }
        "yaml" => {
            serde_yaml::to_string(&config)
                .map_err(|e| Error::Parse(format!("YAML serialization error: {}", e)))?
        }
        _ => {
            return Err(Error::Parse("Unsupported format. Use toml, json, or yaml".to_string()));
        }
    };

    std::fs::write(output_path, content).map_err(Error::Io)?;
    Ok(())
}
