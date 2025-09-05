# envz Module

The `envz` module provides comprehensive environment variable management with runtime bridge integration for optimal performance and cross-platform compatibility. It offers production-ready environment operations for CURSED applications.

## Features

### Core Environment Operations
- Get, set, and unset environment variables
- Environment variable existence checking
- Type-safe environment variable retrieval (string, integer, boolean)
- Default value support for missing variables

### Advanced Features
- Environment variable expansion in strings (${VAR} and $VAR format)
- PATH environment variable parsing
- Common system directory detection (home, temp)
- Environment type detection (development, production)
- Process environment preparation for spawning

### System Integration
- Cross-platform environment variable access
- Shell and editor detection
- User information retrieval
- Temporary directory resolution

### Runtime Bridge Integration
- High-performance system call integration via Zig runtime
- Pure CURSED fallback implementations for compatibility
- Automatic type conversion and validation
- Comprehensive error handling

## Usage Examples

### Basic Environment Operations
```cursed
yeet "envz"

// Set environment variable
sus set_err tea = envz.set_env("MY_APP_CONFIG", "production")
ready (set_err == "") {
    vibez.spill("Environment variable set successfully")
}

// Get environment variable
(config_value, get_err) := envz.get_env("MY_APP_CONFIG")
ready (get_err == "") {
    vibez.spill("Config:", config_value)
}

// Check if environment variable exists
ready (envz.env_exists("MY_APP_CONFIG")) {
    vibez.spill("Configuration found")
}

// Remove environment variable
sus unset_err tea = envz.unset_env("MY_APP_CONFIG")
```

### Environment Variables with Defaults
```cursed
// Get with string default
sus port tea = envz.get_env_with_default("PORT", "8080")
vibez.spill("Server port:", port)

// Get as integer with default
sus max_connections normie = envz.get_env_as_int("MAX_CONNECTIONS", 100)
vibez.spill("Max connections:", max_connections)

// Get as boolean with default
sus debug_mode lit = envz.get_env_as_bool("DEBUG", cringe)
ready (debug_mode) {
    vibez.spill("Debug mode enabled")
}
```

### Environment Variable Expansion
```cursed
// Expand variables in strings
sus welcome_msg tea = "Welcome ${USER} to your home at ${HOME}"
sus expanded tea = envz.expand_env(welcome_msg)
vibez.spill(expanded)

// Configuration with environment variables
sus db_url tea = "postgresql://${DB_USER}:${DB_PASS}@${DB_HOST}:${DB_PORT}/${DB_NAME}"
sus final_url tea = envz.expand_env(db_url)
```

### System Information
```cursed
// Get user information
(user_name, user_err) := envz.get_user_name()
ready (user_err == "") {
    vibez.spill("Current user:", user_name)
}

// Get home directory
(home_dir, home_err) := envz.get_home_dir()
ready (home_err == "") {
    vibez.spill("Home directory:", home_dir)
}

// Get temporary directory
(temp_dir, temp_err) := envz.get_temp_dir_env()
ready (temp_err == "") {
    vibez.spill("Temp directory:", temp_dir)
}
```

### Development Tools
```cursed
// Get preferred editor
(editor, editor_err) := envz.get_editor()
ready (editor_err == "") {
    vibez.spill("Preferred editor:", editor)
}

// Get shell
(shell, shell_err) := envz.get_shell()
ready (shell_err == "") {
    vibez.spill("Current shell:", shell)
}
```

### PATH Environment Variable
```cursed
// Get PATH as array of directories
sus path_dirs []tea = envz.get_path_env()
sus i normie = 0
bestie (i < len(path_dirs)) {
    vibez.spill("PATH entry:", path_dirs[i])
    i = i + 1
}
```

### Environment Type Detection
```cursed
// Check environment type
ready (envz.is_development_env()) {
    vibez.spill("Running in development mode")
    // Enable debug features
}

ready (envz.is_production_env()) {
    vibez.spill("Running in production mode")
    // Enable performance optimizations
}
```

