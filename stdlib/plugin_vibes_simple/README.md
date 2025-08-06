# plugin_vibes_simple

Simple plugin system for CURSED applications providing dynamic loading, management, and extensibility. Enables modular architecture with runtime plugin discovery and lifecycle management.

## Overview

The `plugin_vibes_simple` module provides:
- Basic plugin loading and unloading
- Plugin registry management
- Simple plugin discovery
- Lifecycle management for loaded plugins
- Foundation for extensible applications

## Core Functions

### Plugin Management

#### `load_plugin(plugin_name: tea) -> lit`
Loads a plugin by name with basic validation.

**Parameters:**
- `plugin_name`: Name of the plugin to load

**Returns:** `based` if loaded successfully, `cringe` if failed

**Supported Plugins:**
- `"auth"`: Authentication plugin
- `"logger"`: Logging plugin

**Example:**
```cursed
yeet "plugin_vibes_simple"

// Load authentication plugin
lowkey load_plugin("auth") {
    vibez.spill("✅ Authentication plugin loaded")
} yikes {
    vibez.spill("❌ Failed to load authentication plugin")
}

// Load logging plugin
lowkey load_plugin("logger") {
    vibez.spill("✅ Logging plugin loaded")
}
```

#### `list_loaded_plugins() -> tea`
Returns a list of currently loaded plugins.

**Returns:** String containing loaded plugin names

**Example:**
```cursed
// Check loaded plugins
sus loaded tea = list_loaded_plugins()
vibez.spill("Currently loaded: " + loaded)
```

#### `unload_plugin(plugin_name: tea) -> lit`
Unloads a specified plugin and cleans up resources.

**Parameters:**
- `plugin_name`: Name of the plugin to unload

**Returns:** `based` if unloaded successfully

**Example:**
```cursed
// Unload a plugin
lowkey unload_plugin("auth") {
    vibez.spill("✅ Authentication plugin unloaded")
}
```

## Usage Examples

### Basic Plugin Workflow

```cursed
yeet "plugin_vibes_simple"

// Application startup - load essential plugins
slay initialize_plugins() {
    vibez.spill("🔌 Initializing plugin system...")
    
    // Load core plugins
    sus plugins []tea = []tea{"auth", "logger"}
    
    bestie i := 0; i < len(plugins); i = i + 1 {
        sus plugin_name tea = plugins[i]
        lowkey load_plugin(plugin_name) {
            vibez.spill("Loaded plugin: " + plugin_name)
        } yikes {
            vibez.spill("Failed to load plugin: " + plugin_name)
        }
    }
    
    // Display loaded plugins
    sus loaded tea = list_loaded_plugins()
    vibez.spill("Active plugins: " + loaded)
}

// Application shutdown - cleanup plugins
slay cleanup_plugins() {
    vibez.spill("🧹 Cleaning up plugins...")
    
    sus active_plugins tea = list_loaded_plugins()
    lowkey string_length(active_plugins) > 0 {
        unload_plugin("auth")
        unload_plugin("logger")
    }
    
    vibez.spill("Plugin cleanup completed")
}
```

### Plugin-Based Architecture

```cursed
// Plugin-aware application structure
slay plugin_aware_application() {
    // Initialize core system
    initialize_application()
    
    // Load plugins based on configuration
    load_plugins_from_config()
    
    // Main application loop
    bestie application_running {
        // Use plugin functionality
        authenticate_user_via_plugin()
        log_activity_via_plugin()
        
        // Process business logic
        handle_user_requests()
    }
    
    // Shutdown with plugin cleanup
    cleanup_plugins()
    shutdown_application()
}

slay load_plugins_from_config() {
    // Read plugin configuration
    sus config []tea = read_plugin_config()
    
    bestie i := 0; i < len(config); i = i + 1 {
        sus plugin_name tea = config[i]
        load_plugin(plugin_name)
    }
}
```

### Conditional Plugin Loading

```cursed
// Load plugins based on runtime conditions
slay conditional_plugin_loading() {
    // Load auth plugin only if authentication is required
    lowkey requires_authentication() {
        lowkey load_plugin("auth") {
            vibez.spill("Authentication enabled")
        } yikes {
            vibez.spill("Warning: Could not enable authentication")
        }
    }
    
    // Load logger plugin only in debug mode
    lowkey is_debug_mode() {
        load_plugin("logger")
    }
    
    // Check what's actually loaded
    sus loaded tea = list_loaded_plugins()
    vibez.spill("Runtime configuration: " + loaded)
}
```

## Plugin Development Framework

### Plugin Interface (Future Enhancement)

