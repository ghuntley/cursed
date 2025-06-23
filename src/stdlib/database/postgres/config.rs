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
    Disable,
    /// Use SSL if available, but don't verify certificates
    Allow,
    /// Prefer SSL connections but allow non-SSL
    Prefer,
    /// Require SSL connection
    Require,
    /// Require SSL and verify that server certificate is issued by trusted CA
    VerifyCa,
    /// Require SSL, verify CA, and verify that server certificate matches hostname
    VerifyFull,
}

impl Default for SslMode {
    fn default() -> Self {
        SslMode::Prefer
    }
}

impl std::fmt::Display for SslMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SslMode::Disable => write!(f, "disable"),
            SslMode::Allow => write!(f, "allow"),
            SslMode::Prefer => write!(f, "prefer"),
            SslMode::Require => write!(f, "require"),
            SslMode::VerifyCa => write!(f, "verify-ca"),
            SslMode::VerifyFull => write!(f, "verify-full"),
        }
    }
}

impl std::str::FromStr for SslMode {
    type Err = PostgresError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "disable" => Ok(SslMode::Disable),
            "allow" => Ok(SslMode::Allow),
            "prefer" => Ok(SslMode::Prefer),
            "require" => Ok(SslMode::Require),
            "verify-ca" => Ok(SslMode::VerifyCa),
            "verify-full" => Ok(SslMode::VerifyFull),
            _ => Err(PostgresError::new(
                PostgresErrorKind::InvalidConfiguration,
                &format!("Invalid SSL mode: {}", s),
            )),
        }
    }
}

/// Comprehensive PostgreSQL configuration
#[derive(Debug, Clone)]
pub struct PostgresConfig {
    /// Database host (default: localhost)
    pub host: String,
    /// Database port (default: 5432)
    pub port: u16,
    /// Database name
    pub database: String,
    /// Username for authentication
    pub username: String,
    /// Password for authentication
    pub password: Option<String>,
    /// SSL/TLS mode
    pub ssl_mode: SslMode,
    /// Connection timeout
    pub connect_timeout: Duration,
    /// Query timeout
    pub query_timeout: Duration,
    /// Application name for connection identification
    pub application_name: String,
    /// Additional connection parameters
    pub parameters: HashMap<String, String>,
    /// Maximum number of connections in pool
    pub max_connections: u32,
    /// Minimum number of connections in pool
    pub min_connections: u32,
    /// Maximum lifetime of a connection
    pub max_lifetime: Option<Duration>,
    /// Maximum idle time for a connection
    pub idle_timeout: Option<Duration>,
    /// Connection retry attempts
    pub retry_attempts: u32,
    /// Retry delay between attempts
    pub retry_delay: Duration,
}

impl Default for PostgresConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 5432,
            database: "postgres".to_string(),
            username: "postgres".to_string(),
            password: None,
            ssl_mode: SslMode::Prefer,
            connect_timeout: Duration::from_secs(30),
            query_timeout: Duration::from_secs(300),
            application_name: "CURSED Database Client".to_string(),
            parameters: HashMap::new(),
            max_connections: 100,
            min_connections: 10,
            max_lifetime: Some(Duration::from_secs(3600)), // 1 hour
            idle_timeout: Some(Duration::from_secs(600)),   // 10 minutes
            retry_attempts: 3,
            retry_delay: Duration::from_secs(1),
        }
    }
}

impl PostgresConfig {
    /// Create new configuration with required parameters
    pub fn new<S: Into<String>>(host: S, port: u16, database: S, username: S) -> Self {
        Self {
            host: host.into(),
            port,
            database: database.into(),
            username: username.into(),
            ..Default::default()
        }
    }

    /// Set password for authentication
    pub fn with_password<S: Into<String>>(mut self, password: S) -> Self {
        self.password = Some(password.into());
        self
    }

    /// Set SSL mode
    pub fn with_ssl_mode(mut self, ssl_mode: SslMode) -> Self {
        self.ssl_mode = ssl_mode;
        self
    }

