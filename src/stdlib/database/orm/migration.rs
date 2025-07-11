//! Database migration system for CURSED ORM

use std::collections::HashMap;
use std::sync::Arc;

use crate::error::CursedError;
use super::{SqlValue, DatabaseConnection, DatabaseError};

/// Result type for migration operations
pub type MigrationResult<T> = Result<T, DatabaseError>;

/// Migration version tracking
#[derive(Debug, Clone)]
pub struct MigrationVersion {
    pub version: String,
    pub applied_at: Option<String>,
    pub rollback_sql: Option<String>,
}

/// Database migration manager
pub struct MigrationManager {
    connection: Arc<dyn DatabaseConnection>,
    migrations_table: String,
    migrations: Vec<Box<dyn Migration>>,
}

/// Migration trait for all database migrations
pub trait Migration: Send + Sync {
    fn version(&self) -> &str;
    fn description(&self) -> &str;
    fn up(&self, connection: &dyn DatabaseConnection) -> MigrationResult<()>;
    fn down(&self, connection: &dyn DatabaseConnection) -> MigrationResult<()>;
}

/// Create table migration operation
#[derive(Debug, Clone)]
pub struct CreateTable {
    pub table_name: String,
    pub columns: Vec<ColumnDefinition>,
    pub constraints: Vec<TableConstraint>,
}

/// Drop table migration operation
#[derive(Debug, Clone)]
pub struct DropTable {
    pub table_name: String,
    pub if_exists: bool,
}

/// Add column migration operation
#[derive(Debug, Clone)]
pub struct AddColumn {
    pub table_name: String,
    pub column: ColumnDefinition,
}

/// Drop column migration operation
#[derive(Debug, Clone)]
pub struct DropColumn {
    pub table_name: String,
    pub column_name: String,
}

/// Add index migration operation
#[derive(Debug, Clone)]
pub struct AddIndex {
    pub table_name: String,
    pub index_name: String,
    pub columns: Vec<String>,
    pub unique: bool,
}

/// Drop index migration operation
#[derive(Debug, Clone)]
pub struct DropIndex {
    pub table_name: String,
    pub index_name: String,
}

/// Rename table migration operation
#[derive(Debug, Clone)]
pub struct RenameTable {
    pub old_name: String,
    pub new_name: String,
}

/// Rename column migration operation
#[derive(Debug, Clone)]
pub struct RenameColumn {
    pub table_name: String,
    pub old_name: String,
    pub new_name: String,
}

/// Modify column migration operation
#[derive(Debug, Clone)]
pub struct ModifyColumn {
    pub table_name: String,
    pub column_name: String,
    pub new_definition: ColumnDefinition,
}

/// Column definition for migrations
#[derive(Debug, Clone)]
pub struct ColumnDefinition {
    pub name: String,
    pub column_type: ColumnType,
    pub nullable: bool,
    pub default_value: Option<String>,
    pub auto_increment: bool,
    pub primary_key: bool,
    pub unique: bool,
}

/// Column types for database schemas
#[derive(Debug, Clone)]
pub enum ColumnType {
    Integer,
    BigInteger,
    SmallInteger,
    Float,
    Double,
    Decimal { precision: u8, scale: u8 },
    Boolean,
    Char { length: u16 },
    Varchar { length: u16 },
    Text,
    LongText,
    Binary { length: u16 },
    VarBinary { length: u16 },
    Blob,
    Date,
    Time,
    DateTime,
    Timestamp,
    Json,
    Uuid,
}

/// Table constraints for migrations
#[derive(Debug, Clone)]
pub enum TableConstraint {
    PrimaryKey { columns: Vec<String> },
    ForeignKey {
        columns: Vec<String>,
        references_table: String,
        references_columns: Vec<String>,
        on_delete: Option<String>,
        on_update: Option<String>,
    },
    Unique { columns: Vec<String> },
    Check { expression: String },
    Index { name: String, columns: Vec<String> },
}

impl MigrationManager {
    /// Create a new migration manager
    pub fn new(connection: Arc<dyn DatabaseConnection>) -> Self {
        Self {
            connection,
            migrations_table: "migrations".to_string(),
            migrations: Vec::new(),
        }
    }

    /// Set custom migrations table name
    pub fn with_migrations_table(mut self, table_name: &str) -> Self {
        self.migrations_table = table_name.to_string();
        self
    }

