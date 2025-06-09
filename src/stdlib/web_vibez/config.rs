/// Configuration management for web server settings
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct WebVibezConfig {
    pub server: ServerConfig,
    pub security: SecurityConfig,
    pub performance: PerformanceConfig,
    pub session: SessionConfig,
    pub template: TemplateConfig,
    pub static_files: StaticFileConfig,
    pub logging: LoggingConfig,
    pub development: DevelopmentConfig,
}

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub max_connections: usize,
    pub request_timeout: Duration,
    pub keep_alive_timeout: Duration,
    pub header_timeout: Duration,
    pub max_header_size: usize,
    pub max_body_size: usize,
}

#[derive(Debug, Clone)]
pub struct SecurityConfig {
    pub csrf_secret: String,
    pub session_secret: String,
    pub enable_xss_protection: bool,
    pub enable_csrf_protection: bool,
    pub allowed_origins: Vec<String>,
    pub content_security_policy: Option<String>,
    pub hsts_max_age: Option<u64>,
    pub enable_secure_headers: bool,
}

#[derive(Debug, Clone)]
pub struct PerformanceConfig {
    pub enable_compression: bool,
    pub compression_level: u8,
    pub compression_threshold: usize,
    pub connection_pool_size: usize,
    pub keep_alive_connections: usize,
    pub enable_http2: bool,
    pub enable_caching: bool,
    pub cache_max_age: Duration,
}

#[derive(Debug, Clone)]
pub struct SessionConfig {
    pub cookie_name: String,
    pub max_age: Duration,
    pub secure: bool,
    pub http_only: bool,
    pub same_site: SameSitePolicy,
    pub store_type: SessionStoreType,
    pub cleanup_interval: Duration,
}

#[derive(Debug, Clone)]
pub struct TemplateConfig {
    pub template_dir: PathBuf,
    pub cache_templates: bool,
    pub auto_reload: bool,
    pub template_extension: String,
    pub custom_filters: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct StaticFileConfig {
    pub static_dir: PathBuf,
    pub enable_caching: bool,
    pub cache_max_age: Duration,
    pub enable_compression: bool,
    pub enable_etag: bool,
    pub enable_last_modified: bool,
    pub allowed_extensions: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct LoggingConfig {
    pub enable_request_logging: bool,
    pub enable_response_logging: bool,
    pub enable_error_logging: bool,
    pub log_level: LogLevel,
    pub log_format: LogFormat,
    pub access_log_path: Option<PathBuf>,
    pub error_log_path: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct DevelopmentConfig {
    pub enable_hot_reload: bool,
    pub enable_debug_mode: bool,
    pub enable_metrics: bool,
    pub metrics_endpoint: String,
    pub health_check_endpoint: String,
    pub debug_endpoint: Option<String>,
}

#[derive(Debug, Clone)]
pub enum SameSitePolicy {
    Strict,
    Lax,
    None,
}

#[derive(Debug, Clone)]
pub enum SessionStoreType {
    Memory,
    File(PathBuf),
    Redis(String),
    Database(String),
}

#[derive(Debug, Clone)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

#[derive(Debug, Clone)]
pub enum LogFormat {
    Common,
    Combined,
    Json,
    Custom(String),
}

impl Default for WebVibezConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            security: SecurityConfig::default(),
            performance: PerformanceConfig::default(),
            session: SessionConfig::default(),
            template: TemplateConfig::default(),
            static_files: StaticFileConfig::default(),
            logging: LoggingConfig::default(),
            development: DevelopmentConfig::default(),
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            max_connections: 1000,
            request_timeout: Duration::from_secs(30),
            keep_alive_timeout: Duration::from_secs(5),
            header_timeout: Duration::from_secs(10),
            max_header_size: 8192,
            max_body_size: 10 * 1024 * 1024, // 10MB
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            csrf_secret: "changeme".to_string(),
            session_secret: "changeme".to_string(),
            enable_xss_protection: true,
            enable_csrf_protection: true,
            allowed_origins: Vec::from(["*".to_string()]),
            content_security_policy: Some("default-src 'self'".to_string()),
            hsts_max_age: Some(31536000), // 1 year
            enable_secure_headers: true,
        }
    }
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            enable_compression: true,
            compression_level: 6,
            compression_threshold: 1024,
            connection_pool_size: 100,
            keep_alive_connections: 50,
            enable_http2: true,
            enable_caching: true,
            cache_max_age: Duration::from_secs(3600),
        }
    }
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            cookie_name: "cursed_session".to_string(),
            max_age: Duration::from_secs(24 * 60 * 60), // 24 hours
            secure: false,
            http_only: true,
            same_site: SameSitePolicy::Lax,
            store_type: SessionStoreType::Memory,
            cleanup_interval: Duration::from_secs(300), // 5 minutes
        }
    }
}

