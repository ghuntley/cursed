# ConfigZ - Enhanced Configuration Management for CURSED

## Overview

The `configz` module provides a comprehensive configuration management framework for CURSED applications. It supports multiple configuration formats, environment variable integration, schema validation, type safety, and secure configuration handling.

## Features

### Multi-Format Support
- **JSON** - Standard JSON configuration files
- **YAML** - Human-readable YAML format
- **TOML** - Tom's Obvious Minimal Language
- **INI** - Traditional INI/config files
- **Environment Files** - `.env` style files
- **Environment Variables** - System environment integration

### Advanced Capabilities
- **Schema Definition** - Define required/optional keys with validation rules
- **Type Safety** - Automatic type detection and conversion
- **Environment Substitution** - `${VAR}` expansion in configuration values
- **Configuration Layering** - Merge multiple config sources with precedence
- **Validation Framework** - Comprehensive validation with custom rules
- **Default Values** - Automatic default value application
- **Error Handling** - Detailed validation error reporting

## Basic Usage

### Quick Start

```cursed
yeet "configz"

// Load configuration from file with auto-detection
sus ctx ConfigContext = load_configuration_file("config.json")

// Get configuration values with defaults
sus app_name tea = get_config_string(ctx, "app_name", "MyApp")
sus port normie = get_config_int(ctx, "port", 3000)
sus debug lit = get_config_bool(ctx, "debug", cap)

// Check if configuration is valid
ready (is_configuration_valid(ctx)) {
    vibez.spill("Configuration loaded successfully!")
} otherwise {
    sus errors []tea = get_validation_errors(ctx)
    vibez.spill("Configuration errors:", errors)
}
```

### Schema-Based Configuration

```cursed
yeet "configz"

// Create configuration schema
sus schema ConfigSchema = create_schema("app_config")
schema = add_required_key(schema, "database_url")
schema = add_required_key(schema, "api_key")
schema = add_optional_key(schema, "debug", "false")
schema = add_optional_key(schema, "port", "3000")

// Add validation rules
schema = add_validator(schema, "database_url", "url")
schema = add_validator(schema, "port", "integer")
schema = add_validator(schema, "api_key", "min_length:10")

// Load configuration with schema validation
sus config_files []tea = ["config.json", "config.local.json"]
sus ctx ConfigContext = load_config_with_defaults(config_files, schema)

// Check validation results
ready (is_configuration_valid(ctx)) {
    vibez.spill("Configuration is valid!")
} otherwise {
    sus errors []tea = get_validation_errors(ctx)
    sus i normie = 0
    bestie (i < len(errors)) {
        vibez.spill("Error:", errors[i])
        i = i + 1
    }
}
```

## Configuration Formats

### JSON Configuration

```json
{
  "database": {
    "host": "${DB_HOST}",
    "port": 5432,
    "name": "myapp"
  },
  "app": {
    "name": "MyApplication",
    "debug": true,
    "features": ["auth", "api", "logging"]
  }
}
```

```cursed
// Parse JSON configuration
sus json_content tea = read_file_content("config.json")
sus ctx ConfigContext = parse_json_advanced(json_content)
```

### YAML Configuration

```yaml
database:
  host: ${DB_HOST}
  port: 5432
  name: myapp

app:
  name: MyApplication
  debug: true
  features:
    - auth
    - api
    - logging
```

```cursed
// Parse YAML configuration
sus yaml_content tea = read_file_content("config.yaml")
sus ctx ConfigContext = parse_yaml_advanced(yaml_content)
```

### TOML Configuration

```toml
[database]
host = "${DB_HOST}"
port = 5432
name = "myapp"

[app]
name = "MyApplication"
debug = true
features = ["auth", "api", "logging"]
```

```cursed
// Parse TOML configuration
sus toml_content tea = read_file_content("config.toml")
sus ctx ConfigContext = parse_toml_advanced(toml_content)
```

### Environment File Configuration

```bash
# .env file
DATABASE_HOST=localhost
DATABASE_PORT=5432
DATABASE_NAME=myapp
APP_NAME=MyApplication
DEBUG=true
FEATURES=auth,api,logging
```

```cursed
// Parse environment file
sus env_content tea = read_file_content(".env")
sus ctx ConfigContext = parse_env_advanced(env_content)
```

## Environment Variable Integration

### Environment Variable Substitution

Configuration values can reference environment variables using `${VAR}` syntax:

```json
{
  "database_url": "${DATABASE_URL}",
  "api_endpoint": "https://${API_HOST}:${API_PORT}/api",
  "log_file": "${HOME}/logs/app.log"
}
```

