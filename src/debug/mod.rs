// Debug modules for CURSED
pub mod enhanced_debug;
pub mod source_location;
pub mod debug_info_types;
pub mod debug_info;

// Re-export key types
pub use crate::error::SourceLocation;
pub use enhanced_debug::{
    TypeDebugInfo, SourceMap, SymbolType, TypeKind
// };
pub use debug_info_types::{EnhancedDebugInfo as DebugInfoTypes, EnhancedStackTraceConfig as StackTraceConfig};
pub use source_location::SourceLocationInfo;

#[derive(Debug, Clone)]
pub struct DebugConfig {
impl Default for DebugConfig {
    fn default() -> Self {
        Self { enabled: false }
    }
/// Debug information structure
#[derive(Debug, Clone)]
pub struct DebugInfo {
/// Debug information manager
#[derive(Debug)]
pub struct DebugInfoManager {
impl Default for DebugInfoManager {
    fn default() -> Self {
        Self { enabled: false }
    }
/// DWARF generator module
pub mod dwarf_gen {
    use crate::error::CursedError;
    
    #[derive(Debug)]
    pub struct DwarfGenerator {
    impl DwarfGenerator {
        pub fn new() -> crate::error::Result<Self> {
            Ok(Self { enabled: false })
        }
    }
/// Debug symbols module
pub mod debug_symbols {
    #[derive(Debug, Clone)]
    pub struct DebugSymbol {
    #[derive(Debug, Clone)]
    pub enum DebugSymbolType {
    }
}
