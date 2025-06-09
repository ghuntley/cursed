/// fr fr SQL dialect implementations - speaking the right SQL language periodt
///
/// This module provides SQL dialect-specific implementations for different
/// database systems. Each dialect knows its own quirks bestie!

use std::collections::HashMap;

/// fr fr SQL dialect enumeration
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SqlDialect {
    /// ANSI SQL standard
    Ansi,
    /// PostgreSQL dialect
    PostgreSQL,
    /// MySQL dialect
    MySQL,
    /// SQLite dialect
    SQLite,
    /// SQL Server dialect
    SQLServer,
    /// Oracle dialect
    Oracle,
    /// H2 dialect
    H2,
    /// Custom dialect
    Custom(String),
}

/// fr fr SQL dialect trait
pub trait SqlDialectTrait: std::fmt::Debug + Send + Sync {
    /// slay Get dialect name
    fn name(&self) -> &str;
    
    /// slay Get parameter placeholder for index
    fn parameter_placeholder(&self, index: usize) -> String;
    
    /// slay Get named parameter placeholder
    fn named_parameter_placeholder(&self, name: &str) -> String;
    
    /// slay Quote identifier (table/column name)
    fn quote_identifier(&self, identifier: &str) -> String;
    
    /// slay Quote string literal
    fn quote_string(&self, value: &str) -> String;
    
    /// slay Get LIMIT clause syntax
    fn limit_clause(&self, limit: u64, offset: Option<u64>) -> String;
    
    /// slay Get supported features
    fn supported_features(&self) -> &DialectFeatures;
    
    /// slay Get SQL keywords
    fn keywords(&self) -> &SqlKeywords;
    
    /// slay Get SQL functions
    fn functions(&self) -> &SqlFunctions;
    
    /// slay Convert type to dialect-specific SQL
    fn type_to_sql(&self, sql_type: &crate::stdlib::packages::db_sql::SqlType) -> String;
}

/// fr fr PostgreSQL dialect implementation
#[derive(Debug)]
pub struct PostgreSqlDialect {
    features: DialectFeatures,
    keywords: SqlKeywords,
    functions: SqlFunctions,
}

/// fr fr MySQL dialect implementation
#[derive(Debug)]
pub struct MySqlDialect {
    features: DialectFeatures,
    keywords: SqlKeywords,
    functions: SqlFunctions,
}

/// fr fr SQLite dialect implementation
#[derive(Debug)]
pub struct SqliteDialect {
    features: DialectFeatures,
    keywords: SqlKeywords,
    functions: SqlFunctions,
}

/// fr fr Dialect features and capabilities
#[derive(Debug, Clone)]
pub struct DialectFeatures {
    /// Supports CTEs (Common Table Expressions)
    pub supports_cte: bool,
    /// Supports window functions
    pub supports_window_functions: bool,
    /// Supports JSON functions
    pub supports_json: bool,
    /// Supports recursive queries
    pub supports_recursive: bool,
    /// Supports UPSERT operations
    pub supports_upsert: bool,
    /// Supports arrays
    pub supports_arrays: bool,
    /// Supports stored procedures
    pub supports_stored_procedures: bool,
    /// Supports full-text search
    pub supports_fulltext: bool,
    /// Supports GIS/spatial data
    pub supports_spatial: bool,
    /// Maximum identifier length
    pub max_identifier_length: usize,
    /// Case sensitivity of identifiers
    pub case_sensitive_identifiers: bool,
}

/// fr fr SQL keywords for the dialect
#[derive(Debug, Clone)]
pub struct SqlKeywords {
    /// Reserved words
    pub reserved_words: Vec<String>,
    /// Function names
    pub function_names: Vec<String>,
    /// Type names
    pub type_names: Vec<String>,
}

/// fr fr SQL functions available in the dialect
#[derive(Debug, Clone)]
pub struct SqlFunctions {
    /// String functions
    pub string_functions: HashMap<String, String>,
    /// Date/time functions
    pub datetime_functions: HashMap<String, String>,
    /// Math functions
    pub math_functions: HashMap<String, String>,
    /// Aggregate functions
    pub aggregate_functions: HashMap<String, String>,
    /// JSON functions
    pub json_functions: HashMap<String, String>,
}