```cursed
// Environment variables are automatically expanded
sus ctx ConfigContext = load_configuration_file("config.json")
sus db_url tea = get_config_string(ctx, "database_url", "")
// db_url will contain the expanded value from DATABASE_URL environment variable
```

### Loading Environment Configuration

```cursed
// Load configuration from environment variables
sus env_ctx ConfigContext = load_environment_config()

// Common environment variables are automatically loaded:
// HOME, PATH, USER, NODE_ENV, DATABASE_URL, API_KEY, etc.
sus home_dir tea = get_config_string(env_ctx, "HOME", "/tmp")
sus node_env tea = get_config_string(env_ctx, "NODE_ENV", "development")
```

## Configuration Layering and Merging

### Multiple Configuration Sources

```cursed
// Load configuration from multiple sources with precedence
// Order: base config < environment-specific config < environment variables

sus config_files []tea = [
    "config.json",           // Base configuration
    "config.production.json" // Environment-specific overrides
]

sus schema ConfigSchema = create_schema("layered_config")
schema = add_required_key(schema, "database_url")
schema = add_optional_key(schema, "debug", "false")

// Create layered configuration
sus ctx ConfigContext = load_config_with_defaults(config_files, schema)
```

### Manual Configuration Merging

```cursed
// Create base configuration
sus base_ctx ConfigContext = load_configuration_file("base.json")

// Create override configuration
sus override_ctx ConfigContext = load_configuration_file("overrides.json")

// Merge configurations (override takes precedence)
sus merged_ctx ConfigContext = merge_configurations(base_ctx, override_ctx)
```

## Schema Definition and Validation

### Creating Schemas

```cursed
// Create a new schema
sus schema ConfigSchema = create_schema("api_server")

// Add required keys
schema = add_required_key(schema, "database_url")
schema = add_required_key(schema, "jwt_secret")
schema = add_required_key(schema, "redis_url")

// Add optional keys with defaults
schema = add_optional_key(schema, "port", "3000")
schema = add_optional_key(schema, "debug", "false")
schema = add_optional_key(schema, "log_level", "info")
schema = add_optional_key(schema, "max_connections", "100")
```

### Validation Rules

```cursed
// Add validation rules to schema
schema = add_validator(schema, "database_url", "url")
schema = add_validator(schema, "redis_url", "url")
schema = add_validator(schema, "port", "integer")
schema = add_validator(schema, "debug", "boolean")
schema = add_validator(schema, "jwt_secret", "min_length:32")
schema = add_validator(schema, "max_connections", "integer")
schema = add_validator(schema, "log_level", "required")

// Validate configuration against schema
sus ctx ConfigContext = load_configuration_file("config.json")
sus validated_ctx ConfigContext = validate_against_schema(ctx, schema)

// Check validation results
ready (!is_configuration_valid(validated_ctx)) {
    vibez.spill("Configuration validation failed:")
    sus errors []tea = get_validation_errors(validated_ctx)
    sus i normie = 0
    bestie (i < len(errors)) {
        vibez.spill("  -", errors[i])
        i = i + 1
    }
}
```

### Available Validation Rules

