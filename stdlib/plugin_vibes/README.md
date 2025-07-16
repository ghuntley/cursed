# plugin_vibes Module

A comprehensive plugin system for CURSED applications that provides dynamic loading, management, and execution of plugins with enterprise-grade security, event handling, and monitoring capabilities.

## Overview

The `plugin_vibes` module enables CURSED applications to support a robust plugin architecture with:

- **Dynamic Plugin Loading**: Load and unload plugins at runtime
- **Secure Sandboxing**: Isolated execution environments with permission controls
- **API Management**: Register and call plugin APIs with validation
- **Event System**: Comprehensive event broadcasting and handling
- **Inter-Plugin Communication**: Message passing and shared data storage
- **Health Monitoring**: Performance metrics and health checking
- **Configuration Management**: Plugin-specific configuration with validation
- **Error Recovery**: Automatic error handling and recovery mechanisms

## Quick Start

### Basic Plugin System Setup

```cursed
yeet "plugin_vibes"

# Initialize the plugin system
slay init_application() lit {
    # Discover available plugins
    sus plugins := discover_plugins("./plugins")
    vibez.spill("Found plugins: " + plugins)
    
    # Load core plugins
    load_plugin("security")
    load_plugin("logging")
    load_plugin("auth")
    
    # Set up event handlers
    register_event_handler("logging", "error", "log_error")
    register_event_handler("auth", "login_attempt", "validate_login")
    
    damn based
}

# Clean shutdown
slay shutdown_application() lit {
    # Unload plugins in reverse order
    unload_plugin("auth")
    unload_plugin("logging")
    unload_plugin("security")
    
    damn based
}
```

### Simple Plugin Usage

```cursed
# Load a plugin and use its API
load_plugin("auth")

# Register an API function
register_plugin_api("auth", "validate_token", "auth_validate_function")

# Call the plugin API
sus result := call_plugin_api("auth", "validate_token", "{\"token\":\"abc123\"}")
vibez.spill("Validation result: " + result)

# Check plugin health
sus health := check_plugin_health("auth")
vibez.spill("Auth plugin health: " + health)
```

## Core Features

### 1. Plugin Discovery and Loading

```cursed
# Discover plugins in a directory
sus available_plugins := discover_plugins("./plugins")
# Returns: "auth,logging,cache,stats"

# Load a specific plugin
sus loaded := load_plugin("auth")
# Returns: based (true) if successful

# Check what plugins are loaded
sus active_plugins := list_loaded_plugins()
# Returns: "auth,logging,cache"

# Unload a plugin
sus unloaded := unload_plugin("auth")
# Returns: based (true) if successful
```

### 2. API Registration and Management

```cursed
# Register a plugin API
register_plugin_api("auth", "validate_credentials", "auth_validate")
register_plugin_api("auth", "refresh_token", "auth_refresh")

# Get all APIs for a plugin
sus apis := get_plugin_apis("auth")
# Returns: "validate_credentials,refresh_token"

# Call a plugin API
sus auth_result := call_plugin_api("auth", "validate_credentials", 
                                  "{\"username\":\"alice\",\"password\":\"secret\"}")

# Unregister an API
unregister_plugin_api("auth", "validate_credentials")
```

### 3. Event System

```cursed
# Register event handlers
register_event_handler("logging", "user_login", "log_user_login")
register_event_handler("analytics", "user_login", "track_login")

# Broadcast an event to all interested plugins
sus handlers_notified := broadcast_event("user_login", "{\"user\":\"alice\",\"time\":\"2023-12-01T10:00:00Z\"}")
vibez.spill("Event handled by " + tea(handlers_notified) + " plugins")

# Send event to specific plugin
send_event_to_plugin("logging", "error_occurred", "{\"level\":\"critical\",\"message\":\"Database connection failed\"}")

# Queue delayed events
queue_event("cleanup_temp_files", "{}", 300000)  # 5 minutes delay
process_event_queue()  # Process ready events
```

### 4. Security and Permissions

```cursed
# Create a secure sandbox for untrusted plugins
create_plugin_sandbox("untrusted_plugin", "{\"memory_limit\":1024,\"time_limit\":30}")

# Set specific permissions
set_plugin_permission("untrusted_plugin", "file_read", based)   # Allow file reading
set_plugin_permission("untrusted_plugin", "file_write", cap)   # Deny file writing
set_plugin_permission("untrusted_plugin", "network", cap)      # Deny network access

# Check permissions
sus can_read_files := check_plugin_permission("untrusted_plugin", "file_read")
sus can_access_network := check_plugin_permission("untrusted_plugin", "network")

# Get security report
sus security_report := validate_plugin_security("untrusted_plugin")
vibez.spill("Security status: " + security_report)
```

