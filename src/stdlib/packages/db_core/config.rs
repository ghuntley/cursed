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
    pub connection: ConnectionConfig,
    /// Pool configuration
    pub pool: PoolConfig,
    /// Security configuration
    pub security: SecurityConfig,
    /// Performance configuration
    pub performance: PerformanceConfig,
    /// Logging configuration
    pub logging: LoggingConfig,
    /// Additional driver-specific configuration
    pub driver_config: DriverConfig,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            connection: ConnectionConfig::default(),
            pool: PoolConfig::default(),
            security: SecurityConfig::default(),
            performance: PerformanceConfig::default(),
            logging: LoggingConfig::default(),
            driver_config: DriverConfig::default(),
        }
    }
}

/// fr fr Connection configuration
#[derive(Debug, Clone)]
pub struct ConnectionConfig {
    /// Database URL or connection string
    pub connection_string: String,
    /// Connection timeout
    pub connect_timeout: Duration,
    /// Query timeout
    pub query_timeout: Duration,
    /// Whether to use SSL/TLS
    pub use_ssl: bool,
    /// SSL certificate path
    pub ssl_cert_path: Option<String>,
    /// SSL key path
    pub ssl_key_path: Option<String>,
    /// SSL CA path
    pub ssl_ca_path: Option<String>,
    /// Application name for connection identification
    pub application_name: Option<String>,
    /// Additional connection parameters
    pub parameters: HashMap<String, String>,
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        Self {
            connection_string: String::new(),
            connect_timeout: Duration::from_secs(30),
            query_timeout: Duration::from_secs(300),
            use_ssl: false,
            ssl_cert_path: None,
            ssl_key_path: None,
            ssl_ca_path: None,
            application_name: Some("CURSED".to_string()),
            parameters: HashMap::new(),
        }
    }
}

/// fr fr Connection pool configuration
#[derive(Debug, Clone)]
pub struct PoolConfig {
    /// Maximum number of connections in pool
    pub max_connections: usize,
    /// Minimum number of connections to maintain
    pub min_connections: usize,
    /// Maximum lifetime of a connection
    pub max_connection_lifetime: Duration,
    /// Maximum idle time for a connection
    pub max_idle_time: Duration,
    /// Timeout when acquiring connection from pool
    pub acquire_timeout: Duration,
    /// Whether to validate connections on acquire
    pub validate_on_acquire: bool,
    /// Whether to validate connections on return
    pub validate_on_return: bool,
    /// Connection test query
    pub test_query: Option<String>,
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            max_connections: 100,
            min_connections: 10,
            max_connection_lifetime: Duration::from_secs(3600), // 1 hour
            max_idle_time: Duration::from_secs(600), // 10 minutes
            acquire_timeout: Duration::from_secs(30),
            validate_on_acquire: true,
            validate_on_return: false,
            test_query: Some("SELECT 1".to_string()),
        }
    }
}

/// fr fr Security configuration
#[derive(Debug, Clone)]
pub struct SecurityConfig {
    /// Whether to enable query logging (may expose sensitive data)
    pub enable_query_logging: bool,
    /// Whether to log parameter values
    pub log_parameters: bool,
    /// Maximum query length to log
    pub max_query_log_length: usize,
    /// Fields to redact in logs
    pub redacted_fields: Vec<String>,
    /// Whether to enable connection encryption
    pub require_encryption: bool,
    /// Minimum TLS version
    pub min_tls_version: TlsVersion,
    /// Allowed cipher suites
    pub allowed_ciphers: Vec<String>,
    /// Whether to verify server certificates
    pub verify_server_cert: bool,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            enable_query_logging: false,
            log_parameters: false,
            max_query_log_length: 1000,
            redacted_fields: vec![
                "password".to_string(),
                "secret".to_string(),
                "token".to_string(),
                "key".to_string(),
            ],
            require_encryption: false,
            min_tls_version: TlsVersion::V1_2,
            allowed_ciphers: Vec::new(),
            verify_server_cert: true,
        }
    }
}

/// fr fr Performance configuration
#[derive(Debug, Clone)]
pub struct PerformanceConfig {
    /// Whether to enable prepared statement caching
    pub enable_prepared_statement_cache: bool,
    /// Maximum number of prepared statements to cache
    pub prepared_statement_cache_size: usize,
    /// Whether to enable query result caching
    pub enable_query_cache: bool,
    /// Maximum size of query cache in MB
    pub query_cache_size_mb: usize,
    /// Query cache TTL
    pub query_cache_ttl: Duration,
    /// Whether to enable connection pooling
    pub enable_connection_pooling: bool,
    /// Whether to enable batch execution optimization
    pub enable_batch_optimization: bool,
    /// Batch size for bulk operations
    pub batch_size: usize,
    /// Whether to enable async query execution
    pub enable_async_execution: bool,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            enable_prepared_statement_cache: true,
            prepared_statement_cache_size: 1000,
            enable_query_cache: false,
            query_cache_size_mb: 100,
            query_cache_ttl: Duration::from_secs(300), // 5 minutes
            enable_connection_pooling: true,
            enable_batch_optimization: true,
            batch_size: 1000,
            enable_async_execution: true,
        }
    }
}

