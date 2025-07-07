# CURSED Configuration Management Library

A comprehensive configuration management system for CURSED applications with support for multiple formats, type conversion, variable expansion, and schema validation.

## Overview

The config module provides a unified interface for loading, parsing, and managing configuration files in various formats. It supports INI, environment variables, JSON, and YAML-like formats with advanced features like variable expansion, schema validation, and configuration merging.

## Supported Formats

### INI Format
Classic INI format with sections, comments, and key-value pairs:

```ini
# Global configuration
debug=true
app_name=MyApp

[database]
host=localhost
port=5432
name=myapp
username=admin
password="secret123"

[server]
host=0.0.0.0
port=8080
workers=4
timeout=30

; Alternative comment style
[logging]
level=info
file=/var/log/app.log
rotate=daily
```

**Features:**
- Section support with `[section]` syntax
- Comments with `#` and `;`
- Quoted values with `"` and `'`
- Global keys outside sections
- Whitespace handling

### Environment Variables
Environment variable format with optional export statements:

```bash
# Environment configuration
NODE_ENV=production
DEBUG=false
export DATABASE_URL=postgres://localhost:5432/myapp
API_KEY="sk-1234567890abcdef"
REDIS_URL='redis://localhost:6379'
WORKERS=8
TIMEOUT=60

# Web server configuration
WEB_HOST=0.0.0.0
WEB_PORT=3000
WEB_CORS_ORIGIN=*
```

**Features:**
- Standard `KEY=value` syntax
- Support for `export` statements
- Quoted values with `"` and `'`
- Comments with `#`
- Automatic uppercase conversion for output

### JSON Format
Standard JSON with automatic flattening to dot notation:

```json
{
  "app": {
    "name": "MyApp",
    "version": "1.0.0",
    "debug": true
  },
  "database": {
    "host": "localhost",
    "port": 5432,
    "credentials": {
      "username": "admin",
      "password": "secret"
    }
  },
  "features": {
    "authentication": true,
    "caching": false,
    "logging": {
      "level": "info",
      "file": "/var/log/app.log"
    }
  }
}
```

**Features:**
- Automatic flattening to dot notation (e.g., `app.name`, `database.credentials.username`)
- Full JSON spec compliance
- Nested object support
- Type preservation

### YAML-like Format
Simple YAML-like format with indentation-based structure:

```yaml
# YAML-like configuration
app:
  name: MyApp
  version: 2.0.0
  debug: false

database:
  host: db.example.com
  port: 5432
  pool:
    min: 2
    max: 10

server:
  host: 0.0.0.0
  port: 8080
  ssl:
    enabled: true
    cert: /path/to/cert.pem
    key: /path/to/key.pem
```

**Features:**
- Indentation-based structure
- Colon-separated key-value pairs
- Comments with `#`
- Nested configuration support

## Core Functions

### File Operations

#### `load_file(filepath tea) map`
Load configuration from file with automatic format detection based on file extension.

```cursed
sus config map = load_file("config.ini");
sus json_config map = load_file("app.json");
sus env_config map = load_file(".env");
```

#### `save_file(config map, filepath tea) lit`
Save configuration to file in the appropriate format.

```cursed
sus success lit = save_file(config, "output.ini");
sus json_success lit = save_file(config, "output.json");
```

### Format-Specific Parsing

#### `parse_ini(content tea) map`
Parse INI format configuration from string content.

```cursed
sus ini_content tea = "[database]\nhost=localhost\nport=5432";
sus config map = parse_ini(ini_content);
```

#### `parse_env(content tea) map`
Parse environment variable format from string content.

```cursed
sus env_content tea = "DATABASE_HOST=localhost\nDATABASE_PORT=5432";
sus config map = parse_env(env_content);
```

#### `parse_json_config(content tea) map`
Parse JSON configuration with automatic flattening.

```cursed
sus json_content tea = "{\"db\": {\"host\": \"localhost\"}}";
sus config map = parse_json_config(json_content);
// Access as: get_value(config, "db.host")
```

### Format-Specific Serialization

#### `stringify_ini(config map) tea`
Convert configuration to INI format string.

```cursed
sus ini_output tea = stringify_ini(config);
```

#### `stringify_env(config map) tea`
Convert configuration to environment variable format.

```cursed
sus env_output tea = stringify_env(config);
```

### Configuration Access

#### `get_value(config map, key tea) tea`
Get configuration value with path support using dot notation.

```cursed
sus host tea = get_value(config, "database.host");
sus port tea = get_value(config, "server.port");
```

#### `set_value(config map, key tea, value tea) map`
Set configuration value with path support.

```cursed
sus updated_config map = set_value(config, "database.host", "remote.db.com");
```

