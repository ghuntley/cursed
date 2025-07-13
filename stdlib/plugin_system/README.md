# Plugin System Module

Pure CURSED implementation of a comprehensive plugin management system for dynamic loading, discovery, lifecycle management, and security without FFI dependencies.

## Overview

The plugin_system module provides functionality for loading and managing plugins to extend applications at runtime. It includes features for plugin discovery, loading/unloading, sandboxing, security verification, and lifecycle management.

## Core Types

### `Plug`
Represents a loaded plugin handle.
- Type: `normie` (plugin ID)

### `PlugStatus`
Plugin status enumeration.
- `PLUG_STATUS_UNLOADED = 0` - Plugin is not loaded
- `PLUG_STATUS_LOADED = 1` - Plugin is loaded and active
- `PLUG_STATUS_ERROR = 2` - Plugin encountered an error
- `PLUG_STATUS_SANDBOXED = 3` - Plugin is loaded in sandbox mode

### `PlugCapability`
String type representing plugin capabilities.
- Type: `tea` (capability name)

## Plugin Discovery

### `discover_plugins(directory tea) normie`
Discovers plugins in the specified directory.
- **Parameters**: `directory` - Path to search for plugins
- **Returns**: Number of plugins found

```cursed
sus count := plugin_system.discover_plugins("./plugins")
vibez.spill("Found plugins:", count)
```

## Plugin Loading

### `load_plugin(path tea) Plug`
Loads a plugin from the specified path.
- **Parameters**: `path` - Path to plugin file
- **Returns**: Plugin handle

### `load_plugin_with_options(path tea, verify_signature lit, sandbox lit) Plug`
Loads a plugin with additional security options.
- **Parameters**: 
  - `path` - Path to plugin file
  - `verify_signature` - Whether to verify plugin signature
  - `sandbox` - Whether to load in sandboxed mode
- **Returns**: Plugin handle

```cursed
sus plugin := plugin_system.load_plugin("./plugins/math_tools.so")
sus secure_plugin := plugin_system.load_plugin_with_options("./plugins/secure.so", based, based)
```

## Plugin Information

### `get_plugin_name(plugin Plug) tea`
Gets the name of a loaded plugin.

### `get_plugin_path(plugin Plug) tea`
Gets the file path of a plugin.

### `get_plugin_status(plugin Plug) PlugStatus`
Gets the current status of a plugin.

### `get_plugin_version(plugin Plug) tea`
Gets the version string of a plugin.

### `get_plugin_author(plugin Plug) tea`
Gets the author of a plugin.

### `get_plugin_description(plugin Plug) tea`
Gets the description of a plugin.

```cursed
sus plugin := plugin_system.load_plugin("./plugins/demo.so")
sus name := plugin_system.get_plugin_name(plugin)
sus version := plugin_system.get_plugin_version(plugin)
sus author := plugin_system.get_plugin_author(plugin)
vibez.spill("Plugin:", name, "v" + version, "by", author)
```

## Plugin Capabilities

### `get_plugin_capabilities(plugin Plug) tea`
Gets comma-separated list of plugin capabilities.

### `has_capability(plugin Plug, capability tea) lit`
Checks if plugin has a specific capability.

```cursed
sus capabilities := plugin_system.get_plugin_capabilities(plugin)
vibez.spill("Capabilities:", capabilities)

sus has_math := plugin_system.has_capability(plugin, "math")
if has_math {
    vibez.spill("Plugin supports math operations")
}
```

## Plugin Registry

### `register_plugin(name tea, plugin Plug) lit`
Registers a plugin with a friendly name.

### `find_plugin_by_name(name tea) Plug`
Finds a plugin by its registered name.

### `list_loaded_plugins() normie`
Returns the count of currently loaded plugins.

```cursed
sus success := plugin_system.register_plugin("math_tools", plugin)
sus found := plugin_system.find_plugin_by_name("math_tools")
sus count := plugin_system.list_loaded_plugins()
```

## Plugin Lifecycle

### `initialize_plugin(plugin Plug) lit`
Initializes a loaded plugin.

### `cleanup_plugin(plugin Plug) lit`
Cleans up plugin resources.

### `unload_plugin(plugin Plug) lit`
Unloads a plugin from memory.

### `reload_plugin(plugin Plug) lit`
Reloads a plugin (unload + load).

```cursed
sus init_success := plugin_system.initialize_plugin(plugin)
sus cleanup_success := plugin_system.cleanup_plugin(plugin)
sus unload_success := plugin_system.unload_plugin(plugin)
```

## Plugin Security

### `verify_plugin_signature(path tea, public_key tea) lit`
Verifies the digital signature of a plugin.

### `generate_plugin_manifest(name tea, version tea, author tea, description tea) tea`
Generates a plugin manifest in JSON format.

### `validate_plugin(path tea) lit`
Validates plugin format and dependencies.

```cursed
sus signature_valid := plugin_system.verify_plugin_signature("./plugin.so", "public_key")
sus manifest := plugin_system.generate_plugin_manifest("MyPlugin", "1.0.0", "Developer", "Description")
sus is_valid := plugin_system.validate_plugin("./plugin.so")
```

## Plugin Sandboxing

### `create_sandbox() normie`
Creates a new sandbox environment.

