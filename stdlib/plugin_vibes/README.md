# plugin_vibes - Dynamic Plugin System

A comprehensive plugin system for CURSED applications enabling dynamic loading, management, and execution of plugins with security, sandboxing, and inter-plugin communication.

## Overview

The `plugin_vibes` module provides a complete plugin architecture with:

- **Dynamic Loading**: Load/unload plugins at runtime (.so, .dll, .dylib support)
- **Security Sandboxing**: Isolated execution environments with permission control
- **API Management**: Plugin API registration and discovery
- **Event System**: Inter-plugin communication via events and messaging
- **Lifecycle Management**: Complete plugin lifecycle with hooks and monitoring
- **Configuration**: Per-plugin configuration management with validation
- **Health Monitoring**: Plugin health checks, metrics, and performance monitoring
- **Error Recovery**: Comprehensive error handling and recovery mechanisms

## Quick Start

### Basic Plugin System Setup

```cursed
yeet "plugin_vibes"

# Initialize plugin system
slay init_app() lit {
    # Discover available plugins
    sus plugins := discover_plugins("./plugins")
    vibez.spill("Found plugins: " + plugins)
    
    # Load core plugins
    load_plugin("security")
    load_plugin("logging")
    load_plugin("auth")
    
    # Setup event handlers
    register_event_handler("auth", "login_attempt", "validate_login")
    register_event_handler("logging", "error", "log_error")
    
    damn based
}

# Handle user authentication
slay authenticate_user(username tea, password tea) tea {
    # Broadcast login attempt
    sus event_data := "{\"username\":\"" + username + "\"}"
    broadcast_event("login_attempt", event_data)
    
    # Call auth plugin
    sus result := call_plugin_api("auth", "validate_credentials", 
                                 "{\"username\":\"" + username + "\",\"password\":\"" + password + "\"}")
    
    damn result
}
```

### Secure Plugin Loading

```cursed
# Load plugin with security restrictions
slay load_secure_plugin(plugin_name tea) lit {
    # Create sandbox with memory and time limits
    sus restrictions := "{\"memory_limit\":2048,\"time_limit\":30}"
    create_plugin_sandbox(plugin_name, restrictions)
    
    # Set specific permissions
    set_plugin_permission(plugin_name, "file_read", based)
    set_plugin_permission(plugin_name, "network", cap)  # Deny network access
    
    # Load and validate
    sus loaded := load_plugin(plugin_name)
    lowkey loaded {
        sus security := validate_plugin_security(plugin_name)
        lowkey string_contains(security, "RISK") {
            quarantine_plugin(plugin_name, "security_violation")
            damn cap
        }
    }
    
    damn loaded
}
```

## Core Features

### 1. Plugin Discovery and Loading

#### Plugin Discovery
```cursed
# Discover plugins in directory
sus plugins := discover_plugins("./plugins")
# Returns: "auth,logging,cache"

# Load individual plugins
sus loaded := load_plugin("auth")
assert_true(loaded)

# Unload when done
sus unloaded := unload_plugin("auth")
assert_true(unloaded)
```

#### Plugin States
- `unloaded` - Plugin not loaded
- `loading` - Initialization in progress  
- `active` - Plugin loaded and active
- `suspended` - Temporarily disabled
- `error` - Plugin in error state
- `unloading` - Shutdown in progress

```cursed
# Check plugin state
sus state := get_plugin_state("auth")
# Returns: "active"

# Change plugin state
set_plugin_state("auth", "suspended")
```

### 2. API Registration and Management

```cursed
# Register plugin APIs
register_plugin_api("auth", "validate_token", "auth_validate_func")
register_plugin_api("auth", "refresh_token", "auth_refresh_func")

# Discover available APIs
sus apis := get_plugin_apis("auth")
# Returns: "validate_token,refresh_token,get_user_info"

# Call plugin APIs
sus result := call_plugin_api("auth", "validate_token", 
                             "{\"token\":\"abc123\"}")
# Returns: {"valid":true,"user":"alice"}
```

### 3. Event System

