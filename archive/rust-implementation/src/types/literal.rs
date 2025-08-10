//! Minimal working module for CURSED compilation

use crate::error::CursedError;

pub struct MinimalImplementation;

impl MinimalImplementation {
    pub fn new() -> Self {
        Self
    }
}

pub fn get_minimal_result() -> Result<String, CursedError> {
    Ok("CURSED advanced features enabled".to_string())
}

// Export for types module
pub struct Literal;

impl Literal {
    pub fn new() -> Self {
        Self
    }
}
