/// fr fr Database connection management - keeping those connections fresh periodt
///
/// This module handles database connection configuration, state management,
/// and connection lifecycle. Because proper connection management is crucial bestie!

use crate::stdlib::packages::db_core::{
    DatabaseError, ErrorKind, ConnectionError,
    TransactionIsolation
};
use crate::stdlib::packages::db_core::error::{DatabaseResult as DbResult};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use url::Url;

/// fr fr Connection configuration - all the details needed to connect
#[derive(Debug, Clone)]
pub struct ConnectionConfig {
    /// Database driver name (e.g., "postgresql", "mysql", "sqlite")
    pub driver: String,
    /// Host address
    pub host: Option<String>,
    /// Port number
    pub port: Option<u16>,
    /// Database name
    pub database: String,
    /// Username
    pub username: Option<String>,
    /// Password
    pub password: Option<String>,
    /// Connection options
    pub options: ConnectionOptions,
    /// SSL configuration
    pub ssl_config: Option<SslConfig>,
    /// Connection timeout
    pub connect_timeout: Option<Duration>,
    /// Query timeout
    pub query_timeout: Option<Duration>,
    /// Additional parameters
    pub parameters: HashMap<String, String>,
}

/// fr fr Connection options for fine-tuning
#[derive(Debug, Clone)]
pub struct ConnectionOptions {
    /// Maximum number of connections in pool
    pub max_connections: Option<usize>,
    /// Minimum number of connections in pool
    pub min_connections: Option<usize>,
    /// Connection idle timeout
    pub idle_timeout: Option<Duration>,
    /// Maximum connection lifetime
    pub max_lifetime: Option<Duration>,
    /// Enable connection validation
    pub validate_connections: bool,
    /// Default transaction isolation
    pub default_isolation: Option<TransactionIsolation>,
    /// Enable auto-commit
    pub auto_commit: bool,
    /// Statement cache size
    pub statement_cache_size: Option<usize>,
    /// Enable read-only mode
    pub read_only: bool,
    /// Connection charset/encoding
    pub charset: Option<String>,
    /// Time zone
    pub timezone: Option<String>,
}

/// fr fr SSL configuration for secure connections
#[derive(Debug, Clone)]
pub struct SslConfig {
    /// SSL mode (require, prefer, disable, etc.)
    pub mode: SslMode,
    /// Certificate file path
    pub cert_file: Option<String>,
    /// Private key file path
    pub key_file: Option<String>,
    /// CA certificate file path
    pub ca_file: Option<String>,
    /// Verify server certificate
    pub verify_server: bool,
    /// Verify certificate hostname
    pub verify_hostname: bool,
}

/// fr fr SSL modes for connection security
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

/// fr fr Connection state tracking
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionState {
    /// Connection is being established
    Connecting,
    /// Connection is active and ready
    Active,
    /// Connection is idle in pool
    Idle,
    /// Connection is in transaction
    InTransaction,
    /// Connection is being validated
    Validating,
    /// Connection is closed
    Closed,
    /// Connection failed
    Failed(String),
}

/// fr fr Connection information and metadata
#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    /// Unique connection identifier
    pub id: String,
    /// Database server information
    pub server_info: ServerInfo,
    /// Connection state
    pub state: ConnectionState,
    /// When connection was established
    pub connected_at: SystemTime,
    /// Last activity timestamp
    pub last_activity: SystemTime,
    /// Current transaction ID (if any)
    pub transaction_id: Option<String>,
    /// Connection statistics
    pub stats: ConnectionStats,
}

/// fr fr Database server information
#[derive(Debug, Clone)]
pub struct ServerInfo {
    /// Server version
    pub version: String,
    /// Server name/type
    pub name: String,
    /// Protocol version
    pub protocol_version: String,
    /// Server capabilities
    pub capabilities: Vec<String>,
    /// Server configuration
    pub config: HashMap<String, String>,
}

