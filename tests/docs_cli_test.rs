//! Comprehensive tests for CURSED documentation CLI
//!
//! Tests command-line interface functionality, configuration loading,
//! and integration with the documentation generation system.

use cursed::docs::{
    config::{ConfigLoader, DocConfigFile, CliConfig},
    DocConfig, DocumentationGenerator, DocError,
};
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;
use clap::{Command, Arg, ArgAction, value_parser};

#[test]
fn test_cli_creation() {
    let cli = create_test_cli();
    assert_eq!(cli.get_name(), "cursed-doc");
}

#[test]
fn test_default_arguments() {
    let cli = create_test_cli();
    let matches = cli.try_get_matches_from(&["cursed-doc"]).unwrap();
    
    assert_eq!(matches.get_one::<String>("output").unwrap(), "docs/html");
    assert_eq!(matches.get_one::<String>("package-name").unwrap(), "CURSED Package");
    assert_eq!(matches.get_one::<String>("package-version").unwrap(), "1.0.0");
    assert!(!matches.get_flag("include-private"));
    assert!(!matches.get_flag("no-search"));
    assert!(!matches.get_flag("serve"));
    assert!(!matches.get_flag("watch"));
    assert!(!matches.get_flag("open"));
}

#[test]
fn test_verbose_flag() {
    let cli = create_test_cli();
    let matches = cli.try_get_matches_from(&["cursed-doc", "-vv"]).unwrap();
    
    assert_eq!(matches.get_count("verbose"), 2);
}

#[test]
fn test_serve_options() {
    let cli = create_test_cli();
    let matches = cli.try_get_matches_from(&[
        "cursed-doc",
        "--serve",
        "--port", "9090",
        "--host", "0.0.0.0",
        "--watch",
        "--open"
    ]).unwrap();
    
    assert!(matches.get_flag("serve"));
    assert!(matches.get_flag("watch"));
    assert!(matches.get_flag("open"));
    assert_eq!(matches.get_one::<u16>("port").unwrap(), &9090);
    assert_eq!(matches.get_one::<String>("host").unwrap(), "0.0.0.0");
}

#[test]
fn test_output_format_options() {
    let cli = create_test_cli();
    
    // Test HTML format
    let matches = cli.clone().try_get_matches_from(&[
        "cursed-doc", "--output-format", "html"
    ]).unwrap();
    assert_eq!(matches.get_one::<String>("output-format").unwrap(), "html");
    
    // Test JSON format
    let matches = cli.clone().try_get_matches_from(&[
        "cursed-doc", "--output-format", "json"
    ]).unwrap();
    assert_eq!(matches.get_one::<String>("output-format").unwrap(), "json");
    
    // Test Markdown format
    let matches = cli.try_get_matches_from(&[
        "cursed-doc", "--output-format", "markdown"
    ]).unwrap();
    assert_eq!(matches.get_one::<String>("output-format").unwrap(), "markdown");
}

#[test]
fn test_parallel_jobs_option() {
    let cli = create_test_cli();
    let matches = cli.try_get_matches_from(&[
        "cursed-doc", "--jobs", "8"
    ]).unwrap();
    
    assert_eq!(matches.get_one::<usize>("jobs").unwrap(), &8);
}

#[test]
fn test_configuration_file_options() {
    let cli = create_test_cli();
    let matches = cli.try_get_matches_from(&[
        "cursed-doc",
        "--config-file", "custom.toml",
        "--generate-config", "output.toml"
    ]).unwrap();
    
    assert_eq!(matches.get_one::<String>("config-file").unwrap(), "custom.toml");
    assert_eq!(matches.get_one::<String>("generate-config").unwrap(), "output.toml");
}

#[test]
fn test_custom_config() {
    let cli = create_test_cli();
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
        "--exclude", "example",
        "--stats"
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
    assert!(matches.get_flag("stats"));
    assert_eq!(matches.get_one::<usize>("max-depth").unwrap(), &5);
    
    let excludes: Vec<&String> = matches.get_many("exclude").unwrap().collect();
    assert_eq!(excludes.len(), 2);
    assert!(excludes.contains(&&"test".to_string()));
    assert!(excludes.contains(&&"example".to_string()));
}

#[test]
fn test_config_file_loading_toml() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("test.toml");
    
    let config_content = r#""
[package]
name = "Test Package"
version = "1.2.3"
description = "A test package"

[generation]
source_dirs = ["src", "lib"]
output_dir = "output"
include_private = true
enable_search = false

[server]
host = "0.0.0.0"
port = 9000
"#";
    
    fs::write(&config_path, config_content).unwrap();
    
    let loader = ConfigLoader::new();
    let config = loader.load_from_file(config_path.to_str().unwrap()).unwrap();
    
    assert_eq!(config.package.name, "Test Package");
    assert_eq!(config.package.version, "1.2.3");
    assert_eq!(config.package.description, Some("A test package".to_string()));
    assert_eq!(config.generation.source_dirs, vec!["src", "lib"]);
    assert_eq!(config.generation.output_dir, "output");
    assert!(config.generation.include_private);
    assert!(!config.generation.enable_search);
    assert_eq!(config.server.host, "0.0.0.0");
    assert_eq!(config.server.port, 9000);
}

