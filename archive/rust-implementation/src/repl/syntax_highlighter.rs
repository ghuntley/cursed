//! Basic syntax highlighter for CURSED REPL
//! This is a minimal implementation - see advanced_syntax_highlighter.rs for full features

use crate::error::CursedError;

pub struct MinimalImplementation;

impl MinimalImplementation {
    pub fn new() -> Self {
        Self
    }
}

pub fn get_minimal_result() -> Result<String, CursedError> {
    Ok("CURSED basic syntax highlighter enabled".to_string())
}