### `execute_in_sandbox(sandbox_id normie, plugin Plug, function_name tea) lit`
Executes a plugin function within a sandbox.

```cursed
sus sandbox := plugin_system.create_sandbox()
sus exec_success := plugin_system.execute_in_sandbox(sandbox, plugin, "calculate")
```

## Plugin Manager

### `create_plugin_manager(plugin_dir tea, auto_load lit) normie`
Creates a plugin manager instance.

### `start_plugin_manager(manager_id normie) lit`
Starts the plugin manager.

### `stop_plugin_manager(manager_id normie) lit`
Stops the plugin manager.

```cursed
sus manager := plugin_system.create_plugin_manager("./plugins", based)
sus start_ok := plugin_system.start_plugin_manager(manager)
# ... use manager ...
sus stop_ok := plugin_system.stop_plugin_manager(manager)
```

## Plugin Installation

### `install_plugin_from_url(url tea, destination tea) lit`
Downloads and installs a plugin from URL.

### `uninstall_plugin(name tea) lit`
Uninstalls a plugin by name.

### `is_plugin_compatible(plugin Plug, api_version tea) lit`
Checks plugin compatibility with host API version.

```cursed
sus install_ok := plugin_system.install_plugin_from_url("https://example.com/plugin.so", "./plugins/")
sus uninstall_ok := plugin_system.uninstall_plugin("old_plugin")
sus compatible := plugin_system.is_plugin_compatible(plugin, "1.0")
```

## Extension Points

### `create_extension_point(name tea) normie`
Creates a new extension point for plugins to hook into.

### `register_extension(point_id normie, plugin Plug) lit`
Registers a plugin with an extension point.

### `call_extension_point(point_id normie, data tea) tea`
Calls all plugins registered to an extension point.

```cursed
sus ext_point := plugin_system.create_extension_point("filter_content")
sus register_ok := plugin_system.register_extension(ext_point, plugin)
sus result := plugin_system.call_extension_point(ext_point, "input_data")
```

## Utility Functions

### `is_valid_plugin(plugin Plug) lit`
Checks if a plugin handle is valid.

### `get_total_plugins() normie`
Gets total number of plugins in registry.

### `get_plugin_memory_usage(plugin Plug) normie`
Gets estimated memory usage of a plugin.

### `reset_plugin_registry()`
Resets the plugin registry (for testing).

```cursed
sus valid := plugin_system.is_valid_plugin(plugin)
sus total := plugin_system.get_total_plugins()
sus memory := plugin_system.get_plugin_memory_usage(plugin)
```

## Complete Example

```cursed
yeet "plugin_system"
yeet "vibez"

slay main() {
    # Create plugin manager
    sus manager := plugin_system.create_plugin_manager("./plugins", based)
    plugin_system.start_plugin_manager(manager)
    
    # Load and register a plugin
    sus math_plugin := plugin_system.load_plugin("./plugins/math_tools.so")
    plugin_system.register_plugin("math", math_plugin)
    
    # Check plugin info
    sus name := plugin_system.get_plugin_name(math_plugin)
    sus version := plugin_system.get_plugin_version(math_plugin)
    sus capabilities := plugin_system.get_plugin_capabilities(math_plugin)
    
    vibez.spill("Loaded plugin:", name)
    vibez.spill("Version:", version)
    vibez.spill("Capabilities:", capabilities)
    
    # Create extension point
    sus filter_point := plugin_system.create_extension_point("data_filter")
    plugin_system.register_extension(filter_point, math_plugin)
    
    # Use extension point
    sus result := plugin_system.call_extension_point(filter_point, "test_data")
    vibez.spill("Filtered result:", result)
    
    # Load secure plugin with sandboxing
    sus secure_plugin := plugin_system.load_plugin_with_options("./plugins/untrusted.so", based, based)
    
    if plugin_system.is_plugin_compatible(secure_plugin, "1.0") {
        sus sandbox := plugin_system.create_sandbox()
        plugin_system.execute_in_sandbox(sandbox, secure_plugin, "process_data")
    }
    
    # Cleanup
    plugin_system.unload_plugin(math_plugin)
    plugin_system.unload_plugin(secure_plugin)
    plugin_system.stop_plugin_manager(manager)
}
```

## Testing

Run the comprehensive test suite:

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/plugin_system/test_plugin_system.csd

# Test compilation mode
cargo run --bin cursed -- compile stdlib/plugin_system/test_plugin_system.csd
./test_plugin_system

# Both-mode verification
test_both_modes() {
    cargo run --bin cursed stdlib/plugin_system/test_plugin_system.csd > interp_output.txt
    cargo run --bin cursed -- compile stdlib/plugin_system/test_plugin_system.csd
    ./test_plugin_system > comp_output.txt
    diff interp_output.txt comp_output.txt
}
```

## Implementation Notes

This module provides a pure CURSED implementation without FFI dependencies, using arrays and counters to simulate plugin management. In a production implementation, this would interface with actual dynamic library loading, file system operations, and process sandboxing.

The module includes:
- Dynamic plugin discovery and loading
- Plugin lifecycle management
- Security features (signature verification, sandboxing)
- Extension point system for host application integration
- Hot reloading capabilities
- Memory and resource management
- Comprehensive error handling

All functions follow CURSED naming conventions and use appropriate Gen Z slang terms while maintaining professional functionality.
