/// fr fr Core database interfaces and types - the foundation periodt
/// 
/// This module provides the fundamental database abstractions that all
/// other database packages build upon. Think of it as the blueprint bestie!

// Core traits and interfaces
pub mod traits;
pub mod connection;
pub mod query;
pub mod transaction;
pub mod result;
pub mod metadata;
pub mod error;
pub mod config;

// Re-export all the important types for easy access - periodt
pub use traits::{
    DatabaseDriver, DatabaseConnection, DatabaseTransaction, 
    QueryExecutor, ResultSet, PreparedStatement, ConnectionManager,
    DriverInfo, ParameterMetadata, PoolStats, QueryStream, DriverFeature, SqlDialect
};
pub use connection::{
    ConnectionConfig, ConnectionInfo, ConnectionState, 
    DatabaseConnectionImpl, ConnectionOptions
};
pub use query::{
    Query, QueryBuilder, QueryType, Parameter, ParameterSet,
    SqlQuery, NoSqlQuery, QueryPlan, QueryCache, ExecutionStep
};
pub use transaction::{
    Transaction, TransactionIsolation, TransactionState,
    TransactionOptions, SavePoint, TransactionManager
};
pub use result::{
    DatabaseQueryResult, Row, Column, ColumnType, ResultSetImpl,
    ResultMetadata, ExecuteResult, QueryStats, ColumnValue, RowMetadata
};
pub use metadata::{
    DatabaseMetadata, TableMetadata, ColumnInfo, IndexInfo,
    SchemaInfo, ConstraintInfo, ForeignKeyInfo, StatisticsInfo
};
pub use error::{
    DatabaseError, DatabaseResult as DbResult, ErrorKind,
    ConnectionError, QueryError, TransactionError, DriverError
};
pub use config::{
    DatabaseConfig, DriverConfig, PoolConfig, 
    SecurityConfig, PerformanceConfig, LoggingConfig
};

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// fr fr Global database driver registry - manages all available drivers
static DRIVER_REGISTRY: std::sync::LazyLock<Arc<RwLock<DriverRegistry>>> = 
    std::sync::LazyLock::new(|| Arc::new(RwLock::new(DriverRegistry::new())));

/// fr fr Driver registry for managing database drivers
#[derive(Debug, Default)]
pub struct DriverRegistry {
    drivers: HashMap<String, Arc<dyn DatabaseDriver + Send + Sync>>,
}

impl DriverRegistry {
    /// slay Create a new driver registry
    pub fn new() -> Self {
        Self {
            drivers: HashMap::new(),
        }
    }

    /// slay Register a database driver
    pub fn register_driver<T>(&mut self, name: String, driver: T) 
    where
        T: DatabaseDriver + Send + Sync + 'static,
    {
        self.drivers.insert(name, Arc::new(driver));
    }

    /// slay Get a driver by name
    pub fn get_driver(&self, name: &str) -> Option<Arc<dyn DatabaseDriver + Send + Sync>> {
        self.drivers.get(name).map(|d| Arc::clone(d))
    }

    /// slay List all available drivers
    pub fn list_drivers(&self) -> Vec<String> {
        self.drivers.keys().cloned().collect()
    }
}

/// slay Register a database driver globally
pub fn register_driver<T>(name: &str, driver: T) -> DbResult<()>
where
    T: DatabaseDriver + Send + Sync + 'static,
{
    let mut registry = DRIVER_REGISTRY.write()
        .map_err(|_| DatabaseError::driver("Failed to acquire driver registry lock"))?;
    
    registry.register_driver(name.to_string(), driver);
    Ok(())
}

/// slay Get a driver by name from global registry
pub fn get_driver(name: &str) -> DbResult<Arc<dyn DatabaseDriver + Send + Sync>> {
    let registry = DRIVER_REGISTRY.read()
        .map_err(|_| DatabaseError::driver("Failed to acquire driver registry lock"))?;
    
    registry.get_driver(name)
        .ok_or_else(|| DatabaseError::driver(&format!("Driver '{}' not found", name)))
}

/// slay List all available drivers globally
pub fn list_drivers() -> Vec<String> {
    DRIVER_REGISTRY.read()
        .map(|registry| registry.list_drivers())
        .unwrap_or_default()
}

/// fr fr Database helper functions and utilities
pub mod utils {
    use super::*;
    
    /// slay Create a database connection with default config
    pub async fn connect(driver_name: &str, connection_string: &str) -> DbResult<Box<dyn DatabaseConnection>> {
        let driver = get_driver(driver_name)?;
        let config = ConnectionConfig::from_string(connection_string)?;
        driver.connect(config).await
    }
    
    /// slay Quick query execution helper
    pub async fn quick_query(
        driver_name: &str, 
        connection_string: &str, 
        query: &str
    ) -> DbResult<Box<dyn ResultSet>> {
        let mut conn = connect(driver_name, connection_string).await?;
        conn.query(query, &[]).await
    }
    
    /// slay Check if a driver is available
    pub fn is_driver_available(name: &str) -> bool {
        list_drivers().contains(&name.to_string())
    }
}

/// fr fr Initialize the db_core package
pub fn init_db_core() -> DbResult<()> {
    println!("🗄️ db_core package initialized - database foundation ready bestie!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_driver_registry() {
        let mut registry = DriverRegistry::new();
        assert_eq!(registry.list_drivers().len(), 0);
        
        // In a real test, we'd register a mock driver here
        assert!(registry.get_driver("nonexistent").is_none());
    }
    
    #[test]
    fn test_init_db_core() {
        assert!(init_db_core().is_ok());
    }
    
    #[test]
    fn test_utils() {
        assert!(!utils::is_driver_available("nonexistent"));
    }
}
