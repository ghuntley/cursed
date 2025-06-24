/// PostgreSQL Database Driver Implementation
/// 
/// Provides production-ready PostgreSQL database driver with connection pooling,
/// comprehensive error handling, and integration with the CURSED database system.

use std::sync::Arc;
use std::time::SystemTime;
use crate::stdlib::database::{
    Driver, DriverConn, DatabaseError, SqlIsolationLevel
};
use crate::stdlib::database::driver::DriverCapabilities;
use super::config::{PostgresConfig, PostgresConnectionString};
use super::connection::PostgresConnection;
use super::pool::{PostgresPool, PostgresPoolConfig};
use super::error::{PostgresError, PostgresErrorKind, PostgresResult};

/// PostgreSQL database driver
#[derive(Debug, Clone)]
pub struct PostgresDriver {
    /// Driver name
    name: String,
    /// Driver version
    version: String,
    /// Creation timestamp
    created_at: SystemTime,
    /// Default configuration
    default_config: PostgresConfig,
    /// Connection pool (if enabled)
    pool: Option<Arc<PostgresPool>>,
}

impl PostgresDriver {
    /// Create new PostgreSQL driver
    pub fn new() -> Self {
        Self {
            name: "PostgreSQL Driver for CURSED".to_string(),
            version: "1.0.0".to_string(),
            created_at: SystemTime::now(),
            default_config: PostgresConfig::default(),
            pool: None,
        }
    }

    /// Create driver with custom default configuration
    pub fn with_config(config: PostgresConfig) -> Self {
        Self {
            name: "PostgreSQL Driver for CURSED".to_string(),
            version: "1.0.0".to_string(),
            created_at: SystemTime::now(),
            default_config: config,
            pool: None,
        }
    }

    /// Create driver with connection pool
    pub async fn with_pool(config: PostgresConfig) -> PostgresResult<Self> {
        let pool = PostgresPool::new(config.clone()).await?;
        
        Ok(Self {
            name: "PostgreSQL Driver for CURSED (Pooled)".to_string(),
            version: "1.0.0".to_string(),
            created_at: SystemTime::now(),
            default_config: config,
            pool: Some(Arc::new(pool)),
        })
    }

    /// Get driver capabilities
    pub fn get_capabilities() -> DriverCapabilities {
        DriverCapabilities {
            supports_transactions: true,
            supports_prepared_statements: true,
            supports_multiple_result_sets: false, // PostgreSQL doesn't support multiple result sets in single query
            supports_stored_procedures: true,
            supports_batch_operations: true,
            supports_concurrent_connections: true,
            max_connections: Some(1000), // Reasonable default
            supported_isolation_levels: vec![
                SqlIsolationLevel::LevelReadCommitted,
                SqlIsolationLevel::LevelRepeatableRead,
                SqlIsolationLevel::LevelSerializable,
            ],
            max_query_length: Some(1_000_000), // 1MB query limit
            max_parameter_count: Some(65535),   // PostgreSQL limit
        }
    }

    /// Parse connection string and create connection
    pub async fn connect_with_string(dsn: &str) -> PostgresResult<PostgresConnection> {
        let config = PostgresConnectionString::parse(dsn)?;
        PostgresConnection::new(config).await
    }

    /// Create connection pool from connection string
    pub async fn create_pool_with_string(dsn: &str) -> PostgresResult<PostgresPool> {
        let config = PostgresConnectionString::parse(dsn)?;
        PostgresPool::new(config).await
    }

    /// Test connection with given configuration
    pub async fn test_connection(config: &PostgresConfig) -> PostgresResult<()> {
        let mut connection = PostgresConnection::new(config.clone()).await?;
        if !connection.is_alive() {
            return Err(PostgresError::ConnectionLost);
        }
        Ok(())
    }

    /// Get pool statistics if pool is enabled
    pub fn get_pool_stats(&self) -> Option<super::pool::PoolStatistics> {
        self.pool.as_ref().map(|pool| pool.get_statistics())
    }

