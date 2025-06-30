//! PostgreSQL configuration implementation

use crate::error::CursedError;
use std::collections::HashMap;
use std::fmt;

/// Result type for PostgreSQL configuration operations
pub type PostgresConfigResult<T> = Result<T, CursedError>;

/// PostgreSQL configuration
#[derive(Debug, Clone)]
pub struct PostgresConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
    pub ssl_mode: SslMode,
    pub connect_timeout: Option<u64>,
    pub query_timeout: Option<u64>,
    pub application_name: Option<String>,
    pub search_path: Option<String>,
    pub timezone: Option<String>,
    pub options: HashMap<String, String>,
}

/// PostgreSQL connection string builder
#[derive(Debug, Clone)]
pub struct PostgresConnectionString {
    config: PostgresConfig,
}

/// SSL mode for PostgreSQL connections
#[derive(Debug, Clone, PartialEq)]
pub enum SslMode {
    Disable,
    Allow,
    Prefer,
    Require,
    VerifyCa,
    VerifyFull,
}

impl PostgresConfig {
    /// Create a new PostgreSQL configuration
    pub fn new() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 5432,
            database: "postgres".to_string(),
            username: "postgres".to_string(),
            password: String::new(),
            ssl_mode: SslMode::Prefer,
            connect_timeout: None,
            query_timeout: None,
            application_name: None,
            search_path: None,
            timezone: None,
            options: HashMap::new(),
        }
    }
    
    /// Set the host
    pub fn host(mut self, host: &str) -> Self {
        self.host = host.to_string();
        self
    }
    
    /// Set the port
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }
    
    /// Set the database name
    pub fn database(mut self, database: &str) -> Self {
        self.database = database.to_string();
        self
    }
    
    /// Set the username
    pub fn username(mut self, username: &str) -> Self {
        self.username = username.to_string();
        self
    }
    
    /// Set the password
    pub fn password(mut self, password: &str) -> Self {
        self.password = password.to_string();
        self
    }
    
    /// Set the SSL mode
    pub fn ssl_mode(mut self, ssl_mode: SslMode) -> Self {
        self.ssl_mode = ssl_mode;
        self
    }
    
    /// Set the connection timeout
    pub fn connect_timeout(mut self, timeout: u64) -> Self {
        self.connect_timeout = Some(timeout);
        self
    }
    
    /// Set the query timeout
    pub fn query_timeout(mut self, timeout: u64) -> Self {
        self.query_timeout = Some(timeout);
        self
    }
    
    /// Set the application name
    pub fn application_name(mut self, name: &str) -> Self {
        self.application_name = Some(name.to_string());
        self
    }
    
    /// Set the search path
    pub fn search_path(mut self, path: &str) -> Self {
        self.search_path = Some(path.to_string());
        self
    }
    
    /// Set the timezone
    pub fn timezone(mut self, timezone: &str) -> Self {
        self.timezone = Some(timezone.to_string());
        self
    }
    
    /// Add a custom option
    pub fn option(mut self, key: &str, value: &str) -> Self {
        self.options.insert(key.to_string(), value.to_string());
        self
    }
    
    /// Validate the configuration
    pub fn validate(&self) -> PostgresConfigResult<()> {
        if self.host.is_empty() {
            return Err(CursedError::runtime_error("Host cannot be empty"));
        }
        
        if self.port == 0 {
            return Err(CursedError::runtime_error("Port must be greater than 0"));
        }
        
        if self.database.is_empty() {
            return Err(CursedError::runtime_error("Database name cannot be empty"));
        }
        
        if self.username.is_empty() {
            return Err(CursedError::runtime_error("Username cannot be empty"));
        }
        
        Ok(())
    }
    
    /// Build a connection string
    pub fn connection_string(&self) -> PostgresConnectionString {
        PostgresConnectionString::new(self.clone())
    }
}

impl Default for PostgresConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl PostgresConnectionString {
    /// Create a new connection string builder
    pub fn new(config: PostgresConfig) -> Self {
        Self {
            config,
        }
    }
    
