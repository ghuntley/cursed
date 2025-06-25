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
    /// fr fr Numeric version (e.g., 3039004)
    /// fr fr Source ID string
impl SqliteVersion {
    /// slay Parse version from string
    pub fn parse(version_str: &str) -> SqliteResult<Self> {
        let parts: Vec<&str> = version_str.split('.').collect();
        if parts.len() < 3 {
            return Err(SqliteError::invalid_parameter("Invalid version format"));
        let major: i32 = parts[0].parse()
            .map_err(|_| SqliteError::invalid_parameter("Invalid major version"))?;
        let minor: i32 = parts[1].parse()
            .map_err(|_| SqliteError::invalid_parameter("Invalid minor version"))?;
        let patch: i32 = parts[2].parse()
            .map_err(|_| SqliteError::invalid_parameter("Invalid patch version"))?;

        let version_number = major * 1000000 + minor * 1000 + patch;

        Ok(Self {
        })
    /// slay Check if version is at least the specified version
    pub fn is_at_least(&self, major: i32, minor: i32, patch: i32) -> bool {
        let required_version = major * 1000000 + minor * 1000 + patch;
        self.version_number >= required_version
    /// slay Get major version
    pub fn major(&self) -> i32 {
        self.version_number / 1000000
    /// slay Get minor version
    pub fn minor(&self) -> i32 {
        (self.version_number / 1000) % 1000
    /// slay Get patch version
    pub fn patch(&self) -> i32 {
        self.version_number % 1000
    /// slay Check if this is a stable release
    pub fn is_stable(&self) -> bool {
        // SQLite stable versions typically have even minor numbers
        self.minor() % 2 == 0
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
        }
    }
/// fr fr SQLite feature detection
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SqliteFeatures {
    /// fr fr Thread safety level (0=none, 1=serialized, 2=multi-thread)
    /// fr fr Memory management enabled
    /// fr fr Full-text search (FTS) versions
    /// fr fr JSON support
    /// fr fr R-Tree spatial index
    /// fr fr ICU internationalization
    /// fr fr Loadable extensions
    /// fr fr Virtual table support
    /// fr fr Database encryption
    /// fr fr Math functions
    /// fr fr Statistics functions  
    /// fr fr Session extension
    /// fr fr Preupdate hooks
    /// fr fr Window functions
    /// fr fr Generated columns
    /// fr fr Without ROWID tables
    /// fr fr Common Table Expressions
    /// fr fr Recursive CTEs
    /// fr fr Partial indexes
    /// fr fr Expression indexes
    /// fr fr Foreign key support
    /// fr fr Triggers
    /// fr fr Views
    /// fr fr Compound SELECT
    /// fr fr UPSERT
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
    /// slay Check if feature is available
    pub fn has_feature(&self, feature: &str) -> bool {
        match feature.to_lowercase().as_str() {
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
    /// slay Get thread safety description
    pub fn thread_safety_description(&self) -> &'static str {
        match self.threadsafe {
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
            }
        }
        
        escaped
    /// slay Check if string is SQL keyword
    pub fn is_sql_keyword(word: &str) -> bool {
        const KEYWORDS: &[&str] = &[
        ];
        
        KEYWORDS.contains(&word.to_uppercase().as_str())
    /// slay Generate CREATE TABLE statement
    pub fn generate_create_table(
        columns: &[(String, String, Vec<String>)], // (name, type, constraints)
    ) -> String {
        let mut sql = String::from("CREATE TABLE ");
        
        if if_not_exists {
            sql.push_str("IF NOT EXISTS ");
        sql.push_str(&Self::quote_identifier(table_name));
        sql.push_str(" (\n");
        
        let mut parts = Vec::new();
        
        // Add column definitions
        for (name, data_type, constraints) in columns {
                data_type
            );
            
            for constraint in constraints {
                column_def.push(' ');
                column_def.push_str(constraint);
            parts.push(column_def);
        // Add table constraints
        for constraint in table_constraints {
            parts.push(format!("  {}", constraint));
        sql.push_str(&parts.join(",\n"));
        sql.push_str("\n)");
        
        sql
    /// slay Generate CREATE INDEX statement
    pub fn generate_create_index(
    ) -> String {
        let mut sql = String::from("CREATE ");
        
        if unique {
            sql.push_str("UNIQUE ");
        sql.push_str("INDEX ");
        
        if if_not_exists {
            sql.push_str("IF NOT EXISTS ");
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
        sql
    /// slay Validate table name
    pub fn validate_table_name(name: &str) -> SqliteResult<()> {
        if name.is_empty() {
            return Err(SqliteError::invalid_parameter("Table name cannot be empty"));
        if name.len() > 128 {
            return Err(SqliteError::invalid_parameter("Table name too long (max 128 characters)"));
        // Check for valid characters (alphanumeric, underscore, starts with letter or underscore)
        let first_char = name.chars().next().unwrap();
        if !first_char.is_alphabetic() && first_char != '_' {
            return Err(SqliteError::invalid_parameter("Table name must start with letter or underscore"));
        for ch in name.chars() {
            if !ch.is_alphanumeric() && ch != '_' {
                return Err(SqliteError::invalid_parameter("Table name contains invalid characters"));
            }
        }
        
        Ok(())
    /// slay Validate column name
    pub fn validate_column_name(name: &str) -> SqliteResult<()> {
        // Same rules as table name for simplicity
        Self::validate_table_name(name)
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
            error.severity
        )
    /// slay Build connection string from components
    pub fn build_connection_string(
    ) -> String {
        if parameters.is_empty() {
            return path.to_string();
        let mut uri = if path == ":memory:" {
            ":memory:".to_string()
        } else {
            format!("file:{}", path)

        let params: Vec<String> = parameters.iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect();

        if !params.is_empty() {
            uri.push('?');
            uri.push_str(&params.join("&"));
        uri
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

