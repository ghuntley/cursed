use crate::error::CursedError;
// Build Configuration System
// 
// Defines the configuration structure for CURSED projects including
// project metadata, build targets, profiles, and compilation settings.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use toml;

/// Main build configuration for a CURSED project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    /// Project metadata
    
    /// Build targets (executables, libraries, etc.)
    #[serde(default)]
    
    /// Build profiles (dev, release, custom, etc.)
    #[serde(default)]
    
    /// Dependencies
    #[serde(default)]
    
    /// Development dependencies
    #[serde(default = "HashMap::new")]
    
    /// Build dependencies
    #[serde(default = "HashMap::new")]
    
    /// Feature flags
    #[serde(default)]
    
    /// Build scripts and hooks
    #[serde(default)]
    
    /// Tool configurations
    #[serde(default)]
/// Project metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMetadata {
    /// Project name
    
    /// Project version
    
    /// Project description
    #[serde(default)]
    
    /// Authors
    #[serde(default)]
    
    /// License
    #[serde(default)]
    
    /// Repository URL
    #[serde(default)]
    
    /// Documentation URL
    #[serde(default)]
    
    /// Keywords
    #[serde(default)]
    
    /// Categories
    #[serde(default)]
    
    /// CURSED edition
    #[serde(default = "default_edition")]
