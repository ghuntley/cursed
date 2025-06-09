/// PostgreSQL driver implementation for CURSED database operations
/// 
/// This module provides the main PostgreSQL driver that implements the
/// Driver trait and handles connection management, capability reporting,
/// and driver registration.

use std::sync::Arc;
use super::super::{
    Driver, DriverConn, DriverCapabilities, DatabaseError, DatabaseErrorKind, SqlIsolationLevel
};
use super::{
    PostgreSQLConnection, PostgreSQLConfig, PostgreSQLError,
    config::ConnectionString
};

/// fr fr PostgreSQL driver implementation
#[derive(Debug, Clone)]
pub struct PostgreSQLDriver {
    /// fr fr Driver name
    name: String,
    /// fr fr Driver capabilities
    capabilities: DriverCapabilities,
    /// fr fr Default configuration
    default_config: PostgreSQLConfig,
}

impl PostgreSQLDriver {
    /// slay Create a new PostgreSQL driver
    pub fn new() -> Self {
        Self {
            name: "postgres".to_string(),
            capabilities: Self::build_capabilities(),
            default_config: PostgreSQLConfig::default(),
        }
    }
    
    /// slay Create driver with custom configuration
    pub fn with_config(config: PostgreSQLConfig) -> Self {
        Self {
            name: "postgres".to_string(),
            capabilities: Self::build_capabilities(),
            default_config: config,
        }
    }
    
    /// slay Build PostgreSQL-specific capabilities
    fn build_capabilities() -> DriverCapabilities {
        DriverCapabilities {
            supports_transactions: true,
            supports_prepared_statements: true,
            supports_multiple_result_sets: false, // PostgreSQL doesn't support this directly
            supports_stored_procedures: true,
            supports_batch_operations: true,
            supports_concurrent_connections: true,
            max_connections: Some(200), // Default PostgreSQL max_connections
            supported_isolation_levels: vec![
                SqlIsolationLevel::LevelReadUncommitted,
                SqlIsolationLevel::LevelReadCommitted,
                SqlIsolationLevel::LevelRepeatableRead,
                SqlIsolationLevel::LevelSerializable,
            ],
            max_query_length: Some(1_073_741_823), // ~1GB max query size
            max_parameter_count: Some(65535), // PostgreSQL parameter limit
        }
    }
    
    /// slay Parse and validate connection string
    fn parse_connection_string(&self, data_source_name: &str) -> Result<PostgreSQLConfig, PostgreSQLError> {
        if data_source_name.is_empty() {
            return Ok(self.default_config.clone());
        }
        
        ConnectionString::parse(data_source_name)?.to_config()
    }
    
    /// slay Validate connection configuration
    fn validate_config(&self, config: &PostgreSQLConfig) -> Result<(), PostgreSQLError> {
        // Validate host
        if config.host.is_empty() {
            return Err(PostgreSQLError::new(
                DatabaseErrorKind::ConfigurationError,
                "Host cannot be empty".to_string()
            ));
        }
        
        // Validate port
        if config.port == 0 || config.port > 65535 {
            return Err(PostgreSQLError::new(
                DatabaseErrorKind::ConfigurationError,
                format!("Invalid port: {}", config.port)
            ));
        }
        
        // Validate database name
        if config.dbname.is_empty() {
            return Err(PostgreSQLError::new(
                DatabaseErrorKind::ConfigurationError,
                "Database name cannot be empty".to_string()
            ));
        }
        
        // Validate user
        if config.user.is_empty() {
            return Err(PostgreSQLError::new(
                DatabaseErrorKind::ConfigurationError,
                "Username cannot be empty".to_string()
            ));
        }
        
        // Validate timeouts
        if config.connect_timeout.as_secs() == 0 {
            return Err(PostgreSQLError::new(
                DatabaseErrorKind::ConfigurationError,
                "Connect timeout must be greater than 0".to_string()
            ));
        }
        
        if config.query_timeout.as_secs() == 0 {
            return Err(PostgreSQLError::new(
                DatabaseErrorKind::ConfigurationError,
                "Query timeout must be greater than 0".to_string()
            ));
        }
        
        Ok(())
    }
    
