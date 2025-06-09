/// PostgreSQL connection configuration and string parsing for CURSED database operations
/// 
/// This module provides comprehensive configuration support for PostgreSQL connections
/// including URI parsing, SSL configuration, and connection parameter management.

use std::collections::HashMap;
use std::time::Duration;
use super::error::PostgreSQLError;

/// fr fr PostgreSQL-specific configuration options
#[derive(Debug, Clone)]
pub struct PostgreSQLConfig {
    /// fr fr Database host
    pub host: String,
    /// fr fr Database port
    pub port: u16,
    /// fr fr Database name
    pub dbname: String,
    /// fr fr Username
    pub user: String,
    /// fr fr Password
    pub password: Option<String>,
    /// fr fr Connection timeout
    pub connect_timeout: Duration,
    /// fr fr Query timeout
    pub query_timeout: Duration,
    /// fr fr SSL mode
    pub ssl_mode: SslMode,
    /// fr fr SSL certificate file
    pub ssl_cert: Option<String>,
    /// fr fr SSL key file
    pub ssl_key: Option<String>,
    /// fr fr SSL CA file
    pub ssl_ca: Option<String>,
    /// fr fr Application name
    pub application_name: String,
    /// fr fr Client encoding
    pub client_encoding: String,
    /// fr fr Timezone
    pub timezone: Option<String>,
    /// fr fr Statement timeout
    pub statement_timeout: Option<Duration>,
    /// fr fr Lock timeout
    pub lock_timeout: Option<Duration>,
    /// fr fr Idle in transaction timeout
    pub idle_in_transaction_timeout: Option<Duration>,
    /// fr fr Additional parameters
    pub extra_params: HashMap<String, String>,
}

impl Default for PostgreSQLConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 5432,
            dbname: "postgres".to_string(),
            user: "postgres".to_string(),
            password: None,
            connect_timeout: Duration::from_secs(30),
            query_timeout: Duration::from_secs(300),
            ssl_mode: SslMode::Prefer,
            ssl_cert: None,
            ssl_key: None,
            ssl_ca: None,
            application_name: "cursed_app".to_string(),
            client_encoding: "UTF8".to_string(),
            timezone: None,
            statement_timeout: None,
            lock_timeout: None,
            idle_in_transaction_timeout: None,
            extra_params: HashMap::new(),
        }
    }
}

impl PostgreSQLConfig {
    /// slay Create a new configuration
    pub fn new() -> Self {
        Self::default()
    }
    
    /// slay Set host
    pub fn host(mut self, host: String) -> Self {
        self.host = host;
        self
    }
    
    /// slay Set port
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }
    
    /// slay Set database name
    pub fn dbname(mut self, dbname: String) -> Self {
        self.dbname = dbname;
        self
    }
    
    /// slay Set user
    pub fn user(mut self, user: String) -> Self {
        self.user = user;
        self
    }
    
    /// slay Set password
    pub fn password(mut self, password: String) -> Self {
        self.password = Some(password);
        self
    }
    
    /// slay Set SSL mode
    pub fn ssl_mode(mut self, ssl_mode: SslMode) -> Self {
        self.ssl_mode = ssl_mode;
        self
    }
    
    /// slay Set application name
    pub fn application_name(mut self, name: String) -> Self {
        self.application_name = name;
        self
    }
    
    /// slay Set connection timeout
    pub fn connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = timeout;
        self
    }
    
    /// slay Set query timeout
    pub fn query_timeout(mut self, timeout: Duration) -> Self {
        self.query_timeout = timeout;
        self
    }
    
    /// slay Add extra parameter
    pub fn extra_param(mut self, key: String, value: String) -> Self {
        self.extra_params.insert(key, value);
        self
    }
    
    /// slay Build connection string
    pub fn to_connection_string(&self) -> String {
        let mut parts = Vec::new();
        
        parts.push(format!("host={}", self.host));
        parts.push(format!("port={}", self.port));
        parts.push(format!("dbname={}", self.dbname));
        parts.push(format!("user={}", self.user));
        
        if let Some(ref password) = self.password {
            parts.push(format!("password={}", password));
        }
        
        parts.push(format!("connect_timeout={}", self.connect_timeout.as_secs()));
        parts.push(format!("sslmode={}", self.ssl_mode.to_string()));
        parts.push(format!("application_name={}", self.application_name));
        parts.push(format!("client_encoding={}", self.client_encoding));
        
        if let Some(ref ssl_cert) = self.ssl_cert {
            parts.push(format!("sslcert={}", ssl_cert));
        }
        
        if let Some(ref ssl_key) = self.ssl_key {
            parts.push(format!("sslkey={}", ssl_key));
        }
        
        if let Some(ref ssl_ca) = self.ssl_ca {
            parts.push(format!("sslrootcert={}", ssl_ca));
        }
        
        if let Some(ref timezone) = self.timezone {
            parts.push(format!("timezone={}", timezone));
        }
        
        if let Some(statement_timeout) = self.statement_timeout {
            parts.push(format!("statement_timeout={}ms", statement_timeout.as_millis()));
        }
        
        if let Some(lock_timeout) = self.lock_timeout {
            parts.push(format!("lock_timeout={}ms", lock_timeout.as_millis()));
        }
        
        if let Some(idle_timeout) = self.idle_in_transaction_timeout {
            parts.push(format!("idle_in_transaction_session_timeout={}ms", idle_timeout.as_millis()));
        }
        
        // Add extra parameters
        for (key, value) in &self.extra_params {
            parts.push(format!("{}={}", key, value));
        }
        
        parts.join(" ")
    }
    
    /// slay Build PostgreSQL URI
    pub fn to_uri(&self) -> String {
        let mut uri = String::from("postgresql://");
        
        uri.push_str(&self.user);
        if let Some(ref password) = self.password {
            uri.push(':');
            uri.push_str(password);
        }
        
        uri.push('@');
        uri.push_str(&self.host);
        uri.push(':');
        uri.push_str(&self.port.to_string());
        uri.push('/');
        uri.push_str(&self.dbname);
        
        // Add query parameters
        let mut params = Vec::new();
        
        if self.ssl_mode != SslMode::Prefer {
            params.push(format!("sslmode={}", self.ssl_mode.to_string()));
        }
        
        if self.application_name != "cursed_app" {
            params.push(format!("application_name={}", self.application_name));
        }
        
        if self.connect_timeout != Duration::from_secs(30) {
            params.push(format!("connect_timeout={}", self.connect_timeout.as_secs()));
        }
        
        for (key, value) in &self.extra_params {
            params.push(format!("{}={}", key, value));
        }
        
        if !params.is_empty() {
            uri.push('?');
            uri.push_str(&params.join("&"));
        }
        
        uri
    }
}

