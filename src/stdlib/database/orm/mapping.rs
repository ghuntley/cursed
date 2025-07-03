//! Functional implementation for mapping

use crate::error::CursedError;
use std::collections::HashMap;
use super::migration::ColumnType;
use crate::stdlib::packages::ModuleError;

/// Result type for mapping operations
pub type ModuleResult<T> = Result<T, CursedError>;

/// SQL type mapping for different database systems
#[derive(Debug, Clone)]
pub struct SqlTypeMapping {
    pub cursed_type: String,
    pub sql_types: HashMap<DatabaseSystem, String>,
    pub default_sql_type: String,
}

/// Supported database systems
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DatabaseSystem {
    PostgreSQL,
    MySQL,
    SQLite,
    MariaDB,
    SqlServer,
    Oracle,
}

/// Custom type mapping function
pub type MappingFunction = fn(&str) -> Result<String, CursedError>;

/// Custom mapping for specific types
#[derive(Debug)]
pub struct CustomMapping {
    pub cursed_type: String,
    pub target_system: DatabaseSystem,
    pub mapper: MappingFunction,
    pub reverse_mapper: Option<MappingFunction>,
}

/// Registry for managing type mappings
#[derive(Debug)]
pub struct MappingRegistry {
    pub sql_mappings: HashMap<String, SqlTypeMapping>,
    pub custom_mappings: HashMap<String, CustomMapping>,
    pub default_system: DatabaseSystem,
}

impl SqlTypeMapping {
    pub fn new(cursed_type: &str, default_sql_type: &str) -> Self {
        Self {
            cursed_type: cursed_type.to_string(),
            sql_types: HashMap::new(),
            default_sql_type: default_sql_type.to_string(),
        }
    }
    
    pub fn add_mapping(mut self, system: DatabaseSystem, sql_type: &str) -> Self {
        self.sql_types.insert(system, sql_type.to_string());
        self
    }
    
    pub fn get_sql_type(&self, system: &DatabaseSystem) -> &str {
        self.sql_types
            .get(system)
            .map(|s| s.as_str())
            .unwrap_or(&self.default_sql_type)
    }
}

impl CustomMapping {
    pub fn new(cursed_type: &str, target_system: DatabaseSystem, mapper: MappingFunction) -> Self {
        Self {
            cursed_type: cursed_type.to_string(),
            target_system,
            mapper,
            reverse_mapper: None,
        }
    }
    
    pub fn with_reverse_mapper(mut self, reverse_mapper: MappingFunction) -> Self {
        self.reverse_mapper = Some(reverse_mapper);
        self
    }
    
    pub fn map_to_sql(&self, value: &str) -> Result<String, CursedError> {
        (self.mapper)(value)
    }
    
    pub fn map_from_sql(&self, value: &str) -> Result<String, CursedError> {
        match &self.reverse_mapper {
            Some(mapper) => mapper(value),
            None => Err(CursedError::runtime_error(&"Reverse mapping not available".to_string())),
        }
    }
}

impl MappingRegistry {
    pub fn new(default_system: DatabaseSystem) -> Self {
        let mut registry = Self {
            sql_mappings: HashMap::new(),
            custom_mappings: HashMap::new(),
            default_system,
        };
        
        registry.register_default_mappings();
        registry
    }
    
    pub fn register_sql_mapping(&mut self, mapping: SqlTypeMapping) {
        self.sql_mappings.insert(mapping.cursed_type.clone(), mapping);
    }
    
    pub fn register_custom_mapping(&mut self, mapping: CustomMapping) {
        let key = format!("{}:{:?}", mapping.cursed_type, mapping.target_system);
        self.custom_mappings.insert(key, mapping);
    }
    
    pub fn map_type(&self, cursed_type: &str, target_system: &DatabaseSystem) -> Result<String, CursedError> {
        // First check for custom mappings
        let custom_key = format!("{}:{:?}", cursed_type, target_system);
        if let Some(custom_mapping) = self.custom_mappings.get(&custom_key) {
            return custom_mapping.map_to_sql(cursed_type);
        }
        
        // Then check for standard SQL mappings
        if let Some(sql_mapping) = self.sql_mappings.get(cursed_type) {
            return Ok(sql_mapping.get_sql_type(target_system).to_string());
        }
        
        Err(CursedError::runtime_error(&format!("No mapping found for type: {}", "placeholder")))
    }
    
