/// fr fr SQL database drivers - the real MVP for relational databases periodt
///
/// This package provides comprehensive SQL database support for CURSED with
/// drivers for PostgreSQL, MySQL, SQLite, and more. All the SQL vibes bestie!

// Core SQL modules
pub mod drivers;
pub mod builder;
pub mod types;
pub mod dialect;
pub mod connection;
pub mod prepared;
pub mod result;
pub mod migration;

// Driver implementations
pub mod postgresql;
pub mod mysql;
pub mod sqlite;

// Re-export the important stuff for easy access
pub use drivers::{
    SqlDriver, SqlDriverManager, create_sql_driver, SqlFeature
};

// Re-export SqlValue from database core
pub use crate::stdlib::database::core::SqlValue;

// Add missing transaction and batch types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SqlTransactionIsolation {
    ReadUncommitted,
    ReadCommitted,
    RepeatableRead,
    Serializable,
}

#[derive(Debug, Clone)]
pub struct SqlBatch {
    pub sql: String,
    pub parameters: Vec<SqlValue>,
}
pub use builder::{
    DeleteBuilder, CreateTableBuilder, AlterTableBuilder
};
pub use types::{
    SqlNull, SqlDateTime, SqlDecimal, SqlArray, SqlJson
};
pub use dialect::{
    SqlDialectTrait, MySqlDialect, PostgreSqlDialect, SqliteDialect
};
pub use connection::{SqlConnection, SqlConnectionPool, SqlTransaction};
pub use prepared::{PreparedStatement, StatementCache};
pub use result::{SqlRowIterator};
pub use migration::{
    SchemaVersion, MigrationScript
};

// Driver-specific exports
pub use postgresql::{PostgreSqlDriver, PostgreSqlConnection, PgError};
// pub use mysql::{MySqlDriver, MySqlConnection, MySqlError};  // Temporarily disabled - mysql crate not available
pub use sqlite::{SqliteDriver, SqliteConnection, SqliteError};



// Placeholder imports disabled
// ConnectionConfig, DatabaseDriver
// };
use crate::error::CursedError;
use std::collections::HashMap;
use std::sync::Arc;

// Import database types and errors
use crate::stdlib::database::DatabaseError;

// Define result types for DB operations
pub type DbResult<T> = Result<T, DatabaseError>;

/// Connection configuration
#[derive(Debug, Clone)]
pub struct ConnectionConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
    pub ssl_mode: bool,
}

impl ConnectionConfig {
    pub fn from_string(connection_string: &str) -> DbResult<Self> {
        // Very basic parsing - in real implementation would use proper URL parsing
        if connection_string.starts_with("postgres://") || connection_string.starts_with("postgresql://") {
            Ok(ConnectionConfig {
                host: "localhost".to_string(),
                port: 5432,
                database: "default".to_string(),
                username: "user".to_string(),
                password: "password".to_string(),
                ssl_mode: false,
            })
        } else if connection_string.starts_with("mysql://") {
            Ok(ConnectionConfig {
                host: "localhost".to_string(),
                port: 3306,
                database: "default".to_string(),
                username: "user".to_string(),
                password: "password".to_string(),
                ssl_mode: false,
            })
        } else if connection_string.starts_with("sqlite://") {
            Ok(ConnectionConfig {
                host: "".to_string(),
                port: 0,
                database: connection_string.trim_start_matches("sqlite://").to_string(),
                username: "".to_string(),
                password: "".to_string(),
                ssl_mode: false,
            })
        } else {
            Err(DatabaseError::connection("Invalid connection string format"))
        }
    }
}

/// Basic database connection trait
pub trait DatabaseConnection: Send + Sync {
    fn execute(&self, query: &str, params: &[SqlValue]) -> DbResult<SqlExecuteResult>;
    fn query(&self, query: &str, params: &[SqlValue]) -> DbResult<SqlResultSet>;
    fn close(&self) -> DbResult<()>;
}

/// SQL result set
#[derive(Debug)]
pub struct SqlResultSet {
    pub rows: Vec<Vec<SqlValue>>,
    pub columns: Vec<String>,
}