    /// Add a migration
    pub fn add_migration(&mut self, migration: Box<dyn Migration>) {
        self.migrations.push(migration);
    }

    /// Initialize migrations table
    pub fn initialize(&self) -> MigrationResult<()> {
        let create_table_sql = format!(
            "CREATE TABLE IF NOT EXISTS {} (
                version VARCHAR(255) PRIMARY KEY,
                applied_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                rollback_sql TEXT
            )",
            self.migrations_table
        );

        self.connection.exec(create_table_sql, vec![])
            .map_err(|e| DatabaseError::query(&format!("Failed to create migrations table: {}", e)))?;

        println!("🗃️ Migrations table initialized");
        Ok(())
    }

    /// Run all pending migrations
    pub fn migrate(&self) -> MigrationResult<()> {
        self.initialize()?;

        let applied_versions = self.get_applied_versions()?;
        let mut pending_migrations = Vec::new();

        for migration in &self.migrations {
            if !applied_versions.contains(&migration.version().to_string()) {
                pending_migrations.push(migration);
            }
        }

        if pending_migrations.is_empty() {
            println!("✅ No pending migrations");
            return Ok(());
        }

        println!("🔄 Running {} pending migrations", pending_migrations.len());

        for migration in pending_migrations {
            println!("🔄 Applying migration: {} - {}", migration.version(), migration.description());
            
            migration.up(self.connection.as_ref())?;
            
            let insert_sql = format!(
                "INSERT INTO {} (version, applied_at) VALUES (?, ?)",
                self.migrations_table
            );
            
            let now = chrono::Utc::now().to_rfc3339();
            self.connection.exec(insert_sql, vec![
                SqlValue::String(migration.version().to_string()),
                SqlValue::String(now),
            ])
            .map_err(|e| DatabaseError::query(&format!("Failed to record migration: {}", e)))?;

            println!("✅ Applied migration: {}", migration.version());
        }

        Ok(())
    }

    /// Rollback the last migration
    pub fn rollback(&self) -> MigrationResult<()> {
        let last_version = self.get_last_applied_version()?;
        
        if let Some(version) = last_version {
            self.rollback_to(&version)?;
        } else {
            println!("⚠️ No migrations to rollback");
        }

        Ok(())
    }

    /// Rollback to a specific migration version
    pub fn rollback_to(&self, target_version: &str) -> MigrationResult<()> {
        let applied_versions = self.get_applied_versions()?;
        let mut to_rollback = Vec::new();

        // Find migrations to rollback (in reverse order)
        for migration in self.migrations.iter().rev() {
            if applied_versions.contains(&migration.version().to_string()) {
                to_rollback.push(migration);
                if migration.version() == target_version {
                    break;
                }
            }
        }

        if to_rollback.is_empty() {
            println!("⚠️ No migrations to rollback to version {}", target_version);
            return Ok(());
        }

        println!("🔄 Rolling back {} migrations", to_rollback.len());

        for migration in to_rollback {
            println!("🔄 Rolling back migration: {} - {}", migration.version(), migration.description());
            
            migration.down(self.connection.as_ref())?;
            
            let delete_sql = format!(
                "DELETE FROM {} WHERE version = ?",
                self.migrations_table
            );
            
            self.connection.exec(delete_sql, vec![
                SqlValue::String(migration.version().to_string()),
            ])
            .map_err(|e| DatabaseError::query(&format!("Failed to remove migration record: {}", e)))?;

            println!("✅ Rolled back migration: {}", migration.version());
        }

        Ok(())
    }

    /// Get applied migration versions
    fn get_applied_versions(&self) -> MigrationResult<Vec<String>> {
        let sql = format!("SELECT version FROM {} ORDER BY applied_at", self.migrations_table);
        
        match self.connection.query(sql, vec![]) {
            Ok(result) => {
                let mut versions = Vec::new();
                for row in result.rows() {
                    if let Some(version_val) = row.get("version") {
                        if let Some(version) = version_val.as_str() {
                            versions.push(version.to_string());
                        }
                    }
                }
                Ok(versions)
            }
            Err(e) => Err(DatabaseError::query(&format!("Failed to get applied versions: {}", e))),
        }
    }

    /// Get the last applied migration version
    fn get_last_applied_version(&self) -> MigrationResult<Option<String>> {
        let sql = format!("SELECT version FROM {} ORDER BY applied_at DESC LIMIT 1", self.migrations_table);
        
        match self.connection.query(sql, vec![]) {
            Ok(result) => {
                if let Some(row) = result.rows().first() {
                    if let Some(version_val) = row.get("version") {
                        if let Some(version) = version_val.as_str() {
                            return Ok(Some(version.to_string()));
                        }
                    }
                }
                Ok(None)
            }
            Err(e) => Err(DatabaseError::query(&format!("Failed to get last applied version: {}", e))),
        }
    }

    /// Get migration status
    pub fn status(&self) -> MigrationResult<Vec<MigrationVersion>> {
        let applied_versions = self.get_applied_versions()?;
        let mut status = Vec::new();

        for migration in &self.migrations {
            let version = migration.version().to_string();
            let applied_at = if applied_versions.contains(&version) {
                Some("Applied".to_string())
            } else {
                None
            };

            status.push(MigrationVersion {
                version,
                applied_at,
                rollback_sql: None,
            });
        }

        Ok(status)
    }
}