/// fr fr Connection statistics
#[derive(Debug, Clone, Default)]
pub struct ConnectionStats {
    /// Number of queries executed
    pub queries_executed: u64,
    /// Number of transactions started
    pub transactions_started: u64,
    /// Number of transactions committed
    pub transactions_committed: u64,
    /// Number of transactions rolled back
    pub transactions_rolled_back: u64,
    /// Total bytes sent
    pub bytes_sent: u64,
    /// Total bytes received
    pub bytes_received: u64,
    /// Total connection time
    pub total_time: Duration,
    /// Last error (if any)
    pub last_error: Option<String>,
}

/// fr fr Database connection implementation
#[derive(Debug)]
pub struct DatabaseConnectionImpl {
    /// Connection configuration
    config: ConnectionConfig,
    /// Connection information
    info: ConnectionInfo,
    /// Internal connection handle (driver-specific)
    handle: Option<Box<dyn std::any::Any + Send + Sync>>,
}

impl ConnectionConfig {
    /// slay Create a new connection config
    pub fn new(driver: &str, database: &str) -> Self {
        Self {
            driver: driver.to_string(),
            host: None,
            port: None,
            database: database.to_string(),
            username: None,
            password: None,
            options: ConnectionOptions::default(),
            ssl_config: None,
            connect_timeout: None,
            query_timeout: None,
            parameters: HashMap::new(),
        }
    }

    /// slay Create config from connection string
    pub fn from_string(connection_string: &str) -> DbResult<Self> {
        let url = Url::parse(connection_string)
            .map_err(|e| DatabaseError::connection(
                ConnectionError::InvalidConnectionString,
                &format!("Invalid connection string: {}", e)
            ))?;

        let driver = url.scheme().to_string();
        let database = url.path().trim_start_matches('/').to_string();
        
        if database.is_empty() {
            return Err(DatabaseError::connection(
                ConnectionError::InvalidConnectionString,
                "Database name is required"
            ));
        }

        let mut config = Self::new(&driver, &database);
        
        // Set host and port
        if let Some(host) = url.host_str() {
            config.host = Some(host.to_string());
        }
        
        config.port = url.port();
        
        // Set credentials
        config.username = Some(url.username().to_string()).filter(|u| !u.is_empty());
        config.password = url.password().map(|p| p.to_string());
        
        // Parse query parameters
        for (key, value) in url.query_pairs() {
            config.parameters.insert(key.to_string(), value.to_string());
        }
        
        // Apply common parameters
        config.apply_parameters()?;
        
        Ok(config)
    }

    /// slay Apply parameters to configuration
    fn apply_parameters(&mut self) -> DbResult<()> {
        // Connection timeout
        if let Some(timeout_str) = self.parameters.get("connect_timeout") {
            let timeout_secs: u64 = timeout_str.parse()
                .map_err(|_| DatabaseError::config("Invalid connect_timeout value"))?;
            self.connect_timeout = Some(Duration::from_secs(timeout_secs));
        }

        // Query timeout
        if let Some(timeout_str) = self.parameters.get("query_timeout") {
            let timeout_secs: u64 = timeout_str.parse()
                .map_err(|_| DatabaseError::config("Invalid query_timeout value"))?;
            self.query_timeout = Some(Duration::from_secs(timeout_secs));
        }

        // SSL mode
        if let Some(ssl_mode) = self.parameters.get("sslmode") {
            let ssl_config = SslConfig {
                mode: match ssl_mode.as_str() {
                    "disable" => SslMode::Disable,
                    "allow" => SslMode::Allow,
                    "prefer" => SslMode::Prefer,
                    "require" => SslMode::Require,
                    "verify-ca" => SslMode::VerifyCA,
                    "verify-full" => SslMode::VerifyFull,
                    _ => return Err(DatabaseError::config("Invalid SSL mode")),
                },
                cert_file: self.parameters.get("sslcert").cloned(),
                key_file: self.parameters.get("sslkey").cloned(),
                ca_file: self.parameters.get("sslrootcert").cloned(),
                verify_server: ssl_mode != "disable",
                verify_hostname: ssl_mode == "verify-full",
            };
            self.ssl_config = Some(ssl_config);
        }

        // Auto-commit
        if let Some(auto_commit_str) = self.parameters.get("autocommit") {
            self.options.auto_commit = auto_commit_str.parse()
                .unwrap_or(true);
        }

        // Read-only
        if let Some(read_only_str) = self.parameters.get("readonly") {
            self.options.read_only = read_only_str.parse()
                .unwrap_or(false);
        }

        // Charset
        if let Some(charset) = self.parameters.get("charset") {
            self.options.charset = Some(charset.clone());
        }

        Ok(())
    }