#[test]
fn test_config_file_loading_json() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("test.json");
    
    let config_content = r#""
{
  "package": {
    "name": "JSON Package",
    "version": "2.0.0",
    "description": "A JSON config test"
  },
  "generation": {
    "source_dirs": ["src"],
    "output_dir": "json_output",
    "include_private": false,
    "enable_search": true,
    "exclude_patterns": ["*_test.csd"]
  },
  "html": {
    "theme": "dark",
    "show_line_numbers": true
  },
  "server": {
    "host": "127.0.0.1",
    "port": 8080,
    "watch_by_default": true
  },
  "files": {
    "extensions": ["csd"],
    "ignore_dirs": ["target", "build"],
    "follow_symlinks": false,
    "case_sensitive": true
  }
}
"#";
    
    fs::write(&config_path, config_content).unwrap();
    
    let loader = ConfigLoader::new();
    let config = loader.load_from_file(config_path.to_str().unwrap()).unwrap();
    
    assert_eq!(config.package.name, "JSON Package");
    assert_eq!(config.package.version, "2.0.0");
    assert_eq!(config.generation.output_dir, "json_output");
    assert_eq!(config.generation.exclude_patterns, vec!["*_test.csd"]);
    assert_eq!(config.html.theme, Some("dark".to_string()));
    assert!(config.html.show_line_numbers);
    assert!(config.server.watch_by_default);
    assert_eq!(config.files.extensions, vec!["csd"]);
    assert!(config.files.case_sensitive);
}

#[test]
fn test_config_file_loading_yaml() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("test.yaml");
    
    let config_content = r#""
package:
  name: "YAML Package"
  version: "3.0.0"
  description: "A YAML config test"
  authors: ["Author 1", "Author 2"]

generation:
  source_dirs: ["src", "examples"]
  output_dir: "yaml_output"
  include_private: true
  enable_search: true
  parallel_jobs: 4

html:
  custom_css: "styles.css"
  syntax_theme: "monokai"
  enable_code_folding: false

server:
  host: "localhost"
  port: 3000
  auto_open: true
"#";
    
    fs::write(&config_path, config_content).unwrap();
    
    let loader = ConfigLoader::new();
    let config = loader.load_from_file(config_path.to_str().unwrap()).unwrap();
    
    assert_eq!(config.package.name, "YAML Package");
    assert_eq!(config.package.version, "3.0.0");
    assert_eq!(config.package.authors, Some(vec!["Author 1".to_string(), "Author 2".to_string()]));
    assert_eq!(config.generation.source_dirs, vec!["src", "examples"]);
    assert_eq!(config.generation.parallel_jobs, Some(4));
    assert_eq!(config.html.custom_css, Some("styles.css".to_string()));
    assert_eq!(config.html.syntax_theme, Some("monokai".to_string()));
    assert!(!config.html.enable_code_folding);
    assert_eq!(config.server.host, "localhost");
    assert_eq!(config.server.port, 3000);
    assert!(config.server.auto_open);
}

#[test]
fn test_environment_variable_loading() {
    // Set environment variables
    std::env::set_var("CURSED_DOC_PACKAGE_NAME", "Env Package");
    std::env::set_var("CURSED_DOC_OUTPUT_DIR", "env_output");
    std::env::set_var("CURSED_DOC_INCLUDE_PRIVATE", "true");
    std::env::set_var("CURSED_DOC_SERVER_PORT", "7070");
    
    let loader = ConfigLoader::new();
    let env_config = loader.load_from_env().unwrap();
    
    assert_eq!(env_config.get("package_name"), Some(&"Env Package".to_string()));
    assert_eq!(env_config.get("output_dir"), Some(&"env_output".to_string()));
    assert_eq!(env_config.get("include_private"), Some(&"true".to_string()));
    assert_eq!(env_config.get("server_port"), Some(&"7070".to_string()));
    
    // Test merge with environment
    let mut config = DocConfigFile::default();
    config = loader.merge_with_env(config).unwrap();
    
    assert_eq!(config.package.name, "Env Package");
    assert_eq!(config.generation.output_dir, "env_output");
    assert!(config.generation.include_private);
    assert_eq!(config.server.port, 7070);
    
    // Clean up
    std::env::remove_var("CURSED_DOC_PACKAGE_NAME");
    std::env::remove_var("CURSED_DOC_OUTPUT_DIR");
    std::env::remove_var("CURSED_DOC_INCLUDE_PRIVATE");
    std::env::remove_var("CURSED_DOC_SERVER_PORT");
}