impl CreateTable {
    pub fn new(table_name: &str) -> Self {
        Self {
            table_name: table_name.to_string(),
            columns: Vec::new(),
            constraints: Vec::new(),
        }
    }
    
    pub fn add_column(mut self, column: ColumnDefinition) -> Self {
        self.columns.push(column);
        self
    }
    
    pub fn add_constraint(mut self, constraint: TableConstraint) -> Self {
        self.constraints.push(constraint);
        self
    }

    pub fn to_sql(&self) -> String {
        let mut sql = format!("CREATE TABLE {} (", self.table_name);
        
        let mut parts = Vec::new();
        
        // Add columns
        for column in &self.columns {
            parts.push(column.to_sql());
        }
        
        // Add constraints
        for constraint in &self.constraints {
            parts.push(constraint.to_sql());
        }
        
        sql.push_str(&parts.join(", "));
        sql.push(')');
        
        sql
    }
}

impl DropTable {
    pub fn new(table_name: &str) -> Self {
        Self {
            table_name: table_name.to_string(),
            if_exists: false,
        }
    }
    
    pub fn if_exists(mut self) -> Self {
        self.if_exists = true;
        self
    }

    pub fn to_sql(&self) -> String {
        if self.if_exists {
            format!("DROP TABLE IF EXISTS {}", self.table_name)
        } else {
            format!("DROP TABLE {}", self.table_name)
        }
    }
}

impl AddColumn {
    pub fn new(table_name: &str, column: ColumnDefinition) -> Self {
        Self {
            table_name: table_name.to_string(),
            column,
        }
    }

    pub fn to_sql(&self) -> String {
        format!("ALTER TABLE {} ADD COLUMN {}", self.table_name, self.column.to_sql())
    }
}

impl DropColumn {
    pub fn new(table_name: &str, column_name: &str) -> Self {
        Self {
            table_name: table_name.to_string(),
            column_name: column_name.to_string(),
        }
    }

    pub fn to_sql(&self) -> String {
        format!("ALTER TABLE {} DROP COLUMN {}", self.table_name, self.column_name)
    }
}

impl AddIndex {
    pub fn new(table_name: &str, index_name: &str, columns: Vec<String>) -> Self {
        Self {
            table_name: table_name.to_string(),
            index_name: index_name.to_string(),
            columns,
            unique: false,
        }
    }
    
    pub fn unique(mut self) -> Self {
        self.unique = true;
        self
    }

    pub fn to_sql(&self) -> String {
        let index_type = if self.unique { "UNIQUE INDEX" } else { "INDEX" };
        format!(
            "CREATE {} {} ON {} ({})",
            index_type,
            self.index_name,
            self.table_name,
            self.columns.join(", ")
        )
    }
}

impl ColumnDefinition {
    pub fn new(name: &str, column_type: ColumnType) -> Self {
        Self {
            name: name.to_string(),
            column_type,
            nullable: true,
            default_value: None,
            auto_increment: false,
            primary_key: false,
            unique: false,
        }
    }
    
    pub fn not_null(mut self) -> Self {
        self.nullable = false;
        self
    }
    
    pub fn primary_key(mut self) -> Self {
        self.primary_key = true;
        self.nullable = false;
        self
    }
    
    pub fn unique(mut self) -> Self {
        self.unique = true;
        self
    }
    
    pub fn default_value(mut self, value: &str) -> Self {
        self.default_value = Some(value.to_string());
        self
    }
    