```cursed
// Framework for plugin development (planned)
collab Plugin {
    slay initialize() -> lit
    slay get_name() -> tea
    slay get_version() -> tea
    slay shutdown() -> lit
}

// Authentication plugin implementation (example)
squad AuthPlugin {
    spill name tea = "auth"
    spill version tea = "1.0.0"
    spill initialized lit = cringe
}

slay (p *AuthPlugin) initialize() -> lit {
    p.initialized = based
    vibez.spill("Auth plugin initialized")
    damn based
}

slay (p *AuthPlugin) get_name() -> tea {
    damn p.name
}

slay (p *AuthPlugin) shutdown() -> lit {
    p.initialized = cringe
    vibez.spill("Auth plugin shutdown")
    damn based
}
```

### Plugin Registry (Enhanced Version)

```cursed
// Enhanced plugin registry system (future)
squad PluginRegistry {
    spill plugins map[tea]Plugin
    spill load_order []tea
    spill plugin_paths []tea
}

slay registry_add_plugin_path(registry PluginRegistry, path tea) PluginRegistry {
    registry.plugin_paths = append_string(registry.plugin_paths, path)
    damn registry
}

slay registry_discover_plugins(registry PluginRegistry) []tea {
    // Scan plugin paths for available plugins
    sus available_plugins []tea = []tea{}
    
    bestie i := 0; i < len(registry.plugin_paths); i = i + 1 {
        sus path tea = registry.plugin_paths[i]
        sus found_plugins []tea = scan_directory_for_plugins(path)
        available_plugins = merge_string_arrays(available_plugins, found_plugins)
    }
    
    damn available_plugins
}
```

## Configuration and Discovery

### Plugin Configuration

```cursed
// Plugin configuration management
squad PluginConfig {
    spill enabled_plugins []tea
    spill plugin_settings map[tea]map[tea]tea
    spill auto_load lit
}

slay read_plugin_config() []tea {
    // Simple configuration for now
    sus default_plugins []tea = []tea{"auth", "logger"}
    damn default_plugins
}

slay save_plugin_config(plugins []tea) lit {
    // Save plugin configuration to file
    vibez.spill("Saving plugin configuration...")
    damn based
}
```

### Plugin Discovery

```cursed
// Plugin discovery mechanisms
slay discover_available_plugins() []tea {
    // Discover plugins in standard locations
    sus plugin_dirs []tea = []tea{
        "plugins/",
        "lib/plugins/",
        "/usr/local/lib/cursed/plugins/"
    }
    
    sus available []tea = []tea{}
    
    bestie i := 0; i < len(plugin_dirs); i = i + 1 {
        sus dir tea = plugin_dirs[i]
        lowkey directory_exists(dir) {
            sus found []tea = scan_plugin_directory(dir)
            available = merge_arrays(available, found)
        }
    }
    
    damn available
}

slay scan_plugin_directory(dir tea) []tea {
    // Scan directory for plugin files
    sus plugins []tea = []tea{}
    
    // Look for .csd files with plugin metadata
    sus files []tea = list_directory_files(dir)
    bestie i := 0; i < len(files); i = i + 1 {
        sus file tea = files[i]
        lowkey is_plugin_file(file) {
            sus plugin_name tea = extract_plugin_name(file)
            plugins = append_string(plugins, plugin_name)
        }
    }
    
    damn plugins
}
```

## Error Handling and Validation

### Plugin Validation

```cursed
// Plugin validation before loading
slay validate_plugin(plugin_name tea) lit {
    // Check plugin name format
    lowkey string_length(plugin_name) == 0 {
        damn cringe
    }
    
    // Check if plugin name is valid
    lowkey contains_invalid_characters(plugin_name) {
        damn cringe
    }
    
    // Check if plugin file exists (future enhancement)
    lowkey !plugin_file_exists(plugin_name) {
        damn cringe
    }
    
    damn based
}

slay contains_invalid_characters(name tea) lit {
    // Check for invalid characters in plugin name
    bestie i := 0; i < string_length(name); i = i + 1 {
        sus char normie = char_at(name, i)
        lowkey !is_alphanumeric(char) && char != '_' && char != '-' {
            damn based
        }
    }
    damn cringe
}
```

### Error Recovery