/// fr fr SSL connection modes for PostgreSQL
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SslMode {
    /// Disable SSL
    Disable,
    /// Allow SSL but don't require it
    Allow,
    /// Prefer SSL but fall back to non-SSL
    Prefer,
    /// Require SSL
    Require,
    /// Require SSL and verify server certificate
    VerifyCa,
    /// Require SSL and verify server certificate and hostname
    VerifyFull,
}

impl SslMode {
    /// slay Convert to PostgreSQL string
    pub fn to_string(&self) -> &'static str {
        match self {
            Self::Disable => "disable",
            Self::Allow => "allow",
            Self::Prefer => "prefer",
            Self::Require => "require",
            Self::VerifyCa => "verify-ca",
            Self::VerifyFull => "verify-full",
        }
    }
    
    /// slay Parse from string
    pub fn from_string(s: &str) -> Result<Self, PostgreSQLError> {
        match s.to_lowercase().as_str() {
            "disable" => Ok(Self::Disable),
            "allow" => Ok(Self::Allow),
            "prefer" => Ok(Self::Prefer),
            "require" => Ok(Self::Require),
            "verify-ca" => Ok(Self::VerifyCa),
            "verify-full" => Ok(Self::VerifyFull),
            _ => Err(PostgreSQLError::new(
                super::super::DatabaseErrorKind::ConfigurationError,
                format!("Invalid SSL mode: {}", s)
            )),
        }
    }
}

/// fr fr Connection string parser for PostgreSQL
#[derive(Debug, Clone)]
pub struct ConnectionString {
    /// fr fr Raw connection string
    pub raw: String,
    /// fr fr Parsed parameters
    pub params: HashMap<String, String>,
}

impl ConnectionString {
    /// slay Parse connection string
    pub fn parse(conn_str: &str) -> Result<Self, PostgreSQLError> {
        let mut params = HashMap::new();
        
        if conn_str.starts_with("postgresql://") || conn_str.starts_with("postgres://") {
            // Parse URI format
            Self::parse_uri(conn_str, &mut params)?;
        } else {
            // Parse key=value format
            Self::parse_key_value(conn_str, &mut params)?;
        }
        
        Ok(Self {
            raw: conn_str.to_string(),
            params,
        })
    }
    