impl PostgreSqlDialect {
    /// slay Create new PostgreSQL dialect
    pub fn new() -> Self {
        let features = DialectFeatures {
            supports_cte: true,
            supports_window_functions: true,
            supports_json: true,
            supports_recursive: true,
            supports_upsert: true,
            supports_arrays: true,
            supports_stored_procedures: true,
            supports_fulltext: true,
            supports_spatial: true,
            max_identifier_length: 63,
            case_sensitive_identifiers: false,
        };

        let keywords = SqlKeywords {
            reserved_words: vec![
                "SELECT".to_string(), "FROM".to_string(), "WHERE".to_string(),
                "INSERT".to_string(), "UPDATE".to_string(), "DELETE".to_string(),
                "CREATE".to_string(), "ALTER".to_string(), "DROP".to_string(),
                "INDEX".to_string(), "TABLE".to_string(), "VIEW".to_string(),
            ],
            function_names: vec![
                "NOW".to_string(), "CURRENT_TIMESTAMP".to_string(),
                "LENGTH".to_string(), "SUBSTRING".to_string(),
            ],
            type_names: vec![
                "INTEGER".to_string(), "TEXT".to_string(), "BOOLEAN".to_string(),
                "TIMESTAMP".to_string(), "JSON".to_string(), "JSONB".to_string(),
            ],
        };

        let mut string_functions = HashMap::new();
        string_functions.insert("LENGTH".to_string(), "LENGTH".to_string());
        string_functions.insert("SUBSTRING".to_string(), "SUBSTRING".to_string());
        string_functions.insert("UPPER".to_string(), "UPPER".to_string());
        string_functions.insert("LOWER".to_string(), "LOWER".to_string());

        let mut datetime_functions = HashMap::new();
        datetime_functions.insert("NOW".to_string(), "NOW()".to_string());
        datetime_functions.insert("CURRENT_TIMESTAMP".to_string(), "CURRENT_TIMESTAMP".to_string());

        let mut math_functions = HashMap::new();
        math_functions.insert("ABS".to_string(), "ABS".to_string());
        math_functions.insert("ROUND".to_string(), "ROUND".to_string());

        let mut aggregate_functions = HashMap::new();
        aggregate_functions.insert("COUNT".to_string(), "COUNT".to_string());
        aggregate_functions.insert("SUM".to_string(), "SUM".to_string());
        aggregate_functions.insert("AVG".to_string(), "AVG".to_string());

        let mut json_functions = HashMap::new();
        json_functions.insert("JSON_EXTRACT".to_string(), "JSON_EXTRACT_PATH".to_string());
        json_functions.insert("JSON_OBJECT".to_string(), "JSON_BUILD_OBJECT".to_string());

        let functions = SqlFunctions {
            string_functions,
            datetime_functions,
            math_functions,
            aggregate_functions,
            json_functions,
        };

        Self { features, keywords, functions }
    }
}

impl SqlDialectTrait for PostgreSqlDialect {
    fn name(&self) -> &str {
        "PostgreSQL"
    }

    fn parameter_placeholder(&self, index: usize) -> String {
        format!("${}", index + 1) // PostgreSQL uses $1, $2, etc.
    }

    fn named_parameter_placeholder(&self, name: &str) -> String {
        format!("${}", name) // PostgreSQL also supports named parameters
    }

    fn quote_identifier(&self, identifier: &str) -> String {
        format!("\"{}\"", identifier.replace('"', "\"\""))
    }

    fn quote_string(&self, value: &str) -> String {
        format!("'{}'", value.replace('\'', "''"))
    }

    fn limit_clause(&self, limit: u64, offset: Option<u64>) -> String {
        if let Some(offset) = offset {
            format!("LIMIT {} OFFSET {}", limit, offset)
        } else {
            format!("LIMIT {}", limit)
        }
    }

    fn supported_features(&self) -> &DialectFeatures {
        &self.features
    }

    fn keywords(&self) -> &SqlKeywords {
        &self.keywords
    }

    fn functions(&self) -> &SqlFunctions {
        &self.functions
    }

