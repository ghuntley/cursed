# Plugin System for CURSED

A comprehensive plugin system implementation in pure CURSED that provides dynamic loading, plugin management, and extension APIs.

## Features

### Core Plugin Management
- **Dynamic Plugin Loading**: Load and unload plugins at runtime
- **Plugin Registration**: Register plugins with metadata and dependencies
- **Plugin Discovery**: Automatic plugin discovery and registration
- **Hot Reload**: Reload plugins without restarting the application
- **Dependency Resolution**: Automatic dependency loading and management

### Hook System
- **Plugin Hooks**: Register and execute hooks at specific points in the application
- **Priority-Based Execution**: Execute hooks in priority order
- **Conditional Hooks**: Execute hooks based on conditions
- **Hook Statistics**: Track hook execution performance and errors

### Event System
- **Event Listeners**: Register event listeners for plugin lifecycle events
- **Event Triggering**: Trigger events and notify registered listeners
- **Event Filtering**: Filter events based on conditions
- **Event Statistics**: Track event execution and performance

### Security and Permissions
- **Permission System**: Control plugin access to system resources
- **API Context**: Manage plugin API access and rate limiting
- **Sandboxing**: Isolate plugins in secure environments
- **Resource Limits**: Enforce resource usage limits per plugin

### Configuration Management
- **Plugin Configuration**: Store and retrieve plugin-specific configuration
- **Configuration Validation**: Validate configuration against schemas
- **Environment Overrides**: Override configuration with environment variables
- **Configuration Encryption**: Encrypt sensitive configuration data

## Usage

### Basic Plugin System Setup

```cursed
yeet "plugin_system"

# Initialize the plugin system
plugin_system_init()

# Create plugin metadata
sus metadata := plugin_create_metadata(
    "my_plugin",
    "1.0.0",
    "My awesome plugin",
    "Your Name"
)

# Register the plugin
plugin_register(metadata)

# Load the plugin
plugin_load("my_plugin")
```

### Plugin Hooks

```cursed
# Register a hook
plugin_register_hook("my_plugin", "before_process", "my_callback", PLUGIN_PRIORITY_HIGH)

# Execute hooks
plugin_execute_hooks("before_process", "context_data")
```

### Event System

```cursed
# Register event listener
plugin_register_event_listener("my_plugin", PLUGIN_EVENT_LOAD, "on_load_callback")

# Trigger event
plugin_trigger_event(PLUGIN_EVENT_LOAD, "my_plugin", "Plugin loaded successfully")
```

### Configuration

```cursed
# Set configuration
plugin_set_config("my_plugin", "setting1", "value1")

# Get configuration
sus value := plugin_get_config("my_plugin", "setting1")
```

### Security

```cursed
# Check permission
sus has_permission := plugin_check_permission("my_plugin", "read_files")

# Create API context
sus context := plugin_create_api_context("my_plugin")
```

## Plugin Structure

A typical plugin should implement the following structure:

```cursed
# Plugin metadata
fam MyPluginMetadata {
    name tea
    version tea
    description tea
    author tea
    dependencies [5]tea
    entry_point tea
}

# Plugin initialization
slay plugin_main() lit {
    # Plugin initialization code
    damn based
}

# Plugin cleanup
slay plugin_cleanup() lit {
    # Plugin cleanup code
    damn based
}

# Hook implementations
slay my_hook_callback(context tea) lit {
    # Hook implementation
    damn based
}

# Event handlers
slay on_event_callback(event_data tea) lit {
    # Event handler implementation
    damn based
}
```

## Constants

### Plugin Status
- `PLUGIN_STATUS_UNLOADED` (0): Plugin is not loaded
- `PLUGIN_STATUS_LOADED` (1): Plugin is loaded and ready
- `PLUGIN_STATUS_ACTIVE` (2): Plugin is active and running
- `PLUGIN_STATUS_ERROR` (3): Plugin has an error

