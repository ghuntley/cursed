//! Jump threading optimization pass

use crate::error::{CursedError, Result};

/// Jump threading pass
pub struct JumpThreadingPass;

impl JumpThreadingPass {
    pub fn new() -> Self {
        Self
    }
}