    fn type_to_sql(&self, sql_type: &crate::stdlib::packages::db_sql::SqlType) -> String {
        match sql_type {
            crate::stdlib::packages::db_sql::SqlType::Boolean => "BOOLEAN".to_string(),
            crate::stdlib::packages::db_sql::SqlType::Integer => "INTEGER".to_string(),
            crate::stdlib::packages::db_sql::SqlType::BigInt => "BIGINT".to_string(),
            crate::stdlib::packages::db_sql::SqlType::Text => "TEXT".to_string(),
            crate::stdlib::packages::db_sql::SqlType::Json => "JSONB".to_string(),
            crate::stdlib::packages::db_sql::SqlType::Timestamp => "TIMESTAMP".to_string(),
            crate::stdlib::packages::db_sql::SqlType::Uuid => "UUID".to_string(),
            _ => sql_type.to_sql(), // Fallback to default
        }
    }
}

impl MySqlDialect {
    /// slay Create new MySQL dialect
    pub fn new() -> Self {
        let features = DialectFeatures {
            supports_cte: true, // MySQL 8.0+
            supports_window_functions: true, // MySQL 8.0+
            supports_json: true,
            supports_recursive: true, // MySQL 8.0+
            supports_upsert: true, // ON DUPLICATE KEY UPDATE
            supports_arrays: false, // MySQL doesn't have native arrays
            supports_stored_procedures: true,
            supports_fulltext: true,
            supports_spatial: true,
            max_identifier_length: 64,
            case_sensitive_identifiers: false,
        };

        let keywords = SqlKeywords {
            reserved_words: vec![
                "SELECT".to_string(), "FROM".to_string(), "WHERE".to_string(),
                "INSERT".to_string(), "UPDATE".to_string(), "DELETE".to_string(),
                "DUPLICATE".to_string(), "KEY".to_string(),
            ],
            function_names: vec![
                "NOW".to_string(), "CURDATE".to_string(),
                "CHAR_LENGTH".to_string(), "SUBSTRING".to_string(),
            ],
            type_names: vec![
                "INT".to_string(), "VARCHAR".to_string(), "TEXT".to_string(),
                "DATETIME".to_string(), "JSON".to_string(),
            ],
        };

        let functions = SqlFunctions {
            string_functions: {
                let mut map = HashMap::new();
                map.insert("LENGTH".to_string(), "CHAR_LENGTH".to_string());
                map.insert("SUBSTRING".to_string(), "SUBSTRING".to_string());
                map
            },
            datetime_functions: {
                let mut map = HashMap::new();
                map.insert("NOW".to_string(), "NOW()".to_string());
                map.insert("CURRENT_TIMESTAMP".to_string(), "CURRENT_TIMESTAMP()".to_string());
                map
            },
            math_functions: HashMap::new(),
            aggregate_functions: HashMap::new(),
            json_functions: {
                let mut map = HashMap::new();
                map.insert("JSON_EXTRACT".to_string(), "JSON_EXTRACT".to_string());
                map.insert("JSON_OBJECT".to_string(), "JSON_OBJECT".to_string());
                map
            },
        };

        Self { features, keywords, functions }
    }
}

impl SqlDialectTrait for MySqlDialect {
    fn name(&self) -> &str {
        "MySQL"
    }

    fn parameter_placeholder(&self, _index: usize) -> String {
        "?".to_string() // MySQL uses ? for all parameters
    }

    fn named_parameter_placeholder(&self, _name: &str) -> String {
        "?".to_string() // MySQL doesn't support named parameters natively
    }

    fn quote_identifier(&self, identifier: &str) -> String {
        format!("`{}`", identifier.replace('`', "``"))
    }

    fn quote_string(&self, value: &str) -> String {
        format!("'{}'", value.replace('\'', "''").replace('\\', "\\\\"))
    }

    fn limit_clause(&self, limit: u64, offset: Option<u64>) -> String {
        if let Some(offset) = offset {
            format!("LIMIT {}, {}", offset, limit)
        } else {
            format!("LIMIT {}", limit)
        }
    }

    fn supported_features(&self) -> &DialectFeatures {
        &self.features
    }

    fn keywords(&self) -> &SqlKeywords {
        &self.keywords
    }

    fn functions(&self) -> &SqlFunctions {
        &self.functions
    }

