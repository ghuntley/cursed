/// fr fr SQLite driver implementation that slays with comprehensive database features periodt
/// 
/// This module implements the main SQLite driver that integrates with the
/// CURSED database system, providing connection management, prepared statements,
/// transactions, and SQLite-specific functionality.

use std::sync::{Arc, Mutex, RwLock};
use std::collections::HashMap;
use std::time::{SystemTime, Duration};
use super::{
    SqliteError, SqliteResult, SqliteConfig, SqliteConnectionString,
    SqliteStats, SqliteVersion, SqliteFeatures,
    init_sqlite, is_sqlite_initialized
};
use crate::error::CursedError;
use super::real_connection::RealSqliteConnection;
use super::super::{
    Driver, DriverConn, DatabaseError, DatabaseErrorKind,
    SqlIsolationLevel, VibeContext
};

use super::super::driver::DriverCapabilities;

/// fr fr SQLite driver capabilities
#[derive(Debug, Clone)]
pub struct SqliteDriverCapabilities {
    /// fr fr Base driver capabilities
    pub base: DriverCapabilities,
    /// fr fr SQLite version information
    pub version: SqliteVersion,
    /// fr fr Available SQLite features
    pub features: SqliteFeatures,
    /// fr fr Maximum database size (theoretical)
    pub max_database_size: u64,
    /// fr fr Maximum number of columns
    pub max_columns: u32,
    /// fr fr Maximum SQL statement length
    pub max_sql_length: u32,
    /// fr fr Maximum page size
    pub max_page_size: u32,
    /// fr fr Supported journal modes
    pub supported_journal_modes: Vec<String>,
    /// fr fr Supported synchronous modes
    pub supported_synchronous_modes: Vec<String>,
    /// fr fr Extensions support
    pub supports_extensions: bool,
    /// fr fr Virtual tables support
    pub supports_virtual_tables: bool,
    /// fr fr FTS (Full-Text Search) support
    pub supports_fts: bool,
    /// fr fr JSON support
    pub supports_json: bool,
    /// fr fr CTE (Common Table Expressions) support
    pub supports_cte: bool,
    /// fr fr Window functions support
    pub supports_window_functions: bool,
}

impl SqliteDriverCapabilities {
    /// slay Create new SQLite driver capabilities
    pub fn new() -> SqliteResult<Self> {
        let version = super::ffi::SqliteFFI::get_version()?;
        let features = SqliteFeatures::detect()?;
        
        let base = DriverCapabilities {
            supports_transactions: true,
            supports_prepared_statements: true,
            supports_multiple_result_sets: false,
            supports_stored_procedures: false,
            supports_batch_operations: true,
            supports_concurrent_connections: true,
            max_connections: Some(1000),
            supported_isolation_levels: vec![
                SqlIsolationLevel::LevelDefault,
                SqlIsolationLevel::LevelReadUncommitted,
                SqlIsolationLevel::LevelReadCommitted,
                SqlIsolationLevel::LevelSerializable,
            ],
            max_query_length: Some(1_000_000),
            max_parameter_count: Some(999),
        };

        Ok(Self {
            base,
            version: version.clone(),
            features: features.clone(),
            max_database_size: 281_474_976_710_656, // 256TB
            max_columns: 2000,
            max_sql_length: 1_000_000,
            max_page_size: 65536,
            supported_journal_modes: vec![
                "DELETE".to_string(),
                "PERSIST".to_string(),
                "MEMORY".to_string(),
                "WAL".to_string(),
                "OFF".to_string(),
                "TRUNCATE".to_string(),
            ],
            supported_synchronous_modes: vec![
                "OFF".to_string(),
                "NORMAL".to_string(),
                "FULL".to_string(),
                "EXTRA".to_string(),
            ],
            supports_extensions: features.has_loadable_extensions,
            supports_virtual_tables: features.has_virtual_tables,
            supports_fts: features.has_fts5,
            supports_json: features.has_json1,
            supports_cte: version.version_number >= 3008003, // 3.8.3+
            supports_window_functions: version.version_number >= 3025000, // 3.25.0+
        })
    }

    /// slay Check if feature is supported
    pub fn supports_feature(&self, feature: &str) -> bool {
        match feature.to_lowercase().as_str() {
            "transactions" => self.base.supports_transactions,
            "prepared_statements" => self.base.supports_prepared_statements,
            "batch_operations" => self.base.supports_batch_operations,
            "concurrent_connections" => self.base.supports_concurrent_connections,
            "extensions" => self.supports_extensions,
            "virtual_tables" => self.supports_virtual_tables,
            "fts" | "full_text_search" => self.supports_fts,
            "json" => self.supports_json,
            "cte" | "common_table_expressions" => self.supports_cte,
            "window_functions" => self.supports_window_functions,
            _ => false,
        }
    }

