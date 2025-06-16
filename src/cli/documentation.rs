//! CLI Documentation Commands
//! 
//! Command-line interface for CURSED documentation generation with
//! comprehensive options and integration with the documentation system.

use crate::docs::{DocumentationGenerator, DocGeneratorConfig, DocFormat, ApiExtractor, ExampleGenerator};
use crate::error::Error;
use clap::{ArgMatches, Command, Arg, ArgAction};
use std::path::PathBuf;
use std::str::FromStr;
use tokio::time::{timeout, Duration};
use tracing::{info, warn, error, debug};

/// Add documentation subcommands to the CLI
pub fn add_documentation_commands(app: Command) -> Command {
    app.subcommand(
        Command::new("doc")
            .about("Generate comprehensive documentation for CURSED projects")
            .long_about("Generate documentation from CURSED source code with support for \
                        multiple output formats, cross-references, and Gen Z slang examples.")
            .arg(
                Arg::new("input")
                    .help("Input directory or file to document")
                    .value_name("INPUT")
                    .default_value(".")
                    .long_help("Source directory or file to generate documentation from")
            )
            .arg(
                Arg::new("output")
                    .short('o')
                    .long("output")
                    .value_name("DIR")
                    .help("Output directory for generated documentation")
                    .default_value("docs")
            )
            .arg(
                Arg::new("format")
                    .short('f')
                    .long("format")
                    .value_name("FORMAT")
                    .help("Output format: html, markdown, json, xml")
                    .action(ArgAction::Append)
                    .value_parser(["html", "markdown", "md", "json", "xml"])
                    .default_value("html")
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
                    .help("Project authors")
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
                Arg::new("watch")
                    .short('w')
                    .long("watch")
                    .action(ArgAction::SetTrue)
                    .help("Watch files and regenerate on changes")
            )
            .arg(
            Arg::new("serve")
            .long("serve")
            .value_name("PORT")
            .help("Serve documentation on HTTP server")
            .value_parser(clap::value_parser!(u16))
            )
            .arg(
                Arg::new("live")
                .long("live")
                .action(ArgAction::SetTrue)
                .help("Enable live reload with hot documentation regeneration")
            )
            .arg(
                Arg::new("playground")
                .long("playground")
                .action(ArgAction::SetTrue)
                .help("Enable interactive code playground")
            )
            .arg(
                Arg::new("api-explorer")
                .long("api-explorer")
                .action(ArgAction::SetTrue)
                .help("Enable interactive API explorer")
            )
            .arg(
                Arg::new("open")
                    .long("open")
                    .action(ArgAction::SetTrue)
                    .help("Open documentation in browser")
            )
            .arg(
                Arg::new("verbose")
                    .short('v')
                    .long("verbose")
                    .action(ArgAction::SetTrue)
                    .help("Enable verbose output")
            )
    )
}

/// Handle documentation command
pub async fn handle_documentation_command(matches: &ArgMatches) -> Result<(), Error> {
    let input_path = PathBuf::from(matches.get_one::<String>("input").unwrap());
    let output_dir = PathBuf::from(matches.get_one::<String>("output").unwrap());
    let verbose = matches.get_flag("verbose");
    
    if verbose {
        info!("Starting CURSED documentation generation");
        info!("Input: {}", input_path.display());
        info!("Output: {}", output_dir.display());
    }

    // Parse formats
    let format_strings: Vec<&String> = matches.get_many::<String>("format").unwrap_or_default().collect();
    let mut formats = Vec::new();
    
    for format_str in format_strings {
        match DocFormat::from_str(format_str) {
            Ok(format) => formats.push(format),
            Err(e) => {
                error!("Invalid format '{}': {}", format_str, e);
                return Err(Error::General(format!("Invalid format: {}", format_str)));
            }
        }
    }
    
    if formats.is_empty() {
        formats.push(DocFormat::Html);
    }

    // Build configuration
    let config = DocGeneratorConfig {
        output_dir: output_dir.clone(),
        format: formats[0].clone(), // Use first format as primary
        include_examples: !matches.get_flag("no-examples"),
        include_private: matches.get_flag("include-private"),
        generate_cross_refs: !matches.get_flag("no-cross-refs"),
        custom_css: None,
        template_dir: None,
        title: matches.get_one::<String>("title").unwrap().clone(),
        description: matches.get_one::<String>("description").cloned(),
        version: matches.get_one::<String>("version").cloned(),
        authors: matches.get_many::<String>("author")
            .map(|authors| authors.cloned().collect())
            .unwrap_or_default(),
        base_url: None,
    };

    // Generate documentation for each format
    for format in formats {
        let mut format_config = config.clone();
        format_config.format = format.clone();
        
        // Create format-specific output directory
        let format_output_dir = if format_strings.len() > 1 {
            output_dir.join(format.to_string())
        } else {
            output_dir.clone()
        };
        format_config.output_dir = format_output_dir;

        info!("Generating {} documentation...", format);
        
        // Generate with timeout
        let generation_result = timeout(
            Duration::from_secs(300), // 5 minute timeout
            generate_documentation_format(format_config, &input_path)
        ).await;

        match generation_result {
            Ok(Ok(())) => {
                info!("✅ {} documentation generated successfully", format);
            }
            Ok(Err(e)) => {
                error!("❌ Failed to generate {} documentation: {}", format, e);
                return Err(e);
            }
            Err(_) => {
                error!("❌ Documentation generation timed out for format {}", format);
                return Err(Error::General("Documentation generation timed out".to_string()));
            }
        }
    }

    // Handle watch mode
    if matches.get_flag("watch") {
        info!("👀 Watching for file changes...");
        return watch_and_regenerate(config, &input_path).await;
    }

    // Handle serve mode
    if let Some(port) = matches.get_one::<u16>("serve") {
        let enable_live = matches.get_flag("live");
        let enable_playground = matches.get_flag("playground");
        let enable_api_explorer = matches.get_flag("api-explorer");
        
        if enable_live || enable_playground || enable_api_explorer {
            info!("🌐 Starting live documentation server on port {}...", port);
            return start_live_documentation_server(&input_path, &output_dir, *port, 
                enable_live, enable_playground, enable_api_explorer).await;
        } else {
            info!("🌐 Starting documentation server on port {}...", port);
            return serve_documentation(&output_dir, *port).await;
        }
    }

    // Handle open browser
    if matches.get_flag("open") {
        open_documentation(&output_dir)?;
    }

    info!("🎉 Documentation generation completed successfully!");
    Ok(())
}

