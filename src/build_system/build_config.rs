//! Build Configuration System
//! 
//! Defines the configuration structure for CURSED projects including
//! project metadata, build targets, profiles, and compilation settings.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use toml;

/// Main build configuration for a CURSED project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    /// Project metadata
    pub project: ProjectMetadata,
    
    /// Build targets (executables, libraries, etc.)
    #[serde(default)]
    pub targets: Vec<BuildTarget>,
    
    /// Build profiles (dev, release, custom, etc.)
    #[serde(default)]
    pub profiles: HashMap<String, BuildProfile>,
    
    /// Dependencies
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
    
    /// Development dependencies
    #[serde(default = "HashMap::new")]
    pub dev_dependencies: HashMap<String, String>,
    
    /// Build dependencies
    #[serde(default = "HashMap::new")]
    pub build_dependencies: HashMap<String, String>,
    
    /// Feature flags
    #[serde(default)]
    pub features: HashMap<String, Vec<String>>,
    
    /// Build scripts and hooks
    #[serde(default)]
    pub scripts: HashMap<String, String>,
    
    /// Tool configurations
    #[serde(default)]
    pub tools: ToolConfigurations,
}

/// Project metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMetadata {
    /// Project name
    pub name: String,
    
    /// Project version
    pub version: String,
    
    /// Project description
    #[serde(default)]
    pub description: Option<String>,
    
    /// Authors
    #[serde(default)]
    pub authors: Vec<String>,
    
    /// License
    #[serde(default)]
    pub license: Option<String>,
    
    /// Repository URL
    #[serde(default)]
    pub repository: Option<String>,
    
    /// Documentation URL
    #[serde(default)]
    pub documentation: Option<String>,
    
    /// Keywords
    #[serde(default)]
    pub keywords: Vec<String>,
    
    /// Categories
    #[serde(default)]
    pub categories: Vec<String>,
    
    /// CURSED edition
    #[serde(default = "default_edition")]
    pub edition: String,
}

fn default_edition() -> String {
    "2024".to_string()
}

/// Build target definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildTarget {
    /// Target name
    pub name: String,
    
    /// Target type (bin, lib, staticlib, dylib)
    #[serde(rename = "type")]
    pub target_type: TargetType,
    
    /// Entry point file path
    pub path: PathBuf,
    
    /// Required features for this target
    #[serde(default)]
    pub required_features: Vec<String>,
    
    /// Target-specific configurations
    #[serde(default)]
    pub config: HashMap<String, toml::Value>,
}

/// Target types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TargetType {
    /// Executable binary
    Bin,
    /// Library
    Lib,
    /// Static library
    StaticLib,
    /// Dynamic library
    DynLib,
    /// C-compatible dynamic library
    CDynLib,
}

/// Build profile configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildProfile {
    /// Inherits from another profile
    #[serde(default)]
    pub inherits: Option<String>,
    
    /// Optimization level (none, basic, max, size)
    #[serde(default = "default_optimization")]
    pub optimization: OptimizationLevel,
    
    /// Include debug information
    #[serde(default = "default_debug")]
    pub debug: bool,
    
    /// Strip symbols from output
    #[serde(default)]
    pub strip: bool,
    
    /// Link-time optimization
    #[serde(default)]
    pub lto: bool,
    
    /// Panic strategy (unwind, abort)
    #[serde(default = "default_panic")]
    pub panic: PanicStrategy,
    
    /// Code generation units
    #[serde(default)]
    pub codegen_units: Option<u32>,
    
    /// Additional LLVM flags
    #[serde(default)]
    pub llvm_args: Vec<String>,
    
    /// Environment variables for build
    #[serde(default)]
    pub env: HashMap<String, String>,
}

fn default_optimization() -> OptimizationLevel {
    OptimizationLevel::Basic
}

fn default_debug() -> bool {
    true
}

fn default_panic() -> PanicStrategy {
    PanicStrategy::Unwind
}

/// Optimization levels
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OptimizationLevel {
    /// No optimization
    None,
    /// Basic optimizations
    Basic,
    /// Maximum optimizations
    Max,
    /// Size optimizations
    Size,
}

