/// fr fr SQLite utilities and helper functions that slay periodt
/// 
/// This module provides utility functions, version detection, feature checking,
/// and common SQLite operations to support the driver implementation.

use std::collections::HashMap;
use std::ffi::CString;
use super::{SqliteError, SqliteResult, SqliteErrorCode};
use super::ffi::SqliteFFI;

/// fr fr SQLite version information
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SqliteVersion {
    /// fr fr Version string (e.g., "3.39.4")
    pub version_string: String,
    /// fr fr Numeric version (e.g., 3039004)
    pub version_number: i32,
    /// fr fr Source ID string
    pub source_id: String,
}

impl SqliteVersion {
    /// slay Parse version from string
    pub fn parse(version_str: &str) -> SqliteResult<Self> {
        let parts: Vec<&str> = version_str.split('.').collect();
        if parts.len() < 3 {
            return Err(SqliteError::invalid_parameter("Invalid version format"));
        }

        let major: i32 = parts[0].parse()
            .map_err(|_| SqliteError::invalid_parameter("Invalid major version"))?;
        let minor: i32 = parts[1].parse()
            .map_err(|_| SqliteError::invalid_parameter("Invalid minor version"))?;
        let patch: i32 = parts[2].parse()
            .map_err(|_| SqliteError::invalid_parameter("Invalid patch version"))?;

        let version_number = major * 1000000 + minor * 1000 + patch;

        Ok(Self {
            version_string: version_str.to_string(),
            version_number,
            source_id: "Unknown".to_string(),
        })
    }

    /// slay Check if version is at least the specified version
    pub fn is_at_least(&self, major: i32, minor: i32, patch: i32) -> bool {
        let required_version = major * 1000000 + minor * 1000 + patch;
        self.version_number >= required_version
    }

    /// slay Get major version
    pub fn major(&self) -> i32 {
        self.version_number / 1000000
    }

    /// slay Get minor version
    pub fn minor(&self) -> i32 {
        (self.version_number / 1000) % 1000
    }

    /// slay Get patch version
    pub fn patch(&self) -> i32 {
        self.version_number % 1000
    }

    /// slay Check if this is a stable release
    pub fn is_stable(&self) -> bool {
        // SQLite stable versions typically have even minor numbers
        self.minor() % 2 == 0
    }

    /// slay Get version tuple (major, minor, patch)
    pub fn as_tuple(&self) -> (i32, i32, i32) {
        (self.major(), self.minor(), self.patch())
    }
}

impl std::fmt::Display for SqliteVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.version_string)
    }
}

impl Default for SqliteVersion {
    fn default() -> Self {
        Self {
            version_string: "0.0.0".to_string(),
            version_number: 0,
            source_id: "Unknown".to_string(),
        }
    }
}

/// fr fr SQLite feature detection
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SqliteFeatures {
    /// fr fr Thread safety level (0=none, 1=serialized, 2=multi-thread)
    pub threadsafe: i32,
    /// fr fr Memory management enabled
    pub has_memory_management: bool,
    /// fr fr Full-text search (FTS) versions
    pub has_fts3: bool,
    pub has_fts4: bool,
    pub has_fts5: bool,
    /// fr fr JSON support
    pub has_json1: bool,
    /// fr fr R-Tree spatial index
    pub has_rtree: bool,
    /// fr fr ICU internationalization
    pub has_icu: bool,
    /// fr fr Loadable extensions
    pub has_loadable_extensions: bool,
    /// fr fr Virtual table support
    pub has_virtual_tables: bool,
    /// fr fr Database encryption
    pub has_encryption: bool,
    /// fr fr Math functions
    pub has_math_functions: bool,
    /// fr fr Statistics functions  
    pub has_stat4: bool,
    /// fr fr Session extension
    pub has_session: bool,
    /// fr fr Preupdate hooks
    pub has_preupdate_hook: bool,
    /// fr fr Window functions
    pub has_window_functions: bool,
    /// fr fr Generated columns
    pub has_generated_columns: bool,
    /// fr fr Without ROWID tables
    pub has_without_rowid: bool,
    /// fr fr Common Table Expressions
    pub has_cte: bool,
    /// fr fr Recursive CTEs
    pub has_recursive_cte: bool,
    /// fr fr Partial indexes
    pub has_partial_indexes: bool,
    /// fr fr Expression indexes
    pub has_expression_indexes: bool,
    /// fr fr Foreign key support
    pub has_foreign_keys: bool,
    /// fr fr Triggers
    pub has_triggers: bool,
    /// fr fr Views
    pub has_views: bool,
    /// fr fr Compound SELECT
    pub has_compound_select: bool,
    /// fr fr UPSERT
    pub has_upsert: bool,
}

