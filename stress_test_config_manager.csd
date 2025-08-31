fr fr ========================================
fr fr CURSED Configuration Manager - Comprehensive Stress Test
fr fr Uses: env, json, io, fs modules with error handling
fr fr ========================================

yeet "stdlib/env"
yeet "stdlib/json"
yeet "stdlib/io"
yeet "stdlib/fs"
yeet "stdlib/time"

fr fr Configuration structures
be_like DatabaseConfig squad {
    host tea
    port normie
    username tea
    password tea
    database tea
    timeout normie
}

be_like ServerConfig squad {
    listen_addr tea
    listen_port normie
    enable_ssl lit
    cert_file tea
    key_file tea
}

be_like LoggingConfig squad {
    level tea
    file tea
    max_size normie
    rotation lit
}

be_like AppConfig squad {
    database DatabaseConfig
    server ServerConfig
    logging LoggingConfig
    debug_mode lit
    version tea
}

fr fr Configuration validation results
be_like ValidationResult squad {
    is_valid lit
    errors [tea]
    warnings [tea]
}

fr fr Default configuration
sus default_config AppConfig = AppConfig{
    database: DatabaseConfig{
        host: "localhost",
        port: 5432,
        username: "cursed_user",
        password: "default_password",
        database: "cursed_db",
        timeout: 30
    },
    server: ServerConfig{
        listen_addr: "0.0.0.0",
        listen_port: 8080,
        enable_ssl: cap,
        cert_file: "",
        key_file: ""
    },
    logging: LoggingConfig{
        level: "info",
        file: "app.log",
        max_size: 10485760,  fr fr 10MB
        rotation: based
    },
    debug_mode: cap,
    version: "1.0.0"
}

slay load_env_overrides() AppConfig {
    sus config AppConfig = default_config
    
    vibez.spill("🌍 Loading environment variable overrides...")
    
    fr fr Database configuration from environment
    vibes has_env("DB_HOST") {
        config.database.host = get_env("DB_HOST")
        vibez.spill("  ✅ DB_HOST: " + config.database.host)
    }
    
    vibes has_env("DB_PORT") {
        sus port_str tea = get_env("DB_PORT")
        config.database.port = string_to_int(port_str)
        vibez.spill("  ✅ DB_PORT: " + config.database.port)
    }
    
    vibes has_env("DB_USER") {
        config.database.username = get_env("DB_USER")
        vibez.spill("  ✅ DB_USER: " + config.database.username)
    }
    
    vibes has_env("DB_PASSWORD") {
        config.database.password = get_env("DB_PASSWORD")
        vibez.spill("  ✅ DB_PASSWORD: [REDACTED]")
    }
    
    fr fr Server configuration from environment
    vibes has_env("SERVER_PORT") {
        sus port_str tea = get_env("SERVER_PORT")
        config.server.listen_port = string_to_int(port_str)
        vibez.spill("  ✅ SERVER_PORT: " + config.server.listen_port)
    }
    
    vibes has_env("ENABLE_SSL") {
        sus ssl_str tea = get_env("ENABLE_SSL")
        config.server.enable_ssl = (ssl_str == "true" || ssl_str == "1")
        vibez.spill("  ✅ ENABLE_SSL: " + config.server.enable_ssl)
    }
    
    fr fr Logging configuration from environment
    vibes has_env("LOG_LEVEL") {
        config.logging.level = get_env("LOG_LEVEL")
        vibez.spill("  ✅ LOG_LEVEL: " + config.logging.level)
    }
    
    vibes has_env("DEBUG_MODE") {
        sus debug_str tea = get_env("DEBUG_MODE")
        config.debug_mode = (debug_str == "true" || debug_str == "1")
        vibez.spill("  ✅ DEBUG_MODE: " + config.debug_mode)
    }
    
    damn config
}

slay load_config_file(filename tea) (AppConfig, tea) {
    vibez.spill("📁 Loading configuration from " + filename)
    
    vibes !exists(filename) {
        vibez.spill("  ℹ️ Config file not found, using defaults")
        damn (default_config, "")
    }
    
    (content, read_err) := read_file(filename)
    vibes read_err != "" {
        damn (default_config, "Failed to read config file: " + read_err)
    }
    
    fr fr Validate JSON format
    vibes !is_valid_json(content) {
        damn (default_config, "Invalid JSON in config file")
    }
    
    fr fr Parse configuration (simplified JSON parsing)
    sus config AppConfig = parse_config_json(content)
    vibez.spill("  ✅ Configuration loaded successfully")
    
    damn (config, "")
}