/// Generate documentation for a specific format
async fn generate_documentation_format(config: DocGeneratorConfig, input_path: &PathBuf) -> Result<(), Error> {
    let mut generator = DocumentationGenerator::new(config);
    
    if input_path.is_file() {
        generator.generate_from_files(vec![input_path.clone()])?;
    } else if input_path.is_dir() {
        generator.generate_from_directory(input_path)?;
    } else {
        return Err(Error::General(format!("Invalid input path: {}", input_path.display())));
    }
    
    Ok(())
}

/// Watch files and regenerate documentation on changes
async fn watch_and_regenerate(config: DocGeneratorConfig, input_path: &PathBuf) -> Result<(), Error> {
    use notify::{Watcher, RecommendedWatcher, RecursiveMode, Event, EventKind};
    use notify::event::{CreateKind, ModifyKind, RemoveKind};
    use std::sync::mpsc::channel;
    use std::time::Instant;

    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(1))
        .map_err(|e| Error::General(format!("Failed to create file watcher: {}", e)))?;

    watcher.watch(input_path, RecursiveMode::Recursive)
        .map_err(|e| Error::General(format!("Failed to watch directory: {}", e)))?;

    info!("Watching {} for changes. Press Ctrl+C to stop.", input_path.display());
    
    let mut last_generation = Instant::now();
    let debounce_duration = Duration::from_millis(500);

    loop {
        match rx.recv() {
            Ok(Ok(event)) => {
                if should_regenerate(&event) {
                    let now = Instant::now();
                    if now.duration_since(last_generation) > debounce_duration {
                        info!("📝 File change detected, regenerating documentation...");
                        
                        match generate_documentation_format(config.clone(), input_path).await {
                            Ok(()) => {
                                info!("✅ Documentation regenerated successfully");
                                last_generation = now;
                            }
                            Err(e) => {
                                warn!("⚠️ Failed to regenerate documentation: {}", e);
                            }
                        }
                    }
                }
            }
            Ok(Err(e)) => {
                warn!("File watcher error: {}", e);
            }
            Err(e) => {
                error!("File watcher channel error: {}", e);
                break;
            }
        }
    }

    Ok(())
}

/// Check if file event should trigger regeneration
fn should_regenerate(event: &Event) -> bool {
    match &event.kind {
        EventKind::Create(CreateKind::File) |
        EventKind::Modify(ModifyKind::Data(_)) |
        EventKind::Remove(RemoveKind::File) => {
            // Only regenerate for .csd files
            event.paths.iter().any(|path| {
                path.extension().map_or(false, |ext| ext == "csd")
            })
        }
        _ => false,
    }
}

