# Environment Variables (env)

The `env` module provides access to environment variables and system configuration for CURSED programs.

## Purpose

This module implements cross-platform environment variable access, system configuration retrieval, and environment manipulation for CURSED applications.

## Main Functions

### Environment Variable Access
- `env.get(name)` - Get environment variable value
- `env.get_or_default(name, default)` - Get with default value
- `env.set(name, value)` - Set environment variable
- `env.unset(name)` - Remove environment variable
- `env.has(name)` - Check if variable exists
- `env.list_all()` - Get all environment variables

### System Information
- `env.platform()` - Get platform name (linux, darwin, windows)
- `env.architecture()` - Get CPU architecture (x86_64, arm64, etc.)
- `env.home_dir()` - Get user home directory
- `env.temp_dir()` - Get temporary directory
- `env.current_dir()` - Get current working directory
- `env.executable_path()` - Get current executable path

### Path Operations
- `env.path_separator()` - Get path separator for platform
- `env.search_path()` - Get PATH environment variable as array
- `env.which(command)` - Find executable in PATH
- `env.expand_path(path)` - Expand ~ and environment variables

## Usage Examples

### Basic Environment Variable Access

```cursed
yeet "env"

fr fr Get common environment variables
sus user = env.get("USER")
if user.is_some() {
    vibez.spillf("Current user: {}", user.unwrap())
} else {
    vibez.spill("USER not set")
}

fr fr Get with default value
sus shell = env.get_or_default("SHELL", "/bin/sh")
vibez.spillf("Shell: {}", shell)

fr fr Check if variable exists
if env.has("PATH") {
    vibez.spill("PATH is set")
    sus path_dirs = env.search_path()
    vibez.spillf("PATH has {} directories", path_dirs.len())
}
```

### System Information

```cursed
yeet "env"

vibez.spillf("Platform: {}", env.platform())
vibez.spillf("Architecture: {}", env.architecture())
vibez.spillf("Home directory: {}", env.home_dir())
vibez.spillf("Temp directory: {}", env.temp_dir())
vibez.spillf("Current directory: {}", env.current_dir())
vibez.spillf("Executable: {}", env.executable_path())
```

### Setting and Managing Variables

```cursed
yeet "env"

fr fr Set custom environment variables
env.set("CURSED_APP_MODE", "production")
env.set("CURSED_LOG_LEVEL", "info")

fr fr Use the variables
sus mode = env.get_or_default("CURSED_APP_MODE", "development")
sus log_level = env.get_or_default("CURSED_LOG_LEVEL", "debug")

vibez.spillf("Running in {} mode with {} logging", mode, log_level)

fr fr Clean up
env.unset("CURSED_APP_MODE")
env.unset("CURSED_LOG_LEVEL")
```

### Configuration Management

```cursed
yeet "env"

squad AppConfig {
    spill database_url tea
    spill port normie
    spill debug_mode lit
    spill log_level tea
}

slay load_config() AppConfig {
    sus db_url = env.get_or_default("DATABASE_URL", "sqlite:///default.db")
    sus port_str = env.get_or_default("PORT", "8080")
    sus port = port_str.to_int().unwrap_or(8080)
    sus debug = env.get_or_default("DEBUG", "false") == "true"
    sus log_level = env.get_or_default("LOG_LEVEL", "info")
    
    damn AppConfig{
        database_url: db_url,
        port: port,
        debug_mode: debug,
        log_level: log_level
    }
}

sus config = load_config()
vibez.spillf("Config: DB={}, Port={}, Debug={}", 
    config.database_url, config.port, config.debug_mode)
```

### Finding Executables

```cursed
yeet "env"

sus executables []tea = ["git", "node", "python3", "cargo"]

bestie exe in executables {
    sus path = env.which(exe)
    if path.is_some() {
        vibez.spillf("{}: {}", exe, path.unwrap())
    } else {
        vibez.spillf("{}: not found", exe)
    }
}
```

### Path Expansion and Manipulation

