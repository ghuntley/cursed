/// fr fr SQLite PRAGMA management that slays periodt
/// 
/// This module provides a comprehensive interface for managing SQLite
/// PRAGMA statements, which control database behavior and configuration.

use std::collections::HashMap;
use super::{SqliteError, SqliteResult};

/// fr fr SQLite PRAGMA values
#[derive(Debug, Clone, PartialEq)]
pub enum PragmaValue {
    Integer(i64),
    Real(f64),
    Text(String),
    Boolean(bool),
}

impl PragmaValue {
    /// slay Convert to SQL string
    pub fn to_sql(&self) -> String {
        match self {
            PragmaValue::Integer(i) => i.to_string(),
            PragmaValue::Real(f) => f.to_string(),
            PragmaValue::Text(s) => format!("'{}'", s.replace('\'', "''")),
            PragmaValue::Boolean(b) => if *b { "ON" } else { "OFF" }.to_string(),
        }
    }

    /// slay Parse from string
    pub fn from_str(s: &str) -> Self {
        if let Ok(i) = s.parse::<i64>() {
            PragmaValue::Integer(i)
        } else if let Ok(f) = s.parse::<f64>() {
            PragmaValue::Real(f)
        } else {
            match s.to_uppercase().as_str() {
                "ON" | "TRUE" | "YES" | "1" => PragmaValue::Boolean(true),
                "OFF" | "FALSE" | "NO" | "0" => PragmaValue::Boolean(false),
                _ => PragmaValue::Text(s.to_string()),
            }
        }
    }
}

/// fr fr Individual SQLite PRAGMA
#[derive(Debug, Clone)]
pub struct SqlitePragma {
    pub name: String,
    pub value: Option<PragmaValue>,
    pub schema: Option<String>,
    pub description: String,
    pub readonly: bool,
}

impl SqlitePragma {
    /// slay Create new PRAGMA
    pub fn new(name: &str, value: Option<PragmaValue>) -> Self {
        Self {
            name: name.to_string(),
            value,
            schema: None,
            description: String::new(),
            readonly: false,
        }
    }

    /// slay Create read-only PRAGMA
    pub fn readonly(name: &str) -> Self {
        Self {
            name: name.to_string(),
            value: None,
            schema: None,
            description: String::new(),
            readonly: true,
        }
    }

    /// slay Set schema
    pub fn with_schema(mut self, schema: &str) -> Self {
        self.schema = Some(schema.to_string());
        self
    }

    /// slay Set description
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    /// slay Generate SQL statement
    pub fn to_sql(&self) -> String {
        let mut sql = String::from("PRAGMA ");
        
        if let Some(ref schema) = self.schema {
            sql.push_str(schema);
            sql.push('.');
        }
        
        sql.push_str(&self.name);
        
        if let Some(ref value) = self.value {
            sql.push_str(" = ");
            sql.push_str(&value.to_sql());
        }
        
        sql
    }
}

/// fr fr SQLite PRAGMA manager
#[derive(Debug)]
pub struct SqlitePragmaManager {
    pragmas: HashMap<String, SqlitePragma>,
}

impl SqlitePragmaManager {
    /// slay Create new PRAGMA manager
    pub fn new() -> Self {
        let mut manager = Self {
            pragmas: HashMap::new(),
        };
        
        // Register built-in PRAGMAs
        manager.register_builtin_pragmas();
        manager
    }

