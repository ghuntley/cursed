//! Configuration for bootstrap compilation mode
//!
//! This module provides configuration options for enabling and customizing
//! bootstrap compilation mode in the CURSED compiler.

use std::path::PathBuf;
use serde::{Deserialize, Serialize};

/// Configuration for bootstrap compilation mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootstrapConfig {
    /// Whether bootstrap mode is enabled
    pub enabled: bool,
    
    /// Whether to enforce strict bootstrap subset compliance
    pub strict_mode: bool,
    
    /// Whether to generate warnings for non-optimal bootstrap code
    pub generate_warnings: bool,
    
    /// Maximum allowed program complexity for bootstrap
    pub max_statements: Option<usize>,
    
    /// Allowed standard library modules in bootstrap mode
    pub allowed_stdlib_modules: Vec<String>,
    
    /// Whether to allow experimental features in bootstrap mode
    pub allow_experimental: bool,
    
    /// Output directory for bootstrap compilation artifacts
    pub output_dir: Option<PathBuf>,
    
    /// Whether to generate bootstrap-specific optimizations
    pub optimize_for_bootstrap: bool,
    
    /// Whether to validate subset compliance before compilation
    pub validate_subset: bool,
}

impl Default for BootstrapConfig {
    fn default() -> Self {
        BootstrapConfig {
            enabled: false,
            strict_mode: true,
            generate_warnings: true,
            max_statements: None,
            allowed_stdlib_modules: vec![
                "vibez".to_string(),     // I/O operations
                "mathz".to_string(),     // Math operations
                "stringz".to_string(),   // String operations
                "timez".to_string(),     // Time operations
            ],
            allow_experimental: false,
            output_dir: None,
            optimize_for_bootstrap: true,
            validate_subset: true,
        }
    }
}

impl BootstrapConfig {
    /// Creates a new default bootstrap configuration
    pub fn new() -> Self {
        Default::default()
    }
    
    /// Creates a configuration for strict bootstrap mode
    pub fn strict() -> Self {
        BootstrapConfig {
            enabled: true,
            strict_mode: true,
            generate_warnings: true,
            max_statements: Some(1000), // Limit complexity for bootstrap
            allow_experimental: false,
            validate_subset: true,
            ..Default::default()
        }
    }
    
    /// Creates a configuration for lenient bootstrap mode
    pub fn lenient() -> Self {
        BootstrapConfig {
            enabled: true,
            strict_mode: false,
            generate_warnings: false,
            max_statements: None,
            allow_experimental: true,
            validate_subset: false,
            ..Default::default()
        }
    }
    
    /// Enables bootstrap mode
    pub fn enable(&mut self) -> &mut Self {
        self.enabled = true;
        self
    }
    
    /// Disables bootstrap mode
    pub fn disable(&mut self) -> &mut Self {
        self.enabled = false;
        self
    }
    
    /// Sets strict mode
    pub fn with_strict_mode(&mut self, strict: bool) -> &mut Self {
        self.strict_mode = strict;
        self
    }
    
    /// Sets warning generation
    pub fn with_warnings(&mut self, warnings: bool) -> &mut Self {
        self.generate_warnings = warnings;
        self
    }
    
    /// Sets maximum statement limit
    pub fn with_max_statements(&mut self, max: Option<usize>) -> &mut Self {
        self.max_statements = max;
        self
    }
    
    /// Adds an allowed standard library module
    pub fn allow_stdlib_module(&mut self, module: String) -> &mut Self {
        if !self.allowed_stdlib_modules.contains(&module) {
            self.allowed_stdlib_modules.push(module);
        }
        self
    }
    
    /// Sets the allowed standard library modules
    pub fn with_stdlib_modules(&mut self, modules: Vec<String>) -> &mut Self {
        self.allowed_stdlib_modules = modules;
        self
    }
    
    /// Sets experimental feature allowance
    pub fn with_experimental(&mut self, allow: bool) -> &mut Self {
        self.allow_experimental = allow;
        self
    }
    
    /// Sets the output directory
    pub fn with_output_dir(&mut self, dir: PathBuf) -> &mut Self {
        self.output_dir = Some(dir);
        self
    }
    
    /// Sets bootstrap optimization
    pub fn with_bootstrap_optimization(&mut self, optimize: bool) -> &mut Self {
        self.optimize_for_bootstrap = optimize;
        self
    }
    
    /// Sets subset validation
    pub fn with_subset_validation(&mut self, validate: bool) -> &mut Self {
        self.validate_subset = validate;
        self
    }
    
    /// Validates the configuration for consistency
    pub fn validate(&self) -> Result<(), String> {
        if self.enabled && self.allowed_stdlib_modules.is_empty() {
            return Err("Bootstrap mode requires at least one allowed stdlib module".to_string());
        }
        
        if let Some(max) = self.max_statements {
            if max < 10 {
                return Err("Maximum statements must be at least 10 for meaningful programs".to_string());
            }
        }
        
        if self.strict_mode && self.allow_experimental {
            return Err("Strict mode and experimental features are mutually exclusive".to_string());
        }
        
        Ok(())
    }
    
