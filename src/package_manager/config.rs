//! Package manager configuration system for CURSED
//!
//! This module handles configuration for package registries, authentication,
//! and other package manager settings.

use crate::error::{CursedError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Duration;

/// Global package manager configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageConfig {
    /// Default registry configuration
    pub default_registry: String,
    /// Available registries
    pub registries: HashMap<String, RegistryConfig>,
    /// Authentication tokens for registries
    pub auth: HashMap<String, AuthConfig>,
    /// Cache settings
    pub cache: CacheSettings,
    /// Download settings
    pub download: DownloadSettings,
    /// Mirror configuration
    pub mirrors: Vec<MirrorConfig>,
}

/// Registry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryConfig {
    /// Base URL of the registry
    pub url: String,
    /// Optional display name
    pub name: Option<String>,
    /// Request timeout in seconds
    #[serde(default = "default_timeout")]
    pub timeout_seconds: u64,
    /// Maximum retry attempts
    #[serde(default = "default_max_retries")]
    pub max_retries: u32,
    /// Whether to verify SSL certificates
    #[serde(default = "default_verify_ssl")]
    pub verify_ssl: bool,
    /// Custom headers to send with requests
    #[serde(default)]
    pub headers: HashMap<String, String>,
}

/// Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    /// Authentication type
    pub auth_type: AuthType,
    /// Token or credentials
    pub credentials: String,
    /// Optional username for basic auth
    pub username: Option<String>,
}

/// Authentication type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AuthType {
    /// Bearer token authentication
    Bearer,
    /// Basic authentication (username/password)
    Basic,
    /// API key authentication
    ApiKey,
    /// Custom header authentication
    Custom { header_name: String },
}

/// Cache settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheSettings {
    /// Cache directory
    pub directory: PathBuf,
    /// Maximum cache size in bytes
    #[serde(default = "default_cache_size")]
    pub max_size_bytes: u64,
    /// Cache entry TTL in seconds
    #[serde(default = "default_cache_ttl")]
    pub ttl_seconds: u64,
    /// Whether to clean cache on startup
    #[serde(default)]
    pub clean_on_startup: bool,
}

/// Download settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadSettings {
    /// Maximum concurrent downloads
    #[serde(default = "default_concurrent_downloads")]
    pub max_concurrent: usize,
    /// Download timeout in seconds
    #[serde(default = "default_download_timeout")]
    pub timeout_seconds: u64,
    /// Number of retry attempts
    #[serde(default = "default_download_retries")]
    pub max_retries: u32,
    /// User agent string
    #[serde(default = "default_user_agent")]
    pub user_agent: String,
    /// Whether to verify checksums
    #[serde(default = "default_verify_checksums")]
    pub verify_checksums: bool,
}

/// Mirror configuration for package sources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MirrorConfig {
    /// Original registry URL pattern to match
    pub source_pattern: String,
    /// Mirror URL to use instead
    pub mirror_url: String,
    /// Optional priority (higher = preferred)
    #[serde(default)]
    pub priority: u32,
    /// Whether this mirror is currently enabled
    #[serde(default = "default_true")]
    pub enabled: bool,
}

// Default value functions for serde
fn default_timeout() -> u64 { 30 }
fn default_max_retries() -> u32 { 3 }
fn default_verify_ssl() -> bool { true }
fn default_cache_size() -> u64 { 1024 * 1024 * 1024 } // 1GB
fn default_cache_ttl() -> u64 { 3600 * 24 } // 24 hours
fn default_concurrent_downloads() -> usize { 4 }
fn default_download_timeout() -> u64 { 300 } // 5 minutes
fn default_download_retries() -> u32 { 3 }
fn default_user_agent() -> String { "cursed-package-manager/1.0".to_string() }
fn default_verify_checksums() -> bool { true }
fn default_true() -> bool { true }

impl Default for PackageConfig {
    fn default() -> Self {
        let mut registries = HashMap::new();
        registries.insert(
            "official".to_string(),
            RegistryConfig {
                url: "https://packages.cursed-lang.org".to_string(),
                name: Some("Official CURSED Registry".to_string()),
                timeout_seconds: default_timeout(),
                max_retries: default_max_retries(),
                verify_ssl: default_verify_ssl(),
                headers: HashMap::new(),
            }
        );

        Self {
            default_registry: "official".to_string(),
            registries,
            auth: HashMap::new(),
            cache: CacheSettings {
                directory: PathBuf::from("target/package-cache"),
                max_size_bytes: default_cache_size(),
                ttl_seconds: default_cache_ttl(),
                clean_on_startup: false,
            },
            download: DownloadSettings {
                max_concurrent: default_concurrent_downloads(),
                timeout_seconds: default_download_timeout(),
                max_retries: default_download_retries(),
                user_agent: default_user_agent(),
                verify_checksums: default_verify_checksums(),
            },
            mirrors: Vec::new(),
        }
    }
}