    /// slay Register built-in PRAGMAs
    fn register_builtin_pragmas(&mut self) {
        // Configuration PRAGMAs
        self.register(SqlitePragma::new("page_size", None)
            .with_description("Database page size in bytes"));
        self.register(SqlitePragma::new("cache_size", None)
            .with_description("Maximum number of pages in cache"));
        self.register(SqlitePragma::new("temp_store", None)
            .with_description("Storage location for temporary files"));
        self.register(SqlitePragma::new("journal_mode", None)
            .with_description("Journal mode for transactions"));
        self.register(SqlitePragma::new("synchronous", None)
            .with_description("Synchronization mode"));
        self.register(SqlitePragma::new("locking_mode", None)
            .with_description("Database locking mode"));
        self.register(SqlitePragma::new("auto_vacuum", None)
            .with_description("Automatic vacuum mode"));
        self.register(SqlitePragma::new("incremental_vacuum", None)
            .with_description("Incremental vacuum pages"));
        self.register(SqlitePragma::new("wal_autocheckpoint", None)
            .with_description("WAL auto-checkpoint threshold"));
        self.register(SqlitePragma::new("wal_checkpoint", None)
            .with_description("WAL checkpoint operation"));

        // Memory and I/O PRAGMAs
        self.register(SqlitePragma::new("mmap_size", None)
            .with_description("Memory-mapped I/O size"));
        self.register(SqlitePragma::new("cache_spill", None)
            .with_description("Cache spill threshold"));
        self.register(SqlitePragma::new("temp_store_directory", None)
            .with_description("Temporary file directory"));

        // Foreign key and constraint PRAGMAs
        self.register(SqlitePragma::new("foreign_keys", None)
            .with_description("Foreign key constraint enforcement"));
        self.register(SqlitePragma::new("defer_foreign_keys", None)
            .with_description("Defer foreign key checks"));
        self.register(SqlitePragma::new("recursive_triggers", None)
            .with_description("Recursive trigger support"));
        self.register(SqlitePragma::new("ignore_check_constraints", None)
            .with_description("Ignore CHECK constraints"));

        // Security PRAGMAs  
        self.register(SqlitePragma::new("secure_delete", None)
            .with_description("Secure deletion of data"));
        self.register(SqlitePragma::new("trusted_schema", None)
            .with_description("Trust schema definitions"));

        // Information PRAGMAs (read-only)
        self.register(SqlitePragma::readonly("schema_version")
            .with_description("Schema version number"));
        self.register(SqlitePragma::readonly("user_version")
            .with_description("User-defined version"));
        self.register(SqlitePragma::readonly("application_id")
            .with_description("Application ID"));
        self.register(SqlitePragma::readonly("freelist_count")
            .with_description("Number of free pages"));
        self.register(SqlitePragma::readonly("page_count")
            .with_description("Total number of pages"));
        self.register(SqlitePragma::readonly("encoding")
            .with_description("Text encoding"));
        self.register(SqlitePragma::readonly("integrity_check")
            .with_description("Database integrity check"));
        self.register(SqlitePragma::readonly("quick_check")
            .with_description("Quick integrity check"));

        // Analysis PRAGMAs
        self.register(SqlitePragma::new("analysis_limit", None)
            .with_description("ANALYZE command limit"));
        self.register(SqlitePragma::new("optimize", None)
            .with_description("Database optimization"));

        // Performance PRAGMAs
        self.register(SqlitePragma::new("query_only", None)
            .with_description("Read-only query mode"));
        self.register(SqlitePragma::new("read_uncommitted", None)
            .with_description("Read uncommitted data"));
        self.register(SqlitePragma::new("busy_timeout", None)
            .with_description("Busy handler timeout"));

        // Table and index PRAGMAs
        self.register(SqlitePragma::readonly("table_info")
            .with_description("Table column information"));
        self.register(SqlitePragma::readonly("index_list")
            .with_description("Table index list"));
        self.register(SqlitePragma::readonly("index_info")
            .with_description("Index column information"));
        self.register(SqlitePragma::readonly("foreign_key_list")
            .with_description("Foreign key definitions"));
    }

    /// slay Register PRAGMA
    pub fn register(&mut self, pragma: SqlitePragma) {
        self.pragmas.insert(pragma.name.clone(), pragma);
    }

    /// slay Get PRAGMA by name
    pub fn get(&self, name: &str) -> Option<&SqlitePragma> {
        self.pragmas.get(name)
    }

    /// slay Check if PRAGMA exists
    pub fn exists(&self, name: &str) -> bool {
        self.pragmas.contains_key(name)
    }

    /// slay List all PRAGMA names
    pub fn list_names(&self) -> Vec<String> {
        self.pragmas.keys().cloned().collect()
    }