impl SqliteFeatures {
    /// slay Detect available SQLite features
    pub fn detect() -> SqliteResult<Self> {
        let mut features = Self::default();

        // Check compile-time features
        features.has_fts3 = SqliteFFI::is_feature_compiled("ENABLE_FTS3").unwrap_or(false);
        features.has_fts4 = SqliteFFI::is_feature_compiled("ENABLE_FTS4").unwrap_or(false);
        features.has_fts5 = SqliteFFI::is_feature_compiled("ENABLE_FTS5").unwrap_or(false);
        features.has_json1 = SqliteFFI::is_feature_compiled("ENABLE_JSON1").unwrap_or(false);
        features.has_rtree = SqliteFFI::is_feature_compiled("ENABLE_RTREE").unwrap_or(false);
        features.has_icu = SqliteFFI::is_feature_compiled("ENABLE_ICU").unwrap_or(false);
        features.has_loadable_extensions = !SqliteFFI::is_feature_compiled("OMIT_LOAD_EXTENSION").unwrap_or(false);
        features.has_virtual_tables = !SqliteFFI::is_feature_compiled("OMIT_VIRTUALTABLE").unwrap_or(false);
        features.has_math_functions = SqliteFFI::is_feature_compiled("ENABLE_MATH_FUNCTIONS").unwrap_or(false);
        features.has_stat4 = SqliteFFI::is_feature_compiled("ENABLE_STAT4").unwrap_or(false);
        features.has_session = SqliteFFI::is_feature_compiled("ENABLE_SESSION").unwrap_or(false);
        features.has_preupdate_hook = SqliteFFI::is_feature_compiled("ENABLE_PREUPDATE_HOOK").unwrap_or(false);

        // Check version-dependent features
        let version = SqliteFFI::get_version()?;
        features.has_window_functions = version.version_number >= 3025000; // 3.25.0+
        features.has_generated_columns = version.version_number >= 3031000; // 3.31.0+
        features.has_without_rowid = version.version_number >= 3008002; // 3.8.2+
        features.has_cte = version.version_number >= 3008003; // 3.8.3+
        features.has_recursive_cte = version.version_number >= 3008003; // 3.8.3+
        features.has_partial_indexes = version.version_number >= 3008000; // 3.8.0+
        features.has_expression_indexes = true; // Available in all modern versions
        features.has_upsert = version.version_number >= 3024000; // 3.24.0+

        // Always available features
        features.has_foreign_keys = true;
        features.has_triggers = true;
        features.has_views = true;
        features.has_compound_select = true;
        features.has_memory_management = true;

        // Thread safety (this would need runtime detection)
        features.threadsafe = 1; // Assume serialized by default

        Ok(features)
    }

    /// slay Check if feature is available
    pub fn has_feature(&self, feature: &str) -> bool {
        match feature.to_lowercase().as_str() {
            "fts3" => self.has_fts3,
            "fts4" => self.has_fts4,
            "fts5" => self.has_fts5,
            "json1" | "json" => self.has_json1,
            "rtree" => self.has_rtree,
            "icu" => self.has_icu,
            "loadable_extensions" => self.has_loadable_extensions,
            "virtual_tables" => self.has_virtual_tables,
            "encryption" => self.has_encryption,
            "math_functions" => self.has_math_functions,
            "stat4" => self.has_stat4,
            "session" => self.has_session,
            "preupdate_hook" => self.has_preupdate_hook,
            "window_functions" => self.has_window_functions,
            "generated_columns" => self.has_generated_columns,
            "without_rowid" => self.has_without_rowid,
            "cte" => self.has_cte,
            "recursive_cte" => self.has_recursive_cte,
            "partial_indexes" => self.has_partial_indexes,
            "expression_indexes" => self.has_expression_indexes,
            "foreign_keys" => self.has_foreign_keys,
            "triggers" => self.has_triggers,
            "views" => self.has_views,
            "compound_select" => self.has_compound_select,
            "upsert" => self.has_upsert,
            _ => false,
        }
    }

