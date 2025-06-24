use crate::error::Error;
/// fr fr Database configuration management - settings that matter periodt
///
/// This module handles database configuration, driver settings, connection pools,
/// security options, and performance tuning. Configuration is key bestie!

use std::collections::HashMap;
use std::time::Duration;
use std::path::PathBuf;

/// fr fr Main database configuration
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    /// Driver-specific configuration
    pub driver_config: DriverConfig,
    /// Connection pool configuration
    pub pool_config: PoolConfig,
    /// Security configuration
    pub security_config: SecurityConfig,
    /// Performance tuning configuration
    pub performance_config: PerformanceConfig,
    /// Logging configuration
    pub logging_config: LoggingConfig,
    /// Custom properties
    pub custom_properties: HashMap<String, String>,
}

/// fr fr Driver-specific configuration
#[derive(Debug, Clone)]
pub struct DriverConfig {
    /// Driver name (postgresql, mysql, sqlite, etc.)
    pub driver_name: String,
    /// Driver version preference
    pub driver_version: Option<String>,
    /// Driver-specific properties
    pub properties: HashMap<String, String>,
    /// Connection string template
    pub connection_template: Option<String>,
    /// Default database name
    pub default_database: Option<String>,
    /// Driver initialization parameters
    pub init_parameters: HashMap<String, String>,
}

/// fr fr Connection pool configuration
#[derive(Debug, Clone)]
pub struct PoolConfig {
    /// Minimum number of connections in pool
    pub min_connections: usize,
    /// Maximum number of connections in pool
    pub max_connections: usize,
    /// Initial number of connections to create
    pub initial_connections: usize,
    /// Connection timeout when acquiring from pool
    pub acquire_timeout: Duration,
    /// Maximum lifetime of a connection
    pub max_lifetime: Duration,
    /// Idle timeout before connection is closed
    pub idle_timeout: Duration,
    /// How often to test idle connections
    pub validation_interval: Duration,
    /// SQL query to validate connections
    pub validation_query: Option<String>,
    /// Whether to test connections before use
    pub test_on_borrow: bool,
    /// Whether to test connections when returned
    pub test_on_return: bool,
    /// Whether to enable connection pooling
    pub enabled: bool,
}

/// fr fr Security configuration
#[derive(Debug, Clone)]
pub struct SecurityConfig {
    /// SSL/TLS configuration
    pub ssl_config: SslConfig,
    /// Authentication configuration
    pub auth_config: AuthConfig,
    /// Encryption settings
    pub encryption_config: EncryptionConfig,
    /// Access control settings
    pub access_control: AccessControlConfig,
    /// Audit settings
    pub audit_config: AuditConfig,
}

/// fr fr SSL/TLS configuration
#[derive(Debug, Clone)]
pub struct SslConfig {
    /// Whether SSL is enabled
    pub enabled: bool,
    /// SSL mode (disable, allow, prefer, require, verify-ca, verify-full)
    pub mode: SslMode,
    /// Path to client certificate
    pub client_cert_path: Option<PathBuf>,
    /// Path to client private key
    pub client_key_path: Option<PathBuf>,
    /// Path to CA certificate
    pub ca_cert_path: Option<PathBuf>,
    /// Whether to verify server certificate
    pub verify_server_cert: bool,
    /// Whether to verify certificate hostname
    pub verify_hostname: bool,
    /// SSL protocol version
    pub protocol_version: Option<String>,
    /// Cipher suites
    pub cipher_suites: Vec<String>,
}

/// fr fr SSL modes
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SslMode {
    /// Disable SSL completely
    Disable,
    /// Allow SSL but don't require it
    Allow,
    /// Prefer SSL but fall back to non-SSL
    Prefer,
    /// Require SSL connection
    Require,
    /// Require SSL and verify CA
    VerifyCA,
    /// Require SSL and verify full certificate
    VerifyFull,
}