    /// slay Get configuration PRAGMAs
    pub fn configuration_pragmas(&self) -> Vec<&SqlitePragma> {
        self.pragmas.values()
            .filter(|p| !p.readonly)
            .collect()
    }

    /// slay Get information PRAGMAs (read-only)
    pub fn information_pragmas(&self) -> Vec<&SqlitePragma> {
        self.pragmas.values()
            .filter(|p| p.readonly)
            .collect()
    }

    /// slay Create PRAGMA statement
    pub fn create_statement(&self, name: &str, value: Option<PragmaValue>) -> SqliteResult<String> {
        if let Some(pragma) = self.get(name) {
            if pragma.readonly && value.is_some() {
                return Err(SqliteError::invalid_parameter(
                    &format!("PRAGMA {} is read-only", name)
                ));
            }

            let mut pragma_with_value = pragma.clone();
            pragma_with_value.value = value;
            Ok(pragma_with_value.to_sql())
        } else {
            Err(SqliteError::invalid_parameter(
                &format!("Unknown PRAGMA: {}", name)
            ))
        }
    }

    /// slay Create multiple PRAGMA statements
    pub fn create_statements(&self, pragmas: &[(String, Option<PragmaValue>)]) -> SqliteResult<Vec<String>> {
        let mut statements = Vec::new();
        
        for (name, value) in pragmas {
            let statement = self.create_statement(name, value.clone())?;
            statements.push(statement);
        }
        
        Ok(statements)
    }

    /// slay Validate PRAGMA combination
    pub fn validate_combination(&self, pragmas: &[(String, Option<PragmaValue>)]) -> SqliteResult<()> {
        // Check for conflicting PRAGMAs
        let mut journal_mode = None;
        let mut wal_related = false;
        
        for (name, value) in pragmas {
            match name.as_str() {
                "journal_mode" => {
                    if let Some(PragmaValue::Text(mode)) = value {
                        journal_mode = Some(mode.to_uppercase());
                    }
                }
                "wal_autocheckpoint" | "wal_checkpoint" => {
                    wal_related = true;
                }
                _ => {}
            }
        }
        
        // Check for WAL-related PRAGMAs without WAL mode
        if wal_related {
            if let Some(ref mode) = journal_mode {
                if mode != "WAL" {
                    return Err(SqliteError::invalid_parameter(
                        "WAL-related PRAGMAs require journal_mode=WAL"
                    ));
                }
            }
        }
        
        Ok(())
    }

    /// slay Get recommended PRAGMAs for performance
    pub fn performance_pragmas() -> Vec<(String, PragmaValue)> {
        vec![
            ("synchronous".to_string(), PragmaValue::Text("NORMAL".to_string())),
            ("cache_size".to_string(), PragmaValue::Integer(-64000)), // 64MB
            ("temp_store".to_string(), PragmaValue::Text("MEMORY".to_string())),
            ("mmap_size".to_string(), PragmaValue::Integer(268435456)), // 256MB
        ]
    }

    /// slay Get recommended PRAGMAs for safety
    pub fn safety_pragmas() -> Vec<(String, PragmaValue)> {
        vec![
            ("synchronous".to_string(), PragmaValue::Text("FULL".to_string())),
            ("foreign_keys".to_string(), PragmaValue::Boolean(true)),
            ("secure_delete".to_string(), PragmaValue::Boolean(true)),
            ("trusted_schema".to_string(), PragmaValue::Boolean(false)),
        ]
    }

    /// slay Get recommended PRAGMAs for WAL mode
    pub fn wal_mode_pragmas() -> Vec<(String, PragmaValue)> {
        vec![
            ("journal_mode".to_string(), PragmaValue::Text("WAL".to_string())),
            ("synchronous".to_string(), PragmaValue::Text("NORMAL".to_string())),
            ("wal_autocheckpoint".to_string(), PragmaValue::Integer(1000)),
            ("cache_size".to_string(), PragmaValue::Integer(-64000)),
        ]
    }
}

impl Default for SqlitePragmaManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pragma_value() {
        let int_val = PragmaValue::Integer(42);
        assert_eq!(int_val.to_sql(), "42");

