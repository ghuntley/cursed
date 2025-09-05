# CURSED Config Module

A comprehensive configuration management library for the CURSED programming language. This module provides multi-format configuration parsing, environment variable handling, and validation capabilities with pure CURSED implementation.

## Features

- **Multi-Format Support**: JSON, YAML, TOML, INI, and ENV file formats
- **Auto-Detection**: Automatic format detection from content or filename
- **Environment Variables**: Full environment variable support with expansion
- **Validation**: Configuration validation and schema checking
- **Merging**: Configuration merging and value manipulation
- **Pure CURSED**: No FFI dependencies, fully portable implementation

## Supported Formats

### JSON Configuration
```json
{
  "database": {
    "host": "localhost",
    "port": "5432"
  },
  "app": {
    "name": "MyApp",
    "debug": "true"
  }
}
```

### INI Configuration
```ini
[database]
host=localhost
port=5432

[app]
name=MyApp
debug=true
```

### YAML Configuration
```yaml
database:
  host: localhost
  port: 5432
app:
  name: MyApp
  debug: true
```

### TOML Configuration
```toml
[database]
host = "localhost"
port = "5432"

[app]
name = "MyApp"
debug = "true"
```

### Environment Configuration
```env
DATABASE_HOST=localhost
DATABASE_PORT=5432
APP_NAME=MyApp
DEBUG=true
```

## API Reference

### Core Functions

#### `parse(content tea) tea`
Parse configuration with automatic format detection.

```cursed
sus config_content tea = "{\"api\": \"localhost\", \"port\": \"8080\"}"
sus parsed_config tea = config.parse(config_content)
```

#### `parse_with_format(content tea, format tea) tea`
Parse configuration with explicit format specification.

```cursed
sus ini_content tea = "[api]\nhost=localhost\nport=8080"
sus parsed_ini tea = config.parse_with_format(ini_content, "ini")
```

#### `load_config_from_file(filename tea) tea`
Load configuration from file (simulated).

```cursed
sus file_config tea = config.load_config_from_file("config.json")
```

### Format Detection

#### `detect_format(content tea) tea`
Auto-detect configuration format from content.

```cursed
sus format tea = config.detect_format("{\"key\": \"value\"}")
# Returns: "json"
```

#### `detect_format_from_filename(filename tea) tea`
Detect format from file extension.

```cursed
sus format tea = config.detect_format_from_filename("app.yaml")
# Returns: "yaml"
```

### Environment Variables

#### `get_env(key tea) tea`
Get environment variable value.

```cursed
sus home_path tea = config.get_env("HOME")
```

#### `set_env(key tea, value tea) lit`
Set environment variable (simulated).

```cursed
config.set_env("MY_VAR", "my_value")
```

#### `has_env(key tea) lit`
Check if environment variable exists.

```cursed
bestie config.has_env("PATH") {
    vibez.spill("PATH is set")
}
```

#### `expand_env_vars(input tea) tea`
Expand environment variables in string.

```cursed
sus template tea = "Config at ${HOME}/app.conf"
sus expanded tea = config.expand_env_vars(template)
# Returns: "Config at /home/user/app.conf"
```

### Configuration Manipulation

#### `get_value(config tea, key tea) tea`
Get value from configuration by key.

```cursed
sus db_host tea = config.get_value(my_config, "database_host")
```

#### `set_value(config tea, key tea, value tea) tea`
Set value in configuration.

```cursed
sus updated_config tea = config.set_value(my_config, "debug", "false")
```

#### `has_key(config tea, key tea) lit`
Check if configuration has a specific key.

```cursed
bestie config.has_key(my_config, "api_key") {
    vibez.spill("API key is configured")
}
```

#### `merge(config1 tea, config2 tea) tea`
Merge two configurations (config2 takes precedence).

```cursed
sus base_config tea = config.parse("{\"host\": \"localhost\"}")
sus env_config tea = config.parse("{\"port\": \"8080\"}")
sus merged tea = config.merge(base_config, env_config)
```

### Validation

#### `validate(config tea) lit`
Validate configuration format.

```cursed
bestie config.validate(my_config) {
    vibez.spill("Configuration is valid")
} else {
    vibez.spill("Invalid configuration")
}
```

#### `validate_config(config tea, schema tea) lit`
Validate configuration against schema.

