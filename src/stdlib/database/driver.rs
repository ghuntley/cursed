/// fr fr Database driver interface and registry for SQLSlay
/// 
/// This module defines the driver interface that database implementations
/// must satisfy, along with a registry for managing multiple drivers.
/// 
/// Why a robust driver system is essential for database connectivity:
/// - Different databases have different connection protocols and features
/// - Driver abstraction allows switching databases without changing application code
/// - Registration system enables plugin-style database support
/// - Standard interface ensures consistent behavior across databases
/// - Type safety prevents runtime errors from driver mismatches

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::time::Duration;
use super::{DatabaseError, DatabaseErrorKind, SqlValue, TxOptions, VibeContext};
use crate::error::Error;

/// fr fr Result structure for query operations
#[derive(Debug, Clone)]
pub struct QueryResult {
    /// fr fr Column names in result set
    pub column_names: Vec<String>,
    /// fr fr Column types in result set
    pub column_types: Vec<String>,
    /// fr fr Rows of data
    pub rows: Vec<Vec<SqlValue>>,
    /// fr fr Error if query failed
    pub error: Option<DatabaseError>,
}

impl QueryResult {
    /// slay Create a new successful query result
    pub fn new(column_names: Vec<String>, column_types: Vec<String>, rows: Vec<Vec<SqlValue>>) -> Self {
        Self {
            column_names,
            column_types,
            rows,
            error: None,
        }
    }

    /// slay Create a new error result
    pub fn with_error(error: DatabaseError) -> Self {
        Self {
            column_names: Vec::from([]),
            column_types: Vec::from([]),
            rows: Vec::from([]),
            error: Some(error),
        }
    }
}

/// fr fr Result structure for execute operations
#[derive(Debug, Clone)]
pub struct ExecuteResult {
    /// fr fr Last inserted ID (if applicable)
    pub last_insert_id: Option<i64>,
    /// fr fr Number of rows affected
    pub rows_affected: i64,
}

impl ExecuteResult {
    /// slay Create a new execute result
    pub fn new(last_insert_id: Option<i64>, rows_affected: i64) -> Self {
        Self {
            last_insert_id,
            rows_affected,
        }
    }
}

/// fr fr Main driver interface that all database drivers must implement
pub trait Driver: Send + Sync + std::fmt::Debug {
    /// slay Open a new connection to the database
    fn open(&self, data_source_name: &str) -> Result<(), Error>;
    
    /// slay Get the name of this driver
    fn name(&self) -> &str;
    
    /// slay Get driver-specific capabilities
    fn capabilities(&self) -> DriverCapabilities;
    
    /// slay Clone this driver
    fn clone_driver(&self) -> Box<dyn Driver>;
}

/// fr fr Database connection interface for driver implementations
pub trait DriverConn: Send + Sync + std::fmt::Debug {
    /// slay Prepare a statement on this connection
    fn prepare(&self, query: &str) -> Result<(), Error>;
    
    /// slay Execute a query that returns rows
    fn query(&self, query: &str, args: &[SqlValue]) -> Result<(), Error>;
    
    /// slay Execute a query that doesn't return rows
    fn execute(&self, query: &str, args: &[SqlValue]) -> Result<(), Error>;
    
    /// slay Begin a transaction
    fn begin_transaction(&self, opts: TxOptions) -> Result<(), Error>;
    
    /// slay Ping the database to check connectivity
    fn ping(&self) -> Result<(), Error>;
    
    /// slay Close this connection
    fn close(&self) -> Result<(), Error>;
    
    /// slay Check if this connection is still alive
    fn is_alive(&self) -> bool;
    
    /// slay Get connection metadata
    fn metadata(&self) -> ConnectionMetadata;
    
    /// slay Clone this connection
    fn clone(&self) -> Box<dyn DriverConn>;
}

/// fr fr Prepared statement interface for driver implementations
pub trait DriverStmt: Send + Sync + std::fmt::Debug {
    /// slay Execute this statement with arguments that returns rows
    fn query(&self, args: &[SqlValue]) -> Result<(), Error>;
    
    /// slay Execute this statement with arguments that doesn't return rows
    fn execute(&self, args: &[SqlValue]) -> Result<(), Error>;
    
    /// slay Close this statement
    fn close(&self) -> Result<(), Error>;
    
    /// slay Get the original query string
    fn query_string(&self) -> &str;
    
    /// slay Get parameter count for this statement
    fn parameter_count(&self) -> usize;
    
    /// slay Get column count for this statement
    fn column_count(&self) -> usize;
    
    /// slay Clone this statement
    fn clone(&self) -> Box<dyn DriverStmt>;
}