    pub fn map_column_type(&self, column_type: &ColumnType, target_system: &DatabaseSystem) -> Result<String, CursedError> {
        let type_str = match column_type {
            ColumnType::Integer => "integer",
            ColumnType::BigInteger => "bigint",
            ColumnType::SmallInteger => "smallint",
            ColumnType::Float => "float",
            ColumnType::Double => "double",
            ColumnType::Decimal { precision, scale } => {
                return Ok(format!("decimal({}, {})", precision, scale));
            }
            ColumnType::Boolean => "boolean",
            ColumnType::Char { length } => {
                return Ok(format!("char({})", length));
            }
            ColumnType::Varchar { length } => {
                return Ok(format!("varchar({})", length));
            }
            ColumnType::Text => "text",
            ColumnType::LongText => "longtext",
            ColumnType::Binary { length } => {
                return Ok(format!("binary({})", length));
            }
            ColumnType::VarBinary { length } => {
                return Ok(format!("varbinary({})", length));
            }
            ColumnType::Blob => "blob",
            ColumnType::Date => "date",
            ColumnType::Time => "time",
            ColumnType::DateTime => "datetime",
            ColumnType::Timestamp => "timestamp",
            ColumnType::Json => "json",
            ColumnType::Uuid => "uuid",
        };
        
        self.map_type(type_str, target_system)
    }
    
