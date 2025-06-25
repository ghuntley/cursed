/// fr fr Core database traits - the contracts that keep everything together periodt
///
/// These traits define the fundamental interfaces that all database drivers
/// and components must implement. Think of them as the rules of the game bestie!

// use crate::stdlib::packages::db_core::{
    Row, Column, ColumnType, Parameter
};
// use crate::stdlib::packages::db_core::result::{
    ExecuteResult, QueryStats, ResultMetadata
};
// use crate::stdlib::packages::db_core::query::{
    Query, QueryPlan as ImportedQueryPlan, ExecutionStep as ImportedExecutionStep
};
// use crate::stdlib::packages::db_core::transaction::{
    TransactionOptions, TransactionState, TransactionIsolation
};

// Export SavePoint for external use (TransactionState already imported above)
// pub use crate::stdlib::packages::db_core::transaction::{SavePoint as TransactionSavePoint};
// use crate::stdlib::packages::db_core::connection::{
    ConnectionConfig, ConnectionInfo as ImportedConnectionInfo
};
use crate::error::CursedError;
// use crate::stdlib::packages::db_core::error::{DatabaseResult as DbResult};

use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;

/// fr fr Core database driver trait - the big boss interface periodt
#[async_trait]
pub trait DatabaseDriver: std::fmt::Debug + Send + Sync {
    /// slay Create a new database connection
    async fn connect(&self, config: ConnectionConfig) -> DbResult<Box<dyn DatabaseConnection>>;
    
    /// slay Get driver information
    fn driver_info(&self) -> DriverInfo;
    
    /// slay Check if the driver supports a feature
    fn supports_feature(&self, feature: DriverFeature) -> bool;
    
    /// slay Get supported SQL dialect
    fn sql_dialect(&self) -> SqlDialect;
    
    /// slay Validate connection string format
    fn validate_connection_string(&self, connection_string: &str) -> DbResult<()>;
}

/// fr fr Database connection trait - handles actual database interactions
#[async_trait]
pub trait DatabaseConnection: std::fmt::Debug + Send + Sync {
    /// slay Execute a query and return results
    async fn query(&mut self, sql: &str, parameters: &[Parameter]) -> DbResult<Box<dyn ResultSet>>;
    
    /// slay Execute a statement that doesn't return data
    async fn execute(&mut self, sql: &str, parameters: &[Parameter]) -> DbResult<ExecuteResult>;
    
    /// slay Prepare a statement for multiple executions
    async fn prepare(&mut self, sql: &str) -> DbResult<Box<dyn PreparedStatement>>;
    
    /// slay Begin a new transaction
    async fn begin_transaction(&mut self, options: Option<TransactionOptions>) -> DbResult<Box<dyn DatabaseTransaction>>;
    
    /// slay Check if connection is still alive
    async fn ping(&mut self) -> DbResult<()>;
    
    /// slay Close the connection
    async fn close(self: Box<Self>) -> DbResult<()>;
    
    /// slay Get connection metadata
    fn connection_info(&self) -> ImportedConnectionInfo;
}

/// fr fr Transaction management trait
#[async_trait]
pub trait DatabaseTransaction: std::fmt::Debug + Send + Sync {
    /// slay Commit the transaction
    async fn commit(self: Box<Self>) -> DbResult<()>;
    
    /// slay Rollback the transaction
    async fn rollback(self: Box<Self>) -> DbResult<()>;
    
    /// slay Create a savepoint
    async fn savepoint(&mut self, name: &str) -> DbResult<SavePoint>;
    
    /// slay Rollback to a savepoint
    async fn rollback_to_savepoint(&mut self, savepoint: &SavePoint) -> DbResult<()>;
    
    /// slay Execute query within transaction
    async fn query(&mut self, sql: &str, parameters: &[Parameter]) -> DbResult<Box<dyn ResultSet>>;
    
    /// slay Execute statement within transaction
    async fn execute(&mut self, sql: &str, parameters: &[Parameter]) -> DbResult<ExecuteResult>;
    
    /// slay Get transaction state
    fn state(&self) -> TransactionState;
}

/// fr fr Query executor trait for advanced query handling
#[async_trait]
pub trait QueryExecutor: std::fmt::Debug + Send + Sync {
    /// slay Execute a query plan
    async fn execute_plan(&mut self, plan: ImportedQueryPlan) -> DbResult<Box<dyn ResultSet>>;
    
    /// slay Execute batch of queries
    async fn execute_batch(&mut self, queries: Vec<Query>) -> DbResult<Vec<ExecuteResult>>;
    
    /// slay Stream large result sets
    async fn stream_query(&mut self, sql: &str, parameters: &[Parameter]) -> DbResult<Box<dyn QueryStream>>;
    
    /// slay Get query execution statistics
    async fn query_stats(&self, query_id: &str) -> DbResult<QueryStats>;
}

/// fr fr Result set trait for handling query results
pub trait ResultSet: std::fmt::Debug + Send + Sync {
    /// slay Get the next row
    fn next(&mut self) -> DbResult<Option<Row>>;
    
    /// slay Get all remaining rows
    fn collect(&mut self) -> DbResult<Vec<Row>>;
    
    /// slay Get column metadata
    fn columns(&self) -> &[Column];
    
    /// slay Get result metadata
    fn metadata(&self) -> &ResultMetadata;
    
    /// slay Check if there are more rows
    fn has_next(&self) -> bool;
    
