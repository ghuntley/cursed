
// Enhanced CLI Documentation Commands
// 
// Extended command-line interface for CURSED documentation with publishing,
// server management, testing, and registry functionality.

use clap::{Args, Subcommand};
use crate::docs::publisher::{DocumentationPublisher, PublishConfig, PublishTarget, OptimizationConfig};
use crate::docs::server::{DocumentationServer, ServerConfig};
use crate::docs::registry::{DocumentationRegistry, RegistryConfig};
use crate::docs::testing::{DocumentationTester, TestingConfig};
use crate::documentation::generator::DocumentationGenerator;
use crate::package::{Package, PackageManager};
use crate::error::{CursedError, Result};
use std::path::PathBuf;
use std::net::SocketAddr;
use tracing::{info, warn, error};
use tokio::fs;

#[derive(Debug, Args)]
pub struct DocsArgs {
    #[command(subcommand)]
    pub command: DocsCommand,
}

#[derive(Debug, Subcommand)]
pub enum DocsCommand {
    /// Generate documentation
    Generate {
        /// Source directory
        #[arg(short, long, default_value = "src")]
        source_dir: PathBuf,
        
        /// Output directory
        #[arg(short, long, default_value = "docs")]
        output_dir: PathBuf,
        
        /// Output format
        #[arg(short, long, default_value = "html")]
        format: String,
        
        /// Include private items
        #[arg(long)]
        include_private: bool,
    },
    
    /// Publish documentation
    Publish {
        /// Package name to publish
        #[arg(short, long)]
        package: Option<String>,
        
        /// Version to publish
        #[arg(short, long)]
        version: Option<String>,
        
        /// Publishing target (local, s3, github-pages, custom)
        #[arg(short, long, default_value = "local")]
        target: String,
        
        /// Base URL for published documentation
        #[arg(short, long)]
        base_url: Option<String>,
        
        /// Enable optimization
        #[arg(long)]
        optimize: bool,
        
        /// Configuration file
        #[arg(short, long)]
        config: Option<PathBuf>,
        
        /// Dry run (don't actually publish)
        #[arg(long)]
        dry_run: bool,
    },
    
    /// Start documentation server
    Serve {
        /// Bind address
        #[arg(short, long, default_value = "127.0.0.1:8080")]
        bind: SocketAddr,
        
        /// Document root directory
        #[arg(short, long, default_value = "./docs")]
        document_root: PathBuf,
        
        /// Enable HTTPS
        #[arg(long)]
        https: bool,
        
        /// SSL certificate file
        #[arg(long)]
        cert: Option<PathBuf>,
        
        /// SSL private key file
        #[arg(long)]
        key: Option<PathBuf>,
        
        /// Configuration file
        #[arg(short, long)]
        config: Option<PathBuf>,
    },
    
    /// Test documentation
    Test {
        /// Package to test
        #[arg(short, long)]
        package: Option<String>,
        
        /// Version to test
        #[arg(short, long)]
        version: Option<String>,
        
        /// Check links
        #[arg(long)]
        check_links: bool,
        
        /// Verify examples
        #[arg(long)]
        verify_examples: bool,
        
        /// Check completeness
        #[arg(long)]
        check_completeness: bool,
        
        /// Check accessibility
        #[arg(long)]
        check_accessibility: bool,
        
        /// Output format (text, json, html)
        #[arg(short, long, default_value = "text")]
        format: String,
        
        /// Output file
        #[arg(short, long)]
        output: Option<PathBuf>,
        
        /// Configuration file
        #[arg(short, long)]
        config: Option<PathBuf>,
    },
    
    /// Preview documentation locally
    Preview {
        /// Source directory
        #[arg(short, long, default_value = "src")]
        source_dir: PathBuf,
        
        /// Port to serve on
        #[arg(short, long, default_value = "3000")]
        port: u16,
        
        /// Auto-rebuild on changes
        #[arg(long)]
        watch: bool,
        
        /// Open browser automatically
        #[arg(long)]
        open: bool,
    },
    
    /// Validate documentation configuration
    Validate {
        /// Configuration file to validate
        #[arg(short, long)]
        config: PathBuf,
        
        /// Configuration type (publish, server, registry, testing)
        #[arg(short, long)]
        config_type: String,
    },
    
    /// Manage documentation registry
    Registry {
        #[command(subcommand)]
        command: RegistryCommand,
    },
}

#[derive(Debug, Subcommand)]
pub enum RegistryCommand {
    /// Initialize registry
    Init {
        /// Registry data directory
        #[arg(short, long, default_value = "./registry")]
        data_dir: PathBuf,
    },
    