/// fr fr Transaction interface for driver implementations
pub trait DriverTx: Send + Sync + std::fmt::Debug {
    /// slay Prepare a statement within this transaction
    fn prepare(&self, query: &str) -> Result<(), Error>;
    
    /// slay Execute a query that returns rows within this transaction
    fn query(&self, query: &str, args: &[SqlValue]) -> Result<(), Error>;
    
    /// slay Execute a query that doesn't return rows within this transaction
    fn execute(&self, query: &str, args: &[SqlValue]) -> Result<(), Error>;
    
    /// slay Commit this transaction
    fn commit(&self) -> Result<(), Error>;
    
    /// slay Rollback this transaction
    fn rollback(&self) -> Result<(), Error>;
    
    /// slay Get transaction options
    fn options(&self) -> &TxOptions;
    
    /// slay Check if transaction is still active
    fn is_active(&self) -> bool;
    
    /// slay Clone this transaction
    fn clone(&self) -> Box<dyn DriverTx>;
}

/// fr fr Driver capabilities for feature detection
#[derive(Debug, Clone)]
pub struct DriverCapabilities {
    /// fr fr Supports transactions
    pub supports_transactions: bool,
    /// fr fr Supports prepared statements
    pub supports_prepared_statements: bool,
    /// fr fr Supports multiple result sets
    pub supports_multiple_result_sets: bool,
    /// fr fr Supports stored procedures
    pub supports_stored_procedures: bool,
    /// fr fr Supports batch operations
    pub supports_batch_operations: bool,
    /// fr fr Supports concurrent connections
    pub supports_concurrent_connections: bool,
    /// fr fr Maximum number of connections
    pub max_connections: Option<usize>,
    /// fr fr Supported isolation levels
    pub supported_isolation_levels: Vec<super::SqlIsolationLevel>,
    /// fr fr Maximum query length
    pub max_query_length: Option<usize>,
    /// fr fr Maximum parameter count
    pub max_parameter_count: Option<usize>,
}

impl Default for DriverCapabilities {
    fn default() -> Self {
        Self {
            supports_transactions: true,
            supports_prepared_statements: true,
            supports_multiple_result_sets: false,
            supports_stored_procedures: false,
            supports_batch_operations: false,
            supports_concurrent_connections: true,
            max_connections: Some(100),
            supported_isolation_levels: vec![
                super::SqlIsolationLevel::LevelDefault,
                super::SqlIsolationLevel::LevelReadCommitted,
                super::SqlIsolationLevel::LevelSerializable,
            ],
            max_query_length: Some(1_000_000),
            max_parameter_count: Some(1000),
        }
    }
}

/// fr fr Connection metadata
#[derive(Debug, Clone)]
pub struct ConnectionMetadata {
    /// fr fr Database server version
    pub server_version: String,
    /// fr fr Database name
    pub database_name: String,
    /// fr fr Database server host
    pub server_host: String,
    /// fr fr Database server port
    pub server_port: u16,
    /// fr fr Connection username
    pub username: String,
    /// fr fr Connection established time
    pub connected_at: std::time::SystemTime,
    /// fr fr Additional metadata
    pub additional_info: HashMap<String, String>,
}

impl Default for ConnectionMetadata {
    fn default() -> Self {
        Self {
            server_version: "Unknown".to_string(),
            database_name: "Unknown".to_string(),
            server_host: "localhost".to_string(),
            server_port: 0,
            username: "Unknown".to_string(),
            connected_at: std::time::SystemTime::now(),
            additional_info: HashMap::new(),
        }
    }
}

/// fr fr Global driver registry for managing database drivers
#[derive(Debug)]
pub struct DriverRegistry {
    /// fr fr Registered drivers by name
    drivers: RwLock<HashMap<String, Box<dyn Driver>>>,
}

impl DriverRegistry {
    /// slay Create a new driver registry
    pub fn new() -> Self {
        Self {
            drivers: RwLock::new(HashMap::new()),
        }
    }

    /// slay Register a driver with the given name
    pub fn register(&self, name: String, driver: Box<dyn Driver>) -> Result<(), Error> {
        let mut drivers = self.drivers.write().map_err(|_| {
            DatabaseError::new(DatabaseErrorKind::DriverError, "Failed to acquire driver registry lock")
        })?;

        if drivers.contains_key(&name) {
            return Err(DatabaseError::new(
                DatabaseErrorKind::DriverError,
                &format!("Driver '{}' is already registered", name)
            ));
        }

        drivers.insert(name, driver);
        Ok(())
    }

    /// slay Unregister a driver
    pub fn unregister(&self, name: &str) -> Result<(), Error> {
        let mut drivers = self.drivers.write().map_err(|_| {
            DatabaseError::new(DatabaseErrorKind::DriverError, "Failed to acquire driver registry lock")
        })?;

        drivers.remove(name);
        Ok(())
    }