    /// Parse a connection string into a config
    pub fn parse(connection_string: &str) -> PostgresConfigResult<PostgresConfig> {
        let mut config = PostgresConfig::new();
        
        // Simple parsing - in a real implementation, this would be more robust
        if connection_string.starts_with("postgresql://") || connection_string.starts_with("postgres://") {
            // URL format: postgresql://user:password@host:port/database?options
            if let Some(url_part) = connection_string.strip_prefix("postgresql://")
                .or_else(|| connection_string.strip_prefix("postgres://")) {
                
                let parts: Vec<&str> = url_part.split('@').collect();
                if parts.len() == 2 {
                    // Parse user:password
                    let auth_parts: Vec<&str> = parts[0].split(':').collect();
                    if !auth_parts.is_empty() {
                        config.username = auth_parts[0].to_string();
                        if auth_parts.len() > 1 {
                            config.password = auth_parts[1].to_string();
                        }
                    }
                    
                    // Parse host:port/database
                    let host_db_parts: Vec<&str> = parts[1].split('/').collect();
                    if !host_db_parts.is_empty() {
                        let host_port_parts: Vec<&str> = host_db_parts[0].split(':').collect();
                        config.host = host_port_parts[0].to_string();
                        if host_port_parts.len() > 1 {
                            if let Ok(port) = host_port_parts[1].parse::<u16>() {
                                config.port = port;
                            }
                        }
                        
                        if host_db_parts.len() > 1 {
                            let db_query_parts: Vec<&str> = host_db_parts[1].split('?').collect();
                            config.database = db_query_parts[0].to_string();
                            
                            // Parse query parameters
                            if db_query_parts.len() > 1 {
                                for param in db_query_parts[1].split('&') {
                                    let param_parts: Vec<&str> = param.split('=').collect();
                                    if param_parts.len() == 2 {
                                        match param_parts[0] {
                                            "sslmode" => {
                                                config.ssl_mode = SslMode::from_str(param_parts[1]);
                                            }
                                            "connect_timeout" => {
                                                if let Ok(timeout) = param_parts[1].parse::<u64>() {
                                                    config.connect_timeout = Some(timeout);
                                                }
                                            }
                                            "application_name" => {
                                                config.application_name = Some(param_parts[1].to_string());
                                            }
                                            _ => {
                                                config.options.insert(param_parts[0].to_string(), param_parts[1].to_string());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            // Key-value format: host=localhost port=5432 dbname=mydb user=postgres
            for pair in connection_string.split_whitespace() {
                let parts: Vec<&str> = pair.split('=').collect();
                if parts.len() == 2 {
                    match parts[0] {
                        "host" => config.host = parts[1].to_string(),
                        "port" => {
                            if let Ok(port) = parts[1].parse::<u16>() {
                                config.port = port;
                            }
                        }
                        "dbname" => config.database = parts[1].to_string(),
                        "user" => config.username = parts[1].to_string(),
                        "password" => config.password = parts[1].to_string(),
                        "sslmode" => config.ssl_mode = SslMode::from_str(parts[1]),
                        _ => {
                            config.options.insert(parts[0].to_string(), parts[1].to_string());
                        }
                    }
                }
            }
        }
        
        config.validate()?;
        Ok(config)
    }
    
    /// Build the connection string
    pub fn build(&self) -> String {
        let mut conn_str = format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.config.username,
            self.config.password,
            self.config.host,
            self.config.port,
            self.config.database
        );
        
        let mut params = Vec::new();
        
        // Add SSL mode
        if self.config.ssl_mode != SslMode::Prefer {
            params.push(format!("sslmode={}", self.config.ssl_mode.to_string()));
        }
        
        // Add timeouts
        if let Some(timeout) = self.config.connect_timeout {
            params.push(format!("connect_timeout={}", timeout));
        }
        
        if let Some(ref app_name) = self.config.application_name {
            params.push(format!("application_name={}", app_name));
        }
        
        if let Some(ref search_path) = self.config.search_path {
            params.push(format!("search_path={}", search_path));
        }
        
        if let Some(ref timezone) = self.config.timezone {
            params.push(format!("timezone={}", timezone));
        }
        
        // Add custom options
        for (key, value) in &self.config.options {
            params.push(format!("{}={}", key, value));
        }
        
        if !params.is_empty() {
            conn_str.push('?');
            conn_str.push_str(&params.join("&"));
        }
        
        conn_str
    }
    
    /// Get the underlying config
    pub fn config(&self) -> &PostgresConfig {
        &self.config
    }
}

impl fmt::Display for PostgresConnectionString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.build())
    }
}

impl SslMode {
    /// Parse SSL mode from string
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "disable" => SslMode::Disable,
            "allow" => SslMode::Allow,
            "prefer" => SslMode::Prefer,
            "require" => SslMode::Require,
            "verify-ca" => SslMode::VerifyCa,
            "verify-full" => SslMode::VerifyFull,
            _ => SslMode::Prefer, // Default
        }
    }
    
    /// Convert SSL mode to string
    pub fn to_string(&self) -> String {
        match self {
            SslMode::Disable => "disable".to_string(),
            SslMode::Allow => "allow".to_string(),
            SslMode::Prefer => "prefer".to_string(),
            SslMode::Require => "require".to_string(),
            SslMode::VerifyCa => "verify-ca".to_string(),
            SslMode::VerifyFull => "verify-full".to_string(),
        }
    }
    
    /// Check if SSL is required
    pub fn requires_ssl(&self) -> bool {
        matches!(self, SslMode::Require | SslMode::VerifyCa | SslMode::VerifyFull)
    }
    
    /// Check if certificate verification is required
    pub fn requires_cert_verification(&self) -> bool {
        matches!(self, SslMode::VerifyCa | SslMode::VerifyFull)
    }
    
    /// Check if hostname verification is required
    pub fn requires_hostname_verification(&self) -> bool {
        matches!(self, SslMode::VerifyFull)
    }
}

impl fmt::Display for SslMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

/// PostgreSQL environment configuration
pub struct PostgresEnvConfig;

impl PostgresEnvConfig {
    /// Load configuration from environment variables
    pub fn from_env() -> PostgresConfig {
        let mut config = PostgresConfig::new();
        
        if let Ok(host) = std::env::var("PGHOST") {
            config.host = host;
        }
        
        if let Ok(port) = std::env::var("PGPORT") {
            if let Ok(port_num) = port.parse::<u16>() {
                config.port = port_num;
            }
        }
        
        if let Ok(database) = std::env::var("PGDATABASE") {
            config.database = database;
        }
        
        if let Ok(username) = std::env::var("PGUSER") {
            config.username = username;
        }
        
        if let Ok(password) = std::env::var("PGPASSWORD") {
            config.password = password;
        }
        
        if let Ok(ssl_mode) = std::env::var("PGSSLMODE") {
            config.ssl_mode = SslMode::from_str(&ssl_mode);
        }
        
        if let Ok(timeout) = std::env::var("PGCONNECT_TIMEOUT") {
            if let Ok(timeout_num) = timeout.parse::<u64>() {
                config.connect_timeout = Some(timeout_num);
            }
        }
        
        if let Ok(app_name) = std::env::var("PGAPPNAME") {
            config.application_name = Some(app_name);
        }
        
        config
    }
    
    /// Set environment variables from configuration
    pub fn to_env(config: &PostgresConfig) {
        std::env::set_var("PGHOST", &config.host);
        std::env::set_var("PGPORT", &config.port.to_string());
        std::env::set_var("PGDATABASE", &config.database);
        std::env::set_var("PGUSER", &config.username);
        std::env::set_var("PGPASSWORD", &config.password);
        std::env::set_var("PGSSLMODE", &config.ssl_mode.to_string());
        
        if let Some(timeout) = config.connect_timeout {
            std::env::set_var("PGCONNECT_TIMEOUT", &timeout.to_string());
        }
        
        if let Some(ref app_name) = config.application_name {
            std::env::set_var("PGAPPNAME", app_name);
        }
    }
}
