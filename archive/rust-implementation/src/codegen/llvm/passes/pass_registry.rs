//! Pass registry for managing optimization passes

use crate::error::{CursedError, Result};
use std::collections::HashMap;

/// Registry for optimization passes
pub struct PassRegistry {
    passes: HashMap<String, PassRegistration>,
}

impl PassRegistry {
    pub fn new() -> Self {
        Self {
            passes: HashMap::new(),
        }
    }
}

/// Pass registration information
pub struct PassRegistration {
    pub name: String,
    pub description: String,
}

/// Pass dependencies
pub struct PassDependency {
    pub name: String,
    pub required: bool,
}
