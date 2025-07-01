//! SQL injection prevention and secure database operations

use crate::error::CursedError;
use std::collections::HashMap;
use std::fmt;

pub type DatabaseResult<T> = Result<T, CursedError>;

/// SQL parameter types with validation
#[derive(Debug, Clone)]
pub enum SqlParameter {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Null,
    Blob(Vec<u8>),
}

impl SqlParameter {
    /// Validate parameter based on type constraints
    pub fn validate(&self) -> DatabaseResult<()> {
        match self {
            SqlParameter::String(s) => {
                // Prevent extremely long strings
                if s.len() > 1_000_000 {
                    return Err(CursedError::runtime_error("String parameter too long"));
                }
                Ok(())
            }
            SqlParameter::Blob(b) => {
                // Prevent extremely large blobs
                if b.len() > 10_000_000 {
                    return Err(CursedError::runtime_error("Blob parameter too large"));
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    /// Get parameter as SQL-safe string for logging (not execution)
    pub fn safe_display(&self) -> String {
        match self {
            SqlParameter::String(s) => {
                if s.len() > 50 {
                    format!("\"{}...\" (length: {})", &s[..47], s.len())
                } else {
                    format!("\"{}\"", s)
                }
            }
            SqlParameter::Integer(i) => i.to_string(),
            SqlParameter::Float(f) => f.to_string(),
            SqlParameter::Boolean(b) => b.to_string(),
            SqlParameter::Null => "NULL".to_string(),
            SqlParameter::Blob(b) => format!("<blob {} bytes>", b.len()),
        }
    }
}

/// Prepared statement builder with SQL injection prevention
pub struct PreparedStatement {
    sql: String,
    parameter_count: usize,
    validated: bool,
}

impl PreparedStatement {
    /// Create a new prepared statement
    pub fn new(sql: &str) -> DatabaseResult<Self> {
        let mut stmt = Self {
            sql: sql.to_string(),
            parameter_count: 0,
            validated: false,
        };
        
        stmt.validate_sql()?;
        Ok(stmt)
    }

    /// Validate SQL statement for safety
    fn validate_sql(&mut self) -> DatabaseResult<()> {
        // Count parameter placeholders
        self.parameter_count = self.sql.matches('?').count();
        
        // Basic SQL injection checks
        self.check_dangerous_patterns()?;
        self.check_parameter_consistency()?;
        
        self.validated = true;
        Ok(())
    }

    /// Check for dangerous SQL patterns
    fn check_dangerous_patterns(&self) -> DatabaseResult<()> {
        let sql_lower = self.sql.to_lowercase();
        
        // Forbidden patterns that could indicate injection
        let dangerous_patterns = [
            "; drop ",
            "; delete ",
            "; truncate ",
            "union select",
            "' or '1'='1",
            "\" or \"1\"=\"1",
            "' or 1=1",
            "\" or 1=1",
            "--",
            "/*",
            "*/",
            "xp_cmdshell",
            "sp_executesql",
        ];

        for pattern in &dangerous_patterns {
            if sql_lower.contains(pattern) {
                return Err(CursedError::runtime_error(&format!(
                    "Potentially dangerous SQL pattern detected: {}", pattern
                )));
            }
        }

        Ok(())
    }

    /// Check parameter placeholder consistency
    fn check_parameter_consistency(&self) -> DatabaseResult<()> {
        // Ensure all quotes are properly paired
        let mut in_single_quote = false;
        let mut in_double_quote = false;
        let mut escape_next = false;

        for ch in self.sql.chars() {
            if escape_next {
                escape_next = false;
                continue;
            }

            match ch {
                '\\' => escape_next = true,
                '\'' if !in_double_quote => in_single_quote = !in_single_quote,
                '"' if !in_single_quote => in_double_quote = !in_double_quote,
                _ => {}
            }
        }

        if in_single_quote || in_double_quote {
            return Err(CursedError::runtime_error("Unmatched quotes in SQL"));
        }

        Ok(())
    }

    /// Execute with parameters (prevents SQL injection)
    pub fn execute(&self, params: &[SqlParameter]) -> DatabaseResult<QueryResult> {
        if !self.validated {
            return Err(CursedError::runtime_error("Statement not validated"));
        }

        if params.len() != self.parameter_count {
            return Err(CursedError::runtime_error(&format!(
                "Parameter count mismatch: expected {}, got {}",
                self.parameter_count,
                params.len()
            )));
        }

        // Validate all parameters
        for param in params {
            param.validate()?;
        }

        // Log query for debugging (safe display only)
        self.log_query_execution(params)?;

        // In a real implementation, this would use actual database drivers
        // with proper parameter binding (e.g., rusqlite, tokio-postgres)
        Ok(QueryResult {
            rows_affected: 1,
            columns: vec!["id".to_string(), "name".to_string()],
            rows: vec![],
        })
    }

    /// Safe query logging
    fn log_query_execution(&self, params: &[SqlParameter]) -> DatabaseResult<()> {
        let param_displays: Vec<String> = params.iter()
            .map(|p| p.safe_display())
            .collect();

        println!("🔒 SECURE QUERY: {} | PARAMS: [{}]", 
            self.sql, 
            param_displays.join(", ")
        );
        Ok(())
    }
}

/// Query result container
pub struct QueryResult {
    pub rows_affected: u64,
    pub columns: Vec<String>,
    pub rows: Vec<HashMap<String, SqlParameter>>,
}

/// Query builder with injection prevention
pub struct SecureQueryBuilder {
    query_type: QueryType,
    table: Option<String>,
    columns: Vec<String>,
    conditions: Vec<WhereCondition>,
    parameters: Vec<SqlParameter>,
    validated_table_names: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum QueryType {
    Select,
    Insert,
    Update,
    Delete,
}

/// Safe WHERE condition with parameterization
#[derive(Debug, Clone)]
pub struct WhereCondition {
    column: String,
    operator: ComparisonOperator,
    parameter_index: usize,
}

#[derive(Debug, Clone)]
pub enum ComparisonOperator {
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Like,
    In,
    IsNull,
    IsNotNull,
}

impl ComparisonOperator {
    fn to_sql(&self) -> &'static str {
        match self {
            ComparisonOperator::Equal => "=",
            ComparisonOperator::NotEqual => "!=",
            ComparisonOperator::LessThan => "<",
            ComparisonOperator::LessThanOrEqual => "<=",
            ComparisonOperator::GreaterThan => ">",
            ComparisonOperator::GreaterThanOrEqual => ">=",
            ComparisonOperator::Like => "LIKE",
            ComparisonOperator::In => "IN",
            ComparisonOperator::IsNull => "IS NULL",
            ComparisonOperator::IsNotNull => "IS NOT NULL",
        }
    }
}

impl SecureQueryBuilder {
    /// Create new query builder with table name validation
    pub fn new(allowed_tables: Vec<String>) -> Self {
        Self {
            query_type: QueryType::Select,
            table: None,
            columns: Vec::new(),
            conditions: Vec::new(),
            parameters: Vec::new(),
            validated_table_names: allowed_tables,
        }
    }

    /// Set query type to SELECT
    pub fn select(mut self) -> Self {
        self.query_type = QueryType::Select;
        self
    }

    /// Set table name (validated against allowlist)
    pub fn from(mut self, table: &str) -> DatabaseResult<Self> {
        // Validate table name
        if !self.validated_table_names.contains(&table.to_string()) {
            return Err(CursedError::runtime_error(&format!(
                "Table '{}' not in allowed list", table
            )));
        }

        // Additional table name validation
        if !Self::is_valid_identifier(table) {
            return Err(CursedError::runtime_error(&format!(
                "Invalid table name: {}", table
            )));
        }

        self.table = Some(table.to_string());
        Ok(self)
    }

    /// Add columns to SELECT (validated)
    pub fn columns(mut self, columns: &[&str]) -> DatabaseResult<Self> {
        for column in columns {
            if !Self::is_valid_identifier(column) {
                return Err(CursedError::runtime_error(&format!(
                    "Invalid column name: {}", column
                )));
            }
            self.columns.push(column.to_string());
        }
        Ok(self)
    }

    /// Add WHERE condition with parameter
    pub fn where_eq(mut self, column: &str, value: SqlParameter) -> DatabaseResult<Self> {
        if !Self::is_valid_identifier(column) {
            return Err(CursedError::runtime_error(&format!(
                "Invalid column name: {}", column
            )));
        }

        value.validate()?;

        let condition = WhereCondition {
            column: column.to_string(),
            operator: ComparisonOperator::Equal,
            parameter_index: self.parameters.len(),
        };

        self.conditions.push(condition);
        self.parameters.push(value);
        Ok(self)
    }

    /// Validate SQL identifier (table/column names)
    fn is_valid_identifier(name: &str) -> bool {
        if name.is_empty() || name.len() > 64 {
            return false;
        }

        // Must start with letter or underscore
        let first_char = name.chars().next().unwrap();
        if !first_char.is_ascii_alphabetic() && first_char != '_' {
            return false;
        }

        // Must contain only alphanumeric characters and underscores
        name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
    }

    /// Build prepared statement
    pub fn build(self) -> DatabaseResult<PreparedStatement> {
        let table = self.table.ok_or_else(|| {
            CursedError::runtime_error("Table name required")
        })?;

        let mut sql = match self.query_type {
            QueryType::Select => {
                let columns = if self.columns.is_empty() {
                    "*".to_string()
                } else {
                    self.columns.join(", ")
                };
                format!("SELECT {} FROM {}", columns, table)
            }
            _ => return Err(CursedError::runtime_error("Query type not yet implemented")),
        };

        // Add WHERE conditions
        if !self.conditions.is_empty() {
            sql.push_str(" WHERE ");
            let condition_strs: Vec<String> = self.conditions.iter()
                .map(|c| format!("{} {} ?", c.column, c.operator.to_sql()))
                .collect();
            sql.push_str(&condition_strs.join(" AND "));
        }

        PreparedStatement::new(&sql)
    }

    /// Execute the built query
    pub fn execute(self) -> DatabaseResult<QueryResult> {
        let stmt = self.build()?;
        stmt.execute(&self.parameters)
    }
}

/// Database connection with security controls
pub struct SecureConnection {
    allowed_tables: Vec<String>,
    max_query_time: std::time::Duration,
    query_count: std::sync::atomic::AtomicU64,
}

impl SecureConnection {
    pub fn new(allowed_tables: Vec<String>) -> Self {
        Self {
            allowed_tables,
            max_query_time: std::time::Duration::from_secs(30),
            query_count: std::sync::atomic::AtomicU64::new(0),
        }
    }

    /// Create secure query builder
    pub fn query_builder(&self) -> SecureQueryBuilder {
        SecureQueryBuilder::new(self.allowed_tables.clone())
    }

    /// Execute prepared statement with rate limiting
    pub fn execute_prepared(&self, stmt: &PreparedStatement, params: &[SqlParameter]) -> DatabaseResult<QueryResult> {
        // Rate limiting check
        let current_count = self.query_count.load(std::sync::atomic::Ordering::SeqCst);
        if current_count > 1000 {
            return Err(CursedError::runtime_error("Query rate limit exceeded"));
        }

        self.query_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        // Execute with timeout
        let start = std::time::Instant::now();
        let result = stmt.execute(params);
        
        if start.elapsed() > self.max_query_time {
            return Err(CursedError::runtime_error("Query timeout exceeded"));
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sql_injection_prevention() {
        let malicious_input = "'; DROP TABLE users; --";
        let param = SqlParameter::String(malicious_input.to_string());
        
        // This should be safe when used as a parameter
        let stmt = PreparedStatement::new("SELECT * FROM users WHERE name = ?").unwrap();
        let result = stmt.execute(&[param]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_dangerous_sql_detection() {
        let dangerous_sql = "SELECT * FROM users; DROP TABLE users; --";
        let result = PreparedStatement::new(dangerous_sql);
        assert!(result.is_err());
    }

    #[test]
    fn test_secure_query_builder() {
        let allowed_tables = vec!["users".to_string(), "posts".to_string()];
        let conn = SecureConnection::new(allowed_tables);
        
        let result = conn.query_builder()
            .select()
            .from("users").unwrap()
            .columns(&["id", "name"]).unwrap()
            .where_eq("id", SqlParameter::Integer(1)).unwrap()
            .execute();
            
        assert!(result.is_ok());
    }

    #[test]
    fn test_table_name_validation() {
        let allowed_tables = vec!["users".to_string()];
        let conn = SecureConnection::new(allowed_tables);
        
        // Should reject non-allowed table
        let result = conn.query_builder()
            .select()
            .from("evil_table");
        assert!(result.is_err());
        
        // Should reject invalid identifiers
        let result = conn.query_builder()
            .select()
            .from("users; DROP TABLE");
        assert!(result.is_err());
    }
}
