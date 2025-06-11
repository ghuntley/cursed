# CURSED Environment Variables Module

## Overview

The CURSED environment variables module provides comprehensive environment variable operations with cross-platform support, type safety, and robust error handling. This module follows the established patterns of other CURSED standard library modules and integrates seamlessly with the existing error handling system.

## Features

### Core Operations
- **Basic Environment Variables**: Get, set, remove, and check existence of environment variables
- **Type Safety**: Parse environment variables to specific types with validation
- **Cross-Platform Support**: Works consistently on Windows, macOS, and Linux
- **Unicode Support**: Full UTF-8 handling for environment variable names and values

### Advanced Functionality
- **Environment Variable Expansion**: Support for `${VAR}` and `$VAR` syntax with defaults and conditionals
- **Configuration Parsing**: Parse complex configuration formats from environment variables
- **List Processing**: Handle PATH-like, comma-separated, and custom-delimited lists
- **Duration and Memory Size Parsing**: Parse human-readable time and memory specifications
- **Error Handling**: Comprehensive error types with meaningful messages

### Platform Utilities
- **System Information**: Get home directory, temp directory, username, hostname
- **Path Handling**: Platform-appropriate path separators and PATH parsing
- **Case Sensitivity**: Handle platform-specific case sensitivity rules

## Module Structure

```
src/stdlib/env/
├── mod.rs           # Public API and re-exports
├── error.rs         # Error handling and types
├── core.rs          # Basic environment variable operations
├── parsing.rs       # Type parsing and conversion
└── expansion.rs     # Variable expansion and substitution
```

## Core Functions

### Basic Operations

```cursed
import "stdlib::env";

// Get environment variable
let value = get_env("HOME");                    // Option<String>
let path = get_env_with_default("PATH", "");    // String with default

// Set and remove
set_env("MY_VAR", "value")?;                    // Result<(), EnvError>
remove_env("MY_VAR")?;                          // Result<(), EnvError>

// Check existence
if env_exists("DEBUG") {
    println("Debug mode enabled");
}

// Get all variables
let all_vars = get_all_env();                   // HashMap<String, String>
```

### Type Parsing

```cursed
// Parse as specific types
let port: u16 = parse_env("PORT")?;             // Parse with error handling
let debug: bool = get_bool_env("DEBUG")?;       // Boolean parsing
let timeout: f64 = get_float_env("TIMEOUT")?;   // Float parsing

// Parse with defaults
let workers = parse_env_with_default("WORKERS", 4u32)?;

// Numeric bounds checking
let percent = get_numeric_env("CPU_LIMIT", 0.0, 100.0)?;
```

### List Processing

```cursed
// Parse different list formats
let hosts = parse_env_list("ALLOWED_HOSTS")?;           // Comma-separated
let paths = parse_env_colon_list("LD_LIBRARY_PATH")?;   // Colon-separated
let search = parse_env_semicolon_list("SEARCH_PATH")?;  // Semicolon-separated

// Platform-appropriate parsing
let path_entries = get_path_env("PATH");                 // Vec<PathBuf>
let platform_list = parse_env_path_list("LIBRARY_PATH")?;
```

### Duration and Memory Parsing

```cursed
// Duration parsing with units
let cache_ttl = parse_env_duration("CACHE_TTL")?;       // "5m", "2h", "30s"
let session_timeout = parse_env_duration("TIMEOUT")?;   // "1d", "12h"

// Memory size parsing
let heap_size = parse_env_memory_size("MAX_HEAP")?;     // "512MB", "2GB"
let buffer_size = parse_env_memory_size("BUFFER")?;     // "64KB", "1024B"
```

### Configuration Parsing

```cursed
// Key-value configuration
set_env("DB_CONFIG", "host=localhost,port=5432,ssl=true")?;
let config = parse_env_config("DB_CONFIG", ",")?;       // HashMap<String, String>

// Access parsed values
let host = config.get("host").unwrap();                 // "localhost"
let port = config.get("port").unwrap().parse::<u16>()?; // 5432
```

## Environment Variable Expansion

### Basic Expansion

