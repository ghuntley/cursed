// Minimal stack module for CURSED minimal build

use crate::error::{CursedError, Result};

pub struct RuntimeStack {
    // Minimal implementation
}

impl RuntimeStack {
    pub fn new() -> Self {
        RuntimeStack {}
    }
    
    pub fn push_frame(&mut self) -> Result<()> {
        Ok(())
    }
    
    pub fn pop_frame(&mut self) -> Result<()> {
        Ok(())
    }
}