/// Panic strategies
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PanicStrategy {
    /// Stack unwinding
    Unwind,
    /// Process abort
    Abort,
}

/// Tool-specific configurations
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToolConfigurations {
    /// Formatter configuration
    #[serde(default)]
    pub formatter: FormatterConfig,
    
    /// Linter configuration
    #[serde(default)]
    pub linter: LinterConfig,
    
    /// Documentation configuration
    #[serde(default)]
    pub docs: DocsConfig,
    
    /// Package manager configuration
    #[serde(default)]
    pub package_manager: PackageManagerConfig,
    
    /// Compiler-specific configurations
    #[serde(default)]
    pub compiler: CompilerConfig,
    
    /// Cross-compilation targets
    #[serde(default)]
    pub targets: HashMap<String, CrossTargetConfig>,
}

/// Formatter configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormatterConfig {
    /// Indentation style (spaces or tabs)
    #[serde(default = "default_indent_style")]
    pub indent_style: String,
    
    /// Number of spaces per indent level
    #[serde(default = "default_indent_size")]
    pub indent_size: u32,
    
    /// Maximum line width
    #[serde(default = "default_line_width")]
    pub line_width: u32,
    
    /// Brace style (same_line, next_line, next_line_unindented)
    #[serde(default = "default_brace_style")]
    pub brace_style: String,
    
    /// Format on build
    #[serde(default)]
    pub format_on_build: bool,
    
    /// Additional formatter options
    #[serde(default)]
    pub options: HashMap<String, toml::Value>,
}

/// Linter configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinterConfig {
    /// Lint on build
    #[serde(default)]
    pub lint_on_build: bool,
    
    /// Auto-fix issues when possible
    #[serde(default)]
    pub auto_fix: bool,
    
    /// Severity level (error, warning, info)
    #[serde(default = "default_severity")]
    pub severity: String,
    
    /// Disabled rules
    #[serde(default)]
    pub disabled_rules: Vec<String>,
    
    /// Enabled rules (if empty, all default rules are enabled)
    #[serde(default)]
    pub enabled_rules: Vec<String>,
    
    /// Additional linter options
    #[serde(default)]
    pub options: HashMap<String, toml::Value>,
}

/// Documentation configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DocsConfig {
    /// Generate docs on build
    #[serde(default)]
    pub generate_on_build: bool,
    
    /// Output format (html, markdown, json)
    #[serde(default = "default_docs_format")]
    pub format: String,
    
    /// Output directory
    #[serde(default = "default_docs_output")]
    pub output_dir: PathBuf,
    
    /// Include private items
    #[serde(default)]
    pub include_private: bool,
    
    /// Theme for HTML output
    #[serde(default = "default_docs_theme")]
    pub theme: String,
    
    /// Additional documentation options
    #[serde(default)]
    pub options: HashMap<String, toml::Value>,
}

/// Package manager configuration  
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PackageManagerConfig {
    /// Registry URL
    #[serde(default = "default_registry_url")]
    pub registry: String,
    
    /// Cache directory
    #[serde(default)]
    pub cache_dir: Option<PathBuf>,
    
    /// Offline mode
    #[serde(default)]
    pub offline: bool,
    
    /// Additional package manager options
    #[serde(default)]
    pub options: HashMap<String, toml::Value>,
}

/// Compiler configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CompilerConfig {
    /// Default target triple
    #[serde(default)]
    pub default_target: Option<String>,
    
    /// LLVM optimization passes
    #[serde(default)]
    pub llvm_passes: Vec<String>,
    
    /// Parallel compilation threads
    #[serde(default)]
    pub parallel_threads: Option<u32>,
    
    /// Incremental compilation
    #[serde(default = "default_incremental")]
    pub incremental: bool,
    
    /// Debug information configuration
    #[serde(default)]
    pub debug: DebugBuildConfig,
    
    /// Additional compiler options
    #[serde(default)]
    pub options: HashMap<String, toml::Value>,
}