#### Event Registration
```cursed
# Register event handlers
register_event_handler("logger", "user_login", "log_login")
register_event_handler("auth", "login_attempt", "validate_login")
register_event_handler("stats", "user_action", "track_action")
```

#### Event Broadcasting
```cursed
# Broadcast to all handlers
sus count := broadcast_event("user_login", "{\"user\":\"alice\"}")
# Returns: 3 (number of plugins that handled the event)

# Send to specific plugin
sus sent := send_event_to_plugin("logger", "error", "{\"message\":\"Failed login\"}")
```

#### Event Queue
```cursed
# Queue delayed events
queue_event("cleanup_cache", "{}", 5000)  # 5 second delay

# Process queued events
sus processed := process_event_queue()
# Returns: number of events processed

# Clear queue
clear_event_queue()
```

### 4. Security and Sandboxing

#### Permission System
```cursed
# Set plugin permissions
set_plugin_permission("untrusted", "file_read", based)
set_plugin_permission("untrusted", "network", cap)
set_plugin_permission("untrusted", "exec", cap)

# Check permissions
sus can_read := check_plugin_permission("untrusted", "file_read")
# Returns: based (true)

# Get all permissions
sus perms := get_plugin_permissions("untrusted")
# Returns: "file_read"
```

#### Available Permissions
- `file_read` - Read file system access
- `file_write` - Write file system access
- `network` - Network access
- `exec` - Execute external commands
- `memory` - Advanced memory operations
- `system` - System-level operations

#### Sandbox Management
```cursed
# Create isolated sandbox
sus restrictions := "{\"memory_limit\":1024,\"time_limit\":30,\"network\":false}"
create_plugin_sandbox("untrusted", restrictions)

# Validate plugin security
sus report := validate_plugin_security("untrusted")
# Returns: "SECURITY: Plugin validated successfully"

# Destroy sandbox when done
destroy_plugin_sandbox("untrusted")
```

### 5. Inter-Plugin Communication

#### Direct Messaging
```cursed
# Send message between plugins
send_message("auth", "logger", "{\"event\":\"login_success\",\"user\":\"alice\"}")

# Receive messages
sus message := receive_message("logger")
# Returns: {"sender":"auth","message":"login_success"}

# Broadcast to all plugins
sus recipients := broadcast_message("system", "{\"shutdown\":true}")
# Returns: 5 (number of recipients)
```

#### Shared Data Store
```cursed
# Store shared data
set_shared_data("user_count", "1024", "stats")
set_shared_data("cache_size", "2048", "cache")

# Retrieve data
sus count := get_shared_data("user_count", "dashboard")
# Returns: "1024"

# List available keys
sus keys := list_shared_keys("stats")
# Returns: "user_count,session_data,cache_size"

# Clean up
delete_shared_data("old_key", "stats")
```

### 6. Configuration Management

```cursed
# Load plugin configuration
sus config := load_plugin_config("auth")
# Returns: {"debug_mode":true,"timeout":30,"retries":3}

# Get specific values
sus debug := get_config_value("auth", "debug_mode")
# Returns: "true"

# Update configuration
set_config_value("auth", "timeout", "60")

# Save updated config
save_plugin_config("auth", "{\"timeout\":60,\"debug_mode\":false}")

# Validate configuration
sus valid := validate_plugin_config("auth", "{\"timeout\":30}")
# Returns: based (true)

# Get config schema
sus schema := get_config_schema("auth")
# Returns: JSON schema definition
```

### 7. Health Monitoring

#### Health Checks
```cursed
# Check plugin health
sus health := check_plugin_health("auth")
# Returns: "healthy", "warning", "critical", or "unresponsive"

# Get detailed metrics
sus metrics := get_plugin_metrics("auth")
# Returns: {"api_calls":42,"memory_usage":1024,"response_time":25,"error_count":0}

# Performance monitoring
sus perf := get_plugin_performance("cache")
# Returns: {"avg_response_time":25,"throughput":100,"cpu_usage":5.2}
```

#### Logging and Activity
```cursed
# Log plugin activity
log_plugin_activity("auth", "token_validated")

# Get plugin logs
sus logs := get_plugin_logs("auth", 50)  # Last 50 entries
# Returns: [{"timestamp":"2025-01-01T12:00:00Z","level":"INFO","message":"Plugin initialized"}]

# Reset metrics
reset_plugin_metrics("auth")
```

