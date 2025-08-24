# configz - Advanced Configuration Management

The `configz` module provides production-grade configuration management for CURSED applications with multi-format support, hot reloading, validation, and environment variable integration.

## Features

### 🔧 Multiple Configuration Formats
- **JSON**: Full JSON support with nested objects and arrays
- **YAML**: Hierarchical YAML configuration with proper indentation handling
- **TOML**: Modern TOML format with sections and typed values
- **INI**: Classic INI files with sections and key-value pairs
- **Environment Variables**: Automatic environment variable integration

### 🔥 Hot Reloading
- **File Watching**: Automatic detection of configuration file changes
- **Reload Callbacks**: Custom handlers triggered on configuration reload
- **Zero-Downtime**: Updates without application restart

### ✅ Validation & Defaults
- **Type Validation**: Ensure configuration values match expected types
- **Custom Validators**: URL validation, email validation, positive numbers, etc.
- **Required Fields**: Mark configuration values as required
- **Default Values**: Fallback values when configuration is missing

### 🌍 Environment Integration
- **Auto-Detection**: Automatic type detection for environment variables
- **Naming Conventions**: Support for multiple env var naming styles
- **Priority System**: Environment variables can override file configuration

### 🚀 Production Ready
- **Error Handling**: Graceful handling of missing files and invalid configuration
- **Debug Information**: Comprehensive debugging and introspection
- **Performance**: Optimized for high-performance applications
- **Memory Safe**: Arena allocator integration for leak-free operation

## Quick Start

```cursed
yeet "configz"

fr fr Create configuration manager
sus config ConfigManager = config_create()

fr fr Add configuration sources (priority: higher = takes precedence)
config = config_add_source(config, "file", "config.json", 10)
config = config_add_source(config, "env", "", 20)  fr fr Environment vars override files

fr fr Set default values
sus db_host_default ConfigValue = ConfigValue{}
db_host_default.type = "string"
db_host_default.string_value = "localhost"
config = config_set_default(config, "database.host", db_host_default)

sus db_port_default ConfigValue = ConfigValue{}
db_port_default.type = "number"
db_port_default.number_value = 5432.0
config = config_set_default(config, "database.port", db_port_default)

fr fr Add validation rules
config = config_add_validation(config, "database.host", "string", "required", "Database host is required")
config = config_add_validation(config, "database.port", "number", "positive_number", "Database port must be positive")
config = config_add_validation(config, "server.url", "string", "valid_url", "Server URL must be valid")

fr fr Load all configuration
config = config_load_all(config)

fr fr Access configuration values
sus db_host tea = config_get_string(config, "database.host", "localhost")
sus db_port normie = config_get_number(config, "database.port", 5432.0)
sus debug_mode lit = config_get_boolean(config, "app.debug", cringe)

vibez.spill("Database Host: " + db_host)
vibez.spill("Database Port: " + number_to_string(db_port))
vibez.spill("Debug Mode: " + (debug_mode ? "enabled" : "disabled"))
```

## Configuration File Examples

### JSON Configuration (config.json)
```json
{
  "database": {
    "host": "db.example.com",
    "port": 5432,
    "username": "app_user",
    "ssl": true
  },
  "server": {
    "port": 8080,
    "host": "0.0.0.0",
    "url": "https://api.example.com"
  },
  "features": {
    "cache_enabled": true,
    "rate_limit": 1000,
    "allowed_origins": ["https://example.com", "https://app.example.com"]
  }
}
```

### YAML Configuration (config.yaml)
```yaml
database:
  host: db.example.com
  port: 5432
  username: app_user
  ssl: true

server:
  port: 8080
  host: 0.0.0.0
  url: https://api.example.com

features:
  cache_enabled: true
  rate_limit: 1000
  allowed_origins:
    - https://example.com
    - https://app.example.com
```

### TOML Configuration (config.toml)
```toml
[database]
host = "db.example.com"
port = 5432
username = "app_user"
ssl = true

[server]
port = 8080
host = "0.0.0.0"
url = "https://api.example.com"

[features]
cache_enabled = true
rate_limit = 1000
allowed_origins = ["https://example.com", "https://app.example.com"]
```

### INI Configuration (config.ini)
```ini
[database]
host = db.example.com
port = 5432
username = app_user
ssl = yes

[server]
port = 8080
host = 0.0.0.0
url = https://api.example.com

[features]
cache_enabled = true
rate_limit = 1000
```

## Environment Variables

The configz module supports automatic environment variable integration with multiple naming conventions:

```bash
# Standard dot notation
export app.database.host=prod-db.example.com
export app.database.port=5432

# Underscore notation (converted to dots)
export APP_DATABASE_HOST=prod-db.example.com
export APP_DATABASE_PORT=5432

# Mixed notation
export APP_SERVER__PORT=8080  # Double underscore becomes single underscore
```