/// fr fr Authentication configuration
#[derive(Debug, Clone)]
pub struct AuthConfig {
    /// Authentication method
    pub method: AuthMethod,
    /// Username
    pub username: Option<String>,
    /// Password (should be encrypted/hashed)
    pub password: Option<String>,
    /// Kerberos principal
    pub kerberos_principal: Option<String>,
    /// LDAP configuration
    pub ldap_config: Option<LdapConfig>,
    /// OAuth configuration
    pub oauth_config: Option<OAuthConfig>,
    /// Token-based authentication
    pub token_config: Option<TokenConfig>,
}

/// fr fr Authentication methods
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthMethod {
    /// Username/password
    Password,
    /// Kerberos authentication
    Kerberos,
    /// LDAP authentication
    Ldap,
    /// OAuth 2.0
    OAuth,
    /// Token-based
    Token,
    /// Certificate-based
    Certificate,
    /// No authentication
    None,
}

/// fr fr LDAP configuration
#[derive(Debug, Clone)]
pub struct LdapConfig {
    /// LDAP server URL
    pub server_url: String,
    /// Base DN for user search
    pub base_dn: String,
    /// User search filter
    pub user_filter: String,
    /// Bind DN for LDAP connection
    pub bind_dn: Option<String>,
    /// Bind password
    pub bind_password: Option<String>,
}

/// fr fr OAuth configuration
#[derive(Debug, Clone)]
pub struct OAuthConfig {
    /// Authorization server URL
    pub auth_server_url: String,
    /// Client ID
    pub client_id: String,
    /// Client secret
    pub client_secret: String,
    /// Scopes
    pub scopes: Vec<String>,
    /// Redirect URI
    pub redirect_uri: String,
}

/// fr fr Token configuration
#[derive(Debug, Clone)]
pub struct TokenConfig {
    /// Token type (Bearer, API Key, etc.)
    pub token_type: String,
    /// Token value
    pub token_value: String,
    /// Token expiration
    pub expires_at: Option<std::time::SystemTime>,
    /// Refresh token
    pub refresh_token: Option<String>,
}

/// fr fr Encryption configuration
#[derive(Debug, Clone)]
pub struct EncryptionConfig {
    /// Whether to encrypt data at rest
    pub encrypt_at_rest: bool,
    /// Whether to encrypt data in transit
    pub encrypt_in_transit: bool,
    /// Encryption algorithm
    pub algorithm: String,
    /// Key management configuration
    pub key_config: KeyConfig,
}

/// fr fr Key management configuration
#[derive(Debug, Clone)]
pub struct KeyConfig {
    /// Key provider (file, hsm, kms, etc.)
    pub provider: String,
    /// Key path or identifier
    pub key_path: String,
    /// Key rotation interval
    pub rotation_interval: Option<Duration>,
}

/// fr fr Access control configuration
#[derive(Debug, Clone)]
pub struct AccessControlConfig {
    /// Whether to enable role-based access control
    pub enable_rbac: bool,
    /// Default roles for new connections
    pub default_roles: Vec<String>,
    /// IP address whitelist
    pub ip_whitelist: Vec<String>,
    /// IP address blacklist
    pub ip_blacklist: Vec<String>,
    /// Maximum failed login attempts
    pub max_failed_attempts: u32,
    /// Lockout duration after failed attempts
    pub lockout_duration: Duration,
}

/// fr fr Audit configuration
#[derive(Debug, Clone)]
pub struct AuditConfig {
    /// Whether auditing is enabled
    pub enabled: bool,
    /// What events to audit
    pub audit_events: Vec<AuditEvent>,
    /// Audit log destination
    pub log_destination: AuditDestination,
    /// Log retention period
    pub retention_period: Duration,
}

/// fr fr Audit events
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuditEvent {
    /// Connection events
    Connection,
    /// Authentication events
    Authentication,
    /// Query execution
    Query,
    /// Data modification
    DataModification,
    /// Schema changes
    SchemaChange,
    /// Permission changes
    PermissionChange,
}