/// Serve documentation on HTTP server
async fn serve_documentation(docs_dir: &PathBuf, port: u16) -> Result<(), Error> {
    use warp::Filter;
    
    if !docs_dir.exists() {
        return Err(Error::General(format!("Documentation directory does not exist: {}", docs_dir.display())));
    }

    info!("📖 Serving documentation at http://localhost:{}", port);
    info!("📁 Document root: {}", docs_dir.display());
    
    let files = warp::fs::dir(docs_dir.clone());
    let routes = files
        .or(warp::path::end().and(warp::fs::file(docs_dir.join("index.html"))))
        .with(warp::cors().allow_any_origin());

    println!("🌐 Documentation server running at http://localhost:{}", port);
    println!("📖 Open your browser to view the documentation");
    println!("🛑 Press Ctrl+C to stop the server");

    warp::serve(routes)
        .run(([127, 0, 0, 1], port))
        .await;

    Ok(())
}

/// Start live documentation server with hot reload and interactive features
async fn start_live_documentation_server(
    input_path: &PathBuf,
    output_dir: &PathBuf,
    port: u16,
    enable_live: bool,
    enable_playground: bool,
    enable_api_explorer: bool,
) -> Result<(), Error> {
    use crate::documentation::live_server::{LiveDocumentationServer, LiveServerConfig};
    use std::time::Duration;
    
    // Create live server configuration
    let mut config = LiveServerConfig {
        port,
        host: "127.0.0.1".to_string(),
        watch_debounce: Duration::from_millis(500),
        enable_playground,
        enable_api_explorer,
        auto_open_browser: true,
        ..Default::default()
    };
    
    // Create and start live server
    let mut server = LiveDocumentationServer::new(config)
        .map_err(|e| Error::General(format!("Failed to create live server: {}", e)))?;
    
    // Start serving with hot reload
    server.start_serving(&[input_path], output_dir).await
        .map_err(|e| Error::General(format!("Failed to start live server: {}", e)))?;
    
    Ok(())
}

/// Open documentation in default browser
fn open_documentation(docs_dir: &PathBuf) -> Result<(), Error> {
    let index_file = docs_dir.join("index.html");
    
    if !index_file.exists() {
        return Err(Error::General("Documentation not found. Run generation first.".to_string()));
    }

    info!("🌐 Opening documentation in browser...");
    
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(["/c", "start", &index_file.to_string_lossy()])
            .spawn()
            .map_err(|e| Error::General(format!("Failed to open browser: {}", e)))?;
    }

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&index_file)
            .spawn()
            .map_err(|e| Error::General(format!("Failed to open browser: {}", e)))?;
    }

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&index_file)
            .spawn()
            .map_err(|e| Error::General(format!("Failed to open browser: {}", e)))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_documentation_generation() {
        let temp_dir = TempDir::new().unwrap();
        let output_dir = temp_dir.path().join("docs");
        
        let config = DocGeneratorConfig {
            output_dir: output_dir.clone(),
            format: DocFormat::Html,
            include_examples: true,
            include_private: false,
            generate_cross_refs: true,
            custom_css: None,
            template_dir: None,
            title: "Test Documentation".to_string(),
            description: Some("Test project".to_string()),
            version: Some("1.0.0".to_string()),
            authors: vec!["Test Author".to_string()],
            base_url: None,
        };

        // Test basic config creation
        assert_eq!(config.title, "Test Documentation");
        assert_eq!(config.include_examples, true);
        assert_eq!(config.include_private, false);
    }

    #[test]
    fn test_format_parsing() {
        assert!(matches!(DocFormat::from_str("html"), Ok(DocFormat::Html)));
        assert!(matches!(DocFormat::from_str("markdown"), Ok(DocFormat::Markdown)));
        assert!(matches!(DocFormat::from_str("md"), Ok(DocFormat::Markdown)));
        assert!(matches!(DocFormat::from_str("json"), Ok(DocFormat::Json)));
        assert!(matches!(DocFormat::from_str("xml"), Ok(DocFormat::Xml)));
        assert!(DocFormat::from_str("invalid").is_err());
    }

    #[test]
    fn test_should_regenerate() {
        use notify::event::{CreateKind, ModifyKind, DataChange};
        use std::path::Path;
        
        let event = Event {
            kind: EventKind::Create(CreateKind::File),
            paths: vec![Path::new("test.csd").to_path_buf()],
            attrs: Default::default(),
        };
        assert!(should_regenerate(&event));

        let event = Event {
            kind: EventKind::Modify(ModifyKind::Data(DataChange::Content)),
            paths: vec![Path::new("test.csd").to_path_buf()],
            attrs: Default::default(),
        };
        assert!(should_regenerate(&event));

        let event = Event {
            kind: EventKind::Create(CreateKind::File),
            paths: vec![Path::new("test.txt").to_path_buf()],
            attrs: Default::default(),
        };
        assert!(!should_regenerate(&event));
    }
}