    /// slay Get row count (if available)
    fn row_count(&self) -> Option<usize>;
    
    /// slay Check if result set is empty
    fn is_empty(&self) -> bool {
        self.row_count().unwrap_or(0) == 0
    }
}

/// fr fr Prepared statement trait for efficient repeated execution
#[async_trait]
pub trait PreparedStatement: std::fmt::Debug + Send + Sync {
    /// slay Execute with parameters
    async fn execute(&mut self, parameters: &[Parameter]) -> DbResult<ExecuteResult>;
    
    /// slay Query with parameters
    async fn query(&mut self, parameters: &[Parameter]) -> DbResult<Box<dyn ResultSet>>;
    
    /// slay Get parameter metadata
    fn parameter_metadata(&self) -> &[ParameterMetadata];
    
    /// slay Get result set metadata
    fn result_metadata(&self) -> &ResultMetadata;
    
    /// slay Close the prepared statement
    async fn close(self: Box<Self>) -> DbResult<()>;
}

/// fr fr Connection manager trait for pooling
#[async_trait]
pub trait ConnectionManager: std::fmt::Debug + Send + Sync {
    /// slay Get a connection from the pool
    async fn get_connection(&self) -> DbResult<Box<dyn DatabaseConnection>>;
    
    /// slay Return a connection to the pool
    async fn return_connection(&self, connection: Box<dyn DatabaseConnection>) -> DbResult<()>;
    
    /// slay Get pool statistics
    fn pool_stats(&self) -> PoolStats;
    
    /// slay Close all connections
    async fn close_all(&self) -> DbResult<()>;
}

/// fr fr Query stream trait for streaming large results
#[async_trait]
pub trait QueryStream: std::fmt::Debug + Send + Sync {
    /// slay Get the next row asynchronously
    async fn next_row(&mut self) -> DbResult<Option<Row>>;
    
    /// slay Get metadata for the stream
    fn metadata(&self) -> &ResultMetadata;
    
    /// slay Close the stream
    async fn close(self: Box<Self>) -> DbResult<()>;
}

/// fr fr Driver information
#[derive(Debug, Clone)]
pub struct DriverInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub vendor: String,
    pub supported_features: Vec<DriverFeature>,
}

/// fr fr Connection information
#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    pub database_name: String,
    pub server_version: String,
    pub protocol_version: String,
    pub connection_id: String,
    pub is_read_only: bool,
    pub transaction_isolation: TransactionIsolation,
}

/// fr fr Savepoint for transaction management
#[derive(Debug, Clone)]
pub struct SavePoint {
    pub name: String,
    pub id: String,
    pub created_at: std::time::SystemTime,
}

/// fr fr Parameter metadata for prepared statements
#[derive(Debug, Clone)]
pub struct ParameterMetadata {
    pub position: usize,
    pub name: Option<String>,
    pub data_type: ColumnType,
    pub is_nullable: bool,
    pub precision: Option<u32>,
    pub scale: Option<u32>,
}

/// fr fr Pool statistics
#[derive(Debug, Clone)]
pub struct PoolStats {
    pub total_connections: usize,
    pub active_connections: usize,
    pub idle_connections: usize,
    pub pending_requests: usize,
    pub max_connections: usize,
}

/// fr fr Query plan for advanced execution
#[derive(Debug, Clone)]
pub struct QueryPlan {
    pub query: Query,
    pub estimated_cost: f64,
    pub estimated_rows: Option<usize>,
    pub execution_steps: Vec<ImportedExecutionStep>,
}

/// fr fr Execution step in a query plan
#[derive(Debug, Clone)]
pub struct ExecutionStep {
    pub step_type: String,
    pub table_name: Option<String>,
    pub index_name: Option<String>,
    pub estimated_cost: f64,
    pub estimated_rows: Option<usize>,
}

/// fr fr Supported driver features
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DriverFeature {
    Transactions,
    PreparedStatements,
    BatchExecution,
    Streaming,
    ConnectionPooling,
    AsyncExecution,
    StoredProcedures,
    BinaryData,
    LargeObjects,
    FullTextSearch,
    JsonSupport,
    UuidSupport,
    ArraySupport,
    GeoSpatialSupport,
}

/// fr fr SQL dialects
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SqlDialect {
    Ansi,
    PostgreSQL,
    MySQL,
    SQLite,
    SQLServer,
    Oracle,
    H2,
    Custom(String),
}

// Transaction types moved to transaction module to avoid conflicts

impl DriverInfo {
    /// slay Create new driver info
    pub fn new(name: &str, version: &str, description: &str, vendor: &str) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            description: description.to_string(),
            vendor: vendor.to_string(),
            supported_features: Vec::new(),
        }
    }
    
    /// slay Add a supported feature
    pub fn with_feature(mut self, feature: DriverFeature) -> Self {
        self.supported_features.push(feature);
        self
    }
}

impl ImportedConnectionInfo {
    /// slay Create new connection info
    pub fn new(database_name: &str, server_version: &str) -> Self {
        Self {
            database_name: database_name.to_string(),
            server_version: server_version.to_string(),
            protocol_version: "1.0".to_string(),
            connection_id: uuid::Uuid::new_v4().to_string(),
            is_read_only: false,
            transaction_isolation: TransactionIsolation::ReadCommitted,
        }
    }
}

impl SavePoint {
    /// slay Create a new savepoint
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            id: uuid::Uuid::new_v4().to_string(),
            created_at: std::time::SystemTime::now(),
        }
    }
}