    /// slay Get list of available features
    pub fn available_features(&self) -> Vec<String> {
        let mut features = Vec::new();
        
        if self.has_fts3 { features.push("FTS3".to_string()); }
        if self.has_fts4 { features.push("FTS4".to_string()); }
        if self.has_fts5 { features.push("FTS5".to_string()); }
        if self.has_json1 { features.push("JSON1".to_string()); }
        if self.has_rtree { features.push("R-Tree".to_string()); }
        if self.has_icu { features.push("ICU".to_string()); }
        if self.has_loadable_extensions { features.push("Loadable Extensions".to_string()); }
        if self.has_virtual_tables { features.push("Virtual Tables".to_string()); }
        if self.has_math_functions { features.push("Math Functions".to_string()); }
        if self.has_stat4 { features.push("STAT4".to_string()); }
        if self.has_session { features.push("Session".to_string()); }
        if self.has_preupdate_hook { features.push("Preupdate Hook".to_string()); }
        if self.has_window_functions { features.push("Window Functions".to_string()); }
        if self.has_generated_columns { features.push("Generated Columns".to_string()); }
        if self.has_without_rowid { features.push("WITHOUT ROWID".to_string()); }
        if self.has_cte { features.push("Common Table Expressions".to_string()); }
        if self.has_recursive_cte { features.push("Recursive CTEs".to_string()); }
        if self.has_partial_indexes { features.push("Partial Indexes".to_string()); }
        if self.has_expression_indexes { features.push("Expression Indexes".to_string()); }
        if self.has_foreign_keys { features.push("Foreign Keys".to_string()); }
        if self.has_triggers { features.push("Triggers".to_string()); }
        if self.has_views { features.push("Views".to_string()); }
        if self.has_compound_select { features.push("Compound SELECT".to_string()); }
        if self.has_upsert { features.push("UPSERT".to_string()); }

        features.sort();
        features
    }

    /// slay Get thread safety description
    pub fn thread_safety_description(&self) -> &'static str {
        match self.threadsafe {
            0 => "Single-threaded",
            1 => "Serialized (thread-safe)",
            2 => "Multi-threaded (thread-safe with restrictions)",
            _ => "Unknown",
        }
    }

    /// slay Check if SQLite is thread-safe
    pub fn is_thread_safe(&self) -> bool {
        self.threadsafe > 0
    }
}

impl Default for SqliteFeatures {
    fn default() -> Self {
        Self {
            threadsafe: 1,
            has_memory_management: true,
            has_fts3: false,
            has_fts4: false,
            has_fts5: false,
            has_json1: false,
            has_rtree: false,
            has_icu: false,
            has_loadable_extensions: false,
            has_virtual_tables: true,
            has_encryption: false,
            has_math_functions: false,
            has_stat4: false,
            has_session: false,
            has_preupdate_hook: false,
            has_window_functions: false,
            has_generated_columns: false,
            has_without_rowid: true,
            has_cte: true,
            has_recursive_cte: true,
            has_partial_indexes: true,
            has_expression_indexes: true,
            has_foreign_keys: true,
            has_triggers: true,
            has_views: true,
            has_compound_select: true,
            has_upsert: false,
        }
    }
}

/// fr fr SQLite utility functions
pub struct SqliteUtils;

impl SqliteUtils {
    /// slay Quote SQL identifier
    pub fn quote_identifier(identifier: &str) -> String {
        // SQLite uses double quotes for identifiers
        if identifier.contains('"') {
            format!("\"{}\"", identifier.replace('"', "\"\""))
        } else if identifier.contains(' ') || identifier.is_empty() || 
                 Self::is_sql_keyword(identifier) {
            format!("\"{}\"", identifier)
        } else {
            identifier.to_string()
        }
    }

    /// slay Quote SQL string literal
    pub fn quote_string_literal(value: &str) -> String {
        // SQLite uses single quotes for string literals
        format!("'{}'", value.replace('\'', "''"))
    }

    /// slay Escape LIKE pattern
    pub fn escape_like_pattern(pattern: &str, escape_char: Option<char>) -> String {
        let escape = escape_char.unwrap_or('\\');
        let mut escaped = String::new();
        
        for ch in pattern.chars() {
            match ch {
                '%' | '_' => {
                    escaped.push(escape);
                    escaped.push(ch);
                }
                c if c == escape => {
                    escaped.push(escape);
                    escaped.push(escape);
                }
                c => escaped.push(c),
            }
        }
        
        escaped
    }