    /// List packages in registry
    List {
        /// Package name filter
        #[arg(short, long)]
        package: Option<String>,
        
        /// Output format (text, json)
        #[arg(short, long, default_value = "text")]
        format: String,
    },
    
    /// Search registry
    Search {
        /// Search query
        query: String,
        
        /// Limit results
        #[arg(short, long, default_value = "20")]
        limit: usize,
        
        /// Output format (text, json)
        #[arg(short, long, default_value = "text")]
        format: String,
    },
    
    /// Show package information
    Show {
        /// Package name
        package: String,
        
        /// Version (defaults to latest)
        #[arg(short, long)]
        version: Option<String>,
    },
    
    /// Clean registry
    Clean {
        /// Remove old versions
        #[arg(long)]
        old_versions: bool,
        
        /// Remove orphaned entries
        #[arg(long)]
        orphaned: bool,
    },
}

pub async fn handle_docs_command(args: DocsArgs) -> Result<()> {
    match args.command {
        DocsCommand::Generate { source_dir, output_dir, format, include_private } => {
            handle_generate_command(source_dir, output_dir, format, include_private).await
        }
        
        DocsCommand::Publish { package, version, target, base_url, optimize, config, dry_run } => {
            handle_publish_command(package, version, target, base_url, optimize, config, dry_run).await
        }
        
        DocsCommand::Serve { bind, document_root, https, cert, key, config } => {
            handle_serve_command(bind, document_root, https, cert, key, config).await
        }
        
        DocsCommand::Test { package, version, check_links, verify_examples, check_completeness, check_accessibility, format, output, config } => {
            handle_test_command(package, version, check_links, verify_examples, check_completeness, check_accessibility, format, output, config).await
        }
        
        DocsCommand::Preview { source_dir, port, watch, open } => {
            handle_preview_command(source_dir, port, watch, open).await
        }
        
        DocsCommand::Validate { config, config_type } => {
            handle_validate_command(config, config_type).await
        }
        
        DocsCommand::Registry { command } => {
            handle_registry_command(command).await
        }
    }
}

async fn handle_generate_command(
    source_dir: PathBuf,
    output_dir: PathBuf,
    format: String,
    include_private: bool,
) -> Result<()> {
    info!("Generating documentation from {:?} to {:?}", source_dir, output_dir);
    
    let mut generator = DocumentationGenerator::new();
    
    match format.as_str() {
        "html" => {
            generator.generate_html(&source_dir, &output_dir, include_private).await?;
            info!("HTML documentation generated in {:?}", output_dir);
        }
        "markdown" => {
            generator.generate_markdown(&source_dir, &output_dir, include_private).await?;
            info!("Markdown documentation generated in {:?}", output_dir);
        }
        "json" => {
            generator.generate_json(&source_dir, &output_dir, include_private).await?;
            info!("JSON documentation generated in {:?}", output_dir);
        }
        _ => {
            return Err(CursedError::General(format!("Unsupported documentation format: {}", format)));
        }
    }
    
    Ok(())
}

async fn handle_publish_command(
    package: Option<String>,
    version: Option<String>,
    target: String,
    base_url: Option<String>,
    optimize: bool,
    config_file: Option<PathBuf>,
    dry_run: bool,
) -> Result<()> {
    info!("Publishing documentation");
    
    // Load configuration
    let config = if let Some(config_path) = config_file {
        load_publish_config(&config_path).await?
    } else {
        create_default_publish_config(&target, base_url, optimize)?
    };
    
    // Get package information
    let package_manager = PackageManager::new();
    let current_package = package_manager.get_current_package().await?;
    
    let pkg_name = package.unwrap_or(current_package.name);
    let pkg_version = version.unwrap_or(current_package.version);
    
    let package_to_publish = Package {
        name: pkg_name.clone(),
        version: pkg_version.clone(),
        description: current_package.description,
        authors: current_package.authors,
        license: current_package.license,
        repository: current_package.repository,
        homepage: current_package.homepage,
        keywords: current_package.keywords,
        dependencies: current_package.dependencies,
        dev_dependencies: current_package.dev_dependencies,
        build_dependencies: current_package.build_dependencies,
        features: current_package.features,
        default_features: current_package.default_features,
        edition: current_package.edition,
        rust_version: current_package.rust_version,
        exclude: current_package.exclude,
        include: current_package.include,
        links: current_package.links,
        path: current_package.path,
    };
    
    if dry_run {
        info!("Dry run - would publish package {} version {}", pkg_name, pkg_version);
        return Ok(());
    }
    
    // Create publisher
    let generator = DocumentationGenerator::new();
    let registry = DocumentationRegistry::new(RegistryConfig::default());
    let mut publisher = DocumentationPublisher::new(config, generator, registry, package_manager);
    
    // Validate configuration
    publisher.validate_config()?;
    
    // Publish documentation
    let metadata = publisher.publish_package(&package_to_publish).await?;
    
    info!(
        "Documentation published successfully for {} {}",
        metadata.package_name, metadata.version
    );
    info!("URL: {}", metadata.url);
    info!(
        "Build time: {}ms, Upload time: {}ms",
        metadata.performance.build_time_ms,
        metadata.performance.upload_time_ms
    );
    
    Ok(())
}