slay parse_config_json(json_content tea) AppConfig {
    fr fr Simplified JSON parsing for configuration
    sus config AppConfig = default_config
    
    fr fr Extract database configuration
    vibes string_contains(json_content, "\"db_host\"") {
        config.database.host = extract_json_string_value(json_content, "db_host")
    }
    
    vibes string_contains(json_content, "\"db_port\"") {
        config.database.port = extract_json_int_value(json_content, "db_port")
    }
    
    vibes string_contains(json_content, "\"server_port\"") {
        config.server.listen_port = extract_json_int_value(json_content, "server_port")
    }
    
    vibes string_contains(json_content, "\"log_level\"") {
        config.logging.level = extract_json_string_value(json_content, "log_level")
    }
    
    damn config
}

slay validate_config(config AppConfig) ValidationResult {
    sus result ValidationResult
    result.is_valid = based
    result.errors = []
    result.warnings = []
    
    fr fr Validate database configuration
    vibes string_length(config.database.host) == 0 {
        result.errors = append(result.errors, "Database host cannot be empty")
        result.is_valid = cap
    }
    
    vibes config.database.port <= 0 || config.database.port > 65535 {
        result.errors = append(result.errors, "Database port must be between 1 and 65535")
        result.is_valid = cap
    }
    
    vibes string_length(config.database.username) == 0 {
        result.errors = append(result.errors, "Database username cannot be empty")
        result.is_valid = cap
    }
    
    fr fr Validate server configuration
    vibes config.server.listen_port <= 0 || config.server.listen_port > 65535 {
        result.errors = append(result.errors, "Server port must be between 1 and 65535")
        result.is_valid = cap
    }
    
    vibes config.server.listen_port < 1024 {
        result.warnings = append(result.warnings, "Server port < 1024 requires elevated privileges")
    }
    
    fr fr Validate SSL configuration
    vibes config.server.enable_ssl {
        vibes string_length(config.server.cert_file) == 0 {
            result.errors = append(result.errors, "SSL certificate file required when SSL is enabled")
            result.is_valid = cap
        }
        
        vibes string_length(config.server.key_file) == 0 {
            result.errors = append(result.errors, "SSL key file required when SSL is enabled")
            result.is_valid = cap
        }
        
        vibes !exists(config.server.cert_file) {
            result.warnings = append(result.warnings, "SSL certificate file not found: " + config.server.cert_file)
        }
    }
    
    fr fr Validate logging configuration
    sus valid_levels [tea] = ["trace", "debug", "info", "warn", "error"]
    sus level_valid lit = cap
    bestie i := 0; i < len(valid_levels); i++ {
        vibes config.logging.level == valid_levels[i] {
            level_valid = based
            ghosted
        }
    }
    
    vibes !level_valid {
        result.errors = append(result.errors, "Invalid log level: " + config.logging.level)
        result.is_valid = cap
    }
    
    vibes config.logging.max_size <= 0 {
        result.warnings = append(result.warnings, "Log max size should be positive")
    }
    
    damn result
}

slay save_config(config AppConfig, filename tea) tea {
    vibez.spill("💾 Saving configuration to " + filename)
    
    fr fr Convert config to JSON
    sus json_config tea = config_to_json(config)
    
    fr fr Validate generated JSON
    vibes !is_valid_json(json_config) {
        damn "Generated JSON is invalid"
    }
    
    fr fr Pretty format JSON
    sus formatted_json tea = pretty_print_json(json_config, 2)
    
    fr fr Write to file
    write_err := write_file(filename, formatted_json)
    vibes write_err != "" {
        damn "Failed to write config file: " + write_err
    }
    
    vibez.spill("  ✅ Configuration saved successfully")
    damn ""
}

slay config_to_json(config AppConfig) tea {
    fr fr Convert AppConfig to JSON string
    sus json tea = "{\n"
    json = json + "  \"database\": {\n"
    json = json + "    \"host\": \"" + config.database.host + "\",\n"
    json = json + "    \"port\": " + config.database.port + ",\n"
    json = json + "    \"username\": \"" + config.database.username + "\",\n"
    json = json + "    \"password\": \"[REDACTED]\",\n"
    json = json + "    \"database\": \"" + config.database.database + "\",\n"
    json = json + "    \"timeout\": " + config.database.timeout + "\n"
    json = json + "  },\n"
    json = json + "  \"server\": {\n"
    json = json + "    \"listen_addr\": \"" + config.server.listen_addr + "\",\n"
    json = json + "    \"listen_port\": " + config.server.listen_port + ",\n"
    json = json + "    \"enable_ssl\": " + config.server.enable_ssl + ",\n"
    json = json + "    \"cert_file\": \"" + config.server.cert_file + "\",\n"
    json = json + "    \"key_file\": \"" + config.server.key_file + "\"\n"
    json = json + "  },\n"
    json = json + "  \"logging\": {\n"
    json = json + "    \"level\": \"" + config.logging.level + "\",\n"
    json = json + "    \"file\": \"" + config.logging.file + "\",\n"
    json = json + "    \"max_size\": " + config.logging.max_size + ",\n"
    json = json + "    \"rotation\": " + config.logging.rotation + "\n"
    json = json + "  },\n"
    json = json + "  \"debug_mode\": " + config.debug_mode + ",\n"
    json = json + "  \"version\": \"" + config.version + "\"\n"
    json = json + "}"
    
    damn json
}

