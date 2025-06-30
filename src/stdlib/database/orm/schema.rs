//! Functional implementation for schema

use crate::error::CursedError;
use std::collections::HashMap;
use super::migration::{ColumnDefinition, ColumnType, TableConstraint};

/// Result type for schema operations
pub type ModuleResult<T> = Result<T, CursedError>;

/// Database schema representation
#[derive(Debug, Clone)]
pub struct DatabaseSchema {
    pub name: String,
    pub tables: HashMap<String, TableSchema>,
    pub version: String,
    pub created_at: String,
    pub updated_at: String,
}

/// Table schema definition
#[derive(Debug, Clone)]
pub struct TableSchema {
    pub name: String,
    pub columns: HashMap<String, ColumnDefinition>,
    pub constraints: Vec<TableConstraint>,
    pub indexes: Vec<IndexDefinition>,
}

/// Index definition
#[derive(Debug, Clone)]
pub struct IndexDefinition {
    pub name: String,
    pub columns: Vec<String>,
    pub unique: bool,
    pub index_type: IndexType,
}

/// Index types
#[derive(Debug, Clone)]
pub enum IndexType {
    BTree,
    Hash,
    GiST,
    GIN,
    Full,
}

/// Schema comparison result
#[derive(Debug, Clone)]
pub struct SchemaComparison {
    pub differences: Vec<SchemaDifference>,
    pub is_compatible: bool,
}

/// Schema difference types
#[derive(Debug, Clone)]
pub enum SchemaDifference {
    TableAdded { name: String },
    TableRemoved { name: String },
    TableModified { name: String, changes: Vec<TableChange> },
    IndexAdded { table: String, index: String },
    IndexRemoved { table: String, index: String },
    ConstraintAdded { table: String, constraint: String },
    ConstraintRemoved { table: String, constraint: String },
}

/// Table-level changes
#[derive(Debug, Clone)]
pub enum TableChange {
    ColumnAdded { name: String, definition: ColumnDefinition },
    ColumnRemoved { name: String },
    ColumnModified { name: String, old_type: ColumnType, new_type: ColumnType },
    ColumnRenamed { old_name: String, new_name: String },
}

/// Schema comparator for detecting differences
#[derive(Debug)]
pub struct SchemaComparator {
    pub strict_mode: bool,
    pub ignore_comments: bool,
}

/// Schema migrator for applying changes
#[derive(Debug)]
pub struct SchemaMigrator {
    pub dry_run: bool,
    pub backup_enabled: bool,
    pub rollback_enabled: bool,
}

impl DatabaseSchema {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            tables: HashMap::new(),
            version: "1.0.0".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        }
    }
    
    pub fn add_table(&mut self, table: TableSchema) {
        self.tables.insert(table.name.clone(), table);
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }
    
    pub fn remove_table(&mut self, table_name: &str) -> Option<TableSchema> {
        let result = self.tables.remove(table_name);
        if result.is_some() {
            self.updated_at = chrono::Utc::now().to_rfc3339();
        }
        result
    }
    
    pub fn get_table(&self, name: &str) -> Option<&TableSchema> {
        self.tables.get(name)
    }
    
    pub fn get_table_mut(&mut self, name: &str) -> Option<&mut TableSchema> {
        self.tables.get_mut(name)
    }
    
    pub fn table_exists(&self, name: &str) -> bool {
        self.tables.contains_key(name)
    }
}

impl TableSchema {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            columns: HashMap::new(),
            constraints: Vec::new(),
            indexes: Vec::new(),
        }
    }
    
    pub fn add_column(&mut self, column: ColumnDefinition) {
        self.columns.insert(column.name.clone(), column);
    }
    
    pub fn remove_column(&mut self, column_name: &str) -> Option<ColumnDefinition> {
        self.columns.remove(column_name)
    }
    
    pub fn add_constraint(&mut self, constraint: TableConstraint) {
        self.constraints.push(constraint);
    }
    
    pub fn add_index(&mut self, index: IndexDefinition) {
        self.indexes.push(index);
    }
    
    pub fn get_column(&self, name: &str) -> Option<&ColumnDefinition> {
        self.columns.get(name)
    }
    
    pub fn column_exists(&self, name: &str) -> bool {
        self.columns.contains_key(name)
    }
}

impl IndexDefinition {
    pub fn new(name: &str, columns: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            columns,
            unique: false,
            index_type: IndexType::BTree,
        }
    }
    
    pub fn unique(mut self) -> Self {
        self.unique = true;
        self
    }
    
    pub fn with_type(mut self, index_type: IndexType) -> Self {
        self.index_type = index_type;
        self
    }
}

impl SchemaComparator {
    pub fn new() -> Self {
        Self {
            strict_mode: false,
            ignore_comments: true,
        }
    }
    
    pub fn strict(mut self) -> Self {
        self.strict_mode = true;
        self
    }
    
