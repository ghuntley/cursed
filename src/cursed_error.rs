// Core CURSED error type for high-level operations
// This provides a unified error interface for the CURSED language

use crate::error_types::Error;

/// Main CURSED error type - a wrapper around the core Error enum
#[derive(Debug, Clone)]
pub struct CursedError {
    pub error: Error,
    pub context: Option<String>,
}

impl CursedError {
    pub fn new(error: Error) -> Self {
        Self { error, context: None }
    }

    pub fn with_context(mut self, context: String) -> Self {
        self.context = Some(context);
        self
    }
}

impl From<Error> for CursedError {
    fn from(error: Error) -> Self {
        Self::new(error)
    }
}

impl std::fmt::Display for CursedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(context) = &self.context {
            write!(f, "{}: {}", context, self.error)
        } else {
            write!(f, "{}", self.error)
        }
    }
}

impl std::error::Error for CursedError {}
