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
pub mod debug_info_manager;

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

// Use the real debug info manager implementation
pub use debug_info_manager::DebugInfoManager;

// Use the comprehensive debug statistics from enhanced_debug module
pub use enhanced_debug::DebugStatistics;

// Note: debug_symbols module is already defined above and imported
