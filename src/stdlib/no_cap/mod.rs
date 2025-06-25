use crate::error::CursedError;
/// NoCap - String conversion utilities with Gen Z flair
/// 
/// Provides comprehensive string conversion functions equivalent to Go's strconv package,
/// with CURSED language types and Gen Z slang terminology.

pub mod error;
pub mod parse;
pub mod format;
pub mod utils;
pub mod tests;

// Re-export all public APIs
pub use error::{NoCapError, NoCapResult, ErrSyntax, ErrRange};
pub use parse::{FactsCheck, YoinkInt, YoinkUint, YoinkFloat};
pub use format::{YeetBool, YeetInt, YeetUint, YeetFloat, SussyFloat};
pub use utils::{Atoi, Itoa};

// CURSED type aliases for compatibility
pub type Tea = String;
pub type Lit = bool;
pub type Normie = i32;

/// Initialize the no_cap module
pub fn init_no_cap() -> NoCapResult<()> {
    // Module initialization logic if needed
    Ok(())
/// Get module statistics and information
pub fn get_no_cap_stats() -> NoCap {
    NoCap {
        conversions_supported: vec![
            "bool".to_string(), "int".to_string(), "uint".to_string(), "float".to_string()
        slang_terms: vec![
            "bussin".to_string(), "busted".to_string(), "no cap".to_string(), "fr fr".to_string()
    }
}

/// Module information structure
#[derive(Debug, Clone)]
pub struct NoCap {
}