    /// slay Parse URI format (postgresql://user:pass@host:port/dbname?param1=value1)
    fn parse_uri(uri: &str, params: &mut HashMap<String, String>) -> Result<(), PostgreSQLError> {
        // Remove scheme
        let without_scheme = uri.trim_start_matches("postgresql://").trim_start_matches("postgres://");
        
        // Split at '?' for query parameters
        let (main_part, query_part) = if let Some(pos) = without_scheme.find('?') {
            (&without_scheme[..pos], Some(&without_scheme[pos + 1..]))
        } else {
            (without_scheme, None)
        };
        
        // Parse main part (user:pass@host:port/dbname)
        let (auth_part, host_db_part) = if let Some(pos) = main_part.find('@') {
            (Some(&main_part[..pos]), &main_part[pos + 1..])
        } else {
            (None, main_part)
        };
        
        // Parse authentication
        if let Some(auth) = auth_part {
            if let Some(pos) = auth.find(':') {
                params.insert("user".to_string(), auth[..pos].to_string());
                params.insert("password".to_string(), auth[pos + 1..].to_string());
            } else {
                params.insert("user".to_string(), auth.to_string());
            }
        }
        
        // Parse host:port/dbname
        let (host_port, dbname) = if let Some(pos) = host_db_part.find('/') {
            (&host_db_part[..pos], Some(&host_db_part[pos + 1..]))
        } else {
            (host_db_part, None)
        };
        
        // Parse host and port
        if let Some(pos) = host_port.rfind(':') {
            let host = &host_port[..pos];
            let port_str = &host_port[pos + 1..];
            
            params.insert("host".to_string(), host.to_string());
            if !port_str.is_empty() {
                params.insert("port".to_string(), port_str.to_string());
            }
        } else if !host_port.is_empty() {
            params.insert("host".to_string(), host_port.to_string());
        }
        
        // Set database name
        if let Some(db) = dbname {
            if !db.is_empty() {
                params.insert("dbname".to_string(), db.to_string());
            }
        }
        
        // Parse query parameters
        if let Some(query) = query_part {
            Self::parse_query_string(query, params)?;
        }
        
        Ok(())
    }
    
    /// slay Parse key=value format (host=localhost port=5432 dbname=mydb)
    fn parse_key_value(conn_str: &str, params: &mut HashMap<String, String>) -> Result<(), PostgreSQLError> {
        let mut chars = conn_str.chars().peekable();
        
        while chars.peek().is_some() {
            // Skip whitespace
            while chars.peek() == Some(&' ') || chars.peek() == Some(&'\t') {
                chars.next();
            }
            
            if chars.peek().is_none() {
                break;
            }
            
            // Read key
            let mut key = String::new();
            while let Some(&ch) = chars.peek() {
                if ch == '=' || ch == ' ' || ch == '\t' {
                    break;
                }
                key.push(chars.next().unwrap());
            }
            
            if key.is_empty() {
                break;
            }
            
            // Skip whitespace and '='
            while chars.peek() == Some(&' ') || chars.peek() == Some(&'\t') {
                chars.next();
            }
            
            if chars.next() != Some('=') {
                return Err(PostgreSQLError::new(
                    super::super::DatabaseErrorKind::ConfigurationError,
                    format!("Expected '=' after key '{}'", key)
                ));
            }
            
            // Skip whitespace after '='
            while chars.peek() == Some(&' ') || chars.peek() == Some(&'\t') {
                chars.next();
            }
            
            // Read value
            let mut value = String::new();
            let quoted = chars.peek() == Some(&'\'') || chars.peek() == Some(&'"');
            
            if quoted {
                let quote_char = chars.next().unwrap();
                while let Some(ch) = chars.next() {
                    if ch == quote_char {
                        break;
                    }
                    if ch == '\\' {
                        if let Some(escaped) = chars.next() {
                            value.push(escaped);
                        }
                    } else {
                        value.push(ch);
                    }
                }
            } else {
                while let Some(&ch) = chars.peek() {
                    if ch == ' ' || ch == '\t' {
                        break;
                    }
                    value.push(chars.next().unwrap());
                }
            }
            
            params.insert(key, value);
        }
        
        Ok(())
    }
    
    /// slay Parse query string (param1=value1&param2=value2)
    fn parse_query_string(query: &str, params: &mut HashMap<String, String>) -> Result<(), PostgreSQLError> {
        for pair in query.split('&') {
            if let Some(pos) = pair.find('=') {
                let key = &pair[..pos];
                let value = &pair[pos + 1..];
                
                // URL decode key and value
                let decoded_key = Self::url_decode(key)?;
                let decoded_value = Self::url_decode(value)?;
                
                params.insert(decoded_key, decoded_value);
            } else if !pair.is_empty() {
                params.insert(Self::url_decode(pair)?, String::new());
            }
        }
        
        Ok(())
    }
    
