/// Configuration management for web server settings
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::SystemTime;
use serde::{Deserialize, Serialize};
// use notify::{Watcher, RecursiveMode, watcher, DebouncedEvent};
// use std::sync::mpsc::channel;

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub max_connections: usize,
    pub request_timeout: Duration,
    pub keep_alive_timeout: Duration,
    pub header_timeout: Duration,
    pub connection_timeout: Duration,
    pub max_header_size: usize,
    pub max_body_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    pub cookie_name: String,
    pub max_age: Duration,
    pub secure: bool,
    pub http_only: bool,
    pub same_site: SameSitePolicy,
    pub store_type: SessionStoreType,
    pub cleanup_interval: Duration,
    pub database_timeout: Duration,
    pub session_timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateConfig {
    pub template_dir: PathBuf,
    pub cache_templates: bool,
    pub auto_reload: bool,
    pub template_extension: String,
    pub custom_filters: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticFileConfig {
    pub static_dir: PathBuf,
    pub enable_caching: bool,
    pub cache_max_age: Duration,
    pub enable_compression: bool,
    pub enable_etag: bool,
    pub enable_last_modified: bool,
    pub allowed_extensions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub enable_request_logging: bool,
    pub enable_response_logging: bool,
    pub enable_error_logging: bool,
    pub log_level: LogLevel,
    pub log_format: LogFormat,
    pub access_log_path: Option<PathBuf>,
    pub error_log_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevelopmentConfig {
    pub enable_hot_reload: bool,
    pub enable_debug_mode: bool,
    pub enable_metrics: bool,
    pub metrics_endpoint: String,
    pub health_check_endpoint: String,
    pub debug_endpoint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SameSitePolicy {
    Strict,
    Lax,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionStoreType {
    Memory,
    File(PathBuf),
    Redis(String),
    Database(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
            connection_timeout: Duration::from_secs(15),
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
            database_timeout: Duration::from_secs(10),
            session_timeout: Duration::from_secs(30 * 60), // 30 minutes
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

/// Configuration error types
#[derive(Debug)]
pub enum ConfigError {
    IoError(std::io::Error),
    ParseError(toml::de::Error),
    ValidationError(String),
    EnvironmentError(String),
    InvalidValue { field: String, value: String, reason: String },
    MissingRequiredField(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::IoError(e) => write!(f, "IO error: {}", e),
            ConfigError::ParseError(e) => write!(f, "Parse error: {}", e),
            ConfigError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            ConfigError::EnvironmentError(msg) => write!(f, "Environment error: {}", msg),
            ConfigError::InvalidValue { field, value, reason } => {
                write!(f, "Invalid value '{}' for field '{}': {}", value, field, reason)
            }
            ConfigError::MissingRequiredField(field) => {
                write!(f, "Missing required field: {}", field)
            }
        }
    }
}

impl std::error::Error for ConfigError {}

impl From<std::io::Error> for ConfigError {
    fn from(err: std::io::Error) -> Self {
        ConfigError::IoError(err)
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(err: toml::de::Error) -> Self {
        ConfigError::ParseError(err)
    }
}

/// Configuration watcher for hot reloading
pub struct ConfigWatcher {
    config: Arc<std::sync::RwLock<WebVibezConfig>>,
    watching: Arc<AtomicBool>,
    file_path: PathBuf,
    last_modified: SystemTime,
}

impl ConfigWatcher {
    pub fn new(config: WebVibezConfig, file_path: PathBuf) -> Result<(), Error> {
        let last_modified = std::fs::metadata(&file_path)
            .map_err(ConfigError::from)?
            .modified()
            .map_err(ConfigError::from)?;

        Ok(ConfigWatcher {
            config: Arc::new(std::sync::RwLock::new(config)),
            watching: Arc::new(AtomicBool::new(false)),
            file_path,
            last_modified,
        })
    }

    pub fn start_watching(&self) -> Result<(), Error> {
        if self.watching.load(Ordering::Relaxed) {
            return Ok(());
        }

        self.watching.store(true, Ordering::Relaxed);
        
        // Simple file modification time watching instead of complex file system watching
        let config_clone = Arc::clone(&self.config);
        let watching_clone = Arc::clone(&self.watching);
        let file_path_clone = self.file_path.clone();
        let mut last_modified = self.last_modified;

        thread::spawn(move || {
            while watching_clone.load(Ordering::Relaxed) {
                if let Ok(metadata) = std::fs::metadata(&file_path_clone) {
                    if let Ok(modified) = metadata.modified() {
                        if modified > last_modified {
                            if let Ok(new_config) = WebVibezConfig::from_file_with_env(
                                file_path_clone.to_str().unwrap()
                            ) {
                                if let Ok(mut config) = config_clone.write() {
                                    *config = new_config;
                                    last_modified = modified;
                                    println!("Configuration reloaded from {}", file_path_clone.display());
                                }
                            }
                        }
                    }
                }
                thread::sleep(Duration::from_secs(1));
            }
        });

        Ok(())
    }

    pub fn stop_watching(&self) {
        self.watching.store(false, Ordering::Relaxed);
    }

    pub fn get_config(&self) -> Arc<std::sync::RwLock<WebVibezConfig>> {
        Arc::clone(&self.config)
    }
}

/// Expand environment variables in TOML content
fn expand_environment_variables(content: &str) -> Result<(), Error> {
    let mut expanded = content.to_string();
    
    // Simple regex-like replacement for environment variables
    // Look for patterns like ${VAR_NAME} or ${VAR_NAME:default_value}
    let mut start = 0;
    while let Some(dollar_pos) = expanded[start..].find("${") {
        let abs_start = start + dollar_pos;
        if let Some(end_pos) = expanded[abs_start..].find('}') {
            let abs_end = abs_start + end_pos;
            let var_content = &expanded[abs_start + 2..abs_end];
            
            let (var_name, default_value) = if let Some(colon_pos) = var_content.find(':') {
                (&var_content[..colon_pos], Some(&var_content[colon_pos + 1..]))
            } else {
                (var_content, None)
            };
            
            let value = match std::env::var(var_name) {
                Ok(val) => val,
                Err(_) => {
                    if let Some(default) = default_value {
                        default.to_string()
                    } else {
                        return Err(ConfigError::EnvironmentError(
                            format!("Environment variable '{}' not found and no default provided", var_name)
                        ));
                    }
                }
            };
            
            expanded.replace_range(abs_start..=abs_end, &value);
            start = abs_start + value.len();
        } else {
            start = abs_start + 2;
        }
    }
    
    Ok(expanded)
}

impl WebVibezConfig {
    /// Load configuration from file with enhanced error handling
    pub fn from_file(path: &str) -> Result<(), Error> {
        let content = std::fs::read_to_string(path)?;
        Self::from_toml(&content)
    }

    /// Load configuration with environment variable support
    pub fn from_file_with_env(path: &str) -> Result<(), Error> {
        let content = std::fs::read_to_string(path)?;
        let expanded_content = expand_environment_variables(&content)?;
        Self::from_toml_enhanced(&expanded_content)
    }

    /// Parse configuration from TOML string (legacy method)
    pub fn from_toml(toml_str: &str) -> Result<(), Error> {
        // Parse the TOML string into a toml::Value first for custom handling
        let toml_value: toml::Value = toml::from_str(toml_str)?;
        
        // Convert the toml::Value into our config struct with custom deserializer
        let mut config = Self::default();
        
        if let toml::Value::Table(table) = toml_value {
            // Parse server config
            if let Some(server_table) = table.get("server").and_then(|v| v.as_table()) {
                config.server = parse_server_config(server_table)?;
            }
            
            // Parse security config
            if let Some(security_table) = table.get("security").and_then(|v| v.as_table()) {
                config.security = parse_security_config(security_table)?;
            }
            
            // Parse performance config
            if let Some(performance_table) = table.get("performance").and_then(|v| v.as_table()) {
                config.performance = parse_performance_config(performance_table)?;
            }
            
            // Parse session config
            if let Some(session_table) = table.get("session").and_then(|v| v.as_table()) {
                config.session = parse_session_config(session_table)?;
            }
            
            // Parse template config
            if let Some(template_table) = table.get("template").and_then(|v| v.as_table()) {
                config.template = parse_template_config(template_table)?;
            }
            
            // Parse static files config
            if let Some(static_table) = table.get("static_files").and_then(|v| v.as_table()) {
                config.static_files = parse_static_file_config(static_table)?;
            }
            
            // Parse logging config
            if let Some(logging_table) = table.get("logging").and_then(|v| v.as_table()) {
                config.logging = parse_logging_config(logging_table)?;
            }
            
            // Parse development config
            if let Some(dev_table) = table.get("development").and_then(|v| v.as_table()) {
                config.development = parse_development_config(dev_table)?;
            }
        }
        
        Ok(config)
    }

    /// Parse configuration from TOML string with comprehensive validation
    pub fn from_toml_enhanced(toml_str: &str) -> Result<(), Error> {
        // Parse the TOML string into a toml::Value first for custom handling
        let toml_value: toml::Value = toml::from_str(toml_str)?;
        
        // Convert the toml::Value into our config struct with enhanced validation
        let mut config = Self::default();
        
        if let toml::Value::Table(table) = toml_value {
            // Parse server config with validation
            if let Some(server_table) = table.get("server").and_then(|v| v.as_table()) {
                config.server = parse_server_config_enhanced(server_table)?;
            }
            
            // Parse security config with validation
            if let Some(security_table) = table.get("security").and_then(|v| v.as_table()) {
                config.security = parse_security_config_enhanced(security_table)?;
            }
            
            // Parse performance config with validation
            if let Some(performance_table) = table.get("performance").and_then(|v| v.as_table()) {
                config.performance = parse_performance_config_enhanced(performance_table)?;
            }
            
            // Parse session config with validation
            if let Some(session_table) = table.get("session").and_then(|v| v.as_table()) {
                config.session = parse_session_config_enhanced(session_table)?;
            }
            
            // Parse template config with validation
            if let Some(template_table) = table.get("template").and_then(|v| v.as_table()) {
                config.template = parse_template_config_enhanced(template_table)?;
            }
            
            // Parse static files config with validation
            if let Some(static_table) = table.get("static_files").and_then(|v| v.as_table()) {
                config.static_files = parse_static_file_config_enhanced(static_table)?;
            }
            
            // Parse logging config with validation
            if let Some(logging_table) = table.get("logging").and_then(|v| v.as_table()) {
                config.logging = parse_logging_config_enhanced(logging_table)?;
            }
            
            // Parse development config with validation
            if let Some(dev_table) = table.get("development").and_then(|v| v.as_table()) {
                config.development = parse_development_config_enhanced(dev_table)?;
            }
        }
        
        // Validate the complete configuration
        config.validate_enhanced()?;
        
        Ok(config)
    }

    /// Create a configuration watcher for hot reloading
    pub fn create_watcher(path: &str) -> Result<(), Error> {
        let config = Self::from_file_with_env(path)?;
        ConfigWatcher::new(config, PathBuf::from(path))
    }

    /// Convert configuration to TOML string
    pub fn to_toml(&self) -> String {
        let mut toml_string = String::new();
        
        // Server section
        toml_string.push_str("[server]\n");
        toml_string.push_str(&format!("host = \"{}\"\n", self.server.host));
        toml_string.push_str(&format!("port = {}\n", self.server.port));
        toml_string.push_str(&format!("max_connections = {}\n", self.server.max_connections));
        toml_string.push_str(&format!("request_timeout = {}\n", self.server.request_timeout.as_secs()));
        toml_string.push_str(&format!("keep_alive_timeout = {}\n", self.server.keep_alive_timeout.as_secs()));
        toml_string.push_str(&format!("header_timeout = {}\n", self.server.header_timeout.as_secs()));
        toml_string.push_str(&format!("max_header_size = {}\n", self.server.max_header_size));
        toml_string.push_str(&format!("max_body_size = {}\n", self.server.max_body_size));
        toml_string.push_str("\n");
        
        // Security section
        toml_string.push_str("[security]\n");
        toml_string.push_str(&format!("csrf_secret = \"{}\"\n", self.security.csrf_secret));
        toml_string.push_str(&format!("session_secret = \"{}\"\n", self.security.session_secret));
        toml_string.push_str(&format!("enable_xss_protection = {}\n", self.security.enable_xss_protection));
        toml_string.push_str(&format!("enable_csrf_protection = {}\n", self.security.enable_csrf_protection));
        toml_string.push_str(&format!("allowed_origins = {:?}\n", self.security.allowed_origins));
        if let Some(ref csp) = self.security.content_security_policy {
            toml_string.push_str(&format!("content_security_policy = \"{}\"\n", csp));
        }
        if let Some(hsts) = self.security.hsts_max_age {
            toml_string.push_str(&format!("hsts_max_age = {}\n", hsts));
        }
        toml_string.push_str(&format!("enable_secure_headers = {}\n", self.security.enable_secure_headers));
        toml_string.push_str("\n");
        
        // Performance section
        toml_string.push_str("[performance]\n");
        toml_string.push_str(&format!("enable_compression = {}\n", self.performance.enable_compression));
        toml_string.push_str(&format!("compression_level = {}\n", self.performance.compression_level));
        toml_string.push_str(&format!("compression_threshold = {}\n", self.performance.compression_threshold));
        toml_string.push_str(&format!("connection_pool_size = {}\n", self.performance.connection_pool_size));
        toml_string.push_str(&format!("keep_alive_connections = {}\n", self.performance.keep_alive_connections));
        toml_string.push_str(&format!("enable_http2 = {}\n", self.performance.enable_http2));
        toml_string.push_str(&format!("enable_caching = {}\n", self.performance.enable_caching));
        toml_string.push_str(&format!("cache_max_age = {}\n", self.performance.cache_max_age.as_secs()));
        toml_string.push_str("\n");
        
        // Session section
        toml_string.push_str("[session]\n");
        toml_string.push_str(&format!("cookie_name = \"{}\"\n", self.session.cookie_name));
        toml_string.push_str(&format!("max_age = {}\n", self.session.max_age.as_secs()));
        toml_string.push_str(&format!("secure = {}\n", self.session.secure));
        toml_string.push_str(&format!("http_only = {}\n", self.session.http_only));
        toml_string.push_str(&format!("same_site = \"{:?}\"\n", self.session.same_site));
        toml_string.push_str(&format!("store_type = \"{:?}\"\n", self.session.store_type));
        toml_string.push_str(&format!("cleanup_interval = {}\n", self.session.cleanup_interval.as_secs()));
        toml_string.push_str("\n");
        
        // Template section
        toml_string.push_str("[template]\n");
        toml_string.push_str(&format!("template_dir = \"{}\"\n", self.template.template_dir.display()));
        toml_string.push_str(&format!("cache_templates = {}\n", self.template.cache_templates));
        toml_string.push_str(&format!("auto_reload = {}\n", self.template.auto_reload));
        toml_string.push_str(&format!("template_extension = \"{}\"\n", self.template.template_extension));
        toml_string.push_str("\n");
        
        // Static files section
        toml_string.push_str("[static_files]\n");
        toml_string.push_str(&format!("static_dir = \"{}\"\n", self.static_files.static_dir.display()));
        toml_string.push_str(&format!("enable_caching = {}\n", self.static_files.enable_caching));
        toml_string.push_str(&format!("cache_max_age = {}\n", self.static_files.cache_max_age.as_secs()));
        toml_string.push_str(&format!("enable_compression = {}\n", self.static_files.enable_compression));
        toml_string.push_str(&format!("enable_etag = {}\n", self.static_files.enable_etag));
        toml_string.push_str(&format!("enable_last_modified = {}\n", self.static_files.enable_last_modified));
        toml_string.push_str(&format!("allowed_extensions = {:?}\n", self.static_files.allowed_extensions));
        toml_string.push_str("\n");
        
        // Logging section
        toml_string.push_str("[logging]\n");
        toml_string.push_str(&format!("enable_request_logging = {}\n", self.logging.enable_request_logging));
        toml_string.push_str(&format!("enable_response_logging = {}\n", self.logging.enable_response_logging));
        toml_string.push_str(&format!("enable_error_logging = {}\n", self.logging.enable_error_logging));
        toml_string.push_str(&format!("log_level = \"{:?}\"\n", self.logging.log_level));
        toml_string.push_str(&format!("log_format = \"{:?}\"\n", self.logging.log_format));
        if let Some(ref path) = self.logging.access_log_path {
            toml_string.push_str(&format!("access_log_path = \"{}\"\n", path.display()));
        }
        if let Some(ref path) = self.logging.error_log_path {
            toml_string.push_str(&format!("error_log_path = \"{}\"\n", path.display()));
        }
        toml_string.push_str("\n");
        
        // Development section
        toml_string.push_str("[development]\n");
        toml_string.push_str(&format!("enable_hot_reload = {}\n", self.development.enable_hot_reload));
        toml_string.push_str(&format!("enable_debug_mode = {}\n", self.development.enable_debug_mode));
        toml_string.push_str(&format!("enable_metrics = {}\n", self.development.enable_metrics));
        toml_string.push_str(&format!("metrics_endpoint = \"{}\"\n", self.development.metrics_endpoint));
        toml_string.push_str(&format!("health_check_endpoint = \"{}\"\n", self.development.health_check_endpoint));
        if let Some(ref endpoint) = self.development.debug_endpoint {
            toml_string.push_str(&format!("debug_endpoint = \"{}\"\n", endpoint));
        }
        
        toml_string
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

    /// Enhanced validation with comprehensive checks
    pub fn validate_enhanced(&self) -> Result<(), Error> {
        // Server validation
        if self.server.port == 0 {
            return Err(ConfigError::InvalidValue {
                field: "server.port".to_string(),
                value: "0".to_string(),
                reason: "Port cannot be 0".to_string(),
            });
        }

        if self.server.port > 65535 {
            return Err(ConfigError::InvalidValue {
                field: "server.port".to_string(),
                value: self.server.port.to_string(),
                reason: "Port must be <= 65535".to_string(),
            });
        }

        if self.server.max_connections == 0 {
            return Err(ConfigError::InvalidValue {
                field: "server.max_connections".to_string(),
                value: "0".to_string(),
                reason: "Max connections cannot be 0".to_string(),
            });
        }

        if self.server.max_header_size < 1024 {
            return Err(ConfigError::InvalidValue {
                field: "server.max_header_size".to_string(),
                value: self.server.max_header_size.to_string(),
                reason: "Header size should be at least 1KB".to_string(),
            });
        }

        if self.server.max_body_size < 1024 {
            return Err(ConfigError::InvalidValue {
                field: "server.max_body_size".to_string(),
                value: self.server.max_body_size.to_string(),
                reason: "Body size should be at least 1KB".to_string(),
            });
        }

        // Security validation
        if self.security.csrf_secret == "changeme" {
            return Err(ConfigError::ValidationError(
                "CSRF secret must be changed from default 'changeme'".to_string()
            ));
        }

        if self.security.session_secret == "changeme" {
            return Err(ConfigError::ValidationError(
                "Session secret must be changed from default 'changeme'".to_string()
            ));
        }

        if self.security.csrf_secret.len() < 32 {
            return Err(ConfigError::InvalidValue {
                field: "security.csrf_secret".to_string(),
                value: "[REDACTED]".to_string(),
                reason: "CSRF secret should be at least 32 characters".to_string(),
            });
        }

        if self.security.session_secret.len() < 32 {
            return Err(ConfigError::InvalidValue {
                field: "security.session_secret".to_string(),
                value: "[REDACTED]".to_string(),
                reason: "Session secret should be at least 32 characters".to_string(),
            });
        }

        // Performance validation
        if self.performance.compression_level > 9 {
            return Err(ConfigError::InvalidValue {
                field: "performance.compression_level".to_string(),
                value: self.performance.compression_level.to_string(),
                reason: "Compression level must be between 0 and 9".to_string(),
            });
        }

        if self.performance.connection_pool_size == 0 {
            return Err(ConfigError::InvalidValue {
                field: "performance.connection_pool_size".to_string(),
                value: "0".to_string(),
                reason: "Connection pool size cannot be 0".to_string(),
            });
        }

        // Template validation
        if !self.template.template_dir.exists() && !self.development.enable_debug_mode {
            return Err(ConfigError::ValidationError(
                format!("Template directory does not exist: {}", self.template.template_dir.display())
            ));
        }

        // Static files validation
        if !self.static_files.static_dir.exists() && !self.development.enable_debug_mode {
            return Err(ConfigError::ValidationError(
                format!("Static files directory does not exist: {}", self.static_files.static_dir.display())
            ));
        }

        // Session validation
        if self.session.max_age.as_secs() == 0 {
            return Err(ConfigError::InvalidValue {
                field: "session.max_age".to_string(),
                value: "0".to_string(),
                reason: "Session max age cannot be 0".to_string(),
            });
        }

        Ok(())
    }

    /// Create production-ready configuration
    pub fn production() -> Self {
        let mut config = Self::default();
        config.server.host = "0.0.0.0".to_string();
        config.security.enable_secure_headers = true;
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

// Helper functions for parsing TOML sections
fn parse_server_config(table: &toml::map::Map<String, toml::Value>) -> Result<(), Error> {
    let mut config = ServerConfig::default();
    
    if let Some(host) = table.get("host").and_then(|v| v.as_str()) {
        config.host = host.to_string();
    }
    if let Some(port) = table.get("port").and_then(|v| v.as_integer()) {
        config.port = port as u16;
    }
    if let Some(max_conn) = table.get("max_connections").and_then(|v| v.as_integer()) {
        config.max_connections = max_conn as usize;
    }
    if let Some(timeout) = table.get("request_timeout").and_then(|v| v.as_integer()) {
        config.request_timeout = Duration::from_secs(timeout as u64);
    }
    if let Some(timeout) = table.get("keep_alive_timeout").and_then(|v| v.as_integer()) {
        config.keep_alive_timeout = Duration::from_secs(timeout as u64);
    }
    if let Some(timeout) = table.get("header_timeout").and_then(|v| v.as_integer()) {
        config.header_timeout = Duration::from_secs(timeout as u64);
    }
    if let Some(timeout) = table.get("connection_timeout").and_then(|v| v.as_integer()) {
        config.connection_timeout = Duration::from_secs(timeout as u64);
    }
    if let Some(size) = table.get("max_header_size").and_then(|v| v.as_integer()) {
        config.max_header_size = size as usize;
    }
    if let Some(size) = table.get("max_body_size").and_then(|v| v.as_integer()) {
        config.max_body_size = size as usize;
    }
    
    Ok(config)
}

fn parse_security_config(table: &toml::map::Map<String, toml::Value>) -> Result<(), Error> {
    let mut config = SecurityConfig::default();
    
    if let Some(secret) = table.get("csrf_secret").and_then(|v| v.as_str()) {
        config.csrf_secret = secret.to_string();
    }
    if let Some(secret) = table.get("session_secret").and_then(|v| v.as_str()) {
        config.session_secret = secret.to_string();
    }
    if let Some(enable) = table.get("enable_xss_protection").and_then(|v| v.as_bool()) {
        config.enable_xss_protection = enable;
    }
    if let Some(enable) = table.get("enable_csrf_protection").and_then(|v| v.as_bool()) {
        config.enable_csrf_protection = enable;
    }
    if let Some(origins) = table.get("allowed_origins").and_then(|v| v.as_array()) {
        config.allowed_origins = origins.iter()
            .filter_map(|v| v.as_str())
            .map(|s| s.to_string())
            .collect();
    }
    if let Some(csp) = table.get("content_security_policy").and_then(|v| v.as_str()) {
        config.content_security_policy = Some(csp.to_string());
    }
    if let Some(hsts) = table.get("hsts_max_age").and_then(|v| v.as_integer()) {
        config.hsts_max_age = Some(hsts as u64);
    }
    if let Some(enable) = table.get("enable_secure_headers").and_then(|v| v.as_bool()) {
        config.enable_secure_headers = enable;
    }
    
    Ok(config)
}

fn parse_performance_config(table: &toml::map::Map<String, toml::Value>) -> Result<(), Error> {
    let mut config = PerformanceConfig::default();
    
    if let Some(enable) = table.get("enable_compression").and_then(|v| v.as_bool()) {
        config.enable_compression = enable;
    }
    if let Some(level) = table.get("compression_level").and_then(|v| v.as_integer()) {
        config.compression_level = level as u8;
    }
    if let Some(threshold) = table.get("compression_threshold").and_then(|v| v.as_integer()) {
        config.compression_threshold = threshold as usize;
    }
    if let Some(pool_size) = table.get("connection_pool_size").and_then(|v| v.as_integer()) {
        config.connection_pool_size = pool_size as usize;
    }
    if let Some(keep_alive) = table.get("keep_alive_connections").and_then(|v| v.as_integer()) {
        config.keep_alive_connections = keep_alive as usize;
    }
    if let Some(enable) = table.get("enable_http2").and_then(|v| v.as_bool()) {
        config.enable_http2 = enable;
    }
    if let Some(enable) = table.get("enable_caching").and_then(|v| v.as_bool()) {
        config.enable_caching = enable;
    }
    if let Some(max_age) = table.get("cache_max_age").and_then(|v| v.as_integer()) {
        config.cache_max_age = Duration::from_secs(max_age as u64);
    }
    
    Ok(config)
}

fn parse_session_config(table: &toml::map::Map<String, toml::Value>) -> Result<(), Error> {
    let mut config = SessionConfig::default();
    
    if let Some(name) = table.get("cookie_name").and_then(|v| v.as_str()) {
        config.cookie_name = name.to_string();
    }
    if let Some(max_age) = table.get("max_age").and_then(|v| v.as_integer()) {
        config.max_age = Duration::from_secs(max_age as u64);
    }
    if let Some(secure) = table.get("secure").and_then(|v| v.as_bool()) {
        config.secure = secure;
    }
    if let Some(http_only) = table.get("http_only").and_then(|v| v.as_bool()) {
        config.http_only = http_only;
    }
    if let Some(same_site) = table.get("same_site").and_then(|v| v.as_str()) {
        config.same_site = match same_site {
            "Strict" => SameSitePolicy::Strict,
            "Lax" => SameSitePolicy::Lax,
            "None" => SameSitePolicy::None,
            _ => SameSitePolicy::Lax,
        };
    }
    if let Some(store_type) = table.get("store_type").and_then(|v| v.as_str()) {
        config.store_type = match store_type {
            "Memory" => SessionStoreType::Memory,
            s if s.starts_with("File(") => {
                let path = s.trim_start_matches("File(").trim_end_matches(")");
                SessionStoreType::File(PathBuf::from(path))
            }
            s if s.starts_with("Redis(") => {
                let conn = s.trim_start_matches("Redis(").trim_end_matches(")");
                SessionStoreType::Redis(conn.to_string())
            }
            s if s.starts_with("Database(") => {
                let conn = s.trim_start_matches("Database(").trim_end_matches(")");
                SessionStoreType::Database(conn.to_string())
            }
            _ => SessionStoreType::Memory,
        };
    }
    if let Some(interval) = table.get("cleanup_interval").and_then(|v| v.as_integer()) {
        config.cleanup_interval = Duration::from_secs(interval as u64);
    }
    if let Some(timeout) = table.get("database_timeout").and_then(|v| v.as_integer()) {
        config.database_timeout = Duration::from_secs(timeout as u64);
    }
    if let Some(timeout) = table.get("session_timeout").and_then(|v| v.as_integer()) {
        config.session_timeout = Duration::from_secs(timeout as u64);
    }
    
    Ok(config)
}

fn parse_template_config(table: &toml::map::Map<String, toml::Value>) -> Result<(), Error> {
    let mut config = TemplateConfig::default();
    
    if let Some(dir) = table.get("template_dir").and_then(|v| v.as_str()) {
        config.template_dir = PathBuf::from(dir);
    }
    if let Some(cache) = table.get("cache_templates").and_then(|v| v.as_bool()) {
        config.cache_templates = cache;
    }
    if let Some(auto_reload) = table.get("auto_reload").and_then(|v| v.as_bool()) {
        config.auto_reload = auto_reload;
    }
    if let Some(ext) = table.get("template_extension").and_then(|v| v.as_str()) {
        config.template_extension = ext.to_string();
    }
    if let Some(filters) = table.get("custom_filters").and_then(|v| v.as_table()) {
        for (key, value) in filters {
            if let Some(filter_value) = value.as_str() {
                config.custom_filters.insert(key.clone(), filter_value.to_string());
            }
        }
    }
    
    Ok(config)
}

fn parse_static_file_config(table: &toml::map::Map<String, toml::Value>) -> Result<(), Error> {
    let mut config = StaticFileConfig::default();
    
    if let Some(dir) = table.get("static_dir").and_then(|v| v.as_str()) {
        config.static_dir = PathBuf::from(dir);
    }
    if let Some(enable) = table.get("enable_caching").and_then(|v| v.as_bool()) {
        config.enable_caching = enable;
    }
    if let Some(max_age) = table.get("cache_max_age").and_then(|v| v.as_integer()) {
        config.cache_max_age = Duration::from_secs(max_age as u64);
    }
    if let Some(enable) = table.get("enable_compression").and_then(|v| v.as_bool()) {
        config.enable_compression = enable;
    }
    if let Some(enable) = table.get("enable_etag").and_then(|v| v.as_bool()) {
        config.enable_etag = enable;
    }
    if let Some(enable) = table.get("enable_last_modified").and_then(|v| v.as_bool()) {
        config.enable_last_modified = enable;
    }
    if let Some(extensions) = table.get("allowed_extensions").and_then(|v| v.as_array()) {
        config.allowed_extensions = extensions.iter()
            .filter_map(|v| v.as_str())
            .map(|s| s.to_string())
            .collect();
    }
    
    Ok(config)
}

fn parse_logging_config(table: &toml::map::Map<String, toml::Value>) -> Result<(), Error> {
    let mut config = LoggingConfig::default();
    
    if let Some(enable) = table.get("enable_request_logging").and_then(|v| v.as_bool()) {
        config.enable_request_logging = enable;
    }
    if let Some(enable) = table.get("enable_response_logging").and_then(|v| v.as_bool()) {
        config.enable_response_logging = enable;
    }
    if let Some(enable) = table.get("enable_error_logging").and_then(|v| v.as_bool()) {
        config.enable_error_logging = enable;
    }
    if let Some(level) = table.get("log_level").and_then(|v| v.as_str()) {
        config.log_level = match level {
            "Error" => LogLevel::Error,
            "Warn" => LogLevel::Warn,
            "Info" => LogLevel::Info,
            "Debug" => LogLevel::Debug,
            "Trace" => LogLevel::Trace,
            _ => LogLevel::Info,
        };
    }
    if let Some(format) = table.get("log_format").and_then(|v| v.as_str()) {
        config.log_format = match format {
            "Common" => LogFormat::Common,
            "Combined" => LogFormat::Combined,
            "Json" => LogFormat::Json,
            s if s.starts_with("Custom(") => {
                let custom = s.trim_start_matches("Custom(").trim_end_matches(")");
                LogFormat::Custom(custom.to_string())
            }
            _ => LogFormat::Combined,
        };
    }
    if let Some(path) = table.get("access_log_path").and_then(|v| v.as_str()) {
        config.access_log_path = Some(PathBuf::from(path));
    }
    if let Some(path) = table.get("error_log_path").and_then(|v| v.as_str()) {
        config.error_log_path = Some(PathBuf::from(path));
    }
    
    Ok(config)
}

fn parse_development_config(table: &toml::map::Map<String, toml::Value>) -> Result<(), Error> {
    let mut config = DevelopmentConfig::default();
    
    if let Some(enable) = table.get("enable_hot_reload").and_then(|v| v.as_bool()) {
        config.enable_hot_reload = enable;
    }
    if let Some(enable) = table.get("enable_debug_mode").and_then(|v| v.as_bool()) {
        config.enable_debug_mode = enable;
    }
    if let Some(enable) = table.get("enable_metrics").and_then(|v| v.as_bool()) {
        config.enable_metrics = enable;
    }
    if let Some(endpoint) = table.get("metrics_endpoint").and_then(|v| v.as_str()) {
        config.metrics_endpoint = endpoint.to_string();
    }
    if let Some(endpoint) = table.get("health_check_endpoint").and_then(|v| v.as_str()) {
        config.health_check_endpoint = endpoint.to_string();
    }
    if let Some(endpoint) = table.get("debug_endpoint").and_then(|v| v.as_str()) {
        config.debug_endpoint = Some(endpoint.to_string());
    }
    
    Ok(config)
}

// Enhanced parsing functions with comprehensive validation

fn parse_server_config_enhanced(table: &toml::map::Map<String, toml::Value>) -> Result<(), Error> {
    let mut config = ServerConfig::default();
    
    if let Some(host) = table.get("host").and_then(|v| v.as_str()) {
        config.host = host.to_string();
    }
    
    if let Some(port) = table.get("port") {
        match port.as_integer() {
            Some(p) => {
                if p < 1 || p > 65535 {
                    return Err(ConfigError::InvalidValue {
                        field: "server.port".to_string(),
                        value: p.to_string(),
                        reason: "Port must be between 1 and 65535".to_string(),
                    });
                }
                config.port = p as u16;
            }
            None => return Err(ConfigError::InvalidValue {
                field: "server.port".to_string(),
                value: format!("{:?}", port),
                reason: "Port must be a number".to_string(),
            }),
        }
    }
    
    if let Some(max_conn) = table.get("max_connections") {
        match max_conn.as_integer() {
            Some(mc) if mc > 0 => config.max_connections = mc as usize,
            Some(mc) => return Err(ConfigError::InvalidValue {
                field: "server.max_connections".to_string(),
                value: mc.to_string(),
                reason: "Max connections must be greater than 0".to_string(),
            }),
            None => return Err(ConfigError::InvalidValue {
                field: "server.max_connections".to_string(),
                value: format!("{:?}", max_conn),
                reason: "Max connections must be a number".to_string(),
            }),
        }
    }
    
    if let Some(timeout) = table.get("request_timeout") {
        match timeout.as_integer() {
            Some(t) if t > 0 => config.request_timeout = Duration::from_secs(t as u64),
            Some(t) => return Err(ConfigError::InvalidValue {
                field: "server.request_timeout".to_string(),
                value: t.to_string(),
                reason: "Request timeout must be greater than 0".to_string(),
            }),
            None => return Err(ConfigError::InvalidValue {
                field: "server.request_timeout".to_string(),
                value: format!("{:?}", timeout),
                reason: "Request timeout must be a number".to_string(),
            }),
        }
    }
    
    if let Some(size) = table.get("max_header_size") {
        match size.as_integer() {
            Some(s) if s >= 1024 => config.max_header_size = s as usize,
            Some(s) => return Err(ConfigError::InvalidValue {
                field: "server.max_header_size".to_string(),
                value: s.to_string(),
                reason: "Max header size must be at least 1024 bytes".to_string(),
            }),
            None => return Err(ConfigError::InvalidValue {
                field: "server.max_header_size".to_string(),
                value: format!("{:?}", size),
                reason: "Max header size must be a number".to_string(),
            }),
        }
    }
    
    if let Some(size) = table.get("max_body_size") {
        match size.as_integer() {
            Some(s) if s >= 1024 => config.max_body_size = s as usize,
            Some(s) => return Err(ConfigError::InvalidValue {
                field: "server.max_body_size".to_string(),
                value: s.to_string(),
                reason: "Max body size must be at least 1024 bytes".to_string(),
            }),
            None => return Err(ConfigError::InvalidValue {
                field: "server.max_body_size".to_string(),
                value: format!("{:?}", size),
                reason: "Max body size must be a number".to_string(),
            }),
        }
    }
    
    Ok(config)
}

fn parse_security_config_enhanced(table: &toml::map::Map<String, toml::Value>) -> Result<(), Error> {
    let mut config = SecurityConfig::default();
    
    if let Some(secret) = table.get("csrf_secret").and_then(|v| v.as_str()) {
        if secret == "changeme" {
            return Err(ConfigError::ValidationError(
                "CSRF secret must be changed from default 'changeme'".to_string()
            ));
        }
        if secret.len() < 32 {
            return Err(ConfigError::InvalidValue {
                field: "security.csrf_secret".to_string(),
                value: "[REDACTED]".to_string(),
                reason: "CSRF secret must be at least 32 characters".to_string(),
            });
        }
        config.csrf_secret = secret.to_string();
    }
    
    if let Some(secret) = table.get("session_secret").and_then(|v| v.as_str()) {
        if secret == "changeme" {
            return Err(ConfigError::ValidationError(
                "Session secret must be changed from default 'changeme'".to_string()
            ));
        }
        if secret.len() < 32 {
            return Err(ConfigError::InvalidValue {
                field: "security.session_secret".to_string(),
                value: "[REDACTED]".to_string(),
                reason: "Session secret must be at least 32 characters".to_string(),
            });
        }
        config.session_secret = secret.to_string();
    }
    
    if let Some(enable) = table.get("enable_xss_protection").and_then(|v| v.as_bool()) {
        config.enable_xss_protection = enable;
    }
    if let Some(enable) = table.get("enable_csrf_protection").and_then(|v| v.as_bool()) {
        config.enable_csrf_protection = enable;
    }
    if let Some(origins) = table.get("allowed_origins").and_then(|v| v.as_array()) {
        config.allowed_origins = origins.iter()
            .filter_map(|v| v.as_str())
            .map(|s| s.to_string())
            .collect();
    }
    if let Some(csp) = table.get("content_security_policy").and_then(|v| v.as_str()) {
        config.content_security_policy = Some(csp.to_string());
    }
    if let Some(hsts) = table.get("hsts_max_age").and_then(|v| v.as_integer()) {
        config.hsts_max_age = Some(hsts as u64);
    }
    if let Some(enable) = table.get("enable_secure_headers").and_then(|v| v.as_bool()) {
        config.enable_secure_headers = enable;
    }
    
    Ok(config)
}

fn parse_performance_config_enhanced(table: &toml::map::Map<String, toml::Value>) -> Result<(), Error> {
    let mut config = PerformanceConfig::default();
    
    if let Some(enable) = table.get("enable_compression").and_then(|v| v.as_bool()) {
        config.enable_compression = enable;
    }
    
    if let Some(level) = table.get("compression_level") {
        match level.as_integer() {
            Some(l) if l >= 0 && l <= 9 => config.compression_level = l as u8,
            Some(l) => return Err(ConfigError::InvalidValue {
                field: "performance.compression_level".to_string(),
                value: l.to_string(),
                reason: "Compression level must be between 0 and 9".to_string(),
            }),
            None => return Err(ConfigError::InvalidValue {
                field: "performance.compression_level".to_string(),
                value: format!("{:?}", level),
                reason: "Compression level must be a number".to_string(),
            }),
        }
    }
    
    if let Some(threshold) = table.get("compression_threshold") {
        match threshold.as_integer() {
            Some(t) if t > 0 => config.compression_threshold = t as usize,
            Some(t) => return Err(ConfigError::InvalidValue {
                field: "performance.compression_threshold".to_string(),
                value: t.to_string(),
                reason: "Compression threshold must be greater than 0".to_string(),
            }),
            None => return Err(ConfigError::InvalidValue {
                field: "performance.compression_threshold".to_string(),
                value: format!("{:?}", threshold),
                reason: "Compression threshold must be a number".to_string(),
            }),
        }
    }
    
    if let Some(pool_size) = table.get("connection_pool_size") {
        match pool_size.as_integer() {
            Some(ps) if ps > 0 => config.connection_pool_size = ps as usize,
            Some(ps) => return Err(ConfigError::InvalidValue {
                field: "performance.connection_pool_size".to_string(),
                value: ps.to_string(),
                reason: "Connection pool size must be greater than 0".to_string(),
            }),
            None => return Err(ConfigError::InvalidValue {
                field: "performance.connection_pool_size".to_string(),
                value: format!("{:?}", pool_size),
                reason: "Connection pool size must be a number".to_string(),
            }),
        }
    }
    
    if let Some(keep_alive) = table.get("keep_alive_connections").and_then(|v| v.as_integer()) {
        config.keep_alive_connections = keep_alive as usize;
    }
    if let Some(enable) = table.get("enable_http2").and_then(|v| v.as_bool()) {
        config.enable_http2 = enable;
    }
    if let Some(enable) = table.get("enable_caching").and_then(|v| v.as_bool()) {
        config.enable_caching = enable;
    }
    if let Some(max_age) = table.get("cache_max_age").and_then(|v| v.as_integer()) {
        config.cache_max_age = Duration::from_secs(max_age as u64);
    }
    
    Ok(config)
}

fn parse_session_config_enhanced(table: &toml::map::Map<String, toml::Value>) -> Result<(), Error> {
    let mut config = SessionConfig::default();
    
    if let Some(name) = table.get("cookie_name").and_then(|v| v.as_str()) {
        if name.is_empty() {
            return Err(ConfigError::InvalidValue {
                field: "session.cookie_name".to_string(),
                value: "".to_string(),
                reason: "Cookie name cannot be empty".to_string(),
            });
        }
        config.cookie_name = name.to_string();
    }
    
    if let Some(max_age) = table.get("max_age") {
        match max_age.as_integer() {
            Some(ma) if ma > 0 => config.max_age = Duration::from_secs(ma as u64),
            Some(ma) => return Err(ConfigError::InvalidValue {
                field: "session.max_age".to_string(),
                value: ma.to_string(),
                reason: "Session max age must be greater than 0".to_string(),
            }),
            None => return Err(ConfigError::InvalidValue {
                field: "session.max_age".to_string(),
                value: format!("{:?}", max_age),
                reason: "Session max age must be a number".to_string(),
            }),
        }
    }
    
    if let Some(secure) = table.get("secure").and_then(|v| v.as_bool()) {
        config.secure = secure;
    }
    if let Some(http_only) = table.get("http_only").and_then(|v| v.as_bool()) {
        config.http_only = http_only;
    }
    if let Some(same_site) = table.get("same_site").and_then(|v| v.as_str()) {
        config.same_site = match same_site {
            "Strict" => SameSitePolicy::Strict,
            "Lax" => SameSitePolicy::Lax,
            "None" => SameSitePolicy::None,
            _ => return Err(ConfigError::InvalidValue {
                field: "session.same_site".to_string(),
                value: same_site.to_string(),
                reason: "Same site policy must be 'Strict', 'Lax', or 'None'".to_string(),
            }),
        };
    }
    if let Some(store_type) = table.get("store_type").and_then(|v| v.as_str()) {
        config.store_type = match store_type {
            "Memory" => SessionStoreType::Memory,
            s if s.starts_with("File(") => {
                let path = s.trim_start_matches("File(").trim_end_matches(")");
                SessionStoreType::File(PathBuf::from(path))
            }
            s if s.starts_with("Redis(") => {
                let conn = s.trim_start_matches("Redis(").trim_end_matches(")");
                SessionStoreType::Redis(conn.to_string())
            }
            s if s.starts_with("Database(") => {
                let conn = s.trim_start_matches("Database(").trim_end_matches(")");
                SessionStoreType::Database(conn.to_string())
            }
            _ => return Err(ConfigError::InvalidValue {
                field: "session.store_type".to_string(),
                value: store_type.to_string(),
                reason: "Invalid session store type".to_string(),
            }),
        };
    }
    if let Some(interval) = table.get("cleanup_interval").and_then(|v| v.as_integer()) {
        config.cleanup_interval = Duration::from_secs(interval as u64);
    }
    
    Ok(config)
}

fn parse_template_config_enhanced(table: &toml::map::Map<String, toml::Value>) -> Result<(), Error> {
    let mut config = TemplateConfig::default();
    
    if let Some(dir) = table.get("template_dir").and_then(|v| v.as_str()) {
        config.template_dir = PathBuf::from(dir);
    }
    if let Some(cache) = table.get("cache_templates").and_then(|v| v.as_bool()) {
        config.cache_templates = cache;
    }
    if let Some(auto_reload) = table.get("auto_reload").and_then(|v| v.as_bool()) {
        config.auto_reload = auto_reload;
    }
    if let Some(ext) = table.get("template_extension").and_then(|v| v.as_str()) {
        if !ext.starts_with('.') {
            return Err(ConfigError::InvalidValue {
                field: "template.template_extension".to_string(),
                value: ext.to_string(),
                reason: "Template extension must start with '.'".to_string(),
            });
        }
        config.template_extension = ext.to_string();
    }
    if let Some(filters) = table.get("custom_filters").and_then(|v| v.as_table()) {
        for (key, value) in filters {
            if let Some(filter_value) = value.as_str() {
                config.custom_filters.insert(key.clone(), filter_value.to_string());
            }
        }
    }
    
    Ok(config)
}

fn parse_static_file_config_enhanced(table: &toml::map::Map<String, toml::Value>) -> Result<(), Error> {
    let mut config = StaticFileConfig::default();
    
    if let Some(dir) = table.get("static_dir").and_then(|v| v.as_str()) {
        config.static_dir = PathBuf::from(dir);
    }
    if let Some(enable) = table.get("enable_caching").and_then(|v| v.as_bool()) {
        config.enable_caching = enable;
    }
    if let Some(max_age) = table.get("cache_max_age").and_then(|v| v.as_integer()) {
        config.cache_max_age = Duration::from_secs(max_age as u64);
    }
    if let Some(enable) = table.get("enable_compression").and_then(|v| v.as_bool()) {
        config.enable_compression = enable;
    }
    if let Some(enable) = table.get("enable_etag").and_then(|v| v.as_bool()) {
        config.enable_etag = enable;
    }
    if let Some(enable) = table.get("enable_last_modified").and_then(|v| v.as_bool()) {
        config.enable_last_modified = enable;
    }
    if let Some(extensions) = table.get("allowed_extensions").and_then(|v| v.as_array()) {
        let exts: Vec<String> = extensions.iter()
            .filter_map(|v| v.as_str())
            .map(|s| {
                if s.starts_with('.') {
                    s.to_string()
                } else {
                    format!(".{}", s)
                }
            })
            .collect();
        config.allowed_extensions = exts;
    }
    
    Ok(config)
}

fn parse_logging_config_enhanced(table: &toml::map::Map<String, toml::Value>) -> Result<(), Error> {
    let mut config = LoggingConfig::default();
    
    if let Some(enable) = table.get("enable_request_logging").and_then(|v| v.as_bool()) {
        config.enable_request_logging = enable;
    }
    if let Some(enable) = table.get("enable_response_logging").and_then(|v| v.as_bool()) {
        config.enable_response_logging = enable;
    }
    if let Some(enable) = table.get("enable_error_logging").and_then(|v| v.as_bool()) {
        config.enable_error_logging = enable;
    }
    if let Some(level) = table.get("log_level").and_then(|v| v.as_str()) {
        config.log_level = match level {
            "Error" => LogLevel::Error,
            "Warn" => LogLevel::Warn,
            "Info" => LogLevel::Info,
            "Debug" => LogLevel::Debug,
            "Trace" => LogLevel::Trace,
            _ => return Err(ConfigError::InvalidValue {
                field: "logging.log_level".to_string(),
                value: level.to_string(),
                reason: "Log level must be 'Error', 'Warn', 'Info', 'Debug', or 'Trace'".to_string(),
            }),
        };
    }
    if let Some(format) = table.get("log_format").and_then(|v| v.as_str()) {
        config.log_format = match format {
            "Common" => LogFormat::Common,
            "Combined" => LogFormat::Combined,
            "Json" => LogFormat::Json,
            s if s.starts_with("Custom(") => {
                let custom = s.trim_start_matches("Custom(").trim_end_matches(")");
                LogFormat::Custom(custom.to_string())
            }
            _ => return Err(ConfigError::InvalidValue {
                field: "logging.log_format".to_string(),
                value: format.to_string(),
                reason: "Log format must be 'Common', 'Combined', 'Json', or 'Custom(...)'".to_string(),
            }),
        };
    }
    if let Some(path) = table.get("access_log_path").and_then(|v| v.as_str()) {
        config.access_log_path = Some(PathBuf::from(path));
    }
    if let Some(path) = table.get("error_log_path").and_then(|v| v.as_str()) {
        config.error_log_path = Some(PathBuf::from(path));
    }
    
    Ok(config)
}

fn parse_development_config_enhanced(table: &toml::map::Map<String, toml::Value>) -> Result<(), Error> {
    let mut config = DevelopmentConfig::default();
    
    if let Some(enable) = table.get("enable_hot_reload").and_then(|v| v.as_bool()) {
        config.enable_hot_reload = enable;
    }
    if let Some(enable) = table.get("enable_debug_mode").and_then(|v| v.as_bool()) {
        config.enable_debug_mode = enable;
    }
    if let Some(enable) = table.get("enable_metrics").and_then(|v| v.as_bool()) {
        config.enable_metrics = enable;
    }
    if let Some(endpoint) = table.get("metrics_endpoint").and_then(|v| v.as_str()) {
        if !endpoint.starts_with('/') {
            return Err(ConfigError::InvalidValue {
                field: "development.metrics_endpoint".to_string(),
                value: endpoint.to_string(),
                reason: "Endpoint must start with '/'".to_string(),
            });
        }
        config.metrics_endpoint = endpoint.to_string();
    }
    if let Some(endpoint) = table.get("health_check_endpoint").and_then(|v| v.as_str()) {
        if !endpoint.starts_with('/') {
            return Err(ConfigError::InvalidValue {
                field: "development.health_check_endpoint".to_string(),
                value: endpoint.to_string(),
                reason: "Endpoint must start with '/'".to_string(),
            });
        }
        config.health_check_endpoint = endpoint.to_string();
    }
    if let Some(endpoint) = table.get("debug_endpoint").and_then(|v| v.as_str()) {
        if !endpoint.starts_with('/') {
            return Err(ConfigError::InvalidValue {
                field: "development.debug_endpoint".to_string(),
                value: endpoint.to_string(),
                reason: "Endpoint must start with '/'".to_string(),
            });
        }
        config.debug_endpoint = Some(endpoint.to_string());
    }
    
    Ok(config)
}