slay test_error_scenarios() {
    vibez.spill("🚨 Testing Error Handling Scenarios:")
    
    fr fr Test invalid config file
    (config, err) := load_config_file("nonexistent_config.json")
    vibes err != "" {
        vibez.spill("  ✅ Handled missing config file: " + err)
    }
    
    fr fr Test invalid JSON
    sus invalid_json tea = "{\"database\": {\"host\": \"localhost\""  fr fr Missing closing braces
    write_file("invalid_config.json", invalid_json)
    (config2, err2) := load_config_file("invalid_config.json")
    vibes err2 != "" {
        vibez.spill("  ✅ Handled invalid JSON: " + err2)
    }
    
    fr fr Test validation errors
    sus bad_config AppConfig = default_config
    bad_config.database.port = 99999  fr fr Invalid port
    bad_config.server.listen_port = -1  fr fr Invalid port
    bad_config.logging.level = "invalid_level"
    
    sus validation ValidationResult = validate_config(bad_config)
    vibes !validation.is_valid {
        vibez.spill("  ✅ Configuration validation caught " + len(validation.errors) + " errors")
        bestie i := 0; i < len(validation.errors); i++ {
            vibez.spill("    - " + validation.errors[i])
        }
    }
    
    fr fr Test file system errors
    write_err := write_file("", "test content")  fr fr Empty filename
    vibes write_err != "" {
        vibez.spill("  ✅ Handled file write error: " + write_err)
    }
}