    /// slay Get a driver by name
    pub fn get(&self, name: &str) -> Result<(), Error> {
        let drivers = self.drivers.read().map_err(|_| {
            DatabaseError::new(DatabaseErrorKind::DriverError, "Failed to acquire driver registry lock")
        })?;

        drivers.get(name)
            .map(|driver| driver.clone_driver())
            .ok_or_else(|| DatabaseError::new(
                DatabaseErrorKind::DriverError,
                &format!("Driver '{}' not found", name)
            ))
    }

    /// slay List all registered driver names
    pub fn list_drivers(&self) -> Result<(), Error> {
        let drivers = self.drivers.read().map_err(|_| {
            DatabaseError::new(DatabaseErrorKind::DriverError, "Failed to acquire driver registry lock")
        })?;

        Ok(drivers.keys().cloned().collect())
    }

    /// slay Check if a driver is registered
    pub fn has_driver(&self, name: &str) -> Result<(), Error> {
        let drivers = self.drivers.read().map_err(|_| {
            DatabaseError::new(DatabaseErrorKind::DriverError, "Failed to acquire driver registry lock")
        })?;

        Ok(drivers.contains_key(name))
    }

    /// slay Get capabilities for a driver
    pub fn get_capabilities(&self, name: &str) -> Result<(), Error> {
        let driver = self.get(name)?;
        Ok(driver.capabilities())
    }
}

impl Default for DriverRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// fr fr Global driver registry instance
lazy_static::lazy_static! {
    pub static ref GLOBAL_DRIVER_REGISTRY: DriverRegistry = DriverRegistry::new();
}

/// slay Register a driver globally
pub fn register_driver(name: String, driver: Box<dyn Driver>) -> Result<(), Error> {
    GLOBAL_DRIVER_REGISTRY.register(name, driver)
}

/// slay Get a driver from the global registry
pub fn get_driver(name: &str) -> Result<(), Error> {
    GLOBAL_DRIVER_REGISTRY.get(name)
}

/// slay List all globally registered drivers
pub fn list_drivers() -> Result<(), Error> {
    GLOBAL_DRIVER_REGISTRY.list_drivers()
}

/// slay Check if a driver is globally registered
pub fn has_driver(name: &str) -> Result<(), Error> {
    GLOBAL_DRIVER_REGISTRY.has_driver(name)
}

/// slay Get capabilities for a globally registered driver
pub fn get_driver_capabilities(name: &str) -> Result<(), Error> {
    GLOBAL_DRIVER_REGISTRY.get_capabilities(name)
}

/// fr fr Mock driver for testing purposes
#[derive(Debug, Clone)]
pub struct MockDriver {
    /// fr fr Name of this mock driver
    pub name: String,
    /// fr fr Capabilities of this mock driver
    pub capabilities: DriverCapabilities,
}

impl MockDriver {
    /// slay Create a new mock driver
    pub fn new(name: String) -> Self {
        Self {
            name,
            capabilities: DriverCapabilities::default(),
        }
    }

    /// slay Create a mock driver with custom capabilities
    pub fn with_capabilities(name: String, capabilities: DriverCapabilities) -> Self {
        Self {
            name,
            capabilities,
        }
    }
}

impl Driver for MockDriver {
    fn open(&self, data_source_name: &str) -> Result<(), Error> {
        Ok(Box::new(MockDriverConn::new(data_source_name.to_string())))
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn capabilities(&self) -> DriverCapabilities {
        self.capabilities.clone()
    }

    fn clone_driver(&self) -> Box<dyn Driver> {
        Box::new(self.clone())
    }
}

/// fr fr Mock driver connection for testing
#[derive(Debug, Clone)]
pub struct MockDriverConn {
    /// fr fr Data source name
    pub data_source_name: String,
    /// fr fr Whether this connection is alive
    pub alive: bool,
    /// fr fr Connection metadata
    pub metadata: ConnectionMetadata,
}

impl MockDriverConn {
    /// slay Create a new mock connection
    pub fn new(data_source_name: String) -> Self {
        Self {
            data_source_name: data_source_name.clone(),
            alive: true,
            metadata: ConnectionMetadata {
                database_name: "mock_db".to_string(),
                server_host: "localhost".to_string(),
                server_port: 5432,
                username: "mock_user".to_string(),
                connected_at: std::time::SystemTime::now(),
                ..Default::default()
            },
        }
    }
}

impl DriverConn for MockDriverConn {
    fn prepare(&self, query: &str) -> Result<(), Error> {
        Ok(Box::new(MockDriverStmt::new(query.to_string())))
    }

