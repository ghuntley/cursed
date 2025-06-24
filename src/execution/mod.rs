// Minimal execution module for CURSED minimal build

use crate::error::{Error, Result};

// Basic value types for minimal build
#[derive(Debug, Clone)]
pub enum CursedValue {
    Nil,
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
}

impl std::fmt::Display for CursedValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CursedValue::Nil => write!(f, "nil"),
            CursedValue::Integer(i) => write!(f, "{}", i),
            CursedValue::Float(fl) => write!(f, "{}", fl),
            CursedValue::String(s) => write!(f, "\"{}\"", s),
            CursedValue::Boolean(b) => write!(f, "{}", b),
        }
    }
}

// Basic execution engine for minimal build
pub struct CursedExecutionEngine {
    // Minimal state
}

impl CursedExecutionEngine {
    pub fn new() -> Result<Self> {
        Ok(CursedExecutionEngine {})
    }
    
    pub fn execute(&mut self, source: &str) -> Result<CursedValue> {
        // Minimal implementation - just return nil for now
        tracing::info!("Executing CURSED source (minimal): {}", source.len());
        Ok(CursedValue::Nil)
    }
    
    pub fn execute_file(&mut self, path: &str) -> Result<CursedValue> {
        let source = std::fs::read_to_string(path)?;
        self.execute(&source)
    }
    
    pub fn execute_repl(&mut self, code: &str) -> Result<String> {
        let result = self.execute(code)?;
        Ok(format!("{}", result))
    }
    
    pub fn get_value_manager(&self) -> ValueManager {
        ValueManager {}
    }
}

// Basic value manager for minimal build
pub struct ValueManager {}

impl ValueManager {
    pub fn format_value(&self, value: &CursedValue) -> String {
        format!("{}", value)
    }
}

// Re-export submodules that exist
pub mod execution_context;