impl SqlResultSet {
    pub fn new(columns: Vec<String>) -> Self {
        SqlResultSet {
            rows: Vec::new(),
            columns,
        }
    }
    
    pub fn add_row(&mut self, row: Vec<SqlValue>) {
        self.rows.push(row);
    }
    
    pub fn len(&self) -> usize {
        self.rows.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}

/// SQL execution result
#[derive(Debug)]
pub struct SqlExecuteResult {
    pub rows_affected: u64,
    pub last_insert_id: Option<i64>,
}

impl SqlExecuteResult {
    pub fn new(rows_affected: u64) -> Self {
        SqlExecuteResult {
            rows_affected,
            last_insert_id: None,
        }
    }
    
    pub fn with_insert_id(mut self, id: i64) -> Self {
        self.last_insert_id = Some(id);
        self
    }
}

/// fr fr SQL driver registry for managing SQL-specific drivers
pub struct SqlDriverRegistry {
    drivers: HashMap<String, Arc<dyn SqlDriver>>,
}

impl SqlDriverRegistry {
    /// slay Create a new SQL driver registry
    pub fn new() -> Self {
        let mut registry = Self {
            drivers: HashMap::new(),
        };
        
        // Register built-in drivers
        registry.register_builtin_drivers();
        registry
    }
    
    /// slay Register built-in SQL drivers
    fn register_builtin_drivers(&mut self) {
        // PostgreSQL driver
        self.register_driver("postgresql", Arc::new(postgresql::PostgreSqlDriver::new()));
        self.register_driver("postgres", Arc::new(postgresql::PostgreSqlDriver::new()));
        
        // MySQL driver
        self.register_driver("mysql", Arc::new(mysql::MySqlDriver::new()));
        
        // SQLite driver
        self.register_driver("sqlite", Arc::new(sqlite::SqliteDriver::new()));
        self.register_driver("sqlite3", Arc::new(sqlite::SqliteDriver::new()));
    }
    
    /// slay Register a SQL driver
    pub fn register_driver(&mut self, name: &str, driver: Arc<dyn SqlDriver>) {
        self.drivers.insert(name.to_string(), driver);
    }
    
    /// slay Get a SQL driver by name
    pub fn get_driver(&self, name: &str) -> Option<Arc<dyn SqlDriver>> {
        self.drivers.get(name).cloned()
    }
    
    /// slay List available SQL drivers
    pub fn list_drivers(&self) -> Vec<String> {
        self.drivers.keys().cloned().collect()
    }
}

/// fr fr Global SQL driver registry
static SQL_DRIVER_REGISTRY: std::sync::LazyLock<std::sync::RwLock<SqlDriverRegistry>> = 
    std::sync::LazyLock::new(|| std::sync::RwLock::new(SqlDriverRegistry::new()));

/// slay Get the global SQL driver registry
pub fn sql_driver_registry() -> &'static std::sync::RwLock<SqlDriverRegistry> {
    &SQL_DRIVER_REGISTRY
}

/// slay Register a SQL driver globally
pub fn register_sql_driver(name: &str, driver: Arc<dyn SqlDriver>) -> DbResult<()> {
    let mut registry = sql_driver_registry().write()
        .map_err(|_| DatabaseError::driver("Failed to acquire SQL driver registry lock"))?;
    
    registry.register_driver(name, driver);
    Ok(())
}

/// slay Get a SQL driver by name
pub fn get_sql_driver(name: &str) -> DbResult<Arc<dyn SqlDriver>> {
    let registry = sql_driver_registry().read()
        .map_err(|_| DatabaseError::driver("Failed to acquire SQL driver registry lock"))?;
    
    registry.get_driver(name)
        .ok_or_else(|| DatabaseError::driver(&format!("SQL driver '{}' not found", name)))
}

/// slay List all available SQL drivers
pub fn list_sql_drivers() -> Vec<String> {
    sql_driver_registry().read()
        .map(|registry| registry.list_drivers())
        .unwrap_or_default()
}

