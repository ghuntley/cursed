//! Functional implementation for core

use crate::error::CursedError;
use crate::stdlib::packages::ModuleError;

/// Result type for core operations
pub type ModuleResult<T> = Result<T, CursedError>;

/// core operations handler
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
            return Err(CursedError::runtime_error(&"Module is disabled"));
        }
        Ok(format!("Processed: {}", data))
    }
    
    /// Get module info
    pub fn info(&self) -> String {
        format!("Module: core, Enabled: {}", self.enabled)
    }
}

impl Default for ModuleHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// SQL value representation
#[derive(Debug, Clone, PartialEq)]
pub enum SqlValue {
    Null,
    Bool(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Bytes(Vec<u8>),
    DateTime(String),
    Decimal(String),
    Array(Vec<SqlValue>),
    Json(String),
}

impl SqlValue {
    pub fn is_null(&self) -> bool {
        matches!(self, SqlValue::Null)
    }
    
    pub fn as_str(&self) -> Option<&str> {
        match self {
            SqlValue::String(s) => Some(s),
            _ => None,
        }
    }
    
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            SqlValue::Integer(i) => Some(*i),
            _ => None,
        }
    }
    
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            SqlValue::Float(f) => Some(*f),
            _ => None,
        }
    }
    
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            SqlValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
}

impl From<&str> for SqlValue {
    fn from(s: &str) -> Self {
        SqlValue::String(s.to_string())
    }
}

impl From<String> for SqlValue {
    fn from(s: String) -> Self {
        SqlValue::String(s)
    }
}

impl From<i64> for SqlValue {
    fn from(i: i64) -> Self {
        SqlValue::Integer(i)
    }
}

impl From<f64> for SqlValue {
    fn from(f: f64) -> Self {
        SqlValue::Float(f)
    }
}

impl From<bool> for SqlValue {
    fn from(b: bool) -> Self {
        SqlValue::Bool(b)
    }
}

/// Initialize core processing
pub fn init_core() -> ModuleResult<()> {
    let handler = ModuleHandler::new();
    let result = handler.process("test")?;
    if !result.contains("test") {
        return Err(CursedError::runtime_error(&"Module test failed"));
    }
    println!("⚙️  Module processing (core) initialized");
    Ok(())
}

/// Test core functionality
pub fn test_core() -> ModuleResult<()> {
    let handler = ModuleHandler::new();
    let result = handler.process("Hello, CURSED!")?;
    if !result.contains("Hello, CURSED!") {
        return Err(CursedError::runtime_error(&"Module test failed"));
    }
    Ok(())
}