Environment variables automatically override file-based configuration when loaded with higher priority.

## Hot Reloading

Enable automatic configuration reloading when files change:

```cursed
fr fr Enable file watching
config = config_enable_watching(config)

fr fr Add reload callback
config = config_add_reload_callback(config, "database_reconnect", "reconnect_database")

fr fr In your main loop, check for changes
bestie (based) {
    config = config_check_for_changes(config)
    
    fr fr Your application logic here
    sleep(1000)  fr fr Check every second
}
```

## Validation Rules

Add comprehensive validation to ensure configuration correctness:

```cursed
fr fr Built-in validators
config = config_add_validation(config, "database.*", "string", "required", "Database fields are required")
config = config_add_validation(config, "*.port", "number", "positive_number", "Ports must be positive")
config = config_add_validation(config, "*.url", "string", "valid_url", "URLs must be valid")
config = config_add_validation(config, "*.email", "string", "valid_email", "Email addresses must be valid")

fr fr Pattern matching
config = config_add_validation(config, "api.*", "string", "required", "API configuration required")
config = config_add_validation(config, "cache.*", "", "", "")  # Any type allowed
```

## Advanced Usage

### Multiple Configuration Sources

```cursed
fr fr Load from multiple sources with priority
config = config_add_source(config, "file", "/etc/myapp/default.json", 5)    # System defaults
config = config_add_source(config, "file", "./config.json", 10)            # Application config  
config = config_add_source(config, "file", "./config.local.json", 15)      # Local overrides
config = config_add_source(config, "env", "", 20)                          # Environment variables (highest priority)

config = config_load_all(config)
```

### Working with Arrays

```cursed
fr fr Get array configuration
sus allowed_origins []ConfigValue = config_get_array(config, "features.allowed_origins")
sus origin_count drip = array_length(allowed_origins)

sus i drip = 0
bestie (i < origin_count) {
    sus origin ConfigValue = allowed_origins[i]
    ready (origin.type == "string") {
        vibez.spill("Allowed origin: " + origin.string_value)
    }
    i = i + 1
}
```

### Configuration Debugging

```cursed
fr fr Get all configuration keys
sus all_keys []tea = config_get_all_keys(config)
sus key_count drip = array_length(all_keys)

vibez.spill("All configuration keys:")
sus i drip = 0
bestie (i < key_count) {
    vibez.spill("  " + all_keys[i])
    i = i + 1
}

fr fr Get keys with specific prefix
sus db_keys []tea = config_get_keys_with_prefix(config, "database")
sus db_key_count drip = array_length(db_keys)

vibez.spill("Database configuration:")
sus j drip = 0
bestie (j < db_key_count) {
    sus key tea = db_keys[j]
    sus value tea = config_get_string(config, key, "")
    vibez.spill("  " + key + " = " + value)
    j = j + 1
}

fr fr Export configuration as JSON
sus json_config tea = config_export_json(config)
vibez.spill("Current configuration as JSON:")
vibez.spill(json_config)

fr fr Get detailed debug information
sus debug_info tea = config_debug_info(config)
vibez.spill(debug_info)
```

## Error Handling

The configz module provides robust error handling for production environments:

- **Missing Files**: Gracefully handles missing configuration files with warnings
- **Invalid JSON/YAML/TOML**: Reports parsing errors with file and line information
- **Type Mismatches**: Validates configuration types and reports validation errors
- **Required Fields**: Ensures required configuration values are present

## Performance Characteristics

- **Load Time**: Configuration loading is optimized for startup performance
- **Memory Usage**: Uses arena allocators to prevent memory leaks
- **File Watching**: Efficient file system monitoring with minimal overhead
- **Validation**: Fast pattern matching and type checking

## Integration Examples

### Web Server Configuration

```cursed
yeet "configz"
yeet "networkz"

sus config ConfigManager = config_create()
config = config_add_source(config, "file", "server.json", 10)
config = config_add_source(config, "env", "", 20)

fr fr Set server defaults
sus port_default ConfigValue = ConfigValue{}
port_default.type = "number"
port_default.number_value = 8080.0
config = config_set_default(config, "server.port", port_default)

config = config_load_all(config)

fr fr Start server with configuration
sus server_port normie = config_get_number(config, "server.port", 8080.0)
sus server_host tea = config_get_string(config, "server.host", "localhost")

vibez.spill("Starting server on " + server_host + ":" + number_to_string(server_port))
```

### Database Connection