    /// slay Set host and port
    pub fn with_host(mut self, host: &str, port: u16) -> Self {
        self.host = Some(host.to_string());
        self.port = Some(port);
        self
    }

    /// slay Set credentials
    pub fn with_credentials(mut self, username: &str, password: &str) -> Self {
        self.username = Some(username.to_string());
        self.password = Some(password.to_string());
        self
    }

    /// slay Add connection parameter
    pub fn with_parameter(mut self, key: &str, value: &str) -> Self {
        self.parameters.insert(key.to_string(), value.to_string());
        self
    }

    /// slay Convert to connection string
    pub fn to_connection_string(&self) -> String {
        let mut url = format!("{}://", self.driver);
        
        // Add credentials
        if let Some(username) = &self.username {
            url.push_str(username);
            if let Some(password) = &self.password {
                url.push(':');
                url.push_str(password);
            }
            url.push('@');
        }
        
        // Add host and port
        if let Some(host) = &self.host {
            url.push_str(host);
            if let Some(port) = self.port {
                url.push(':');
                url.push_str(&port.to_string());
            }
        }
        
        // Add database
        url.push('/');
        url.push_str(&self.database);
        
        // Add parameters
        if !self.parameters.is_empty() {
            url.push('?');
            let params: Vec<String> = self.parameters.iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect();
            url.push_str(&params.join("&"));
        }
        
        url
    }
}

impl ConnectionOptions {
    /// slay Create default connection options
    pub fn new() -> Self {
        Self::default()
    }

    /// slay Set connection pool size
    pub fn with_pool_size(mut self, min: usize, max: usize) -> Self {
        self.min_connections = Some(min);
        self.max_connections = Some(max);
        self
    }

    /// slay Set timeouts
    pub fn with_timeouts(mut self, idle: Duration, max_lifetime: Duration) -> Self {
        self.idle_timeout = Some(idle);
        self.max_lifetime = Some(max_lifetime);
        self
    }
}

impl Default for ConnectionOptions {
    fn default() -> Self {
        Self {
            max_connections: Some(10),
            min_connections: Some(1),
            idle_timeout: Some(Duration::from_secs(300)), // 5 minutes
            max_lifetime: Some(Duration::from_secs(1800)), // 30 minutes
            validate_connections: true,
            default_isolation: Some(TransactionIsolation::ReadCommitted),
            auto_commit: true,
            statement_cache_size: Some(100),
            read_only: false,
            charset: None,
            timezone: None,
        }
    }
}

impl ConnectionInfo {
    /// slay Create new connection info
    pub fn new(id: String, server_info: ServerInfo) -> Self {
        let now = SystemTime::now();
        Self {
            id,
            server_info,
            state: ConnectionState::Connecting,
            connected_at: now,
            last_activity: now,
            transaction_id: None,
            stats: ConnectionStats::default(),
        }
    }

    /// slay Update last activity timestamp
    pub fn update_activity(&mut self) {
        self.last_activity = SystemTime::now();
    }