### Environment Listing
```cursed
// List all environment variables
(env_vars, list_err) := envz.list_env()
ready (list_err == "") {
    sus i normie = 0
    bestie (i < len(env_vars)) {
        vibez.spill("Environment variable:", env_vars[i])
        i = i + 1
    }
}

// Prepare environment for new process
sus process_env []tea = envz.copy_env_to_new_process()
// Use process_env when spawning new processes
```

### Boolean Environment Variables
```cursed
// Common boolean patterns
envz.set_env("ENABLE_LOGGING", "true")
envz.set_env("USE_SSL", "1")
envz.set_env("DEBUG_MODE", "yes")
envz.set_env("PRODUCTION", "on")

sus logging lit = envz.get_env_as_bool("ENABLE_LOGGING", cringe)    // true
sus ssl lit = envz.get_env_as_bool("USE_SSL", cringe)              // true  
sus debug lit = envz.get_env_as_bool("DEBUG_MODE", cringe)         // true
sus prod lit = envz.get_env_as_bool("PRODUCTION", cringe)          // true

// False patterns
envz.set_env("DISABLE_CACHE", "false")
envz.set_env("MAINTENANCE", "0")
envz.set_env("VERBOSE", "no")
envz.set_env("AUTO_UPDATE", "off")

sus cache lit = envz.get_env_as_bool("DISABLE_CACHE", based)       // false
sus maint lit = envz.get_env_as_bool("MAINTENANCE", based)         // false
sus verbose lit = envz.get_env_as_bool("VERBOSE", based)           // false
sus update lit = envz.get_env_as_bool("AUTO_UPDATE", based)        // false
```

### Configuration Management
```cursed
// Application configuration from environment
sus config AppConfig = AppConfig{
    port: envz.get_env_as_int("PORT", 3000),
    host: envz.get_env_with_default("HOST", "localhost"),
    debug: envz.get_env_as_bool("DEBUG", cringe),
    db_url: envz.expand_env("${DATABASE_URL}"),
    log_level: envz.get_env_with_default("LOG_LEVEL", "info"),
    max_connections: envz.get_env_as_int("MAX_CONNECTIONS", 100)
}
```

## Function Reference

### Core Operations
- `get_env(name)` - Get environment variable value
- `set_env(name, value)` - Set environment variable  
- `unset_env(name)` - Remove environment variable
- `env_exists(name)` - Check if environment variable exists
- `list_env()` - List all environment variables

### Typed Retrieval
- `get_env_with_default(name, default_value)` - Get with string default
- `get_env_as_int(name, default_value)` - Get as integer with default
- `get_env_as_bool(name, default_value)` - Get as boolean with default

### String Operations
- `expand_env(text)` - Expand environment variables in text

### System Information
- `get_home_dir()` - Get user home directory
- `get_temp_dir_env()` - Get temporary directory
- `get_user_name()` - Get current user name
- `get_shell()` - Get user shell
- `get_editor()` - Get preferred editor

### PATH Operations
- `get_path_env()` - Get PATH as array of directories

### Environment Detection
- `is_development_env()` - Check if in development environment
- `is_production_env()` - Check if in production environment

### Process Operations
- `copy_env_to_new_process()` - Get environment for process spawning
- `clear_env()` - Clear all environment variables (dangerous)

## Environment Variable Types

### Boolean Values
The `get_env_as_bool()` function recognizes these values:

**True values**: "true", "1", "yes", "on" (case-insensitive)
**False values**: "false", "0", "no", "off" (case-insensitive)

### Integer Parsing
The `get_env_as_int()` function parses decimal integers and returns the default value if parsing fails.

### String Expansion
The `expand_env()` function supports these formats:
- `${VAR}` - Standard shell expansion
- `$VAR` - Simple variable expansion

## Error Handling

All environment operations return error strings. An empty string indicates success, while any non-empty string indicates an error occurred.

