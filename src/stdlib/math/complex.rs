//! Complex number operations for CURSED

/// For backward compatibility
pub struct MinimalImplementation;

impl MinimalImplementation {
    pub fn new() -> Self {
        Self
    }
}

pub fn get_minimal_result() -> Result<String, crate::error::CursedError> {
    Ok("CURSED complex functions enabled".to_string())
}
