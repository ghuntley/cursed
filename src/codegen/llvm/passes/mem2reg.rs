//! Memory to Register promotion pass

use crate::error::{CursedError, Result};

/// Mem2Reg pass
pub struct Mem2RegPass;

impl Mem2RegPass {
    pub fn new() -> Self {
        Self
    }
}