/// Debug build configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugBuildConfig {
    /// Enable debug information generation
    #[serde(default = "default_true")]
    pub enabled: bool,
    
    /// Debug information level (0-3)
    #[serde(default = "default_debug_level")]
    pub level: u8,
    
    /// Include source code in debug information
    #[serde(default)]
    pub include_source: bool,
    
    /// Generate optimized debug information
    #[serde(default)]
    pub optimized: bool,
    
    /// Compress debug sections
    #[serde(default)]
    pub compress: bool,
    
    /// Split debug information into separate file
    #[serde(default)]
    pub split_debug_info: bool,
    
    /// Debug information output directory
    #[serde(default)]
    pub output_dir: Option<PathBuf>,
    
    /// DWARF version to generate (2, 3, 4, or 5)
    #[serde(default = "default_dwarf_version")]
    pub dwarf_version: u8,
    
    /// Generate debug info for inlined functions
    #[serde(default = "default_true")]
    pub inline_debug: bool,
    
    /// Generate debug info for type definitions
    #[serde(default = "default_true")]
    pub type_debug: bool,
    
    /// Generate debug info for variables
    #[serde(default = "default_true")]
    pub variable_debug: bool,
    
    /// Generate debug info for function parameters
    #[serde(default = "default_true")]
    pub parameter_debug: bool,
    
    /// Generate debug info for local scopes
    #[serde(default = "default_true")]
    pub scope_debug: bool,
    
    /// Generate debug info for line numbers
    #[serde(default = "default_true")]
    pub line_debug: bool,
    
    /// Generate debug info for column numbers
    #[serde(default)]
    pub column_debug: bool,
}

impl Default for DebugBuildConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            level: 2,
            include_source: false,
            optimized: false,
            compress: false,
            split_debug_info: false,
            output_dir: None,
            dwarf_version: 4,
            inline_debug: true,
            type_debug: true,
            variable_debug: true,
            parameter_debug: true,
            scope_debug: true,
            line_debug: true,
            column_debug: false,
        }
    }
}

fn default_true() -> bool {
    true
}

fn default_debug_level() -> u8 {
    2
}

fn default_dwarf_version() -> u8 {
    4
}

/// Cross-compilation target configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossTargetConfig {
    /// Target triple (e.g., x86_64-pc-windows-gnu)
    pub triple: String,
    
    /// Linker to use for this target
    #[serde(default)]
    pub linker: Option<String>,
    
    /// Additional linker arguments
    #[serde(default)]
    pub linker_args: Vec<String>,
    
    /// Environment variables for cross-compilation
    #[serde(default)]
    pub env: HashMap<String, String>,
    
    /// Target-specific configuration
    #[serde(default)]
    pub config: HashMap<String, toml::Value>,
}

fn default_indent_style() -> String { "spaces".to_string() }
fn default_indent_size() -> u32 { 4 }
fn default_line_width() -> u32 { 100 }
fn default_brace_style() -> String { "same_line".to_string() }
fn default_severity() -> String { "warning".to_string() }
fn default_docs_format() -> String { "html".to_string() }
fn default_docs_output() -> PathBuf { PathBuf::from("docs") }
fn default_docs_theme() -> String { "light".to_string() }
fn default_registry_url() -> String { "https://registry.cursed-lang.org".to_string() }
fn default_incremental() -> bool { true }

