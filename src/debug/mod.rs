/// Enhanced debugging system for CURSED programming language
///
/// Provides comprehensive debugging capabilities including source mapping,
/// symbol resolution, runtime inspection, and LLVM debug integration.

pub mod enhanced_debug;
pub mod source_mapper;
pub mod symbol_resolver;

pub use enhanced_debug::*;
pub use source_mapper::*;
pub use symbol_resolver::*;

/// Re-exports for convenience
pub use crate::runtime::debug_info::{DebugInfo, VariableInfo, EnhancedStackTrace};
pub use crate::error::debug_context::{DebugContext, ErrorSeverity};

// Re-export common debug types that LLVM codegen expects
pub use crate::error::SourceLocation;

// Debug configuration struct
#[derive(Debug, Clone)]
pub struct DebugConfig {
    pub enabled: bool,
    pub optimization_level: u32,
    pub include_source_maps: bool,
    pub optimized_debug: bool,
}

impl Default for DebugConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            optimization_level: 0,
            include_source_maps: true,
            optimized_debug: false,
        }
    }
}

// Placeholder types for compatibility
pub struct DebugInfoManager;
impl DebugInfoManager {
    pub fn new() -> Self { Self }
    pub fn initialize_compilation_unit(&mut self, _file: std::path::PathBuf, _producer: String) -> Result<(), crate::error::Error> { Ok(()) }
    pub fn begin_function(&mut self, _name: String, _location: crate::error::SourceLocation) -> Result<(), crate::error::Error> { Ok(()) }
    pub fn generate_debug_location(&self, _location: &crate::error::SourceLocation) -> String { String::new() }
    pub fn end_function(&mut self) -> Result<(), crate::error::Error> { Ok(()) }
    pub fn add_variable(&mut self, _name: String, _type_name: String, _location: crate::error::SourceLocation) -> Result<(), crate::error::Error> { Ok(()) }
    pub fn generate_llvm_debug_metadata(&self) -> Result<String, crate::error::Error> { Ok(String::new()) }
    pub fn set_current_location(&mut self, _location: crate::error::SourceLocation) {}
    pub fn current_location(&self) -> Option<crate::error::SourceLocation> { None }
    pub fn generate_line_table(&self) -> Vec<(u32, String)> { Vec::new() }
    pub fn is_enabled(&self) -> bool { false }
    pub fn statistics(&self) -> DebugStatistics { DebugStatistics::default() }
    pub fn validate(&self) -> Result<(), Vec<String>> { Ok(()) }
    pub fn clear(&mut self) {}
    pub fn update_config(&mut self, _config: DebugConfig) {}
    pub fn config(&self) -> &DebugConfig { 
        static CONFIG: DebugConfig = DebugConfig {
            enabled: true,
            optimization_level: 0,
            include_source_maps: true,
            optimized_debug: false,
        };
        &CONFIG
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

pub mod debug_symbols {
    pub struct DebugSymbolTable;
    impl DebugSymbolTable {
        pub fn new() -> Self { Self }
        pub fn enter_scope(&mut self) {}
        pub fn exit_scope(&mut self) {}
        pub fn add_symbol(&mut self, _symbol: DebugSymbol) {}
    }
    
    pub struct DebugSymbol;
    impl DebugSymbol {
        pub fn variable(_name: String, _type_info: String, _location: crate::error::SourceLocation) -> Self {
            Self
        }
    }
}
