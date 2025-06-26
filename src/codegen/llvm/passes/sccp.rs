//! Sparse Conditional Constant Propagation pass

use crate::error::{CursedError, Result};

/// SCCP pass
pub struct SccpPass;

impl SccpPass {
    pub fn new() -> Self {
        Self
    }
}
