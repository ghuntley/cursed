//! Functional implementation for types

use crate::error::CursedError;
use crate::stdlib::packages::ModuleError;

/// Result type for types operations
pub type ModuleResult<T> = Result<T, CursedError>;

/// types operations handler
pub struct ModuleHandler {
    enabled: bool,
}

impl ModuleHandler {
    /// Create a new module handler
    pub fn new() -> Self {
        Self {
            enabled: true,
        }
    }
    
    /// Enable or disable the module
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
    
    /// Check if module is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    /// Process data
    pub fn process(&self, data: &str) -> ModuleResult<String> {
        if !self.enabled {
            return Err(CursedError::runtime_error(&"Module is disabled".to_string()));
        }
        Ok(format!("Processed: {}", data))
    }
    
    /// Get module info
    pub fn info(&self) -> String {
        format!("Module: types, Enabled: {}", self.enabled)
    }
}

impl Default for ModuleHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Initialize types processing
pub fn init_types() -> ModuleResult<()> {
    let handler = ModuleHandler::new();
    let result = handler.process("test")?;
    if !result.contains("test") {
        return Err(CursedError::runtime_error(&"Module test failed".to_string()));
    }
    println!("⚙️  Module processing (types) initialized");
    Ok(())
}

/// Test types functionality
pub fn test_types() -> ModuleResult<()> {
    let handler = ModuleHandler::new();
    let result = handler.process("Hello, CURSED!")?;
    if !result.contains("Hello, CURSED!") {
        return Err(CursedError::runtime_error(&"Module test failed".to_string()));
    }
    Ok(())
}