### 8. Plugin Information

```cursed
# Get plugin metadata
sus info := get_plugin_info("auth")
# Returns: {"name":"auth","version":"1.2.3","author":"CURSED Dev","description":"Authentication plugin"}

# List all loaded plugins
sus plugins := list_loaded_plugins()
# Returns: "auth,logger,cache"

# Check version
sus version := get_plugin_version("auth")
# Returns: "1.2.3"

# Check dependencies
sus deps := get_plugin_dependencies("auth")
# Returns: ["core","security"]

# Validate dependencies
sus deps_ok := check_plugin_dependencies("auth")
# Returns: based (all dependencies satisfied)
```

### 9. Advanced Features

#### Hooks and Filters
```cursed
# Register plugin hooks
register_plugin_hook("pre_request", "auth", "check_authentication")
register_plugin_hook("post_response", "logger", "log_response")

# Apply filters to data
sus clean_data := apply_plugin_filters("sanitize_input", user_input)

# Get plugins for specific hook
sus hook_plugins := get_hook_plugins("pre_request")
# Returns: "auth,security"
```

#### Plugin Templates
```cursed
# Create new plugin template
sus template := create_plugin_template("my_plugin", "api")
# Creates basic plugin structure

# Validate plugin structure
sus valid := validate_plugin_structure("./my_plugin")

# Package for distribution
package_plugin("my_plugin", "./dist/my_plugin.plugin")
```

### 10. Error Handling and Recovery

#### Error Management
```cursed
# Get plugin errors
sus errors := get_plugin_errors("problematic_plugin")
# Returns: [{"type":"runtime_error","message":"Plugin crashed","timestamp":"..."}]

# Clear errors
clear_plugin_errors("problematic_plugin")

# Set error handler
set_error_handler("problematic_plugin", "handle_plugin_error")
```

#### Recovery Mechanisms
```cursed
# Restart plugin
restart_plugin("crashed_plugin")

# Recover with different modes
recover_plugin("failed_plugin", "soft")    # Restart in same state
recover_plugin("failed_plugin", "reset")   # Reset to default state  
recover_plugin("failed_plugin", "safe")    # Start in safe mode

# Quarantine malicious plugins
quarantine_plugin("malicious_plugin", "security_violation")
```

## Advanced Usage Patterns

### Plugin-Based Web Server

```cursed
# Web server with plugin architecture
slay init_web_server() lit {
    # Load core plugins
    load_plugin("router")
    load_plugin("auth")
    load_plugin("cors")
    load_plugin("rate_limiter")
    
    # Setup request pipeline hooks
    register_plugin_hook("pre_request", "auth", "authenticate")
    register_plugin_hook("pre_request", "cors", "add_cors_headers")
    register_plugin_hook("pre_request", "rate_limiter", "check_rate_limit")
    
    # Setup route handlers
    register_event_handler("router", "GET /api/users", "get_users")
    register_event_handler("router", "POST /api/login", "handle_login")
    
    damn based
}

# Handle incoming request
slay handle_request(method tea, path tea, headers tea) tea {
    # Apply pre-request hooks
    sus hook_result := apply_plugin_filters("pre_request", headers)
    lowkey string_contains(hook_result, "REJECT") {
        damn "{\"error\":\"Request rejected\"}"
    }
    
    # Route request
    sus route_event := method + " " + path
    sus handled := send_event_to_plugin("router", route_event, headers)
    
    lowkey handled {
        damn "{\"status\":\"success\"}"
    } else {
        damn "{\"error\":\"Route not found\"}"
    }
}
```

### Microservices Plugin System

