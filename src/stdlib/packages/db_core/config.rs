/// fr fr Database configuration - setting up connections right periodt
///
/// This module handles all database configuration including connection settings,
/// pool configuration, security settings, and performance tuning. Get it right bestie!

// use crate::stdlib::packages::db_core::error::{DatabaseResult as DbResult, DatabaseError};
use std::collections::HashMap;
use std::time::Duration;

/// fr fr Main database configuration
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    /// Connection configuration
    /// Pool configuration
    /// Security configuration
    /// Performance configuration
    /// Logging configuration
    /// Additional driver-specific configuration
impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
        }
    }
/// fr fr Connection configuration
#[derive(Debug, Clone)]
pub struct ConnectionConfig {
    /// Database URL or connection string
    /// Connection timeout
    /// Query timeout
    /// Whether to use SSL/TLS
    /// SSL certificate path
    /// SSL key path
    /// SSL CA path
    /// Application name for connection identification
    /// Additional connection parameters
impl Default for ConnectionConfig {
    fn default() -> Self {
        Self {
        }
    }
/// fr fr Connection pool configuration
#[derive(Debug, Clone)]
pub struct PoolConfig {
    /// Maximum number of connections in pool
    /// Minimum number of connections to maintain
    /// Maximum lifetime of a connection
    /// Maximum idle time for a connection
    /// Timeout when acquiring connection from pool
    /// Whether to validate connections on acquire
    /// Whether to validate connections on return
    /// Connection test query
impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            max_connection_lifetime: Duration::from_secs(3600), // 1 hour
            max_idle_time: Duration::from_secs(600), // 10 minutes
        }
    }
/// fr fr Security configuration
#[derive(Debug, Clone)]
pub struct SecurityConfig {
    /// Whether to enable query logging (may expose sensitive data)
    /// Whether to log parameter values
    /// Maximum query length to log
    /// Fields to redact in logs
    /// Whether to enable connection encryption
    /// Minimum TLS version
    /// Allowed cipher suites
    /// Whether to verify server certificates
impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            redacted_fields: vec![
        }
    }
/// fr fr Performance configuration
#[derive(Debug, Clone)]
pub struct PerformanceConfig {
    /// Whether to enable prepared statement caching
    /// Maximum number of prepared statements to cache
    /// Whether to enable query result caching
    /// Maximum size of query cache in MB
    /// Query cache TTL
    /// Whether to enable connection pooling
    /// Whether to enable batch execution optimization
    /// Batch size for bulk operations
    /// Whether to enable async query execution
impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            query_cache_ttl: Duration::from_secs(300), // 5 minutes
        }
    }
/// fr fr Logging configuration
#[derive(Debug, Clone)]
pub struct LoggingConfig {
    /// Whether to enable database operation logging
    /// Log level for database operations
    /// Whether to log slow queries
    /// Threshold for slow query logging
    /// Whether to log connection events
    /// Whether to log transaction events
    /// Whether to log pool statistics
    /// Pool stats logging interval
    /// Whether to include stack traces in error logs
impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
        }
    }
/// fr fr Driver-specific configuration
#[derive(Debug, Clone)]
pub struct DriverConfig {
    /// Driver name
    /// Driver version
    /// Driver-specific options
    /// Driver feature flags
    /// Driver initialization parameters
impl Default for DriverConfig {
    fn default() -> Self {
        Self {
        }
    }
/// fr fr TLS version enumeration
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TlsVersion {
/// fr fr Log level enumeration
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
impl DatabaseConfig {
    /// slay Create a new database configuration
    pub fn new() -> Self {
        Self::default()
    /// slay Create configuration from connection string
    pub fn from_connection_string(connection_string: &str) -> DbResult<Self> {
        let mut config = Self::default();
        config.connection.connection_string = connection_string.to_string();
        
        // Parse additional parameters from connection string if needed
        if let Ok(url) = url::Url::parse(connection_string) {
            // Extract driver from scheme
            config.driver_config.driver_name = url.scheme().to_string();
            
            // Extract application name if present
            for (key, value) in url.query_pairs() {
                match key.as_ref() {
                    "application_name" => {
                        config.connection.application_name = Some(value.to_string());
                    "connect_timeout" => {
                        if let Ok(timeout) = value.parse::<u64>() {
                            config.connection.connect_timeout = Duration::from_secs(timeout);
                        }
                    "sslmode" => {
                        config.connection.use_ssl = value != "disable";
                    _ => {
                        config.connection.parameters.insert(key.to_string(), value.to_string());
                    }
                }
            }
        }
        
        Ok(config)
    /// slay Set connection timeout
    pub fn with_connect_timeout(mut self, timeout: Duration) -> Self {
        self.connection.connect_timeout = timeout;
        self
    /// slay Set query timeout
    pub fn with_query_timeout(mut self, timeout: Duration) -> Self {
        self.connection.query_timeout = timeout;
        self
    /// slay Set max connections
    pub fn with_max_connections(mut self, max_connections: usize) -> Self {
        self.pool.max_connections = max_connections;
        self
    /// slay Enable SSL
    pub fn with_ssl(mut self) -> Self {
        self.connection.use_ssl = true;
        self.security.require_encryption = true;
        self
    /// slay Set application name
    pub fn with_application_name(mut self, name: &str) -> Self {
        self.connection.application_name = Some(name.to_string());
        self
    /// slay Enable query logging
    pub fn with_query_logging(mut self) -> Self {
        self.security.enable_query_logging = true;
        self.logging.enable_logging = true;
        self
    /// slay Add connection parameter
    pub fn with_parameter(mut self, key: &str, value: &str) -> Self {
        self.connection.parameters.insert(key.to_string(), value.to_string());
        self
    /// slay Add driver option
    pub fn with_driver_option(mut self, key: &str, value: &str) -> Self {
        self.driver_config.options.insert(key.to_string(), value.to_string());
        self
    /// slay Validate configuration
    pub fn validate(&self) -> DbResult<()> {
        if self.connection.connection_string.is_empty() {
            return Err(DatabaseError::config("Connection string is required"));
        if self.pool.max_connections == 0 {
            return Err(DatabaseError::config("Max connections must be greater than 0"));
        if self.pool.min_connections > self.pool.max_connections {
            return Err(DatabaseError::config("Min connections cannot exceed max connections"));
        if self.connection.connect_timeout.is_zero() {
            return Err(DatabaseError::config("Connect timeout must be greater than 0"));
        if self.connection.query_timeout.is_zero() {
            return Err(DatabaseError::config("Query timeout must be greater than 0"));
        Ok(())
    }
}

