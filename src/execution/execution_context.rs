// Minimal execution context for CURSED minimal build

use crate::error::{CursedError, Result};
use crate::execution::CursedValue;

pub struct ExecutionContext {
    // Minimal implementation
}

impl ExecutionContext {
    pub fn new() -> Self {
        ExecutionContext {}
    }
    
    pub fn push_scope(&mut self) -> Result<()> {
        Ok(())
    }
    
    pub fn pop_scope(&mut self) -> Result<()> {
        Ok(())
    }
    
    pub fn set_variable(&mut self, _name: &str, _value: CursedValue) -> Result<()> {
        Ok(())
    }
}

pub struct GlobalExecutionContext {
    // Minimal implementation
}

impl GlobalExecutionContext {
    pub fn new() -> Self {
        GlobalExecutionContext {}
    }
    
    pub fn push_scope(&mut self) -> Result<()> {
        Ok(())
    }
    
    pub fn set_variable(&mut self, _name: &str, _value: CursedValue) -> Result<()> {
        Ok(())
    }
}
