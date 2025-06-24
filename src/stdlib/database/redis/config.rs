/// Redis configuration management and validation
/// 
/// Provides comprehensive configuration options for Redis connections
/// including security, performance, and reliability settings.

use std::time::Duration;
use serde::{Deserialize, Serialize};
use tracing::{debug, info, instrument};

use super::DatabaseError;
use crate::error::Error;

/// Comprehensive Redis configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfiguration {
    /// Connection settings
    pub connection: ConnectionConfig,
    /// Security settings
    pub security: SecurityConfig,
    /// Performance settings
    pub performance: PerformanceConfig,
    /// Monitoring settings
    pub monitoring: MonitoringConfig,
}

/// Connection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionConfig {
    /// Redis server host
    pub host: String,
    /// Redis server port
    pub port: u16,
    /// Database number (0-15)
    pub database: u8,
    /// Connection timeout
    pub timeout: Duration,
    /// Keep-alive settings
    pub keep_alive: Option<Duration>,
    /// TCP nodelay
    pub tcp_nodelay: bool,
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Username (Redis 6.0+)
    pub username: Option<String>,
    /// Password for authentication
    pub password: Option<String>,
    /// Enable TLS/SSL
    pub use_tls: bool,
    /// TLS certificate path
    pub tls_cert_file: Option<String>,
    /// TLS key file path
    pub tls_key_file: Option<String>,
    /// TLS CA certificate file
    pub tls_ca_file: Option<String>,
    /// Verify TLS certificates
    pub tls_verify: bool,
}

/// Performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Maximum connections in pool
    pub max_connections: usize,
    /// Minimum idle connections
    pub min_idle_connections: usize,
    /// Command timeout
    pub command_timeout: Duration,
    /// Connection idle timeout
    pub idle_timeout: Duration,
    /// Enable pipelining
    pub pipelining: bool,
    /// Pipeline batch size
    pub pipeline_batch_size: usize,
    /// Connection retry attempts
    pub retry_attempts: usize,
    /// Retry delay
    pub retry_delay: Duration,
}

/// Monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// Enable command monitoring
    pub enable_monitoring: bool,
    /// Enable slow log monitoring
    pub enable_slow_log: bool,
    /// Slow command threshold
    pub slow_command_threshold: Duration,
    /// Enable connection monitoring
    pub enable_connection_monitoring: bool,
    /// Monitoring interval
    pub monitoring_interval: Duration,
}

impl Default for RedisConfiguration {
    fn default() -> Self {
        Self {
            connection: ConnectionConfig::default(),
            security: SecurityConfig::default(),
            performance: PerformanceConfig::default(),
            monitoring: MonitoringConfig::default(),
        }
    }
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 6379,
            database: 0,
            timeout: Duration::from_secs(5),
            keep_alive: Some(Duration::from_secs(60)),
            tcp_nodelay: true,
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            username: None,
            password: None,
            use_tls: false,
            tls_cert_file: None,
            tls_key_file: None,
            tls_ca_file: None,
            tls_verify: true,
        }
    }
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            max_connections: 10,
            min_idle_connections: 2,
            command_timeout: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(300),
            pipelining: false,
            pipeline_batch_size: 50,
            retry_attempts: 3,
            retry_delay: Duration::from_millis(100),
        }
    }
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            enable_monitoring: true,
            enable_slow_log: true,
            slow_command_threshold: Duration::from_millis(100),
            enable_connection_monitoring: true,
            monitoring_interval: Duration::from_secs(30),
        }
    }
}

impl RedisConfiguration {
    /// Create configuration from URL
    #[instrument]
    pub fn from_url(url: &str) -> Result<(), Error> {
        info!(url = url, "Creating Redis configuration from URL");
        
        // Parse URL (simplified implementation)
        let mut config = Self::default();
        
        if url.starts_with("redis://") {
            // Standard Redis URL
            config.security.use_tls = false;
        } else if url.starts_with("rediss://") {
            // TLS Redis URL
            config.security.use_tls = true;
        } else {
            return Err(DatabaseError::Configuration("Invalid Redis URL scheme".to_string()));
        }
        
        // Extract host, port, auth from URL
        // This is a simplified parser - in production you'd use a proper URL parser
        debug!("Parsed Redis configuration from URL");
        
        Ok(config)
    }
    