impl BuildConfig {
    /// Load configuration from TOML file
    pub fn load_from_file(path: &PathBuf) -> Result<Self, ConfigError> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| ConfigError::IoError(e))?;
        
        let config: BuildConfig = toml::from_str(&content)
            .map_err(|e| ConfigError::ParseError(e))?;
        
        config.validate()?;
        Ok(config)
    }
    
    /// Save configuration to TOML file
    pub fn save_to_file(&self, path: &PathBuf) -> Result<(), ConfigError> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| ConfigError::SerializeError(e))?;
        
        std::fs::write(path, content)
            .map_err(|e| ConfigError::IoError(e))?;
        
        Ok(())
    }
    
    /// Validate the configuration
    pub fn validate(&self) -> Result<(), ConfigError> {
        // Validate project name
        if self.project.name.trim().is_empty() {
            return Err(ConfigError::ValidationError("Project name cannot be empty".to_string()));
        }
        
        // Validate version format (semantic versioning)
        if !is_valid_semver(&self.project.version) {
            return Err(ConfigError::ValidationError("Invalid version format".to_string()));
        }
        
        // Validate target paths exist
        for target in &self.targets {
            if !target.path.exists() {
                return Err(ConfigError::ValidationError(
                    format!("Target path does not exist: {}", target.path.display())
                ));
            }
        }
        
        // Validate profile inheritance
        for (name, profile) in &self.profiles {
            if let Some(ref parent) = profile.inherits {
                if !self.profiles.contains_key(parent) {
                    return Err(ConfigError::ValidationError(
                        format!("Profile '{}' inherits from non-existent profile '{}'", name, parent)
                    ));
                }
            }
        }
        
        Ok(())
    }
    
    /// Get effective profile configuration (with inheritance resolved)
    pub fn get_effective_profile(&self, profile_name: &str) -> Result<BuildProfile, ConfigError> {
        let mut visited = std::collections::HashSet::new();
        self.resolve_profile(profile_name, &mut visited)
    }
    
    fn resolve_profile(&self, name: &str, visited: &mut std::collections::HashSet<String>) -> Result<BuildProfile, ConfigError> {
        if visited.contains(name) {
            return Err(ConfigError::ValidationError(
                format!("Circular profile inheritance detected: {}", name)
            ));
        }
        
        visited.insert(name.to_string());
        
        let profile = self.profiles.get(name)
            .ok_or_else(|| ConfigError::ValidationError(format!("Profile not found: {}", name)))?
            .clone();
        
        if let Some(ref parent_name) = profile.inherits {
            let mut parent = self.resolve_profile(parent_name, visited)?;
            
            // Merge parent profile with current profile
            if profile.optimization != default_optimization() || parent.optimization == default_optimization() {
                parent.optimization = profile.optimization;
            }
            if profile.debug != default_debug() || parent.debug == default_debug() {
                parent.debug = profile.debug;
            }
            parent.strip = profile.strip || parent.strip;
            parent.lto = profile.lto || parent.lto;
            if profile.panic != default_panic() || parent.panic == default_panic() {
                parent.panic = profile.panic;
            }
            if profile.codegen_units.is_some() {
                parent.codegen_units = profile.codegen_units;
            }
            parent.llvm_args.extend(profile.llvm_args);
            parent.env.extend(profile.env);
            
            Ok(parent)
        } else {
            Ok(profile)
        }
    }
    
    /// Create default configuration for a new project
    pub fn default_for_project(name: &str, project_type: ProjectType) -> Self {
        let mut config = BuildConfig {
            project: ProjectMetadata {
                name: name.to_string(),
                version: "0.1.0".to_string(),
                description: Some(format!("A CURSED {} project", match project_type {
                    ProjectType::Binary => "binary",
                    ProjectType::Library => "library",
                })),
                authors: vec!["Your Name <your.email@example.com>".to_string()],
                license: Some("MIT OR Apache-2.0".to_string()),
                repository: None,
                documentation: None,
                keywords: vec![],
                categories: vec![],
                edition: "2024".to_string(),
            },
            targets: vec![],
            profiles: HashMap::new(),
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            build_dependencies: HashMap::new(),
            features: HashMap::new(),
            scripts: HashMap::new(),
            tools: ToolConfigurations::default(),
        };
        
        // Add default profiles
        config.profiles.insert("dev".to_string(), BuildProfile {
            inherits: None,
            optimization: OptimizationLevel::None,
            debug: true,
            strip: false,
            lto: false,
            panic: PanicStrategy::Unwind,
            codegen_units: None,
            llvm_args: vec![],
            env: HashMap::new(),
        });
        
        config.profiles.insert("release".to_string(), BuildProfile {
            inherits: None,
            optimization: OptimizationLevel::Max,
            debug: false,
            strip: true,
            lto: true,
            panic: PanicStrategy::Abort,
            codegen_units: Some(1),
            llvm_args: vec![],
            env: HashMap::new(),
        });
        
        // Add default target based on project type
        match project_type {
            ProjectType::Binary => {
                config.targets.push(BuildTarget {
                    name: name.to_string(),
                    target_type: TargetType::Bin,
                    path: PathBuf::from("src/main.csd"),
                    required_features: vec![],
                    config: HashMap::new(),
                });
            }
            ProjectType::Library => {
                config.targets.push(BuildTarget {
                    name: name.to_string(),
                    target_type: TargetType::Lib,
                    path: PathBuf::from("src/lib.csd"),
                    required_features: vec![],
                    config: HashMap::new(),
                });
            }
        }
        
        config
    }
}