    pub fn auto_increment(mut self) -> Self {
        self.auto_increment = true;
        self
    }

    pub fn to_sql(&self) -> String {
        let mut sql = format!("{} {}", self.name, self.column_type.to_sql());
        
        if self.primary_key {
            sql.push_str(" PRIMARY KEY");
        }
        
        if self.unique && !self.primary_key {
            sql.push_str(" UNIQUE");
        }
        
        if !self.nullable && !self.primary_key {
            sql.push_str(" NOT NULL");
        }
        
        if self.auto_increment {
            sql.push_str(" AUTO_INCREMENT");
        }
        
        if let Some(ref default) = self.default_value {
            sql.push_str(&format!(" DEFAULT {}", default));
        }
        
        sql
    }
}

impl ColumnType {
    pub fn to_sql(&self) -> String {
        match self {
            ColumnType::Integer => "INTEGER".to_string(),
            ColumnType::BigInteger => "BIGINT".to_string(),
            ColumnType::SmallInteger => "SMALLINT".to_string(),
            ColumnType::Float => "FLOAT".to_string(),
            ColumnType::Double => "DOUBLE".to_string(),
            ColumnType::Decimal { precision, scale } => format!("DECIMAL({}, {})", precision, scale),
            ColumnType::Boolean => "BOOLEAN".to_string(),
            ColumnType::Char { length } => format!("CHAR({})", length),
            ColumnType::Varchar { length } => format!("VARCHAR({})", length),
            ColumnType::Text => "TEXT".to_string(),
            ColumnType::LongText => "LONGTEXT".to_string(),
            ColumnType::Binary { length } => format!("BINARY({})", length),
            ColumnType::VarBinary { length } => format!("VARBINARY({})", length),
            ColumnType::Blob => "BLOB".to_string(),
            ColumnType::Date => "DATE".to_string(),
            ColumnType::Time => "TIME".to_string(),
            ColumnType::DateTime => "DATETIME".to_string(),
            ColumnType::Timestamp => "TIMESTAMP".to_string(),
            ColumnType::Json => "JSON".to_string(),
            ColumnType::Uuid => "UUID".to_string(),
        }
    }
}

impl TableConstraint {
    pub fn to_sql(&self) -> String {
        match self {
            TableConstraint::PrimaryKey { columns } => {
                format!("PRIMARY KEY ({})", columns.join(", "))
            }
            TableConstraint::ForeignKey { 
                columns, 
                references_table, 
                references_columns, 
                on_delete, 
                on_update 
            } => {
                let mut sql = format!(
                    "FOREIGN KEY ({}) REFERENCES {} ({})",
                    columns.join(", "),
                    references_table,
                    references_columns.join(", ")
                );
                
                if let Some(on_delete) = on_delete {
                    sql.push_str(&format!(" ON DELETE {}", on_delete));
                }
                
                if let Some(on_update) = on_update {
                    sql.push_str(&format!(" ON UPDATE {}", on_update));
                }
                
                sql
            }
            TableConstraint::Unique { columns } => {
                format!("UNIQUE ({})", columns.join(", "))
            }
            TableConstraint::Check { expression } => {
                format!("CHECK ({})", expression)
            }
            TableConstraint::Index { name, columns } => {
                format!("INDEX {} ({})", name, columns.join(", "))
            }
        }
    }
}

/// Initialize migration system
pub fn init_migration() -> Result<(), CursedError> {
    println!("📁 Migration system initialized");
    Ok(())
}

/// Test migration functionality
pub fn test_migration() -> Result<(), CursedError> {
    let create_table = CreateTable::new("users")
        .add_column(ColumnDefinition::new("id", ColumnType::Integer).primary_key().auto_increment())
        .add_column(ColumnDefinition::new("name", ColumnType::Varchar { length: 255 }).not_null())
        .add_column(ColumnDefinition::new("email", ColumnType::Varchar { length: 255 }).unique())
        .add_constraint(TableConstraint::Index { 
            name: "idx_users_email".to_string(), 
            columns: vec!["email".to_string()] 
        });

    let drop_table = DropTable::new("users").if_exists();
    let add_column = AddColumn::new("users", ColumnDefinition::new("created_at", ColumnType::Timestamp));
    let add_index = AddIndex::new("users", "idx_users_name", vec!["name".to_string()]);

    println!("✅ Migration operations created successfully");
    Ok(())
}