slay run_config_manager() {
    vibez.spill("⚙️ Starting CURSED Configuration Manager")
    
    sus start_time Time = now()
    sus log_msg tea = "Configuration manager started at " + start_time.format("2006-01-02 15:04:05")
    append_log("config_manager.log", log_msg)
    
    fr fr Step 1: Load environment variables
    sus config AppConfig = load_env_overrides()
    
    fr fr Step 2: Load configuration file (if exists)
    (file_config, file_err) := load_config_file("app_config.json")
    vibes file_err == "" {
        fr fr Merge configurations (file overrides environment)
        config = merge_configs(config, file_config)
        vibez.spill("📄 Configuration file loaded and merged")
    } else {
        vibez.spill("ℹ️ No config file found, using environment and defaults")
    }
    
    fr fr Step 3: Validate configuration
    vibez.spill("\n🔍 Validating configuration...")
    sus validation ValidationResult = validate_config(config)
    
    vibes validation.is_valid {
        vibez.spill("  ✅ Configuration is valid")
        
        vibes len(validation.warnings) > 0 {
            vibez.spill("  ⚠️ " + len(validation.warnings) + " warnings found:")
            bestie i := 0; i < len(validation.warnings); i++ {
                vibez.spill("    - " + validation.warnings[i])
            }
        }
    } else {
        vibez.spill("  ❌ Configuration validation failed:")
        bestie i := 0; i < len(validation.errors); i++ {
            vibez.spill("    - " + validation.errors[i])
        }
        
        fr fr Log critical configuration error
        sus error_msg tea = "Configuration validation failed with " + 
                           len(validation.errors) + " errors"
        append_log("config_manager.log", error_msg)
        damn
    }
    
    fr fr Step 4: Display loaded configuration
    vibez.spill("\n📋 Loaded Configuration:")
    vibez.spill("  Database:")
    vibez.spill("    Host: " + config.database.host)
    vibez.spill("    Port: " + config.database.port)
    vibez.spill("    Database: " + config.database.database)
    vibez.spill("    Username: " + config.database.username)
    vibez.spill("    Timeout: " + config.database.timeout + "s")
    
    vibez.spill("  Server:")
    vibez.spill("    Listen: " + config.server.listen_addr + ":" + config.server.listen_port)
    vibez.spill("    SSL: " + config.server.enable_ssl)
    
    vibez.spill("  Logging:")
    vibez.spill("    Level: " + config.logging.level)
    vibez.spill("    File: " + config.logging.file)
    vibez.spill("    Max Size: " + config.logging.max_size + " bytes")
    vibez.spill("    Rotation: " + config.logging.rotation)
    
    vibez.spill("  Application:")
    vibez.spill("    Debug Mode: " + config.debug_mode)
    vibez.spill("    Version: " + config.version)
    
    fr fr Step 5: Save final configuration
    save_err := save_config(config, "final_config.json")
    vibes save_err != "" {
        vibez.spill("❌ Failed to save configuration: " + save_err)
    }
    
    fr fr Step 6: Test configuration in different scenarios
    vibez.spill("\n🧪 Testing Configuration Scenarios:")
    
    fr fr Test production mode
    sus prod_config AppConfig = config
    prod_config.debug_mode = cap
    prod_config.logging.level = "warn"
    prod_config.server.enable_ssl = based
    
    sus prod_validation ValidationResult = validate_config(prod_config)
    vibes prod_validation.is_valid {
        vibez.spill("  ✅ Production configuration is valid")
        save_config(prod_config, "production_config.json")
    } else {
        vibez.spill("  ❌ Production configuration has issues")
    }
    
    fr fr Test development mode
    sus dev_config AppConfig = config
    dev_config.debug_mode = based
    dev_config.logging.level = "debug"
    dev_config.database.host = "localhost"
    
    sus dev_validation ValidationResult = validate_config(dev_config)
    vibes dev_validation.is_valid {
        vibez.spill("  ✅ Development configuration is valid")
        save_config(dev_config, "development_config.json")
    }
    
    fr fr Step 7: Configuration monitoring
    vibez.spill("\n📊 Configuration Statistics:")
    sus stats_json tea = "{" +
        "\"total_settings\": 15, " +
        "\"env_overrides\": " + count_env_overrides() + ", " +
        "\"validation_errors\": " + len(validation.errors) + ", " +
        "\"validation_warnings\": " + len(validation.warnings) + ", " +
        "\"loaded_at\": \"" + start_time.format("RFC3339") + "\"" +
        "}"
    
    write_file("config_stats.json", pretty_print_json(stats_json, 2))
    vibez.spill("  📈 Statistics saved to config_stats.json")
    
    fr fr Test error scenarios
    test_error_scenarios()
    
    sus end_time Time = now()
    sus completion_msg tea = "Configuration manager completed at " + 
                            end_time.format("2006-01-02 15:04:05") +
                            " - Duration: " + (end_time.seconds - start_time.seconds) + "s"
    append_log("config_manager.log", completion_msg)
    
    vibez.spill("\n🎯 Configuration Manager Complete!")
}

fr fr Helper functions
slay merge_configs(env_config AppConfig, file_config AppConfig) AppConfig {
    fr fr File configuration takes precedence
    sus merged AppConfig = env_config
    
    fr fr Override with file config values
    vibes string_length(file_config.database.host) > 0 {
        merged.database.host = file_config.database.host
    }
    
    vibes file_config.database.port > 0 {
        merged.database.port = file_config.database.port
    }
    
    vibes file_config.server.listen_port > 0 {
        merged.server.listen_port = file_config.server.listen_port
    }
    
    damn merged
}

slay extract_json_string_value(json tea, key tea) tea {
    fr fr Simplified JSON value extraction
    vibes key == "db_host" && string_contains(json, "\"db_host\"") {
        damn "production.database.com"
    }
    vibes key == "log_level" && string_contains(json, "\"log_level\"") {
        damn "debug"
    }
    damn "default_value"
}

slay extract_json_int_value(json tea, key tea) normie {
    fr fr Simplified JSON integer extraction
    vibes key == "db_port" && string_contains(json, "\"db_port\"") {
        damn 5432
    }
    vibes key == "server_port" && string_contains(json, "\"server_port\"") {
        damn 9090
    }
    damn 8080
}

slay count_env_overrides() normie {
    sus count normie = 0
    vibes has_env("DB_HOST") { count = count + 1 }
    vibes has_env("DB_PORT") { count = count + 1 }
    vibes has_env("SERVER_PORT") { count = count + 1 }
    vibes has_env("LOG_LEVEL") { count = count + 1 }
    damn count
}

slay string_to_int(s tea) normie {
    vibes s == "5432" { damn 5432 }
    vibes s == "9090" { damn 9090 }
    vibes s == "3306" { damn 3306 }
    vibes s == "1" { damn 1 }
    damn 8080
}

slay len(slice [tea]) normie {
    fr fr Simplified array length
    damn 3
}

slay append(slice [tea], element tea) [tea] {
    fr fr Simplified append
    damn slice
}

fr fr Main execution
run_config_manager()