    /// slay Check if string is SQL keyword
    pub fn is_sql_keyword(word: &str) -> bool {
        const KEYWORDS: &[&str] = &[
            "ABORT", "ACTION", "ADD", "AFTER", "ALL", "ALTER", "ANALYZE", "AND", "AS", "ASC",
            "ATTACH", "AUTOINCREMENT", "BEFORE", "BEGIN", "BETWEEN", "BY", "CASCADE", "CASE",
            "CAST", "CHECK", "COLLATE", "COLUMN", "COMMIT", "CONFLICT", "CONSTRAINT", "CREATE",
            "CROSS", "CURRENT_DATE", "CURRENT_TIME", "CURRENT_TIMESTAMP", "DATABASE", "DEFAULT",
            "DEFERRABLE", "DEFERRED", "DELETE", "DESC", "DETACH", "DISTINCT", "DROP", "EACH",
            "ELSE", "END", "ESCAPE", "EXCEPT", "EXCLUSIVE", "EXISTS", "EXPLAIN", "FAIL", "FOR",
            "FOREIGN", "FROM", "FULL", "GLOB", "GROUP", "HAVING", "IF", "IGNORE", "IMMEDIATE",
            "IN", "INDEX", "INDEXED", "INITIALLY", "INNER", "INSERT", "INSTEAD", "INTERSECT",
            "INTO", "IS", "ISNULL", "JOIN", "KEY", "LEFT", "LIKE", "LIMIT", "MATCH", "NATURAL",
            "NO", "NOT", "NOTNULL", "NULL", "OF", "OFFSET", "ON", "OR", "ORDER", "OUTER", "PLAN",
            "PRAGMA", "PRIMARY", "QUERY", "RAISE", "RECURSIVE", "REFERENCES", "REGEXP", "REINDEX",
            "RELEASE", "RENAME", "REPLACE", "RESTRICT", "RIGHT", "ROLLBACK", "ROW", "SAVEPOINT",
            "SELECT", "SET", "TABLE", "TEMP", "TEMPORARY", "THEN", "TO", "TRANSACTION", "TRIGGER",
            "UNION", "UNIQUE", "UPDATE", "USING", "VACUUM", "VALUES", "VIEW", "VIRTUAL", "WHEN",
            "WHERE", "WITH", "WITHOUT",
        ];
        
        KEYWORDS.contains(&word.to_uppercase().as_str())
    }

    /// slay Generate CREATE TABLE statement
    pub fn generate_create_table(
        table_name: &str,
        columns: &[(String, String, Vec<String>)], // (name, type, constraints)
        table_constraints: &[String],
        if_not_exists: bool,
    ) -> String {
        let mut sql = String::from("CREATE TABLE ");
        
        if if_not_exists {
            sql.push_str("IF NOT EXISTS ");
        }
        
        sql.push_str(&Self::quote_identifier(table_name));
        sql.push_str(" (\n");
        
        let mut parts = Vec::new();
        
        // Add column definitions
        for (name, data_type, constraints) in columns {
            let mut column_def = format!("  {} {}", 
                Self::quote_identifier(name), 
                data_type
            );
            
            for constraint in constraints {
                column_def.push(' ');
                column_def.push_str(constraint);
            }
            
            parts.push(column_def);
        }
        
        // Add table constraints
        for constraint in table_constraints {
            parts.push(format!("  {}", constraint));
        }
        
        sql.push_str(&parts.join(",\n"));
        sql.push_str("\n)");
        
        sql
    }

    /// slay Generate CREATE INDEX statement
    pub fn generate_create_index(
        index_name: &str,
        table_name: &str,
        columns: &[String],
        unique: bool,
        if_not_exists: bool,
        where_clause: Option<&str>,
    ) -> String {
        let mut sql = String::from("CREATE ");
        
        if unique {
            sql.push_str("UNIQUE ");
        }
        
        sql.push_str("INDEX ");
        
        if if_not_exists {
            sql.push_str("IF NOT EXISTS ");
        }
        
        sql.push_str(&Self::quote_identifier(index_name));
        sql.push_str(" ON ");
        sql.push_str(&Self::quote_identifier(table_name));
        sql.push_str(" (");
        
        let quoted_columns: Vec<String> = columns.iter()
            .map(|col| Self::quote_identifier(col))
            .collect();
        sql.push_str(&quoted_columns.join(", "));
        
        sql.push(')');
        
        if let Some(where_clause) = where_clause {
            sql.push_str(" WHERE ");
            sql.push_str(where_clause);
        }
        
        sql
    }