    /// slay Get feature description
    pub fn feature_description(&self, feature: &str) -> Option<String> {
        match feature.to_lowercase().as_str() {
            "transactions" => Some("ACID transactions with savepoints".to_string()),
            "prepared_statements" => Some("Compiled SQL statements with parameter binding".to_string()),
            "batch_operations" => Some("Multiple operations in single transaction".to_string()),
            "concurrent_connections" => Some("Thread-safe database access".to_string()),
            "extensions" => Some("Loadable SQLite extensions".to_string()),
            "virtual_tables" => Some("Virtual table modules".to_string()),
            "fts" => Some("Full-text search with FTS5".to_string()),
            "json" => Some("JSON data type and functions".to_string()),
            "cte" => Some("Common Table Expressions (WITH clause)".to_string()),
            "window_functions" => Some("SQL window functions".to_string()),
            _ => None,
        }
    }
}

impl Default for SqliteDriverCapabilities {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| {
            // Fallback if we can't detect capabilities
            Self {
                base: DriverCapabilities::default(),
                version: SqliteVersion {
                    version_string: "Unknown".to_string(),
                    version_number: 0,
                    source_id: "Unknown".to_string(),
                },
                features: SqliteFeatures::default(),
                max_database_size: 281_474_976_710_656,
                max_columns: 2000,
                max_sql_length: 1_000_000,
                max_page_size: 65536,
                supported_journal_modes: Vec::from(["DELETE".to_string()]),
                supported_synchronous_modes: Vec::from(["FULL".to_string()]),
                supports_extensions: false,
                supports_virtual_tables: false,
                supports_fts: false,
                supports_json: false,
                supports_cte: false,
                supports_window_functions: false,
            }
        })
    }
}

/// fr fr Main SQLite driver implementation
#[derive(Debug)]
pub struct SqliteDriver {
    /// fr fr Driver capabilities
    capabilities: SqliteDriverCapabilities,
    /// fr fr Global driver statistics
    stats: Arc<RwLock<SqliteStats>>,
    /// fr fr Active connections registry
    connections: Arc<Mutex<HashMap<String, Arc<RealSqliteConnection>>>>,
    /// fr fr Driver configuration
    config: Arc<RwLock<SqliteConfig>>,
    /// fr fr Driver initialization timestamp
    initialized_at: SystemTime,
    /// fr fr Driver name and version
    driver_info: DriverInfo,
}

/// fr fr Driver information
#[derive(Debug, Clone)]
pub struct DriverInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub build_date: String,
}

impl DriverInfo {
    /// slay Create new driver info
    pub fn new() -> Self {
        Self {
            name: "SQLite Driver for CURSED".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            description: "High-performance SQLite database driver with comprehensive features".to_string(),
            author: "CURSED Database Team".to_string(),
            build_date: std::env::var("BUILD_DATE").unwrap_or_else(|_| "unknown".to_string()),
        }
    }
}

impl Default for DriverInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl SqliteDriver {
    /// slay Create new SQLite driver
    pub fn new() -> SqliteResult<Self> {
        // Initialize SQLite library if not already done
        if !is_sqlite_initialized() {
            init_sqlite()?;
        }

        let capabilities = SqliteDriverCapabilities::new()?;
        let stats = Arc::new(RwLock::new(SqliteStats::default()));
        let connections = Arc::new(Mutex::new(HashMap::new()));
        let config = Arc::new(RwLock::new(SqliteConfig::default()));

        Ok(Self {
            capabilities,
            stats,
            connections,
            config,
            initialized_at: SystemTime::now(),
            driver_info: DriverInfo::new(),
        })
    }

    /// slay Create driver with custom configuration
    pub fn with_config(config: SqliteConfig) -> SqliteResult<Self> {
        let mut driver = Self::new()?;
        
        {
            let mut driver_config = driver.config.write()
                .map_err(|_| SqliteError::internal("Failed to acquire config lock"))?;
            *driver_config = config;
        }

        Ok(driver)
    }

    /// slay Get driver capabilities
    pub fn get_capabilities(&self) -> &SqliteDriverCapabilities {
        &self.capabilities
    }

    /// slay Get driver statistics
    pub fn get_stats(&self) -> SqliteResult<SqliteStats> {
        let stats = self.stats.read()
            .map_err(|_| SqliteError::internal("Failed to acquire stats lock"))?;
        Ok(stats.clone())
    }

    /// slay Update driver statistics
    pub fn update_stats<F>(&self, updater: F) -> SqliteResult<()>
    where
        F: FnOnce(&mut SqliteStats),
    {
        let mut stats = self.stats.write()
            .map_err(|_| SqliteError::internal("Failed to acquire stats lock"))?;
        updater(&mut stats);
        stats.update();
        Ok(())
    }