    fn type_to_sql(&self, sql_type: &crate::stdlib::packages::db_sql::SqlType) -> String {
        match sql_type {
            crate::stdlib::packages::db_sql::SqlType::Boolean => "BOOLEAN".to_string(),
            crate::stdlib::packages::db_sql::SqlType::Integer => "INT".to_string(),
            crate::stdlib::packages::db_sql::SqlType::BigInt => "BIGINT".to_string(),
            crate::stdlib::packages::db_sql::SqlType::Text => "TEXT".to_string(),
            crate::stdlib::packages::db_sql::SqlType::Json => "JSON".to_string(),
            crate::stdlib::packages::db_sql::SqlType::Timestamp => "DATETIME".to_string(),
            crate::stdlib::packages::db_sql::SqlType::VarChar(len) => format!("VARCHAR({})", len),
            _ => sql_type.to_sql(),
        }
    }
}

impl SqliteDialect {
    /// slay Create new SQLite dialect
    pub fn new() -> Self {
        let features = DialectFeatures {
            supports_cte: true,
            supports_window_functions: true, // SQLite 3.25+
            supports_json: true, // SQLite 3.38+
            supports_recursive: true,
            supports_upsert: true, // INSERT OR REPLACE, ON CONFLICT
            supports_arrays: false,
            supports_stored_procedures: false,
            supports_fulltext: true, // FTS extension
            supports_spatial: false, // Requires extension
            max_identifier_length: 255,
            case_sensitive_identifiers: false,
        };

        let keywords = SqlKeywords {
            reserved_words: vec![
                "SELECT".to_string(), "FROM".to_string(), "WHERE".to_string(),
                "REPLACE".to_string(), "CONFLICT".to_string(),
            ],
            function_names: vec![
                "DATETIME".to_string(), "LENGTH".to_string(),
                "SUBSTR".to_string(),
            ],
            type_names: vec![
                "INTEGER".to_string(), "TEXT".to_string(), "REAL".to_string(),
                "BLOB".to_string(),
            ],
        };

        let functions = SqlFunctions {
            string_functions: {
                let mut map = HashMap::new();
                map.insert("LENGTH".to_string(), "LENGTH".to_string());
                map.insert("SUBSTRING".to_string(), "SUBSTR".to_string());
                map
            },
            datetime_functions: {
                let mut map = HashMap::new();
                map.insert("NOW".to_string(), "DATETIME('now')".to_string());
                map.insert("CURRENT_TIMESTAMP".to_string(), "CURRENT_TIMESTAMP".to_string());
                map
            },
            math_functions: HashMap::new(),
            aggregate_functions: HashMap::new(),
            json_functions: {
                let mut map = HashMap::new();
                map.insert("JSON_EXTRACT".to_string(), "JSON_EXTRACT".to_string());
                map.insert("JSON_OBJECT".to_string(), "JSON_OBJECT".to_string());
                map
            },
        };

        Self { features, keywords, functions }
    }
}

impl SqlDialectTrait for SqliteDialect {
    fn name(&self) -> &str {
        "SQLite"
    }

    fn parameter_placeholder(&self, _index: usize) -> String {
        "?".to_string() // SQLite uses ? for all parameters
    }

    fn named_parameter_placeholder(&self, name: &str) -> String {
        format!(":{}", name) // SQLite supports :name syntax
    }

    fn quote_identifier(&self, identifier: &str) -> String {
        format!("\"{}\"", identifier.replace('"', "\"\""))
    }

    fn quote_string(&self, value: &str) -> String {
        format!("'{}'", value.replace('\'', "''"))
    }

    fn limit_clause(&self, limit: u64, offset: Option<u64>) -> String {
        if let Some(offset) = offset {
            format!("LIMIT {} OFFSET {}", limit, offset)
        } else {
            format!("LIMIT {}", limit)
        }
    }

    fn supported_features(&self) -> &DialectFeatures {
        &self.features
    }

    fn keywords(&self) -> &SqlKeywords {
        &self.keywords
    }

    fn functions(&self) -> &SqlFunctions {
        &self.functions
    }