```cursed
yeet "configz"
yeet "dbz"

sus config ConfigManager = config_create()
config = config_add_source(config, "file", "database.toml", 10)
config = config_add_validation(config, "database.host", "string", "required", "Database host required")
config = config_add_validation(config, "database.port", "number", "positive_number", "Database port must be positive")

config = config_load_all(config)

sus db_host tea = config_get_string(config, "database.host", "localhost")
sus db_port drip = drip(config_get_number(config, "database.port", 5432.0))
sus db_name tea = config_get_string(config, "database.name", "app")
sus db_ssl lit = config_get_boolean(config, "database.ssl", cringe)

fr fr Connect to database with configuration
sus connection DatabaseConnection = db_connect(db_host, db_port, db_name, db_ssl)
```

## API Reference

### Core Functions

- `config_create()` - Create new configuration manager
- `config_add_source(manager, type, path, priority)` - Add configuration source
- `config_set_default(manager, key, value)` - Set default value
- `config_load_all(manager)` - Load all configuration sources

### Value Access

- `config_get_string(manager, key, default)` - Get string value
- `config_get_number(manager, key, default)` - Get numeric value  
- `config_get_boolean(manager, key, default)` - Get boolean value
- `config_get_array(manager, key)` - Get array value
- `config_has_key(manager, key)` - Check if key exists

### Validation

- `config_add_validation(manager, pattern, type, validator, message)` - Add validation rule
- Built-in validators: `required`, `positive_number`, `valid_url`, `valid_email`

### Hot Reloading

- `config_enable_watching(manager)` - Enable file watching
- `config_check_for_changes(manager)` - Check for file changes
- `config_add_reload_callback(manager, name, handler)` - Add reload callback

### Debugging

- `config_get_all_keys(manager)` - Get all configuration keys
- `config_get_keys_with_prefix(manager, prefix)` - Get keys with prefix
- `config_export_json(manager)` - Export configuration as JSON
- `config_debug_info(manager)` - Get detailed debug information

## Best Practices

1. **Use Priority Ordering**: Set appropriate priorities for different configuration sources
2. **Environment Variables**: Use environment variables for deployment-specific overrides  
3. **Validation Rules**: Add validation for critical configuration values
4. **Default Values**: Provide sensible defaults for all configuration options
5. **Hot Reloading**: Enable hot reloading for development and production flexibility
6. **Error Handling**: Always check configuration validity during application startup
7. **Documentation**: Document all configuration options and their expected values

## Production Deployment

For production deployments, consider:

- Use file-based configuration for application defaults
- Override with environment variables for deployment-specific settings
- Enable configuration validation to catch errors early
- Use hot reloading for zero-downtime configuration updates
- Monitor configuration file changes in production logs
- Implement configuration rollback mechanisms for critical changes

The configz module is designed to handle enterprise-scale configuration management with reliability, performance, and ease of use.

## Package Structure

The configz package consists of several specialized modules:

### Core Modules

- **[`mod.csd`](./mod.csd)** - Main configuration management system
- **[`demo.csd`](./demo.csd)** - Comprehensive demonstration of all features
- **[`test_configz.csd`](./test_configz.csd)** - Complete test suite

### Advanced Modules

- **[`env_integration.csd`](./env_integration.csd)** - Advanced environment variable processing
- **[`yaml_parser.csd`](./yaml_parser.csd)** - Comprehensive YAML parsing engine
- **[`hot_reload.csd`](./hot_reload.csd)** - Hot reloading and file watching system
- **[`validation.csd`](./validation.csd)** - Advanced configuration validation system

## Quick Test

Run the comprehensive test suite:
```bash
./zig-out/bin/cursed-zig stdlib/configz/test_configz.csd
```

Run the interactive demo:
```bash
./zig-out/bin/cursed-zig stdlib/configz/demo.csd
```

## Memory Safety

✅ **Zero Memory Leaks**: All configz modules pass valgrind validation with no memory leaks
✅ **Production Ready**: Extensively tested with comprehensive test suite
✅ **Enterprise Grade**: Designed for high-load production environments

## Integration with CURSED Ecosystem

The configz package integrates seamlessly with other CURSED stdlib modules:

- **vibez**: For output and logging
- **filez**: For file operations and watching  
- **jsonz**: For JSON configuration parsing
- **stringz**: For string manipulation and processing
- **timez**: For timestamp and scheduling operations
- **envz**: For environment variable handling

## Contributing

To extend the configz package:

1. Follow existing code patterns and naming conventions
2. Add comprehensive tests for new features
3. Ensure memory safety with valgrind validation
4. Update documentation and examples
5. Test integration with other stdlib modules

## License

This package is part of the CURSED programming language standard library and follows the same license terms as the main CURSED distribution.
