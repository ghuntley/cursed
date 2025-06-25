/// Redis configuration management and validation
/// 
/// Provides comprehensive configuration options for Redis connections
/// including security, performance, and reliability settings.

use std::time::Duration;
use serde::{Deserialize, Serialize};
use tracing::{debug, info, instrument};

use crate::error::CursedError;

/// Comprehensive Redis configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfiguration {
    /// Connection settings
    /// Security settings
    /// Performance settings
    /// Monitoring settings
/// Connection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionConfig {
    /// Redis server host
    /// Redis server port
    /// Database number (0-15)
    /// Connection timeout
    /// Keep-alive settings
    /// TCP nodelay
/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Username (Redis 6.0+)
    /// Password for authentication
    /// Enable TLS/SSL
    /// TLS certificate path
    /// TLS key file path
    /// TLS CA certificate file
    /// Verify TLS certificates
/// Performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Maximum connections in pool
    /// Minimum idle connections
    /// Command timeout
    /// Connection idle timeout
    /// Enable pipelining
    /// Pipeline batch size
    /// Connection retry attempts
    /// Retry delay
/// Monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// Enable command monitoring
    /// Enable slow log monitoring
    /// Slow command threshold
    /// Enable connection monitoring
    /// Monitoring interval
impl Default for RedisConfiguration {
    fn default() -> Self {
        Self {
        }
    }
impl Default for ConnectionConfig {
    fn default() -> Self {
        Self {
        }
    }
impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
        }
    }
impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
        }
    }
impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
        }
    }
impl RedisConfiguration {
    /// Create configuration from URL
    #[instrument]
    pub fn from_url(url: &str) -> crate::error::Result<()> {
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
        // Extract host, port, auth from URL
        // This is a simplified parser - in production you'd use a proper URL parser
        debug!("Parsed Redis configuration from URL");
        
        Ok(config)
    /// Validate configuration
    #[instrument(skip(self))]
    pub fn validate(&self) -> crate::error::Result<()> {
        debug!("Validating Redis configuration");
        
        // Validate connection settings
        if self.connection.host.is_empty() {
            return Err(DatabaseError::Configuration("Redis host cannot be empty".to_string()));
        if self.connection.port == 0 {
            return Err(DatabaseError::Configuration("Redis port must be greater than 0".to_string()));
        if self.connection.database > 15 {
            return Err(DatabaseError::Configuration("Redis database number must be 0-15".to_string()));
        // Validate performance settings
        if self.performance.max_connections == 0 {
            return Err(DatabaseError::Configuration("Max connections must be greater than 0".to_string()));
        if self.performance.min_idle_connections > self.performance.max_connections {
            return Err(DatabaseError::Configuration("Min idle connections cannot exceed max connections".to_string()));
        // Validate TLS settings
        if self.security.use_tls {
            if self.security.tls_cert_file.is_none() && self.security.tls_key_file.is_some() {
                return Err(DatabaseError::Configuration("TLS certificate file required when key file is specified".to_string()));
            }
        }
        
        info!("Redis configuration validation successful");
        Ok(())
    /// Get connection URL
    #[instrument(skip(self))]
    pub fn get_connection_url(&self) -> String {
        let scheme = if self.security.use_tls { "rediss" } else { "redis" };
        
        let auth = match (&self.security.username, &self.security.password) {
        
        format!(
            "{}://{}{}:{}/{}",
            self.connection.database
        )
    /// Load configuration from file
    #[instrument]
    pub fn load_from_file(path: &str) -> crate::error::Result<()> {
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
        
        config.validate()?;
        
        debug!("Successfully loaded Redis configuration from file");
        Ok(config)
    /// Save configuration to file
    #[instrument(skip(self))]
    pub fn save_to_file(&self, path: &str) -> crate::error::Result<()> {
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
        
        std::fs::write(path, content)
            .map_err(|e| DatabaseError::Configuration(format!("Failed to write config file: {}", e)))?;
        
        debug!("Successfully saved Redis configuration to file");
        Ok(())
    }
}