    fn type_to_sql(&self, sql_type: &crate::stdlib::packages::db_sql::SqlType) -> String {
        match sql_type {
            crate::stdlib::packages::db_sql::SqlType::Boolean => "INTEGER".to_string(), // SQLite stores boolean as integer
            crate::stdlib::packages::db_sql::SqlType::Integer => "INTEGER".to_string(),
            crate::stdlib::packages::db_sql::SqlType::BigInt => "INTEGER".to_string(),
            crate::stdlib::packages::db_sql::SqlType::Float => "REAL".to_string(),
            crate::stdlib::packages::db_sql::SqlType::Double => "REAL".to_string(),
            crate::stdlib::packages::db_sql::SqlType::Text => "TEXT".to_string(),
            crate::stdlib::packages::db_sql::SqlType::VarChar(_) => "TEXT".to_string(), // SQLite doesn't enforce varchar length
            crate::stdlib::packages::db_sql::SqlType::Binary(_) => "BLOB".to_string(),
            crate::stdlib::packages::db_sql::SqlType::Blob => "BLOB".to_string(),
            crate::stdlib::packages::db_sql::SqlType::Timestamp => "TEXT".to_string(), // SQLite stores timestamps as text
            _ => sql_type.to_sql(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_postgresql_dialect() {
        let dialect = PostgreSqlDialect::new();
        assert_eq!(dialect.name(), "PostgreSQL");
        assert_eq!(dialect.parameter_placeholder(0), "$1");
        assert_eq!(dialect.parameter_placeholder(1), "$2");
        assert_eq!(dialect.quote_identifier("table"), "\"table\"");
        assert_eq!(dialect.quote_string("test's"), "'test''s'");
        assert_eq!(dialect.limit_clause(10, Some(5)), "LIMIT 10 OFFSET 5");
        assert_eq!(dialect.limit_clause(10, None), "LIMIT 10");
    }

    #[test]
    fn test_mysql_dialect() {
        let dialect = MySqlDialect::new();
        assert_eq!(dialect.name(), "MySQL");
        assert_eq!(dialect.parameter_placeholder(0), "?");
        assert_eq!(dialect.parameter_placeholder(1), "?");
        assert_eq!(dialect.quote_identifier("table"), "`table`");
        assert_eq!(dialect.limit_clause(10, Some(5)), "LIMIT 5, 10");
        assert_eq!(dialect.limit_clause(10, None), "LIMIT 10");
    }

    #[test]
    fn test_sqlite_dialect() {
        let dialect = SqliteDialect::new();
        assert_eq!(dialect.name(), "SQLite");
        assert_eq!(dialect.parameter_placeholder(0), "?");
        assert_eq!(dialect.named_parameter_placeholder("name"), ":name");
        assert_eq!(dialect.quote_identifier("table"), "\"table\"");
        assert!(dialect.supported_features().supports_cte);
        assert!(!dialect.supported_features().supports_stored_procedures);
    }

    #[test]
    fn test_dialect_features() {
        let pg_dialect = PostgreSqlDialect::new();
        let mysql_dialect = MySqlDialect::new();
        let sqlite_dialect = SqliteDialect::new();

        assert!(pg_dialect.supported_features().supports_arrays);
        assert!(!mysql_dialect.supported_features().supports_arrays);
        assert!(!sqlite_dialect.supported_features().supports_arrays);

        assert!(pg_dialect.supported_features().supports_stored_procedures);
        assert!(mysql_dialect.supported_features().supports_stored_procedures);
        assert!(!sqlite_dialect.supported_features().supports_stored_procedures);
    }

    #[test]
    fn test_type_conversions() {
        let pg_dialect = PostgreSqlDialect::new();
        let mysql_dialect = MySqlDialect::new();
        let sqlite_dialect = SqliteDialect::new();

        let bool_type = crate::stdlib::packages::db_sql::SqlType::Boolean;
        
        assert_eq!(pg_dialect.type_to_sql(&bool_type), "BOOLEAN");
        assert_eq!(mysql_dialect.type_to_sql(&bool_type), "BOOLEAN");
        assert_eq!(sqlite_dialect.type_to_sql(&bool_type), "INTEGER");
    }

    #[test]
    fn test_sql_functions() {
        let pg_dialect = PostgreSqlDialect::new();
        let functions = pg_dialect.functions();
        
        assert!(functions.string_functions.contains_key("LENGTH"));
        assert!(functions.datetime_functions.contains_key("NOW"));
        assert!(functions.json_functions.contains_key("JSON_EXTRACT"));
    }
}
