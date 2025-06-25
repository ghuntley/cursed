/// Debug configuration and settings
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Debug information generation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugConfig {
    /// Enable debug information generation
    pub generate_debug_info: bool,
    
    /// Enable debug information generation (alternative field name for compatibility)
    pub debug_info_enabled: bool,
    
    /// Debug information level (0-3)
    pub debug_level: u8,
    
    /// Include source code in debug information
    pub include_source: bool,
    
    /// Generate optimized debug information
    pub optimized_debug: bool,
    
    /// Use compressed debug sections
    pub compress_debug: bool,
    
    /// Split debug information into separate file
    pub split_debug_info: bool,
    
    /// Debug information output directory
    pub debug_output_dir: Option<PathBuf>,
    
    /// Additional debug flags for specific features
    pub debug_flags: DebugFlags,
    
    /// DWARF version to generate (2, 3, 4, or 5)
    pub dwarf_version: u8,
}

/// Specific debug feature flags
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugFlags {
    /// Generate debug info for inlined functions
    pub inline_debug: bool,
    
    /// Generate debug info for template instantiations
    pub template_debug: bool,
    
    /// Generate debug info for macros
    pub macro_debug: bool,
    
    /// Generate debug info for type definitions
    pub type_debug: bool,
    
    /// Generate debug info for variables
    pub variable_debug: bool,
    
    /// Generate debug info for function parameters
    pub parameter_debug: bool,
    
    /// Generate debug info for local scopes
    pub scope_debug: bool,
    
    /// Generate debug info for line numbers
    pub line_debug: bool,
    
    /// Generate debug info for column numbers
    pub column_debug: bool,
}

impl Default for DebugConfig {
    fn default() -> Self {
        Self {
            generate_debug_info: true,
            debug_info_enabled: true,
            debug_level: 2,
            include_source: false,
            optimized_debug: false,
            compress_debug: false,
            split_debug_info: false,
            debug_output_dir: None,
            debug_flags: DebugFlags::default(),
            dwarf_version: 4,
        }
    }
}

impl Default for DebugFlags {
    fn default() -> Self {
        Self {
            inline_debug: true,
            template_debug: true,
            macro_debug: false,
            type_debug: true,
            variable_debug: true,
            parameter_debug: true,
            scope_debug: true,
            line_debug: true,
            column_debug: true,
        }
    }
}

impl DebugConfig {
    /// Create a configuration for no debug information
    pub fn none() -> Self {
        Self {
            generate_debug_info: false,
            debug_info_enabled: false,
            debug_level: 0,
            include_source: false,
            optimized_debug: false,
            compress_debug: false,
            split_debug_info: false,
            debug_output_dir: None,
            debug_flags: DebugFlags::none(),
            dwarf_version: 4,
        }
    }
    
    /// Create a configuration for minimal debug information
    pub fn minimal() -> Self {
        Self {
            generate_debug_info: true,
            debug_info_enabled: true,
            debug_level: 1,
            include_source: false,
            optimized_debug: true,
            compress_debug: true,
            split_debug_info: false,
            debug_output_dir: None,
            debug_flags: DebugFlags::minimal(),
            dwarf_version: 4,
        }
    }
    
    /// Create a configuration for full debug information
    pub fn full() -> Self {
        Self {
            generate_debug_info: true,
            debug_info_enabled: true,
            debug_level: 3,
            include_source: true,
            optimized_debug: false,
            compress_debug: false,
            split_debug_info: false,
            debug_output_dir: None,
            debug_flags: DebugFlags::full(),
            dwarf_version: 4,
        }
    }
    
    /// Create a configuration for release builds with debug info
    pub fn release() -> Self {
        Self {
            generate_debug_info: true,
            debug_info_enabled: true,
            debug_level: 1,
            include_source: false,
            optimized_debug: true,
            compress_debug: true,
            split_debug_info: true,
            debug_output_dir: None,
            debug_flags: DebugFlags::release(),
            dwarf_version: 4,
        }
    }
    
    /// Check if any debug information should be generated
    pub fn has_debug_info(&self) -> bool {
        self.generate_debug_info && self.debug_level > 0
    }
    
    /// Check if line number information should be generated
    pub fn has_line_info(&self) -> bool {
        self.has_debug_info() && self.debug_flags.line_debug
    }
    
    /// Check if variable information should be generated
    pub fn has_variable_info(&self) -> bool {
        self.has_debug_info() && self.debug_flags.variable_debug
    }
    
    /// Check if type information should be generated
    pub fn has_type_info(&self) -> bool {
        self.has_debug_info() && self.debug_flags.type_debug
    }
    