```cursed
set_env("USER", "alice")?;
set_env("HOME", "/home/alice")?;

// Simple variable expansion
let greeting = expand_env_vars("Hello $USER")?;
// Result: "Hello alice"

// Braced variables
let path = expand_env_vars("Config: ${HOME}/.config")?;
// Result: "Config: /home/alice/.config"
```

### Advanced Expansion

```cursed
// Default values
let db_url = expand_env_vars("${DATABASE_URL:-sqlite:///app.db}")?;
// Uses default if DATABASE_URL is not set

// Conditional expansion
let debug_flag = expand_env_vars("${DEBUG:+--debug}")?;
// Returns "--debug" if DEBUG is set, empty otherwise

// Complex templates
let config_path = expand_env_vars("${CONFIG_DIR:-${HOME}/.config}/${APP_NAME:-myapp}/config.toml")?;
```

### Custom Expansion

```cursed
// Use custom default values
let mut defaults = HashMap::new();
defaults.insert("APP_NAME".to_string(), "MyApp".to_string());
defaults.insert("VERSION".to_string(), "1.0.0".to_string());

let result = expand_env_vars_with_defaults("${APP_NAME} v${VERSION}", &defaults)?;

// Direct substitution
let mut substitutions = HashMap::new();
substitutions.insert("SERVICE".to_string(), "web-api".to_string());
let template = "Deploying ${SERVICE} to production";
let result = substitute_env_vars(template, &substitutions)?;
```

## Platform-Specific Features

### Cross-Platform Information

```cursed
// Platform-specific path separator
let separator = get_path_separator();    // ":" on Unix, ";" on Windows

// Case sensitivity
let is_case_sensitive = is_case_sensitive_env(); // false on Windows, true on Unix

// Case-insensitive lookup (Windows)
let path = get_env_case_insensitive("path");     // Finds "PATH" on Windows
```

### System Directories

```cursed
// Common system directories
if let Some(home) = get_home_dir() {
    println!("Home: {}", home);
}

if let Some(temp) = get_temp_dir() {
    println!("Temp: {}", temp);
}

if let Some(current) = get_current_dir() {
    println!("Current: {}", current);
}

// User information
if let Some(user) = get_username() {
    println!("User: {}", user);
}

if let Some(hostname) = get_hostname() {
    println!("Host: {}", hostname);
}
```

## Error Handling

The module provides comprehensive error handling with the `EnvError` enum:

```cursed
use stdlib::env::{EnvError, EnvResult};

// Handle specific error types
match parse_env::<i32>("INVALID_NUMBER") {
    Ok(value) => println!("Got: {}", value),
    Err(EnvError::NotFound { key, .. }) => {
        println!("Variable '{}' not found", key);
    }
    Err(EnvError::InvalidValue { key, value, expected_type, .. }) => {
        println!("Invalid {} for '{}': '{}'", expected_type, key, value);
    }
    Err(err) => println!("Error: {}", err),
}
```

### Error Types

- `NotFound`: Environment variable doesn't exist
- `InvalidValue`: Value cannot be parsed to expected type
- `PermissionDenied`: Insufficient permissions for operation
- `InvalidKey`: Invalid key name (empty, contains null bytes)
- `SystemError`: System-level error during operation
- `UnicodeError`: Invalid UTF-8 sequence
- `ExpansionError`: Error during variable expansion
- `General`: General environment error

## Best Practices

### Configuration Management

```cursed
// Use a configuration structure
struct AppConfig {
    database_url: String,
    redis_url: String,
    log_level: String,
    port: u16,
    workers: u32,
    debug: bool,
}

fn load_config() -> Result<AppConfig, EnvError> {
    Ok(AppConfig {
        database_url: expand_env_vars("${DATABASE_URL:-sqlite:///app.db}")?,
        redis_url: get_env_with_default("REDIS_URL", "redis://localhost:6379"),
        log_level: get_env_with_default("LOG_LEVEL", "info"),
        port: get_numeric_env("PORT", 1024, 65535).unwrap_or(8080),
        workers: get_numeric_env("WORKERS", 1, 32).unwrap_or(4),
        debug: get_bool_env("DEBUG").unwrap_or(false),
    })
}
```

### Error Handling Patterns