### Plugin Priority
- `PLUGIN_PRIORITY_LOW` (0): Low priority execution
- `PLUGIN_PRIORITY_NORMAL` (1): Normal priority execution
- `PLUGIN_PRIORITY_HIGH` (2): High priority execution
- `PLUGIN_PRIORITY_CRITICAL` (3): Critical priority execution

### Plugin Events
- `PLUGIN_EVENT_LOAD` (0): Plugin loading event
- `PLUGIN_EVENT_UNLOAD` (1): Plugin unloading event
- `PLUGIN_EVENT_ACTIVATE` (2): Plugin activation event
- `PLUGIN_EVENT_DEACTIVATE` (3): Plugin deactivation event
- `PLUGIN_EVENT_UPDATE` (4): Plugin update event
- `PLUGIN_EVENT_ERROR` (5): Plugin error event

## API Reference

### Core Functions

#### `plugin_system_init() -> lit`
Initialize the plugin system.

#### `plugin_create_metadata(name: tea, version: tea, description: tea, author: tea) -> PluginMetadata`
Create plugin metadata with the specified information.

#### `plugin_register(metadata: PluginMetadata) -> lit`
Register a plugin with the system.

#### `plugin_load(plugin_name: tea) -> lit`
Load a plugin by name.

#### `plugin_unload(plugin_name: tea) -> lit`
Unload a plugin by name.

#### `plugin_find_by_name(plugin_name: tea) -> normie`
Find a plugin by name and return its index.

#### `plugin_get_info(plugin_name: tea) -> PluginMetadata`
Get plugin information by name.

#### `plugin_get_status(plugin_name: tea) -> normie`
Get plugin status by name.

### Hook Functions

#### `plugin_register_hook(plugin_name: tea, hook_name: tea, callback_function: tea, priority: normie) -> lit`
Register a plugin hook.

#### `plugin_execute_hooks(hook_name: tea, context: tea) -> lit`
Execute all hooks for a given hook name.

#### `plugin_remove_hooks(plugin_name: tea) -> lit`
Remove all hooks for a plugin.

### Event Functions

#### `plugin_register_event_listener(plugin_name: tea, event_type: normie, callback_function: tea) -> lit`
Register an event listener.

#### `plugin_trigger_event(event_type: normie, plugin_name: tea, message: tea) -> lit`
Trigger an event.

#### `plugin_remove_listeners(plugin_name: tea) -> lit`
Remove all event listeners for a plugin.

### Configuration Functions

#### `plugin_set_config(plugin_name: tea, key: tea, value: tea) -> lit`
Set plugin configuration.

#### `plugin_get_config(plugin_name: tea, key: tea) -> tea`
Get plugin configuration.

### Security Functions

#### `plugin_check_permission(plugin_name: tea, permission: tea) -> lit`
Check if a plugin has a specific permission.

#### `plugin_create_api_context(plugin_name: tea) -> PluginAPIContext`
Create an API context for a plugin.

### Management Functions

#### `plugin_hot_reload(plugin_name: tea) -> lit`
Hot reload a plugin.

#### `plugin_health_check(plugin_name: tea) -> lit`
Check plugin health.

#### `plugin_resolve_dependencies(plugin_name: tea) -> lit`
Resolve plugin dependencies.

#### `plugin_list_all() -> normie`
List all plugins.

#### `plugin_list_active() -> normie`
List active plugins.

#### `plugin_get_system_stats() -> normie`
Get plugin system statistics.

#### `plugin_cleanup_all() -> lit`
Cleanup all plugins.

## Data Structures

### PluginMetadata
Contains plugin information including name, version, description, author, dependencies, permissions, and configuration.

### PluginRegistry
The main registry that manages all plugins, hooks, event listeners, and system configuration.

### PluginHook
Represents a plugin hook with callback information, priority, and execution statistics.

### PluginEventListener
Represents an event listener with event type, callback, and execution tracking.

### PluginAPIContext
Manages plugin API access, permissions, rate limiting, and security context.

### PluginConfiguration
Handles plugin configuration storage, validation, and encryption.

