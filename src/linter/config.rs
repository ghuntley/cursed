//! Configuration system for the CURSED linter
//!
//! This module provides comprehensive configuration management including
//! file-based configuration, rule customization, and environment variable
//! support. It supports multiple configuration file formats.

use crate::linter::rules::{RuleCategory, RuleSeverity};
use crate::linter::reporter::OutputFormat;
use crate::error::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{debug, info, warn};

/// Main linter configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LinterConfig {
    /// General linting options
    pub general: GeneralConfig,
    
    /// Rule-specific configuration
    pub rules: RuleConfigs,
    
    /// Output and reporting configuration
    pub output: OutputConfig,
    
    /// Auto-fix configuration
    pub auto_fix: bool,
    
    /// Minimum severity level to report
    #[serde(default = "default_min_severity")]
    pub min_severity: RuleSeverity,
    
    /// List of disabled rules
    pub disabled_rules: Option<Vec<String>>,
    
    /// Custom rule parameters
    #[serde(default)]
    pub rule_params: HashMap<String, serde_json::Value>,
}

/// General configuration options
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GeneralConfig {
    /// Maximum line length
    #[serde(default = "default_max_line_length")]
    pub max_line_length: usize,
    
    /// Indentation style (spaces or tabs)
    #[serde(default = "default_indent_style")]
    pub indent_style: IndentStyle,
    
    /// Number of spaces per indent level (when using spaces)
    #[serde(default = "default_indent_size")]
    pub indent_size: usize,
    
    /// Whether to enforce Gen Z naming conventions
    #[serde(default = "default_true")]
    pub enforce_genz_naming: bool,
    
    /// File extensions to process
    #[serde(default = "default_file_extensions")]
    pub file_extensions: Vec<String>,
    
    /// Patterns to ignore (glob patterns)
    #[serde(default)]
    pub ignore_patterns: Vec<String>,
}

/// Rule category configurations
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RuleConfigs {
    /// Style rule configuration
    #[serde(default)]
    pub style: CategoryConfig,
    
    /// Correctness rule configuration
    #[serde(default)]
    pub correctness: CategoryConfig,
    
    /// Performance rule configuration
    #[serde(default)]
    pub performance: CategoryConfig,
    
    /// Complexity rule configuration
    #[serde(default)]
    pub complexity: CategoryConfig,
    
    /// CURSED-specific rule configuration
    #[serde(default)]
    pub cursed_specific: CategoryConfig,
}

/// Configuration for a rule category
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CategoryConfig {
    /// Whether this category is enabled
    #[serde(default = "default_true")]
    pub enabled: bool,
    
    /// Default severity for rules in this category
    #[serde(default = "default_severity")]
    pub default_severity: RuleSeverity,
    
    /// Specific rule overrides
    #[serde(default)]
    pub rule_overrides: HashMap<String, RuleConfig>,
}

/// Configuration for a specific rule
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RuleConfig {
    /// Whether this rule is enabled
    pub enabled: bool,
    
    /// Severity level for this rule
    pub severity: RuleSeverity,
    
    /// Rule-specific parameters
    #[serde(default)]
    pub params: HashMap<String, serde_json::Value>,
}

/// Output configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OutputConfig {
    /// Output format
    #[serde(default)]
    pub format: OutputFormat,
    
    /// Whether to show rule names in output
    #[serde(default = "default_true")]
    pub show_rule_names: bool,
    
    /// Whether to show severity levels
    #[serde(default = "default_true")]
    pub show_severity: bool,
    
    /// Whether to show fix suggestions
    #[serde(default = "default_true")]
    pub show_suggestions: bool,
    
    /// Whether to use colors in output
    #[serde(default = "default_true")]
    pub use_colors: bool,
    
    /// Maximum number of issues to display per file
    pub max_issues_per_file: Option<usize>,
}

/// Indentation style options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum IndentStyle {
    #[serde(rename = "spaces")]
    Spaces,
    #[serde(rename = "tabs")]
    Tabs,
}