    /// Get debug output file extension based on configuration
    pub fn debug_file_extension(&self) -> &'static str {
        if self.split_debug_info {
            match self.dwarf_version {
                2 => ".dwo",
                3 => ".dwo",
                4 => ".dwo",
                5 => ".dwp",
                _ => ".debug",
            }
        } else {
            ""
        }
    }
    
    /// Get LLVM debug info kind
    pub fn llvm_debug_kind(&self) -> &'static str {
        match self.debug_level {
            0 => "NoDebug",
            1 => "LineTablesOnly",
            2 => "FullDebug",
            3 => "FullDebug",
            _ => "FullDebug",
        }
    }
    
    /// Validate the configuration
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        if self.debug_level > 3 {
            errors.push("Debug level must be 0-3".to_string());
        }
        
        if ![2, 3, 4, 5].contains(&self.dwarf_version) {
            errors.push("DWARF version must be 2, 3, 4, or 5".to_string());
        }
        
        if self.split_debug_info && !self.generate_debug_info {
            errors.push("Cannot split debug info when debug info generation is disabled".to_string());
        }
        
        if self.optimized_debug && self.debug_level > 2 {
            errors.push("Optimized debug info is not compatible with high debug levels".to_string());
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
    
    /// Merge this configuration with another, preferring the other's values
    pub fn merge_with(&self, other: &DebugConfig) -> DebugConfig {
        DebugConfig {
            generate_debug_info: other.generate_debug_info,
            debug_info_enabled: other.debug_info_enabled,
            debug_level: other.debug_level,
            include_source: other.include_source,
            optimized_debug: other.optimized_debug,
            compress_debug: other.compress_debug,
            split_debug_info: other.split_debug_info,
            debug_output_dir: other.debug_output_dir.clone().or_else(|| self.debug_output_dir.clone()),
            debug_flags: self.debug_flags.merge_with(&other.debug_flags),
            dwarf_version: other.dwarf_version,
        }
    }
}

impl DebugFlags {
    /// Create flags for no debug information
    pub fn none() -> Self {
        Self {
            inline_debug: false,
            template_debug: false,
            macro_debug: false,
            type_debug: false,
            variable_debug: false,
            parameter_debug: false,
            scope_debug: false,
            line_debug: false,
            column_debug: false,
        }
    }
    
    /// Create flags for minimal debug information
    pub fn minimal() -> Self {
        Self {
            inline_debug: false,
            template_debug: false,
            macro_debug: false,
            type_debug: false,
            variable_debug: false,
            parameter_debug: false,
            scope_debug: false,
            line_debug: true,
            column_debug: false,
        }
    }
    
    /// Create flags for full debug information
    pub fn full() -> Self {
        Self {
            inline_debug: true,
            template_debug: true,
            macro_debug: true,
            type_debug: true,
            variable_debug: true,
            parameter_debug: true,
            scope_debug: true,
            line_debug: true,
            column_debug: true,
        }
    }
    
    /// Create flags for release builds
    pub fn release() -> Self {
        Self {
            inline_debug: false,
            template_debug: false,
            macro_debug: false,
            type_debug: true,
            variable_debug: false,
            parameter_debug: false,
            scope_debug: false,
            line_debug: true,
            column_debug: false,
        }
    }
    
    /// Merge these flags with another set, preferring the other's values
    pub fn merge_with(&self, other: &DebugFlags) -> DebugFlags {
        DebugFlags {
            inline_debug: other.inline_debug,
            template_debug: other.template_debug,
            macro_debug: other.macro_debug,
            type_debug: other.type_debug,
            variable_debug: other.variable_debug,
            parameter_debug: other.parameter_debug,
            scope_debug: other.scope_debug,
            line_debug: other.line_debug,
            column_debug: other.column_debug,
        }
    }
}

/// Builder for debug configuration
pub struct DebugConfigBuilder {
    config: DebugConfig,
}

impl DebugConfigBuilder {
    /// Create a new builder with default configuration
    pub fn new() -> Self {
        Self {
            config: DebugConfig::default(),
        }
    }
    
    /// Set whether to generate debug information
    pub fn debug_info(mut self, enabled: bool) -> Self {
        self.config.generate_debug_info = enabled;
        self
    }
    
    /// Set debug level
    pub fn level(mut self, level: u8) -> Self {
        self.config.debug_level = level;
        self
    }
    
    /// Set whether to include source code
    pub fn include_source(mut self, include: bool) -> Self {
        self.config.include_source = include;
        self
    }
    
    /// Set whether to generate optimized debug info
    pub fn optimized(mut self, optimized: bool) -> Self {
        self.config.optimized_debug = optimized;
        self
    }
    
    /// Set whether to compress debug sections
    pub fn compressed(mut self, compressed: bool) -> Self {
        self.config.compress_debug = compressed;
        self
    }
    
    /// Set whether to split debug information
    pub fn split_debug(mut self, split: bool) -> Self {
        self.config.split_debug_info = split;
        self
    }
    
    /// Set debug output directory
    pub fn output_dir(mut self, dir: PathBuf) -> Self {
        self.config.debug_output_dir = Some(dir);
        self
    }
    
    /// Set DWARF version
    pub fn dwarf_version(mut self, version: u8) -> Self {
        self.config.dwarf_version = version;
        self
    }
    
    /// Enable line debug information
    pub fn with_lines(mut self) -> Self {
        self.config.debug_flags.line_debug = true;
        self
    }
    
    /// Enable variable debug information
    pub fn with_variables(mut self) -> Self {
        self.config.debug_flags.variable_debug = true;
        self
    }
    
    /// Enable type debug information
    pub fn with_types(mut self) -> Self {
        self.config.debug_flags.type_debug = true;
        self
    }
    
    /// Build the configuration
    pub fn build(self) -> DebugConfig {
        self.config
    }
    
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

