use crate::error::CursedError;
/// GlyphGang Unicode package - Unicode character processing with CURSED flair
/// 
/// This module provides comprehensive Unicode support including:
/// - Character classification and property testing
/// - Character case conversion 
/// - Unicode range tables and property support
/// - Enhanced string operations with Unicode awareness
/// - Emoji detection and processing
/// - Bidirectional text support
/// - Script detection and analysis
/// - International text utilities

pub mod core;
pub mod ranges;
pub mod error;
pub mod string_ops;
pub mod emoji;
pub mod bidi;
pub mod script;

// Re-export core functionality
pub use core::{
    // Character classification functions
    
    // Advanced classifications
    
    // Character conversion functions
    
    // Range and character set functions
    
    // Character properties
// };

// Re-export range tables and Unicode properties
pub use ranges::{
    
    // Predefined range tables
    
    // Script tables
    
    // Special categories
// };

// Re-export string operations
pub use string_ops::{
// };

// Re-export emoji support
pub use emoji::{
// };

// Re-export bidirectional text support
pub use bidi::{
// };

// Re-export script detection
pub use script::{
// };

// Re-export error types
pub use error::{
// };

/// Initialize the GlyphGang module
pub fn initialize() -> GlyphGangResult<()> {
    // Initialize Unicode data tables
    ranges::initialize_tables()?;
    
    // Initialize emoji data
    emoji::initialize_emoji_data()?;
    
    // Initialize script detection data
    script::initialize_script_data()?;
    
    Ok(())
/// Get module version and capabilities
pub fn version() -> &'static str {
    "1.0.0"
/// Get supported Unicode version
pub fn unicode_version() -> &'static str {
    "15.0.0"
/// Get module capabilities
pub fn capabilities() -> Vec<&'static str> {
    vec![
    ]
}