    /// slay Get active connection count
    pub fn active_connection_count(&self) -> SqliteResult<usize> {
        let connections = self.connections.lock()
            .map_err(|_| SqliteError::internal("Failed to acquire connections lock"))?;
        Ok(connections.len())
    }

    /// slay Register connection
    pub fn register_connection(&self, connection_id: String, connection: Arc<RealSqliteConnection>) -> SqliteResult<()> {
        let mut connections = self.connections.lock()
            .map_err(|_| SqliteError::internal("Failed to acquire connections lock"))?;
        connections.insert(connection_id, connection);
        
        self.update_stats(|stats| {
            stats.connections_created += 1;
            stats.active_connections += 1;
        })?;

        Ok(())
    }

    /// slay Unregister connection
    pub fn unregister_connection(&self, connection_id: &str) -> SqliteResult<()> {
        let mut connections = self.connections.lock()
            .map_err(|_| SqliteError::internal("Failed to acquire connections lock"))?;
        connections.remove(connection_id);
        
        self.update_stats(|stats| {
            if stats.active_connections > 0 {
                stats.active_connections -= 1;
            }
        })?;

        Ok(())
    }

    /// slay Get connection by ID
    pub fn get_connection(&self, connection_id: &str) -> SqliteResult<Option<Arc<RealSqliteConnection>>> {
        let connections = self.connections.lock()
            .map_err(|_| SqliteError::internal("Failed to acquire connections lock"))?;
        Ok(connections.get(connection_id).cloned())
    }

    /// slay List all active connections
    pub fn list_connections(&self) -> SqliteResult<Vec<String>> {
        let connections = self.connections.lock()
            .map_err(|_| SqliteError::internal("Failed to acquire connections lock"))?;
        Ok(connections.keys().cloned().collect())
    }

    /// slay Set global configuration
    pub fn set_config(&self, config: SqliteConfig) -> SqliteResult<()> {
        config.validate()?;
        
        let mut driver_config = self.config.write()
            .map_err(|_| SqliteError::internal("Failed to acquire config lock"))?;
        *driver_config = config;
        
        Ok(())
    }

    /// slay Get global configuration
    pub fn get_config(&self) -> SqliteResult<SqliteConfig> {
        let config = self.config.read()
            .map_err(|_| SqliteError::internal("Failed to acquire config lock"))?;
        Ok(config.clone())
    }

    /// slay Get driver uptime
    pub fn uptime(&self) -> Duration {
        SystemTime::now().duration_since(self.initialized_at)
            .unwrap_or_default()
    }

    /// slay Get driver information
    pub fn driver_info(&self) -> &DriverInfo {
        &self.driver_info
    }

    /// slay Test database connectivity
    pub fn test_connectivity(&self, connection_string: &str) -> SqliteResult<bool> {
        match self.open(connection_string) {
            Ok(conn) => {
                let ping_result = conn.ping();
                let _ = conn.close(); // Always try to close
                ping_result.map(|_| true).map_err(|e| SqliteError::internal(&e.to_string()))
            }
            Err(_) => Ok(false),
        }
    }

    /// slay Create connection with custom configuration
    pub fn open_with_config(&self, config: SqliteConfig) -> SqliteResult<Box<dyn DriverConn>> {
        config.validate()?;
        
        // Use production connection instead of placeholder
        let connection = super::production_driver::ProductionSqliteConnection::new(config)?;
        Ok(Box::new(connection))
    }

    /// slay Clone driver for connection management (internal use)
    fn clone_for_connection(&self) -> Self {
        // Create a lightweight clone for connection management
        Self {
            capabilities: self.capabilities.clone(),
            stats: Arc::clone(&self.stats),
            connections: Arc::clone(&self.connections),
            config: Arc::clone(&self.config),
            initialized_at: self.initialized_at,
            driver_info: self.driver_info.clone(),
        }
    }

    /// slay Validate connection string
    pub fn validate_connection_string(&self, connection_string: &str) -> SqliteResult<()> {
        SqliteConnectionString::parse(connection_string)?;
        Ok(())
    }

    /// slay Get connection string examples
    pub fn connection_string_examples(&self) -> Vec<(String, String)> {
        vec![
            ("Simple file".to_string(), "database.db".to_string()),
            ("Memory database".to_string(), ":memory:".to_string()),
            ("Read-only file".to_string(), "file:database.db?mode=ro".to_string()),
            ("WAL mode".to_string(), "file:database.db?journal_mode=WAL".to_string()),
            ("URI with parameters".to_string(), "file:database.db?cache=shared&foreign_keys=true".to_string()),
            ("Data source format".to_string(), "Data Source=database.db;Journal Mode=WAL;Foreign Keys=true".to_string()),
        ]
    }

