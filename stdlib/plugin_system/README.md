# Plugin System Module

A comprehensive plugin system for CURSED that enables dynamic loading, management, and execution of plugins with security sandboxing and dependency management.

## Features

- **Dynamic Plugin Loading**: Load and unload plugins at runtime
- **Dependency Management**: Automatic dependency resolution and validation
- **Security Sandboxing**: Restrict plugin access to filesystem and network
- **Hooks System**: Event-driven plugin communication
- **Configuration Management**: Plugin-specific configuration and settings
- **Plugin Discovery**: Automatic discovery of plugins in specified directories
- **Lifecycle Management**: Complete plugin lifecycle from loading to cleanup

## Core Types

### Plugin Structure
```cursed
be_like Plugin squad {
    name tea              // Plugin name
    version tea           // Plugin version
    description tea       // Plugin description
    author tea           // Plugin author
    dependencies [tea]    // Required dependencies
    loaded lit           // Is plugin loaded
    enabled lit          // Is plugin enabled
    entry_point tea      // Main entry file
    exports map[tea]tea  // Plugin exports
    config map[tea]tea   // Plugin configuration
}
```

### Plugin Manager
```cursed
be_like PluginManager squad {
    plugins map[tea]Plugin    // Loaded plugins
    plugin_paths [tea]        // Search paths
    loaded_count normie       // Number of loaded plugins
    enabled_count normie      // Number of enabled plugins
    registry map[tea]tea      // Plugin registry
    hooks map[tea][tea]       // Event hooks
}
```

## Core Functions

### Plugin Manager Operations
```cursed
create_plugin_manager() PluginManager
load_plugin(manager PluginManager, plugin_path tea) PluginManager
unload_plugin(manager PluginManager, plugin_name tea) PluginManager
enable_plugin(manager PluginManager, plugin_name tea) PluginManager
disable_plugin(manager PluginManager, plugin_name tea) PluginManager
```

### Plugin Discovery and Management
```cursed
discover_plugins(manager PluginManager) [tea]
list_plugins(manager PluginManager) [Plugin]
list_enabled_plugins(manager PluginManager) [Plugin]
get_plugin(manager PluginManager, plugin_name tea) Plugin
```

### Plugin Configuration
```cursed
configure_plugin(manager PluginManager, plugin_name tea, config map[tea]tea) PluginManager
check_plugin_permissions(plugin Plugin, permission tea) lit
sandbox_plugin(plugin Plugin) lit
```

### Plugin Communication
```cursed
call_plugin_function(manager PluginManager, plugin_name tea, function_name tea, args map[tea]tea) tea
register_hook(manager PluginManager, hook_name tea, plugin_name tea) PluginManager
fire_hook(manager PluginManager, hook_name tea, data map[tea]tea) [tea]
```

## Usage Examples

### Basic Plugin Management
```cursed
// Create plugin manager
sus manager PluginManager = create_plugin_manager()

// Load a plugin
manager = load_plugin(manager, "./plugins/my_plugin.plugin")

// Enable the plugin
manager = enable_plugin(manager, "my_plugin")

// Configure the plugin
sus config map[tea]tea = {}
config["debug"] = "true"
config["log_level"] = "info"
manager = configure_plugin(manager, "my_plugin", config)
```

### Plugin Discovery
```cursed
// Discover available plugins
sus discovered [tea] = discover_plugins(manager)

// Load all discovered plugins
bestie i := 0; i < len(discovered); i++ {
    manager = load_plugin(manager, discovered[i])
}

// List enabled plugins
sus enabled [Plugin] = list_enabled_plugins(manager)
vibez.spill("Enabled plugins: " + string(len(enabled)))
```

### Hook System
```cursed
// Register plugin for hooks
manager = register_hook(manager, "before_request", "auth_plugin")
manager = register_hook(manager, "after_response", "logging_plugin")

// Fire hooks
sus request_data map[tea]tea = {}
request_data["url"] = "/api/users"
request_data["method"] = "GET"

sus results [tea] = fire_hook(manager, "before_request", request_data)
```

### Plugin Security
```cursed
// Check plugin permissions
sus plugin Plugin = get_plugin(manager, "untrusted_plugin")
vibes check_plugin_permissions(plugin, "filesystem") {
    vibez.spill("Plugin has filesystem access")
} nah {
    vibez.spill("Plugin filesystem access denied")
}

// Apply security sandbox
sandbox_plugin(plugin)
```

## Plugin Manifest Format

Create a `.plugin` file with the following structure:

```json
{
    "name": "my_plugin",
    "version": "1.0.0",
    "description": "My awesome plugin",
    "author": "Developer Name",
    "entry_point": "main.csd",
    "dependencies": ["crypto", "network"],
    "permissions": ["filesystem", "network"],
    "exports": {
        "process_data": "process_data",
        "validate_input": "validate_input"
    }
}
```

## Plugin Development

### Plugin Structure
```
my_plugin/
├── my_plugin.plugin    # Plugin manifest
├── main.csd           # Entry point
├── lib/               # Plugin libraries
├── config/            # Configuration files
└── docs/              # Documentation
```

### Plugin Entry Point
```cursed
// main.csd - Plugin entry point
yeet "plugin_system"

// Plugin initialization
slay plugin_init(config map[tea]tea) lit {
    vibez.spill("Plugin initialized with config")
    damn based
}

// Plugin cleanup
slay plugin_cleanup() lit {
    vibez.spill("Plugin cleanup complete")
    damn based
}

// Exported functions
slay process_data(data tea) tea {
    damn "processed: " + data
}

slay validate_input(input tea) lit {
    damn string_len(input) > 0
}
```

## Security Features

### Permission System
- **filesystem**: Access to file system operations
- **network**: Network communication capabilities
- **admin**: Administrative privileges
- **debug**: Debug and introspection capabilities

### Sandboxing
- Restricted filesystem access
- Network isolation
- Resource limits
- Safe code execution

## Testing

Run the comprehensive test suite:
```bash
cargo run --bin cursed stdlib/plugin_system/test_plugin_system.csd
```

Test both interpretation and compilation modes:
```bash
cargo run --bin cursed stdlib/plugin_system/test_plugin_system.csd
cargo run --bin cursed -- compile stdlib/plugin_system/test_plugin_system.csd
./test_plugin_system
```

## Performance Considerations

- **Plugin Loading**: Minimal overhead for plugin discovery and loading
- **Hook System**: Efficient event dispatching with minimal latency
- **Security Checks**: Fast permission validation with caching
- **Memory Management**: Automatic cleanup of unloaded plugins

## Best Practices

1. **Dependency Management**: Clearly specify plugin dependencies
2. **Security**: Request minimal required permissions
3. **Error Handling**: Implement robust error handling in plugins
4. **Documentation**: Provide comprehensive plugin documentation
5. **Testing**: Test plugins in isolated environments
6. **Performance**: Monitor plugin impact on system performance

## Integration

The plugin system integrates seamlessly with:
- **Error Handling**: Comprehensive error propagation
- **Logging**: Plugin activity logging
- **Configuration**: System-wide configuration management
- **Security**: Application security framework
- **Performance**: System performance monitoring

This plugin system provides a robust foundation for extending CURSED applications with dynamic functionality while maintaining security and performance.