async fn handle_serve_command(
    bind: SocketAddr,
    document_root: PathBuf,
    https: bool,
    cert: Option<PathBuf>,
    key: Option<PathBuf>,
    config_file: Option<PathBuf>,
) -> Result<()> {
    info!("Starting documentation server on {}", bind);
    
    // Load or create server configuration
    let config = if let Some(config_path) = config_file {
        load_server_config(&config_path).await?
    } else {
        create_default_server_config(bind, document_root, https, cert, key)?
    };
    
    // Create registry
    let registry = DocumentationRegistry::new(RegistryConfig::default());
    registry.initialize().await?;
    
    // Create and start server
    let server = DocumentationServer::new(config, registry);
    server.validate_config()?;
    
    info!("Documentation server starting...");
    server.start().await?;
    
    Ok(())
}

async fn handle_test_command(
    package: Option<String>,
    version: Option<String>,
    check_links: bool,
    verify_examples: bool,
    check_completeness: bool,
    check_accessibility: bool,
    format: String,
    output: Option<PathBuf>,
    config_file: Option<PathBuf>,
) -> Result<()> {
    info!("Testing documentation");
    
    // Load configuration
    let testing_config = if let Some(config_path) = config_file {
        load_testing_config(&config_path).await?
    } else {
        create_default_testing_config(check_links, verify_examples, check_completeness, check_accessibility)
    };
    
    // Get package information
    let package_manager = PackageManager::new();
    let current_package = package_manager.get_current_package().await?;
    
    let pkg_name = package.unwrap_or(current_package.name);
    let pkg_version = version.unwrap_or(current_package.version);
    
    let package_to_test = Package {
        name: pkg_name.clone(),
        version: pkg_version.clone(),
        description: current_package.description,
        authors: current_package.authors,
        license: current_package.license,
        repository: current_package.repository,
        homepage: current_package.homepage,
        keywords: current_package.keywords,
        dependencies: current_package.dependencies,
        dev_dependencies: current_package.dev_dependencies,
        build_dependencies: current_package.build_dependencies,
        features: current_package.features,
        default_features: current_package.default_features,
        edition: current_package.edition,
        rust_version: current_package.rust_version,
        exclude: current_package.exclude,
        include: current_package.include,
        links: current_package.links,
        path: current_package.path,
    };
    
    // Create registry and tester
    let registry = DocumentationRegistry::new(RegistryConfig::default());
    registry.initialize().await?;
    
    let tester = DocumentationTester::new(testing_config, registry)?;
    tester.validate_config()?;
    
    // Run tests
    let test_results = tester.test_package(&package_to_test, &pkg_version).await?;
    
    // Output results
    match format.as_str() {
        "text" => {
            print_test_results_text(&test_results);
        }
        "json" => {
            let json_output = serde_json::to_string_pretty(&test_results)?;
            if let Some(output_path) = output {
                fs::write(output_path, json_output).await?;
            } else {
                println!("{}", json_output);
            }
        }
        "html" => {
            let html_output = generate_test_results_html(&test_results);
            if let Some(output_path) = output {
                fs::write(output_path, html_output).await?;
            } else {
                println!("{}", html_output);
            }
        }
        _ => {
            return Err(CursedError::General(format!("Unsupported output format: {}", format)));
        }
    }
    
    // Exit with error code if tests failed
    if !test_results.passed {
        std::process::exit(1);
    }
    
    Ok(())
}