    /// slay Perform driver health check
    pub fn health_check(&self) -> SqliteResult<DriverHealthStatus> {
        let mut status = DriverHealthStatus::new();

        // Check SQLite library initialization
        status.sqlite_initialized = is_sqlite_initialized();
        
        // Check memory usage
        let stats = self.get_stats()?;
        status.memory_usage = stats.memory_usage;
        
        // Check active connections
        status.active_connections = self.active_connection_count()?;
        
        // Check uptime
        status.uptime = self.uptime();
        
        // Test basic functionality
        status.basic_functionality = self.test_connectivity(":memory:").unwrap_or(false);
        
        // Overall health
        status.overall_health = status.sqlite_initialized && 
                               status.basic_functionality &&
                               status.active_connections < 1000; // Arbitrary limit

        Ok(status)
    }
}

/// fr fr Driver health status
#[derive(Debug, Clone)]
pub struct DriverHealthStatus {
    pub overall_health: bool,
    pub sqlite_initialized: bool,
    pub basic_functionality: bool,
    pub memory_usage: u64,
    pub active_connections: usize,
    pub uptime: Duration,
    pub last_check: SystemTime,
}

impl DriverHealthStatus {
    /// slay Create new health status
    pub fn new() -> Self {
        Self {
            overall_health: false,
            sqlite_initialized: false,
            basic_functionality: false,
            memory_usage: 0,
            active_connections: 0,
            uptime: Duration::ZERO,
            last_check: SystemTime::now(),
        }
    }
}

impl Default for DriverHealthStatus {
    fn default() -> Self {
        Self::new()
    }
}

/// fr fr Managed SQLite connection that handles cleanup
#[derive(Debug)]
struct ManagedSqliteConnection {
    connection: Arc<RealSqliteConnection>,
    connection_id: String,
    driver: Arc<SqliteDriver>,
}

impl DriverConn for ManagedSqliteConnection {
    fn prepare(&self, query: &str) -> crate::error::Result<()> {
        self.connection.prepare(query)
    }

    fn query(&self, query: &str, args: &[super::super::SqlValue]) -> crate::error::Result<()> {
        self.connection.query(query, args)
    }

    fn execute(&self, query: &str, args: &[super::super::SqlValue]) -> crate::error::Result<()> {
        self.connection.execute(query, args)
    }

    fn begin_transaction(&self, opts: super::super::TxOptions) -> crate::error::Result<()> {
        self.connection.begin_transaction(opts)
    }

    fn ping(&self) -> crate::error::Result<()> {
        self.connection.ping()
    }

    fn close(&self) -> crate::error::Result<()> {
        let result = self.connection.close();
        // Unregister from driver
        let _ = self.driver.unregister_connection(&self.connection_id);
        result
    }

    fn is_alive(&self) -> bool {
        self.connection.is_alive()
    }

    fn metadata(&self) -> super::super::driver::ConnectionMetadata {
        self.connection.metadata()
    }

    fn clone(&self) -> Box<dyn DriverConn> {
        Box::new(ManagedSqliteConnection {
            connection: Arc::clone(&self.connection),
            connection_id: self.connection_id.clone(),
            driver: Arc::clone(&self.driver),
        })
    }
}

impl Drop for ManagedSqliteConnection {
    fn drop(&mut self) {
        // Ensure cleanup when connection is dropped
        let _ = self.driver.unregister_connection(&self.connection_id);
    }
}

/// fr fr Main Driver trait implementation for SqliteDriver
impl Driver for SqliteDriver {
    fn open(&self, data_source_name: &str) -> crate::error::Result<()> {
        let connection_string = SqliteConnectionString::parse(data_source_name)
            .map_err(|e| DatabaseError::new(DatabaseErrorKind::ConnectionError, &e.to_string()))?;
        
        self.open_with_config(connection_string.config)
            .map_err(|e| e.to_database_error())
    }

    fn name(&self) -> &str {
        &self.driver_info.name
    }

    fn capabilities(&self) -> DriverCapabilities {
        self.capabilities.base.clone()
    }

    fn clone_driver(&self) -> Box<dyn Driver> {
        Box::new(self.clone_for_connection())
    }
}

impl Clone for SqliteDriver {
    fn clone(&self) -> Self {
        Self {
            capabilities: self.capabilities.clone(),
            stats: Arc::clone(&self.stats),
            connections: Arc::clone(&self.connections),
            config: Arc::clone(&self.config),
            initialized_at: self.initialized_at,
            driver_info: self.driver_info.clone(),
        }
    }
}

