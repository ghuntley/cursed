//! Functional implementation for result

use crate::error::CursedError;

/// Result type for result operations
pub type ModuleResult<T> = Result<T, CursedError>;

/// result operations handler
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
        format!("Module: result, Enabled: {}", self.enabled)
    }
}

impl Default for ModuleHandler {
    fn default() -> Self {
        Self::new()
    }
}

use super::SqlValue;
use crate::stdlib::packages::ModuleError;

/// SQL result set
#[derive(Debug)]
pub struct SqlResultSet {
    pub rows: Vec<Vec<SqlValue>>,
    pub columns: Vec<String>,
}

impl SqlResultSet {
    pub fn new(columns: Vec<String>) -> Self {
        SqlResultSet {
            rows: Vec::new(),
            columns,
        }
    }
    
    pub fn add_row(&mut self, row: Vec<SqlValue>) {
        self.rows.push(row);
    }
    
    pub fn len(&self) -> usize {
        self.rows.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
    
    pub fn iter(&self) -> SqlRowIterator {
        SqlRowIterator::new(self)
    }
}

/// SQL execution result
#[derive(Debug)]
pub struct SqlExecuteResult {
    pub rows_affected: u64,
    pub last_insert_id: Option<i64>,
}

impl SqlExecuteResult {
    pub fn new(rows_affected: u64) -> Self {
        SqlExecuteResult {
            rows_affected,
            last_insert_id: None,
        }
    }
    
    pub fn with_insert_id(mut self, id: i64) -> Self {
        self.last_insert_id = Some(id);
        self
    }
}

/// Iterator for SQL rows
pub struct SqlRowIterator<'a> {
    result_set: &'a SqlResultSet,
    current_index: usize,
}

impl<'a> SqlRowIterator<'a> {
    pub fn new(result_set: &'a SqlResultSet) -> Self {
        SqlRowIterator {
            result_set,
            current_index: 0,
        }
    }
}

impl<'a> Iterator for SqlRowIterator<'a> {
    type Item = &'a Vec<SqlValue>;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index < self.result_set.rows.len() {
            let row = &self.result_set.rows[self.current_index];
            self.current_index += 1;
            Some(row)
        } else {
            None
        }
    }
}

/// Initialize result processing
pub fn init_result() -> ModuleResult<()> {
    let handler = ModuleHandler::new();
    let result = handler.process("test")?;
    if !result.contains("test") {
        return Err(CursedError::runtime_error(&"Module test failed"));
    }
    println!("⚙️  Module processing (result) initialized");
    Ok(())
}

/// Test result functionality
pub fn test_result() -> ModuleResult<()> {
    let handler = ModuleHandler::new();
    let result = handler.process("Hello, CURSED!")?;
    if !result.contains("Hello, CURSED!") {
        return Err(CursedError::runtime_error(&"Module test failed"));
    }
    Ok(())
}
