use crate::error::CursedError;
/// Vibez - CURSED formatting and printing utilities with Gen Z flair
/// 
/// This module provides comprehensive formatting and printing functionality
/// for the CURSED programming language, including basic printing, advanced
/// formatting with placeholders, printf-style formatting, and debug utilities.

pub mod print;
pub mod format;
pub mod sprintf;
pub mod debug;

// Re-export all public functions for easy access
pub use print::{
    // Spill functions - the core Gen Z I/O operations
    spill, spillf, spillstr, scan, scanln
};

pub use debug::{
    // Debug functions
    init_debug_system, set_debug_level, get_debug_level, is_debug_enabled,
    debug_log, debug_inspect, debug_error, debug_warn, debug_info, debug_debug, debug_trace,
    set_debug_enabled, get_debug_stats, reset_debug_stats, debug_config
};

// Stub implementations for format, sprintf, and debug modules
// TODO: Implement these properly later

// Module initialization and utilities
use std::sync::Once;
static INIT: Once = Once::new();

/// Initialize the vibez module
pub fn initialize() {
    INIT.call_once(|| {
        // Initialize any needed systems
        println!("🔥 vibez module initialized - ready to spill some facts!");
    });
}

/// Get module version and information
pub fn version() -> &'static str {
    "1.0.0"
}

/// Get module capabilities
pub fn capabilities() -> Vec<&'static str> {
    vec![
        "spill_output",
        "format_strings",
        "stdin_input"
    ]
}
