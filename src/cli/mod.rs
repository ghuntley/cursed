// CLI Module for CURSED
//
// This module provides command-line interface functionality including:
// - Debug CLI commands and options
// - Build optimization commands
// - Package management interface
// - Documentation generation
// - Test execution commands

pub mod debug_cli;
pub mod build_optimization;
pub mod package_manager;
pub mod documentation;
pub mod test_commands;
pub mod optimization_commands;
pub mod jit_commands;
pub mod pgo_commands;
pub mod bootstrap;
pub mod docs_enhanced;
pub mod pgo;

// Re-export key types
pub use debug_cli::{DebugCli, DebugCommand, DebugCliHandler, DebugFormat, ReportFormat};