fn default_edition() -> String {
    "2024".to_string()
/// Build target definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildTarget {
    /// Target name
    
    /// Target type (bin, lib, staticlib, dylib)
    #[serde(rename = "type")]
    
    /// Entry point file path
    
    /// Required features for this target
    #[serde(default)]
    
    /// Target-specific configurations
    #[serde(default)]
/// Target types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TargetType {
    /// Executable binary
    /// Library
    /// Static library
    /// Dynamic library
    /// C-compatible dynamic library
/// Build profile configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildProfile {
    /// Inherits from another profile
    #[serde(default)]
    
    /// Optimization level (none, basic, max, size)
    #[serde(default = "default_optimization")]
    
    /// Include debug information
    #[serde(default = "default_debug")]
    
    /// Strip symbols from output
    #[serde(default)]
    
    /// Link-time optimization
    #[serde(default)]
    
    /// Panic strategy (unwind, abort)
    #[serde(default = "default_panic")]
    
    /// Code generation units
    #[serde(default)]
    
    /// Additional LLVM flags
    #[serde(default)]
    
    /// Environment variables for build
    #[serde(default)]
fn default_optimization() -> OptimizationLevel {
    OptimizationLevel::O1
fn default_debug() -> bool {
    true
fn default_panic() -> PanicStrategy {
    PanicStrategy::Unwind
impl Default for BuildProfile {
    fn default() -> Self {
        Self {
        }
    }
impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            project: ProjectMetadata {
        }
    }
// Use canonical OptimizationLevel from optimization config
pub use crate::common_types::optimization_level::OptimizationLevel;

/// Panic strategies
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PanicStrategy {
    /// Stack unwinding
    /// Process abort
/// Tool-specific configurations
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToolConfigurations {
    /// Formatter configuration
    #[serde(default)]
    
    /// Linter configuration
    #[serde(default)]
    
    /// Documentation configuration
    #[serde(default)]
    
    /// Package manager configuration
    #[serde(default)]
    
    /// Compiler-specific configurations
    #[serde(default)]
    
    /// Cross-compilation targets
    #[serde(default)]
/// Formatter configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormatterConfig {
    /// Indentation style (spaces or tabs)
    #[serde(default = "default_indent_style")]
    
    /// Number of spaces per indent level
    #[serde(default = "default_indent_size")]
    
    /// Maximum line width
    #[serde(default = "default_line_width")]
    
    /// Brace style (same_line, next_line, next_line_unindented)
    #[serde(default = "default_brace_style")]
    
    /// Format on build
    #[serde(default)]
    
    /// Additional formatter options
    #[serde(default)]
/// Linter configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinterConfig {
    /// Lint on build
    #[serde(default)]
    
    /// Auto-fix issues when possible
    #[serde(default)]
    
    /// Severity level (error, warning, info)
    #[serde(default = "default_severity")]
    
    /// Disabled rules
    #[serde(default)]
    
    /// Enabled rules (if empty, all default rules are enabled)
    #[serde(default)]
    
    /// Additional linter options
    #[serde(default)]
/// Documentation configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DocsConfig {
    /// Generate docs on build
    #[serde(default)]
    
    /// Output format (html, markdown, json)
    #[serde(default = "default_docs_format")]
    
    /// Output directory
    #[serde(default = "default_docs_output")]
    
    /// Include private items
    #[serde(default)]
    
    /// Theme for HTML output
    #[serde(default = "default_docs_theme")]
    
    /// Additional documentation options
    #[serde(default)]
/// Package manager configuration  
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PackageManagerConfig {
    /// Registry URL
    #[serde(default = "default_registry_url")]
    
    /// Cache directory
    #[serde(default)]
    
    /// Offline mode
    #[serde(default)]
    
    /// Additional package manager options
    #[serde(default)]
/// Compiler configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CompilerConfig {
    /// Default target triple
    #[serde(default)]
    
    /// LLVM optimization passes
    #[serde(default)]
    
    /// Parallel compilation threads
    #[serde(default)]
    
    /// Incremental compilation
    #[serde(default = "default_incremental")]
    
    /// Debug information configuration
    #[serde(default)]
    
    /// Additional compiler options
    #[serde(default)]
/// Debug build configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugBuildConfig {
    /// Enable debug information generation
    #[serde(default = "default_true")]
    
    /// Debug information level (0-3)
    #[serde(default = "default_debug_level")]
    
    /// Include source code in debug information
    #[serde(default)]
    
    /// Generate optimized debug information
    #[serde(default)]
    
    /// Compress debug sections
    #[serde(default)]
    
    /// Split debug information into separate file
    #[serde(default)]
    
    /// Debug information output directory
    #[serde(default)]
    
    /// DWARF version to generate (2, 3, 4, or 5)
    #[serde(default = "default_dwarf_version")]
    
    /// Generate debug info for inlined functions
    #[serde(default = "default_true")]
    
    /// Generate debug info for type definitions
    #[serde(default = "default_true")]
    
    /// Generate debug info for variables
    #[serde(default = "default_true")]
    
    /// Generate debug info for function parameters
    #[serde(default = "default_true")]
    
    /// Generate debug info for local scopes
    #[serde(default = "default_true")]
    
    /// Generate debug info for line numbers
    #[serde(default = "default_true")]
    
    /// Generate debug info for column numbers
    #[serde(default)]
impl Default for DebugBuildConfig {
    fn default() -> Self {
        Self {
        }
    }
fn default_true() -> bool {
    true
fn default_debug_level() -> u8 {
    2
fn default_dwarf_version() -> u8 {
    4
/// Cross-compilation target configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossTargetConfig {
    /// Target triple (e.g., x86_64-pc-windows-gnu)
    
    /// Linker to use for this target
    #[serde(default)]
    
    /// Additional linker arguments
    #[serde(default)]
    
    /// Environment variables for cross-compilation
    #[serde(default)]
    
    /// Target-specific configuration
    #[serde(default)]
fn default_indent_style() -> String { "spaces".to_string() }
fn default_indent_size() -> u32 { 4 }
fn default_line_width() -> u32 { 100 }
fn default_brace_style() -> String { "same_line".to_string() }
fn default_severity() -> String { "warning".to_string() }
fn default_docs_format() -> String { "html".to_string() }
fn default_docs_output() -> PathBuf { PathBuf::from("docs") }
fn default_docs_theme() -> String { "light".to_string() }
fn default_registry_url() -> String { "https://registry.cursed-lang.org".to_string() }
impl BuildConfig {
    /// Load configuration from TOML file
    pub fn load_from_file(path: &PathBuf) -> crate::error::Result<()> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| ConfigError::IoError(e))?;
        
        let config: BuildConfig = toml::from_str(&content)
            .map_err(|e| ConfigError::ParseError(e))?;
        
        config.validate()?;
        Ok(config)
    /// Save configuration to TOML file
    pub fn save_to_file(&self, path: &PathBuf) -> crate::error::Result<()> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| ConfigError::SerializeError(e))?;
        
        std::fs::write(path, content)
            .map_err(|e| ConfigError::IoError(e))?;
        
        Ok(())
    /// Validate the configuration
    pub fn validate(&self) -> crate::error::Result<()> {
        // Validate project name
        if self.project.name.trim().is_empty() {
            return Err(ConfigError::ValidationError("Project name cannot be empty".to_string()));
        // Validate version format (semantic versioning)
        if !is_valid_semver(&self.project.version) {
            return Err(ConfigError::ValidationError("Invalid version format".to_string()));
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
        Ok(())
    /// Get effective profile configuration (with inheritance resolved)
    pub fn get_effective_profile(&self, profile_name: &str) -> crate::error::Result<()> {
        let mut visited = std::collections::HashSet::new();
        self.resolve_profile(profile_name, &mut visited)
    fn resolve_profile(&self, name: &str, visited: &mut std::collections::HashSet<String>) -> crate::error::Result<()> {
        if visited.contains(name) {
            return Err(ConfigError::ValidationError(
                format!("Circular profile inheritance detected: {}", name)
            ));
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
                description: Some(format!("A CURSED {} project", match project_type {
        
        // Add default profiles
        config.profiles.insert("dev".to_string(), BuildProfile {
        });
        
        config.profiles.insert("release".to_string(), BuildProfile {
        });
        
        // Add default target based on project type
        match project_type {
            ProjectType::Binary => {
                config.targets.push(BuildTarget {
                    path: PathBuf::from("src/main.csd"),
                });
            }
            ProjectType::Library => {
                config.targets.push(BuildTarget {
                    path: PathBuf::from("src/lib.csd"),
                });
            }
        }
        
        config
    }
}

/// Project types for initialization
#[derive(Debug, Clone)]
pub enum ProjectType {
/// Configuration error types
#[derive(Debug, thiserror::CursedError)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    
    #[error("Parse error: {0}")]
    
    #[error("Serialize error: {0}")]
    
    #[error("Validation error: {0}")]
fn is_valid_semver(version: &str) -> bool {
    // Simple semantic version validation
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() != 3 {
        return false;
    parts.iter().all(|part| part.parse::<u32>().is_ok())