## Testing

Run the comprehensive test suite:

```bash
cargo run --bin cursed stdlib/plugin_system/test_plugin_system.csd
```

The test suite covers:
- Plugin system initialization
- Plugin metadata creation
- Plugin registration and management
- Plugin loading and unloading
- Hook system functionality
- Event system functionality
- Configuration management
- Security and permissions
- API context management
- Dependency resolution
- Hot reload functionality
- Health checks
- Error handling
- System cleanup

## Examples

### Example Plugin

```cursed
yeet "plugin_system"

# Initialize plugin system
plugin_system_init()

# Create and register a simple plugin
sus metadata := plugin_create_metadata(
    "logger_plugin",
    "1.0.0",
    "Logging plugin for application events",
    "CURSED Team"
)

plugin_register(metadata)

# Register a hook for logging
plugin_register_hook("logger_plugin", "before_request", "log_request", PLUGIN_PRIORITY_HIGH)

# Register event listener
plugin_register_event_listener("logger_plugin", PLUGIN_EVENT_LOAD, "on_plugin_loaded")

# Load the plugin
plugin_load("logger_plugin")

# Execute hooks
plugin_execute_hooks("before_request", "GET /api/users")

# Check plugin status
sus status := plugin_get_status("logger_plugin")
vibez.spill("Plugin status: " + status)
```

### Advanced Plugin with Dependencies

```cursed
yeet "plugin_system"

# Initialize plugin system
plugin_system_init()

# Create base plugin
sus base_metadata := plugin_create_metadata(
    "base_plugin",
    "1.0.0",
    "Base functionality plugin",
    "CURSED Team"
)
plugin_register(base_metadata)

# Create dependent plugin
sus dependent_metadata := plugin_create_metadata(
    "dependent_plugin",
    "1.0.0",
    "Plugin that depends on base_plugin",
    "CURSED Team"
)

# Add dependency
dependent_metadata.dependencies[0] = "base_plugin"
dependent_metadata.dependency_count = 1

plugin_register(dependent_metadata)

# Load dependent plugin (will automatically load base_plugin)
plugin_load("dependent_plugin")

# Verify both plugins are loaded
vibez.spill("Base plugin status: " + plugin_get_status("base_plugin"))
vibez.spill("Dependent plugin status: " + plugin_get_status("dependent_plugin"))
```

## Performance Considerations

- **Plugin Limits**: Maximum of 100 plugins by default
- **Hook Limits**: Maximum of 200 hooks
- **Event Listener Limits**: Maximum of 150 event listeners
- **Resource Monitoring**: Automatic resource usage tracking
- **Performance Statistics**: Hook and event execution time tracking

## Security Features

- **Permission System**: Fine-grained permission control
- **Sandboxing**: Plugin isolation for security
- **Resource Limits**: Prevent resource exhaustion
- **API Rate Limiting**: Control API access rates
- **Security Tokens**: Secure plugin authentication

## Configuration

The plugin system can be configured through:
- Configuration files
- Environment variables
- Runtime API calls
- Plugin-specific settings

Default configuration:
- Plugin directory: `/plugins`
- Config file: `/etc/cursed/plugins.conf`
- Sandbox enabled: `based`
- Hot reload enabled: `based`
- Dependency resolution enabled: `based`

## Future Enhancements

- **Plugin Marketplace**: Integration with plugin repositories
- **Plugin Signing**: Digital signature verification
- **Advanced Sandboxing**: Enhanced security isolation
- **Plugin Metrics**: Detailed performance and usage metrics
- **Plugin Templates**: Code generation for plugin scaffolding
- **Plugin Documentation**: Automatic documentation generation

## Contributing

To contribute to the plugin system:

1. Follow the CURSED coding standards
2. Write comprehensive tests for new features
3. Update documentation for API changes
4. Test with both interpretation and compilation modes
5. Ensure security best practices are followed

## License

This plugin system is part of the CURSED language project and follows the same license terms.