```cursed
// Plugin error handling and recovery
slay safe_load_plugin(plugin_name tea) (lit, tea) {
    // Validate plugin before loading
    lowkey !validate_plugin(plugin_name) {
        damn cringe, "Invalid plugin name: " + plugin_name
    }
    
    // Attempt to load plugin
    lowkey load_plugin(plugin_name) {
        damn based, ""
    } yikes {
        damn cringe, "Failed to load plugin: " + plugin_name
    }
}

slay recover_from_plugin_error(plugin_name tea, error tea) {
    vibez.spill("Plugin error: " + plugin_name + " - " + error)
    
    // Attempt to unload problematic plugin
    unload_plugin(plugin_name)
    
    // Log error for debugging
    log_plugin_error(plugin_name, error)
    
    // Continue operation without the plugin
    vibez.spill("Continuing without plugin: " + plugin_name)
}
```

## Testing

### Plugin System Tests

```cursed
// Test plugin loading functionality
slay test_plugin_loading() {
    test_start("Plugin Loading")
    
    // Test valid plugin loading
    assert_true(load_plugin("auth"))
    assert_true(load_plugin("logger"))
    
    // Test plugin listing
    sus loaded tea = list_loaded_plugins()
    assert_true(string_length(loaded) > 0)
    
    // Test plugin unloading
    assert_true(unload_plugin("auth"))
    assert_true(unload_plugin("logger"))
    
    print_test_summary()
}

// Test error conditions
slay test_plugin_errors() {
    test_start("Plugin Error Handling")
    
    // Test invalid plugin names
    assert_false(load_plugin(""))
    assert_false(load_plugin("nonexistent"))
    
    // Test double loading (should be idempotent)
    load_plugin("auth")
    assert_true(load_plugin("auth"))  // Should succeed
    
    print_test_summary()
}
```

### Integration Testing

```bash
# Run plugin system tests
zig build test
./zig-out/bin/cursed-zig stdlib/plugin_vibes_simple/test_plugin_vibes_simple.csd
```

## Future Enhancements

### Advanced Plugin Features

1. **Dynamic Loading**: Runtime plugin discovery and loading
2. **Plugin Dependencies**: Dependency resolution between plugins
3. **Plugin Versioning**: Version compatibility checking
4. **Plugin Isolation**: Sandboxing for plugin security
5. **Plugin Communication**: Inter-plugin messaging system

### Enhanced Architecture

```cursed
// Future plugin architecture (planned)
squad AdvancedPluginSystem {
    spill registry PluginRegistry
    spill dependency_resolver DependencyResolver
    spill security_manager SecurityManager
    spill communication_bus MessageBus
}

slay advanced_load_plugin(name tea, version tea) PluginLoadResult {
    // Check dependencies
    // Validate security permissions
    // Load with isolation
    // Register with message bus
    damn result
}
```

### Plugin Marketplace Integration

```cursed
// Plugin marketplace integration (future)
slay discover_marketplace_plugins() []PluginInfo {
    // Query remote plugin repository
    // Check compatibility with current system
    // Return available plugins with metadata
    damn plugins
}

slay install_plugin_from_marketplace(plugin_id tea) lit {
    // Download plugin package
    // Verify digital signature
    // Install with proper permissions
    // Update local registry
    damn based
}
```

## Dependencies

```cursed
yeet "testz"  // Testing framework only
```

**Note:** The module is designed to be dependency-free for maximum compatibility.

## Architecture

### Simple Plugin System Design

1. **Core Layer**: Basic load/unload functionality
2. **Registry Layer**: Plugin tracking and management
3. **Discovery Layer**: Plugin finding and validation
4. **Interface Layer**: Plugin communication protocols

### Extension Points

- **Plugin Types**: Support for different plugin categories
- **Loading Mechanisms**: Dynamic library loading, script plugins
- **Security Models**: Permission systems, sandboxing
- **Communication**: Event systems, API exposure

## Integration Examples

### Application Framework Integration

```cursed
// Framework with plugin support
squad Application {
    spill plugin_system PluginSystem
    spill core_modules []Module
}

slay (app *Application) initialize_with_plugins() {
    // Initialize core application
    app.initialize_core()
    
    // Load essential plugins
    load_plugin("auth")
    load_plugin("logger")
    
    // Start application with plugin support
    app.start_with_plugin_support()
}
```

### Configuration-Driven Plugin Loading

```cursed
// Load plugins based on configuration file
slay load_plugins_from_config_file(config_path tea) {
    sus config_content tea = read_file(config_path)
    sus plugin_list []tea = parse_plugin_config(config_content)
    
    bestie i := 0; i < len(plugin_list); i = i + 1 {
        sus plugin_name tea = plugin_list[i]
        load_plugin(plugin_name)
    }
}
```

The `plugin_vibes_simple` module provides a foundational plugin system that can be extended to support sophisticated plugin architectures while maintaining simplicity for basic use cases.
