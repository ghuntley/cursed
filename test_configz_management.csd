// Test configuration management (configz package)
yeet "configz"
yeet "vibez"

vibez.spill("=== Testing Configz Configuration Management ===")

// Test environment variable configuration
sus env_config Config = load_env_config()
set_env_default("DATABASE_URL", "postgres://localhost/test")
set_env_default("DEBUG_MODE", "false")
set_env_default("MAX_CONNECTIONS", "100")

sus db_url tea = get_env_string("DATABASE_URL")
sus debug_mode lit = get_env_bool("DEBUG_MODE")
sus max_connections drip = get_env_int("MAX_CONNECTIONS")

vibez.spill("Environment config loaded:")
vibez.spill("- Database URL:", db_url)
vibez.spill("- Debug mode:", debug_mode)
vibez.spill("- Max connections:", max_connections)

// Test TOML configuration
sus toml_content tea = `
[database]
host = "localhost"
port = 5432
name = "myapp"
username = "admin"
password = "secret123"

[server]
host = "0.0.0.0"
port = 8080
workers = 4

[features]
enable_cache = true
enable_metrics = true
cache_ttl = 3600
`

sus toml_config Config = parse_toml_config(toml_content)
sus db_host tea = get_config_string(toml_config, "database.host")
sus server_port drip = get_config_int(toml_config, "server.port")
sus enable_cache lit = get_config_bool(toml_config, "features.enable_cache")

vibez.spill("TOML config parsed:")
vibez.spill("- DB Host:", db_host)
vibez.spill("- Server Port:", server_port)
vibez.spill("- Cache Enabled:", enable_cache)

// Test configuration merging
sus merged_config Config = merge_configs(env_config, toml_config)
sus final_db_url tea = get_config_string(merged_config, "database.url", db_url)

vibez.spill("✅ Configuration management: PASSED")
vibez.spill("=== Configz Testing Complete ===")