/// fr fr Logging configuration
#[derive(Debug, Clone)]
pub struct LoggingConfig {
    /// Whether to enable database operation logging
    pub enable_logging: bool,
    /// Log level for database operations
    pub log_level: LogLevel,
    /// Whether to log slow queries
    pub log_slow_queries: bool,
    /// Threshold for slow query logging
    pub slow_query_threshold: Duration,
    /// Whether to log connection events
    pub log_connections: bool,
    /// Whether to log transaction events
    pub log_transactions: bool,
    /// Whether to log pool statistics
    pub log_pool_stats: bool,
    /// Pool stats logging interval
    pub pool_stats_interval: Duration,
    /// Whether to include stack traces in error logs
    pub include_stack_traces: bool,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            enable_logging: true,
            log_level: LogLevel::Info,
            log_slow_queries: true,
            slow_query_threshold: Duration::from_secs(1),
            log_connections: true,
            log_transactions: true,
            log_pool_stats: false,
            pool_stats_interval: Duration::from_secs(60),
            include_stack_traces: false,
        }
    }
}

/// fr fr Driver-specific configuration
#[derive(Debug, Clone)]
pub struct DriverConfig {
    /// Driver name
    pub driver_name: String,
    /// Driver version
    pub driver_version: Option<String>,
    /// Driver-specific options
    pub options: HashMap<String, String>,
    /// Driver feature flags
    pub features: Vec<String>,
    /// Driver initialization parameters
    pub init_params: HashMap<String, String>,
}

impl Default for DriverConfig {
    fn default() -> Self {
        Self {
            driver_name: "generic".to_string(),
            driver_version: None,
            options: HashMap::new(),
            features: Vec::new(),
            init_params: HashMap::new(),
        }
    }
}

/// fr fr TLS version enumeration
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TlsVersion {
    V1_0,
    V1_1,
    V1_2,
    V1_3,
}

/// fr fr Log level enumeration
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    CursedError,
}

impl DatabaseConfig {
    /// slay Create a new database configuration
    pub fn new() -> Self {
        Self::default()
    }

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
                    },
                    "connect_timeout" => {
                        if let Ok(timeout) = value.parse::<u64>() {
                            config.connection.connect_timeout = Duration::from_secs(timeout);
                        }
                    },
                    "sslmode" => {
                        config.connection.use_ssl = value != "disable";
                    },
                    _ => {
                        config.connection.parameters.insert(key.to_string(), value.to_string());
                    }
                }
            }
        }
        
        Ok(config)
    }

    /// slay Set connection timeout
    pub fn with_connect_timeout(mut self, timeout: Duration) -> Self {
        self.connection.connect_timeout = timeout;
        self
    }

    /// slay Set query timeout
    pub fn with_query_timeout(mut self, timeout: Duration) -> Self {
        self.connection.query_timeout = timeout;
        self
    }

    /// slay Set max connections
    pub fn with_max_connections(mut self, max_connections: usize) -> Self {
        self.pool.max_connections = max_connections;
        self
    }

    /// slay Enable SSL
    pub fn with_ssl(mut self) -> Self {
        self.connection.use_ssl = true;
        self.security.require_encryption = true;
        self
    }

    /// slay Set application name
    pub fn with_application_name(mut self, name: &str) -> Self {
        self.connection.application_name = Some(name.to_string());
        self
    }

    /// slay Enable query logging
    pub fn with_query_logging(mut self) -> Self {
        self.security.enable_query_logging = true;
        self.logging.enable_logging = true;
        self
    }

    /// slay Add connection parameter
    pub fn with_parameter(mut self, key: &str, value: &str) -> Self {
        self.connection.parameters.insert(key.to_string(), value.to_string());
        self
    }

    /// slay Add driver option
    pub fn with_driver_option(mut self, key: &str, value: &str) -> Self {
        self.driver_config.options.insert(key.to_string(), value.to_string());
        self
    }

    /// slay Validate configuration
    pub fn validate(&self) -> DbResult<()> {
        if self.connection.connection_string.is_empty() {
            return Err(DatabaseError::config("Connection string is required"));
        }

        if self.pool.max_connections == 0 {
            return Err(DatabaseError::config("Max connections must be greater than 0"));
        }

        if self.pool.min_connections > self.pool.max_connections {
            return Err(DatabaseError::config("Min connections cannot exceed max connections"));
        }

        if self.connection.connect_timeout.is_zero() {
            return Err(DatabaseError::config("Connect timeout must be greater than 0"));
        }

        if self.connection.query_timeout.is_zero() {
            return Err(DatabaseError::config("Query timeout must be greater than 0"));
        }

        Ok(())
    }
}

