use crate::error::CursedError;
/// Test attributes and metadata parsing for CURSED testing framework
/// 
/// Handles parsing and validation of test attributes like #[test], #[ignore],
/// #[should_panic], #[timeout], and custom test metadata.

use std::collections::HashMap;
use std::time::Duration;
use super::{TestError, TestFrameworkResult};

/// Test attribute types
#[derive(Debug, Clone, PartialEq)]
pub enum TestAttribute {
    /// Basic test marker
    /// Test should be ignored
    /// Test is expected to panic
    /// Test timeout
    /// Test setup function
    /// Test teardown function
    /// Test tags/categories
    /// Custom attribute
impl TestAttribute {
    /// Parse attribute from string
    pub fn parse(attr_str: &str) -> TestFrameworkResult<Self> {
        let attr_str = attr_str.trim();
        
        if attr_str == "#[test]" {
            return Ok(TestAttribute::Test);
        if attr_str == "#[ignore]" {
            return Ok(TestAttribute::Ignore(None));
        if attr_str.starts_with("#[ignore(") && attr_str.ends_with(")]") {
            let reason = extract_string_parameter(attr_str, "ignore")?;
            return Ok(TestAttribute::Ignore(Some(reason)));
        if attr_str == "#[should_panic]" {
            return Ok(TestAttribute::ShouldPanic(None));
        if attr_str.starts_with("#[should_panic(") && attr_str.ends_with(")]") {
            let message = extract_string_parameter(attr_str, "should_panic")?;
            return Ok(TestAttribute::ShouldPanic(Some(message)));
        if attr_str.starts_with("#[timeout(") && attr_str.ends_with(")]") {
            let timeout = extract_timeout_parameter(attr_str)?;
            return Ok(TestAttribute::Timeout(timeout));
        if attr_str.starts_with("#[setup(") && attr_str.ends_with(")]") {
            let setup_fn = extract_string_parameter(attr_str, "setup")?;
            return Ok(TestAttribute::Setup(setup_fn));
        if attr_str.starts_with("#[teardown(") && attr_str.ends_with(")]") {
            let teardown_fn = extract_string_parameter(attr_str, "teardown")?;
            return Ok(TestAttribute::Teardown(teardown_fn));
        if attr_str.starts_with("#[tag(") && attr_str.ends_with(")]") {
            let tag = extract_string_parameter(attr_str, "tag")?;
            return Ok(TestAttribute::Tag(tag));
        // Try to parse as custom attribute
        if attr_str.starts_with("#[") && attr_str.ends_with("]") {
            let inner = &attr_str[2..attr_str.len()-1];
            if let Some(paren_pos) = inner.find('(') {
                let name = inner[..paren_pos].trim().to_string();
                let value = extract_string_parameter(attr_str, &name)?;
                return Ok(TestAttribute::Custom(name, Some(value)));
            } else {
                let name = inner.trim().to_string();
                return Ok(TestAttribute::Custom(name, None));
            }
        }
        
        Err(TestError::ConfigError(format!("Invalid test attribute: {}", attr_str)).into())
    /// Convert attribute to string representation
    pub fn to_string(&self) -> String {
        match self {
        }
    }
/// Collection of test attributes
#[derive(Debug, Clone, Default)]
pub struct TestAttributes {
    /// List of attributes
    /// Parsed metadata cache
impl TestAttributes {
    /// Create new empty attributes collection
    pub fn new() -> Self {
        Self {
        }
    }
    
    /// Add an attribute
    pub fn add(&mut self, attribute: TestAttribute) {
        self.attributes.push(attribute);
        self.metadata_cache = None; // Invalidate cache
    /// Check if test has a specific attribute type
    pub fn has_attribute(&self, attr_type: &str) -> bool {
        self.attributes.iter().any(|attr| {
            match attr {
            }
        })
    /// Get all attributes of a specific type
    pub fn get_attributes(&self, attr_type: &str) -> Vec<&TestAttribute> {
        self.attributes.iter().filter(|attr| {
            match attr {
            }
        }).collect()
    /// Get parsed metadata
    pub fn get_metadata(&mut self) -> &ParsedTestMetadata {
        if self.metadata_cache.is_none() {
            self.metadata_cache = Some(self.parse_metadata());
        }
        self.metadata_cache.as_ref().unwrap()
    /// Parse attributes into structured metadata
    fn parse_metadata(&self) -> ParsedTestMetadata {
        let mut metadata = ParsedTestMetadata::new();
        
        for attr in &self.attributes {
            match attr {
                TestAttribute::Test => {
                    metadata.is_test = true;
                }
                TestAttribute::Ignore(reason) => {
                    metadata.ignore = true;
                    metadata.ignore_reason = reason.clone();
                }
                TestAttribute::ShouldPanic(message) => {
                    metadata.should_panic = true;
                    metadata.expected_panic_message = message.clone();
                }
                TestAttribute::Timeout(duration) => {
                    metadata.timeout = Some(*duration);
                }
                TestAttribute::Setup(func) => {
                    metadata.setup_function = Some(func.clone());
                }
                TestAttribute::Teardown(func) => {
                    metadata.teardown_function = Some(func.clone());
                }
                TestAttribute::Tag(tag) => {
                    metadata.tags.push(tag.clone());
                }
                TestAttribute::Custom(name, value) => {
                    metadata.custom_attributes.insert(name.clone(), value.clone());
                }
            }
        metadata
    /// Parse attributes from multiple strings
    pub fn parse_from_strings(attr_strings: &[String]) -> TestFrameworkResult<Self> {
        let mut attributes = TestAttributes::new();
        
        for attr_str in attr_strings {
            let attr = TestAttribute::parse(attr_str)?;
            attributes.add(attr);
        Ok(attributes)
    /// Validate attributes for consistency
    pub fn validate(&self) -> TestFrameworkResult<()> {
        let has_test = self.has_attribute("test");
        let has_ignore = self.has_attribute("ignore");
        let has_timeout = self.has_attribute("timeout");
        
        // A test function must have the #[test] attribute
        if !has_test && (has_ignore || has_timeout) {
            return Err(TestError::ConfigError(
                "Test attributes require #[test] attribute".to_string()
            ).into());
        // Check for duplicate timeouts
        let timeout_attrs = self.get_attributes("timeout");
        if timeout_attrs.len() > 1 {
            return Err(TestError::ConfigError(
                "Multiple timeout attributes not allowed".to_string()
            ).into());
        // Check for duplicate setup/teardown
        let setup_attrs = self.get_attributes("setup");
        if setup_attrs.len() > 1 {
            return Err(TestError::ConfigError(
                "Multiple setup attributes not allowed".to_string()
            ).into());
        let teardown_attrs = self.get_attributes("teardown");
        if teardown_attrs.len() > 1 {
            return Err(TestError::ConfigError(
                "Multiple teardown attributes not allowed".to_string()
            ).into());
        Ok(())
    }
}

/// Parsed test metadata
#[derive(Debug, Clone)]
pub struct ParsedTestMetadata {
    /// Whether this is a test function
    /// Whether test should be ignored
    /// Reason for ignoring (if any)
    /// Whether test should panic
    /// Expected panic message (if any)
    /// Test timeout
    /// Setup function name
    /// Teardown function name
    /// Test tags
    /// Custom attributes
impl ParsedTestMetadata {
    /// Create new empty metadata
    pub fn new() -> Self {
        Self {
        }
    }
    
    /// Check if test should be executed
    pub fn should_execute(&self) -> bool {
        self.is_test && !self.ignore
    /// Get effective timeout (with default fallback)
    pub fn get_timeout(&self, default: Duration) -> Duration {
        self.timeout.unwrap_or(default)
    /// Check if test has a specific tag
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.contains(&tag.to_string())
    /// Get custom attribute value
    pub fn get_custom_attribute(&self, name: &str) -> Option<&Option<String>> {
        self.custom_attributes.get(name)
    }
}

impl Default for ParsedTestMetadata {
    fn default() -> Self {
        Self::new()
    }
}

/// Test ignore configuration
#[derive(Debug, Clone)]
pub struct TestIgnore {
    /// Whether test is ignored
    /// Reason for ignoring
    /// Conditions under which to ignore
impl TestIgnore {
    /// Create new ignore configuration
    pub fn new(ignored: bool) -> Self {
        Self {
        }
    }
    
    /// Set ignore reason
    pub fn with_reason(mut self, reason: String) -> Self {
        self.reason = Some(reason);
        self
    /// Add ignore condition
    pub fn with_condition(mut self, condition: IgnoreCondition) -> Self {
        self.conditions.push(condition);
        self
    /// Check if test should be ignored based on conditions
    pub fn should_ignore(&self, context: &IgnoreContext) -> bool {
        if self.ignored {
            return true;
        // Check conditions
        for condition in &self.conditions {
            if condition.matches(context) {
                return true;
            }
        }
        
        false
    }
}

/// Ignore condition
#[derive(Debug, Clone)]
pub enum IgnoreCondition {
    /// Ignore on specific platform
    /// Ignore when environment variable is set
    /// Ignore based on feature flags
    /// Custom condition
impl IgnoreCondition {
    /// Check if condition matches current context
    pub fn matches(&self, context: &IgnoreContext) -> bool {
        match self {
            IgnoreCondition::Platform(platform) => {
                context.platform.as_ref().map_or(false, |p| p == platform)
            }
            IgnoreCondition::EnvVar(var, expected_value) => {
                match std::env::var(var) {
                    Ok(value) => {
                        if let Some(expected) = expected_value {
                            value == *expected
                        } else {
                            true // Just check if variable exists
                        }
                    }
                }
            }
            IgnoreCondition::Feature(feature) => {
                context.features.contains(feature)
            }
            IgnoreCondition::Custom(_) => {
                // Custom conditions need to be evaluated by the test framework
                false
            }
        }
    }
}

/// Context for evaluating ignore conditions
#[derive(Debug, Clone, Default)]
pub struct IgnoreContext {
    /// Current platform
    /// Available features
    /// Environment variables
    /// Custom context data
impl IgnoreContext {
    /// Create context from current environment
    pub fn from_environment() -> Self {
        let platform = Some(std::env::consts::OS.to_string());
        let env_vars: HashMap<String, String> = std::env::vars().collect();
        
        Self {
        }
    }
/// Expected panic configuration
#[derive(Debug, Clone)]
pub struct TestExpectedPanic {
    /// Whether panic is expected
    /// Expected panic message (exact match)
    /// Expected panic message pattern (regex-like)
impl TestExpectedPanic {
    /// Create new expected panic configuration
    pub fn new(expected: bool) -> Self {
        Self {
        }
    }
    
    /// Set expected message
    pub fn with_message(mut self, message: String) -> Self {
        self.expected_message = Some(message);
        self
    /// Set expected pattern
    pub fn with_pattern(mut self, pattern: String) -> Self {
        self.expected_pattern = Some(pattern);
        self
    /// Check if panic message matches expectations
    pub fn matches_panic(&self, panic_message: &str) -> bool {
        if !self.expected {
            return false;
        if let Some(ref expected_msg) = self.expected_message {
            return panic_message == expected_msg;
        if let Some(ref pattern) = self.expected_pattern {
            // Simple pattern matching (in real implementation, use regex)
            return panic_message.contains(pattern);
        // If no specific message/pattern, any panic is acceptable
        true
    }
}

/// Helper function to extract string parameter from attribute
fn extract_string_parameter(attr_str: &str, attr_name: &str) -> TestFrameworkResult<String> {
    let prefix = format!("#[{}(", attr_name);
    if !attr_str.starts_with(&prefix) || !attr_str.ends_with(")]") {
        return Err(TestError::ConfigError(
            format!("Invalid {} attribute format", attr_name)
        ).into());
    let param_start = prefix.len();
    let param_end = attr_str.len() - 2;
    let param_str = &attr_str[param_start..param_end];
    
    // Handle quoted strings
    if param_str.starts_with('"') && param_str.ends_with('"') {
        Ok(param_str[1..param_str.len()-1].to_string())
    } else if param_str.starts_with('\'') && param_str.ends_with('\'') {
        Ok(param_str[1..param_str.len()-1].to_string())
    } else {
        Ok(param_str.to_string())
    }
}

/// Helper function to extract timeout parameter
fn extract_timeout_parameter(attr_str: &str) -> TestFrameworkResult<Duration> {
    let param = extract_string_parameter(attr_str, "timeout")?;
    
    // Try different formats
    if param.ends_with("ms") {
        let millis: u64 = param[..param.len()-2].parse()
            .map_err(|_| TestError::ConfigError("Invalid timeout value".to_string()))?;
        Ok(Duration::from_millis(millis))
    } else if param.ends_with('s') {
        let secs: u64 = param[..param.len()-1].parse()
            .map_err(|_| TestError::ConfigError("Invalid timeout value".to_string()))?;
        Ok(Duration::from_secs(secs))
    } else {
        // Assume milliseconds
        let millis: u64 = param.parse()
            .map_err(|_| TestError::ConfigError("Invalid timeout value".to_string()))?;
        Ok(Duration::from_millis(millis))
    }
}

/// Parse test attributes from source code lines
pub fn parse_test_attributes(lines: &[String]) -> TestFrameworkResult<TestAttributes> {
    let mut attributes = TestAttributes::new();
    
    for line in lines {
        let trimmed = line.trim();
        if trimmed.starts_with("#[") && trimmed.ends_with(']') {
            let attr = TestAttribute::parse(trimmed)?;
            attributes.add(attr);
        }
    }
    
    attributes.validate()?;
    Ok(attributes)
/// Validate test attributes for consistency and correctness
pub fn validate_test_attributes(attributes: &TestAttributes) -> TestFrameworkResult<()> {
    attributes.validate()
}