    /// Set connection timeout
    pub fn with_connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = timeout;
        self
    }

    /// Set query timeout
    pub fn with_query_timeout(mut self, timeout: Duration) -> Self {
        self.query_timeout = timeout;
        self
    }

    /// Set application name
    pub fn with_application_name<S: Into<String>>(mut self, name: S) -> Self {
        self.application_name = name.into();
        self
    }

    /// Add connection parameter
    pub fn with_parameter<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
        self.parameters.insert(key.into(), value.into());
        self
    }

    /// Set connection pool limits
    pub fn with_pool_limits(mut self, min: u32, max: u32) -> Self {
        self.min_connections = min;
        self.max_connections = max;
        self
    }

    /// Convert to tokio-postgres configuration
    pub fn to_tokio_config(&self) -> tokio_postgres::Config {
        let mut config = tokio_postgres::Config::new();
        
        config.host(&self.host);
        config.port(self.port);
        config.dbname(&self.database);
        config.user(&self.username);
        
        if let Some(ref password) = self.password {
            config.password(password);
        }
        
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
        }
        
        config
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), Error> {
        if self.host.is_empty() {
            return Err(PostgresError::new(
                PostgresErrorKind::InvalidConfiguration,
                "Host cannot be empty",
            ));
        }
        
        if self.port == 0 {
            return Err(PostgresError::new(
                PostgresErrorKind::InvalidConfiguration,
                "Port must be greater than 0",
            ));
        }
        
        if self.database.is_empty() {
            return Err(PostgresError::new(
                PostgresErrorKind::InvalidConfiguration,
                "Database name cannot be empty",
            ));
        }
        
        if self.username.is_empty() {
            return Err(PostgresError::new(
                PostgresErrorKind::InvalidConfiguration,
                "Username cannot be empty",
            ));
        }
        
        if self.max_connections == 0 {
            return Err(PostgresError::new(
                PostgresErrorKind::InvalidConfiguration,
                "Max connections must be greater than 0",
            ));
        }
        
        if self.min_connections > self.max_connections {
            return Err(PostgresError::new(
                PostgresErrorKind::InvalidConfiguration,
                "Min connections cannot be greater than max connections",
            ));
        }
        
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
    pub fn parse(dsn: &str) -> Result<(), Error> {
        // Try URL format first
        if dsn.starts_with("postgresql://") || dsn.starts_with("postgres://") {
            Self::parse_url(dsn)
        } else {
            // Try key-value format
            Self::parse_key_value(dsn)
        }
    }
    
    /// Parse URL format connection string
    fn parse_url(dsn: &str) -> Result<(), Error> {
        let url = url::Url::parse(dsn).map_err(|e| {
            PostgresError::new(
                PostgresErrorKind::InvalidConfiguration,
                &format!("Invalid connection URL: {}", e),
            )
        })?;
        
        let mut config = PostgresConfig::default();
        
        if let Some(host) = url.host_str() {
            config.host = host.to_string();
        }
        
        if let Some(port) = url.port() {
            config.port = port;
        }
        
        let path = url.path();
        if path.len() > 1 {
            config.database = path[1..].to_string(); // Skip leading '/'
        }
        
        config.username = url.username().to_string();
        
        if let Some(password) = url.password() {
            config.password = Some(password.to_string());
        }
        
        // Parse query parameters
        for (key, value) in url.query_pairs() {
            match key.as_ref() {
                "sslmode" => {
                    config.ssl_mode = value.parse()?;
                }
                "connect_timeout" => {
                    let timeout_secs: u64 = value.parse().map_err(|_| {
                        PostgresError::new(
                            PostgresErrorKind::InvalidConfiguration,
                            "Invalid connect_timeout value",
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
        
        config.validate()?;
        Ok(config)
    }
    
    /// Parse key-value format connection string
    fn parse_key_value(dsn: &str) -> Result<(), Error> {
        let mut config = PostgresConfig::default();
        
        for pair in dsn.split_whitespace() {
            if let Some((key, value)) = pair.split_once('=') {
                match key {
                    "host" => config.host = value.to_string(),
                    "port" => {
                        config.port = value.parse().map_err(|_| {
                            PostgresError::new(
                                PostgresErrorKind::InvalidConfiguration,
                                "Invalid port value",
                            )
                        })?;
                    }
                    "dbname" | "database" => config.database = value.to_string(),
                    "user" | "username" => config.username = value.to_string(),
                    "password" => config.password = Some(value.to_string()),
                    "sslmode" => {
                        config.ssl_mode = value.parse()?;
                    }
                    "connect_timeout" => {
                        let timeout_secs: u64 = value.parse().map_err(|_| {
                            PostgresError::new(
                                PostgresErrorKind::InvalidConfiguration,
                                "Invalid connect_timeout value",
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ssl_mode_parsing() {
        assert_eq!("disable".parse::<SslMode>().unwrap(), SslMode::Disable);
        assert_eq!("prefer".parse::<SslMode>().unwrap(), SslMode::Prefer);
        assert_eq!("require".parse::<SslMode>().unwrap(), SslMode::Require);
        assert!("invalid".parse::<SslMode>().is_err());
    }

    #[test]
    fn test_config_validation() {
        let config = PostgresConfig::default();
        assert!(config.validate().is_ok());
        
        let mut invalid_config = PostgresConfig::default();
        invalid_config.host = "".to_string();
        assert!(invalid_config.validate().is_err());
    }

    #[test]
    fn test_url_parsing() {
        let dsn = "postgresql://user:pass@localhost:5432/mydb?sslmode=require";
        let config = PostgresConnectionString::parse(dsn).unwrap();
        
        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 5432);
        assert_eq!(config.database, "mydb");
        assert_eq!(config.username, "user");
        assert_eq!(config.password, Some("pass".to_string()));
        assert_eq!(config.ssl_mode, SslMode::Require);
    }

    #[test]
    fn test_key_value_parsing() {
        let dsn = "host=localhost port=5432 dbname=mydb user=user password=pass sslmode=disable";
        let config = PostgresConnectionString::parse(dsn).unwrap();
        
        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 5432);
        assert_eq!(config.database, "mydb");
        assert_eq!(config.username, "user");
        assert_eq!(config.password, Some("pass".to_string()));
        assert_eq!(config.ssl_mode, SslMode::Disable);
    }
}