    /// slay Check if connection is stale
    pub fn is_stale(&self, max_age: Duration) -> bool {
        self.last_activity.elapsed().unwrap_or(Duration::ZERO) > max_age
    }
}

impl ConnectionStats {
    /// slay Record query execution
    pub fn record_query(&mut self) {
        self.queries_executed += 1;
    }

    /// slay Record transaction start
    pub fn record_transaction_start(&mut self) {
        self.transactions_started += 1;
    }

    /// slay Record transaction commit
    pub fn record_transaction_commit(&mut self) {
        self.transactions_committed += 1;
    }

    /// slay Record transaction rollback
    pub fn record_transaction_rollback(&mut self) {
        self.transactions_rolled_back += 1;
    }

    /// slay Record data transfer
    pub fn record_data_transfer(&mut self, sent: u64, received: u64) {
        self.bytes_sent += sent;
        self.bytes_received += received;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_config_creation() {
        let config = ConnectionConfig::new("postgresql", "test_db")
            .with_host("localhost", 5432)
            .with_credentials("user", "password")
            .with_parameter("sslmode", "require");

        assert_eq!(config.driver, "postgresql");
        assert_eq!(config.database, "test_db");
        assert_eq!(config.host, Some("localhost".to_string()));
        assert_eq!(config.port, Some(5432));
        assert_eq!(config.username, Some("user".to_string()));
        assert_eq!(config.password, Some("password".to_string()));
    }

    #[test]
    fn test_connection_string_parsing() {
        let connection_string = "postgresql://user:password@localhost:5432/test_db?sslmode=require&connect_timeout=30";
        let config = ConnectionConfig::from_string(connection_string).unwrap();

        assert_eq!(config.driver, "postgresql");
        assert_eq!(config.database, "test_db");
        assert_eq!(config.host, Some("localhost".to_string()));
        assert_eq!(config.port, Some(5432));
        assert_eq!(config.username, Some("user".to_string()));
        assert_eq!(config.password, Some("password".to_string()));
        assert_eq!(config.connect_timeout, Some(Duration::from_secs(30)));
    }

    #[test]
    fn test_connection_string_generation() {
        let config = ConnectionConfig::new("postgresql", "test_db")
            .with_host("localhost", 5432)
            .with_credentials("user", "password")
            .with_parameter("sslmode", "require");

        let connection_string = config.to_connection_string();
        assert!(connection_string.contains("postgresql://"));
        assert!(connection_string.contains("user:password@"));
        assert!(connection_string.contains("localhost:5432"));
        assert!(connection_string.contains("/test_db"));
        assert!(connection_string.contains("sslmode=require"));
    }

    #[test]
    fn test_connection_options() {
        let options = ConnectionOptions::new()
            .with_pool_size(5, 20)
            .with_timeouts(Duration::from_secs(60), Duration::from_secs(3600));

        assert_eq!(options.min_connections, Some(5));
        assert_eq!(options.max_connections, Some(20));
        assert_eq!(options.idle_timeout, Some(Duration::from_secs(60)));
        assert_eq!(options.max_lifetime, Some(Duration::from_secs(3600)));
    }

    #[test]
    fn test_connection_stats() {
        let mut stats = ConnectionStats::default();
        
        stats.record_query();
        stats.record_transaction_start();
        stats.record_transaction_commit();
        stats.record_data_transfer(100, 200);

        assert_eq!(stats.queries_executed, 1);
        assert_eq!(stats.transactions_started, 1);
        assert_eq!(stats.transactions_committed, 1);
        assert_eq!(stats.bytes_sent, 100);
        assert_eq!(stats.bytes_received, 200);
    }

    #[test]
    fn test_invalid_connection_string() {
        let result = ConnectionConfig::from_string("invalid://");
        assert!(result.is_err());

        let result = ConnectionConfig::from_string("postgresql://localhost/");
        assert!(result.is_err()); // Empty database name
    }
}
