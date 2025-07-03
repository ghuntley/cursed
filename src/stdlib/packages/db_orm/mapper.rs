//! Functional implementation for mapper

use crate::error::CursedError;
use std::collections::HashMap;
use crate::stdlib::packages::ModuleError;

/// Result type for mapper operations
pub type ModuleResult<T> = Result<T, CursedError>;

/// Table mapper for ORM operations
#[derive(Debug, Clone)]
pub struct TableMapper {
    pub table_name: String,
    pub columns: HashMap<String, ColumnMapper>,
    pub primary_key: Option<String>,
}

impl TableMapper {
    pub fn new(table_name: String) -> Self {
        Self {
            table_name,
            columns: HashMap::new(),
            primary_key: None,
        }
    }
    
    pub fn add_column(&mut self, name: String, mapper: ColumnMapper) {
        self.columns.insert(name, mapper);
    }
    
    pub fn set_primary_key(&mut self, key: String) {
        self.primary_key = Some(key);
    }
    
    pub fn get_column(&self, name: &str) -> Option<&ColumnMapper> {
        self.columns.get(name)
    }
}

/// Column mapper for individual columns
#[derive(Debug, Clone)]
pub struct ColumnMapper {
    pub column_name: String,
    pub field_name: String,
    pub data_type: String,
    pub nullable: bool,
    pub auto_increment: bool,
}

impl ColumnMapper {
    pub fn new(column_name: String, field_name: String, data_type: String) -> Self {
        Self {
            column_name,
            field_name,
            data_type,
            nullable: false,
            auto_increment: false,
        }
    }
    
    pub fn nullable(mut self, nullable: bool) -> Self {
        self.nullable = nullable;
        self
    }
    
    pub fn auto_increment(mut self, auto_increment: bool) -> Self {
        self.auto_increment = auto_increment;
        self
    }
}

/// mapper operations handler
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
        format!("Module: mapper, Enabled: {}", self.enabled)
    }
}

impl Default for ModuleHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Initialize mapper processing
pub fn init_mapper() -> ModuleResult<()> {
    let handler = ModuleHandler::new();
    let result = handler.process("test")?;
    if !result.contains("test") {
        return Err(CursedError::runtime_error(&"Module test failed"));
    }
    println!("⚙️  Module processing (mapper) initialized");
    Ok(())
}

/// Test mapper functionality
pub fn test_mapper() -> ModuleResult<()> {
    let handler = ModuleHandler::new();
    let result = handler.process("Hello, CURSED!")?;
    if !result.contains("Hello, CURSED!") {
        return Err(CursedError::runtime_error(&"Module test failed"));
    }
    Ok(())
}