        let bool_val = PragmaValue::Boolean(true);
        assert_eq!(bool_val.to_sql(), "ON");

        let text_val = PragmaValue::Text("test'value".to_string());
        assert_eq!(text_val.to_sql(), "'test''value'");

        assert_eq!(PragmaValue::from_str("123"), PragmaValue::Integer(123));
        assert_eq!(PragmaValue::from_str("3.14"), PragmaValue::Real(3.14));
        assert_eq!(PragmaValue::from_str("ON"), PragmaValue::Boolean(true));
        assert_eq!(PragmaValue::from_str("test"), PragmaValue::Text("test".to_string()));
    }

    #[test]
    fn test_pragma_creation() {
        let pragma = SqlitePragma::new("page_size", Some(PragmaValue::Integer(4096)))
            .with_description("Database page size");
        
        assert_eq!(pragma.name, "page_size");
        assert_eq!(pragma.value, Some(PragmaValue::Integer(4096)));
        assert!(!pragma.readonly);
        assert_eq!(pragma.to_sql(), "PRAGMA page_size = 4096");

        let readonly_pragma = SqlitePragma::readonly("schema_version");
        assert!(readonly_pragma.readonly);
        assert_eq!(readonly_pragma.to_sql(), "PRAGMA schema_version");
    }

    #[test]
    fn test_pragma_with_schema() {
        let pragma = SqlitePragma::new("table_info", Some(PragmaValue::Text("users".to_string())))
            .with_schema("main");
        
        assert_eq!(pragma.to_sql(), "PRAGMA main.table_info = 'users'");
    }

    #[test]
    fn test_pragma_manager() {
        let manager = SqlitePragmaManager::new();
        
        assert!(manager.exists("page_size"));
        assert!(manager.exists("foreign_keys"));
        assert!(!manager.exists("nonexistent"));

        let pragma = manager.get("page_size").unwrap();
        assert_eq!(pragma.name, "page_size");

        let config_pragmas = manager.configuration_pragmas();
        assert!(!config_pragmas.is_empty());

        let info_pragmas = manager.information_pragmas();
        assert!(!info_pragmas.is_empty());
    }

    #[test]
    fn test_statement_creation() {
        let manager = SqlitePragmaManager::new();
        
        let statement = manager.create_statement("page_size", Some(PragmaValue::Integer(4096))).unwrap();
        assert_eq!(statement, "PRAGMA page_size = 4096");

        let readonly_err = manager.create_statement("schema_version", Some(PragmaValue::Integer(1)));
        assert!(readonly_err.is_err());

        let unknown_err = manager.create_statement("unknown_pragma", None);
        assert!(unknown_err.is_err());
    }

    #[test]
    fn test_pragma_validation() {
        let manager = SqlitePragmaManager::new();
        
        let valid_pragmas = vec![
            ("journal_mode".to_string(), Some(PragmaValue::Text("WAL".to_string()))),
            ("wal_autocheckpoint".to_string(), Some(PragmaValue::Integer(1000))),
        ];
        assert!(manager.validate_combination(&valid_pragmas).is_ok());

        let invalid_pragmas = vec![
            ("journal_mode".to_string(), Some(PragmaValue::Text("DELETE".to_string()))),
            ("wal_autocheckpoint".to_string(), Some(PragmaValue::Integer(1000))),
        ];
        assert!(manager.validate_combination(&invalid_pragmas).is_err());
    }

    #[test]
    fn test_recommended_pragmas() {
        let performance = SqlitePragmaManager::performance_pragmas();
        assert!(!performance.is_empty());
        assert!(performance.iter().any(|(name, _)| name == "synchronous"));

        let safety = SqlitePragmaManager::safety_pragmas();
        assert!(!safety.is_empty());
        assert!(safety.iter().any(|(name, _)| name == "foreign_keys"));

        let wal = SqlitePragmaManager::wal_mode_pragmas();
        assert!(!wal.is_empty());
        assert!(wal.iter().any(|(name, _)| name == "journal_mode"));
    }
}
