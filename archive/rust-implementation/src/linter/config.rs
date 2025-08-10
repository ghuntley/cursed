//! Linter configuration module

use super::{Severity, Category};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Linter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinterConfig {
    /// Rule-specific configurations
    pub rules: HashMap<String, RuleConfig>,
    
    /// Global severity overrides
    pub severity_overrides: HashMap<String, Severity>,
    
    /// Categories to enable/disable
    pub enabled_categories: Vec<Category>,
    
    /// Rules to explicitly disable
    pub disabled_rules: Vec<String>,
    
    /// Maximum number of issues to report
    pub max_issues: Option<usize>,
    
    /// Whether to include fix suggestions
    pub include_fixes: bool,
}

/// Configuration for individual rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleConfig {
    /// Whether the rule is enabled
    pub enabled: bool,
    
    /// Severity override for this rule
    pub severity: Option<Severity>,
    
    /// Rule-specific options
    pub options: HashMap<String, serde_json::Value>,
}

impl Default for LinterConfig {
    fn default() -> Self {
        Self {
            rules: HashMap::new(),
            severity_overrides: HashMap::new(),
            enabled_categories: vec![
                Category::Style,
                Category::Performance,
                Category::Security,
                Category::Correctness,
                Category::BestPractice,
            ],
            disabled_rules: vec![],
            max_issues: None,
            include_fixes: true,
        }
    }
}

impl LinterConfig {
    /// Create a new configuration with all rules enabled
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a strict configuration with higher severity levels
    pub fn strict() -> Self {
        let mut config = Self::default();
        config.severity_overrides.insert("unused_variable".to_string(), Severity::Error);
        config.severity_overrides.insert("dead_code".to_string(), Severity::Error);
        config.severity_overrides.insert("performance_issue".to_string(), Severity::Warning);
        config
    }

    /// Create a minimal configuration with only critical rules
    pub fn minimal() -> Self {
        let mut config = Self::default();
        config.enabled_categories = vec![Category::Security, Category::Correctness];
        config
    }

    /// Check if a rule is enabled
    pub fn is_rule_enabled(&self, rule_name: &str) -> bool {
        // Check if rule is explicitly disabled
        if self.disabled_rules.contains(&rule_name.to_string()) {
            return false;
        }

        // Check rule-specific configuration
        if let Some(rule_config) = self.rules.get(rule_name) {
            return rule_config.enabled;
        }

        // Default to enabled
        true
    }

    /// Get severity for a rule
    pub fn get_rule_severity(&self, rule_name: &str, default: Severity) -> Severity {
        // Check rule-specific severity override
        if let Some(rule_config) = self.rules.get(rule_name) {
            if let Some(severity) = &rule_config.severity {
                return severity.clone();
            }
        }

        // Check global severity override
        if let Some(severity) = self.severity_overrides.get(rule_name) {
            return severity.clone();
        }

        // Use default
        default
    }

    /// Get rule options
    pub fn get_rule_options(&self, rule_name: &str) -> HashMap<String, serde_json::Value> {
        self.rules
            .get(rule_name)
            .map(|config| config.options.clone())
            .unwrap_or_default()
    }

    /// Enable a specific rule
    pub fn enable_rule(&mut self, rule_name: &str) {
        // Remove from disabled list if present
        self.disabled_rules.retain(|rule| rule != rule_name);
        
        self.rules.entry(rule_name.to_string())
            .or_insert(RuleConfig {
                enabled: true,
                severity: None,
                options: HashMap::new(),
            })
            .enabled = true;
    }

    /// Disable a specific rule
    pub fn disable_rule(&mut self, rule_name: &str) {
        self.disabled_rules.push(rule_name.to_string());
    }

    /// Set severity for a rule
    pub fn set_rule_severity(&mut self, rule_name: &str, severity: Severity) {
        self.severity_overrides.insert(rule_name.to_string(), severity);
    }

    /// Set option for a rule
    pub fn set_rule_option(&mut self, rule_name: &str, key: &str, value: serde_json::Value) {
        self.rules.entry(rule_name.to_string())
            .or_insert(RuleConfig {
                enabled: true,
                severity: None,
                options: HashMap::new(),
            })
            .options.insert(key.to_string(), value);
    }

    /// Load configuration from TOML file
    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: Self = toml::from_str(&content)?;
        Ok(config)
    }

    /// Save configuration to TOML file
    pub fn to_file<P: AsRef<std::path::Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = LinterConfig::default();
        assert!(config.is_rule_enabled("test_rule"));
        assert_eq!(config.enabled_categories.len(), 5);
        assert!(config.include_fixes);
    }

    #[test]
    fn test_rule_configuration() {
        let mut config = LinterConfig::new();
        
        // Test enabling/disabling rules
        config.disable_rule("test_rule");
        assert!(!config.is_rule_enabled("test_rule"));
        
        config.enable_rule("test_rule");
        assert!(config.is_rule_enabled("test_rule"));
    }

    #[test]
    fn test_severity_overrides() {
        let mut config = LinterConfig::new();
        
        // Test severity overrides
        config.set_rule_severity("test_rule", Severity::Error);
        assert_eq!(
            config.get_rule_severity("test_rule", Severity::Warning),
            Severity::Error
        );
    }

    #[test]
    fn test_rule_options() {
        let mut config = LinterConfig::new();
        
        // Test rule options
        config.set_rule_option("test_rule", "max_length", serde_json::Value::Number(80.into()));
        let options = config.get_rule_options("test_rule");
        assert!(options.contains_key("max_length"));
    }

    #[test]
    fn test_preset_configurations() {
        let strict = LinterConfig::strict();
        let minimal = LinterConfig::minimal();
        
        assert!(!strict.severity_overrides.is_empty());
        assert_eq!(minimal.enabled_categories.len(), 2);
    }
}