impl Default for LinterConfig {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            rules: RuleConfigs::default(),
            output: OutputConfig::default(),
            auto_fix: false,
            min_severity: RuleSeverity::Info,
            disabled_rules: None,
            rule_params: HashMap::new(),
        }
    }
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            max_line_length: default_max_line_length(),
            indent_style: default_indent_style(),
            indent_size: default_indent_size(),
            enforce_genz_naming: default_true(),
            file_extensions: default_file_extensions(),
            ignore_patterns: Vec::new(),
        }
    }
}

impl Default for RuleConfigs {
    fn default() -> Self {
        Self {
            style: CategoryConfig::default(),
            correctness: CategoryConfig::default(),
            performance: CategoryConfig::default(),
            complexity: CategoryConfig::default(),
            cursed_specific: CategoryConfig::default(),
        }
    }
}

impl Default for CategoryConfig {
    fn default() -> Self {
        Self {
            enabled: default_true(),
            default_severity: default_severity(),
            rule_overrides: HashMap::new(),
        }
    }
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            format: OutputFormat::Human,
            show_rule_names: default_true(),
            show_severity: default_true(),
            show_suggestions: default_true(),
            use_colors: default_true(),
            max_issues_per_file: None,
        }
    }
}

/// Configuration loader for different file formats
pub struct ConfigLoader;

impl ConfigLoader {
    /// Load configuration from a file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<LinterConfig, Error> {
        let path = path.as_ref();
        let content = fs::read_to_string(path)
            .map_err(|e| Error::IoError(e))?;
        
        let config = match path.extension().and_then(|ext| ext.to_str()) {
            Some("toml") => toml::from_str(&content)
                .map_err(|e| Error::from_str(&format!("TOML parse error: {}", e)))?,
            Some("json") => serde_json::from_str(&content)
                .map_err(|e| Error::from_str(&format!("JSON parse error: {}", e)))?,
            Some("yaml") | Some("yml") => serde_yaml::from_str(&content)
                .map_err(|e| Error::Configuration(format!("YAML parse error: {}", e)))?,
            _ => return Err(Error::Configuration(
                "Unsupported configuration file format. Use .toml, .json, or .yaml".to_string()
            )),
        };
        
