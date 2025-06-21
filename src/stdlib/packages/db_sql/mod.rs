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
pub use builder::{
    SqlQueryBuilder, SelectBuilder, InsertBuilder, UpdateBuilder, 
    DeleteBuilder, CreateTableBuilder, AlterTableBuilder
};
pub use types::{
    SqlValue, SqlType, SqlParameter, SqlRow, SqlColumn,
    SqlNull, SqlDateTime, SqlDecimal, SqlArray, SqlJson
};
pub use dialect::{
    SqlDialect, PostgreSqlDialect, MySqlDialect, SqliteDialect,
    DialectFeatures, SqlKeywords, SqlFunctions, SqlDialectTrait
};
pub use connection::{SqlConnection, SqlConnectionPool, SqlTransaction};
pub use prepared::{PreparedStatement, StatementCache};
pub use result::{SqlResultSet, SqlExecuteResult, SqlRowIterator};
pub use migration::{
    SqlMigration, MigrationRunner, MigrationStatus, 
    SchemaVersion, MigrationScript
};

// Driver-specific exports
pub use postgresql::{PostgreSqlDriver, PostgreSqlConnection, PgError};
// pub use mysql::{MySqlDriver, MySqlConnection, MySqlError};  // Temporarily disabled - mysql crate not available
pub use sqlite::{SqliteDriver, SqliteConnection, SqliteError};

use crate::stdlib::packages::db_core::{
    DatabaseError, DriverRegistry, 
    ConnectionConfig, DatabaseDriver
};
use crate::stdlib::packages::db_core::error::{DatabaseResult as DbResult};
use std::sync::Arc;

/// fr fr SQL driver registry for managing SQL-specific drivers
#[derive(Debug)]
pub struct SqlDriverRegistry {
    drivers: std::collections::HashMap<String, Arc<dyn SqlDriver>>,
}

impl SqlDriverRegistry {
    /// slay Create a new SQL driver registry
    pub fn new() -> Self {
        let mut registry = Self {
            drivers: std::collections::HashMap::new(),
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
    use crate::stdlib::packages::db_core::DatabaseConnection;

    /// slay Create a SQL connection quickly
    pub async fn sql_connect(driver_name: &str, connection_string: &str) -> DbResult<Box<dyn DatabaseConnection>> {
        let driver = get_sql_driver(driver_name)?;
        let config = ConnectionConfig::from_string(connection_string)?;
        driver.connect(config).await
    }

    /// slay Execute a quick SQL query
    pub async fn sql_query(
        driver_name: &str,
        connection_string: &str,
        sql: &str,
        params: &[SqlValue]
    ) -> DbResult<SqlResultSet> {
        let mut conn = sql_connect(driver_name, connection_string).await?;
        let db_params: Vec<crate::stdlib::packages::db_core::Parameter> = params.iter()
            .map(|v| v.clone().into())
            .collect();
        
        let result = conn.query(sql, &db_params).await?;
        // Convert result to SqlResultSet
        Ok(SqlResultSet::from_database_result(result))
    }

    /// slay Execute a SQL statement (INSERT, UPDATE, DELETE)
    pub async fn sql_execute(
        driver_name: &str,
        connection_string: &str,
        sql: &str,
        params: &[SqlValue]
    ) -> DbResult<SqlExecuteResult> {
        let mut conn = sql_connect(driver_name, connection_string).await?;
        let db_params: Vec<crate::stdlib::packages::db_core::Parameter> = params.iter()
            .map(|v| v.clone().into())
            .collect();
        
        let result = conn.execute(sql, &db_params).await?;
        Ok(SqlExecuteResult::from_execute_result(result))
    }

    /// slay Check if a SQL driver is available
    pub fn is_sql_driver_available(name: &str) -> bool {
        list_sql_drivers().contains(&name.to_string())
    }

    /// slay Get SQL dialect for a driver
    pub fn get_sql_dialect(driver_name: &str) -> DbResult<Box<dyn SqlDialectTrait>> {
        match driver_name {
            "postgresql" | "postgres" => Ok(Box::new(PostgreSqlDialect::new())),
            "mysql" => Ok(Box::new(MySqlDialect::new())),
            "sqlite" | "sqlite3" => Ok(Box::new(SqliteDialect::new())),
            _ => Err(DatabaseError::driver(&format!("Unknown SQL dialect for driver '{}'", driver_name))),
        }
    }
}

/// fr fr SQL package configuration
#[derive(Debug, Clone)]
pub struct SqlConfig {
    /// Default connection timeout
    pub default_timeout: std::time::Duration,
    /// Default query timeout
    pub default_query_timeout: std::time::Duration,
    /// Enable statement caching by default
    pub enable_statement_cache: bool,
    /// Default statement cache size
    pub statement_cache_size: usize,
    /// Enable connection pooling by default
    pub enable_connection_pooling: bool,
    /// Default connection pool size
    pub default_pool_size: usize,
    /// Enable SQL query logging
    pub enable_query_logging: bool,
    /// Enable performance monitoring
    pub enable_performance_monitoring: bool,
}

impl Default for SqlConfig {
    fn default() -> Self {
        Self {
            default_timeout: std::time::Duration::from_secs(30),
            default_query_timeout: std::time::Duration::from_secs(60),
            enable_statement_cache: true,
            statement_cache_size: 100,
            enable_connection_pooling: true,
            default_pool_size: 10,
            enable_query_logging: false,
            enable_performance_monitoring: true,
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
        version: "1.0.0".to_string(),
        supported_drivers: list_sql_drivers(),
        features: vec![
            "Connection pooling".to_string(),
            "Prepared statements".to_string(),
            "Transactions".to_string(),
            "Migrations".to_string(),
            "Query builder".to_string(),
            "Multiple SQL dialects".to_string(),
        ],
    }
}

/// fr fr SQL package information structure
#[derive(Debug, Clone)]
pub struct SqlPackageInfo {
    pub version: String,
    pub supported_drivers: Vec<String>,
    pub features: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sql_driver_registry() {
        let registry = SqlDriverRegistry::new();
        let drivers = registry.list_drivers();
        
        // Should have at least the built-in drivers
        assert!(drivers.contains(&"postgresql".to_string()));
        assert!(drivers.contains(&"mysql".to_string()));
        assert!(drivers.contains(&"sqlite".to_string()));
    }

    #[test]
    fn test_init_db_sql() {
        assert!(init_db_sql().is_ok());
    }

    #[test]
    fn test_utils() {
        assert!(utils::is_sql_driver_available("postgresql"));
        assert!(utils::is_sql_driver_available("mysql"));
        assert!(utils::is_sql_driver_available("sqlite"));
        assert!(!utils::is_sql_driver_available("nonexistent"));
    }

    #[test]
    fn test_sql_config() {
        let config = SqlConfig::default();
        assert_eq!(config.default_timeout, std::time::Duration::from_secs(30));
        assert!(config.enable_statement_cache);
        assert!(config.enable_connection_pooling);
    }

    #[test]
    fn test_package_info() {
        let info = sql_package_info();
        assert_eq!(info.version, "1.0.0");
        assert!(!info.supported_drivers.is_empty());
        assert!(!info.features.is_empty());
    }
}
