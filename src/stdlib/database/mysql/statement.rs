/// fr fr MySQL prepared statement implementation
/// 
/// This module provides prepared statement functionality for MySQL connections,
/// supporting parameter binding, result handling, and proper resource management.

use std::sync::Arc;
use mysql::{Pool, PooledConn, Row, Statement};
use mysql::prelude::*;

use crate::stdlib::database::{
    DriverStmt, DatabaseError, SqlValue,
    driver::{QueryResult, ExecuteResult}
};
use super::error::{MySqlError, MySqlResult};
use super::types::{convert_from_sql_value, extract_value_by_index, get_column_info};
use super::driver::MySqlConfig;

/// fr fr MySQL prepared statement wrapper
#[derive(Debug)]
pub struct MySqlStatement {
    /// Connection pool for getting connections
    pool: Arc<Pool>,
    /// Original query string
    query: String,
    /// Statement configuration
    config: MySqlConfig,
    /// Cached parameter count
    parameter_count: usize,
}

impl MySqlStatement {
    /// Create a new MySQL prepared statement
    pub fn new(pool: Arc<Pool>, query: String, config: MySqlConfig) -> MySqlResult<Self> {
        // Count parameter placeholders in the query
        let parameter_count = query.matches('?').count();

        Ok(Self {
            pool,
            query,
            config,
            parameter_count,
        })
    }

    /// Get a connection from the pool
    fn get_connection(&self) -> MySqlResult<PooledConn> {
        self.pool.get_conn()
            .map_err(|e| MySqlError::pool_error(&format!("Failed to get connection: {}", e)))
    }

    /// Prepare the statement on a connection
    fn prepare_on_connection(&self, conn: &mut PooledConn) -> MySqlResult<Statement> {
        conn.prep(&self.query)
            .map_err(|e| MySqlError::query_error(&format!("Failed to prepare statement: {}", e), Some(&self.query)))
    }

    /// Execute the prepared statement and return query results
    fn execute_query_internal(&self, args: &[SqlValue]) -> MySqlResult<QueryResult> {
        // Validate parameter count
        if args.len() != self.parameter_count {
            return Err(MySqlError::query_error(
                &format!("Parameter count mismatch: expected {}, got {}", self.parameter_count, args.len()),
                Some(&self.query)
            ));
        }

        let mut conn = self.get_connection()?;
        
        // Convert CURSED SqlValues to MySQL Values
        let mysql_params: MySqlResult<Vec<mysql::Value>> = args.iter()
            .map(convert_from_sql_value)
            .collect();
        let mysql_params = mysql_params?;

        // Execute the prepared statement
        let rows: Vec<Row> = if mysql_params.is_empty() {
            // Query without parameters
            conn.query(&self.query)
                .map_err(|e| MySqlError::query_error(&format!("Statement execution failed: {}", e), Some(&self.query)))?
        } else {
            // Query with parameters
            conn.exec(&self.query, mysql_params)
                .map_err(|e| MySqlError::query_error(&format!("Prepared statement execution failed: {}", e), Some(&self.query)))?
        };

        // Convert result to QueryResult
        if rows.is_empty() {
            return Ok(QueryResult::new(Vec::new(), Vec::new(), Vec::new()));
        }

        // Get column information from first row
        let (column_names, column_types) = get_column_info(&rows[0]);

        // Convert all rows
        let mut result_rows = Vec::new();
        for row in rows {
            let mut row_values = Vec::new();
            for i in 0..column_names.len() {
                let value = extract_value_by_index(&row, i)?;
                row_values.push(value);
            }
            result_rows.push(row_values);
        }

        Ok(QueryResult::new(column_names, column_types, result_rows))
    }

    /// Execute the prepared statement and return execution results
    fn execute_command_internal(&self, args: &[SqlValue]) -> MySqlResult<ExecuteResult> {
        // Validate parameter count
        if args.len() != self.parameter_count {
            return Err(MySqlError::query_error(
                &format!("Parameter count mismatch: expected {}, got {}", self.parameter_count, args.len()),
                Some(&self.query)
            ));
        }

        let mut conn = self.get_connection()?;
        
        // Convert CURSED SqlValues to MySQL Values
        let mysql_params: MySqlResult<Vec<mysql::Value>> = args.iter()
            .map(convert_from_sql_value)
            .collect();
        let mysql_params = mysql_params?;

        // Execute the prepared statement
        if mysql_params.is_empty() {
            // Execute without parameters
            conn.query_drop(&self.query)
                .map_err(|e| MySqlError::query_error(&format!("Statement execution failed: {}", e), Some(&self.query)))?;
        } else {
            // Execute with parameters
            conn.exec_drop(&self.query, mysql_params)
                .map_err(|e| MySqlError::query_error(&format!("Prepared statement execution failed: {}", e), Some(&self.query)))?;
        }

        // Get execution statistics
        let affected_rows = conn.affected_rows() as i64;
        let last_insert_id = {
            let id = conn.last_insert_id();
            if id > 0 { Some(id as i64) } else { None }
        };

        Ok(ExecuteResult::new(last_insert_id, affected_rows))
    }

