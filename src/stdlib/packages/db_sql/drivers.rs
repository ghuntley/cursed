/// fr fr SQL driver interfaces - the contracts for SQL database drivers periodt
///
/// This module defines the core interfaces that all SQL drivers must implement.
/// Think of it as the rules that make SQL drivers work together bestie!

use crate::stdlib::packages::db_core::{
    DatabaseDriver, DatabaseConnection, DatabaseResult as DbResult,
    ConnectionConfig, DriverInfo, DriverFeature, DatabaseError
};
use crate::stdlib::packages::db_sql::{
    SqlDialect, SqlValue, SqlType, PreparedStatement, SqlResultSet,
    SqlExecuteResult, SqlConnection, SqlTransaction
};
use async_trait::async_trait;
use std::collections::HashMap;

/// fr fr Core SQL driver trait - what every SQL driver needs to implement
#[async_trait]
pub trait SqlDriver: DatabaseDriver + Send + Sync {
    /// slay Create a SQL-specific connection
    async fn sql_connect(&self, config: ConnectionConfig) -> DbResult<Box<dyn SqlConnection>>;
    
    /// slay Get the SQL dialect this driver uses
    fn sql_dialect(&self) -> Box<dyn SqlDialect>;
    
    /// slay Get supported SQL types
    fn supported_types(&self) -> Vec<SqlType>;
    
    /// slay Check if driver supports a specific SQL feature
    fn supports_sql_feature(&self, feature: SqlFeature) -> bool;
    
    /// slay Get driver-specific configuration options
    fn configuration_options(&self) -> Vec<ConfigurationOption>;
    
    /// slay Validate SQL syntax (basic check)
    fn validate_sql(&self, sql: &str) -> DbResult<()>;
    
    /// slay Get driver performance characteristics
    fn performance_info(&self) -> DriverPerformanceInfo;
    
    /// slay Get driver limitations
    fn limitations(&self) -> DriverLimitations;
}

/// fr fr SQL-specific connection interface
#[async_trait]
pub trait SqlConnection: DatabaseConnection + Send + Sync {
    /// slay Execute SQL query with SQL-specific result
    async fn sql_query(&mut self, sql: &str, params: &[SqlValue]) -> DbResult<SqlResultSet>;
    
    /// slay Execute SQL statement with SQL-specific result
    async fn sql_execute(&mut self, sql: &str, params: &[SqlValue]) -> DbResult<SqlExecuteResult>;
    
    /// slay Prepare a SQL statement
    async fn sql_prepare(&mut self, sql: &str) -> DbResult<Box<dyn PreparedStatement>>;
    
    /// slay Begin SQL transaction with options
    async fn sql_begin_transaction(&mut self, isolation: Option<SqlTransactionIsolation>) -> DbResult<Box<dyn SqlTransaction>>;
    
    /// slay Execute multiple SQL statements in batch
    async fn sql_batch(&mut self, statements: &[SqlBatch]) -> DbResult<Vec<SqlExecuteResult>>;
    
    /// slay Get SQL connection metadata
    fn sql_connection_info(&self) -> SqlConnectionInfo;
    
    /// slay Set connection-level SQL variables
    async fn set_sql_variable(&mut self, name: &str, value: &SqlValue) -> DbResult<()>;
    
    /// slay Get connection-level SQL variable
    async fn get_sql_variable(&mut self, name: &str) -> DbResult<SqlValue>;
}

/// fr fr SQL transaction interface
#[async_trait]
pub trait SqlTransaction: Send + Sync {
    /// slay Execute query within transaction
    async fn sql_query(&mut self, sql: &str, params: &[SqlValue]) -> DbResult<SqlResultSet>;
    
    /// slay Execute statement within transaction
    async fn sql_execute(&mut self, sql: &str, params: &[SqlValue]) -> DbResult<SqlExecuteResult>;
    