    fn register_default_mappings(&mut self) {
        // Integer types
        self.register_sql_mapping(
            SqlTypeMapping::new("integer", "INTEGER")
                .add_mapping(DatabaseSystem::PostgreSQL, "INTEGER")
                .add_mapping(DatabaseSystem::MySQL, "INT")
                .add_mapping(DatabaseSystem::SQLite, "INTEGER")
        );
        
        self.register_sql_mapping(
            SqlTypeMapping::new("bigint", "BIGINT")
                .add_mapping(DatabaseSystem::PostgreSQL, "BIGINT")
                .add_mapping(DatabaseSystem::MySQL, "BIGINT")
                .add_mapping(DatabaseSystem::SQLite, "INTEGER")
        );
        
        self.register_sql_mapping(
            SqlTypeMapping::new("smallint", "SMALLINT")
                .add_mapping(DatabaseSystem::PostgreSQL, "SMALLINT")
                .add_mapping(DatabaseSystem::MySQL, "SMALLINT")
                .add_mapping(DatabaseSystem::SQLite, "INTEGER")
        );
        
        // Floating point types
        self.register_sql_mapping(
            SqlTypeMapping::new("float", "FLOAT")
                .add_mapping(DatabaseSystem::PostgreSQL, "REAL")
                .add_mapping(DatabaseSystem::MySQL, "FLOAT")
                .add_mapping(DatabaseSystem::SQLite, "REAL")
        );
        
        self.register_sql_mapping(
            SqlTypeMapping::new("double", "DOUBLE")
                .add_mapping(DatabaseSystem::PostgreSQL, "DOUBLE PRECISION")
                .add_mapping(DatabaseSystem::MySQL, "DOUBLE")
                .add_mapping(DatabaseSystem::SQLite, "REAL")
        );
        
        // Boolean type
        self.register_sql_mapping(
            SqlTypeMapping::new("boolean", "BOOLEAN")
                .add_mapping(DatabaseSystem::PostgreSQL, "BOOLEAN")
                .add_mapping(DatabaseSystem::MySQL, "BOOLEAN")
                .add_mapping(DatabaseSystem::SQLite, "INTEGER")
        );
        
        // Text types
        self.register_sql_mapping(
            SqlTypeMapping::new("text", "TEXT")
                .add_mapping(DatabaseSystem::PostgreSQL, "TEXT")
                .add_mapping(DatabaseSystem::MySQL, "TEXT")
                .add_mapping(DatabaseSystem::SQLite, "TEXT")
        );
        
        self.register_sql_mapping(
            SqlTypeMapping::new("longtext", "TEXT")
                .add_mapping(DatabaseSystem::PostgreSQL, "TEXT")
                .add_mapping(DatabaseSystem::MySQL, "LONGTEXT")
                .add_mapping(DatabaseSystem::SQLite, "TEXT")
        );
        
        // Date/Time types
        self.register_sql_mapping(
            SqlTypeMapping::new("date", "DATE")
                .add_mapping(DatabaseSystem::PostgreSQL, "DATE")
                .add_mapping(DatabaseSystem::MySQL, "DATE")
                .add_mapping(DatabaseSystem::SQLite, "TEXT")
        );
        
        self.register_sql_mapping(
            SqlTypeMapping::new("datetime", "DATETIME")
                .add_mapping(DatabaseSystem::PostgreSQL, "TIMESTAMP")
                .add_mapping(DatabaseSystem::MySQL, "DATETIME")
                .add_mapping(DatabaseSystem::SQLite, "TEXT")
        );
        
        self.register_sql_mapping(
            SqlTypeMapping::new("timestamp", "TIMESTAMP")
                .add_mapping(DatabaseSystem::PostgreSQL, "TIMESTAMP")
                .add_mapping(DatabaseSystem::MySQL, "TIMESTAMP")
                .add_mapping(DatabaseSystem::SQLite, "TEXT")
        );
        
        // JSON type
        self.register_sql_mapping(
            SqlTypeMapping::new("json", "TEXT")
                .add_mapping(DatabaseSystem::PostgreSQL, "JSONB")
                .add_mapping(DatabaseSystem::MySQL, "JSON")
                .add_mapping(DatabaseSystem::SQLite, "TEXT")
        );
        
        // UUID type
        self.register_sql_mapping(
            SqlTypeMapping::new("uuid", "TEXT")
                .add_mapping(DatabaseSystem::PostgreSQL, "UUID")
                .add_mapping(DatabaseSystem::MySQL, "CHAR(36)")
                .add_mapping(DatabaseSystem::SQLite, "TEXT")
        );
        
        // Blob type
        self.register_sql_mapping(
            SqlTypeMapping::new("blob", "BLOB")
                .add_mapping(DatabaseSystem::PostgreSQL, "BYTEA")
                .add_mapping(DatabaseSystem::MySQL, "BLOB")
                .add_mapping(DatabaseSystem::SQLite, "BLOB")
        );
    }
    
    pub fn get_supported_systems(&self) -> Vec<DatabaseSystem> {
        vec![
            DatabaseSystem::PostgreSQL,
            DatabaseSystem::MySQL,
            DatabaseSystem::SQLite,
            DatabaseSystem::MariaDB,
            DatabaseSystem::SqlServer,
            DatabaseSystem::Oracle,
        ]
    }
    
    pub fn validate_mapping(&self, cursed_type: &str) -> bool {
        self.sql_mappings.contains_key(cursed_type) ||
        self.custom_mappings.iter().any(|(_, mapping)| mapping.cursed_type == cursed_type)
    }
}

/// mapping operations handler
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
        format!("Module: mapping, Enabled: {}", self.enabled)
    }
}

impl Default for ModuleHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Initialize mapping processing
pub fn init_mapping() -> ModuleResult<()> {
    let handler = ModuleHandler::new();
    let result = handler.process("test")?;
    if !result.contains("test") {
        return Err(CursedError::runtime_error(&"Module test failed".to_string()));
    }
    println!("⚙️  Module processing (mapping) initialized");
    Ok(())
}

/// Test mapping functionality
pub fn test_mapping() -> ModuleResult<()> {
    let handler = ModuleHandler::new();
    let result = handler.process("Hello, CURSED!")?;
    if !result.contains("Hello, CURSED!") {
        return Err(CursedError::runtime_error(&"Module test failed".to_string()));
    }
    Ok(())
}