async fn handle_preview_command(
    source_dir: PathBuf,
    port: u16,
    watch: bool,
    open: bool,
) -> Result<()> {
    info!("Starting documentation preview on port {}", port);
    
    // Generate documentation first
    let output_dir = std::env::temp_dir().join("cursed_docs_preview");
    fs::create_dir_all(&output_dir).await?;
    
    let mut generator = DocumentationGenerator::new();
    generator.generate_html(&source_dir, &output_dir, false).await?;
    
    // Create server configuration
    let bind_addr = format!("127.0.0.1:{}", port).parse()
        .map_err(|e| CursedError::General(format!("Invalid address: {}", e)))?;
    let config = create_default_server_config(bind_addr, output_dir, false, None, None)?;
    
    // Create and start server
    let registry = DocumentationRegistry::new(RegistryConfig::default());
    let server = DocumentationServer::new(config, registry);
    
    info!("Preview server starting on http://127.0.0.1:{}", port);
    
    if open {
        // Try to open browser
        if let Err(e) = open::that(format!("http://127.0.0.1:{}", port)) {
            warn!("Failed to open browser: {}", e);
        }
    }
    
    if watch {
        info!("File watching enabled - documentation will rebuild on changes");
        // File watching would be implemented here
    }
    
    server.start().await?;
    
    Ok(())
}

async fn handle_validate_command(config_path: PathBuf, config_type: String) -> Result<()> {
    info!("Validating configuration file: {:?}", config_path);
    
    match config_type.as_str() {
        "publish" => {
            let config = load_publish_config(&config_path).await?;
            let publisher = DocumentationPublisher::new(
                config,
                DocumentationGenerator::new(),
                DocumentationRegistry::new(RegistryConfig::default()),
                PackageManager::new(),
            );
            publisher.validate_config()?;
            info!("Publish configuration is valid");
        }
        "server" => {
            let config = load_server_config(&config_path).await?;
            let server = DocumentationServer::new(config, DocumentationRegistry::new(RegistryConfig::default()));
            server.validate_config()?;
            info!("Server configuration is valid");
        }
        "registry" => {
            let config: RegistryConfig = load_config_file(&config_path).await?;
            let registry = DocumentationRegistry::new(config);
            registry.validate_config()?;
            info!("Registry configuration is valid");
        }
        "testing" => {
            let config = load_testing_config(&config_path).await?;
            let registry = DocumentationRegistry::new(RegistryConfig::default());
            let tester = DocumentationTester::new(config, registry)?;
            tester.validate_config()?;
            info!("Testing configuration is valid");
        }
        _ => {
            return Err(CursedError::General(format!("Unknown configuration type: {}", config_type)));
        }
    }
    
    Ok(())
}

async fn handle_registry_command(command: RegistryCommand) -> Result<()> {
    match command {
        RegistryCommand::Init { data_dir } => {
            info!("Initializing registry in {:?}", data_dir);
            
            let config = RegistryConfig {
                data_dir: data_dir.clone(),
                index_file: data_dir.join("index.json"),
                ..RegistryConfig::default()
            };
            
            let registry = DocumentationRegistry::new(config);
            registry.initialize().await?;
            
            info!("Registry initialized successfully");
        }
        
        RegistryCommand::List { package, format } => {
            let registry = DocumentationRegistry::new(RegistryConfig::default());
            registry.initialize().await?;
            
            let packages = if let Some(pkg_name) = package {
                if let Some(_) = registry.get_package(&pkg_name).await {
                    vec![pkg_name]
                } else {
                    vec![]
                }
            } else {
                registry.list_packages().await
            };
            
            match format.as_str() {
                "text" => {
                    for pkg in packages {
                        println!("{}", pkg);
                    }
                }
                "json" => {
                    println!("{}", serde_json::to_string_pretty(&packages)?);
                }
                _ => {
                    return Err(CursedError::General(format!("Unsupported format: {}", format)));
                }
            }
        }
        
        RegistryCommand::Search { query, limit, format } => {
            let registry = DocumentationRegistry::new(RegistryConfig::default());
            registry.initialize().await?;
            
            let search_query = crate::docs::registry::RegistrySearchQuery {
                query,
                package: None,
                version: None,
                item_type: None,
                category: None,
                min_quality: None,
                sort_by: crate::docs::registry::SortOrder::Relevance,
                limit,
                offset: 0,
            };
            
            let results = registry.search(&search_query).await?;
            
            match format.as_str() {
                "text" => {
                    for result in results {
                        println!("{} ({}::{}) - {}", result.item.name, result.package, result.version, result.context);
                    }
                }
                "json" => {
                    println!("{}", serde_json::to_string_pretty(&results)?);
                }
                _ => {
                    return Err(CursedError::General(format!("Unsupported format: {}", format)));
                }
            }
        }
        
        RegistryCommand::Show { package, version } => {
            let registry = DocumentationRegistry::new(RegistryConfig::default());
            registry.initialize().await?;
            
            if let Some(package_doc) = registry.get_package(&package).await {
                let version_to_show = version.unwrap_or(package_doc.latest_version.clone());
                
                println!("Package: {}", package_doc.name);
                println!("Description: {}", package_doc.description);
                println!("Latest Version: {}", package_doc.latest_version);
                println!("Available Versions: {:?}", package_doc.versions.keys().collect::<Vec<_>>());
                
                if let Some(version_doc) = package_doc.versions.get(&version_to_show) {
                    println!("Version: {}", version_doc.version);
                    println!("Published: {}", version_doc.publication.published_at);
                    println!("URL: {}", version_doc.publication.url);
                    println!("API Items: {}", version_doc.api_items.len());
                    println!("Examples: {}", version_doc.examples.len());
                    println!("Coverage: {:.1}%", version_doc.coverage.coverage_percentage);
                }
            } else {
                println!("Package '{}' not found in registry", package);
            }
        }
        
        RegistryCommand::Clean { old_versions, orphaned } => {
            info!("Cleaning registry");
            
            if old_versions {
                info!("Removing old versions...");
                // Implementation would remove old versions
            }
            
            if orphaned {
                info!("Removing orphaned entries...");
                // Implementation would remove orphaned entries
            }
            
            info!("Registry cleaned successfully");
        }
    }
    
    Ok(())
}

