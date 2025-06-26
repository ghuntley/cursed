// Debug Module for CURSED
//
// This module provides debugging functionality including:
// - Debug information management
// - Source location tracking
// - Symbol information
// - Stack trace generation
// - DWARF debugging format support

// TODO: Enable these modules once they are implemented
// pub mod enhanced_debug;
// pub mod debug_info_types;
// pub mod source_location;
// pub mod dwarf_gen;
// pub mod debug_info;

// Re-export key types
pub use crate::error::SourceLocation;

// Placeholder types for missing debug imports
#[derive(Debug, Clone)]
pub struct EnhancedDebugInfo {
    pub debug_info: DebugInfo,
    pub enhanced_features: bool,
}

#[derive(Debug, Clone)]
pub struct DebugInfoRegistry {
    pub symbols: std::collections::HashMap<String, SymbolMetadata>,
}

#[derive(Debug, Clone)]  
pub struct SymbolMetadata {
    pub name: String,
    pub symbol_type: SymbolType,
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
pub enum SymbolType {
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

impl Default for EnhancedDebugInfo {
    fn default() -> Self {
        Self {
            debug_info: DebugInfo::default(),
            enhanced_features: false,
        }
    }
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
    pub debug_symbols: Vec<DebugSymbol>,
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
    pub fn add_symbol(&mut self, symbol: DebugSymbol) {
        self.debug_symbols.push(symbol);
    }

    /// Find symbol by name
    pub fn find_symbol(&self, name: &str) -> Option<&DebugSymbol> {
        self.debug_symbols.iter().find(|s| s.name == name)
    }
}

/// DWARF generator module
pub mod dwarf_gen {
    use crate::error::CursedError;
    
    #[derive(Debug)]
    pub struct DwarfGenerator {
        pub enabled: bool,
    }
    
    impl DwarfGenerator {
        pub fn new() -> crate::error::Result<Self> {
            Ok(Self { enabled: false })
        }
        
        pub fn generate_dwarf(&self, _debug_info: &super::DebugInfo) -> crate::error::Result<Vec<u8>> {
            if !self.enabled {
                return Err(CursedError::General("DWARF generation disabled".to_string()));
            }
            // TODO: Implement DWARF generation
            Ok(vec![])
        }
    }
    
    impl Default for DwarfGenerator {
        fn default() -> Self {
            Self { enabled: false }
        }
    }
}

/// Debug symbols module
pub mod debug_symbols {
    #[derive(Debug, Clone)]
    pub struct DebugSymbol {
        pub name: String,
        pub symbol_type: DebugSymbolType,
        pub address: u64,
        pub size: u32,
    }
    
    #[derive(Debug, Clone)]
    pub enum DebugSymbolType {
        Function,
        Variable,
        Type,
        Module,
    }
    
    impl DebugSymbol {
        pub fn new(name: String, symbol_type: DebugSymbolType, address: u64, size: u32) -> Self {
            Self {
                name,
                symbol_type,
                address,
                size,
            }
        }
    }
}

// Re-export debug symbols for easier access
pub use debug_symbols::{DebugSymbol, DebugSymbolType};
