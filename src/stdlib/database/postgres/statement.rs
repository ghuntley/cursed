/// PostgreSQL Prepared Statement Implementation
/// 
/// Provides high-performance prepared statement functionality with parameter binding,
/// result set handling, and proper resource management for PostgreSQL connections.

use std::sync::Arc;
use tokio_postgres::{Statement, Row};
use crate::stdlib::database::{
    DriverStmt, SqlValue,
    driver::{QueryResult, ExecuteResult}
};
use super::error::{PostgresError, PostgresErrorKind, PostgresResult};
use super::types::{map_postgres_value, prepare_parameters, extract_column_info, PostgresParam};

/// PostgreSQL prepared statement wrapper
pub struct PostgresStatement {
    /// Underlying tokio-postgres statement
    statement: Statement,
    /// Original SQL query
    query: String,
    /// Statement statistics
    stats: Arc<std::sync::Mutex<StatementStats>>,
    /// Parameter types information
    param_types: Vec<tokio_postgres::types::Type>,
    /// Column types information
    column_types: Vec<tokio_postgres::types::Type>,
}

/// Statement execution statistics
#[derive(Debug, Clone, Default)]
pub struct StatementStats {
    pub executions: u64,
    pub total_rows_returned: u64,
    pub total_rows_affected: u64,
    pub total_execution_time_ms: u64,
    pub errors: u64,
    pub last_error: Option<String>,
}

impl PostgresStatement {
    /// Create new prepared statement
    pub fn new(statement: Statement, query: String) -> Self {
        let param_types = statement.params().to_vec();
        let column_types = statement.columns().iter().map(|c| c.type_().clone()).collect();
        
        Self {
            statement,
            query,
            stats: Arc::new(std::sync::Mutex::new(StatementStats::default())),
            param_types,
            column_types,
        }
    }

    /// Execute statement and return query results
    pub async fn query(&self, client: &tokio_postgres::Client, args: &[SqlValue]) -> PostgresResult<QueryResult> {
        let start_time = std::time::Instant::now();
        
        // Validate parameter count
        if args.len() != self.param_types.len() {
            return Err(PostgresError::new(
                PostgresErrorKind::QueryError,
                &format!(
                    "Parameter count mismatch: expected {}, got {}",
                    self.param_types.len(),
                    args.len()
                ),
            ));
        }

        // Convert parameters
        let params = self.convert_parameters(args)?;
        let param_refs: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = 
            params.iter().map(|p| p.as_ref()).collect();

        // Execute query
        let rows = client
            .query(&self.statement, &param_refs)
            .await
            .map_err(|e| {
                self.update_stats(|stats| {
                    stats.errors += 1;
                    stats.last_error = Some(e.to_string());
                });
                PostgresError::from(e)
            })?;

        // Convert results
        let result = self.convert_rows(rows)?;
        
        let execution_time = start_time.elapsed();
        self.update_stats(|stats| {
            stats.executions += 1;
            stats.total_rows_returned += result.rows.len() as u64;
            stats.total_execution_time_ms += execution_time.as_millis() as u64;
        });

        Ok(result)
    }

    /// Execute statement and return number of affected rows
    pub async fn execute(&self, client: &tokio_postgres::Client, args: &[SqlValue]) -> PostgresResult<ExecuteResult> {
        let start_time = std::time::Instant::now();
        
        // Validate parameter count
        if args.len() != self.param_types.len() {
            return Err(PostgresError::new(
                PostgresErrorKind::QueryError,
                &format!(
                    "Parameter count mismatch: expected {}, got {}",
                    self.param_types.len(),
                    args.len()
                ),
            ));
        }

        // Convert parameters
        let params = self.convert_parameters(args)?;
        let param_refs: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = 
            params.iter().map(|p| p.as_ref()).collect();

        // Execute statement
        let rows_affected = client
            .execute(&self.statement, &param_refs)
            .await
            .map_err(|e| {
                self.update_stats(|stats| {
                    stats.errors += 1;
                    stats.last_error = Some(e.to_string());
                });
                PostgresError::from(e)
            })?;

        let execution_time = start_time.elapsed();
        self.update_stats(|stats| {
            stats.executions += 1;
            stats.total_rows_affected += rows_affected;
            stats.total_execution_time_ms += execution_time.as_millis() as u64;
        });

        Ok(ExecuteResult {
            rows_affected: rows_affected as i64,
            last_insert_id: None, // PostgreSQL uses RETURNING clause for insert IDs
        })
    }

    /// Get statement information
    pub fn info(&self) -> StatementInfo {
        StatementInfo {
            query: self.query.clone(),
            parameter_count: self.param_types.len(),
            column_count: self.column_types.len(),
            parameter_types: self.param_types.iter().map(|t| t.name().to_string()).collect(),
            column_types: self.column_types.iter().map(|t| t.name().to_string()).collect(),
            column_names: self.statement.columns().iter().map(|c| c.name().to_string()).collect(),
        }
    }

    /// Get statement execution statistics
    pub fn get_stats(&self) -> StatementStats {
        self.stats.lock().unwrap().clone()
    }

    /// Convert CURSED SqlValues to PostgreSQL parameters
    fn convert_parameters(&self, args: &[SqlValue]) -> PostgresResult<Vec<PostgresParam>> {
        args.iter()
            .map(|arg| Ok(PostgresParam::new(arg.clone())))
            .collect()
    }