/// fr fr Audit destinations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuditDestination {
    /// Write to file
    File(PathBuf),
    /// Write to database table
    Database(String),
    /// Send to syslog
    Syslog,
    /// Send to external service
    External(String),
}

/// fr fr Performance configuration
#[derive(Debug, Clone)]
pub struct PerformanceConfig {
    /// Query timeout
    pub query_timeout: Duration,
    /// Connection timeout
    pub connection_timeout: Duration,
    /// Statement cache size
    pub statement_cache_size: usize,
    /// Result set fetch size
    pub fetch_size: usize,
    /// Whether to enable query optimization
    pub enable_optimization: bool,
    /// Query plan cache size
    pub plan_cache_size: usize,
    /// Statistics collection interval
    pub stats_collection_interval: Duration,
    /// Batch size for bulk operations
    pub batch_size: usize,
    /// Memory settings
    pub memory_config: MemoryConfig,
}

/// fr fr Memory configuration
#[derive(Debug, Clone)]
pub struct MemoryConfig {
    /// Maximum memory usage for connections
    pub max_memory_per_connection: usize,
    /// Maximum memory for result sets
    pub max_result_set_memory: usize,
    /// Memory allocation strategy
    pub allocation_strategy: MemoryAllocationStrategy,
}

/// fr fr Memory allocation strategies
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemoryAllocationStrategy {
    /// Allocate upfront
    Eager,
    /// Allocate as needed
    Lazy,
    /// Use memory pools
    Pooled,
}

/// fr fr Logging configuration
#[derive(Debug, Clone)]
pub struct LoggingConfig {
    /// Whether logging is enabled
    pub enabled: bool,
    /// Log level
    pub level: LogLevel,
    /// Log destination
    pub destination: LogDestination,
    /// Log format
    pub format: LogFormat,
    /// Whether to log SQL statements
    pub log_statements: bool,
    /// Whether to log parameters
    pub log_parameters: bool,
    /// Whether to log execution times
    pub log_execution_times: bool,
    /// Whether to log errors
    pub log_errors: bool,
    /// Log rotation settings
    pub rotation_config: LogRotationConfig,
}

/// fr fr Log levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    /// Trace level
    Trace,
    /// Debug level
    Debug,
    /// Info level
    Info,
    /// Warning level
    Warn,
    /// Error level
    Error,
}

/// fr fr Log destinations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogDestination {
    /// Standard output
    Stdout,
    /// Standard error
    Stderr,
    /// File
    File(PathBuf),
    /// Syslog
    Syslog,
    /// Custom destination
    Custom(String),
}

/// fr fr Log formats
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogFormat {
    /// Plain text
    Text,
    /// JSON format
    Json,
    /// Structured format
    Structured,
    /// Custom format
    Custom(String),
}

/// fr fr Log rotation configuration
#[derive(Debug, Clone)]
pub struct LogRotationConfig {
    /// Whether rotation is enabled
    pub enabled: bool,
    /// Maximum file size before rotation
    pub max_file_size: usize,
    /// Maximum number of files to keep
    pub max_files: usize,
    /// Rotation interval
    pub rotation_interval: Option<Duration>,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            driver_config: DriverConfig::default(),
            pool_config: PoolConfig::default(),
            security_config: SecurityConfig::default(),
            performance_config: PerformanceConfig::default(),
            logging_config: LoggingConfig::default(),
            custom_properties: HashMap::new(),
        }
    }
}

impl Default for DriverConfig {
    fn default() -> Self {
        Self {
            driver_name: "postgresql".to_string(),
            driver_version: None,
            properties: HashMap::new(),
            connection_template: None,
            default_database: None,
            init_parameters: HashMap::new(),
        }
    }
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            min_connections: 1,
            max_connections: 10,
            initial_connections: 1,
            acquire_timeout: Duration::from_secs(30),
            max_lifetime: Duration::from_secs(1800), // 30 minutes
            idle_timeout: Duration::from_secs(600),  // 10 minutes
            validation_interval: Duration::from_secs(300), // 5 minutes
            validation_query: Some("SELECT 1".to_string()),
            test_on_borrow: true,
            test_on_return: false,
            enabled: true,
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            ssl_config: SslConfig::default(),
            auth_config: AuthConfig::default(),
            encryption_config: EncryptionConfig::default(),
            access_control: AccessControlConfig::default(),
            audit_config: AuditConfig::default(),
        }
    }
}

