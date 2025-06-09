//! Configuration system for CURSED documentation generator
//!
//! Provides comprehensive configuration loading from TOML, JSON, and YAML files,
//! environment variables, and command-line arguments with proper precedence.

use crate::docs::{DocError, DocResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// File-based configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocConfigFile {
    /// Package information
    pub package: PackageConfig,
    
    /// Generation settings
    pub generation: GenerationConfig,
    
    /// HTML output settings
    pub html: HtmlConfig,
    
    /// Server settings for serve mode
    pub server: ServerConfig,
    
    /// File processing settings
    pub files: FileConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageConfig {
    /// Package name
    pub name: String,
    
    /// Package version
    pub version: String,
    
    /// Package description
    pub description: Option<String>,
    
    /// Package authors
    pub authors: Option<Vec<String>>,
    
    /// Package homepage
    pub homepage: Option<String>,
    
    /// Package repository
    pub repository: Option<String>,
    
    /// Package license
    pub license: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationConfig {
    /// Source directories to scan
    pub source_dirs: Vec<String>,
    
    /// Output directory
    pub output_dir: String,
    
    /// Include private items
    pub include_private: bool,
    
    /// Generate search functionality
    pub enable_search: bool,
    
    /// Maximum directory scanning depth
    pub max_depth: Option<usize>,
    
    /// File patterns to exclude
    pub exclude_patterns: Vec<String>,
    
    /// Number of parallel jobs
    pub parallel_jobs: Option<usize>,
    
    /// Generate sitemap
    pub sitemap_base_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HtmlConfig {
    /// Custom CSS file
    pub custom_css: Option<String>,
    
    /// Custom JavaScript file
    pub custom_js: Option<String>,
    
    /// Theme name
    pub theme: Option<String>,
    
    /// Syntax highlighting theme
    pub syntax_theme: Option<String>,
    
    /// Show line numbers in code blocks
    pub show_line_numbers: bool,
    
    /// Enable code folding
    pub enable_code_folding: bool,
    
    /// Custom footer content
    pub footer: Option<String>,
    
    /// Custom head content (additional meta tags, etc.)
    pub head_content: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Default host for server mode
    pub host: String,
    
    /// Default port for server mode
    pub port: u16,
    
    /// Enable watch mode by default
    pub watch_by_default: bool,
    
    /// Watch polling interval in milliseconds
    pub watch_interval: u64,
    
    /// Auto-open browser
    pub auto_open: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileConfig {
    /// File extensions to include
    pub extensions: Vec<String>,
    
    /// Directories to ignore
    pub ignore_dirs: Vec<String>,
    
    /// Files to ignore
    pub ignore_files: Vec<String>,
    
    /// Follow symbolic links
    pub follow_symlinks: bool,
    
    /// Case sensitive file matching
    pub case_sensitive: bool,
}

impl Default for DocConfigFile {
    fn default() -> Self {
        Self {
            package: PackageConfig {
                name: "CURSED Package".to_string(),
                version: "1.0.0".to_string(),
                description: None,
                authors: None,
                homepage: None,
                repository: None,
                license: None,
            },
            generation: GenerationConfig {
                source_dirs: vec!["src".to_string()],
                output_dir: "docs/html".to_string(),
                include_private: false,
                enable_search: true,
                max_depth: None,
                exclude_patterns: vec![
                    "test*".to_string(),
                    "*_test.csd".to_string(),
                    "tmp/*".to_string(),
                ],
                parallel_jobs: None,
                sitemap_base_url: None,
            },
            html: HtmlConfig {
                custom_css: None,
                custom_js: None,
                theme: Some("default".to_string()),
                syntax_theme: Some("github".to_string()),
                show_line_numbers: true,
                enable_code_folding: true,
                footer: None,
                head_content: None,
            },
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 8080,
                watch_by_default: true,
                watch_interval: 1000,
                auto_open: false,
            },
            files: FileConfig {
                extensions: vec!["csd".to_string()],
                ignore_dirs: vec![
                    "target".to_string(),
                    "build".to_string(),
                    ".git".to_string(),
                    "node_modules".to_string(),
                    "tmp".to_string(),
                ],
                ignore_files: vec![],
                follow_symlinks: false,
                case_sensitive: true,
            },
        }
    }
}

/// Configuration loader that handles multiple formats and sources
pub struct ConfigLoader {
    environment_prefix: String,
}

impl ConfigLoader {
    /// Create a new configuration loader
    pub fn new() -> Self {
        Self {
            environment_prefix: "CURSED_DOC_".to_string(),
        }
    }

    /// Load configuration from a specific file
    pub fn load_from_file(&self, path: &str) -> DocResult<DocConfigFile> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| DocError::IoError(format!("Failed to read config file {}: {}", path, e)))?;

        let extension = std::path::Path::new(path)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("toml");

        self.parse_content(&content, extension)
    }

    /// Load default configuration from standard locations
    pub fn load_default(&self) -> DocResult<DocConfigFile> {
        let default_paths = [
            ".cursed-doc.toml",
            ".cursed-doc.json",
            ".cursed-doc.yaml",
            ".cursed-doc.yml",
            "cursed-doc.toml",
            "cursed-doc.json",
            "cursed-doc.yaml",
            "cursed-doc.yml",
        ];

        for path in &default_paths {
            if std::path::Path::new(path).exists() {
                return self.load_from_file(path);
            }
        }

        Err(DocError::IoError("No default configuration file found".to_string()))
    }

    /// Parse configuration content based on format
    fn parse_content(&self, content: &str, format: &str) -> DocResult<DocConfigFile> {
        match format {
            "json" => {
                serde_json::from_str(content)
                    .map_err(|e| DocError::ParseError(format!("Invalid JSON configuration: {}", e)))
            }
            "yaml" | "yml" => {
                serde_yaml::from_str(content)
                    .map_err(|e| DocError::ParseError(format!("Invalid YAML configuration: {}", e)))
            }
            _ => {
                toml::from_str(content)
                    .map_err(|e| DocError::ParseError(format!("Invalid TOML configuration: {}", e)))
            }
        }
    }

    /// Load configuration from environment variables
    pub fn load_from_env(&self) -> DocResult<HashMap<String, String>> {
        let mut env_config = HashMap::new();

        for (key, value) in std::env::vars() {
            if key.starts_with(&self.environment_prefix) {
                let config_key = key
                    .strip_prefix(&self.environment_prefix)
                    .unwrap()
                    .to_lowercase();
                env_config.insert(config_key, value);
            }
        }

        Ok(env_config)
    }

    /// Merge configuration with environment variable overrides
    pub fn merge_with_env(&self, mut config: DocConfigFile) -> DocResult<DocConfigFile> {
        let env_config = self.load_from_env()?;

        // Apply environment variable overrides
        if let Some(value) = env_config.get("package_name") {
            config.package.name = value.clone();
        }
        if let Some(value) = env_config.get("package_version") {
            config.package.version = value.clone();
        }
        if let Some(value) = env_config.get("package_description") {
            config.package.description = Some(value.clone());
        }
        if let Some(value) = env_config.get("output_dir") {
            config.generation.output_dir = value.clone();
        }
        if let Some(value) = env_config.get("include_private") {
            config.generation.include_private = value.parse().unwrap_or(false);
        }
        if let Some(value) = env_config.get("enable_search") {
            config.generation.enable_search = value.parse().unwrap_or(true);
        }
        if let Some(value) = env_config.get("server_host") {
            config.server.host = value.clone();
        }
        if let Some(value) = env_config.get("server_port") {
            if let Ok(port) = value.parse() {
                config.server.port = port;
            }
        }

        Ok(config)
    }

    /// Validate configuration
    pub fn validate(&self, config: &DocConfigFile) -> DocResult<()> {
        // Validate source directories exist
        for source_dir in &config.generation.source_dirs {
            let path = std::path::Path::new(source_dir);
            if !path.exists() {
                return Err(DocError::IoError(format!(
                    "Source directory does not exist: {}",
                    source_dir
                )));
            }
            if !path.is_dir() {
                return Err(DocError::IoError(format!(
                    "Source path is not a directory: {}",
                    source_dir
                )));
            }
        }

        // Validate optional file paths
        if let Some(css_path) = &config.html.custom_css {
            let path = std::path::Path::new(css_path);
            if !path.exists() {
                return Err(DocError::IoError(format!(
                    "Custom CSS file does not exist: {}",
                    css_path
                )));
            }
        }

        if let Some(js_path) = &config.html.custom_js {
            let path = std::path::Path::new(js_path);
            if !path.exists() {
                return Err(DocError::IoError(format!(
                    "Custom JavaScript file does not exist: {}",
                    js_path
                )));
            }
        }

        // Validate port range
        if config.server.port == 0 || config.server.port > 65535 {
            return Err(DocError::ParseError(format!(
                "Invalid server port: {}",
                config.server.port
            )));
        }

        // Validate parallel jobs
        if let Some(jobs) = config.generation.parallel_jobs {
            if jobs == 0 {
                return Err(DocError::ParseError(
                    "Parallel jobs must be greater than 0".to_string()
                ));
            }
        }

        Ok(())
    }
}

/// CLI-specific configuration structure
#[derive(Debug, Clone)]
pub struct CliConfig {
    pub config_file: Option<String>,
    pub generate_config: Option<String>,
    pub verbose: u8,
    pub quiet: bool,
    pub serve: bool,
    pub watch: bool,
    pub open: bool,
    pub clean: bool,
    pub stats: bool,
    pub output_format: String,
}

impl Default for CliConfig {
    fn default() -> Self {
        Self {
            config_file: None,
            generate_config: None,
            verbose: 0,
            quiet: false,
            serve: false,
            watch: false,
            open: false,
            clean: false,
            stats: false,
            output_format: "human".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_default_config() {
        let config = DocConfigFile::default();
        assert_eq!(config.package.name, "CURSED Package");
        assert_eq!(config.package.version, "1.0.0");
        assert_eq!(config.generation.output_dir, "docs/html");
        assert!(config.generation.enable_search);
        assert!(!config.generation.include_private);
    }

    #[test]
    fn test_config_loader_creation() {
        let loader = ConfigLoader::new();
        assert_eq!(loader.environment_prefix, "CURSED_DOC_");
    }

    #[test]
    fn test_config_serialization_toml() {
        let config = DocConfigFile::default();
        let toml_content = toml::to_string_pretty(&config).unwrap();
        assert!(toml_content.contains("[package]"));
        assert!(toml_content.contains("[generation]"));
        assert!(toml_content.contains("[html]"));
        assert!(toml_content.contains("[server]"));
        assert!(toml_content.contains("[files]"));
    }

    #[test]
    fn test_config_serialization_json() {
        let config = DocConfigFile::default();
        let json_content = serde_json::to_string_pretty(&config).unwrap();
        assert!(json_content.contains("\"package\""));
        assert!(json_content.contains("\"generation\""));
        assert!(json_content.contains("\"html\""));
        assert!(json_content.contains("\"server\""));
        assert!(json_content.contains("\"files\""));
    }

    #[test]
    fn test_config_serialization_yaml() {
        let config = DocConfigFile::default();
        let yaml_content = serde_yaml::to_string(&config).unwrap();
        assert!(yaml_content.contains("package:"));
        assert!(yaml_content.contains("generation:"));
        assert!(yaml_content.contains("html:"));
        assert!(yaml_content.contains("server:"));
        assert!(yaml_content.contains("files:"));
    }

    #[test]
    fn test_config_file_operations() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test_config.toml");
        
        let config = DocConfigFile::default();
        let content = toml::to_string_pretty(&config).unwrap();
        std::fs::write(&config_path, content).unwrap();
        
        let loader = ConfigLoader::new();
        let loaded_config = loader.load_from_file(config_path.to_str().unwrap()).unwrap();
        
        assert_eq!(loaded_config.package.name, config.package.name);
        assert_eq!(loaded_config.package.version, config.package.version);
    }

    #[test]
    fn test_environment_variable_loading() {
        std::env::set_var("CURSED_DOC_PACKAGE_NAME", "Test Package");
        std::env::set_var("CURSED_DOC_SERVER_PORT", "9090");
        
        let loader = ConfigLoader::new();
        let env_config = loader.load_from_env().unwrap();
        
        assert_eq!(env_config.get("package_name"), Some(&"Test Package".to_string()));
        assert_eq!(env_config.get("server_port"), Some(&"9090".to_string()));
        
        // Clean up
        std::env::remove_var("CURSED_DOC_PACKAGE_NAME");
        std::env::remove_var("CURSED_DOC_SERVER_PORT");
    }

    #[test]
    fn test_config_validation() {
        let loader = ConfigLoader::new();
        let mut config = DocConfigFile::default();
        
        // Test valid config
        config.generation.source_dirs = vec!["src".to_string()];
        std::fs::create_dir_all("src").ok();
        assert!(loader.validate(&config).is_ok());
        
        // Test invalid source directory
        config.generation.source_dirs = vec!["nonexistent".to_string()];
        assert!(loader.validate(&config).is_err());
        
        // Test invalid port
        config.server.port = 0;
        assert!(loader.validate(&config).is_err());
        
        // Clean up
        std::fs::remove_dir_all("src").ok();
    }
}
