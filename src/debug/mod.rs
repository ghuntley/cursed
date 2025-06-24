// Debug modules for CURSED
pub mod enhanced_debug;
pub mod source_location;

// Re-export key types
pub use crate::error::SourceLocation;
pub use enhanced_debug::EnhancedDebug;
pub use source_location::SourceLocationInfo;

#[derive(Debug, Clone)]
pub struct DebugConfig {
    pub enabled: bool,
}

impl Default for DebugConfig {
    fn default() -> Self {
        Self { enabled: false }
    }
}

/// Debug information structure
#[derive(Debug, Clone)]
pub struct DebugInfo {
    pub source_location: SourceLocation,
    pub line: u32,
    pub column: u32,
}

/// Debug information manager
#[derive(Debug)]
pub struct DebugInfoManager {
    pub enabled: bool,
}

impl Default for DebugInfoManager {
    fn default() -> Self {
        Self { enabled: false }
    }
}

/// DWARF generator module
pub mod dwarf_gen {
    use crate::error::Error;
    
    #[derive(Debug)]
    pub struct DwarfGenerator {
        pub enabled: bool,
    }
    
    impl DwarfGenerator {
        pub fn new() -> Result<Self, Error> {
            Ok(Self { enabled: false })
        }
    }
}

/// Debug symbols module
pub mod debug_symbols {
    #[derive(Debug, Clone)]
    pub struct DebugSymbol {
        pub name: String,
        pub symbol_type: DebugSymbolType,
    }
    
    #[derive(Debug, Clone)]
    pub enum DebugSymbolType {
        Function,
        Variable,
        Type,
    }
}