```cursed
# Microservice with plugin-based features
slay init_microservice() lit {
    # Load service-specific plugins
    load_plugin("database")
    load_plugin("cache")
    load_plugin("metrics")
    load_plugin("health_check")
    
    # Configure plugin permissions
    set_plugin_permission("database", "network", based)
    set_plugin_permission("cache", "memory", based)
    set_plugin_permission("metrics", "file_write", based)
    
    # Setup inter-service communication
    register_event_handler("database", "connection_lost", "reconnect_db")
    register_event_handler("cache", "memory_warning", "clear_cache")
    
    damn based
}

# Process business logic with plugins
slay process_order(order_data tea) tea {
    # Validate with plugins
    sus validated := call_plugin_api("validator", "validate_order", order_data)
    lowkey !string_contains(validated, "valid") {
        damn "{\"error\":\"Invalid order\"}"
    }
    
    # Store in database
    sus stored := call_plugin_api("database", "save_order", order_data)
    
    # Update cache
    call_plugin_api("cache", "invalidate", "{\"key\":\"orders\"}")
    
    # Record metrics
    log_plugin_activity("metrics", "order_processed")
    
    damn stored
}
```

### Plugin Development Template

```cursed
# Template for creating new plugins
# File: my_plugin.plugin.💀

yeet "plugin_vibes"

# Plugin metadata
sus PLUGIN_NAME tea = "my_plugin"
sus PLUGIN_VERSION tea = "1.0.0"
sus PLUGIN_AUTHOR tea = "Your Name"

# Plugin initialization
slay my_plugin_init() lit {
    # Register APIs
    register_plugin_api(PLUGIN_NAME, "process_data", "my_plugin_process")
    register_plugin_api(PLUGIN_NAME, "get_status", "my_plugin_status")
    
    # Register event handlers
    register_event_handler(PLUGIN_NAME, "data_received", "my_plugin_handle_data")
    
    # Load configuration
    sus config := load_plugin_config(PLUGIN_NAME)
    
    vibez.spill("Plugin " + PLUGIN_NAME + " v" + PLUGIN_VERSION + " initialized")
    damn based
}

# Plugin API functions
slay my_plugin_process(data tea) tea {
    # Process data according to plugin logic
    sus result := "Processed: " + data
    
    # Log activity
    log_plugin_activity(PLUGIN_NAME, "data_processed")
    
    damn result
}

slay my_plugin_status() tea {
    damn "{\"status\":\"active\",\"version\":\"" + PLUGIN_VERSION + "\"}"
}

# Event handlers
slay my_plugin_handle_data(event_data tea) lit {
    # Handle incoming data events
    sus processed := my_plugin_process(event_data)
    
    # Broadcast result
    broadcast_event("data_processed", processed)
    
    damn based
}

# Plugin cleanup
slay my_plugin_cleanup() lit {
    # Unregister APIs and handlers
    unregister_plugin_api(PLUGIN_NAME, "process_data")
    unregister_plugin_api(PLUGIN_NAME, "get_status")
    unregister_event_handler(PLUGIN_NAME, "data_received")
    
    vibez.spill("Plugin " + PLUGIN_NAME + " cleaned up")
    damn based
}
```

## Performance Characteristics

### Time Complexity
- Plugin loading: O(1) for individual plugins
- Event broadcasting: O(n) where n is number of registered handlers
- API calls: O(1) for direct plugin API calls
- Configuration access: O(1) for cached configurations
- Permission checks: O(1) with hash-based lookup

### Memory Usage
- Plugin isolation prevents memory leaks between plugins
- Shared data store uses efficient key-value storage
- Event queue has configurable size limits to prevent memory bloat
- Metrics collection uses rolling windows to bound memory usage
- Sandbox restrictions limit plugin memory consumption

### Security Guarantees
- Plugins run in isolated sandboxes with configurable restrictions
- Permission system enforces fine-grained access control
- Inter-plugin communication is monitored and logged
- All plugin operations are auditable
- Malicious plugin detection and automatic quarantine

## Security Best Practices

### Plugin Validation
```cursed
# Always validate plugins before loading
slay validate_and_load_plugin(plugin_name tea, plugin_path tea) lit {
    # Validate plugin structure
    lowkey !validate_plugin_structure(plugin_path) {
        vibez.spill("ERROR: Invalid plugin structure")
        damn cap
    }
    
    # Create restrictive sandbox
    sus restrictions := "{\"memory_limit\":512,\"time_limit\":10,\"network\":false}"
    create_plugin_sandbox(plugin_name, restrictions)
    
    # Load with minimal permissions
    load_plugin(plugin_name)
    
    # Validate security after loading
    sus security_report := validate_plugin_security(plugin_name)
    lowkey string_contains(security_report, "RISK") {
        quarantine_plugin(plugin_name, "security_risk_detected")
        damn cap
    }
    
    damn based
}
```