    pub fn compare(&self, old_schema: &DatabaseSchema, new_schema: &DatabaseSchema) -> SchemaComparison {
        let mut differences = Vec::new();
        
        // Compare tables
        for (table_name, new_table) in &new_schema.tables {
            match old_schema.tables.get(table_name) {
                Some(old_table) => {
                    let table_changes = self.compare_tables(old_table, new_table);
                    if !table_changes.is_empty() {
                        differences.push(SchemaDifference::TableModified {
                            name: table_name.clone(),
                            changes: table_changes,
                        });
                    }
                }
                None => {
                    differences.push(SchemaDifference::TableAdded {
                        name: table_name.clone(),
                    });
                }
            }
        }
        
        // Check for removed tables
        for table_name in old_schema.tables.keys() {
            if !new_schema.tables.contains_key(table_name) {
                differences.push(SchemaDifference::TableRemoved {
                    name: table_name.clone(),
                });
            }
        }
        
        let is_compatible = self.is_backward_compatible(&differences);
        
        SchemaComparison {
            differences,
            is_compatible,
        }
    }
    
    fn compare_tables(&self, old_table: &TableSchema, new_table: &TableSchema) -> Vec<TableChange> {
        let mut changes = Vec::new();
        
        // Compare columns
        for (col_name, new_col) in &new_table.columns {
            match old_table.columns.get(col_name) {
                Some(old_col) => {
                    if !self.columns_equal(old_col, new_col) {
                        changes.push(TableChange::ColumnModified {
                            name: col_name.clone(),
                            old_type: old_col.column_type.clone(),
                            new_type: new_col.column_type.clone(),
                        });
                    }
                }
                None => {
                    changes.push(TableChange::ColumnAdded {
                        name: col_name.clone(),
                        definition: new_col.clone(),
                    });
                }
            }
        }
        
        // Check for removed columns
        for col_name in old_table.columns.keys() {
            if !new_table.columns.contains_key(col_name) {
                changes.push(TableChange::ColumnRemoved {
                    name: col_name.clone(),
                });
            }
        }
        
        changes
    }
    
    fn columns_equal(&self, old_col: &ColumnDefinition, new_col: &ColumnDefinition) -> bool {
        // Basic column comparison - can be extended
        old_col.name == new_col.name
            && std::mem::discriminant(&old_col.column_type) == std::mem::discriminant(&new_col.column_type)
            && old_col.nullable == new_col.nullable
    }
    
    fn is_backward_compatible(&self, differences: &[SchemaDifference]) -> bool {
        // Check if changes are backward compatible
        for diff in differences {
            match diff {
                SchemaDifference::TableRemoved { .. } => return false,
                SchemaDifference::TableModified { changes, .. } => {
                    for change in changes {
                        match change {
                            TableChange::ColumnRemoved { .. } => return false,
                            TableChange::ColumnModified { .. } => {
                                if self.strict_mode {
                                    return false;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        true
    }
}

impl SchemaMigrator {
    pub fn new() -> Self {
        Self {
            dry_run: false,
            backup_enabled: true,
            rollback_enabled: true,
        }
    }
    
    pub fn dry_run(mut self) -> Self {
        self.dry_run = true;
        self
    }
    
    pub fn apply_migration(&self, comparison: &SchemaComparison) -> Result<Vec<String>, CursedError> {
        let mut sql_statements = Vec::new();
        
        for difference in &comparison.differences {
            match difference {
                SchemaDifference::TableAdded { name } => {
                    sql_statements.push(format!("CREATE TABLE {} (...);", name));
                }
                SchemaDifference::TableRemoved { name } => {
                    sql_statements.push(format!("DROP TABLE {};", name));
                }
                SchemaDifference::TableModified { name, changes } => {
                    for change in changes {
                        match change {
                            TableChange::ColumnAdded { name: col_name, .. } => {
                                sql_statements.push(format!("ALTER TABLE {} ADD COLUMN {};", name, col_name));
                            }
                            TableChange::ColumnRemoved { name: col_name } => {
                                sql_statements.push(format!("ALTER TABLE {} DROP COLUMN {};", name, col_name));
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        
        if self.dry_run {
            println!("DRY RUN - Would execute:");
            for stmt in &sql_statements {
                println!("  {}", stmt);
            }
        }
        
        Ok(sql_statements)
    }
}

/// schema operations handler
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
        format!("Module: schema, Enabled: {}", self.enabled)
    }
}

impl Default for ModuleHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Initialize schema processing
pub fn init_schema() -> ModuleResult<()> {
    let handler = ModuleHandler::new();
    let result = handler.process("test")?;
    if !result.contains("test") {
        return Err(CursedError::runtime_error("Module test failed"));
    }
    println!("⚙️  Module processing (schema) initialized");
    Ok(())
}

/// Test schema functionality
pub fn test_schema() -> ModuleResult<()> {
    let handler = ModuleHandler::new();
    let result = handler.process("Hello, CURSED!")?;
    if !result.contains("Hello, CURSED!") {
        return Err(CursedError::runtime_error("Module test failed"));
    }
    Ok(())
}