impl Default for SslConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            mode: SslMode::Prefer,
            client_cert_path: None,
            client_key_path: None,
            ca_cert_path: None,
            verify_server_cert: true,
            verify_hostname: true,
            protocol_version: None,
            cipher_suites: Vec::new(),
        }
    }
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            method: AuthMethod::Password,
            username: None,
            password: None,
            kerberos_principal: None,
            ldap_config: None,
            oauth_config: None,
            token_config: None,
        }
    }
}

impl Default for EncryptionConfig {
    fn default() -> Self {
        Self {
            encrypt_at_rest: false,
            encrypt_in_transit: true,
            algorithm: "AES-256".to_string(),
            key_config: KeyConfig::default(),
        }
    }
}

impl Default for KeyConfig {
    fn default() -> Self {
        Self {
            provider: "file".to_string(),
            key_path: "keys/db.key".to_string(),
            rotation_interval: Some(Duration::from_secs(86400 * 30)), // 30 days
        }
    }
}

impl Default for AccessControlConfig {
    fn default() -> Self {
        Self {
            enable_rbac: false,
            default_roles: Vec::new(),
            ip_whitelist: Vec::new(),
            ip_blacklist: Vec::new(),
            max_failed_attempts: 3,
            lockout_duration: Duration::from_secs(900), // 15 minutes
        }
    }
}

impl Default for AuditConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            audit_events: vec![
                AuditEvent::Authentication,
                AuditEvent::DataModification,
                AuditEvent::SchemaChange,
            ],
            log_destination: AuditDestination::File(PathBuf::from("audit.log")),
            retention_period: Duration::from_secs(86400 * 90), // 90 days
        }
    }
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            query_timeout: Duration::from_secs(30),
            connection_timeout: Duration::from_secs(10),
            statement_cache_size: 100,
            fetch_size: 1000,
            enable_optimization: true,
            plan_cache_size: 500,
            stats_collection_interval: Duration::from_secs(60),
            batch_size: 1000,
            memory_config: MemoryConfig::default(),
        }
    }
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            max_memory_per_connection: 64 * 1024 * 1024, // 64MB
            max_result_set_memory: 32 * 1024 * 1024,     // 32MB
            allocation_strategy: MemoryAllocationStrategy::Lazy,
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            level: LogLevel::Info,
            destination: LogDestination::Stdout,
            format: LogFormat::Text,
            log_statements: false,
            log_parameters: false,
            log_execution_times: true,
            log_errors: true,
            rotation_config: LogRotationConfig::default(),
        }
    }
}

impl Default for LogRotationConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            max_file_size: 100 * 1024 * 1024, // 100MB
            max_files: 10,
            rotation_interval: Some(Duration::from_secs(86400)), // 1 day
        }
    }
}

// Configuration builders for easier setup
impl DatabaseConfig {
    /// slay Create new database config
    pub fn new() -> Self {
        Self::default()
    }

    /// slay Set driver configuration
    pub fn with_driver(mut self, driver_config: DriverConfig) -> Self {
        self.driver_config = driver_config;
        self
    }

    /// slay Set pool configuration
    pub fn with_pool(mut self, pool_config: PoolConfig) -> Self {
        self.pool_config = pool_config;
        self
    }

    /// slay Set security configuration
    pub fn with_security(mut self, security_config: SecurityConfig) -> Self {
        self.security_config = security_config;
        self
    }

    /// slay Add custom property
    pub fn with_property(mut self, key: &str, value: &str) -> Self {
        self.custom_properties.insert(key.to_string(), value.to_string());
        self
    }
}