        debug!("Loaded configuration from {}", path.display());
        Ok(config)
    }

    /// Load configuration from multiple sources with precedence
    pub fn load_with_precedence(
        config_file: Option<&Path>,
        env_vars: bool,
        cli_overrides: Option<&CliConfig>,
    ) -> Result<LinterConfig, Error> {
        let mut config = LinterConfig::default();
        
        // 1. Load from config file if provided
        if let Some(config_path) = config_file {
            if config_path.exists() {
                config = Self::load_from_file(config_path)?;
                info!("Loaded configuration from {}", config_path.display());
            } else {
                warn!("Config file not found: {}", config_path.display());
            }
        } else {
            // Look for default config files
            if let Some(default_config) = Self::find_default_config()? {
                config = default_config;
            }
        }
        
        // 2. Apply environment variable overrides
        if env_vars {
            Self::apply_env_overrides(&mut config)?;
        }
        
        // 3. Apply CLI overrides
        if let Some(cli_config) = cli_overrides {
            Self::apply_cli_overrides(&mut config, cli_config);
        }
        
        Ok(config)
    }

    /// Find default configuration file in current directory and parent directories
    fn find_default_config() -> Result<Option<LinterConfig>, Error> {
        let config_names = [
            ".cursed-lint.toml",
            ".cursed-lint.json", 
            ".cursed-lint.yaml",
            ".cursed-lint.yml",
            "cursed-lint.toml",
            "cursed-lint.json",
        ];
        
        let mut current_dir = std::env::current_dir()
            .map_err(|e| Error::IoError(e))?;
        
        loop {
            for config_name in &config_names {
                let config_path = current_dir.join(config_name);
                if config_path.exists() {
                    info!("Found default config: {}", config_path.display());
                    return Ok(Some(Self::load_from_file(&config_path)?));
                }
            }
            
            if !current_dir.pop() {
                break;
            }
        }
        
        debug!("No default configuration file found");
        Ok(None)
    }

    /// Apply environment variable overrides
    fn apply_env_overrides(config: &mut LinterConfig) -> Result<(), Error> {
        // Check for environment variables with CURSED_LINT_ prefix
        if let Ok(max_line_length) = std::env::var("CURSED_LINT_MAX_LINE_LENGTH") {
            config.general.max_line_length = max_line_length.parse()
                .map_err(|_| Error::Configuration("Invalid CURSED_LINT_MAX_LINE_LENGTH".to_string()))?;
        }
        
        if let Ok(indent_size) = std::env::var("CURSED_LINT_INDENT_SIZE") {
            config.general.indent_size = indent_size.parse()
                .map_err(|_| Error::Configuration("Invalid CURSED_LINT_INDENT_SIZE".to_string()))?;
        }
        
        if let Ok(severity) = std::env::var("CURSED_LINT_MIN_SEVERITY") {
            config.min_severity = match severity.to_lowercase().as_str() {
                "info" => RuleSeverity::Info,
                "warning" => RuleSeverity::Warning,
                "error" => RuleSeverity::Error,
                _ => return Err(Error::Configuration("Invalid CURSED_LINT_MIN_SEVERITY".to_string())),
            };
        }
        
        if let Ok(disabled) = std::env::var("CURSED_LINT_DISABLED_RULES") {
            let rules: Vec<String> = disabled.split(',').map(|s| s.trim().to_string()).collect();
            config.disabled_rules = Some(rules);
        }
        
        debug!("Applied environment variable overrides");
        Ok(())
    }

    /// Apply CLI configuration overrides
    fn apply_cli_overrides(config: &mut LinterConfig, cli_config: &CliConfig) {
        if let Some(max_line_length) = cli_config.max_line_length {
            config.general.max_line_length = max_line_length;
        }
        
        if let Some(indent_size) = cli_config.indent_size {
            config.general.indent_size = indent_size;
        }
        
        if let Some(min_severity) = cli_config.min_severity.clone() {
            config.min_severity = min_severity;
        }
        
        if let Some(format) = cli_config.output_format.clone() {
            config.output.format = format;
        }
        
        if cli_config.auto_fix {
            config.auto_fix = true;
        }
        
        if let Some(disabled_rules) = &cli_config.disabled_rules {
            config.disabled_rules = Some(disabled_rules.clone());
        }
        
        debug!("Applied CLI configuration overrides");
    }

    /// Save configuration to file
    pub fn save_to_file<P: AsRef<Path>>(config: &LinterConfig, path: P) -> Result<(), Error> {
        let path = path.as_ref();
        
        let content = match path.extension().and_then(|ext| ext.to_str()) {
            Some("toml") => toml::to_string_pretty(config)
                .map_err(|e| Error::Configuration(format!("TOML serialize error: {}", e)))?,
            Some("json") => serde_json::to_string_pretty(config)
                .map_err(|e| Error::Configuration(format!("JSON serialize error: {}", e)))?,
            Some("yaml") | Some("yml") => serde_yaml::to_string(config)
                .map_err(|e| Error::Configuration(format!("YAML serialize error: {}", e)))?,
            _ => return Err(Error::Configuration(
                "Unsupported configuration file format. Use .toml, .json, or .yaml".to_string()
            )),
        };
        
        fs::write(path, content)
            .map_err(|e| Error::IoError(e))?;
        
        info!("Saved configuration to {}", path.display());
        Ok(())
    }

    /// Generate a default configuration file
    pub fn generate_default_config<P: AsRef<Path>>(path: P) -> Result<(), Error> {
        let config = LinterConfig::default();
        Self::save_to_file(&config, path)
    }
}