    /// Get pool health if pool is enabled
    pub fn get_pool_health(&self) -> Option<super::pool::PoolHealth> {
        self.pool.as_ref().map(|pool| pool.get_health())
    }

    /// Check if driver is using connection pool
    pub fn is_pooled(&self) -> bool {
        self.pool.is_some()
    }

    /// Get default configuration
    pub fn default_config(&self) -> &PostgresConfig {
        &self.default_config
    }

    /// Set default configuration
    pub fn set_default_config(&mut self, config: PostgresConfig) {
        self.default_config = config;
    }

    /// Create connection using default configuration
    pub async fn create_default_connection(&self) -> PostgresResult<PostgresConnection> {
        if let Some(ref pool) = self.pool {
            // If pool is available, get connection from pool
            let pooled_conn = pool.get_connection().await?;
            Ok(PostgresConnection::from_client(
                pooled_conn.into_client(),
                self.default_config.clone(),
            ))
        } else {
            // Create direct connection
            PostgresConnection::new(self.default_config.clone()).await
        }
    }

    /// Close driver and cleanup resources
    pub async fn close(&self) -> PostgresResult<()> {
        if let Some(ref pool) = self.pool {
            pool.close().await;
        }
        Ok(())
    }

    /// Get driver information
    pub fn info(&self) -> DriverInfo {
        DriverInfo {
            name: self.name.clone(),
            version: self.version.clone(),
            created_at: self.created_at,
            is_pooled: self.is_pooled(),
            default_host: self.default_config.host.clone(),
            default_port: self.default_config.port,
            default_database: self.default_config.database.clone(),
            ssl_mode: self.default_config.ssl_mode,
            max_connections: if self.is_pooled() {
                Some(self.default_config.max_connections)
            } else {
                None
            },
        }
    }
}

impl Default for PostgresDriver {
    fn default() -> Self {
        Self::new()
    }
}

impl Driver for PostgresDriver {
    fn open(&self, data_source_name: &str) -> Result<(), Error> {
        // Parse connection string
        let config = PostgresConnectionString::parse(data_source_name)
            .map_err(|e| e.to_database_error())?;

        // For sync API, we need to use tokio runtime
        let rt = tokio::runtime::Handle::current();
        
        let connection = rt.block_on(async {
            if let Some(ref pool) = self.pool {
                // Use pool if available
                let pooled_conn = pool.get_connection().await?;
                Ok(PostgresConnection::from_client(
                pooled_conn.into_client(),
                    config,
                ))
            } else {
                // Create direct connection
                PostgresConnection::new(config).await
            }
        }).map_err(|e: PostgresError| e.to_database_error())?;

        Ok(Box::new(connection))
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn capabilities(&self) -> DriverCapabilities {
        Self::get_capabilities()
    }

    fn clone_driver(&self) -> Box<dyn Driver> {
        Box::new(self.clone())
    }
}

/// Driver information for introspection
#[derive(Debug, Clone)]
pub struct DriverInfo {
    pub name: String,
    pub version: String,
    pub created_at: SystemTime,
    pub is_pooled: bool,
    pub default_host: String,
    pub default_port: u16,
    pub default_database: String,
    pub ssl_mode: super::config::SslMode,
    pub max_connections: Option<u32>,
}

impl std::fmt::Display for DriverInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "PostgreSQL Driver Information:")?;
        writeln!(f, "  Name: {}", self.name)?;
        writeln!(f, "  Version: {}", self.version)?;
        writeln!(f, "  Created: {:?}", self.created_at)?;
        writeln!(f, "  Connection Pool: {}", if self.is_pooled { "Enabled" } else { "Disabled" })?;
        writeln!(f, "  Default Host: {}:{}", self.default_host, self.default_port)?;
        writeln!(f, "  Default Database: {}", self.default_database)?;
        writeln!(f, "  SSL Mode: {}", self.ssl_mode)?;
        
        if let Some(max_conn) = self.max_connections {
            writeln!(f, "  Max Connections: {}", max_conn)?;
        }
        
        Ok(())
    }
}

