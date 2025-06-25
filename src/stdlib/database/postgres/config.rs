/// PostgreSQL Configuration and Connection String Parsing
/// 
/// Provides comprehensive configuration management for PostgreSQL connections
/// including SSL/TLS settings, timeouts, and connection parameters.

use std::collections::HashMap;
use std::time::Duration;
use super::error::{PostgresError, PostgresErrorKind};

/// SSL/TLS connection mode for PostgreSQL
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SslMode {
    /// Do not use SSL
    /// Use SSL if available, but don't verify certificates
    /// Prefer SSL connections but allow non-SSL
    /// Require SSL connection
    /// Require SSL and verify that server certificate is issued by trusted CA
    /// Require SSL, verify CA, and verify that server certificate matches hostname
impl Default for SslMode {
    fn default() -> Self {
        SslMode::Prefer
    }
}

impl std::fmt::Display for SslMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
        }
    }
impl std::str::FromStr for SslMode {
    type Err = PostgresError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            _ => Err(PostgresError::new(
        }
    }
/// Comprehensive PostgreSQL configuration
#[derive(Debug, Clone)]
pub struct PostgresConfig {
    /// Database host (default: localhost)
    /// Database port (default: 5432)
    /// Database name
    /// Username for authentication
    /// Password for authentication
    /// SSL/TLS mode
    /// Connection timeout
    /// Query timeout
    /// Application name for connection identification
    /// Additional connection parameters
    /// Maximum number of connections in pool
    /// Minimum number of connections in pool
    /// Maximum lifetime of a connection
    /// Maximum idle time for a connection
    /// Connection retry attempts
    /// Retry delay between attempts
impl Default for PostgresConfig {
    fn default() -> Self {
        Self {
            max_lifetime: Some(Duration::from_secs(3600)), // 1 hour
            idle_timeout: Some(Duration::from_secs(600)),   // 10 minutes
        }
    }
impl PostgresConfig {
    /// Create new configuration with required parameters
    pub fn new<S: Into<String>>(host: S, port: u16, database: S, username: S) -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Set password for authentication
    pub fn with_password<S: Into<String>>(mut self, password: S) -> Self {
        self.password = Some(password.into());
        self
    /// Set SSL mode
    pub fn with_ssl_mode(mut self, ssl_mode: SslMode) -> Self {
        self.ssl_mode = ssl_mode;
        self
    /// Set connection timeout
    pub fn with_connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = timeout;
        self
    /// Set query timeout
    pub fn with_query_timeout(mut self, timeout: Duration) -> Self {
        self.query_timeout = timeout;
        self
    /// Set application name
    pub fn with_application_name<S: Into<String>>(mut self, name: S) -> Self {
        self.application_name = name.into();
        self
    /// Add connection parameter
    pub fn with_parameter<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
        self.parameters.insert(key.into(), value.into());
        self
    /// Set connection pool limits
    pub fn with_pool_limits(mut self, min: u32, max: u32) -> Self {
        self.min_connections = min;
        self.max_connections = max;
        self
    /// Convert to tokio-postgres configuration
    pub fn to_tokio_config(&self) -> tokio_postgres::Config {
        let mut config = tokio_postgres::Config::new();
        
        config.host(&self.host);
        config.port(self.port);
        config.dbname(&self.database);
        config.user(&self.username);
        
        if let Some(ref password) = self.password {
            config.password(password);
        match self.ssl_mode {
            SslMode::Disable => {
                config.ssl_mode(tokio_postgres::config::SslMode::Disable);
            }
            SslMode::Allow => {
                config.ssl_mode(tokio_postgres::config::SslMode::Allow);
            }
            SslMode::Prefer => {
                config.ssl_mode(tokio_postgres::config::SslMode::Prefer);
            }
            SslMode::Require => {
                config.ssl_mode(tokio_postgres::config::SslMode::Require);
            }
            SslMode::VerifyCa => {
                config.ssl_mode(tokio_postgres::config::SslMode::VerifyCa);
            }
            SslMode::VerifyFull => {
                config.ssl_mode(tokio_postgres::config::SslMode::VerifyFull);
            }
        }
        
        config.connect_timeout(self.connect_timeout);
        config.application_name(&self.application_name);
        
        for (key, value) in &self.parameters {
            config.options(&format!("{}={}", key, value));
        config
    /// Validate configuration
    pub fn validate(&self) -> crate::error::Result<()> {
        if self.host.is_empty() {
            return Err(PostgresError::new(
            ));
        if self.port == 0 {
            return Err(PostgresError::new(
            ));
        if self.database.is_empty() {
            return Err(PostgresError::new(
            ));
        if self.username.is_empty() {
            return Err(PostgresError::new(
            ));
        if self.max_connections == 0 {
            return Err(PostgresError::new(
            ));
        if self.min_connections > self.max_connections {
            return Err(PostgresError::new(
            ));
        Ok(())
    }
}

/// PostgreSQL connection string parser
pub struct PostgresConnectionString;

impl PostgresConnectionString {
    /// Parse PostgreSQL connection string into configuration
    /// 
    /// Supports various formats:
    /// - postgresql://user:password@host:port/database
    /// - postgres://user:password@host:port/database?param=value
    /// - host=host port=port dbname=database user=user password=password
    pub fn parse(dsn: &str) -> crate::error::Result<()> {
        // Try URL format first
        if dsn.starts_with("postgresql://") || dsn.starts_with("postgres://") {
            Self::parse_url(dsn)
        } else {
            // Try key-value format
            Self::parse_key_value(dsn)
        }
    }
    
    /// Parse URL format connection string
    fn parse_url(dsn: &str) -> crate::error::Result<()> {
        let url = url::Url::parse(dsn).map_err(|e| {
            PostgresError::new(
            )
        })?;
        
        let mut config = PostgresConfig::default();
        
        if let Some(host) = url.host_str() {
            config.host = host.to_string();
        if let Some(port) = url.port() {
            config.port = port;
        let path = url.path();
        if path.len() > 1 {
            config.database = path[1..].to_string(); // Skip leading '/'
        config.username = url.username().to_string();
        
        if let Some(password) = url.password() {
            config.password = Some(password.to_string());
        // Parse query parameters
        for (key, value) in url.query_pairs() {
            match key.as_ref() {
                "sslmode" => {
                    config.ssl_mode = value.parse()?;
                }
                "connect_timeout" => {
                    let timeout_secs: u64 = value.parse().map_err(|_| {
                        PostgresError::new(
                        )
                    })?;
                    config.connect_timeout = Duration::from_secs(timeout_secs);
                }
                "application_name" => {
                    config.application_name = value.to_string();
                }
                _ => {
                    config.parameters.insert(key.to_string(), value.to_string());
                }
            }
        config.validate()?;
        Ok(config)
    /// Parse key-value format connection string
    fn parse_key_value(dsn: &str) -> crate::error::Result<()> {
        let mut config = PostgresConfig::default();
        
        for pair in dsn.split_whitespace() {
            if let Some((key, value)) = pair.split_once('=') {
                match key {
                    "port" => {
                        config.port = value.parse().map_err(|_| {
                            PostgresError::new(
                            )
                        })?;
                    }
                    "sslmode" => {
                        config.ssl_mode = value.parse()?;
                    }
                    "connect_timeout" => {
                        let timeout_secs: u64 = value.parse().map_err(|_| {
                            PostgresError::new(
                            )
                        })?;
                        config.connect_timeout = Duration::from_secs(timeout_secs);
                    }
                    "application_name" => {
                        config.application_name = value.to_string();
                    }
                    _ => {
                        config.parameters.insert(key.to_string(), value.to_string());
                    }
                }
            }
        }
        
        config.validate()?;
        Ok(config)
    }
}