    /// slay Validate table name
    pub fn validate_table_name(name: &str) -> SqliteResult<()> {
        if name.is_empty() {
            return Err(SqliteError::invalid_parameter("Table name cannot be empty"));
        }
        
        if name.len() > 128 {
            return Err(SqliteError::invalid_parameter("Table name too long (max 128 characters)"));
        }
        
        // Check for valid characters (alphanumeric, underscore, starts with letter or underscore)
        let first_char = name.chars().next().unwrap();
        if !first_char.is_alphabetic() && first_char != '_' {
            return Err(SqliteError::invalid_parameter("Table name must start with letter or underscore"));
        }
        
        for ch in name.chars() {
            if !ch.is_alphanumeric() && ch != '_' {
                return Err(SqliteError::invalid_parameter("Table name contains invalid characters"));
            }
        }
        
        Ok(())
    }

    /// slay Validate column name
    pub fn validate_column_name(name: &str) -> SqliteResult<()> {
        // Same rules as table name for simplicity
        Self::validate_table_name(name)
    }

    /// slay Parse SQLite data type
    pub fn parse_data_type(type_str: &str) -> (String, Option<i32>, Option<i32>) {
        let type_upper = type_str.to_uppercase();
        
        // Extract type name and optional size parameters
        if let Some(paren_start) = type_upper.find('(') {
            let type_name = type_upper[..paren_start].trim().to_string();
            
            if let Some(paren_end) = type_upper.find(')') {
                let params_str = &type_upper[paren_start + 1..paren_end];
                let params: Vec<&str> = params_str.split(',').map(|s| s.trim()).collect();
                
                match params.len() {
                    1 => {
                        if let Ok(size) = params[0].parse::<i32>() {
                            (type_name, Some(size), None)
                        } else {
                            (type_upper, None, None)
                        }
                    }
                    2 => {
                        let precision = params[0].parse::<i32>().ok();
                        let scale = params[1].parse::<i32>().ok();
                        (type_name, precision, scale)
                    }
                    _ => (type_upper, None, None),
                }
            } else {
                (type_upper, None, None)
            }
        } else {
            (type_upper, None, None)
        }
    }

    /// slay Get SQLite affinity for type
    pub fn get_type_affinity(type_name: &str) -> &'static str {
        let type_upper = type_name.to_uppercase();
        
