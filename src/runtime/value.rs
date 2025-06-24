// Minimal value module for CURSED minimal build

use crate::error::{Error, Result};

pub use crate::execution::CursedValue;

pub struct ValueManager {
    // Minimal implementation  
}

impl ValueManager {
    pub fn new() -> Self {
        ValueManager {}
    }
    
    pub fn format_value(&self, value: &CursedValue) -> String {
        format!("{}", value)
    }
}
