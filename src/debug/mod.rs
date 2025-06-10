/// Enhanced debugging system for CURSED programming language
///
/// Provides comprehensive debugging capabilities including source mapping,
/// symbol resolution, runtime inspection, and LLVM debug integration.

pub mod enhanced_debug;
pub mod source_mapper;
pub mod symbol_resolver;
pub mod debug_config;
pub mod debug_info;
pub mod debug_symbols;
pub mod debug_utils;
pub mod dwarf_gen;

pub use enhanced_debug::*;
pub use source_mapper::*;
pub use symbol_resolver::*;

/// Re-exports for convenience
pub use crate::runtime::debug_info::{DebugInfo, VariableInfo, EnhancedStackTrace};
pub use crate::error::debug_context::{DebugContext, ErrorSeverity};

// Re-export both source location types for compatibility
pub use crate::error::SourceLocation as ErrorSourceLocation;
pub mod source_location;
pub use source_location::SourceLocation;

// Use the comprehensive debug config from debug_config module
pub use debug_config::DebugConfig;

// Placeholder types for compatibility
pub struct DebugInfoManager;
impl DebugInfoManager {
    pub fn new() -> Self { Self }
    pub fn initialize_compilation_unit(&mut self, _file: std::path::PathBuf, _producer: String) -> Result<(), crate::error::Error> { Ok(()) }
    pub fn begin_function(&mut self, _name: String, _location: ErrorSourceLocation) -> Result<(), crate::error::Error> { Ok(()) }
    pub fn generate_debug_location(&self, _location: &ErrorSourceLocation) -> String { String::new() }
    pub fn end_function(&mut self) -> Result<(), crate::error::Error> { Ok(()) }
    pub fn add_variable(&mut self, _name: String, _type_name: String, _location: ErrorSourceLocation) -> Result<(), crate::error::Error> { Ok(()) }
    pub fn generate_llvm_debug_metadata(&self) -> Result<String, crate::error::Error> { Ok(String::new()) }
    pub fn set_current_location(&mut self, _location: ErrorSourceLocation) {}
    pub fn current_location(&self) -> Option<ErrorSourceLocation> { None }
    pub fn generate_line_table(&self) -> Vec<(u32, String)> { Vec::new() }
    pub fn is_enabled(&self) -> bool { false }
    pub fn functions(&self) -> Vec<String> { vec![] }
    pub fn statistics(&self) -> DebugStatistics { DebugStatistics::default() }
    pub fn validate(&self) -> Result<(), Vec<String>> { Ok(()) }
    pub fn clear(&mut self) {}
    pub fn update_config(&mut self, _config: DebugConfig) {}
    pub fn config(&self) -> DebugConfig { 
        DebugConfig::default()
    }
}

#[derive(Debug, Default)]
pub struct DebugStatistics {
    pub symbol_count: usize,
}

impl std::fmt::Display for DebugStatistics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DebugStatistics {{ symbol_count: {} }}", self.symbol_count)
    }
}

// Note: debug_symbols module is already defined above and imported
