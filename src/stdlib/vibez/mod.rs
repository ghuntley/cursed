use crate::error::Error;
/// Vibez - CURSED formatting and printing utilities with Gen Z flair
/// 
/// This module provides comprehensive formatting and printing functionality
/// for the CURSED programming language, including basic printing, advanced
/// formatting with placeholders, printf-style formatting, and debug utilities.

pub mod print;
pub mod format;
pub mod sprintf;
pub mod debug;

// Re-export all public functions for easy access with vibez_ prefix to avoid conflicts
pub use print::{
    print as vibez_print, println as vibez_println, 
    eprint as vibez_eprint, eprintln as vibez_eprintln, 
    print_to, println_to, print_styled, println_styled, 
    print_colored, println_colored,
    PrintStyle, PrintColor,
    // Spill functions - the core Gen Z I/O operations
    spill, spillf, spillstr, scan, scanln
};

pub use format::{
    format, format_args, format_with_context, interpolate,
    FormatError, FormatResult, FormatContext, FormatOptions,
    FormatPlaceholder, FormatSpec, PlaceholderType, FormatAlignment, FormatSign
};

pub use sprintf::{
    sprintf, snprintf, sprintf_to_writer, 
    SprintfError, SprintfResult, FormatSpecifier,
    validate_format_string, count_format_specifiers
};

pub use debug::{
    debug_print, debug_println, debug_format, pretty_print,
    debug_dump, debug_inspect, debug_trace,
    DebugOptions, DebugStyle, DebugLevel,
    set_debug_level, get_debug_level, is_debug_enabled
};

// Module initialization and utilities
use std::sync::Once;
static INIT: Once = Once::new();

/// Initialize the vibez module
pub fn initialize() {
    INIT.call_once(|| {
        debug::init_debug_system();
    });
}

/// Get module version and information
pub fn version() -> &'static str {
    "1.0.0"
}

/// Get module capabilities
pub fn capabilities() -> Vec<&'static str> {
    vec![
        "basic_printing",
        "advanced_formatting", 
        "printf_compatibility",
        "debug_utilities",
        "styled_output",
        "placeholder_interpolation"
    ]
}