/// Project types for initialization
#[derive(Debug, Clone)]
pub enum ProjectType {
    Binary,
    Library,
}

/// Configuration error types
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Parse error: {0}")]
    ParseError(#[from] toml::de::Error),
    
    #[error("Serialize error: {0}")]
    SerializeError(#[from] toml::ser::Error),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
}

fn is_valid_semver(version: &str) -> bool {
    // Simple semantic version validation
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() != 3 {
        return false;
    }
    
    parts.iter().all(|part| part.parse::<u32>().is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_default_config_creation() {
        let config = BuildConfig::default_for_project("test-project", ProjectType::Binary);
        
        assert_eq!(config.project.name, "test-project");
        assert_eq!(config.project.version, "0.1.0");
        assert_eq!(config.targets.len(), 1);
        assert_eq!(config.profiles.len(), 2);
        assert!(config.profiles.contains_key("dev"));
        assert!(config.profiles.contains_key("release"));
    }
    
    #[test]
    fn test_config_validation() {
        let mut config = BuildConfig::default_for_project("test", ProjectType::Binary);
        
        // Valid config should pass
        assert!(config.validate().is_ok());
        
        // Empty name should fail
        config.project.name = "".to_string();
        assert!(config.validate().is_err());
        
        // Invalid version should fail
        config.project.name = "test".to_string();
        config.project.version = "invalid".to_string();
        assert!(config.validate().is_err());
    }
    
    #[test]
    fn test_profile_inheritance() {
        let mut config = BuildConfig::default_for_project("test", ProjectType::Binary);
        
        // Add a custom profile that inherits from release
        config.profiles.insert("production".to_string(), BuildProfile {
            inherits: Some("release".to_string()),
            optimization: OptimizationLevel::Size,
            debug: false,
            strip: true,
            lto: true,
            panic: PanicStrategy::Abort,
            codegen_units: Some(1),
            llvm_args: vec!["-march=native".to_string()],
            env: HashMap::new(),
        });
        
        let effective = config.get_effective_profile("production").unwrap();
        assert_eq!(effective.optimization, OptimizationLevel::Size);
        assert_eq!(effective.debug, false);
        assert_eq!(effective.strip, true);
        assert!(effective.llvm_args.contains(&"-march=native".to_string()));
    }
    
    #[test]
    fn test_config_serialization() {
        let config = BuildConfig::default_for_project("test-project", ProjectType::Library);
        
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("CursedBuild.toml");
        
        // Save and reload
        config.save_to_file(&file_path).unwrap();
        let loaded_config = BuildConfig::load_from_file(&file_path).unwrap();
        
        assert_eq!(config.project.name, loaded_config.project.name);
        assert_eq!(config.project.version, loaded_config.project.version);
        assert_eq!(config.targets.len(), loaded_config.targets.len());
    }
    
    #[test]
    fn test_semver_validation() {
        assert!(is_valid_semver("1.0.0"));
        assert!(is_valid_semver("0.1.0"));
        assert!(is_valid_semver("10.20.30"));
        
        assert!(!is_valid_semver("1.0"));
        assert!(!is_valid_semver("1.0.0.0"));
        assert!(!is_valid_semver("invalid"));
        assert!(!is_valid_semver("1.0.x"));
    }
}
