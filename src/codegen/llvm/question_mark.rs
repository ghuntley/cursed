// Minimal questionmark module - disabled for minimal build
use crate::error::{Error, Result};

// Minimal placeholder implementations
pub struct QuestionmarkDisabled {}

impl QuestionmarkDisabled {
    pub fn new() -> Result<Self> {
        Err(Error::NotImplemented(
            "questionmark is disabled in minimal build".to_string()
        ))
    }
}
