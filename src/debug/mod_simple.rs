// Debug Module for CURSED
//
// This module provides comprehensive debugging functionality including:
// - Enhanced debug information management
// - Source location tracking with full context
// - Symbol information with type details
// - Stack trace generation and formatting
// - DWARF debugging format support
// - Interactive debugging capabilities
// - Error message enhancement with source context

// Debug modules
pub mod simple_dwarf_integration;
pub mod enhanced_debug;

// Re-export key types
pub use crate::error::SourceLocation;
pub use enhanced_debug::{
    EnhancedDebugManager, EnhancedDebugInfo, DebugSymbol, SymbolType, RuntimeState,
    StackFrame, VariableDebugInfo, SourceContext, TypeInfo, FieldInfo,
    create_debug_symbol, create_stack_frame, create_variable_debug_info
};

// Legacy placeholder types for backward compatibility
#[derive(Debug, Clone)]
pub struct DebugInfoRegistry {
    pub symbols: std::collections::HashMap<String, SymbolMetadata>,
}

#[derive(Debug, Clone)]  
pub struct SymbolMetadata {
    pub name: String,
    pub symbol_type: LegacySymbolType,
    pub type_info: Option<TypeDebugInfo>,
}

#[derive(Debug, Clone)]
pub struct TypeDebugInfo {
    pub type_name: String,
    pub type_kind: TypeKind,
    pub size: usize,
}

#[derive(Debug, Clone)]
pub struct SourceMap {
    pub mappings: std::collections::HashMap<u32, SourceLocation>,
}

#[derive(Debug, Clone)]
pub enum LegacySymbolType {
    Function,
    Variable,
    Type,
    Module,
}

#[derive(Debug, Clone)]
pub enum TypeKind {
    Primitive,
    Struct,
    Enum,
    Function,
    Array,
    Pointer,
}

impl Default for DebugInfoRegistry {
    fn default() -> Self {
        Self {
            symbols: std::collections::HashMap::new(),
        }
    }
}

impl Default for SourceMap {
    fn default() -> Self {
        Self {
            mappings: std::collections::HashMap::new(),
        }
    }
}

/// Configuration for debug functionality
#[derive(Debug, Clone)]
pub struct DebugConfig {
    pub enabled: bool,
    pub include_source: bool,
    pub generate_dwarf: bool,
    pub verbose_traces: bool,
}

impl Default for DebugConfig {
    fn default() -> Self {
        Self { 
            enabled: false,
            include_source: true,
            generate_dwarf: false,
            verbose_traces: false,
        }
    }
}

/// Debug information structure
#[derive(Debug, Clone)]
pub struct DebugInfo {
    pub source_file: Option<String>,
    pub line_number: Option<u32>,
    pub column_number: Option<u32>,
    pub function_name: Option<String>,
    pub module_name: Option<String>,
}

impl Default for DebugInfo {
    fn default() -> Self {
        Self {
            source_file: None,
            line_number: None,
            column_number: None,
            function_name: None,
            module_name: None,
        }
    }
}

/// Debug information manager
#[derive(Debug)]
pub struct DebugInfoManager {
    pub config: DebugConfig,
    pub debug_symbols: Vec<LegacyDebugSymbol>,
}

impl Default for DebugInfoManager {
    fn default() -> Self {
        Self { 
            config: DebugConfig::default(),
            debug_symbols: vec![],
        }
    }
}

impl DebugInfoManager {
    /// Create new debug info manager with configuration
    pub fn new(config: DebugConfig) -> Self {
        Self {
            config,
            debug_symbols: vec![],
        }
    }

    /// Add debug symbol
    pub fn add_symbol(&mut self, symbol: LegacyDebugSymbol) {
        self.debug_symbols.push(symbol);
    }

    /// Find symbol by name
    pub fn find_symbol(&self, name: &str) -> Option<&LegacyDebugSymbol> {
        self.debug_symbols.iter().find(|s| s.name == name)
    }
}

#[derive(Debug, Clone)]
pub struct LegacyDebugSymbol {
    pub name: String,
    pub symbol_type: LegacyDebugSymbolType,
    pub address: u64,
    pub size: u32,
}

#[derive(Debug, Clone)]
pub enum LegacyDebugSymbolType {
    Function,
    Variable,
    Type,
    Module,
}

impl LegacyDebugSymbol {
    pub fn new(name: String, symbol_type: LegacyDebugSymbolType, address: u64, size: u32) -> Self {
        Self {
            name,
            symbol_type,
            address,
            size,
        }
    }
}