#[test]
fn test_config_validation() {
    let temp_dir = TempDir::new().unwrap();
    let loader = ConfigLoader::new();
    
    // Create test source directory
    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).unwrap();
    
    let mut config = DocConfigFile::default();
    config.generation.source_dirs = vec![src_dir.to_string_lossy().to_string()];
    
    // Valid configuration should pass
    assert!(loader.validate(&config).is_ok());
    
    // Invalid source directory should fail
    config.generation.source_dirs = vec!["nonexistent".to_string()];
    assert!(loader.validate(&config).is_err());
    
    // Invalid port should fail
    config.generation.source_dirs = vec![src_dir.to_string_lossy().to_string()];
    config.server.port = 0;
    assert!(loader.validate(&config).is_err());
    
    // Invalid parallel jobs should fail
    config.server.port = 8080;
    config.generation.parallel_jobs = Some(0);
    assert!(loader.validate(&config).is_err());
}

#[test]
fn test_default_config_file_search() {
    let temp_dir = TempDir::new().unwrap();
    let original_dir = std::env::current_dir().unwrap();
    
    // Change to temp directory
    std::env::set_current_dir(&temp_dir).unwrap();
    
    // Create a default config file
    let config_content = toml::to_string_pretty(&DocConfigFile::default()).unwrap();
    fs::write(".cursed-doc.toml", config_content).unwrap();
    
    let loader = ConfigLoader::new();
    let config = loader.load_default();
    
    assert!(config.is_ok());
    
    // Restore original directory
    std::env::set_current_dir(original_dir).unwrap();
}

#[test]
fn test_config_generation() {
    let temp_dir = TempDir::new().unwrap();
    
    // Test TOML generation
    let toml_path = temp_dir.path().join("test.toml");
    let config = DocConfigFile::default();
    let content = toml::to_string_pretty(&config).unwrap();
    fs::write(&toml_path, content).unwrap();
    
    assert!(toml_path.exists());
    let content = fs::read_to_string(&toml_path).unwrap();
    assert!(content.contains("[package]"));
    assert!(content.contains("[generation]"));
    
    // Test JSON generation
    let json_path = temp_dir.path().join("test.json");
    let content = serde_json::to_string_pretty(&config).unwrap();
    fs::write(&json_path, content).unwrap();
    
    assert!(json_path.exists());
    let content = fs::read_to_string(&json_path).unwrap();
    assert!(content.contains("\"package\""));
    assert!(content.contains("\"generation\""));
    
    // Test YAML generation
    let yaml_path = temp_dir.path().join("test.yaml");
    let content = serde_yaml::to_string(&config).unwrap();
    fs::write(&yaml_path, content).unwrap();
    
    assert!(yaml_path.exists());
    let content = fs::read_to_string(&yaml_path).unwrap();
    assert!(content.contains("package:"));
    assert!(content.contains("generation:"));
}

#[test]
fn test_cli_config_structure() {
    let cli_config = CliConfig::default();
    
    assert_eq!(cli_config.output_format, "human");
    assert!(!cli_config.serve);
    assert!(!cli_config.watch);
    assert!(!cli_config.open);
    assert!(!cli_config.quiet);
    assert_eq!(cli_config.verbose, 0);
}

#[test]
fn test_invalid_config_handling() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("invalid.toml");
    
    // Invalid TOML content
    let invalid_content = r#""
[package
name = "Invalid"
"#";
    
    fs::write(&config_path, invalid_content).unwrap();
    
    let loader = ConfigLoader::new();
    let result = loader.load_from_file(config_path.to_str().unwrap());
    
    assert!(result.is_err());
    if let Err(DocError::ParseError(msg)) = result {
        assert!(msg.contains("Invalid TOML"));
    } else {
        panic!("Expected ParseError");
    }
}

/// Helper function to create test CLI
fn create_test_cli() -> Command {
    Command::new("cursed-doc")
        .version("1.0.0")
        .about("Test CLI for CURSED documentation generator")
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
            Arg::new("max-depth")
                .long("max-depth")
                .value_name("NUM")
                .help("Maximum directory scanning depth")
                .value_parser(value_parser!(usize))
        )
        .arg(
            Arg::new("exclude")
                .long("exclude")
                .value_name("PATTERN")
                .help("Exclude files matching pattern")
                .action(ArgAction::Append)
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
                .help("Serve documentation locally")
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
                .help("Watch for file changes")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("output-format")
                .long("output-format")
                .value_name("FORMAT")
                .help("Output format")
                .value_parser(["html", "json", "markdown"])
                .default_value("html")
        )
        .arg(
            Arg::new("config-file")
                .long("config-file")
                .value_name("FILE")
                .help("Configuration file")
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
                .help("Number of parallel jobs")
                .value_parser(value_parser!(usize))
                .default_value("0")
        )
        .arg(
            Arg::new("stats")
                .long("stats")
                .help("Show detailed statistics")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("open")
                .long("open")
                .help("Open in browser")
                .action(ArgAction::SetTrue)
        )
}