/// CLI configuration overrides
#[derive(Debug, Clone, Default)]
pub struct CliConfig {
    pub max_line_length: Option<usize>,
    pub indent_size: Option<usize>,
    pub min_severity: Option<RuleSeverity>,
    pub output_format: Option<OutputFormat>,
    pub auto_fix: bool,
    pub disabled_rules: Option<Vec<String>>,
}

// Default value functions
fn default_max_line_length() -> usize { 100 }
fn default_indent_style() -> IndentStyle { IndentStyle::Spaces }
fn default_indent_size() -> usize { 4 }
fn default_true() -> bool { true }
fn default_severity() -> RuleSeverity { RuleSeverity::Warning }
fn default_min_severity() -> RuleSeverity { RuleSeverity::Info }
fn default_file_extensions() -> Vec<String> { vec!["csd".to_string()] }

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_default_config() {
        let config = LinterConfig::default();
        assert_eq!(config.general.max_line_length, 100);
        assert_eq!(config.general.indent_size, 4);
        assert!(config.general.enforce_genz_naming);
        assert!(!config.auto_fix);
    }

    #[test]
    fn test_toml_config_loading() {
        let toml_content = r#"
auto_fix = true
min_severity = "warning"

[general]
max_line_length = 120
indent_size = 2
enforce_genz_naming = false

[output]
format = "json"
use_colors = false
"#;
        
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(toml_content.as_bytes()).unwrap();
        
        let config = ConfigLoader::load_from_file(temp_file.path()).unwrap();
        assert!(config.auto_fix);
        assert_eq!(config.min_severity, RuleSeverity::Warning);
        assert_eq!(config.general.max_line_length, 120);
        assert_eq!(config.general.indent_size, 2);
        assert!(!config.general.enforce_genz_naming);
        assert_eq!(config.output.format, OutputFormat::Json);
        assert!(!config.output.use_colors);
    }

    #[test]
    fn test_json_config_loading() {
        let json_content = r#"{
  "auto_fix": true,
  "min_severity": "error",
  "general": {
    "max_line_length": 80,
    "indent_style": "tabs"
  },
  "rules": {
    "style": {
      "enabled": true,
      "default_severity": "warning"
    }
  }
}"#;
        
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(json_content.as_bytes()).unwrap();
        
        let config = ConfigLoader::load_from_file(temp_file.path()).unwrap();
        assert!(config.auto_fix);
        assert_eq!(config.min_severity, RuleSeverity::Error);
        assert_eq!(config.general.max_line_length, 80);
        assert_eq!(config.general.indent_style, IndentStyle::Tabs);
        assert!(config.rules.style.enabled);
    }

    #[test]
    fn test_config_precedence() {
        let mut cli_config = CliConfig::default();
        cli_config.max_line_length = Some(150);
        cli_config.auto_fix = true;
        
        let mut config = LinterConfig::default();
        config.general.max_line_length = 100;
        
        ConfigLoader::apply_cli_overrides(&mut config, &cli_config);
        
        assert_eq!(config.general.max_line_length, 150);
        assert!(config.auto_fix);
    }

    #[test]
    fn test_rule_config() {
        let rule_config = RuleConfig {
            enabled: true,
            severity: RuleSeverity::Error,
            params: {
                let mut params = HashMap::new();
                params.insert("threshold".to_string(), serde_json::Value::Number(10.into()));
                params
            },
        };
        
        assert!(rule_config.enabled);
        assert_eq!(rule_config.severity, RuleSeverity::Error);
        assert_eq!(rule_config.params.len(), 1);
    }

    #[test]
    fn test_indent_style_serialization() {
        assert_eq!(IndentStyle::Spaces, serde_json::from_str("\"spaces\"").unwrap());
        assert_eq!(IndentStyle::Tabs, serde_json::from_str("\"tabs\"").unwrap());
    }
}