    /// slay URL decode string
    fn url_decode(s: &str) -> Result<String, PostgreSQLError> {
        let mut result = String::new();
        let mut chars = s.chars();
        
        while let Some(ch) = chars.next() {
            if ch == '%' {
                let hex1 = chars.next().ok_or_else(|| PostgreSQLError::new(
                    super::super::DatabaseErrorKind::ConfigurationError,
                    "Invalid percent encoding".to_string()
                ))?;
                let hex2 = chars.next().ok_or_else(|| PostgreSQLError::new(
                    super::super::DatabaseErrorKind::ConfigurationError,
                    "Invalid percent encoding".to_string()
                ))?;
                
                let hex_str = format!("{}{}", hex1, hex2);
                let byte = u8::from_str_radix(&hex_str, 16).map_err(|_| PostgreSQLError::new(
                    super::super::DatabaseErrorKind::ConfigurationError,
                    "Invalid hex in percent encoding".to_string()
                ))?;
                
                result.push(byte as char);
            } else if ch == '+' {
                result.push(' ');
            } else {
                result.push(ch);
            }
        }
        
        Ok(result)
    }
    
    /// slay Get parameter value
    pub fn get(&self, key: &str) -> Option<&String> {
        self.params.get(key)
    }
    
    /// slay Convert to PostgreSQL config
    pub fn to_config(&self) -> Result<PostgreSQLConfig, PostgreSQLError> {
        let mut config = PostgreSQLConfig::default();
        
        if let Some(host) = self.get("host") {
            config.host = host.clone();
        }
        
        if let Some(port_str) = self.get("port") {
            config.port = port_str.parse().map_err(|_| PostgreSQLError::new(
                super::super::DatabaseErrorKind::ConfigurationError,
                format!("Invalid port: {}", port_str)
            ))?;
        }
        
        if let Some(dbname) = self.get("dbname") {
            config.dbname = dbname.clone();
        }
        
        if let Some(user) = self.get("user") {
            config.user = user.clone();
        }
        
        if let Some(password) = self.get("password") {
            config.password = Some(password.clone());
        }
        
        if let Some(ssl_mode_str) = self.get("sslmode") {
            config.ssl_mode = SslMode::from_string(ssl_mode_str)?;
        }
        
        if let Some(app_name) = self.get("application_name") {
            config.application_name = app_name.clone();
        }
        
        if let Some(encoding) = self.get("client_encoding") {
            config.client_encoding = encoding.clone();
        }
        
        if let Some(timeout_str) = self.get("connect_timeout") {
            let timeout_secs: u64 = timeout_str.parse().map_err(|_| PostgreSQLError::new(
                super::super::DatabaseErrorKind::ConfigurationError,
                format!("Invalid connect_timeout: {}", timeout_str)
            ))?;
            config.connect_timeout = Duration::from_secs(timeout_secs);
        }
        
        // Add any unrecognized parameters as extra params
        let known_params = [
            "host", "port", "dbname", "user", "password", "sslmode",
            "application_name", "client_encoding", "connect_timeout"
        ];
        
        for (key, value) in &self.params {
            if !known_params.contains(&key.as_str()) {
                config.extra_params.insert(key.clone(), value.clone());
            }
        }
        
        Ok(config)
    }
}

/// fr fr Connection string builder for easier configuration
#[derive(Debug, Clone)]
pub struct ConnectionStringBuilder {
    config: PostgreSQLConfig,
}

impl ConnectionStringBuilder {
    /// slay Create a new builder
    pub fn new() -> Self {
        Self {
            config: PostgreSQLConfig::default(),
        }
    }
    
    /// slay Build from existing config
    pub fn from_config(config: PostgreSQLConfig) -> Self {
        Self { config }
    }
    
    /// slay Set host
    pub fn host(mut self, host: &str) -> Self {
        self.config.host = host.to_string();
        self
    }
    
    /// slay Set port
    pub fn port(mut self, port: u16) -> Self {
        self.config.port = port;
        self
    }
    
    /// slay Set database name
    pub fn database(mut self, dbname: &str) -> Self {
        self.config.dbname = dbname.to_string();
        self
    }
    
    /// slay Set username
    pub fn username(mut self, user: &str) -> Self {
        self.config.user = user.to_string();
        self
    }
    
    /// slay Set password
    pub fn password(mut self, password: &str) -> Self {
        self.config.password = Some(password.to_string());
        self
    }
    
    /// slay Set SSL mode
    pub fn ssl_mode(mut self, ssl_mode: SslMode) -> Self {
        self.config.ssl_mode = ssl_mode;
        self
    }
    
    /// slay Build connection string
    pub fn build(&self) -> String {
        self.config.to_connection_string()
    }
    
    /// slay Build URI
    pub fn build_uri(&self) -> String {
        self.config.to_uri()
    }
    
    /// slay Get config
    pub fn config(&self) -> &PostgreSQLConfig {
        &self.config
    }
}

impl Default for ConnectionStringBuilder {
    fn default() -> Self {
        Self::new()
    }
}
