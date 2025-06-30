// Application configuration management
export AppConfig, DatabaseConfig, load_settings, save_settings, ConfigError

// Main application configuration
struct AppConfig {
    app_name: string,
    version: string,
    debug_mode: bool,
    database: DatabaseConfig,
    network: NetworkConfig,
    features: FeatureFlags
}

struct DatabaseConfig {
    host: string,
    port: int,
    database_name: string,
    username: string,
    password: string,
    max_connections: int,
    timeout: int
}

struct NetworkConfig {
    api_base_url: string,
    timeout_seconds: int,
    retry_attempts: int,
    rate_limit: int,
    enable_ssl: bool
}

struct FeatureFlags {
    enable_logging: bool,
    enable_caching: bool,
    enable_analytics: bool,
    beta_features: bool
}

enum ConfigError {
    FileNotFound(string),
    ParseError(string),
    ValidationError(string),
    WriteError(string)
}

impl ConfigError {
    func to_string(&self) -> string {
        match self {
            ConfigError::FileNotFound(path) => "Configuration file not found: " + path,
            ConfigError::ParseError(msg) => "Failed to parse configuration: " + msg,
            ConfigError::ValidationError(msg) => "Configuration validation failed: " + msg,
            ConfigError::WriteError(msg) => "Failed to write configuration: " + msg
        }
    }
}

impl AppConfig {
    func default() -> AppConfig {
        return AppConfig {
            app_name: "CURSED Complex Project",
            version: "1.0.0",
            debug_mode: true,
            database: DatabaseConfig::default(),
            network: NetworkConfig::default(),
            features: FeatureFlags::default()
        }
    }
    
    func validate(&self) -> Result<(), ConfigError> {
        if self.app_name.is_empty() {
            return Err(ConfigError::ValidationError("App name cannot be empty"))
        }
        
        if self.database.port < 1 || self.database.port > 65535 {
            return Err(ConfigError::ValidationError("Invalid database port"))
        }
        
        if self.network.timeout_seconds < 1 {
            return Err(ConfigError::ValidationError("Network timeout must be positive"))
        }
        
        return Ok(())
    }
    
    func to_json(&self) -> string {
        let mut json = "{"
        json += "\"app_name\":\"" + self.app_name + "\","
        json += "\"version\":\"" + self.version + "\","
        json += "\"debug_mode\":" + self.debug_mode.to_string() + ","
        json += "\"database\":" + self.database.to_json() + ","
        json += "\"network\":" + self.network.to_json() + ","
        json += "\"features\":" + self.features.to_json()
        json += "}"
        return json
    }
}

impl DatabaseConfig {
    func default() -> DatabaseConfig {
        return DatabaseConfig {
            host: "localhost",
            port: 5432,
            database_name: "cursed_app",
            username: "cursed_user",
            password: "secure_password",
            max_connections: 20,
            timeout: 30
        }
    }
    
    func to_json(&self) -> string {
        let mut json = "{"
        json += "\"host\":\"" + self.host + "\","
        json += "\"port\":" + self.port.to_string() + ","
        json += "\"database_name\":\"" + self.database_name + "\","
        json += "\"username\":\"" + self.username + "\","
        json += "\"max_connections\":" + self.max_connections.to_string() + ","
        json += "\"timeout\":" + self.timeout.to_string()
        json += "}"
        return json
    }
}

impl NetworkConfig {
    func default() -> NetworkConfig {
        return NetworkConfig {
            api_base_url: "https://api.cursed-project.com",
            timeout_seconds: 30,
            retry_attempts: 3,
            rate_limit: 1000,
            enable_ssl: true
        }
    }
    
    func to_json(&self) -> string {
        let mut json = "{"
        json += "\"api_base_url\":\"" + self.api_base_url + "\","
        json += "\"timeout_seconds\":" + self.timeout_seconds.to_string() + ","
        json += "\"retry_attempts\":" + self.retry_attempts.to_string() + ","
        json += "\"rate_limit\":" + self.rate_limit.to_string() + ","
        json += "\"enable_ssl\":" + self.enable_ssl.to_string()
        json += "}"
        return json
    }
}

impl FeatureFlags {
    func default() -> FeatureFlags {
        return FeatureFlags {
            enable_logging: true,
            enable_caching: true,
            enable_analytics: false,
            beta_features: false
        }
    }
    
    func to_json(&self) -> string {
        let mut json = "{"
        json += "\"enable_logging\":" + self.enable_logging.to_string() + ","
        json += "\"enable_caching\":" + self.enable_caching.to_string() + ","
        json += "\"enable_analytics\":" + self.enable_analytics.to_string() + ","
        json += "\"beta_features\":" + self.beta_features.to_string()
        json += "}"
        return json
    }
}

// Public API functions
func load_settings() -> Result<AppConfig, ConfigError> {
    let config_path = "config.json"
    
    // Try to load from file, fall back to defaults
    match load_from_file(config_path) {
        Ok(config) => {
            match config.validate() {
                Ok(()) => Ok(config),
                Err(e) => Err(e)
            }
        },
        Err(ConfigError::FileNotFound(_)) => {
            println("Config file not found, using defaults")
            let default_config = AppConfig::default()
            match save_settings(&default_config) {
                Ok(()) => Ok(default_config),
                Err(e) => {
                    println("Warning: Could not save default config: " + e.to_string())
                    Ok(default_config)
                }
            }
        },
        Err(e) => Err(e)
    }
}

func save_settings(config: &AppConfig) -> Result<(), ConfigError> {
    match config.validate() {
        Ok(()) => {},
        Err(e) => return Err(e)
    }
    
    let config_json = config.to_json()
    let config_path = "config.json"
    
    match write_to_file(config_path, config_json) {
        Ok(()) => {
            println("Configuration saved to " + config_path)
            Ok(())
        },
        Err(e) => Err(ConfigError::WriteError(e))
    }
}

// Helper functions
func load_from_file(path: string) -> Result<AppConfig, ConfigError> {
    // Mock file loading - in real implementation would read actual file
    match read_file_content(path) {
        Ok(content) => parse_config_json(content),
        Err(e) => Err(ConfigError::FileNotFound(path))
    }
}

func parse_config_json(json: string) -> Result<AppConfig, ConfigError> {
    // Mock JSON parsing - in real implementation would use proper JSON parser
    if json.is_empty() {
        return Err(ConfigError::ParseError("Empty configuration file"))
    }
    
    // Return default config for testing
    Ok(AppConfig::default())
}

func read_file_content(path: string) -> Result<string, string> {
    // Mock file reading
    if path == "config.json" {
        return Ok("{}")  // Return empty JSON to trigger default config
    }
    Err("File not found")
}

func write_to_file(path: string, content: string) -> Result<(), string> {
    // Mock file writing
    println("Writing to file " + path + ": " + content.len().to_string() + " bytes")
    Ok(())
}