```cursed
yeet "env"

sus paths []tea = [
    "~/Documents",
    "$HOME/.config",
    "/tmp",
    "."
]

bestie path in paths {
    sus expanded = env.expand_path(path)
    vibez.spillf("{} -> {}", path, expanded)
}

fr fr Platform-specific path handling
sus separator = env.path_separator()
sus parts []tea = ["home", "user", "projects"]
sus full_path = parts.join(separator)
vibez.spillf("Path: {}", full_path)
```

### Environment Variable Listing

```cursed
yeet "env"

sus all_vars = env.list_all()
vibez.spillf("Found {} environment variables", all_vars.len())

fr fr Show some interesting ones
sus interesting []tea = ["HOME", "PATH", "USER", "SHELL", "TERM"]
bestie var in interesting {
    if all_vars.contains_key(var) {
        sus value = all_vars[var]
        if value.len() > 50 {
            vibez.spillf("{}: {}...", var, value.slice(0, 47))
        } else {
            vibez.spillf("{}: {}", var, value)
        }
    }
}
```

## Compilation Examples

### Interpretation Mode
```bash
echo 'yeet "env"
vibez.spillf("Platform: {}", env.platform())
vibez.spillf("Home: {}", env.home_dir())' > env_test.csd

./cursed-unified env_test.csd
```

### Compilation Mode
```bash
./cursed-unified --compile env_test.csd
./env_test
```

## Cross-Platform Considerations

### Platform Detection

```cursed
yeet "env"

match env.platform() {
    "linux" => {
        vibez.spill("Running on Linux")
        sus distribution = env.get_or_default("LSB_RELEASE", "unknown")
        vibez.spillf("Distribution info: {}", distribution)
    },
    "darwin" => {
        vibez.spill("Running on macOS")
        sus version = env.get_or_default("OSTYPE", "unknown")
        vibez.spillf("OS type: {}", version)
    },
    "windows" => {
        vibez.spill("Running on Windows")
        sus username = env.get_or_default("USERNAME", "unknown")
        vibez.spillf("Username: {}", username)
    },
    _ => vibez.spill("Unknown platform")
}
```

### Path Handling

```cursed
yeet "env"

slay build_config_path(filename tea) tea {
    sus home = env.home_dir()
    sus separator = env.path_separator()
    
    match env.platform() {
        "windows" => {
            sus appdata = env.get_or_default("APPDATA", home)
            damn appdata + separator + "MyApp" + separator + filename
        },
        _ => {
            damn home + separator + ".config" + separator + "myapp" + separator + filename
        }
    }
}

sus config_file = build_config_path("settings.json")
vibez.spillf("Config file path: {}", config_file)
```

## Implementation Notes

- Cross-platform environment variable access
- Secure handling of sensitive environment data
- Thread-safe operations
- Proper Unicode support for paths
- Pure CURSED implementation

## Dependencies

- `string_simple` - For string operations
- `path` - For path manipulation
- Core system integration
- No external dependencies

## Security Considerations

1. **Sanitize environment variables** before use
2. **Don't log sensitive values** like passwords or tokens
3. **Validate paths** to prevent directory traversal
4. **Use secure defaults** for missing variables
5. **Clear sensitive variables** after use

## Best Practices

1. **Use defaults** for missing environment variables
2. **Validate configuration** values after loading
3. **Document required** environment variables
4. **Use typed configuration** structures
5. **Handle missing variables** gracefully
6. **Use platform-specific** paths appropriately
7. **Cache expensive lookups** like `which()` calls

## Common Patterns

### Configuration Loading
```cursed
slay load_database_config() DatabaseConfig {
    sus url = env.get("DATABASE_URL")
    if !url.is_some() {
        panic_system.panic("DATABASE_URL environment variable required")
    }
    damn parse_database_url(url.unwrap())
}
```

### Development vs Production
```cursed
slay is_development() lit {
    sus env_mode = env.get_or_default("NODE_ENV", "development")
    damn env_mode == "development"
}
```