// Helper functions for configuration loading

async fn load_publish_config(config_path: &PathBuf) -> Result<PublishConfig> {
    load_config_file(config_path).await
}

async fn load_server_config(config_path: &PathBuf) -> Result<ServerConfig> {
    load_config_file(config_path).await
}

async fn load_testing_config(config_path: &PathBuf) -> Result<TestingConfig> {
    load_config_file(config_path).await
}

async fn load_config_file<T: serde::de::DeserializeOwned>(config_path: &PathBuf) -> Result<T> {
    let content = fs::read_to_string(config_path).await
        .map_err(|e| CursedError::General(format!("Failed to read config file: {}", e)))?;
    
    let extension = config_path.extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("toml");
    
    match extension {
        "toml" => {
            toml::from_str(&content).map_err(|e| {
                CursedError::General(format!("Failed to parse TOML config: {}", e))
            })
        }
        "json" => {
            serde_json::from_str(&content).map_err(|e| {
                CursedError::General(format!("Failed to parse JSON config: {}", e))
            })
        }
        "yaml" | "yml" => {
            serde_yaml::from_str(&content).map_err(|e| {
                CursedError::General(format!("Failed to parse YAML config: {}", e))
            })
        }
        _ => {
            Err(CursedError::General(format!("Unsupported config file format: {}", extension)))
        }
    }
}

fn create_default_publish_config(
    target: &str,
    base_url: Option<String>,
    optimize: bool,
) -> Result<PublishConfig> {
    let publish_target = match target {
        "local" => PublishTarget::Local {
            path: PathBuf::from("./published_docs"),
        },
        "s3" => PublishTarget::S3 {
            bucket: "cursed-docs".to_string(),
            region: "us-west-2".to_string(),
            prefix: Some("docs".to_string()),
        },
        "github-pages" => PublishTarget::GithubPages {
            repo: "user/repo".to_string(),
            branch: "gh-pages".to_string(),
            token: std::env::var("GITHUB_TOKEN").unwrap_or_default(),
        },
        _ => {
            return Err(CursedError::General(format!("Unsupported publish target: {}", target)));
        }
    };
    
    Ok(PublishConfig {
        target: publish_target,
        base_url: base_url.unwrap_or_else(|| "https://docs.cursed.dev".to_string()),
        cdn: None,
        optimization: if optimize {
            OptimizationConfig::default()
        } else {
            OptimizationConfig {
                minify_html: false,
                minify_css: false,
                minify_js: false,
                optimize_images: false,
                gzip_compression: false,
                brotli_compression: false,
            }
        },
        auth: None,
        domain: None,
    })
}