### 5. Plugin Communication

```cursed
# Send message between plugins
send_message("auth_plugin", "logging_plugin", "{\"event\":\"token_validated\",\"user\":\"alice\"}")

# Receive messages
sus message := receive_message("logging_plugin")
bestie string_length(message) > 0 {
    vibez.spill("Received: " + message)
}

# Broadcast message to all plugins
broadcast_message("system_plugin", "{\"maintenance_mode\":true}")

# Shared data storage
set_shared_data("user_count", "1547", "analytics_plugin")
sus count := get_shared_data("user_count", "dashboard_plugin")
sus all_keys := list_shared_keys("analytics_plugin")
```

### 6. Configuration Management

```cursed
# Load plugin configuration
sus config := load_plugin_config("auth")
# Returns: JSON configuration string

# Save plugin configuration
sus config_saved := save_plugin_config("auth", "{\"timeout\":30,\"debug\":false,\"encryption\":\"AES256\"}")

# Get/Set specific config values
sus timeout := get_config_value("auth", "timeout")           # Returns: "30"
set_config_value("auth", "debug", "true")

# Validate configuration
sus config_valid := validate_plugin_config("auth", "{\"timeout\":30}")
sus schema := get_config_schema("auth")
```

### 7. Health Monitoring

```cursed
# Check plugin health
sus health := check_plugin_health("auth")
# Returns: "healthy", "warning", "critical", or "unresponsive"

# Get detailed metrics
sus metrics := get_plugin_metrics("auth")
# Returns: JSON with api_calls, memory_usage, response_times, etc.

# Get performance data
sus performance := get_plugin_performance("auth")
# Returns: Detailed performance statistics

# Log plugin activity
log_plugin_activity("auth", "token_validated")

# Get plugin logs
sus logs := get_plugin_logs("auth", 50)  # Last 50 log entries
```

## Advanced Usage

### Plugin Lifecycle Management

```cursed
# Monitor plugin states
sus state := get_plugin_state("auth")  # "unloaded", "loading", "active", "suspended", "error"

# Set plugin state manually
set_plugin_state("auth", "suspended")

# Register lifecycle hooks
register_lifecycle_hook("auth", "pre_unload", "cleanup_auth_data")
register_lifecycle_hook("auth", "post_load", "initialize_auth_cache")

# Trigger lifecycle events
trigger_lifecycle_event("auth", "config_changed", "{\"new_timeout\":60}")
```

### Error Handling and Recovery

```cursed
# Check for plugin errors
sus errors := get_plugin_errors("problematic_plugin")
bestie string_length(errors) > 0 {
    vibez.spill("Plugin has errors: " + errors)
    clear_plugin_errors("problematic_plugin")
}

# Set up error handlers
set_error_handler("auth", "handle_auth_errors")

# Restart crashed plugins
sus restarted := restart_plugin("crashed_plugin")

# Recover with different modes
recover_plugin("problematic_plugin", "soft")    # Soft restart
recover_plugin("problematic_plugin", "reset")   # Reset to defaults
recover_plugin("problematic_plugin", "safe")    # Start in safe mode

# Quarantine malicious plugins
quarantine_plugin("suspicious_plugin", "detected_malware")
```

### Plugin Templates and Development

```cursed
# Create plugin templates
sus basic_template := create_plugin_template("my_plugin", "basic")
sus api_template := create_plugin_template("api_plugin", "api")
sus middleware_template := create_plugin_template("middleware_plugin", "middleware")

# Validate plugin structure
sus structure_valid := validate_plugin_structure("./plugins/my_plugin.plugin.csd")

# Package plugin for distribution
package_plugin("my_plugin", "./dist/my_plugin.plugin")
```

### Advanced Event Handling

```cursed
# Register plugin hooks for system events
register_plugin_hook("pre_request", "auth_plugin", "check_authentication")
register_plugin_hook("post_response", "logging_plugin", "log_response")

# Apply filters to data
sus sanitized_data := apply_plugin_filters("input_sanitization", user_input)
sus validated_data := apply_plugin_filters("data_validation", sanitized_data)

# Get plugins registered for hooks
sus auth_plugins := get_hook_plugins("pre_request")
```

