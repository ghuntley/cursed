/// fr fr SQL dialect implementations - speaking the right SQL language periodt
///
/// This module provides SQL dialect-specific implementations for different
/// database systems. Each dialect knows its own quirks bestie!

use std::collections::HashMap;

/// fr fr SQL dialect enumeration
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SqlDialect {
    /// ANSI SQL standard
    /// PostgreSQL dialect
    /// MySQL dialect
    /// SQLite dialect
    /// SQL Server dialect
    /// Oracle dialect
    /// H2 dialect
    /// Custom dialect
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
//     fn type_to_sql(&self, sql_type: &crate::stdlib::packages::db_sql::SqlType) -> String;
/// fr fr PostgreSQL dialect implementation
#[derive(Debug)]
pub struct PostgreSqlDialect {
/// fr fr MySQL dialect implementation
#[derive(Debug)]
pub struct MySqlDialect {
/// fr fr SQLite dialect implementation
#[derive(Debug)]
pub struct SqliteDialect {
/// fr fr Dialect features and capabilities
#[derive(Debug, Clone)]
pub struct DialectFeatures {
    /// Supports CTEs (Common Table Expressions)
    /// Supports window functions
    /// Supports JSON functions
    /// Supports recursive queries
    /// Supports UPSERT operations
    /// Supports arrays
    /// Supports stored procedures
    /// Supports full-text search
    /// Supports GIS/spatial data
    /// Maximum identifier length
    /// Case sensitivity of identifiers
/// fr fr SQL keywords for the dialect
#[derive(Debug, Clone)]
pub struct SqlKeywords {
    /// Reserved words
    /// Function names
    /// Type names
/// fr fr SQL functions available in the dialect
#[derive(Debug, Clone)]
pub struct SqlFunctions {
    /// String functions
    /// Date/time functions
    /// Math functions
    /// Aggregate functions
    /// JSON functions
impl PostgreSqlDialect {
    /// slay Create new PostgreSQL dialect
    pub fn new() -> Self {
        let features = DialectFeatures {

        let keywords = SqlKeywords {
            reserved_words: vec![
            function_names: vec![
            type_names: vec![

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

        Self { features, keywords, functions }
    }
impl SqlDialectTrait for PostgreSqlDialect {
    fn name(&self) -> &str {
        "PostgreSQL"
    fn parameter_placeholder(&self, index: usize) -> String {
        format!("${}", index + 1) // PostgreSQL uses $1, $2, etc.
    fn named_parameter_placeholder(&self, name: &str) -> String {
        format!("${}", name) // PostgreSQL also supports named parameters
    fn quote_identifier(&self, identifier: &str) -> String {
        format!("\"{}\"", identifier.replace('"', "\"\""))
    fn quote_string(&self, value: &str) -> String {
        format!("'{}'", value.replace('\'', "''"))
    fn limit_clause(&self, limit: u64, offset: Option<u64>) -> String {
        if let Some(offset) = offset {
            format!("LIMIT {} OFFSET {}", limit, offset)
        } else {
            format!("LIMIT {}", limit)
        }
    }

    fn supported_features(&self) -> &DialectFeatures {
        &self.features
    fn keywords(&self) -> &SqlKeywords {
        &self.keywords
    fn functions(&self) -> &SqlFunctions {
        &self.functions
//     fn type_to_sql(&self, sql_type: &crate::stdlib::packages::db_sql::SqlType) -> String {
        match sql_type {
//             crate::stdlib::packages::db_sql::SqlType::Boolean => "BOOLEAN".to_string(),
//             crate::stdlib::packages::db_sql::SqlType::Integer => "INTEGER".to_string(),
//             crate::stdlib::packages::db_sql::SqlType::BigInt => "BIGINT".to_string(),
//             crate::stdlib::packages::db_sql::SqlType::Text => "TEXT".to_string(),
//             crate::stdlib::packages::db_sql::SqlType::Json => "JSONB".to_string(),
//             crate::stdlib::packages::db_sql::SqlType::Timestamp => "TIMESTAMP".to_string(),
//             crate::stdlib::packages::db_sql::SqlType::Uuid => "UUID".to_string(),
            _ => sql_type.to_sql(), // Fallback to default
        }
    }
impl MySqlDialect {
    /// slay Create new MySQL dialect
    pub fn new() -> Self {
        let features = DialectFeatures {
            supports_cte: true, // MySQL 8.0+
            supports_window_functions: true, // MySQL 8.0+
            supports_recursive: true, // MySQL 8.0+
            supports_upsert: true, // ON DUPLICATE KEY UPDATE
            supports_arrays: false, // MySQL doesn't have native arrays

        let keywords = SqlKeywords {
            reserved_words: vec![
            function_names: vec![
            type_names: vec![

        let functions = SqlFunctions {
            string_functions: {
                let mut map = HashMap::new();
                map.insert("LENGTH".to_string(), "CHAR_LENGTH".to_string());
                map.insert("SUBSTRING".to_string(), "SUBSTRING".to_string());
                map
            datetime_functions: {
                let mut map = HashMap::new();
                map.insert("NOW".to_string(), "NOW()".to_string());
                map.insert("CURRENT_TIMESTAMP".to_string(), "CURRENT_TIMESTAMP()".to_string());
                map
            json_functions: {
                let mut map = HashMap::new();
                map.insert("JSON_EXTRACT".to_string(), "JSON_EXTRACT".to_string());
                map.insert("JSON_OBJECT".to_string(), "JSON_OBJECT".to_string());
                map

        Self { features, keywords, functions }
    }
impl SqlDialectTrait for MySqlDialect {
    fn name(&self) -> &str {
        "MySQL"
    fn parameter_placeholder(&self, _index: usize) -> String {
        "?".to_string() // MySQL uses ? for all parameters
    fn named_parameter_placeholder(&self, _name: &str) -> String {
        "?".to_string() // MySQL doesn't support named parameters natively
    fn quote_identifier(&self, identifier: &str) -> String {
        format!("`{}`", identifier.replace('`', "``"))
    fn quote_string(&self, value: &str) -> String {
        format!("'{}'", value.replace('\'', "''").replace('\\', "\\\\"))
    fn limit_clause(&self, limit: u64, offset: Option<u64>) -> String {
        if let Some(offset) = offset {
            format!("LIMIT {}, {}", offset, limit)
        } else {
            format!("LIMIT {}", limit)
        }
    }

    fn supported_features(&self) -> &DialectFeatures {
        &self.features
    fn keywords(&self) -> &SqlKeywords {
        &self.keywords
    fn functions(&self) -> &SqlFunctions {
        &self.functions
//     fn type_to_sql(&self, sql_type: &crate::stdlib::packages::db_sql::SqlType) -> String {
        match sql_type {
//             crate::stdlib::packages::db_sql::SqlType::Boolean => "BOOLEAN".to_string(),
//             crate::stdlib::packages::db_sql::SqlType::Integer => "INT".to_string(),
//             crate::stdlib::packages::db_sql::SqlType::BigInt => "BIGINT".to_string(),
//             crate::stdlib::packages::db_sql::SqlType::Text => "TEXT".to_string(),
//             crate::stdlib::packages::db_sql::SqlType::Json => "JSON".to_string(),
//             crate::stdlib::packages::db_sql::SqlType::Timestamp => "DATETIME".to_string(),
//             crate::stdlib::packages::db_sql::SqlType::VarChar(len) => format!("VARCHAR({})", len),
        }
    }
impl SqliteDialect {
    /// slay Create new SQLite dialect
    pub fn new() -> Self {
        let features = DialectFeatures {
            supports_window_functions: true, // SQLite 3.25+
            supports_json: true, // SQLite 3.38+
            supports_upsert: true, // INSERT OR REPLACE, ON CONFLICT
            supports_fulltext: true, // FTS extension
            supports_spatial: false, // Requires extension

        let keywords = SqlKeywords {
            reserved_words: vec![
            function_names: vec![
            type_names: vec![

        let functions = SqlFunctions {
            string_functions: {
                let mut map = HashMap::new();
                map.insert("LENGTH".to_string(), "LENGTH".to_string());
                map.insert("SUBSTRING".to_string(), "SUBSTR".to_string());
                map
            datetime_functions: {
                let mut map = HashMap::new();
                map.insert("NOW".to_string(), "DATETIME('now')".to_string());
                map.insert("CURRENT_TIMESTAMP".to_string(), "CURRENT_TIMESTAMP".to_string());
                map
            json_functions: {
                let mut map = HashMap::new();
                map.insert("JSON_EXTRACT".to_string(), "JSON_EXTRACT".to_string());
                map.insert("JSON_OBJECT".to_string(), "JSON_OBJECT".to_string());
                map

        Self { features, keywords, functions }
    }
impl SqlDialectTrait for SqliteDialect {
    fn name(&self) -> &str {
        "SQLite"
    fn parameter_placeholder(&self, _index: usize) -> String {
        "?".to_string() // SQLite uses ? for all parameters
    fn named_parameter_placeholder(&self, name: &str) -> String {
        format!(":{}", name) // SQLite supports :name syntax
    fn quote_identifier(&self, identifier: &str) -> String {
        format!("\"{}\"", identifier.replace('"', "\"\""))
    fn quote_string(&self, value: &str) -> String {
        format!("'{}'", value.replace('\'', "''"))
    fn limit_clause(&self, limit: u64, offset: Option<u64>) -> String {
        if let Some(offset) = offset {
            format!("LIMIT {} OFFSET {}", limit, offset)
        } else {
            format!("LIMIT {}", limit)
        }
    }

    fn supported_features(&self) -> &DialectFeatures {
        &self.features
    fn keywords(&self) -> &SqlKeywords {
        &self.keywords
    fn functions(&self) -> &SqlFunctions {
        &self.functions
//     fn type_to_sql(&self, sql_type: &crate::stdlib::packages::db_sql::SqlType) -> String {
        match sql_type {
//             crate::stdlib::packages::db_sql::SqlType::Boolean => "INTEGER".to_string(), // SQLite stores boolean as integer
//             crate::stdlib::packages::db_sql::SqlType::Integer => "INTEGER".to_string(),
//             crate::stdlib::packages::db_sql::SqlType::BigInt => "INTEGER".to_string(),
//             crate::stdlib::packages::db_sql::SqlType::Float => "REAL".to_string(),
//             crate::stdlib::packages::db_sql::SqlType::Double => "REAL".to_string(),
//             crate::stdlib::packages::db_sql::SqlType::Text => "TEXT".to_string(),
//             crate::stdlib::packages::db_sql::SqlType::VarChar(_) => "TEXT".to_string(), // SQLite doesn't enforce varchar length
//             crate::stdlib::packages::db_sql::SqlType::Binary(_) => "BLOB".to_string(),
//             crate::stdlib::packages::db_sql::SqlType::Blob => "BLOB".to_string(),
//             crate::stdlib::packages::db_sql::SqlType::Timestamp => "TEXT".to_string(), // SQLite stores timestamps as text
        }
    }