    fn query(&self, query: &str, args: &[SqlValue]) -> Result<(), Error> {
        // Mock implementation returns empty result
        Ok(QueryResult::new(
            Vec::from(["column1".to_string(), "column2".to_string()]),
            Vec::from(["TEXT".to_string(), "INTEGER".to_string()]),
            Vec::from([]),
        ))
    }

    fn execute(&self, query: &str, args: &[SqlValue]) -> Result<(), Error> {
        // Mock implementation returns 1 row affected
        Ok(ExecuteResult::new(Some(1), 1))
    }

    fn begin_transaction(&self, opts: TxOptions) -> Result<(), Error> {
        Ok(Box::new(MockDriverTx::new(opts)))
    }

    fn ping(&self) -> Result<(), Error> {
        if self.alive {
            Ok(())
        } else {
            Err(DatabaseError::connection_error("Mock connection is not alive"))
        }
    }

    fn close(&self) -> Result<(), Error> {
        Ok(())
    }

    fn is_alive(&self) -> bool {
        self.alive
    }

    fn metadata(&self) -> ConnectionMetadata {
        self.metadata.clone()
    }

    fn clone(&self) -> Box<dyn DriverConn> {
        Box::new(Clone::clone(self))
    }
}

/// fr fr Mock driver statement for testing
#[derive(Debug, Clone)]
pub struct MockDriverStmt {
    /// fr fr Query string
    pub query: String,
}

impl MockDriverStmt {
    /// slay Create a new mock statement
    pub fn new(query: String) -> Self {
        Self { query }
    }
}

impl DriverStmt for MockDriverStmt {
    fn query(&self, args: &[SqlValue]) -> Result<(), Error> {
        Ok(QueryResult::new(
            vec!["column1".to_string()],
            vec!["TEXT".to_string()],
            vec![vec![SqlValue::String("mock_value".to_string())]],
        ))
    }

    fn execute(&self, args: &[SqlValue]) -> Result<(), Error> {
        Ok(ExecuteResult::new(Some(1), 1))
    }

    fn close(&self) -> Result<(), Error> {
        Ok(())
    }

    fn query_string(&self) -> &str {
        &self.query
    }

    fn parameter_count(&self) -> usize {
        // Count ? placeholders in query (simplified)
        self.query.matches('?').count()
    }

    fn clone(&self) -> Box<dyn DriverStmt> {
        Box::new(Clone::clone(self))
    }
}

/// fr fr Mock driver transaction for testing
#[derive(Debug, Clone)]
pub struct MockDriverTx {
    /// fr fr Transaction options
    pub options: TxOptions,
    /// fr fr Whether transaction is active
    pub active: bool,
}

impl MockDriverTx {
    /// slay Create a new mock transaction
    pub fn new(options: TxOptions) -> Self {
        Self {
            options,
            active: true,
        }
    }
}

impl DriverTx for MockDriverTx {
    fn prepare(&self, query: &str) -> Result<(), Error> {
        Ok(Box::new(MockDriverStmt::new(query.to_string())))
    }

    fn query(&self, query: &str, args: &[SqlValue]) -> Result<(), Error> {
        if !self.active {
            return Err(DatabaseError::transaction_error("Transaction is not active"));
        }
        Ok(QueryResult::new(
            Vec::from(["column1".to_string()]),
            Vec::from(["TEXT".to_string()]),
            Vec::from([]),
        ))
    }

    fn execute(&self, query: &str, args: &[SqlValue]) -> Result<(), Error> {
        if !self.active {
            return Err(DatabaseError::transaction_error("Transaction is not active"));
        }
        Ok(ExecuteResult::new(None, 1))
    }

    fn commit(&self) -> Result<(), Error> {
        if !self.active {
            return Err(DatabaseError::transaction_error("Transaction is not active"));
        }
        Ok(())
    }

    fn rollback(&self) -> Result<(), Error> {
        if !self.active {
            return Err(DatabaseError::transaction_error("Transaction is not active"));
        }
        Ok(())
    }

    fn options(&self) -> &TxOptions {
        &self.options
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn clone(&self) -> Box<dyn DriverTx> {
        Box::new(Clone::clone(self))
    }
}

/// fr fr Helper function to initialize common drivers
pub fn init_common_drivers() -> Result<(), Error> {
    // Register mock driver for testing
    register_driver("mock".to_string(), Box::new(MockDriver::new("mock".to_string())))?;
    
    // In a real implementation, we would register actual drivers like:
    // register_driver("mysql".to_string(), Box::new(MySQLDriver::new()))?;
    // register_driver("postgres".to_string(), Box::new(PostgreSQLDriver::new()))?;
    // register_driver("sqlite".to_string(), Box::new(SQLiteDriver::new()))?;
    
    Ok(())
}
