/// fr fr MySQL connection implementation with connection pooling
/// 
/// This module provides the MySQL connection implementation that integrates
/// with the CURSED database driver interface, providing real MySQL connectivity
/// with proper connection pooling, error handling, and transaction support.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;
use mysql::{Pool, PooledConn, Row};
use mysql::prelude::*;

use crate::stdlib::database::{
    DriverConn, DatabaseError, SqlValue, TxOptions,
    driver::{QueryResult, ExecuteResult, ConnectionMetadata, DriverStmt, DriverTx}
};
use super::error::{MySqlError, MySqlResult};
use super::crate::types::{convert_from_sql_value, extract_value_by_index, get_column_info};
use super::driver::MySqlConfig;
use super::statement::MySqlStatement;
use super::transaction::MySqlTransaction;

/// fr fr MySQL connection using connection pool
#[derive(Debug)]
pub struct MySqlConnection {
    /// Connection pool for managing MySQL connections
    pool: Arc<Pool>,
    /// Data source name (connection string)
    dsn: String,
    /// Connection configuration
    config: MySqlConfig,
    /// Unique connection identifier
    connection_id: String,
    /// Timestamp when connection was created
    connected_at: SystemTime,
    /// Connection metadata cache
    metadata_cache: Option<ConnectionMetadata>,
}

impl MySqlConnection {
    /// Create a new MySQL connection with pool
    pub fn new(pool: Arc<Pool>, dsn: String, config: MySqlConfig) -> MySqlResult<Self> {
        let connection_id = uuid::Uuid::new_v4().to_string();
        let connected_at = SystemTime::now();

        Ok(Self {
            pool,
            dsn,
            config,
            connection_id,
            connected_at,
            metadata_cache: None,
        })
    }

    /// Get a connection from the pool
    fn get_pooled_connection(&self) -> MySqlResult<PooledConn> {
        self.pool.get_conn()
            .map_err(|e| MySqlError::pool_error(&format!("Failed to get connection from pool: {}", e)))
    }