/// PostgreSQL driver builder for advanced configuration
pub struct PostgresDriverBuilder {
    config: PostgresConfig,
    enable_pool: bool,
    pool_config: Option<PostgresPoolConfig>,
}

impl PostgresDriverBuilder {
    /// Create new driver builder
    pub fn new() -> Self {
        Self {
            config: PostgresConfig::default(),
            enable_pool: false,
            pool_config: None,
        }
    }

    /// Set database connection details
    pub fn connection(mut self, host: &str, port: u16, database: &str, username: &str) -> Self {
        self.config.host = host.to_string();
        self.config.port = port;
        self.config.database = database.to_string();
        self.config.username = username.to_string();
        self
    }

    /// Set password
    pub fn password(mut self, password: &str) -> Self {
        self.config.password = Some(password.to_string());
        self
    }

    /// Set SSL mode
    pub fn ssl_mode(mut self, ssl_mode: super::config::SslMode) -> Self {
        self.config.ssl_mode = ssl_mode;
        self
    }

    /// Enable connection pooling
    pub fn with_pool(mut self) -> Self {
        self.enable_pool = true;
        self
    }

    /// Set pool configuration
    pub fn pool_config(mut self, pool_config: PostgresPoolConfig) -> Self {
        self.pool_config = Some(pool_config);
        self
    }

    /// Set connection timeouts
    pub fn timeouts(mut self, connect_timeout: std::time::Duration, query_timeout: std::time::Duration) -> Self {
        self.config.connect_timeout = connect_timeout;
        self.config.query_timeout = query_timeout;
        self
    }

    /// Build the driver
    pub async fn build(self) -> PostgresResult<PostgresDriver> {
        if self.enable_pool {
            PostgresDriver::with_pool(self.config).await
        } else {
            Ok(PostgresDriver::with_config(self.config))
        }
    }
}

impl Default for PostgresDriverBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
use crate::error::Error;

    #[test]
    fn test_driver_creation() {
        let driver = PostgresDriver::new();
        assert_eq!(driver.name(), "PostgreSQL Driver for CURSED");
        assert!(!driver.is_pooled());
    }

    #[test]
    fn test_driver_capabilities() {
        let caps = PostgresDriver::get_capabilities();
        assert!(caps.supports_transactions);
        assert!(caps.supports_prepared_statements);
        assert!(caps.supports_concurrent_connections);
        assert_eq!(caps.max_parameter_count, Some(65535));
    }

    #[test]
    fn test_driver_builder() {
        let builder = PostgresDriverBuilder::new()
            .connection("localhost", 5432, "testdb", "user")
            .password("pass")
            .ssl_mode(super::config::SslMode::Require)
            .with_pool();
        
        // Test that builder accepts all configuration methods
        assert_eq!(builder.config.host, "localhost");
        assert_eq!(builder.config.port, 5432);
        assert_eq!(builder.config.database, "testdb");
        assert_eq!(builder.config.username, "user");
        assert_eq!(builder.config.password, Some("pass".to_string()));
        assert_eq!(builder.config.ssl_mode, super::config::SslMode::Require);
        assert!(builder.enable_pool);
    }

    #[tokio::test]
    async fn test_connection_string_parsing() {
        let dsn = "postgresql://user:pass@localhost:5432/testdb";
        
        // This will fail without a real PostgreSQL server, but tests the parsing
        let result = PostgresDriver::connect_with_string(dsn).await;
        
        // Expect connection failure, not parsing error
        if let Err(err) = result {
            assert!(matches!(
                err.kind,
                PostgresErrorKind::ConnectionFailed | PostgresErrorKind::TimeoutError
            ));
        }
    }

    #[test]
    fn test_driver_info() {
        let driver = PostgresDriver::new();
        let info = driver.info();
        
        assert_eq!(info.name, "PostgreSQL Driver for CURSED");
        assert!(!info.is_pooled);
        assert_eq!(info.default_host, "localhost");
        assert_eq!(info.default_port, 5432);
        assert_eq!(info.default_database, "postgres");
    }
}
