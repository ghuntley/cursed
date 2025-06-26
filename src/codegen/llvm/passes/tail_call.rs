//! Tail call optimization pass

use crate::error::{CursedError, Result};

/// Tail call pass
pub struct TailCallPass;

impl TailCallPass {
    pub fn new() -> Self {
        Self
    }
}