impl DriverConfig {
    /// slay Create new driver config
    pub fn new(driver_name: &str) -> Self {
        Self {
            driver_name: driver_name.to_string(),
            ..Default::default()
        }
    }

    /// slay Set driver version
    pub fn with_version(mut self, version: &str) -> Self {
        self.driver_version = Some(version.to_string());
        self
    }

    /// slay Add driver property
    pub fn with_property(mut self, key: &str, value: &str) -> Self {
        self.properties.insert(key.to_string(), value.to_string());
        self
    }
}

impl PoolConfig {
    /// slay Create new pool config
    pub fn new() -> Self {
        Self::default()
    }

    /// slay Set pool size
    pub fn with_size(mut self, min: usize, max: usize) -> Self {
        self.min_connections = min;
        self.max_connections = max;
        self
    }

    /// slay Set timeouts
    pub fn with_timeouts(mut self, acquire: Duration, idle: Duration) -> Self {
        self.acquire_timeout = acquire;
        self.idle_timeout = idle;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_config_default() {
        let config = DatabaseConfig::default();
        assert_eq!(config.driver_config.driver_name, "postgresql");
        assert!(config.pool_config.enabled);
        assert!(!config.security_config.ssl_config.enabled);
    }

    #[test]
    fn test_driver_config_builder() {
        let config = DriverConfig::new("mysql")
            .with_version("8.0")
            .with_property("charset", "utf8mb4");

        assert_eq!(config.driver_name, "mysql");
        assert_eq!(config.driver_version, Some("8.0".to_string()));
        assert_eq!(config.properties.get("charset"), Some(&"utf8mb4".to_string()));
    }

    #[test]
    fn test_pool_config_builder() {
        let config = PoolConfig::new()
            .with_size(5, 20)
            .with_timeouts(Duration::from_secs(15), Duration::from_secs(300));

        assert_eq!(config.min_connections, 5);
        assert_eq!(config.max_connections, 20);
        assert_eq!(config.acquire_timeout, Duration::from_secs(15));
        assert_eq!(config.idle_timeout, Duration::from_secs(300));
    }

    #[test]
    fn test_ssl_modes() {
        assert_eq!(SslMode::Disable, SslMode::Disable);
        assert_ne!(SslMode::Disable, SslMode::Require);
    }

    #[test]
    fn test_auth_methods() {
        assert_eq!(AuthMethod::Password, AuthMethod::Password);
        assert_ne!(AuthMethod::Password, AuthMethod::Kerberos);
    }

    #[test]
    fn test_log_levels() {
        assert!(LogLevel::Error > LogLevel::Warn);
        assert!(LogLevel::Warn > LogLevel::Info);
        assert!(LogLevel::Info > LogLevel::Debug);
        assert!(LogLevel::Debug > LogLevel::Trace);
    }

    #[test]
    fn test_memory_config() {
        let config = MemoryConfig::default();
        assert_eq!(config.allocation_strategy, MemoryAllocationStrategy::Lazy);
        assert_eq!(config.max_memory_per_connection, 64 * 1024 * 1024);
    }

    #[test]
    fn test_audit_config() {
        let config = AuditConfig::default();
        assert!(!config.enabled);
        assert!(config.audit_events.contains(&AuditEvent::Authentication));
        assert!(config.audit_events.contains(&AuditEvent::DataModification));
    }

    #[test]
    fn test_database_config_builder() {
        let driver_config = DriverConfig::new("postgresql")
            .with_property("ssl", "true");
        
        let pool_config = PoolConfig::new()
            .with_size(2, 15);

        let config = DatabaseConfig::new()
            .with_driver(driver_config)
            .with_pool(pool_config)
            .with_property("app_name", "test_app");

        assert_eq!(config.driver_config.driver_name, "postgresql");
        assert_eq!(config.pool_config.min_connections, 2);
        assert_eq!(config.pool_config.max_connections, 15);
        assert_eq!(config.custom_properties.get("app_name"), Some(&"test_app".to_string()));
    }
}