#### `has_key(config map, key tea) lit`
Check if configuration key exists.

```cursed
sus exists lit = has_key(config, "database.host");
```

#### `get_default(config map, key tea, default_value tea) tea`
Get configuration value with default fallback.

```cursed
sus host tea = get_default(config, "database.host", "localhost");
sus port tea = get_default(config, "server.port", "8080");
```

#### `get_section(config map, section tea) map`
Get entire configuration section as a separate map.

```cursed
sus db_config map = get_section(config, "database");
sus host tea = get_value(db_config, "host"); // No need for "database." prefix
```

### Configuration Merging

#### `merge_configs(base map, override map) map`
Merge two configurations with override priority.

```cursed
sus base_config map = load_file("base.ini");
sus env_config map = load_file("production.ini");
sus final_config map = merge_configs(base_config, env_config);
```

#### `apply_overrides(config map, overrides map) map`
Apply command-line style overrides to configuration.

```cursed
sus overrides map = map_create();
overrides = set_value(overrides, "debug", "true");
overrides = set_value(overrides, "database.host", "localhost");

sus final_config map = apply_overrides(config, overrides);
```

### Schema Validation

#### `validate_schema(config map, schema map) lit`
Validate configuration against a schema definition.

```cursed
sus schema map = map_create();
sus required_keys [tea] = ["database.host", "database.port", "server.host"];
schema = map_set(schema, "required", required_keys);

sus is_valid lit = validate_schema(config, schema);
```

### Variable Expansion

#### `expand_variables(config map) map`
Expand `${VAR}` and `${VAR:default}` style variables.

```cursed
// Configuration with variables
sus config map = map_create();
config = set_value(config, "app_name", "MyApp");
config = set_value(config, "database.url", "postgres://${database.host:localhost}:5432/${app_name}");

sus expanded_config map = expand_variables(config);
// Result: database.url = "postgres://localhost:5432/MyApp"
```

### Environment Integration

#### `load_environment() map`
Load system environment variables into configuration map.

```cursed
sus env_vars map = load_environment();
sus path tea = get_value(env_vars, "PATH");
```

### Type Conversion

#### `get_int_value(config map, key tea, default_value normie) normie`
Get integer value with type conversion and default.

```cursed
sus port normie = get_int_value(config, "server.port", 8080);
```

#### `get_bool_value(config map, key tea, default_value lit) lit`
Get boolean value with type conversion and default.

```cursed
sus debug lit = get_bool_value(config, "debug", cap);
```

#### `get_float_value(config map, key tea, default_value meal) meal`
Get float value with type conversion and default.

```cursed
sus timeout meal = get_float_value(config, "server.timeout", 30.0);
```

#### `get_typed_value(config map, key tea) extra`
Get value with automatic type inference.

```cursed
sus value extra = get_typed_value(config, "some.key");
```

## Usage Patterns

### Basic Application Configuration

```cursed
yeet "config"

slay load_app_config() map {
    // Load base configuration
    sus config map = load_file("config.ini");
    
    // Load environment-specific overrides
    sus env_config map = load_file("production.ini");
    
    // Merge configurations
    sus final_config map = merge_configs(config, env_config);
    
    // Expand variables
    damn expand_variables(final_config);
}

slay main() {
    sus config map = load_app_config();
    
    sus db_host tea = get_value(config, "database.host");
    sus db_port normie = get_int_value(config, "database.port", 5432);
    sus debug lit = get_bool_value(config, "debug", cap);
    
    vibez.spill("Database: " + db_host + ":" + string_from_int(db_port));
    vibez.spill("Debug mode: " + string_from_bool(debug));
}
```

### Environment-Specific Configuration

```cursed
yeet "config"

slay load_environment_config(environment tea) map {
    // Load base configuration
    sus base_config map = load_file("config/base.ini");
    
    // Load environment-specific configuration
    sus env_file tea = "config/" + environment + ".ini";
    sus env_config map = load_file(env_file);
    
    // Load system environment variables
    sus system_env map = load_environment();
    
    // Merge in priority order: base -> environment -> system
    sus config map = merge_configs(base_config, env_config);
    config = merge_configs(config, system_env);
    
    // Expand variables and return
    damn expand_variables(config);
}

slay main() {
    sus environment tea = get_default(load_environment(), "NODE_ENV", "development");
    sus config map = load_environment_config(environment);
    
    vibez.spill("Environment: " + environment);
    vibez.spill("Database URL: " + get_value(config, "database.url"));
}
```

### Configuration with Schema Validation