    /// Validate configuration
    #[instrument(skip(self))]
    pub fn validate(&self) -> Result<(), Error> {
        debug!("Validating Redis configuration");
        
        // Validate connection settings
        if self.connection.host.is_empty() {
            return Err(DatabaseError::Configuration("Redis host cannot be empty".to_string()));
        }
        
        if self.connection.port == 0 {
            return Err(DatabaseError::Configuration("Redis port must be greater than 0".to_string()));
        }
        
        if self.connection.database > 15 {
            return Err(DatabaseError::Configuration("Redis database number must be 0-15".to_string()));
        }
        
        // Validate performance settings
        if self.performance.max_connections == 0 {
            return Err(DatabaseError::Configuration("Max connections must be greater than 0".to_string()));
        }
        
        if self.performance.min_idle_connections > self.performance.max_connections {
            return Err(DatabaseError::Configuration("Min idle connections cannot exceed max connections".to_string()));
        }
        
        // Validate TLS settings
        if self.security.use_tls {
            if self.security.tls_cert_file.is_none() && self.security.tls_key_file.is_some() {
                return Err(DatabaseError::Configuration("TLS certificate file required when key file is specified".to_string()));
            }
        }
        
        info!("Redis configuration validation successful");
        Ok(())
    }
    
    /// Get connection URL
    #[instrument(skip(self))]
    pub fn get_connection_url(&self) -> String {
        let scheme = if self.security.use_tls { "rediss" } else { "redis" };
        
        let auth = match (&self.security.username, &self.security.password) {
            (Some(username), Some(password)) => format!("{}:{}@", username, password),
            (None, Some(password)) => format!(":{}@", password),
            _ => String::new(),
        };
        
        format!(
            "{}://{}{}:{}/{}",
            scheme,
            auth,
            self.connection.host,
            self.connection.port,
            self.connection.database
        )
    }
    
    /// Load configuration from file
    #[instrument]
    pub fn load_from_file(path: &str) -> Result<(), Error> {
        info!(path = path, "Loading Redis configuration from file");
        
        let content = std::fs::read_to_string(path)
            .map_err(|e| DatabaseError::Configuration(format!("Failed to read config file: {}", e)))?;
        
        let config: Self = if path.ends_with(".toml") {
            toml::from_str(&content)
                .map_err(|e| DatabaseError::Configuration(format!("Failed to parse TOML config: {}", e)))?
        } else if path.ends_with(".json") {
            serde_json::from_str(&content)
                .map_err(|e| DatabaseError::Configuration(format!("Failed to parse JSON config: {}", e)))?
        } else if path.ends_with(".yaml") || path.ends_with(".yml") {
            serde_yaml::from_str(&content)
                .map_err(|e| DatabaseError::Configuration(format!("Failed to parse YAML config: {}", e)))?
        } else {
            return Err(DatabaseError::Configuration("Unsupported config file format".to_string()));
        };
        
        config.validate()?;
        
        debug!("Successfully loaded Redis configuration from file");
        Ok(config)
    }
    
    /// Save configuration to file
    #[instrument(skip(self))]
    pub fn save_to_file(&self, path: &str) -> Result<(), Error> {
        info!(path = path, "Saving Redis configuration to file");
        
        let content = if path.ends_with(".toml") {
            toml::to_string_pretty(self)
                .map_err(|e| DatabaseError::Configuration(format!("Failed to serialize TOML config: {}", e)))?
        } else if path.ends_with(".json") {
            serde_json::to_string_pretty(self)
                .map_err(|e| DatabaseError::Configuration(format!("Failed to serialize JSON config: {}", e)))?
        } else if path.ends_with(".yaml") || path.ends_with(".yml") {
            serde_yaml::to_string(self)
                .map_err(|e| DatabaseError::Configuration(format!("Failed to serialize YAML config: {}", e)))?
        } else {
            return Err(DatabaseError::Configuration("Unsupported config file format".to_string()));
        };
        
        std::fs::write(path, content)
            .map_err(|e| DatabaseError::Configuration(format!("Failed to write config file: {}", e)))?;
        
        debug!("Successfully saved Redis configuration to file");
        Ok(())
    }
}
