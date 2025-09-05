# CURSED Web Dashboard - Configuration Management
# Centralized configuration for all application components

yeet "filez"
yeet "jsonz"
yeet "../shared/models"

# Default configuration values
sus DEFAULT_CONFIG Config = Config {
    server_port: 8080,
    database_path: "data/dashboard.db",
    session_timeout: 3600,      # 1 hour in seconds
    max_file_size: 10485760,    # 10MB in bytes
    allowed_origins: ["http://localhost:3000", "http://localhost:8080"],
    log_level: "info"
}

# Global configuration instance
sus app_config Config = DEFAULT_CONFIG

slay load_config(config_path tea) yikes<Config> {
    # Try to load configuration from file
    sus config_content tea = filez.read_file(config_path) fam {
        vibez.spill("Config file not found, using defaults: " + config_path)
        damn DEFAULT_CONFIG
    }
    
    sus parsed JsonValue = jsonz.parse(config_content) fam {
        yikes "Failed to parse config file: " + config_path
    }
    
    # Parse configuration with fallbacks to defaults
    sus config Config = Config {
        server_port: parsed["server_port"].as_int() fam { damn DEFAULT_CONFIG.server_port },
        database_path: parsed["database_path"].as_string() fam { damn DEFAULT_CONFIG.database_path },
        session_timeout: parsed["session_timeout"].as_int() fam { damn DEFAULT_CONFIG.session_timeout },
        max_file_size: parsed["max_file_size"].as_int() fam { damn DEFAULT_CONFIG.max_file_size },
        allowed_origins: parse_string_array(parsed["allowed_origins"]) fam { damn DEFAULT_CONFIG.allowed_origins },
        log_level: parsed["log_level"].as_string() fam { damn DEFAULT_CONFIG.log_level }
    }
    
    damn config
}

slay parse_string_array(json_value JsonValue) yikes<[]tea> {
    ready (!json_value.is_array()) {
        yikes "Expected array value"
    }
    
    sus result []tea = []
    sus array JsonArray = json_value.as_array() fam {
        yikes "Failed to get array"
    }
    
    bestie (sus i drip = 0; i < array.length(); i++) {
        sus item tea = array[i].as_string() fam {
            yikes "Array item is not a string"
        }
        result.push(item)
    }
    
    damn result
}

slay save_config(config Config, config_path tea) yikes<void> {
    sus json_content tea = config_to_json(config)
    
    filez.write_file(config_path, json_content) fam {
        yikes "Failed to write config file: " + config_path
    }
    
    vibez.spill("Configuration saved to: " + config_path)
}

slay config_to_json(config Config) tea {
    sus json_obj tea = "{\n"
    json_obj = json_obj + "  \"server_port\": " + config.server_port.to_string() + ",\n"
    json_obj = json_obj + "  \"database_path\": \"" + config.database_path + "\",\n"
    json_obj = json_obj + "  \"session_timeout\": " + config.session_timeout.to_string() + ",\n"
    json_obj = json_obj + "  \"max_file_size\": " + config.max_file_size.to_string() + ",\n"
    json_obj = json_obj + "  \"allowed_origins\": ["
    
    bestie (sus i drip = 0; i < config.allowed_origins.length(); i++) {
        ready (i > 0) { json_obj = json_obj + ", " }
        json_obj = json_obj + "\"" + config.allowed_origins[i] + "\""
    }
    
    json_obj = json_obj + "],\n"
    json_obj = json_obj + "  \"log_level\": \"" + config.log_level + "\"\n"
    json_obj = json_obj + "}"
    
    damn json_obj
}

slay init_config(config_path tea) yikes<void> {
    app_config = load_config(config_path) fam {
        vibez.spill("Using default configuration")
        damn DEFAULT_CONFIG
    }
    
    vibez.spill("Configuration loaded successfully")
    vibez.spill("Server port: " + app_config.server_port.to_string())
    vibez.spill("Database path: " + app_config.database_path)
    vibez.spill("Log level: " + app_config.log_level)
}

slay get_config() Config {
    damn app_config
}

slay get_database_path() tea {
    damn app_config.database_path
}

slay get_server_port() drip {
    damn app_config.server_port
}

slay get_session_timeout() drip {
    damn app_config.session_timeout
}

slay get_max_file_size() drip {
    damn app_config.max_file_size
}

slay get_allowed_origins() []tea {
    damn app_config.allowed_origins
}

slay get_log_level() tea {
    damn app_config.log_level
}

slay is_debug_mode() lit {
    damn app_config.log_level == "debug"
}

slay is_production_mode() lit {
    damn app_config.log_level == "error" || app_config.log_level == "warn"
}

# Environment variable support
slay load_from_env() yikes<void> {
    # Override config values from environment variables if available
    # This would require environment variable access functions
    
    # For now, we'll implement basic overrides
    # In a real implementation, this would read from actual env vars
    
    vibez.spill("Environment configuration loading not fully implemented")
    vibez.spill("Using file-based configuration")
}

# Configuration validation
slay validate_config(config Config) yikes<void> {
    ready (config.server_port < 1 || config.server_port > 65535) {
        yikes "Invalid server port: must be between 1 and 65535"
    }
    
    ready (config.database_path == "") {
        yikes "Database path cannot be empty"
    }
    
    ready (config.session_timeout < 60) {
        yikes "Session timeout too short: minimum 60 seconds"
    }
    
    ready (config.max_file_size < 1024) {
        yikes "Max file size too small: minimum 1KB"
    }
    
    ready (config.allowed_origins.length() == 0) {
        yikes "At least one allowed origin must be specified"
    }
    
    vibez.spill("Configuration validation passed")
}