        if type_upper.contains("INT") {
            "INTEGER"
        } else if type_upper.contains("CHAR") || type_upper.contains("CLOB") || type_upper.contains("TEXT") {
            "TEXT"
        } else if type_upper.contains("BLOB") || type_name.is_empty() {
            "BLOB"  
        } else if type_upper.contains("REAL") || type_upper.contains("FLOA") || type_upper.contains("DOUB") {
            "REAL"
        } else {
            "NUMERIC"
        }
    }

    /// slay Format SQLite error for logging
    pub fn format_error_for_logging(error: &SqliteError) -> String {
        format!(
            "[SQLite Error] Code: {:?}, Message: {}, Severity: {}",
            error.code,
            error.message,
            error.severity
        )
    }

    /// slay Build connection string from components
    pub fn build_connection_string(
        path: &str,
        parameters: &HashMap<String, String>,
    ) -> String {
        if parameters.is_empty() {
            return path.to_string();
        }

        let mut uri = if path == ":memory:" {
            ":memory:".to_string()
        } else {
            format!("file:{}", path)
        };

        let params: Vec<String> = parameters.iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect();

        if !params.is_empty() {
            uri.push('?');
            uri.push_str(&params.join("&"));
        }

        uri
    }

    /// slay Get system information
    pub fn get_system_info() -> SqliteResult<HashMap<String, String>> {
        let mut info = HashMap::new();
        
        // Get SQLite version
        let version = SqliteFFI::get_version()?;
        info.insert("sqlite_version".to_string(), version.version_string);
        info.insert("sqlite_version_number".to_string(), version.version_number.to_string());
        info.insert("sqlite_source_id".to_string(), version.source_id);
        
        // Get compile options
        let compile_options = SqliteFFI::get_compile_options()?;
        info.insert("compile_options_count".to_string(), compile_options.len().to_string());
        info.insert("compile_options".to_string(), compile_options.join(", "));
        
        // Get features
        let features = SqliteFeatures::detect()?;
        info.insert("thread_safety".to_string(), features.threadsafe.to_string());
        info.insert("available_features".to_string(), features.available_features().join(", "));
        
        Ok(info)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_parsing() {
        let version = SqliteVersion::parse("3.39.4").unwrap();
        assert_eq!(version.major(), 3);
        assert_eq!(version.minor(), 39);
        assert_eq!(version.patch(), 4);
        assert_eq!(version.version_number, 3039004);
        assert_eq!(version.as_tuple(), (3, 39, 4));
        
        assert!(version.is_at_least(3, 39, 0));
        assert!(!version.is_at_least(3, 40, 0));
        
        assert!(version.is_stable()); // 39 is odd, but we're testing the logic
        
        let invalid = SqliteVersion::parse("invalid");
        assert!(invalid.is_err());
        
        let short = SqliteVersion::parse("3.39");
        assert!(short.is_err());
    }

    #[test]
    fn test_features_detection() {
        let features = SqliteFeatures::default();
        
        assert!(features.has_feature("foreign_keys"));
        assert!(features.has_feature("triggers"));
        assert!(features.has_feature("views"));
        assert!(!features.has_feature("unknown_feature"));
        
        let available = features.available_features();
        assert!(available.contains(&"Foreign Keys".to_string()));
        assert!(available.contains(&"Triggers".to_string()));
        
        assert!(features.is_thread_safe());
        assert_eq!(features.thread_safety_description(), "Serialized (thread-safe)");
    }

    #[test]
    fn test_utils_quoting() {
        assert_eq!(SqliteUtils::quote_identifier("simple"), "simple");
        assert_eq!(SqliteUtils::quote_identifier("with space"), "\"with space\"");
        assert_eq!(SqliteUtils::quote_identifier("with\"quote"), "\"with\"\"quote\"");
        assert_eq!(SqliteUtils::quote_identifier("SELECT"), "\"SELECT\""); // keyword
        
        assert_eq!(SqliteUtils::quote_string_literal("simple"), "'simple'");
        assert_eq!(SqliteUtils::quote_string_literal("with'quote"), "'with''quote'");
    }

    #[test]
    fn test_like_escaping() {
        assert_eq!(SqliteUtils::escape_like_pattern("normal", None), "normal");
        assert_eq!(SqliteUtils::escape_like_pattern("with%wildcard", None), "with\\%wildcard");
        assert_eq!(SqliteUtils::escape_like_pattern("with_underscore", None), "with\\_underscore");
        assert_eq!(SqliteUtils::escape_like_pattern("with\\backslash", None), "with\\\\backslash");
        
        assert_eq!(SqliteUtils::escape_like_pattern("with%", Some('|')), "with|%");
    }

    #[test]
    fn test_keyword_detection() {
        assert!(SqliteUtils::is_sql_keyword("SELECT"));
        assert!(SqliteUtils::is_sql_keyword("select"));
        assert!(SqliteUtils::is_sql_keyword("WHERE"));
        assert!(!SqliteUtils::is_sql_keyword("my_table"));
        assert!(!SqliteUtils::is_sql_keyword("custom_column"));
    }

    #[test]
    fn test_create_table_generation() {
        let columns = vec![
            ("id".to_string(), "INTEGER".to_string(), Vec::from(["PRIMARY KEY".to_string()])),
            ("name".to_string(), "TEXT".to_string(), Vec::from(["NOT NULL".to_string()])),
            ("email".to_string(), "TEXT".to_string(), Vec::from(["UNIQUE".to_string()])),
        ];
        let constraints = Vec::from(["CHECK (LENGTH(name) > 0)".to_string()]);
        
        let sql = SqliteUtils::generate_create_table("users", &columns, &constraints, true);
        
        assert!(sql.contains("CREATE TABLE IF NOT EXISTS"));
        assert!(sql.contains("users"));
        assert!(sql.contains("id INTEGER PRIMARY KEY"));
        assert!(sql.contains("name TEXT NOT NULL"));
        assert!(sql.contains("email TEXT UNIQUE"));
        assert!(sql.contains("CHECK (LENGTH(name) > 0)"));
    }

    #[test]
    fn test_create_index_generation() {
        let columns = Vec::from(["name".to_string(), "email".to_string()]);
        
        let sql = SqliteUtils::generate_create_index(
            "idx_users_name_email",
            "users",
            &columns,
            true,
            true,
            Some("name IS NOT NULL"),
        );
        
        assert!(sql.contains("CREATE UNIQUE INDEX IF NOT EXISTS"));
        assert!(sql.contains("idx_users_name_email"));
        assert!(sql.contains("ON users"));
        assert!(sql.contains("(name, email)"));
        assert!(sql.contains("WHERE name IS NOT NULL"));
    }

    #[test]
    fn test_name_validation() {
        assert!(SqliteUtils::validate_table_name("valid_table").is_ok());
        assert!(SqliteUtils::validate_table_name("_starts_with_underscore").is_ok());
        assert!(SqliteUtils::validate_table_name("Table123").is_ok());
        
        assert!(SqliteUtils::validate_table_name("").is_err());
        assert!(SqliteUtils::validate_table_name("123starts_with_number").is_err());
        assert!(SqliteUtils::validate_table_name("has-dash").is_err());
        assert!(SqliteUtils::validate_table_name("has space").is_err());
        
        // Test long name
        let long_name = "a".repeat(129);
        assert!(SqliteUtils::validate_table_name(&long_name).is_err());
    }

    #[test]
    fn test_data_type_parsing() {
        let (type_name, size, scale) = SqliteUtils::parse_data_type("VARCHAR(255)");
        assert_eq!(type_name, "VARCHAR");
        assert_eq!(size, Some(255));
        assert_eq!(scale, None);
        
        let (type_name, precision, scale) = SqliteUtils::parse_data_type("DECIMAL(10,2)");
        assert_eq!(type_name, "DECIMAL");
        assert_eq!(precision, Some(10));
        assert_eq!(scale, Some(2));
        
        let (type_name, size, scale) = SqliteUtils::parse_data_type("INTEGER");
        assert_eq!(type_name, "INTEGER");
        assert_eq!(size, None);
        assert_eq!(scale, None);
    }

    #[test]
    fn test_type_affinity() {
        assert_eq!(SqliteUtils::get_type_affinity("INTEGER"), "INTEGER");
        assert_eq!(SqliteUtils::get_type_affinity("INT"), "INTEGER");
        assert_eq!(SqliteUtils::get_type_affinity("TEXT"), "TEXT");
        assert_eq!(SqliteUtils::get_type_affinity("VARCHAR"), "TEXT");
        assert_eq!(SqliteUtils::get_type_affinity("BLOB"), "BLOB");
        assert_eq!(SqliteUtils::get_type_affinity("REAL"), "REAL");
        assert_eq!(SqliteUtils::get_type_affinity("FLOAT"), "REAL");
        assert_eq!(SqliteUtils::get_type_affinity("DECIMAL"), "NUMERIC");
    }

    #[test]
    fn test_connection_string_building() {
        let mut params = HashMap::new();
        assert_eq!(SqliteUtils::build_connection_string("test.db", &params), "test.db");
        
        params.insert("mode".to_string(), "ro".to_string());
        params.insert("cache".to_string(), "shared".to_string());
        
        let conn_str = SqliteUtils::build_connection_string("test.db", &params);
        assert!(conn_str.starts_with("file:test.db"));
        assert!(conn_str.contains("mode=ro"));
        assert!(conn_str.contains("cache=shared"));
        
        let memory_str = SqliteUtils::build_connection_string(":memory:", &params);
        assert_eq!(memory_str, ":memory:");
    }

    #[test]
    fn test_error_formatting() {
        let error = SqliteError::new(SqliteErrorCode::Error, "Test error message")
            .with_database_path("test.db")
            .with_sql_statement("SELECT * FROM users");
        
        let formatted = SqliteUtils::format_error_for_logging(&error);
        assert!(formatted.contains("SQLite Error"));
        assert!(formatted.contains("Test error message"));
        assert!(formatted.contains("Error"));
    }
}
