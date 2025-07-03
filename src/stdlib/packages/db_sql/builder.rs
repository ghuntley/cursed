//! Functional implementation for builder

use crate::error::CursedError;
use crate::stdlib::packages::ModuleError;

/// Result type for builder operations
pub type ModuleResult<T> = Result<T, CursedError>;

/// builder operations handler
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
        format!("Module: builder, Enabled: {}", self.enabled)
    }
}

impl Default for ModuleHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// SQL DELETE builder
pub struct DeleteBuilder {
    table: Option<String>,
    where_clause: Option<String>,
}

impl DeleteBuilder {
    pub fn new() -> Self {
        DeleteBuilder {
            table: None,
            where_clause: None,
        }
    }
    
    pub fn from(mut self, table: &str) -> Self {
        self.table = Some(table.to_string());
        self
    }
    
    pub fn where_clause(mut self, condition: &str) -> Self {
        self.where_clause = Some(condition.to_string());
        self
    }
    
    pub fn build(&self) -> String {
        let mut sql = String::new();
        if let Some(table) = &self.table {
            sql.push_str(&format!("DELETE FROM {}", table));
            if let Some(where_clause) = &self.where_clause {
                sql.push_str(&format!(" WHERE {}", where_clause));
            }
        }
        sql
    }
}

/// SQL CREATE TABLE builder
pub struct CreateTableBuilder {
    table: Option<String>,
    columns: Vec<String>,
}

impl CreateTableBuilder {
    pub fn new() -> Self {
        CreateTableBuilder {
            table: None,
            columns: Vec::new(),
        }
    }
    
    pub fn table(mut self, name: &str) -> Self {
        self.table = Some(name.to_string());
        self
    }
    
    pub fn column(mut self, definition: &str) -> Self {
        self.columns.push(definition.to_string());
        self
    }
    
    pub fn build(&self) -> String {
        if let Some(table) = &self.table {
            format!("CREATE TABLE {} ({})", table, self.columns.join(", "))
        } else {
            String::new()
        }
    }
}

/// SQL ALTER TABLE builder
pub struct AlterTableBuilder {
    table: Option<String>,
    actions: Vec<String>,
}

impl AlterTableBuilder {
    pub fn new() -> Self {
        AlterTableBuilder {
            table: None,
            actions: Vec::new(),
        }
    }
    
    pub fn table(mut self, name: &str) -> Self {
        self.table = Some(name.to_string());
        self
    }
    
    pub fn add_column(mut self, definition: &str) -> Self {
        self.actions.push(format!("ADD COLUMN {}", definition));
        self
    }
    
    pub fn drop_column(mut self, name: &str) -> Self {
        self.actions.push(format!("DROP COLUMN {}", name));
        self
    }
    
    pub fn build(&self) -> String {
        if let Some(table) = &self.table {
            format!("ALTER TABLE {} {}", table, self.actions.join(", "))
        } else {
            String::new()
        }
    }
}

/// Initialize builder processing
pub fn init_builder() -> ModuleResult<()> {
    let handler = ModuleHandler::new();
    let result = handler.process("test")?;
    if !result.contains("test") {
        return Err(CursedError::runtime_error(&"Module test failed".to_string()));
    }
    println!("⚙️  Module processing (builder) initialized");
    Ok(())
}

/// Test builder functionality
pub fn test_builder() -> ModuleResult<()> {
    let handler = ModuleHandler::new();
    let result = handler.process("Hello, CURSED!")?;
    if !result.contains("Hello, CURSED!") {
        return Err(CursedError::runtime_error(&"Module test failed".to_string()));
    }
    Ok(())
}
