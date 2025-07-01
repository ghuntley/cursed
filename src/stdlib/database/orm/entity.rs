//! Entity trait and related types for CURSED ORM

use crate::error::CursedError;
use std::collections::HashMap;
use super::super::{SqlValue, DatabaseError};

/// Result type for entity operations
pub type ModuleResult<T> = Result<T, CursedError>;

/// Entity metadata for table introspection
#[derive(Debug, Clone)]
pub struct EntityMetadata {
    pub table_name: String,
    pub primary_key: String,
    pub fields: Vec<String>,
    pub relationships: Vec<String>, // For future use
    pub validation_rules: Vec<String>, // For future use
    pub indexes: Vec<String>, // For future use
    pub version: u32,
}

/// Column definition for schema generation
#[derive(Debug, Clone)]
pub struct ColumnDefinition {
    pub name: String,
    pub sql_type: String,
    pub is_nullable: bool,
    pub is_primary_key: bool,
    pub is_unique: bool,
    pub default_value: Option<SqlValue>,
}

impl ColumnDefinition {
    pub fn new(name: &str, sql_type: &str) -> Self {
        Self {
            name: name.to_string(),
            sql_type: sql_type.to_string(),
            is_nullable: false,
            is_primary_key: false,
            is_unique: false,
            default_value: None,
        }
    }
    
    pub fn nullable(mut self) -> Self {
        self.is_nullable = true;
        self
    }
    
    pub fn primary_key(mut self) -> Self {
        self.is_primary_key = true;
        self
    }
    
    pub fn unique(mut self) -> Self {
        self.is_unique = true;
        self
    }
    
    pub fn default_value(mut self, value: SqlValue) -> Self {
        self.default_value = Some(value);
        self
    }
}

/// Primary key attribute for entities
#[derive(Debug, Clone)]
pub struct PrimaryKey {
    pub field_name: String,
    pub auto_increment: bool,
}

impl PrimaryKey {
    pub fn new(field_name: &str) -> Self {
        Self {
            field_name: field_name.to_string(),
            auto_increment: true,
        }
    }
    
    pub fn manual(field_name: &str) -> Self {
        Self {
            field_name: field_name.to_string(),
            auto_increment: false,
        }
    }
}

/// Foreign key attribute for entities
#[derive(Debug, Clone)]
pub struct ForeignKey {
    pub field_name: String,
    pub references_table: String,
    pub references_column: String,
    pub on_delete: ForeignKeyAction,
    pub on_update: ForeignKeyAction,
}

#[derive(Debug, Clone)]
pub enum ForeignKeyAction {
    Cascade,
    SetNull,
    Restrict,
    NoAction,
}

impl ForeignKey {
    pub fn new(field_name: &str, references_table: &str, references_column: &str) -> Self {
        Self {
            field_name: field_name.to_string(),
            references_table: references_table.to_string(),
            references_column: references_column.to_string(),
            on_delete: ForeignKeyAction::NoAction,
            on_update: ForeignKeyAction::NoAction,
        }
    }
    
    pub fn on_delete(mut self, action: ForeignKeyAction) -> Self {
        self.on_delete = action;
        self
    }
    
    pub fn on_update(mut self, action: ForeignKeyAction) -> Self {
        self.on_update = action;
        self
    }
}

/// Timestamped trait for entities with created_at/updated_at fields
#[derive(Debug, Clone)]
pub struct Timestamped {
    pub created_at_field: String,
    pub updated_at_field: String,
    pub auto_update: bool,
}

impl Timestamped {
    pub fn new() -> Self {
        Self {
            created_at_field: "created_at".to_string(),
            updated_at_field: "updated_at".to_string(),
            auto_update: true,
        }
    }
    
    pub fn with_fields(created_at: &str, updated_at: &str) -> Self {
        Self {
            created_at_field: created_at.to_string(),
            updated_at_field: updated_at.to_string(),
            auto_update: true,
        }
    }
}

impl Default for Timestamped {
    fn default() -> Self {
        Self::new()
    }
}

/// entity operations handler
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
            return Err(CursedError::runtime_error("Module is disabled"));
        }
        Ok(format!("Processed: {}", data))
    }
    
    /// Get module info
    pub fn info(&self) -> String {
        format!("Module: entity, Enabled: {}", self.enabled)
    }
}

impl Default for ModuleHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Initialize entity processing
pub fn init_entity() -> ModuleResult<()> {
    let handler = ModuleHandler::new();
    let result = handler.process("test")?;
    if !result.contains("test") {
        return Err(CursedError::runtime_error("Module test failed"));
    }
    println!("⚙️  Module processing (entity) initialized");
    Ok(())
}

/// Test entity functionality
pub fn test_entity() -> ModuleResult<()> {
    let handler = ModuleHandler::new();
    let result = handler.process("Hello, CURSED!")?;
    if !result.contains("Hello, CURSED!") {
        return Err(CursedError::runtime_error("Module test failed"));
    }
    Ok(())
}
