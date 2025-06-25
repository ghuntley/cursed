// Package manager configuration
use crate::error_types::CursedError;
use std::collections::HashMap;
use std::path::PathBuf;

/// Configuration for the package manager
#[derive(Debug, Clone)]
pub struct PackageManagerConfig {
impl Default for PackageManagerConfig {
    fn default() -> Self {
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let cursed_dir = home_dir.join(".cursed");

        Self {
            registry_urls: vec!["https://packages.cursed-lang.org".to_string()],
        }
    }
impl PackageManagerConfig {
    pub fn new() -> Self {
        Self::default()
    pub fn with_registry_url(mut self, url: String) -> Self {
        self.registry_urls = vec![url];
        self
    pub fn add_registry_url(mut self, url: String) -> Self {
        self.registry_urls.push(url);
        self
    pub fn with_cache_dir(mut self, dir: PathBuf) -> Self {
        self.cache_dir = dir;
        self
    pub fn with_timeout(mut self, seconds: u64) -> Self {
        self.timeout_seconds = seconds;
        self
    pub fn with_max_downloads(mut self, max: usize) -> Self {
        self.max_concurrent_downloads = max;
        self
    pub fn enable_offline_mode(mut self) -> Self {
        self.offline_mode = true;
        self
    pub fn allow_pre_releases(mut self) -> Self {
        self.allow_pre_releases = true;
        self
    pub fn with_proxy(mut self, proxy: ProxyConfig) -> Self {
        self.proxy_settings = Some(proxy);
        self
    pub fn add_auth(mut self, registry: String, auth: AuthConfig) -> Self {
        self.authentication.insert(registry, auth);
        self
    pub fn create_directories(&self) -> crate::error_types::Result<()> {
        std::fs::create_dir_all(&self.cache_dir)
            .map_err(|e| CursedError::Io(e.into()))?;
        std::fs::create_dir_all(&self.global_package_dir)
            .map_err(|e| CursedError::Io(e.into()))?;
        Ok(())
    pub fn load_from_file(path: &std::path::Path) -> crate::error_types::Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        let content = std::fs::read_to_string(path)
            .map_err(|e| CursedError::Io(e.into()))?;
        
        // TODO: Implement proper config file parsing (TOML/JSON)
        // For now, return default config
        Ok(Self::default())
    pub fn save_to_file(&self, path: &std::path::Path) -> crate::error_types::Result<()> {
        // TODO: Implement config file serialization
        // For now, just create the directory
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| CursedError::Io(e.into()))?;
        }
        Ok(())
    }
}

/// Proxy configuration
#[derive(Debug, Clone)]
pub struct ProxyConfig {
impl ProxyConfig {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn with_http_proxy(mut self, proxy: String) -> Self {
        self.http_proxy = Some(proxy);
        self
    pub fn with_https_proxy(mut self, proxy: String) -> Self {
        self.https_proxy = Some(proxy);
        self
    pub fn add_no_proxy(mut self, host: String) -> Self {
        self.no_proxy.push(host);
        self
    }
}

impl Default for ProxyConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Authentication configuration
#[derive(Debug, Clone)]
pub enum AuthConfig {
impl AuthConfig {
    pub fn token(token: String) -> Self {
        Self::Token(token)
    pub fn basic(username: String, password: String) -> Self {
        Self::Basic { username, password }
    }

    pub fn bearer(token: String) -> Self {
        Self::Bearer(token)
    pub fn api_key(key: String, header: String) -> Self {
        Self::ApiKey { key, header }
    }

    pub fn to_header(&self) -> (String, String) {
        match self {
            AuthConfig::Basic { username, password } => {
                let credentials = base64::encode(format!("{}:{}", username, password));
                ("Authorization".to_string(), format!("Basic {}", credentials))
            }
        }
    }
/// Registry configuration
#[derive(Debug, Clone)]
pub struct RegistryConfig {
impl RegistryConfig {
    pub fn new(name: String, url: String) -> Self {
        Self {
        }
    }

    pub fn with_auth(mut self, auth: AuthConfig) -> Self {
        self.auth = Some(auth);
        self
    pub fn disable_ssl_verification(mut self) -> Self {
        self.verify_ssl = false;
        self
    pub fn with_timeout(mut self, seconds: u64) -> Self {
        self.timeout_seconds = seconds;
        self
    }
}
