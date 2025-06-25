/// Debug configuration and settings
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Debug information generation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugConfig {
    /// Enable debug information generation
    
    /// Enable debug information generation (alternative field name for compatibility)
    
    /// Debug information level (0-3)
    
    /// Include source code in debug information
    
    /// Generate optimized debug information
    
    /// Use compressed debug sections
    
    /// Split debug information into separate file
    
    /// Debug information output directory
    
    /// Additional debug flags for specific features
    
    /// DWARF version to generate (2, 3, 4, or 5)
/// Specific debug feature flags
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugFlags {
    /// Generate debug info for inlined functions
    
    /// Generate debug info for template instantiations
    
    /// Generate debug info for macros
    
    /// Generate debug info for type definitions
    
    /// Generate debug info for variables
    
    /// Generate debug info for function parameters
    
    /// Generate debug info for local scopes
    
    /// Generate debug info for line numbers
    
    /// Generate debug info for column numbers
impl Default for DebugConfig {
    fn default() -> Self {
        Self {
        }
    }
impl Default for DebugFlags {
    fn default() -> Self {
        Self {
        }
    }
impl DebugConfig {
    /// Create a configuration for no debug information
    pub fn none() -> Self {
        Self {
        }
    }
    
    /// Create a configuration for minimal debug information
    pub fn minimal() -> Self {
        Self {
        }
    }
    
    /// Create a configuration for full debug information
    pub fn full() -> Self {
        Self {
        }
    }
    
    /// Create a configuration for release builds with debug info
    pub fn release() -> Self {
        Self {
        }
    }
    
    /// Check if any debug information should be generated
    pub fn has_debug_info(&self) -> bool {
        self.generate_debug_info && self.debug_level > 0
    /// Check if line number information should be generated
    pub fn has_line_info(&self) -> bool {
        self.has_debug_info() && self.debug_flags.line_debug
    /// Check if variable information should be generated
    pub fn has_variable_info(&self) -> bool {
        self.has_debug_info() && self.debug_flags.variable_debug
    /// Check if type information should be generated
    pub fn has_type_info(&self) -> bool {
        self.has_debug_info() && self.debug_flags.type_debug
    /// Get debug output file extension based on configuration
    pub fn debug_file_extension(&self) -> &'static str {
        if self.split_debug_info {
            match self.dwarf_version {
            }
        } else {
            ""
        }
    }
    
    /// Get LLVM debug info kind
    pub fn llvm_debug_kind(&self) -> &'static str {
        match self.debug_level {
        }
    }
    
    /// Validate the configuration
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        if self.debug_level > 3 {
            errors.push("Debug level must be 0-3".to_string());
        if ![2, 3, 4, 5].contains(&self.dwarf_version) {
            errors.push("DWARF version must be 2, 3, 4, or 5".to_string());
        if self.split_debug_info && !self.generate_debug_info {
            errors.push("Cannot split debug info when debug info generation is disabled".to_string());
        if self.optimized_debug && self.debug_level > 2 {
            errors.push("Optimized debug info is not compatible with high debug levels".to_string());
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
    
    /// Merge this configuration with another, preferring the other's values
    pub fn merge_with(&self, other: &DebugConfig) -> DebugConfig {
        DebugConfig {
        }
    }
impl DebugFlags {
    /// Create flags for no debug information
    pub fn none() -> Self {
        Self {
        }
    }
    
    /// Create flags for minimal debug information
    pub fn minimal() -> Self {
        Self {
        }
    }
    
    /// Create flags for full debug information
    pub fn full() -> Self {
        Self {
        }
    }
    
    /// Create flags for release builds
    pub fn release() -> Self {
        Self {
        }
    }
    
    /// Merge these flags with another set, preferring the other's values
    pub fn merge_with(&self, other: &DebugFlags) -> DebugFlags {
        DebugFlags {
        }
    }
/// Builder for debug configuration
pub struct DebugConfigBuilder {
impl DebugConfigBuilder {
    /// Create a new builder with default configuration
    pub fn new() -> Self {
        Self {
        }
    }
    
    /// Set whether to generate debug information
    pub fn debug_info(mut self, enabled: bool) -> Self {
        self.config.generate_debug_info = enabled;
        self
    /// Set debug level
    pub fn level(mut self, level: u8) -> Self {
        self.config.debug_level = level;
        self
    /// Set whether to include source code
    pub fn include_source(mut self, include: bool) -> Self {
        self.config.include_source = include;
        self
    /// Set whether to generate optimized debug info
    pub fn optimized(mut self, optimized: bool) -> Self {
        self.config.optimized_debug = optimized;
        self
    /// Set whether to compress debug sections
    pub fn compressed(mut self, compressed: bool) -> Self {
        self.config.compress_debug = compressed;
        self
    /// Set whether to split debug information
    pub fn split_debug(mut self, split: bool) -> Self {
        self.config.split_debug_info = split;
        self
    /// Set debug output directory
    pub fn output_dir(mut self, dir: PathBuf) -> Self {
        self.config.debug_output_dir = Some(dir);
        self
    /// Set DWARF version
    pub fn dwarf_version(mut self, version: u8) -> Self {
        self.config.dwarf_version = version;
        self
    /// Enable line debug information
    pub fn with_lines(mut self) -> Self {
        self.config.debug_flags.line_debug = true;
        self
    /// Enable variable debug information
    pub fn with_variables(mut self) -> Self {
        self.config.debug_flags.variable_debug = true;
        self
    /// Enable type debug information
    pub fn with_types(mut self) -> Self {
        self.config.debug_flags.type_debug = true;
        self
    /// Build the configuration
    pub fn build(self) -> DebugConfig {
        self.config
    /// Build and validate the configuration
    pub fn build_validated(self) -> Result<DebugConfig, Vec<String>> {
        self.config.validate()?;
        Ok(self.config)
    }
}

impl Default for DebugConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