    /// slay Test connection with given configuration
    pub fn test_connection(&self, config: &PostgreSQLConfig) -> Result<(), PostgreSQLError> {
        self.validate_config(config)?;
        
        // Try to create a connection and ping
        let conn = PostgreSQLConnection::from_config(config.clone())?;
        conn.ping().map_err(|e| PostgreSQLError::connection_error(&e.to_string()))?;
        
        Ok(())
    }
    
    /// slay Get driver version information
    pub fn version(&self) -> String {
        "PostgreSQL Driver for CURSED v1.0.0".to_string()
    }
    
    /// slay Get supported PostgreSQL versions
    pub fn supported_pg_versions(&self) -> Vec<String> {
        vec![
            "9.6".to_string(),
            "10".to_string(),
            "11".to_string(),
            "12".to_string(),
            "13".to_string(),
            "14".to_string(),
            "15".to_string(),
            "16".to_string(),
        ]
    }
    
    /// slay Check if PostgreSQL version is supported
    pub fn is_version_supported(&self, version: &str) -> bool {
        self.supported_pg_versions().contains(&version.to_string())
    }
    
    /// slay Get PostgreSQL-specific features
    pub fn pg_features(&self) -> Vec<String> {
        vec![
            "Arrays".to_string(),
            "JSON/JSONB".to_string(),
            "Custom Types".to_string(),
            "Inheritance".to_string(),
            "Partitioning".to_string(),
            "Full Text Search".to_string(),
            "PostGIS (if installed)".to_string(),
            "Extensions".to_string(),
            "Triggers".to_string(),
            "Stored Procedures".to_string(),
            "Views".to_string(),
            "Materialized Views".to_string(),
            "CTEs (Common Table Expressions)".to_string(),
            "Window Functions".to_string(),
            "COPY Protocol".to_string(),
            "Listen/Notify".to_string(),
            "Large Objects".to_string(),
            "Connection Pooling".to_string(),
            "Streaming Replication".to_string(),
            "Point-in-Time Recovery".to_string(),
        ]
    }
    
    /// slay Create connection pool configuration
    pub fn pool_config(&self) -> super::pool::PostgreSQLPoolConfig {
        super::pool::PostgreSQLPoolConfig::default()
    }
}

impl Driver for PostgreSQLDriver {
    /// slay Open a new connection to PostgreSQL
    fn open(&self, data_source_name: &str) -> Result<Box<dyn DriverConn>, DatabaseError> {
        let config = self.parse_connection_string(data_source_name)
            .map_err(|e| DatabaseError::new(DatabaseErrorKind::ConfigurationError, &e.to_string()))?;
        
        self.validate_config(&config)
            .map_err(|e| DatabaseError::new(DatabaseErrorKind::ConfigurationError, &e.to_string()))?;
        
        PostgreSQLConnection::from_config(config)
            .map(|conn| Box::new(conn) as Box<dyn DriverConn>)
            .map_err(|e| DatabaseError::new(DatabaseErrorKind::ConnectionError, &e.to_string()))
    }
    
    /// slay Get the name of this driver
    fn name(&self) -> &str {
        &self.name
    }
    
    /// slay Get driver-specific capabilities
    fn capabilities(&self) -> DriverCapabilities {
        self.capabilities.clone()
    }
    
    /// slay Clone this driver
    fn clone_driver(&self) -> Box<dyn Driver> {
        Box::new(self.clone())
    }
}

impl Default for PostgreSQLDriver {
    fn default() -> Self {
        Self::new()
    }
}

/// fr fr PostgreSQL driver factory for creating configured drivers
#[derive(Debug)]
pub struct PostgreSQLDriverFactory {
    /// fr fr Available driver configurations
    configs: std::collections::HashMap<String, PostgreSQLConfig>,
}

impl PostgreSQLDriverFactory {
    /// slay Create a new driver factory
    pub fn new() -> Self {
        Self {
            configs: std::collections::HashMap::new(),
        }
    }
    
    /// slay Register a named configuration
    pub fn register_config(&mut self, name: String, config: PostgreSQLConfig) {
        self.configs.insert(name, config);
    }
    