### Dependency Management

```cursed
# Check plugin dependencies
sus auth_deps := get_plugin_dependencies("auth")  # Returns: "crypto,logging"
sus deps_satisfied := check_plugin_dependencies("auth")

# Install missing dependencies
install_plugin_dependency("auth", "crypto")

# Resolve plugin conflicts
sus conflicts := resolve_plugin_conflicts("auth_v1,auth_v2")
bestie string_length(conflicts) > 0 {
    vibez.spill("Plugin conflicts detected: " + conflicts)
}
```

## Security Best Practices

### 1. Sandbox Untrusted Plugins

```cursed
# Always create sandboxes for third-party plugins
create_plugin_sandbox("third_party_plugin", 
    "{\"memory_limit\":2048,\"time_limit\":30,\"network_restricted\":true}")

# Use minimal permissions
set_plugin_permission("third_party_plugin", "file_read", based)
set_plugin_permission("third_party_plugin", "file_write", cap)
set_plugin_permission("third_party_plugin", "network", cap)
set_plugin_permission("third_party_plugin", "exec", cap)
set_plugin_permission("third_party_plugin", "system", cap)
```

### 2. Validate Plugin Security

```cursed
# Regular security validation
sus security_status := validate_plugin_security("plugin_name")
bestie string_contains(security_status, "RISK:") {
    vibez.spill("Security risk detected!")
    quarantine_plugin("plugin_name", "security_risk_detected")
}
```

### 3. Monitor Plugin Behavior

```cursed
# Regular health checks
sus health := check_plugin_health("plugin_name")
bestie health == "critical" || health == "unresponsive" {
    restart_plugin("plugin_name")
}

# Monitor resource usage
sus metrics := get_plugin_metrics("plugin_name")
# Check memory usage, API call frequency, error rates
```

## Performance Considerations

### Plugin Loading Performance

- Plugin discovery: O(1) for cached results
- Plugin loading: O(1) per plugin
- API registration: O(1) per API
- Event broadcasting: O(n) where n = number of interested plugins

### Memory Management

- Plugins run in isolated memory spaces
- Shared data uses efficient key-value storage
- Event queues have configurable size limits
- Automatic cleanup on plugin unload

### Best Practices

1. **Load plugins on demand** rather than at startup
2. **Use event queuing** for non-critical events
3. **Monitor memory usage** of plugins regularly
4. **Implement graceful degradation** when plugins fail
5. **Use configuration caching** for frequently accessed settings

## Integration Examples

### Web Application with Authentication

```cursed
yeet "plugin_vibes"

slay setup_web_app() lit {
    # Load essential plugins
    load_plugin("security")
    load_plugin("auth")
    load_plugin("session")
    load_plugin("logging")
    
    # Configure authentication
    save_plugin_config("auth", "{\"token_lifetime\":3600,\"refresh_enabled\":true}")
    
    # Set up event handling
    register_event_handler("logging", "auth_attempt", "log_auth_attempt")
    register_event_handler("session", "user_login", "create_session")
    
    # Register API hooks
    register_plugin_hook("pre_request", "auth", "authenticate_request")
    register_plugin_hook("post_request", "logging", "log_request")
    
    damn based
}

slay handle_login(username tea, password tea) tea {
    # Broadcast login attempt
    broadcast_event("auth_attempt", "{\"username\":\"" + username + "\"}")
    
    # Validate credentials
    sus auth_result := call_plugin_api("auth", "validate_credentials", 
        "{\"username\":\"" + username + "\",\"password\":\"" + password + "\"}")
    
    bestie auth_result == "valid" {
        # Create session
        broadcast_event("user_login", "{\"username\":\"" + username + "\"}")
        damn "login_success"
    } else {
        broadcast_event("auth_failure", "{\"username\":\"" + username + "\"}")
        damn "login_failed"
    }
}
```

### Microservice Plugin Architecture