    /// slay Commit the transaction
    async fn commit(self: Box<Self>) -> DbResult<()>;
    
    /// slay Rollback the transaction
    async fn rollback(self: Box<Self>) -> DbResult<()>;
    
    /// slay Create savepoint
    async fn savepoint(&mut self, name: &str) -> DbResult<SqlSavepoint>;
    
    /// slay Rollback to savepoint
    async fn rollback_to_savepoint(&mut self, savepoint: &SqlSavepoint) -> DbResult<()>;
    
    /// slay Get transaction isolation level
    fn isolation_level(&self) -> SqlTransactionIsolation;
    
    /// slay Check if transaction is read-only
    fn is_read_only(&self) -> bool;
}

/// fr fr SQL driver manager for coordinating multiple drivers
#[derive(Debug)]
pub struct SqlDriverManager {
    drivers: HashMap<String, Box<dyn SqlDriver>>,
    default_driver: Option<String>,
}

impl SqlDriverManager {
    /// slay Create a new SQL driver manager
    pub fn new() -> Self {
        Self {
            drivers: HashMap::new(),
            default_driver: None,
        }
    }

    /// slay Register a SQL driver
    pub fn register_driver(&mut self, name: String, driver: Box<dyn SqlDriver>) {
        self.drivers.insert(name.clone(), driver);
        
        // Set as default if it's the first driver
        if self.default_driver.is_none() {
            self.default_driver = Some(name);
        }
    }

    /// slay Get a driver by name
    pub fn get_driver(&self, name: &str) -> Option<&dyn SqlDriver> {
        self.drivers.get(name).map(|d| d.as_ref())
    }

    /// slay Get the default driver
    pub fn get_default_driver(&self) -> Option<&dyn SqlDriver> {
        self.default_driver.as_ref()
            .and_then(|name| self.get_driver(name))
    }

    /// slay Set the default driver
    pub fn set_default_driver(&mut self, name: String) -> DbResult<()> {
        if self.drivers.contains_key(&name) {
            self.default_driver = Some(name);
            Ok(())
        } else {
            Err(DatabaseError::driver(&format!("Driver '{}' not found", name)))
        }
    }

    /// slay List all registered drivers
    pub fn list_drivers(&self) -> Vec<String> {
        self.drivers.keys().cloned().collect()
    }

    /// slay Create a connection using the default driver
    pub async fn connect_default(&self, config: ConnectionConfig) -> DbResult<Box<dyn SqlConnection>> {
        let driver = self.get_default_driver()
            .ok_or_else(|| DatabaseError::driver("No default driver set"))?;
        driver.sql_connect(config).await
    }

    /// slay Create a connection using a specific driver
    pub async fn connect_with_driver(&self, driver_name: &str, config: ConnectionConfig) -> DbResult<Box<dyn SqlConnection>> {
        let driver = self.get_driver(driver_name)
            .ok_or_else(|| DatabaseError::driver(&format!("Driver '{}' not found", driver_name)))?;
        driver.sql_connect(config).await
    }
}

/// fr fr SQL features that drivers may support
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SqlFeature {
    /// Window functions
    WindowFunctions,
    /// Common Table Expressions (CTEs)
    CommonTableExpressions,
    /// Recursive queries
    RecursiveQueries,
    /// JSON data type and functions
    JsonSupport,
    /// Array data types
    ArraySupport,
    /// Full-text search
    FullTextSearch,
    /// Stored procedures
    StoredProcedures,
    /// User-defined functions
    UserDefinedFunctions,
    /// Triggers
    Triggers,
    /// Views
    Views,
    /// Materialized views
    MaterializedViews,
    /// Partitioning
    Partitioning,
    /// Foreign keys
    ForeignKeys,
    /// Check constraints
    CheckConstraints,
    /// Unique constraints
    UniqueConstraints,
    /// Indexes
    Indexes,
    /// Partial indexes
    PartialIndexes,
    /// Expression indexes
    ExpressionIndexes,
    /// GeoSpatial data types
    GeoSpatialSupport,
    /// UUID data type
    UuidSupport,
    /// Binary data types
    BinarySupport,
    /// Large objects
    LargeObjects,
    /// Streaming results
    StreamingResults,
    /// Bulk operations
    BulkOperations,
    /// Asynchronous execution
    AsyncExecution,
}

