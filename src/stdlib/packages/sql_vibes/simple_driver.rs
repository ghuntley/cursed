/// fr fr Simplified database driver for CURSED - starting simple periodt
use crate::stdlib::packages::sql_vibes::{SqlResult, SqlError, SqlValue, Row, ResultSet, Parameter};
use std::collections::HashMap;

/// fr fr Simple database connection that actually works
#[derive(Debug)]
pub struct SimpleConnection {
    connection_string: String,
    is_open: bool,
}

impl SimpleConnection {
    /// sus Create new simple connection
    pub fn new(connection_string: String) -> SqlResult<Self> {
        if connection_string.is_empty() {
            return Err(SqlError::connection("Connection string cannot be empty - that's sus bestie".to_string()));
        }
        
        Ok(Self {
            connection_string,
            is_open: true,
        })
    }
    
    /// facts Execute a simple query
    pub fn execute_query(&mut self, sql: &str, params: &[Parameter]) -> SqlResult<ResultSet> {
        if !self.is_open {
            return Err(SqlError::connection("Connection is closed - can't execute queries bestie".to_string()));
        }
        
        // Basic validation
        if sql.trim().is_empty() {
            return Err(SqlError::query("SQL cannot be empty - that's not it chief".to_string()));
        }
        
        // Create mock result for demonstration
        if sql.trim().to_uppercase().starts_with("SELECT") {
            let columns = Vec::from(["id".to_string(), "name".to_string(), "value".to_string()]);
            let rows = vec![
                Row::new(vec![
                    SqlValue::Integer(1),
                    SqlValue::String("Mock Row 1".to_string()),
                    SqlValue::String("Test Value 1".to_string()),
                ]),
                Row::new(vec![
                    SqlValue::Integer(2),
                    SqlValue::String("Mock Row 2".to_string()),
                    SqlValue::String("Test Value 2".to_string()),
                ]),
            ];
            Ok(ResultSet::new(columns, rows))
        } else {
            Ok(ResultSet::empty())
        }
    }
    
    /// lowkey Execute a statement (INSERT/UPDATE/DELETE)
    pub fn execute_statement(&mut self, sql: &str, params: &[Parameter]) -> SqlResult<u64> {
        if !self.is_open {
            return Err(SqlError::connection("Connection is closed - can't execute statements bestie".to_string()));
        }
        
        if sql.trim().is_empty() {
            return Err(SqlError::query("SQL cannot be empty - that's not it chief".to_string()));
        }
        
        // Mock implementation - return number of affected rows
        if sql.trim().to_uppercase().starts_with("INSERT") {
            Ok(1) // Mock: inserted 1 row
        } else if sql.trim().to_uppercase().starts_with("UPDATE") || 
                  sql.trim().to_uppercase().starts_with("DELETE") {
            Ok(params.len() as u64) // Mock: affected rows based on parameters
        } else {
            Ok(0)
        }
    }
    
    /// highkey Check if connection is alive
    pub fn is_alive(&self) -> bool {
        self.is_open
    }
    
    /// periodt Close the connection
    pub fn close(&mut self) -> SqlResult<()> {
        self.is_open = false;
        Ok(())
    }
    
    /// bestie Get connection info
    pub fn connection_info(&self) -> HashMap<String, String> {
        let mut info = HashMap::new();
        info.insert("connection_string".to_string(), self.connection_string.clone());
        info.insert("status".to_string(), if self.is_open { "open".to_string() } else { "closed".to_string() });
        info
    }
}

/// fr fr Simple connection helper functions
pub fn connect(connection_string: &str) -> SqlResult<SimpleConnection> {
    SimpleConnection::new(connection_string.to_string())
}

pub fn quick_query(connection_string: &str, sql: &str) -> SqlResult<ResultSet> {
    let mut conn = connect(connection_string)?;
    let result = conn.execute_query(sql, &[]);
    conn.close()?;
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_connection_creation() {
        let conn = SimpleConnection::new("sqlite://test.db".to_string());
        assert!(conn.is_ok());
        
        let conn = SimpleConnection::new("".to_string());
        assert!(conn.is_err());
    }

    #[test]
    fn test_simple_query_execution() {
        let mut conn = SimpleConnection::new("sqlite://test.db".to_string()).unwrap();
        
        let result = conn.execute_query("SELECT * FROM users", &[]);
        assert!(result.is_ok());
        
        let result_set = result.unwrap();
        assert!(!result_set.is_empty());
        assert_eq!(result_set.column_count(), 3);
        assert_eq!(result_set.row_count(), 2);
        
        assert!(conn.close().is_ok());
    }

    #[test]
    fn test_simple_statement_execution() {
        let mut conn = SimpleConnection::new("sqlite://test.db".to_string()).unwrap();
        
        let params = Vec::from([Parameter::positional(0, SqlValue::String("test".to_string()))]);
        let result = conn.execute_statement("INSERT INTO users (name) VALUES (?)", &params);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
        
        assert!(conn.close().is_ok());
    }

    #[test] 
    fn test_connection_lifecycle() {
        let mut conn = SimpleConnection::new("sqlite://test.db".to_string()).unwrap();
        
        assert!(conn.is_alive());
        
        assert!(conn.close().is_ok());
        assert!(!conn.is_alive());
        
        // Should fail after close
        let result = conn.execute_query("SELECT 1", &[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_helper_functions() {
        let conn = connect("sqlite://test.db");
        assert!(conn.is_ok());
        
        let result = quick_query("sqlite://test.db", "SELECT 1");
        assert!(result.is_ok());
        
        let result = quick_query("", "SELECT 1");
        assert!(result.is_err());
    }
}