    /// Execute a query and convert the result to QueryResult
    fn execute_query_internal(&self, query: &str, args: &[SqlValue]) -> MySqlResult<QueryResult> {
        let mut conn = self.get_pooled_connection()?;

        // Convert CURSED SqlValues to MySQL Values
        let mysql_params: MySqlResult<Vec<mysql::Value>> = args.iter()
            .map(convert_from_sql_value)
            .collect();
        let mysql_params = mysql_params?;

        // Execute the query
        let rows: Vec<Row> = if mysql_params.is_empty() {
            // Simple query without parameters
            conn.query(query)
                .map_err(|e| MySqlError::query_error(&format!("Query execution failed: {}", e), Some(query)))?
        } else {
            // Prepared statement with parameters
            conn.exec(query, mysql_params)
                .map_err(|e| MySqlError::query_error(&format!("Prepared query execution failed: {}", e), Some(query)))?
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

    /// Execute a command and return the execution result
    fn execute_command_internal(&self, query: &str, args: &[SqlValue]) -> MySqlResult<ExecuteResult> {
        let mut conn = self.get_pooled_connection()?;

        // Convert CURSED SqlValues to MySQL Values
        let mysql_params: MySqlResult<Vec<mysql::Value>> = args.iter()
            .map(convert_from_sql_value)
            .collect();
        let mysql_params = mysql_params?;

        // Execute the command
        if mysql_params.is_empty() {
            // Simple execution without parameters
            conn.query_drop(query)
                .map_err(|e| MySqlError::query_error(&format!("Command execution failed: {}", e), Some(query)))?;
        } else {
            // Prepared statement execution
            conn.exec_drop(query, mysql_params)
                .map_err(|e| MySqlError::query_error(&format!("Prepared command execution failed: {}", e), Some(query)))?;
        }

        // Get execution statistics
        let affected_rows = conn.affected_rows() as i64;
        let last_insert_id = {
            let id = conn.last_insert_id();
            if id > 0 { Some(id as i64) } else { None }
        };

        Ok(ExecuteResult::new(last_insert_id, affected_rows))
    }

    /// Build connection metadata
    fn build_metadata(&self) -> MySqlResult<ConnectionMetadata> {
        let mut conn = self.get_pooled_connection()?;

        // Get server version
        let version_query = "SELECT VERSION()";
        let version_result: Option<String> = conn.query_first(version_query)
            .map_err(|e| MySqlError::query_error(&format!("Failed to get server version: {}", e), Some(version_query)))?;
        let server_version = version_result.unwrap_or_else(|| "Unknown".to_string());

        // Parse DSN for metadata
        let conn_info = super::crate::types::parse_connection_string(&self.dsn)
            .unwrap_or_else(|_| super::crate::types::MySqlConnectionInfo {
                host: "localhost".to_string(),
                port: 3306,
                user: "unknown".to_string(),
                password: String::new(),
                database: "unknown".to_string(),
            });

        // Build additional info
        let mut additional_info = HashMap::new();
        additional_info.insert("driver_version".to_string(), "24.0".to_string());
        additional_info.insert("charset".to_string(), self.config.charset.clone());
        additional_info.insert("ssl_enabled".to_string(), self.config.ssl_enabled.to_string());
        additional_info.insert("compression".to_string(), self.config.compression.to_string());
        
        if let Some(ref timezone) = self.config.timezone {
            additional_info.insert("timezone".to_string(), timezone.clone());
        }

        Ok(ConnectionMetadata {
            server_version,
            database_name: conn_info.database,
            server_host: conn_info.host,
            server_port: conn_info.port,
            username: conn_info.user,
            connected_at: self.connected_at,
            additional_info,
        })
    }
}

impl DriverConn for MySqlConnection {
    fn prepare(&self, query: &str) -> Result<(), Error> {
        let statement = MySqlStatement::new(
            Arc::clone(&self.pool),
            query.to_string(),
            self.config.clone()
        ).map_err(|e| e.to_database_error())?;

        Ok(Box::new(statement))
    }

    fn query(&self, query: &str, args: &[SqlValue]) -> Result<(), Error> {
        self.execute_query_internal(query, args)
            .map_err(|e| e.to_database_error())
    }

    fn execute(&self, query: &str, args: &[SqlValue]) -> Result<(), Error> {
        self.execute_command_internal(query, args)
            .map_err(|e| e.to_database_error())
    }

    fn begin_transaction(&self, opts: TxOptions) -> Result<(), Error> {
        let transaction = MySqlTransaction::new(
            Arc::clone(&self.pool),
            opts,
            self.config.clone()
        ).map_err(|e| e.to_database_error())?;

        Ok(Box::new(transaction))
    }

    fn ping(&self) -> Result<(), Error> {
        let mut conn = self.get_pooled_connection()
            .map_err(|e| e.to_database_error())?;

        conn.query_drop("SELECT 1")
            .map_err(|e| DatabaseError::connection_error(&format!("Ping failed: {}", e)))
    }

    fn close(&self) -> Result<(), Error> {
        // Connection pool handles cleanup automatically
        // Individual connections are returned to the pool when dropped
        Ok(())
    }

    fn is_alive(&self) -> bool {
        self.ping().is_ok()
    }

    fn metadata(&self) -> ConnectionMetadata {
        // Try to use cached metadata first
        if let Some(ref cached) = self.metadata_cache {
            return cached.clone();
        }

        // Build metadata if not cached
        match self.build_metadata() {
            Ok(metadata) => {
                // Cache would require mutable reference, so we don't cache here
                // In a real implementation, we might use Arc<Mutex<Option<ConnectionMetadata>>>
                metadata
            }
            Err(_) => {
                // Return default metadata on error
                ConnectionMetadata {
                    server_version: "Unknown".to_string(),
                    database_name: "Unknown".to_string(),
                    server_host: "localhost".to_string(),
                    server_port: 3306,
                    username: "Unknown".to_string(),
                    connected_at: self.connected_at,
                    additional_info: HashMap::new(),
                }
            }
        }
    }

    fn clone(&self) -> Box<dyn DriverConn> {
        Box::new(MySqlConnection {
            pool: Arc::clone(&self.pool),
            dsn: self.dsn.clone(),
            config: self.config.clone(),
            connection_id: uuid::Uuid::new_v4().to_string(),
            connected_at: SystemTime::now(),
            metadata_cache: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stdlib::database::SqlIsolationLevel;

    #[test]
    fn test_connection_creation() {
        // This test would require a real MySQL instance
        // For now, we just test the structure
        let config = MySqlConfig::default();
        
        // We can't test actual connection without MySQL server
        // But we can test configuration and structure
        assert_eq!(config.max_connections, 100);
        assert_eq!(config.charset, "utf8mb4");
        assert!(!config.ssl_enabled);
    }

    #[test]
    fn test_connection_metadata_structure() {
        let config = MySqlConfig::default();
        let metadata = ConnectionMetadata {
            server_version: "8.0.33".to_string(),
            database_name: "test".to_string(),
            server_host: "localhost".to_string(),
            server_port: 3306,
            username: "testuser".to_string(),
            connected_at: SystemTime::now(),
            additional_info: HashMap::new(),
        };

        assert_eq!(metadata.server_version, "8.0.33");
        assert_eq!(metadata.database_name, "test");
        assert_eq!(metadata.server_port, 3306);
    }

    #[test]
    fn test_transaction_options() {
        let opts = TxOptions {
            isolation: SqlIsolationLevel::LevelReadCommitted,
            read_only: false,
        };

        assert_eq!(opts.isolation, SqlIsolationLevel::LevelReadCommitted);
        assert!(!opts.read_only);
    }
}