impl Default for TemplateConfig {
    fn default() -> Self {
        Self {
            template_dir: PathBuf::from("templates"),
            cache_templates: true,
            auto_reload: false,
            template_extension: ".html".to_string(),
            custom_filters: HashMap::new(),
        }
    }
}

impl Default for StaticFileConfig {
    fn default() -> Self {
        Self {
            static_dir: PathBuf::from("static"),
            enable_caching: true,
            cache_max_age: Duration::from_secs(3600),
            enable_compression: true,
            enable_etag: true,
            enable_last_modified: true,
            allowed_extensions: vec![
                ".html".to_string(),
                ".css".to_string(),
                ".js".to_string(),
                ".png".to_string(),
                ".jpg".to_string(),
                ".jpeg".to_string(),
                ".gif".to_string(),
                ".svg".to_string(),
                ".woff".to_string(),
                ".woff2".to_string(),
                ".ttf".to_string(),
                ".eot".to_string(),
            ],
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            enable_request_logging: true,
            enable_response_logging: true,
            enable_error_logging: true,
            log_level: LogLevel::Info,
            log_format: LogFormat::Combined,
            access_log_path: None,
            error_log_path: None,
        }
    }
}

impl Default for DevelopmentConfig {
    fn default() -> Self {
        Self {
            enable_hot_reload: false,
            enable_debug_mode: false,
            enable_metrics: true,
            metrics_endpoint: "/metrics".to_string(),
            health_check_endpoint: "/health".to_string(),
            debug_endpoint: None,
        }
    }
}

impl WebVibezConfig {
    /// Load configuration from file
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        Self::from_toml(&content)
    }

    /// Parse configuration from TOML string
    pub fn from_toml(toml_str: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // TODO: Implement TOML parsing when CURSED has TOML support
        // For now, return default configuration
        Ok(Self::default())
    }

    /// Convert configuration to TOML string
    pub fn to_toml(&self) -> String {
        // TODO: Implement TOML serialization when CURSED has TOML support
        // For now, return empty string
        String::new()
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.server.port == 0 {
            return Err("Server port cannot be 0".to_string());
        }

        if self.server.max_connections == 0 {
            return Err("Max connections cannot be 0".to_string());
        }

        if self.security.csrf_secret == "changeme" {
            return Err("CSRF secret must be changed from default".to_string());
        }

        if self.security.session_secret == "changeme" {
            return Err("Session secret must be changed from default".to_string());
        }

        if self.performance.compression_level > 9 {
            return Err("Compression level must be between 0 and 9".to_string());
        }

        Ok(())
    }

    /// Create production-ready configuration
    pub fn production() -> Self {
        let mut config = Self::default();
        config.server.host = "0.0.0.0".to_string();
        config.security.secure = true;
        config.development.enable_debug_mode = false;
        config.development.enable_hot_reload = false;
        config.template.auto_reload = false;
        config.template.cache_templates = true;
        config.performance.enable_compression = true;
        config.performance.enable_caching = true;
        config
    }

    /// Create development-friendly configuration
    pub fn development() -> Self {
        let mut config = Self::default();
        config.development.enable_debug_mode = true;
        config.development.enable_hot_reload = true;
        config.template.auto_reload = true;
        config.template.cache_templates = false;
        config.logging.log_level = LogLevel::Debug;
        config
    }
}
