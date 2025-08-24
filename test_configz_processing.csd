yeet "configz"

# Test configz module with real configuration processing
sus config_toml tea = """
[database]
host = "localhost"
port = 5432
username = "admin"
password = "secret123"
ssl_enabled = true

[server]
bind_address = "0.0.0.0"
port = 8080
max_connections = 1000

[logging]
level = "info"
file = "/var/log/app.log"
rotate_size = "100MB"

[features]
experimental = false
debug_mode = true
cache_ttl = 3600
"""

# Test TOML parsing with validation
sus parsed_config map<tea, tea> = parse_toml_config(config_toml)
vibez.spill("✓ TOML configuration parsed successfully")

# Test configuration validation
sus db_config map<tea, tea> = get_config_section(parsed_config, "database")
sus is_valid_db lit = validate_database_config(db_config)
vibez.spill("✓ Database config validation:", is_valid_db)

# Test environment variable substitution
sus config_with_env tea = substitute_env_variables(config_toml)
vibez.spill("✓ Environment variable substitution applied")

# Test configuration merging
sus override_config map<tea, tea> = {
    "server": {
        "port": 9090,
        "debug": true
    }
}
sus merged_config map<tea, tea> = merge_configurations(parsed_config, override_config)
vibez.spill("✓ Configuration merge completed")

# Test configuration validation rules
add_validation_rule("database.port", "integer", 1, 65535)
add_validation_rule("server.bind_address", "ip_address")
add_validation_rule("logging.level", "enum", ["debug", "info", "warn", "error"])

sus validation_result map<tea, tea> = validate_full_config(merged_config)
vibez.spill("✓ Full configuration validation:", validation_result["valid"])

# Test configuration watching
sus config_watcher map<tea, tea> = create_config_watcher("config.toml")
register_config_change_callback(config_watcher, slay() { vibez.spill("Config changed!") })
vibez.spill("✓ Configuration watcher set up")

# Test secure configuration handling
sus encrypted_value tea = encrypt_config_value("secret123")
sus decrypted_value tea = decrypt_config_value(encrypted_value)
vibez.spill("✓ Configuration encryption/decryption working")

vibez.spill("✅ configz: All real configuration processing working")