```cursed
slay setup_microservice() lit {
    # Discover and load service plugins
    sus services := discover_plugins("./services")
    sus service_list := string_split(services, ",")
    
    # Load each service plugin
    bestie string_contains(services, "user_service") {
        load_plugin("user_service")
        set_plugin_permission("user_service", "network", based)
        register_plugin_api("user_service", "get_user", "get_user_handler")
    }
    
    bestie string_contains(services, "notification_service") {
        load_plugin("notification_service")
        set_plugin_permission("notification_service", "network", based)
        register_plugin_api("notification_service", "send_notification", "send_notification_handler")
    }
    
    # Set up inter-service communication
    register_event_handler("notification_service", "user_created", "send_welcome_email")
    register_event_handler("logging_service", "*", "log_all_events")
    
    damn based
}
```

### Development and Testing Environment

```cursed
slay setup_test_environment() lit {
    # Load plugins in safe mode for testing
    sus test_plugins := discover_plugins("./test_plugins")
    
    # Create restricted sandboxes for all test plugins
    bestie string_contains(test_plugins, "test_plugin") {
        create_plugin_sandbox("test_plugin", "{\"memory_limit\":512,\"time_limit\":10}")
        load_plugin("test_plugin")
        
        # Minimal permissions for testing
        set_plugin_permission("test_plugin", "file_read", based)
        set_plugin_permission("test_plugin", "file_write", cap)
        set_plugin_permission("test_plugin", "network", cap)
    }
    
    # Set up test event monitoring
    register_event_handler("test_monitor", "*", "monitor_all_events")
    
    damn based
}

slay run_plugin_tests() lit {
    # Test all loaded plugins
    sus plugins := list_loaded_plugins()
    sus plugin_list := string_split(plugins, ",")
    
    # Check health of each plugin
    bestie string_length(plugins) > 0 {
        sus health := check_plugin_health("test_plugin")
        vibez.spill("Test plugin health: " + health)
        
        # Run plugin API tests
        sus test_result := call_plugin_api("test_plugin", "run_tests", "{}")
        vibez.spill("Test results: " + test_result)
    }
    
    damn based
}
```

## Error Handling Guide

### Common Error Scenarios

1. **Plugin Loading Failures**
   ```cursed
   sus loaded := load_plugin("nonexistent_plugin")
   bestie !loaded {
       vibez.spill("Failed to load plugin - check if plugin exists")
   }
   ```

2. **API Call Failures**
   ```cursed
   sus result := call_plugin_api("inactive_plugin", "test_api", "{}")
   bestie string_starts_with(result, "ERROR:") {
       vibez.spill("API call failed: " + result)
   }
   ```

3. **Permission Denied**
   ```cursed
   sus data := get_shared_data("restricted_key", "unauthorized_plugin")
   bestie data == "ERROR: Access denied" {
       vibez.spill("Plugin lacks permission for this data")
   }
   ```

4. **Plugin Health Issues**
   ```cursed
   sus health := check_plugin_health("problematic_plugin")
   bestie health == "critical" {
       vibez.spill("Plugin is in critical state - attempting recovery")
       recover_plugin("problematic_plugin", "reset")
   }
   ```

## Testing Your Plugin System

Use the comprehensive test suite provided:

```bash
# Run the plugin system tests
cargo run --bin cursed stdlib/plugin_vibes/test_plugin_vibes.csd

# Test both interpretation and compilation modes
cargo run --bin cursed -- compile stdlib/plugin_vibes/test_plugin_vibes.csd
./test_plugin_vibes
```

The test suite covers:
- Plugin discovery and loading
- API registration and calling
- Event system functionality
- Security and permissions
- Configuration management
- Health monitoring
- Error handling and recovery
- Inter-plugin communication
- Advanced features and edge cases

## Dependencies

- `testz`: Testing framework for validation
- No external plugin management libraries
- No FFI dependencies for core functionality
- Pure CURSED implementation for maximum portability

## Thread Safety

All plugin operations are designed to be thread-safe:
- Plugin state management uses atomic operations
- Event system supports concurrent access
- Shared data store has proper synchronization
- Plugin sandboxes provide isolation

## Platform Compatibility

The plugin system works consistently across all platforms:
- No platform-specific plugin handling
- Portable pure CURSED implementation
- Compatible with both interpretation and compilation modes
- Standard plugin interface specification

## Contributing

To contribute to the plugin system:

1. Follow CURSED coding conventions
2. Add comprehensive tests for new features
3. Update documentation for API changes
4. Ensure backward compatibility
5. Test in both interpretation and compilation modes

## License

This module is part of the CURSED standard library and follows the same licensing terms as the CURSED language implementation.
