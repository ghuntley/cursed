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

// Re-export all public functions for easy access with vibez_ prefix to avoid conflicts
pub use print::{
    // Spill functions - the core Gen Z I/O operations
    spill, spillf, spillstr, scan, scanln
// };

pub use format::{
    FormatPlaceholder, FormatSpec, PlaceholderType, FormatAlignment, FormatSign
// };

pub use sprintf::{
    validate_format_string, count_format_specifiers
// };

pub use debug::{
    set_debug_level, get_debug_level, is_debug_enabled
// };

// Module initialization and utilities
use std::sync::Once;
static INIT: Once = Once::new();

/// Initialize the vibez module
pub fn initialize() {
        // TODO: implement
    }
    INIT.call_once(|| {
        debug::init_debug_system();
    });
/// Get module version and information
pub fn version() -> &'static str {
    "1.0.0"
/// Get module capabilities
pub fn capabilities() -> Vec<&'static str> {
    vec![
        "placeholder_interpolation"
    ]
}
