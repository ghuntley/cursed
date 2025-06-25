// Package manager configuration
use crate::error_types::CursedError;
use std::collections::HashMap;
use std::path::PathBuf;

/// Configuration for the package manager
#[derive(Debug, Clone)]
pub struct PackageManagerConfig {
    pub registry_urls: Vec<String>,
    pub cache_dir: PathBuf,
    pub global_package_dir: PathBuf,
    pub local_package_dir: PathBuf,
    pub timeout_seconds: u64,
    pub max_concurrent_downloads: usize,
    pub verify_checksums: bool,
    pub allow_pre_releases: bool,
    pub offline_mode: bool,
    pub proxy_settings: Option<ProxyConfig>,
    pub authentication: HashMap<String, AuthConfig>,
}

impl Default for PackageManagerConfig {
    fn default() -> Self {
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let cursed_dir = home_dir.join(".cursed");

        Self {
            registry_urls: vec!["https://packages.cursed-lang.org".to_string()],
            cache_dir: cursed_dir.join("cache"),
            global_package_dir: cursed_dir.join("packages"),
            local_package_dir: PathBuf::from("cursed_modules"),
            timeout_seconds: 30,
            max_concurrent_downloads: 4,
            verify_checksums: true,
            allow_pre_releases: false,
            offline_mode: false,
            proxy_settings: None,
            authentication: HashMap::new(),
        }
    }
}

impl PackageManagerConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_registry_url(mut self, url: String) -> Self {
        self.registry_urls = vec![url];
        self
    }

    pub fn add_registry_url(mut self, url: String) -> Self {
        self.registry_urls.push(url);
        self
    }

    pub fn with_cache_dir(mut self, dir: PathBuf) -> Self {
        self.cache_dir = dir;
        self
    }

    pub fn with_timeout(mut self, seconds: u64) -> Self {
        self.timeout_seconds = seconds;
        self
    }

    pub fn with_max_downloads(mut self, max: usize) -> Self {
        self.max_concurrent_downloads = max;
        self
    }

    pub fn enable_offline_mode(mut self) -> Self {
        self.offline_mode = true;
        self
    }

    pub fn allow_pre_releases(mut self) -> Self {
        self.allow_pre_releases = true;
        self
    }

    pub fn with_proxy(mut self, proxy: ProxyConfig) -> Self {
        self.proxy_settings = Some(proxy);
        self
    }

    pub fn add_auth(mut self, registry: String, auth: AuthConfig) -> Self {
        self.authentication.insert(registry, auth);
        self
    }

    pub fn create_directories(&self) -> crate::error_types::Result<()> {
        std::fs::create_dir_all(&self.cache_dir)
            .map_err(|e| CursedError::Io(e.into()))?;
        std::fs::create_dir_all(&self.global_package_dir)
            .map_err(|e| CursedError::Io(e.into()))?;
        Ok(())
    }

    pub fn load_from_file(path: &std::path::Path) -> crate::error_types::Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }

        let content = std::fs::read_to_string(path)
            .map_err(|e| CursedError::Io(e.into()))?;
        
        // TODO: Implement proper config file parsing (TOML/JSON)
        // For now, return default config
        Ok(Self::default())
    }

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
    pub http_proxy: Option<String>,
    pub https_proxy: Option<String>,
    pub no_proxy: Vec<String>,
}

impl ProxyConfig {
    pub fn new() -> Self {
        Self {
            http_proxy: None,
            https_proxy: None,
            no_proxy: Vec::new(),
        }
    }

    pub fn with_http_proxy(mut self, proxy: String) -> Self {
        self.http_proxy = Some(proxy);
        self
    }

    pub fn with_https_proxy(mut self, proxy: String) -> Self {
        self.https_proxy = Some(proxy);
        self
    }

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
    Token(String),
    Basic { username: String, password: String },
    Bearer(String),
    ApiKey { key: String, header: String },
}

impl AuthConfig {
    pub fn token(token: String) -> Self {
        Self::Token(token)
    }

    pub fn basic(username: String, password: String) -> Self {
        Self::Basic { username, password }
    }

    pub fn bearer(token: String) -> Self {
        Self::Bearer(token)
    }

    pub fn api_key(key: String, header: String) -> Self {
        Self::ApiKey { key, header }
    }

    pub fn to_header(&self) -> (String, String) {
        match self {
            AuthConfig::Token(token) => ("Authorization".to_string(), format!("Token {}", token)),
            AuthConfig::Basic { username, password } => {
                let credentials = base64::encode(format!("{}:{}", username, password));
                ("Authorization".to_string(), format!("Basic {}", credentials))
            }
            AuthConfig::Bearer(token) => ("Authorization".to_string(), format!("Bearer {}", token)),
            AuthConfig::ApiKey { key, header } => (header.clone(), key.clone()),
        }
    }
}

/// Registry configuration
#[derive(Debug, Clone)]
pub struct RegistryConfig {
    pub name: String,
    pub url: String,
    pub auth: Option<AuthConfig>,
    pub verify_ssl: bool,
    pub timeout_seconds: u64,
}

impl RegistryConfig {
    pub fn new(name: String, url: String) -> Self {
        Self {
            name,
            url,
            auth: None,
            verify_ssl: true,
            timeout_seconds: 30,
        }
    }

    pub fn with_auth(mut self, auth: AuthConfig) -> Self {
        self.auth = Some(auth);
        self
    }

    pub fn disable_ssl_verification(mut self) -> Self {
        self.verify_ssl = false;
        self
    }

    pub fn with_timeout(mut self, seconds: u64) -> Self {
        self.timeout_seconds = seconds;
        self
    }
}