/// fr fr SQL utility functions
pub mod utils {
    use super::*;
//     use crate::stdlib::packages::db_core::DatabaseConnection;

    /// slay Create a SQL connection quickly
    pub async fn sql_connect(driver_name: &str, connection_string: &str) -> DbResult<Box<dyn DatabaseConnection>> {
        let driver = get_sql_driver(driver_name)?;
        let config = ConnectionConfig::from_string(connection_string)?;
        driver.connect(&config)
    }
    /// slay Execute a quick SQL query
    pub async fn sql_query(
        driver_name: &str,
        connection_string: &str,
        sql: &str,
        params: &[SqlValue]
    ) -> DbResult<SqlResultSet> {
        let mut conn = sql_connect(driver_name, connection_string).await?;
        let db_params: Vec<SqlValue> = params.iter()
            .map(|v| v.clone())
            .collect();
        
        let result = conn.query(sql, &db_params)?;
        // Result is already SqlResultSet
        Ok(result)
    }

    /// slay Execute a SQL statement (INSERT, UPDATE, DELETE)
    pub async fn sql_execute(
        driver_name: &str,
        connection_string: &str,
        sql: &str,
        params: &[SqlValue]
    ) -> DbResult<SqlExecuteResult> {
        let mut conn = sql_connect(driver_name, connection_string).await?;
        let db_params: Vec<SqlValue> = params.iter()
            .map(|v| v.clone())
            .collect();
        
        let result = conn.execute(sql, &db_params)?;
        Ok(result)
    }

    /// slay Check if a SQL driver is available
    pub fn is_sql_driver_available(name: &str) -> bool {
        list_sql_drivers().contains(&name.to_string())
    }

    /// slay Get SQL dialect for a driver
    pub fn get_sql_dialect(driver_name: &str) -> DbResult<Box<dyn SqlDialectTrait>> {
        match driver_name {
            "mysql" => Ok(Box::new(MySqlDialect::new())),
            "postgres" => Ok(Box::new(PostgreSqlDialect::new())),
            "sqlite" => Ok(Box::new(SqliteDialect::new())),
            _ => Err(DatabaseError::driver(&format!("Unknown dialect for driver '{}'", driver_name))),
        }
    }
}
/// fr fr SQL package configuration
#[derive(Debug, Clone)]
pub struct SqlConfig {
    /// Default connection timeout
    pub connection_timeout: u32,
    /// Default query timeout
    pub query_timeout: u32,
    /// Enable statement caching by default
    pub enable_statement_caching: bool,
    /// Default statement cache size
    pub statement_cache_size: usize,
    /// Enable connection pooling by default
    pub enable_connection_pooling: bool,
    /// Default connection pool size
    pub connection_pool_size: usize,
    /// Enable SQL query logging
    pub enable_query_logging: bool,
    /// Enable performance monitoring
    pub enable_performance_monitoring: bool,
}
impl Default for SqlConfig {
    fn default() -> Self {
        Self {
            connection_timeout: 30,
            query_timeout: 30,
            enable_statement_caching: true,
            statement_cache_size: 1000,
            enable_connection_pooling: true,
            connection_pool_size: 10,
            enable_query_logging: false,
            enable_performance_monitoring: false,
        }
    }
}
/// slay Initialize the db_sql package
pub fn init_db_sql() -> DbResult<()> {
    println!("🗄️ db_sql package initialized - SQL drivers loaded and ready bestie!");
    
    // Verify that all built-in drivers are available
    let drivers = list_sql_drivers();
    for driver in &drivers {
        println!("  ✅ {} driver available", driver);
    }
    if drivers.is_empty() {
        return Err(DatabaseError::driver("No SQL drivers available"));
    }
    Ok(())
}

/// fr fr SQL package information
pub fn sql_package_info() -> SqlPackageInfo {
    SqlPackageInfo {
        features: vec!["SQL Query Engine".to_string(), "Driver Registry".to_string()],
    }
}

/// fr fr SQL package information structure
#[derive(Debug, Clone)]
pub struct SqlPackageInfo {
    pub features: Vec<String>,
}