    /// slay Create driver with named configuration
    pub fn create_driver(&self, config_name: &str) -> Result<PostgreSQLDriver, PostgreSQLError> {
        if let Some(config) = self.configs.get(config_name) {
            Ok(PostgreSQLDriver::with_config(config.clone()))
        } else {
            Err(PostgreSQLError::new(
                DatabaseErrorKind::ConfigurationError,
                format!("Configuration '{}' not found", config_name)
            ))
        }
    }
    
    /// slay Create driver with connection string
    pub fn create_driver_from_url(&self, url: &str) -> Result<PostgreSQLDriver, PostgreSQLError> {
        let config = ConnectionString::parse(url)?.to_config()?;
        Ok(PostgreSQLDriver::with_config(config))
    }
    
    /// slay List available configurations
    pub fn list_configs(&self) -> Vec<String> {
        self.configs.keys().cloned().collect()
    }
    
    /// slay Get configuration by name
    pub fn get_config(&self, name: &str) -> Option<&PostgreSQLConfig> {
        self.configs.get(name)
    }
}

impl Default for PostgreSQLDriverFactory {
    fn default() -> Self {
        let mut factory = Self::new();
        
        // Register some common configurations
        factory.register_config(
            "localhost".to_string(),
            PostgreSQLConfig::default()
        );
        
        factory.register_config(
            "dev".to_string(),
            PostgreSQLConfig::default()
                .host("localhost".to_string())
                .port(5432)
                .dbname("dev_db".to_string())
                .user("dev_user".to_string())
        );
        
        factory.register_config(
            "test".to_string(),
            PostgreSQLConfig::default()
                .host("localhost".to_string())
                .port(5432)
                .dbname("test_db".to_string())
                .user("test_user".to_string())
        );
        
        factory
    }
}

/// fr fr PostgreSQL driver registry for managing multiple PostgreSQL drivers
#[derive(Debug)]
pub struct PostgreSQLDriverRegistry {
    /// fr fr Registered drivers by name
    drivers: std::collections::HashMap<String, Arc<PostgreSQLDriver>>,
    /// fr fr Factory for creating new drivers
    factory: PostgreSQLDriverFactory,
}

impl PostgreSQLDriverRegistry {
    /// slay Create a new driver registry
    pub fn new() -> Self {
        Self {
            drivers: std::collections::HashMap::new(),
            factory: PostgreSQLDriverFactory::default(),
        }
    }
    
    /// slay Register a driver
    pub fn register(&mut self, name: String, driver: PostgreSQLDriver) {
        self.drivers.insert(name, Arc::new(driver));
    }
    
    /// slay Get a driver
    pub fn get(&self, name: &str) -> Option<Arc<PostgreSQLDriver>> {
        self.drivers.get(name).cloned()
    }
    
    /// slay Create and register driver from configuration
    pub fn register_from_config(&mut self, name: String, config: PostgreSQLConfig) -> Result<(), PostgreSQLError> {
        let driver = PostgreSQLDriver::with_config(config);
        self.register(name, driver);
        Ok(())
    }
    
    /// slay Create and register driver from URL
    pub fn register_from_url(&mut self, name: String, url: &str) -> Result<(), PostgreSQLError> {
        let driver = self.factory.create_driver_from_url(url)?;
        self.register(name, driver);
        Ok(())
    }
    
    /// slay List registered drivers
    pub fn list_drivers(&self) -> Vec<String> {
        self.drivers.keys().cloned().collect()
    }
    
    /// slay Remove driver
    pub fn unregister(&mut self, name: &str) -> Option<Arc<PostgreSQLDriver>> {
        self.drivers.remove(name)
    }
    
    /// slay Get factory
    pub fn factory(&self) -> &PostgreSQLDriverFactory {
        &self.factory
    }
    
    /// slay Get mutable factory
    pub fn factory_mut(&mut self) -> &mut PostgreSQLDriverFactory {
        &mut self.factory
    }
}