    /// Validate that the statement is properly formatted
    pub fn validate(&self) -> MySqlResult<()> {
        if self.query.trim().is_empty() {
            return Err(MySqlError::query_error("Query cannot be empty", Some(&self.query)));
        }

        // Basic SQL injection protection - check for suspicious patterns
        let query_lower = self.query.to_lowercase();
        if query_lower.contains(";--") || query_lower.contains("/*") || query_lower.contains("*/") {
            return Err(MySqlError::query_error("Potentially unsafe query detected", Some(&self.query)));
        }

        Ok(())
    }

    /// Get the query string for debugging
    pub fn query(&self) -> &str {
        &self.query
    }

    /// Get the parameter count
    pub fn param_count(&self) -> usize {
        self.parameter_count
    }
}

impl DriverStmt for MySqlStatement {
    fn query(&self, args: &[SqlValue]) -> Result<QueryResult, DatabaseError> {
        self.execute_query_internal(args)
            .map_err(|e| e.to_database_error())
    }

    fn execute(&self, args: &[SqlValue]) -> Result<ExecuteResult, DatabaseError> {
        self.execute_command_internal(args)
            .map_err(|e| e.to_database_error())
    }

    fn close(&self) -> Result<(), DatabaseError> {
        // MySQL prepared statements are automatically cleaned up when connections are returned to pool
        Ok(())
    }

    fn query_string(&self) -> &str {
        &self.query
    }

    fn parameter_count(&self) -> usize {
        self.parameter_count
    }

    fn clone(&self) -> Box<dyn DriverStmt> {
        Box::new(MySqlStatement {
            pool: Arc::clone(&self.pool),
            query: self.query.clone(),
            config: self.config.clone(),
            parameter_count: self.parameter_count,
        })
    }
}

impl Clone for MySqlStatement {
    fn clone(&self) -> Self {
        Self {
            pool: Arc::clone(&self.pool),
            query: self.query.clone(),
            config: self.config.clone(),
            parameter_count: self.parameter_count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stdlib::database::SqlValue;

    #[test]
    fn test_parameter_counting() {
        let pool = Arc::new(Pool::new("mysql://localhost/test").unwrap());
        let config = MySqlConfig::default();

        // Test query with no parameters
        let stmt1 = MySqlStatement::new(pool.clone(), "SELECT 1".to_string(), config.clone()).unwrap();
        assert_eq!(stmt1.parameter_count(), 0);

        // Test query with one parameter
        let stmt2 = MySqlStatement::new(pool.clone(), "SELECT * FROM users WHERE id = ?".to_string(), config.clone()).unwrap();
        assert_eq!(stmt2.parameter_count(), 1);

        // Test query with multiple parameters
        let stmt3 = MySqlStatement::new(pool.clone(), "UPDATE users SET name = ?, email = ? WHERE id = ?".to_string(), config.clone()).unwrap();
        assert_eq!(stmt3.parameter_count(), 3);
    }

    #[test]
    fn test_query_validation() {
        let pool = Arc::new(Pool::new("mysql://localhost/test").unwrap());
        let config = MySqlConfig::default();

        // Test valid query
        let stmt1 = MySqlStatement::new(pool.clone(), "SELECT * FROM users".to_string(), config.clone()).unwrap();
        assert!(stmt1.validate().is_ok());

        // Test empty query
        let stmt2 = MySqlStatement::new(pool.clone(), "".to_string(), config.clone()).unwrap();
        assert!(stmt2.validate().is_err());

        // Test potentially unsafe query
        let stmt3 = MySqlStatement::new(pool.clone(), "SELECT * FROM users; -- DROP TABLE users".to_string(), config.clone()).unwrap();
        assert!(stmt3.validate().is_err());
    }

    #[test]
    fn test_statement_properties() {
        let pool = Arc::new(Pool::new("mysql://localhost/test").unwrap());
        let config = MySqlConfig::default();
        let query = "SELECT * FROM users WHERE id = ?";

        let stmt = MySqlStatement::new(pool, query.to_string(), config).unwrap();
        
        assert_eq!(stmt.query_string(), query);
        assert_eq!(stmt.parameter_count(), 1);
    }

    #[test]
    fn test_statement_cloning() {
        let pool = Arc::new(Pool::new("mysql://localhost/test").unwrap());
        let config = MySqlConfig::default();
        let query = "SELECT * FROM users WHERE id = ?";

        let stmt1 = MySqlStatement::new(pool, query.to_string(), config).unwrap();
        let stmt2 = stmt1.clone();

        assert_eq!(stmt1.query_string(), stmt2.query_string());
        assert_eq!(stmt1.parameter_count(), stmt2.parameter_count());
    }

    #[test]
    fn test_boxed_statement_cloning() {
        let pool = Arc::new(Pool::new("mysql://localhost/test").unwrap());
        let config = MySqlConfig::default();
        let query = "SELECT * FROM users WHERE id = ?";

        let stmt: Box<dyn DriverStmt> = Box::new(MySqlStatement::new(pool, query.to_string(), config).unwrap());
        let cloned = stmt.clone();

        assert_eq!(stmt.query_string(), cloned.query_string());
        assert_eq!(stmt.parameter_count(), cloned.parameter_count());
    }
}