```cursed
// Use defaults for optional configuration
let cache_enabled = get_bool_env("CACHE_ENABLED").unwrap_or(true);

// Handle required configuration with meaningful errors
let api_key = get_env("API_KEY")
    .ok_or_else(|| env_error("API_KEY is required for production"))?;

// Validate configuration
if let Some(invalid_url) = get_env("DATABASE_URL") {
    if !invalid_url.starts_with("postgresql://") {
        return Err(invalid_value_error("DATABASE_URL", &invalid_url, 
                                     "PostgreSQL URL", "Must start with postgresql://"));
    }
}
```

### Environment Variable Naming

```cursed
// Use consistent naming conventions
const APP_PREFIX: &str = "MYAPP";

fn get_app_env(key: &str) -> Option<String> {
    get_env(&format!("{}_{}", APP_PREFIX, key))
}

// Usage
let database_url = get_app_env("DATABASE_URL");  // Looks for MYAPP_DATABASE_URL
let log_level = get_app_env("LOG_LEVEL");        // Looks for MYAPP_LOG_LEVEL
```

## Integration with Other Modules

### File System Integration

```cursed
use stdlib::{env, fs};

// Use environment for file paths
let config_dir = env::get_env_with_default("CONFIG_DIR", 
    &format!("{}/.config/myapp", env::get_home_dir().unwrap_or_default()));

if !fs::exists(&config_dir)? {
    fs::create_dir_all(&config_dir)?;
}

let config_path = fs::join_path(&config_dir, "config.toml");
```

### Logging Integration

```cursed
use stdlib::{env, io};

// Configure logging based on environment
let log_level = env::get_env_with_default("LOG_LEVEL", "info");
let log_file = env::expand_env_vars("${LOG_DIR:-./logs}/app.log")?;

match log_level.as_str() {
    "debug" => io::println("Debug logging enabled")?,
    "info" => io::println("Info logging enabled")?,
    "warn" => io::println("Warn logging enabled")?,
    "error" => io::println("Error logging only")?,
    _ => return Err(env::invalid_value_error("LOG_LEVEL", &log_level, 
                                          "log level", "Must be debug, info, warn, or error")),
}
```

## Examples

The module includes comprehensive example programs:

- **`examples/env_basic_usage.csd`**: Basic environment variable operations
- **`examples/env_advanced_parsing.csd`**: Advanced parsing and configuration
- **`examples/env_expansion_demo.csd`**: Variable expansion features
- **`examples/env_config_management.csd`**: Real-world configuration management

## Testing

Run the comprehensive test suite:

```bash
# Run all environment variable tests
cargo test --test env_comprehensive_test

# Run with linking fixes for Nix environments
./fix_linking.sh cargo test --test env_comprehensive_test
```

The test suite covers:
- All core environment variable operations
- Type parsing and validation
- Error handling scenarios
- Platform-specific behavior
- Environment variable expansion
- Configuration parsing
- Edge cases and boundary conditions

## Performance Characteristics

- **Memory Efficient**: Minimal allocations, uses standard library optimizations
- **Thread Safe**: All operations are thread-safe using appropriate synchronization
- **Cross-Platform**: Consistent performance across operating systems
- **Error Recovery**: Graceful handling of system errors and edge cases
- **Unicode Safe**: Full UTF-8 support without performance penalties

## Future Enhancements

Potential future improvements:
- **Nested Variable Expansion**: Support `${VAR:+${OTHER_VAR}}` syntax
- **Environment File Loading**: Load variables from `.env` files
- **Template Engine Integration**: Direct integration with template rendering
- **Validation Rules**: Custom validation rules for environment variables
- **Hot Reloading**: Watch for environment variable changes

## Summary

The CURSED environment variables module provides a comprehensive, type-safe, and cross-platform solution for environment variable management. It follows established CURSED patterns, integrates seamlessly with other standard library modules, and provides excellent error handling and documentation.

Key benefits:
- **Type Safety**: Parse environment variables to any type with validation
- **Cross-Platform**: Works consistently across Windows, macOS, and Linux  
- **Rich Functionality**: Advanced parsing, expansion, and configuration features
- **Error Handling**: Comprehensive error types with meaningful messages
- **Performance**: Optimized implementations with minimal overhead
- **Documentation**: Extensive examples and documentation
- **Testing**: Comprehensive test coverage including edge cases

This module enables robust configuration management and environment-based application setup in CURSED programs.