    /// Checks if a standard library module is allowed
    pub fn is_stdlib_module_allowed(&self, module: &str) -> bool {
        self.allowed_stdlib_modules.contains(&module.to_string())
    }
    
    /// Gets the effective output directory (default if not set)
    pub fn get_output_dir(&self) -> PathBuf {
        self.output_dir.clone().unwrap_or_else(|| PathBuf::from("./bootstrap_output"))
    }
    
    /// Checks if the configuration should enforce limits
    pub fn should_enforce_limits(&self) -> bool {
        self.enabled && (self.strict_mode || self.max_statements.is_some())
    }
    
    /// Gets a description of the current configuration
    pub fn describe(&self) -> String {
        if !self.enabled {
            return "Bootstrap mode disabled".to_string();
        }
        
        let mut description = String::from("Bootstrap mode enabled");
        
        if self.strict_mode {
            description.push_str(" (strict)");
        } else {
            description.push_str(" (lenient)");
        }
        
        if let Some(max) = self.max_statements {
            description.push_str(&format!(", max {} statements", max));
        }
        
        if self.allow_experimental {
            description.push_str(", experimental features allowed");
        }
        
        description.push_str(&format!(", {} stdlib modules", self.allowed_stdlib_modules.len()));
        
        description
    }
}

/// Builder pattern for creating bootstrap configurations
pub struct BootstrapConfigBuilder {
    config: BootstrapConfig,
}

impl BootstrapConfigBuilder {
    /// Creates a new configuration builder
    pub fn new() -> Self {
        BootstrapConfigBuilder {
            config: BootstrapConfig::default(),
        }
    }
    
    /// Enables bootstrap mode
    pub fn enabled(mut self) -> Self {
        self.config.enabled = true;
        self
    }
    
    /// Sets strict mode
    pub fn strict(mut self) -> Self {
        self.config.strict_mode = true;
        self
    }
    
    /// Sets lenient mode
    pub fn lenient(mut self) -> Self {
        self.config.strict_mode = false;
        self
    }
    
    /// Enables warnings
    pub fn with_warnings(mut self) -> Self {
        self.config.generate_warnings = true;
        self
    }
    
    /// Sets maximum statements
    pub fn max_statements(mut self, max: usize) -> Self {
        self.config.max_statements = Some(max);
        self
    }
    
    /// Allows an additional stdlib module
    pub fn allow_module(mut self, module: &str) -> Self {
        self.config.allow_stdlib_module(module.to_string());
        self
    }
    
    /// Allows experimental features
    pub fn experimental(mut self) -> Self {
        self.config.allow_experimental = true;
        self
    }
    
    /// Sets output directory
    pub fn output_dir(mut self, dir: PathBuf) -> Self {
        self.config.output_dir = Some(dir);
        self
    }
    
    /// Builds the final configuration
    pub fn build(self) -> Result<BootstrapConfig, String> {
        self.config.validate()?;
        Ok(self.config)
    }
}

impl Default for BootstrapConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = BootstrapConfig::default();
        assert!(!config.enabled);
        assert!(config.strict_mode);
        assert!(config.generate_warnings);
        assert!(config.validate().is_ok());
    }
    
    #[test]
    fn test_strict_config() {
        let config = BootstrapConfig::strict();
        assert!(config.enabled);
        assert!(config.strict_mode);
        assert!(!config.allow_experimental);
        assert!(config.validate().is_ok());
    }
    
    #[test]
    fn test_lenient_config() {
        let config = BootstrapConfig::lenient();
        assert!(config.enabled);
        assert!(!config.strict_mode);
        assert!(config.allow_experimental);
        assert!(config.validate().is_ok());
    }
    
    #[test]
    fn test_config_builder() {
        let config = BootstrapConfigBuilder::new()
            .enabled()
            .strict()
            .max_statements(500)
            .allow_module("debugz")
            .build();
            
        assert!(config.is_ok());
        let config = config.unwrap();
        assert!(config.enabled);
        assert!(config.strict_mode);
        assert_eq!(config.max_statements, Some(500));
        assert!(config.is_stdlib_module_allowed("debugz"));
    }
    
    #[test]
    fn test_config_validation() {
        let mut config = BootstrapConfig::strict();
        config.allowed_stdlib_modules.clear();
        
        assert!(config.validate().is_err());
    }
    
    #[test]
    fn test_stdlib_module_checking() {
        let config = BootstrapConfig::default();
        assert!(config.is_stdlib_module_allowed("vibez"));
        assert!(config.is_stdlib_module_allowed("mathz"));
        assert!(!config.is_stdlib_module_allowed("advanced_module"));
    }
    
    #[test]
    fn test_config_description() {
        let config = BootstrapConfig::strict();
        let description = config.describe();
        assert!(description.contains("Bootstrap mode enabled"));
        assert!(description.contains("strict"));
    }
}