impl Default for RegistryConfig {
    fn default() -> Self {
        Self {
            url: "https://packages.cursed-lang.org".to_string(),
            name: None,
            timeout_seconds: default_timeout(),
            max_retries: default_max_retries(),
            verify_ssl: default_verify_ssl(),
            headers: HashMap::new(),
        }
    }
}

impl PackageConfig {
    /// Load configuration from file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        
        if !path.exists() {
            tracing::info!("Config file not found at {:?}, using defaults", path);
            return Ok(Self::default());
        }

        let content = std::fs::read_to_string(path)
            .map_err(|e| CursedError::General(format!("Failed to read config file: {}", e)))?;

        let config: PackageConfig = toml::from_str(&content)
            .map_err(|e| CursedError::General(format!("Failed to parse config file: {}", e)))?;

        tracing::info!("Loaded package configuration from {:?}", path);
        Ok(config)
    }

    /// Save configuration to file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path = path.as_ref();
        
        // Create parent directories if they don't exist
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| CursedError::General(format!("Failed to create config directory: {}", e)))?;
        }

        let content = toml::to_string_pretty(self)
            .map_err(|e| CursedError::General(format!("Failed to serialize config: {}", e)))?;

        std::fs::write(path, content)
            .map_err(|e| CursedError::General(format!("Failed to write config file: {}", e)))?;

        tracing::info!("Saved package configuration to {:?}", path);
        Ok(())
    }

    /// Get registry configuration by name
    pub fn get_registry(&self, name: &str) -> Option<&RegistryConfig> {
        self.registries.get(name)
    }

    /// Get default registry configuration
    pub fn get_default_registry(&self) -> Option<&RegistryConfig> {
        self.get_registry(&self.default_registry)
    }

    /// Add or update registry
    pub fn add_registry(&mut self, name: String, config: RegistryConfig) {
        self.registries.insert(name, config);
    }

    /// Remove registry
    pub fn remove_registry(&mut self, name: &str) -> Option<RegistryConfig> {
        self.registries.remove(name)
    }

    /// Set authentication for a registry
    pub fn set_auth(&mut self, registry: String, auth: AuthConfig) {
        self.auth.insert(registry, auth);
    }

    /// Get authentication for a registry
    pub fn get_auth(&self, registry: &str) -> Option<&AuthConfig> {
        self.auth.get(registry)
    }

    /// Add mirror configuration
    pub fn add_mirror(&mut self, mirror: MirrorConfig) {
        self.mirrors.push(mirror);
    }

    /// Get active mirrors for a source URL
    pub fn get_mirrors_for_url(&self, url: &str) -> Vec<&MirrorConfig> {
        self.mirrors
            .iter()
            .filter(|mirror| mirror.enabled && url.contains(&mirror.source_pattern))
            .collect()
    }

    /// Apply mirrors to a URL, returning the best mirror or original URL
    pub fn apply_mirrors(&self, url: &str) -> String {
        let mut mirrors = self.get_mirrors_for_url(url);
        
        // Sort by priority (highest first)
        mirrors.sort_by(|a, b| b.priority.cmp(&a.priority));
        
        if let Some(mirror) = mirrors.first() {
            url.replace(&mirror.source_pattern, &mirror.mirror_url)
        } else {
            url.to_string()
        }
    }

    /// Get configuration directory path
    pub fn config_dir() -> Result<PathBuf> {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| CursedError::General("Unable to determine home directory".to_string()))?;
        
        Ok(home_dir.join(".cursed").join("package"))
    }

    /// Get default configuration file path
    pub fn default_config_path() -> Result<PathBuf> {
        Ok(Self::config_dir()?.join("config.toml"))
    }

    /// Load configuration from default location
    pub fn load_default() -> Result<Self> {
        let config_path = Self::default_config_path()?;
        Self::from_file(config_path)
    }

    /// Save configuration to default location
    pub fn save_default(&self) -> Result<()> {
        let config_path = Self::default_config_path()?;
        self.save_to_file(config_path)
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        // Check that default registry exists
        if !self.registries.contains_key(&self.default_registry) {
            return Err(CursedError::General(format!(
                "Default registry '{}' not found in configuration",
                self.default_registry
            )));
        }

        // Validate registry URLs
        for (name, registry) in &self.registries {
            if registry.url.is_empty() {
                return Err(CursedError::General(format!(
                    "Registry '{}' has empty URL",
                    name
                )));
            }
            
            // Basic URL validation
            if !registry.url.starts_with("http://") && !registry.url.starts_with("https://") {
                return Err(CursedError::General(format!(
                    "Registry '{}' has invalid URL: {}",
                    name, registry.url
                )));
            }
        }

        // Validate cache directory
        if self.cache.directory.as_os_str().is_empty() {
            return Err(CursedError::General("Cache directory cannot be empty".to_string()));
        }

        tracing::debug!("Package configuration validation passed");
        Ok(())
    }
}