### Common Error Patterns
```cursed
// Check for errors in environment operations
(value, err) := envz.get_env("CONFIG_VAR")
ready (err != "") {
    vibez.spill("Error getting environment variable:", err)
    damn // Exit early
}

// Use value safely
vibez.spill("Configuration value:", value)
```

### Error Types
- **Empty name errors**: "Empty environment variable name not allowed"
- **System errors**: "Failed to get environment variable: VAR - system error"
- **Permission errors**: Platform-specific permission errors

## System Environment Variables

### Common Variables
- `HOME` / `USERPROFILE` - User home directory
- `USER` / `USERNAME` - Current user name
- `SHELL` - User shell (Unix-like systems)
- `EDITOR` / `VISUAL` - Preferred text editor
- `PATH` - Executable search path
- `TMPDIR` / `TMP` / `TEMP` / `TEMPDIR` - Temporary directory

### Development Environment Detection
The module checks these variables for environment type:
- `NODE_ENV` - Node.js environment
- `RAILS_ENV` - Ruby on Rails environment  
- `ENVIRONMENT` - Generic environment variable

### Production Environment Detection
Recognizes these values as production:
- "production", "prod" (case-insensitive)

### Development Environment Detection  
Recognizes these values as development:
- "development", "dev" (case-insensitive)

## Cross-Platform Compatibility

### Windows Support
- Uses `USERPROFILE` for home directory
- Uses `USERNAME` for user name
- Handles Windows-style path separators
- Supports Windows environment variable expansion

### Unix-like Systems
- Uses `HOME` for home directory
- Uses `USER` for user name  
- Uses `SHELL` for shell detection
- Supports Unix-style path handling

### Path Handling
- Automatic path separator detection
- Cross-platform PATH parsing
- Unicode environment variable support

## Implementation Notes

### Runtime Bridge Pattern
The envz module uses a runtime bridge pattern where critical functions are implemented in the Zig runtime for optimal performance, with pure CURSED fallbacks.

**Runtime Bridge Functions:**
- `runtime_get_env()` - System environment variable retrieval
- `runtime_set_env()` - System environment variable setting
- `runtime_unset_env()` - Environment variable removal
- `runtime_list_env()` - Environment variable enumeration
- `runtime_expand_env()` - Variable expansion in strings
- String utility functions for parsing and conversion

### Performance Optimization
- Minimal system call overhead
- Efficient string processing
- Cached environment variable access where possible
- Optimized boolean and integer parsing

### Security Considerations
- Safe environment variable name validation
- Proper handling of sensitive environment variables
- Secure expansion to prevent injection attacks
- Limited access to system-sensitive variables

## Testing
Run the comprehensive test suite:
```bash
./zig-out/bin/cursed stdlib/envz/test_envz.💀
```

The test suite covers:
- Basic environment variable operations
- Type conversion and default values  
- Boolean environment variable parsing
- Environment variable expansion
- System information retrieval
- PATH environment variable handling
- Environment type detection
- Error handling scenarios

## Best Practices

### Environment Variable Naming
- Use uppercase names for environment variables
- Use underscores to separate words
- Prefix application-specific variables with app name
- Keep names descriptive but concise

### Configuration Management
- Use environment variables for configuration that varies by environment
- Provide sensible defaults for optional configuration
- Validate environment variable values at startup
- Document required environment variables

### Security
- Never log sensitive environment variables
- Use separate variables for different secret types
- Rotate secrets regularly
- Avoid storing secrets in version control

### Error Handling
- Always check error returns from environment operations
- Provide meaningful error messages to users
- Handle missing environment variables gracefully
- Log environment variable access for debugging

### Performance
- Cache frequently accessed environment variables
- Use typed retrieval functions to avoid repeated parsing
- Minimize environment variable modifications
- Consider using configuration files for complex settings

### Cross-Platform Support
- Test environment variable handling on all target platforms
- Use cross-platform detection functions when available
- Handle platform-specific environment variables appropriately
- Provide fallbacks for missing platform features