impl Default for PostgreSQLDriverRegistry {
    fn default() -> Self {
        let mut registry = Self::new();
        
        // Register default PostgreSQL driver
        registry.register("default".to_string(), PostgreSQLDriver::new());
        
        registry
    }
}

/// fr fr Global PostgreSQL driver registry
lazy_static::lazy_static! {
    static ref GLOBAL_PG_DRIVER_REGISTRY: std::sync::Mutex<PostgreSQLDriverRegistry> = 
        std::sync::Mutex::new(PostgreSQLDriverRegistry::default());
}

/// slay Register PostgreSQL driver globally
pub fn register_pg_driver(name: String, driver: PostgreSQLDriver) -> Result<(), PostgreSQLError> {
    let mut registry = GLOBAL_PG_DRIVER_REGISTRY.lock().map_err(|_| {
        PostgreSQLError::new(
            DatabaseErrorKind::DriverError,
            "Failed to acquire global registry lock".to_string()
        )
    })?;
    
    registry.register(name, driver);
    Ok(())
}

/// slay Get PostgreSQL driver from global registry
pub fn get_pg_driver(name: &str) -> Result<Arc<PostgreSQLDriver>, PostgreSQLError> {
    let registry = GLOBAL_PG_DRIVER_REGISTRY.lock().map_err(|_| {
        PostgreSQLError::new(
            DatabaseErrorKind::DriverError,
            "Failed to acquire global registry lock".to_string()
        )
    })?;
    
    registry.get(name).ok_or_else(|| {
        PostgreSQLError::new(
            DatabaseErrorKind::DriverError,
            format!("PostgreSQL driver '{}' not found", name)
        )
    })
}

/// slay List all globally registered PostgreSQL drivers
pub fn list_pg_drivers() -> Result<Vec<String>, PostgreSQLError> {
    let registry = GLOBAL_PG_DRIVER_REGISTRY.lock().map_err(|_| {
        PostgreSQLError::new(
            DatabaseErrorKind::DriverError,
            "Failed to acquire global registry lock".to_string()
        )
    })?;
    
    Ok(registry.list_drivers())
}

/// fr fr Helper functions for driver management
pub mod driver_utils {
    use super::*;
    
    /// slay Create PostgreSQL driver from environment variables
    pub fn driver_from_env() -> Result<PostgreSQLDriver, PostgreSQLError> {
        let mut config = PostgreSQLConfig::default();
        
        if let Ok(host) = std::env::var("PGHOST") {
            config.host = host;
        }
        
        if let Ok(port_str) = std::env::var("PGPORT") {
            config.port = port_str.parse().map_err(|_| {
                PostgreSQLError::new(
                    DatabaseErrorKind::ConfigurationError,
                    format!("Invalid PGPORT: {}", port_str)
                )
            })?;
        }
        
        if let Ok(dbname) = std::env::var("PGDATABASE") {
            config.dbname = dbname;
        }
        
        if let Ok(user) = std::env::var("PGUSER") {
            config.user = user;
        }
        
        if let Ok(password) = std::env::var("PGPASSWORD") {
            config.password = Some(password);
        }
        
        if let Ok(ssl_mode_str) = std::env::var("PGSSLMODE") {
            config.ssl_mode = super::config::SslMode::from_string(&ssl_mode_str)?;
        }
        
        Ok(PostgreSQLDriver::with_config(config))
    }
    
    /// slay Test multiple connection configurations
    pub fn test_connections(configs: &[PostgreSQLConfig]) -> Vec<(usize, Result<(), PostgreSQLError>)> {
        configs.iter()
            .enumerate()
            .map(|(i, config)| {
                let driver = PostgreSQLDriver::with_config(config.clone());
                (i, driver.test_connection(config))
            })
            .collect()
    }
    
    /// slay Create driver with best available configuration
    pub fn create_best_driver(configs: &[PostgreSQLConfig]) -> Result<PostgreSQLDriver, PostgreSQLError> {
        for config in configs {
            let driver = PostgreSQLDriver::with_config(config.clone());
            if driver.test_connection(config).is_ok() {
                return Ok(driver);
            }
        }
        
        Err(PostgreSQLError::connection_error("No working configuration found"))
    }
}