```cursed
yeet "config"

slay create_app_schema() map {
    sus schema map = map_create();
    
    // Define required keys
    sus required_keys [tea] = [
        "database.host",
        "database.port",
        "database.name",
        "server.host",
        "server.port"
    ];
    
    schema = map_set(schema, "required", required_keys);
    damn schema;
}

slay load_validated_config() map {
    sus config map = load_file("app.ini");
    sus schema map = create_app_schema();
    
    simp !validate_schema(config, schema) {
        vibez.spill("Configuration validation failed!");
        // Handle validation error
        damn map_create();
    }
    
    damn config;
}
```

### Multi-Format Configuration Loading

```cursed
yeet "config"

slay load_multi_format_config() map {
    sus config map = map_create();
    
    // Load from multiple sources
    sus ini_config map = load_file("config.ini");
    sus json_config map = load_file("config.json");
    sus env_config map = parse_env(load_environment_file(".env"));
    
    // Merge configurations
    config = merge_configs(config, ini_config);
    config = merge_configs(config, json_config);
    config = merge_configs(config, env_config);
    
    // Apply command-line overrides
    sus overrides map = parse_command_line_args();
    config = apply_overrides(config, overrides);
    
    damn expand_variables(config);
}
```

## Best Practices

### 1. Configuration Layering
Use a layered approach with base configuration and environment-specific overrides:

```
config/
├── base.ini          # Base configuration
├── development.ini   # Development overrides
├── staging.ini       # Staging overrides
├── production.ini    # Production overrides
└── local.ini         # Local development overrides (gitignored)
```

### 2. Environment Variables
Use environment variables for sensitive data and deployment-specific values:

```cursed
// Don't put secrets in config files
sus db_password tea = get_default(load_environment(), "DATABASE_PASSWORD", "");
```

### 3. Variable Expansion
Use variable expansion for dynamic configuration:

```cursed
# In config file
app_name=MyApp
log_file=/var/log/${app_name}.log
backup_dir=/backup/${app_name}/${NODE_ENV}
```

### 4. Schema Validation
Always validate configuration in production:

```cursed
sus config map = load_file("production.ini");
sus schema map = create_production_schema();

simp !validate_schema(config, schema) {
    vibez.spill("CRITICAL: Configuration validation failed!");
    // Exit or use safe defaults
}
```

### 5. Type Conversion
Use typed accessors for type safety:

```cursed
// Instead of string parsing
sus port normie = get_int_value(config, "server.port", 8080);
sus debug lit = get_bool_value(config, "debug", cap);
sus timeout meal = get_float_value(config, "timeout", 30.0);
```

## Security Considerations

### 1. Sensitive Data
- Never commit sensitive data to version control
- Use environment variables for secrets
- Consider using secret management systems

### 2. File Permissions
- Ensure configuration files have appropriate permissions
- Restrict access to configuration directories
- Use secure file storage for production

### 3. Variable Expansion
- Be cautious with variable expansion to prevent injection
- Validate variable names and values
- Use whitelisting for allowed variables

### 4. Input Validation
- Always validate configuration values
- Use schema validation for structure
- Implement range checks for numerical values

## Error Handling

The configuration module handles errors gracefully:

- **Malformed files**: Parses valid sections, skips invalid ones
- **Missing files**: Returns empty configuration
- **Invalid formats**: Falls back to default parsing
- **Type conversion errors**: Uses default values

## Testing

Run the comprehensive test suite:

```bash
# Run configuration module tests
cargo run --bin cursed stdlib/config/test_config.csd

# Run specific test categories
cargo run --bin cursed test --filter config
```

The test suite includes:
- 15+ test functions covering all features
- Edge case testing (empty files, malformed config)
- Unicode and special character support
- Cross-format compatibility testing
- Schema validation testing
- Variable expansion testing

## Performance Considerations

- Configuration parsing is optimized for startup time
- Use caching for frequently accessed configurations
- Consider lazy loading for large configuration files
- Profile configuration loading in production

## Integration with Other Modules

The config module integrates with:
- **string**: For text processing and manipulation
- **json**: For JSON format support
- **collections**: For map and array operations
- **io**: For file operations (when implemented)

## Migration Guide

### From Manual Configuration
Replace manual string parsing with config module functions:

```cursed
// Before
sus config_line tea = "database.host=localhost";
sus parts [tea] = string_split(config_line, "=");
sus host tea = parts[1];

// After
sus config map = parse_ini("database.host=localhost");
sus host tea = get_value(config, "database.host");
```

### From Single Format
Migrate from single format to multi-format support:

```cursed
// Before
sus config map = parse_ini(file_content);

// After
sus config map = load_file("config.ini"); // Auto-detects format
```

## Contributing

When contributing to the config module:
1. Follow existing code patterns
2. Add comprehensive tests for new features
3. Update documentation
4. Ensure backward compatibility
5. Test with all supported formats

## License

This module is part of the CURSED standard library and follows the same licensing terms as the CURSED language.