/// fr fr SQL transaction isolation levels
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SqlTransactionIsolation {
    /// Read uncommitted
    ReadUncommitted,
    /// Read committed
    ReadCommitted,
    /// Repeatable read
    RepeatableRead,
    /// Serializable
    Serializable,
}

/// fr fr SQL batch operation
#[derive(Debug, Clone)]
pub struct SqlBatch {
    /// SQL statement
    pub sql: String,
    /// Parameters for the statement
    pub parameters: Vec<SqlValue>,
    /// Whether to continue on error
    pub continue_on_error: bool,
}

/// fr fr SQL connection information
#[derive(Debug, Clone)]
pub struct SqlConnectionInfo {
    /// Database server version
    pub server_version: String,
    /// Protocol version
    pub protocol_version: String,
    /// Current database name
    pub database_name: String,
    /// Current schema name
    pub schema_name: Option<String>,
    /// Connection character set
    pub character_set: String,
    /// Connection collation
    pub collation: String,
    /// Time zone
    pub time_zone: String,
    /// Auto-commit status
    pub auto_commit: bool,
    /// Read-only status
    pub read_only: bool,
    /// Transaction isolation level
    pub isolation_level: SqlTransactionIsolation,
    /// Server capabilities
    pub capabilities: Vec<String>,
}

/// fr fr SQL savepoint
#[derive(Debug, Clone)]
pub struct SqlSavepoint {
    /// Savepoint name
    pub name: String,
    /// Unique identifier
    pub id: String,
    /// Creation timestamp
    pub created_at: std::time::SystemTime,
}

/// fr fr Configuration option for SQL drivers
#[derive(Debug, Clone)]
pub struct ConfigurationOption {
    /// Option name
    pub name: String,
    /// Option description
    pub description: String,
    /// Option type
    pub option_type: ConfigurationOptionType,
    /// Default value
    pub default_value: Option<String>,
    /// Whether option is required
    pub required: bool,
    /// Valid values (for enum types)
    pub valid_values: Option<Vec<String>>,
}

/// fr fr Configuration option types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigurationOptionType {
    String,
    Integer,
    Boolean,
    Duration,
    Enum,
    Path,
    Url,
}

/// fr fr Driver performance characteristics
#[derive(Debug, Clone)]
pub struct DriverPerformanceInfo {
    /// Typical connection establishment time
    pub connection_time: std::time::Duration,
    /// Typical query execution overhead
    pub query_overhead: std::time::Duration,
    /// Maximum concurrent connections supported
    pub max_connections: Option<usize>,
    /// Supports connection pooling
    pub connection_pooling: bool,
    /// Supports prepared statement caching
    pub statement_caching: bool,
    /// Supports batch operations
    pub batch_operations: bool,
    /// Supports streaming results
    pub streaming_results: bool,
}

/// fr fr Driver limitations
#[derive(Debug, Clone)]
pub struct DriverLimitations {
    /// Maximum SQL statement length
    pub max_statement_length: Option<usize>,
    /// Maximum number of parameters
    pub max_parameters: Option<usize>,
    /// Maximum identifier length
    pub max_identifier_length: Option<usize>,
    /// Maximum string/text length
    pub max_string_length: Option<usize>,
    /// Maximum numeric precision
    pub max_numeric_precision: Option<u32>,
    /// Maximum number of columns in result set
    pub max_columns: Option<usize>,
    /// Maximum number of rows in result set
    pub max_rows: Option<usize>,
    /// Unsupported SQL features
    pub unsupported_features: Vec<SqlFeature>,
}

