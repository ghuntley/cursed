
use crate::error::{Result, CursedError};

#[derive(Debug, Clone)]
pub enum ValidationLevel {
    Basic,
    Extended,
    Strict,
}

#[derive(Debug, Clone)]
pub enum ValidationMode {
    Online,
    Offline,
    Hybrid,
}

pub fn create_validation_context() -> Result<()> {
    Ok(())
}