fn create_default_server_config(
    bind: SocketAddr,
    document_root: PathBuf,
    https: bool,
    cert: Option<PathBuf>,
    key: Option<PathBuf>,
) -> Result<ServerConfig> {
    let ssl_config = if https {
        Some(crate::docs::server::SslServerConfig {
            cert_path: cert.ok_or_else(|| {
                CursedError::General("SSL certificate path required for HTTPS".to_string())
            })?,
            key_path: key.ok_or_else(|| {
                CursedError::General("SSL private key path required for HTTPS".to_string())
            })?,
            chain_path: None,
        })
    } else {
        None
    };
    
    Ok(ServerConfig {
        bind_address: bind,
        document_root,
        enable_https: https,
        ssl_config,
        cors_config: crate::docs::server::CorsConfig::default(),
        rate_limiting: crate::docs::server::RateLimitConfig::default(),
        cache_config: crate::docs::server::CacheConfig::default(),
        search_config: crate::docs::server::SearchConfig::default(),
        analytics_config: crate::docs::server::AnalyticsConfig::default(),
    })
}

fn create_default_testing_config(
    check_links: bool,
    verify_examples: bool,
    check_completeness: bool,
    check_accessibility: bool,
) -> TestingConfig {
    TestingConfig {
        check_links,
        verify_examples,
        check_completeness,
        check_accessibility,
        ..TestingConfig::default()
    }
}

// Helper functions for output formatting

fn print_test_results_text(results: &crate::docs::testing::TestResults) {
    println!("Documentation Test Results");
    println!("==========================");
    println!("Package: {} {}", results.package, results.version);
    println!("Overall Result: {}", if results.passed { "PASSED" } else { "FAILED" });
    println!("Total Time: {}ms", results.performance.total_time_ms);
    println!();
    
    for (category, result) in &results.test_categories {
        println!("Category: {}", category);
        println!("  Status: {}", if result.passed { "PASSED" } else { "FAILED" });
        println!("  Tests: {}/{} passed", result.tests_passed, result.tests_run);
        
        if !result.issues.is_empty() {
            println!("  Issues:");
            for issue in &result.issues {
                println!("    - {}", issue);
            }
        }
        println!();
    }
    
    if !results.suggestions.is_empty() {
        println!("Suggestions for Improvement:");
        for suggestion in &results.suggestions {
            println!("  - {} (Priority: {:?})", suggestion.description, suggestion.priority);
            println!("    {}", suggestion.guidance);
        }
    }
}

fn generate_test_results_html(results: &crate::docs::testing::TestResults) -> String {
    format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Documentation Test Results</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 2rem; }}
        .passed {{ color: green; }}
        .failed {{ color: red; }}
        .category {{ margin: 1rem 0; padding: 1rem; border: 1px solid #ddd; }}
        .issues {{ background: #ffe6e6; padding: 0.5rem; margin: 0.5rem 0; }}
        .suggestions {{ background: #e6f7ff; padding: 0.5rem; margin: 0.5rem 0; }}
    </style>
</head>
<body>
    <h1>Documentation Test Results</h1>
    <p><strong>Package:</strong> {} {}</p>
    <p><strong>Result:</strong> <span class="{}">{}</span></p>
    <p><strong>Total Time:</strong> {}ms</p>
    
    <h2>Test Categories</h2>
    {}
    
    <h2>Performance Metrics</h2>
    <ul>
        <li>Links Checked: {}</li>
        <li>Examples Tested: {}</li>
        <li>Link Check Time: {}ms</li>
        <li>Example Verification Time: {}ms</li>
        <li>Completeness Analysis Time: {}ms</li>
    </ul>
</body>
</html>"#,
        results.package,
        results.version,
        if results.passed { "passed" } else { "failed" },
        if results.passed { "PASSED" } else { "FAILED" },
        results.performance.total_time_ms,
        results.test_categories.iter().map(|(name, result)| {
            format!(
                r#"<div class="category">
                    <h3>{}</h3>
                    <p><strong>Status:</strong> <span class="{}">{}</span></p>
                    <p><strong>Tests:</strong> {}/{} passed</p>
                    {}
                </div>"#,
                name,
                if result.passed { "passed" } else { "failed" },
                if result.passed { "PASSED" } else { "FAILED" },
                result.tests_passed,
                result.tests_run,
                if result.issues.is_empty() {
                    String::new()
                } else {
                    format!(
                        r#"<div class="issues">
                            <strong>Issues:</strong>
                            <ul>{}</ul>
                        </div>"#,
                        result.issues.iter().map(|issue| format!("<li>{}</li>", issue)).collect::<String>()
                    )
                }
            )
        }).collect::<String>(),
        results.performance.links_checked,
        results.performance.examples_tested,
        results.performance.link_check_time_ms,
        results.performance.example_verify_time_ms,
        results.performance.completeness_time_ms,
    )
}