impl RegistryConfig {
    /// Convert to reqwest client timeout
    pub fn timeout(&self) -> Duration {
        Duration::from_secs(self.timeout_seconds)
    }

    /// Build HTTP headers map
    pub fn build_headers(&self, auth: Option<&AuthConfig>) -> Result<reqwest::header::HeaderMap> {
        let mut headers = reqwest::header::HeaderMap::new();
        
        // Add custom headers from config
        for (key, value) in &self.headers {
            let header_name = reqwest::header::HeaderName::from_bytes(key.as_bytes())
                .map_err(|e| CursedError::General(format!("Invalid header name '{}': {}", key, e)))?;
            let header_value = reqwest::header::HeaderValue::from_str(value)
                .map_err(|e| CursedError::General(format!("Invalid header value '{}': {}", value, e)))?;
            headers.insert(header_name, header_value);
        }

        // Add authentication headers
        if let Some(auth_config) = auth {
            match &auth_config.auth_type {
                AuthType::Bearer => {
                    let auth_value = format!("Bearer {}", auth_config.credentials);
                    headers.insert(
                        reqwest::header::AUTHORIZATION,
                        reqwest::header::HeaderValue::from_str(&auth_value)
                            .map_err(|e| CursedError::General(format!("Invalid auth token: {}", e)))?,
                    );
                }
                AuthType::Basic => {
                    let username = auth_config.username.as_deref().unwrap_or("");
                    let credentials = base64::encode(&format!("{}:{}", username, auth_config.credentials));
                    let auth_value = format!("Basic {}", credentials);
                    headers.insert(
                        reqwest::header::AUTHORIZATION,
                        reqwest::header::HeaderValue::from_str(&auth_value)
                            .map_err(|e| CursedError::General(format!("Invalid basic auth: {}", e)))?,
                    );
                }
                AuthType::ApiKey => {
                    headers.insert(
                        reqwest::header::HeaderName::from_static("x-api-key"),
                        reqwest::header::HeaderValue::from_str(&auth_config.credentials)
                            .map_err(|e| CursedError::General(format!("Invalid API key: {}", e)))?,
                    );
                }
                AuthType::Custom { header_name } => {
                    let header_name = reqwest::header::HeaderName::from_bytes(header_name.as_bytes())
                        .map_err(|e| CursedError::General(format!("Invalid custom header name '{}': {}", header_name, e)))?;
                    let header_value = reqwest::header::HeaderValue::from_str(&auth_config.credentials)
                        .map_err(|e| CursedError::General(format!("Invalid custom header value: {}", e)))?;
                    headers.insert(header_name, header_value);
                }
            }
        }

        Ok(headers)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_default_config() {
        let config = PackageConfig::default();
        assert_eq!(config.default_registry, "official");
        assert!(config.registries.contains_key("official"));
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_serialization() {
        let config = PackageConfig::default();
        let toml_str = toml::to_string(&config).unwrap();
        let deserialized: PackageConfig = toml::from_str(&toml_str).unwrap();
        assert_eq!(config.default_registry, deserialized.default_registry);
    }

    #[test]
    fn test_config_file_operations() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");
        
        let config = PackageConfig::default();
        config.save_to_file(&config_path).unwrap();
        
        let loaded_config = PackageConfig::from_file(&config_path).unwrap();
        assert_eq!(config.default_registry, loaded_config.default_registry);
    }

    #[test]
    fn test_mirror_application() {
        let mut config = PackageConfig::default();
        config.add_mirror(MirrorConfig {
            source_pattern: "packages.cursed-lang.org".to_string(),
            mirror_url: "mirror.example.com".to_string(),
            priority: 1,
            enabled: true,
        });

        let original_url = "https://packages.cursed-lang.org/package.tar.gz";
        let mirrored_url = config.apply_mirrors(original_url);
        assert_eq!(mirrored_url, "https://mirror.example.com/package.tar.gz");
    }
}