### Permission Management
```cursed
# Grant permissions incrementally
slay grant_plugin_permissions(plugin_name tea, required_perms tea) lit {
    # Start with no permissions
    set_plugin_permission(plugin_name, "file_read", cap)
    set_plugin_permission(plugin_name, "file_write", cap)
    set_plugin_permission(plugin_name, "network", cap)
    set_plugin_permission(plugin_name, "exec", cap)
    
    # Grant only what's needed
    lowkey string_contains(required_perms, "file_read") {
        set_plugin_permission(plugin_name, "file_read", based)
    }
    
    lowkey string_contains(required_perms, "network") {
        # Network access requires additional validation
        sus network_safe := validate_network_requirements(plugin_name)
        lowkey network_safe {
            set_plugin_permission(plugin_name, "network", based)
        }
    }
    
    damn based
}
```

## Testing

Run the comprehensive test suite:

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/plugin_vibes/test_plugin_vibes.💀

# Test compilation mode  
cargo run --bin cursed -- compile stdlib/plugin_vibes/test_plugin_vibes.💀
./test_plugin_vibes

# Both-mode verification
test_both_modes() {
    cargo run --bin cursed stdlib/plugin_vibes/test_plugin_vibes.💀 > interp_output.txt
    cargo run --bin cursed -- compile stdlib/plugin_vibes/test_plugin_vibes.💀
    ./test_plugin_vibes > comp_output.txt
    diff interp_output.txt comp_output.txt
}
```

The test suite covers:
- Plugin discovery and loading (8 tests)
- API registration and management (7 tests)
- Lifecycle management (5 tests)
- Event system (10 tests)
- Security and sandboxing (8 tests)
- Inter-plugin communication (6 tests)
- Shared data store (6 tests)
- Configuration management (8 tests)
- Health monitoring (8 tests)
- Plugin information (7 tests)
- Advanced features (6 tests)
- Error handling and recovery (10 tests)
- Edge cases and complex scenarios (4 tests)

**Total: 93 comprehensive tests covering all plugin system functionality**

## Dependencies

- `testz`: Testing framework for module validation
- No external plugin management libraries required
- No FFI dependencies for core functionality
- Pure CURSED implementation for maximum portability

## Implementation Notes

### Pure CURSED Design
- Implemented entirely in CURSED without external dependencies
- Compatible with both interpretation and compilation modes
- Self-contained plugin management system
- Uses CURSED string and data types throughout

### Thread Safety
- Plugin operations are designed to be thread-safe
- Event system supports concurrent access patterns
- Shared data store includes proper isolation
- Plugin sandboxes provide process-level isolation

### Error Handling
- All errors returned as descriptive strings with "ERROR:" prefix
- Plugin failures don't crash the host system
- Comprehensive error logging and recovery mechanisms
- Graceful degradation when plugins encounter issues

### Extensibility
- Plugin system is itself extensible via plugins
- New plugin types can be added without core changes
- Custom permission types and security policies supported
- Flexible event and messaging system for custom communication patterns

## Platform Support

The plugin system provides consistent behavior across all platforms supported by CURSED:
- Linux (native and interpreted modes)
- macOS (native and interpreted modes)  
- Windows (native and interpreted modes)
- No platform-specific plugin handling required
- Portable pure CURSED implementation

## Contributing

When extending the plugin system:

1. **Maintain Security**: All new features must respect the security model
2. **Test Coverage**: Add comprehensive tests for new functionality
3. **Documentation**: Update this README with new features and examples
4. **Backward Compatibility**: Ensure existing plugins continue to work
5. **Performance**: Consider performance impact of new features

## License

This module is part of the CURSED standard library and follows the same license terms as the CURSED language implementation.
