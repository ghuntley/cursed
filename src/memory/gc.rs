// Minimal garbage collector for CURSED minimal build

use crate::error::{Error, Result};

pub struct GarbageCollector {
    // Minimal implementation
}

impl GarbageCollector {
    pub fn new() -> Self {
        GarbageCollector {}
    }
    
    pub fn collect(&mut self) -> Result<()> {
        // No-op for minimal build
        Ok(())
    }
    
    pub fn allocate(&mut self, _size: usize) -> Result<*mut u8> {
        Err(Error::NotImplemented("GC allocation disabled in minimal build".to_string()))
    }
}

impl Default for GarbageCollector {
    fn default() -> Self {
        Self::new()
    }
}