- `required` - Value must be present and non-empty
- `integer` - Value must be a valid integer
- `boolean` - Value must be a valid boolean (true/false/1/0/yes/no/on/off)
- `url` - Value must be a valid URL (http://, https://, ftp://)
- `email` - Value must be a valid email address
- `min_length:N` - String must be at least N characters long
- `max_length:N` - String must be at most N characters long

## Type Conversion and Safety

### Automatic Type Detection

```cursed
// The system automatically detects value types
sus ctx ConfigContext = load_configuration_file("config.json")

// Type detection happens automatically
// "true" -> boolean
// "42" -> integer
// "3.14" -> float
// "[1,2,3]" -> array
// "{...}" -> object
// "hello" -> string
```

### Type-Safe Value Retrieval

```cursed
// Get values with automatic type conversion
sus app_name tea = get_config_string(ctx, "app_name", "DefaultApp")
sus port normie = get_config_int(ctx, "port", 3000)
sus debug lit = get_config_bool(ctx, "debug", cap)
sus features []tea = get_config_array(ctx, "features")

// Manual type conversion
sus timeout_str tea = get_config_string(ctx, "timeout", "30")
sus timeout_int normie = convert_to_integer(timeout_str)
sus verbose_str tea = get_config_string(ctx, "verbose", "false")
sus verbose_bool lit = convert_to_boolean(verbose_str)
```

## Advanced Usage Examples

### Web Application Configuration

```cursed
yeet "configz"

slay setup_web_app_config() ConfigContext {
    // Create web application schema
    sus schema ConfigSchema = create_schema("web_app")
    
    // Database configuration
    schema = add_required_key(schema, "database_url")
    schema = add_validator(schema, "database_url", "url")
    
    // Server configuration
    schema = add_optional_key(schema, "port", "3000")
    schema = add_validator(schema, "port", "integer")
    schema = add_optional_key(schema, "host", "localhost")
    
    // Security configuration
    schema = add_required_key(schema, "jwt_secret")
    schema = add_validator(schema, "jwt_secret", "min_length:32")
    schema = add_optional_key(schema, "session_timeout", "3600")
    schema = add_validator(schema, "session_timeout", "integer")
    
    // Feature flags
    schema = add_optional_key(schema, "enable_auth", "true")
    schema = add_validator(schema, "enable_auth", "boolean")
    schema = add_optional_key(schema, "enable_logging", "true")
    schema = add_validator(schema, "enable_logging", "boolean")
    
    // Load configuration with environment-specific overrides
    sus config_files []tea = [
        "config/default.json",
        "config/production.json"
    ]
    
    sus ctx ConfigContext = load_config_with_defaults(config_files, schema)
    
    // Validate configuration
    ready (!is_configuration_valid(ctx)) {
        vibez.spill("Configuration validation failed!")
        sus errors []tea = get_validation_errors(ctx)
        sus i normie = 0
        bestie (i < len(errors)) {
            vibez.spill("Error:", errors[i])
            i = i + 1
        }
        // Could exit application or use fallback config
    }
    
    damn ctx
}

// Use the configuration
sus app_config ConfigContext = setup_web_app_config()
sus database_url tea = get_config_string(app_config, "database_url", "")
sus server_port normie = get_config_int(app_config, "port", 3000)
sus auth_enabled lit = get_config_bool(app_config, "enable_auth", based)
```

### Microservice Configuration

```cursed
yeet "configz"

slay setup_microservice_config() ConfigContext {
    // Create microservice schema with service discovery
    sus schema ConfigSchema = create_schema("microservice")
    
    // Service identification
    schema = add_required_key(schema, "service_name")
    schema = add_required_key(schema, "service_version")
    
    // Network configuration
    schema = add_optional_key(schema, "listen_port", "8080")
    schema = add_validator(schema, "listen_port", "integer")
    schema = add_optional_key(schema, "health_check_port", "8081")
    schema = add_validator(schema, "health_check_port", "integer")
    
    // Service dependencies
    schema = add_optional_key(schema, "database_service_url", "")
    schema = add_optional_key(schema, "auth_service_url", "")
    schema = add_optional_key(schema, "cache_service_url", "")
    
    // Monitoring and observability
    schema = add_optional_key(schema, "metrics_enabled", "true")
    schema = add_validator(schema, "metrics_enabled", "boolean")
    schema = add_optional_key(schema, "trace_sampling_rate", "0.1")
    
    // Load configuration from multiple sources
    sus config_files []tea = [
        "service.yaml",
        "/etc/microservice/config.yaml"
    ]
    
    sus ctx ConfigContext = load_config_with_defaults(config_files, schema)
    
    // Add environment-specific configuration
    sus env_ctx ConfigContext = load_environment_config()
    ctx = merge_configurations(ctx, env_ctx)
    
    // Final validation
    ctx = validate_against_schema(ctx, schema)
    
    damn ctx
}
```

### Development vs Production Configuration

```cursed
yeet "configz"

slay load_environment_specific_config() ConfigContext {
    // Detect current environment
    sus environment tea = detect_environment()
    
    // Base configuration
    sus config_files []tea = ["config/base.json"]
    
    // Add environment-specific configuration
    ready (environment == "development") {
        config_files = append_string(config_files, "config/development.json")
    } alternatively ready (environment == "production") {
        config_files = append_string(config_files, "config/production.json")
    } alternatively ready (environment == "test") {
        config_files = append_string(config_files, "config/test.json")
    }
    
    // Create schema appropriate for environment
    sus schema ConfigSchema = create_schema("env_specific")
    
    ready (environment == "production") {
        // Stricter validation for production
        schema = add_required_key(schema, "database_url")
        schema = add_required_key(schema, "redis_url")
        schema = add_required_key(schema, "jwt_secret")
        schema = add_validator(schema, "database_url", "url")
        schema = add_validator(schema, "redis_url", "url")
        schema = add_validator(schema, "jwt_secret", "min_length:64")
    } otherwise {
        // More lenient for development/test
        schema = add_optional_key(schema, "database_url", "sqlite://./dev.db")
        schema = add_optional_key(schema, "redis_url", "redis://localhost:6379")
        schema = add_optional_key(schema, "jwt_secret", "dev_secret_key")
    }
    
    // Common configuration
    schema = add_optional_key(schema, "debug", environment == "development" ? "true" : "false")
    schema = add_optional_key(schema, "log_level", environment == "development" ? "debug" : "info")
    
    sus ctx ConfigContext = load_config_with_defaults(config_files, schema)
    damn ctx
}
```

## Error Handling

### Validation Error Handling

```cursed
yeet "configz"

slay handle_configuration_errors(ctx ConfigContext) {
    ready (!is_configuration_valid(ctx)) {
        vibez.spill("Configuration validation failed:")
        
        sus errors []tea = get_validation_errors(ctx)
        sus i normie = 0
        bestie (i < len(errors)) {
            vibez.spill("  Error:", errors[i])
            i = i + 1
        }
        
        // Log configuration details for debugging
        vibez.spill("Configuration source:", ctx.source_file)
        vibez.spill("Configuration format:", ctx.format)
        vibez.spill("Environment:", ctx.environment)
        
        // Could implement fallback configuration or exit
        vibez.spill("Using fallback configuration...")
        // return create_fallback_config()
    } otherwise {
        vibez.spill("Configuration loaded successfully from:", ctx.source_file)
    }
}
```

### Format Detection Errors

```cursed
yeet "configz"

slay safe_load_configuration(filename tea) ConfigContext {
    sus content tea = read_file_content(filename)
    
    // Try auto-detection first
    sus format tea = auto_detect_format(content)
    
    sus ctx ConfigContext
    ready (format == format_json()) {
        ctx = parse_json_advanced(content)
    } alternatively ready (format == format_yaml()) {
        ctx = parse_yaml_advanced(content)
    } alternatively ready (format == format_toml()) {
        ctx = parse_toml_advanced(content)
    } otherwise {
        // Fallback to JSON parsing
        vibez.spill("Warning: Could not detect format, trying JSON...")
        ctx = parse_json_advanced(content)
    }
    
    ready (!is_configuration_valid(ctx)) {
        vibez.spill("Failed to parse", filename, "as", format)
        sus errors []tea = get_validation_errors(ctx)
        vibez.spill("Parse errors:", errors)
    }
    
    damn ctx
}
```

## Security Considerations

### Secure Configuration Handling

1. **Environment Variable Expansion**: Only expand trusted environment variables
2. **Input Validation**: Always validate configuration values against schemas
3. **Secret Management**: Use secure methods for sensitive configuration
4. **File Permissions**: Ensure configuration files have appropriate permissions

```cursed
// Example of secure configuration loading
slay load_secure_config() ConfigContext {
    // Create schema with strict validation
    sus schema ConfigSchema = create_schema("secure_app")
    
    // Validate sensitive configuration
    schema = add_required_key(schema, "jwt_secret")
    schema = add_validator(schema, "jwt_secret", "min_length:64")
    
    schema = add_required_key(schema, "database_password")
    schema = add_validator(schema, "database_password", "min_length:12")
    
    // Load configuration
    sus ctx ConfigContext = load_configuration_file("secure_config.json")
    ctx = validate_against_schema(ctx, schema)
    
    // Ensure validation passed
    ready (!is_configuration_valid(ctx)) {
        vibez.spill("SECURITY ERROR: Configuration validation failed!")
        // Should terminate application or use secure defaults
    }
    
    damn ctx
}
```

## Performance Considerations

- Configuration parsing is optimized for startup time
- Environment variable expansion is cached
- Schema validation is performed once during loading
- Type conversion is lazy and cached where possible

## Best Practices

1. **Use Schemas**: Always define schemas for production applications
2. **Layer Configuration**: Use base + environment-specific configuration files
3. **Environment Variables**: Use environment variables for deployment-specific values
4. **Validation**: Validate all configuration values, especially in production
5. **Defaults**: Provide sensible defaults for optional configuration
6. **Documentation**: Document all configuration options and their valid values
7. **Security**: Never commit sensitive configuration to version control

## Integration with Other Modules

The `configz` module integrates seamlessly with other CURSED stdlib modules:

- `envz` - Environment variable management
- `jsonz` - JSON parsing and generation
- `stringz` - String manipulation utilities
- `testz` - Testing framework for configuration validation

## Examples Directory Structure

```
config/
├── base.json              # Base configuration
├── development.json       # Development overrides  
├── production.json        # Production overrides
├── test.json             # Test configuration
└── schema/
    ├── app.schema.json   # Application schema
    └── db.schema.json    # Database schema
```

This comprehensive configuration management system provides the foundation for robust, maintainable CURSED applications with proper configuration handling, validation, and security.