```cursed
sus schema tea = "{\"host\": \"string\", \"port\": \"number\"}"
bestie config.validate_config(my_config, schema) {
    vibez.spill("Configuration matches schema")
}
```

## Usage Examples

### Basic Configuration Loading

```cursed
yeet "config"

# Load JSON configuration
sus json_config tea = "{\"database\": {\"host\": \"localhost\", \"port\": \"5432\"}}"
sus parsed tea = config.parse(json_config)

# Get database host
sus db_host tea = config.get_value(parsed, "host")
vibez.spill("Database host: " + db_host)
```

### Environment Variable Expansion

```cursed
yeet "config"

# Configuration template with environment variables
sus template tea = "database_url=${HOME}/app.db\nuser=${USER}"

# Expand variables
sus expanded tea = config.expand_env_vars(template)

# Parse as environment configuration
sus env_config tea = config.parse_with_format(expanded, "env")
```

### Multi-Format Configuration Loading

```cursed
yeet "config"

# Auto-detect and parse different formats
sus formats [tea] = [
    "{\"api\": \"localhost\"}",           # JSON
    "[api]\nhost=localhost",              # INI
    "api:\n  host: localhost",            # YAML
    "api = \"localhost\"",                # TOML
    "API_HOST=localhost"                  # ENV
]

bestie i := 0; i < len(formats); i++ {
    sus parsed tea = config.parse(formats[i])
    sus format tea = config.detect_format(formats[i])
    vibez.spill("Parsed " + format + " configuration")
}
```

### Configuration Merging and Override

```cursed
yeet "config"

# Base configuration
sus base tea = config.parse("{\"host\": \"localhost\", \"port\": \"8080\", \"debug\": \"false\"}")

# Environment overrides
sus overrides tea = config.parse("{\"port\": \"9090\", \"debug\": \"true\"}")

# Merge configurations (overrides take precedence)
sus final_config tea = config.merge(base, overrides)

# Get final values
sus final_port tea = config.get_value(final_config, "port")
sus debug_mode tea = config.get_value(final_config, "debug")

vibez.spill("Final port: " + final_port)      # "9090"
vibez.spill("Debug mode: " + debug_mode)      # "true"
```

### Dynamic Configuration Updates

```cursed
yeet "config"

# Start with basic configuration
sus app_config tea = config.parse("{\"version\": \"1.0\", \"features\": \"basic\"}")

# Add new configuration values
app_config = config.set_value(app_config, "environment", "production")
app_config = config.set_value(app_config, "logging", "enabled")

# Validate the updated configuration
bestie config.validate(app_config) {
    vibez.spill("Configuration is valid and ready for deployment")
} else {
    vibez.spill("Configuration validation failed")
}
```

## Testing

Run the comprehensive test suite:

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/config/test_config.💀

# Test compilation mode
cargo run --bin cursed -- compile stdlib/config/test_config.💀
./test_config

# Test both modes with verification
test_both_modes stdlib/config/test_config.💀
```

## Error Handling

The config module provides robust error handling:

- **Invalid JSON**: Returns empty string for malformed JSON
- **Missing Keys**: Returns empty string for non-existent keys
- **Format Detection**: Defaults to JSON for unknown formats
- **Environment Variables**: Returns empty string for undefined variables

## Performance Considerations

- **Pure CURSED Implementation**: No FFI overhead
- **String-Based Processing**: Optimized for text configuration formats
- **Minimal Dependencies**: Only requires `json` and `string` modules
- **Efficient Parsing**: Lightweight parsers for each format

## Format-Specific Notes

### JSON
- Full RFC 7159 compliance via json module
- Supports nested objects and arrays
- Handles escape sequences properly

### INI
- Supports sections and key-value pairs
- Handles comments (# and ;)
- Quoted values are properly unquoted

### YAML
- Basic YAML support for simple configurations
- Handles document separators (---)
- Supports quoted and unquoted values

### TOML
- Basic TOML support for key-value pairs
- Handles quoted strings
- Section support (basic implementation)

### ENV
- Standard environment file format
- Supports quoted values (single and double quotes)
- Handles comments starting with #

## Dependencies

- `testz` - For comprehensive testing framework
- `json` - For JSON parsing and validation
- `string` - For string manipulation utilities

## License

This module is part of the CURSED programming language standard library and follows the same licensing terms.