/// slay Create a SQL driver based on driver name
pub fn create_sql_driver(driver_name: &str) -> DbResult<Box<dyn SqlDriver>> {
    match driver_name {
        "postgresql" | "postgres" => {
            Ok(Box::new(crate::stdlib::packages::db_sql::postgresql::PostgreSqlDriver::new()))
        }
        "mysql" => {
            Ok(Box::new(crate::stdlib::packages::db_sql::mysql::MySqlDriver::new()))
        }
        "sqlite" | "sqlite3" => {
            Ok(Box::new(crate::stdlib::packages::db_sql::sqlite::SqliteDriver::new()))
        }
        _ => {
            Err(DatabaseError::driver(&format!("Unknown SQL driver: {}", driver_name)))
        }
    }
}

impl SqlBatch {
    /// slay Create a new SQL batch operation
    pub fn new(sql: &str) -> Self {
        Self {
            sql: sql.to_string(),
            parameters: Vec::new(),
            continue_on_error: false,
        }
    }

    /// slay Add parameters to the batch
    pub fn with_parameters(mut self, params: Vec<SqlValue>) -> Self {
        self.parameters = params;
        self
    }

    /// slay Set continue on error flag
    pub fn continue_on_error(mut self, continue_on_error: bool) -> Self {
        self.continue_on_error = continue_on_error;
        self
    }
}

impl SqlSavepoint {
    /// slay Create a new savepoint
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            id: uuid::Uuid::new_v4().to_string(),
            created_at: std::time::SystemTime::now(),
        }
    }
}

impl ConfigurationOption {
    /// slay Create a new configuration option
    pub fn new(name: &str, description: &str, option_type: ConfigurationOptionType) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            option_type,
            default_value: None,
            required: false,
            valid_values: None,
        }
    }

    /// slay Set default value
    pub fn with_default(mut self, default: &str) -> Self {
        self.default_value = Some(default.to_string());
        self
    }

    /// slay Set as required
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    /// slay Set valid values for enum type
    pub fn with_valid_values(mut self, values: Vec<String>) -> Self {
        self.valid_values = Some(values);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sql_driver_manager() {
        let mut manager = SqlDriverManager::new();
        assert!(manager.list_drivers().is_empty());
        assert!(manager.get_default_driver().is_none());
    }

    #[test]
    fn test_sql_batch() {
        let batch = SqlBatch::new("INSERT INTO users (name) VALUES (?)")
            .with_parameters(vec![SqlValue::Text("Alice".to_string())])
            .continue_on_error(true);

        assert_eq!(batch.sql, "INSERT INTO users (name) VALUES (?)");
        assert_eq!(batch.parameters.len(), 1);
        assert!(batch.continue_on_error);
    }

    #[test]
    fn test_sql_savepoint() {
        let savepoint = SqlSavepoint::new("test_point");
        assert_eq!(savepoint.name, "test_point");
        assert!(!savepoint.id.is_empty());
    }

    #[test]
    fn test_configuration_option() {
        let option = ConfigurationOption::new(
            "timeout",
            "Connection timeout in seconds",
            ConfigurationOptionType::Integer
        )
        .with_default("30")
        .required();

        assert_eq!(option.name, "timeout");
        assert_eq!(option.option_type, ConfigurationOptionType::Integer);
        assert_eq!(option.default_value, Some("30".to_string()));
        assert!(option.required);
    }

    #[test]
    fn test_sql_features() {
        assert_eq!(SqlFeature::WindowFunctions, SqlFeature::WindowFunctions);
        assert_ne!(SqlFeature::WindowFunctions, SqlFeature::JsonSupport);
    }

    #[test]
    fn test_create_sql_driver() {
        // These will fail until we implement the actual drivers
        // but the function should exist and handle unknown drivers
        assert!(create_sql_driver("unknown").is_err());
    }
}