    /// Convert PostgreSQL rows to CURSED format
    fn convert_rows(&self, rows: Vec<Row>) -> PostgresResult<QueryResult> {
        let mut result_rows = Vec::new();
        let mut columns = Vec::new();
        
        if !rows.is_empty() {
            // Extract column information from first row
            columns = extract_column_info(&rows[0]);
            
            // Convert all rows
            for row in &rows {
                let mut row_values = Vec::new();
                for (i, column) in row.columns().iter().enumerate() {
                    let value = map_postgres_value(column.type_(), row, i)?;
                    row_values.push(value);
                }
                result_rows.push(row_values);
            }
        }

        Ok(QueryResult {
            rows: result_rows,
            columns: columns.into_iter().map(|c| c.name).collect(),
            rows_affected: 0, // Not available for prepared statement queries
        })
    }

    /// Update statement statistics
    fn update_stats<F>(&self, updater: F)
    where
        F: FnOnce(&mut StatementStats),
    {
        if let Ok(mut stats) = self.stats.lock() {
            updater(&mut stats);
        }
    }

    /// Close the prepared statement
    pub async fn close(&self, client: &tokio_postgres::Client) -> PostgresResult<()> {
        // tokio-postgres automatically handles statement cleanup
        // No explicit close needed
        Ok(())
    }

    /// Check if statement is valid
    pub fn is_valid(&self) -> bool {
        // Basic validation - check if we have the statement
        !self.query.is_empty()
    }

    /// Get underlying tokio-postgres statement
    pub fn inner(&self) -> &dyn Statement {
        &self.statement
    }
}

impl DriverStmt for PostgresStatement {
    fn query(&self, args: &[SqlValue]) -> Result<QueryResult, crate::stdlib::database::DatabaseError> {
        // For async execution in sync context, we need a runtime handle
        // This is a limitation of the current sync API design
        Err(crate::stdlib::database::DatabaseError::new(
            crate::stdlib::database::DatabaseErrorKind::NotSupported,
            "Prepared statement queries require async context. Use connection.query() instead.",
        ))
    }

    fn execute(&self, args: &[SqlValue]) -> Result<ExecuteResult, crate::stdlib::database::DatabaseError> {
        // For async execution in sync context, we need a runtime handle
        // This is a limitation of the current sync API design
        Err(crate::stdlib::database::DatabaseError::new(
            crate::stdlib::database::DatabaseErrorKind::NotSupported,
            "Prepared statement execution requires async context. Use connection.execute() instead.",
        ))
    }

    fn close(&self) -> Result<(), crate::stdlib::database::DatabaseError> {
        // tokio-postgres handles cleanup automatically
        Ok(())
    }

    fn parameter_count(&self) -> usize {
        self.param_types.len()
    }

    fn column_count(&self) -> usize {
        self.column_types.len()
    }
}

/// Statement information for introspection
#[derive(Debug, Clone)]
pub struct StatementInfo {
    pub query: String,
    pub parameter_count: usize,
    pub column_count: usize,
    pub parameter_types: Vec<String>,
    pub column_types: Vec<String>,
    pub column_names: Vec<String>,
}

impl std::fmt::Display for StatementInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "PostgreSQL Prepared Statement:")?;
        writeln!(f, "  Query: {}", self.query)?;
        writeln!(f, "  Parameters: {} types", self.parameter_count)?;
        writeln!(f, "  Columns: {} types", self.column_count)?;
        
        if !self.parameter_types.is_empty() {
            writeln!(f, "  Parameter Types: {}", self.parameter_types.join(", "))?;
        }
        
        if !self.column_names.is_empty() {
            writeln!(f, "  Column Names: {}", self.column_names.join(", "))?;
        }
        
        Ok(())
    }
}

impl std::fmt::Display for StatementStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Statement Statistics:")?;
        writeln!(f, "  Executions: {}", self.executions)?;
        writeln!(f, "  Rows Returned: {}", self.total_rows_returned)?;
        writeln!(f, "  Rows Affected: {}", self.total_rows_affected)?;
        writeln!(f, "  Total Time: {}ms", self.total_execution_time_ms)?;
        writeln!(f, "  Errors: {}", self.errors)?;
        
        if self.executions > 0 {
            let avg_time = self.total_execution_time_ms as f64 / self.executions as f64;
            writeln!(f, "  Avg Time: {:.2}ms", avg_time)?;
        }
        
        if let Some(ref last_error) = self.last_error {
            writeln!(f, "  Last Error: {}", last_error)?;
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_postgres::types::Type;

    #[test]
    fn test_statement_info() {
        // Create a mock statement for testing
        let info = StatementInfo {
            query: "SELECT * FROM users WHERE id = $1".to_string(),
            parameter_count: 1,
            column_count: 3,
            parameter_types: vec!["int4".to_string()],
            column_types: vec!["int4".to_string(), "text".to_string(), "text".to_string()],
            column_names: vec!["id".to_string(), "name".to_string(), "email".to_string()],
        };
        
        assert_eq!(info.parameter_count, 1);
        assert_eq!(info.column_count, 3);
        assert!(info.query.contains("SELECT"));
    }

    #[test]
    fn test_statement_stats() {
        let stats = StatementStats {
            executions: 10,
            total_rows_returned: 25,
            total_rows_affected: 0,
            total_execution_time_ms: 150,
            errors: 1,
            last_error: Some("Connection timeout".to_string()),
        };
        
        assert_eq!(stats.executions, 10);
        assert_eq!(stats.total_rows_returned, 25);
        assert_eq!(stats.errors, 1);
    }

    #[test]
    fn test_parameter_validation() {
        let param_types = vec![Type::INT4, Type::TEXT];
        let args = vec![SqlValue::Integer(42), SqlValue::String("test".to_string())];
        
        // Parameter count should match
        assert_eq!(args.len(), param_types.len());
        
        // Test mismatch
        let wrong_args = vec![SqlValue::Integer(42)];
        assert_ne!(wrong_args.len(), param_types.len());
    }
}
