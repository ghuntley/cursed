# plugin_vibes (Plugin System)

## Overview
`plugin_vibes` provides a comprehensive plugin system for CURSED applications, enabling dynamic loading, management, and execution of plugins. This module implements secure plugin sandboxing, API registration, event handling, and lifecycle management. All functions are implemented in pure CURSED without external dependencies.

## Core Plugin Operations

### Plugin Discovery and Loading

#### Plugin Discovery
```cursed
slay discover_plugins(directory tea) tea
```
Discovers available plugins in the specified directory.

**Parameters:**
- `directory tea`: Path to the plugin directory

**Returns:**
- `tea`: Comma-separated list of discovered plugin names

**Discovery Rules:**
- Looks for `.plugin.💀` files
- Validates plugin metadata
- Checks for required plugin functions
- Returns empty string if no plugins found

#### Plugin Loading
```cursed
slay load_plugin(plugin_name tea) lit
slay unload_plugin(plugin_name tea) lit
```

**Parameters:**
- `plugin_name tea`: Name of the plugin to load/unload

**Returns:**
- `lit`: `based` if successful, `cap` if failed

**Loading Process:**
1. Validates plugin file exists
2. Checks plugin permissions
3. Initializes plugin sandbox
4. Registers plugin APIs
5. Executes plugin initialization

**Examples:**
```cursed
sus plugins := discover_plugins("./plugins")     # Returns "auth,logging,cache"
sus loaded := load_plugin("auth")                # Returns based
sus unloaded := unload_plugin("auth")            # Returns based
```

### Plugin Registration and API Management

#### API Registration
```cursed
slay register_plugin_api(plugin_name tea, api_name tea, function_ptr tea) lit
slay unregister_plugin_api(plugin_name tea, api_name tea) lit
```

**Parameters:**
- `plugin_name tea`: Name of the plugin
- `api_name tea`: Name of the API function
- `function_ptr tea`: Function pointer or identifier

**Returns:**
- `lit`: `based` if successful, `cap` if failed

#### API Discovery
```cursed
slay get_plugin_apis(plugin_name tea) tea
slay call_plugin_api(plugin_name tea, api_name tea, params tea) tea
```

**Parameters:**
- `plugin_name tea`: Name of the plugin
- `api_name tea`: Name of the API function
- `params tea`: JSON-encoded parameters

**Returns:**
- `tea`: API list or function result

**Examples:**
```cursed
sus registered := register_plugin_api("auth", "validate_token", "auth_validate")
sus apis := get_plugin_apis("auth")              # Returns "validate_token,refresh_token"
sus result := call_plugin_api("auth", "validate_token", "{\"token\":\"abc123\"}")
```

## Plugin Lifecycle Management

### Lifecycle States
```cursed
slay get_plugin_state(plugin_name tea) tea
slay set_plugin_state(plugin_name tea, state tea) lit
```

**Plugin States:**
- `"unloaded"`: Plugin not loaded
- `"loading"`: Plugin initialization in progress
- `"active"`: Plugin loaded and active
- `"suspended"`: Plugin temporarily disabled
- `"error"`: Plugin in error state
- `"unloading"`: Plugin shutdown in progress

**Parameters:**
- `plugin_name tea`: Name of the plugin
- `state tea`: New state to set

**Returns:**
- `tea`: Current state or `lit` for set operations

### Lifecycle Hooks
```cursed
slay register_lifecycle_hook(plugin_name tea, hook_type tea, callback tea) lit
slay trigger_lifecycle_event(plugin_name tea, event_type tea, data tea) lit
```

**Hook Types:**
- `"pre_load"`: Before plugin loading
- `"post_load"`: After successful loading
- `"pre_unload"`: Before plugin unloading
- `"post_unload"`: After successful unloading
- `"on_error"`: When plugin encounters error

**Examples:**
```cursed
sus state := get_plugin_state("auth")            # Returns "active"
sus hook_reg := register_lifecycle_hook("auth", "pre_unload", "cleanup_auth")
sus event_sent := trigger_lifecycle_event("auth", "config_changed", "{\"debug\":true}")
```

## Event System and Hooks

### Event Registration
```cursed
slay register_event_handler(plugin_name tea, event_name tea, handler tea) lit
slay unregister_event_handler(plugin_name tea, event_name tea) lit
```

**Parameters:**
- `plugin_name tea`: Name of the plugin
- `event_name tea`: Name of the event to handle
- `handler tea`: Handler function identifier

**Returns:**
- `lit`: `based` if successful, `cap` if failed

### Event Broadcasting
```cursed
slay broadcast_event(event_name tea, data tea) normie
slay send_event_to_plugin(plugin_name tea, event_name tea, data tea) lit
```

**Parameters:**
- `event_name tea`: Name of the event
- `data tea`: Event data (JSON-encoded)
- `plugin_name tea`: Target plugin name

**Returns:**
- `normie`: Number of plugins that handled the event
- `lit`: `based` if valid, `cringe` otherwise

### Event Queue Management
```cursed
slay queue_event(event_name tea, data tea, delay_ms normie) lit
slay process_event_queue() normie
slay clear_event_queue() lit
```

**Examples:**
```cursed
sus handler_reg := register_event_handler("logger", "user_login", "log_login")
sus broadcast_count := broadcast_event("user_login", "{\"user\":\"alice\"}")
sus queued := queue_event("delayed_cleanup", "{}", 5000)
sus processed := process_event_queue()           # Returns number of events processed
```

## Security and Sandboxing

### Permission System
```cursed
slay set_plugin_permission(plugin_name tea, permission tea, allowed lit) lit
slay check_plugin_permission(plugin_name tea, permission tea) lit
slay get_plugin_permissions(plugin_name tea) tea
```

**Permission Types:**
- `"file_read"`: Read file system access
- `"file_write"`: Write file system access
- `"network"`: Network access
- `"exec"`: Execute external commands
- `"memory"`: Advanced memory operations
- `"system"`: System-level operations

**Parameters:**
- `plugin_name tea`: Name of the plugin
- `permission tea`: Permission type
- `allowed lit`: Whether permission is granted

**Returns:**
- `lit`: Permission status or operation success

### Sandbox Management
```cursed
slay create_plugin_sandbox(plugin_name tea, restrictions tea) lit
slay destroy_plugin_sandbox(plugin_name tea) lit
slay validate_plugin_security(plugin_name tea) tea
```

**Parameters:**
- `plugin_name tea`: Name of the plugin
- `restrictions tea`: JSON-encoded security restrictions

**Returns:**
- `lit`: Operation success
- `tea`: Security validation report

**Examples:**
```cursed
sus perm_set := set_plugin_permission("untrusted", "network", cap)
sus can_read := check_plugin_permission("logger", "file_read")
sus sandbox := create_plugin_sandbox("untrusted", "{\"memory_limit\":1024}")
sus security := validate_plugin_security("auth")  # Returns security report
```

## Plugin Communication

### Inter-Plugin Messaging
```cursed
slay send_message(from_plugin tea, to_plugin tea, message tea) lit
slay receive_message(plugin_name tea) tea
slay broadcast_message(from_plugin tea, message tea) normie
```

**Parameters:**
- `from_plugin tea`: Sender plugin name
- `to_plugin tea`: Recipient plugin name
- `message tea`: Message content (JSON-encoded)
- `plugin_name tea`: Plugin checking for messages

**Returns:**
- `lit`: Send operation success
- `tea`: Received message or empty string
- `normie`: Number of plugins that received broadcast

### Shared Data Store
```cursed
slay set_shared_data(key tea, value tea, plugin_name tea) lit
slay get_shared_data(key tea, plugin_name tea) tea
slay delete_shared_data(key tea, plugin_name tea) lit
slay list_shared_keys(plugin_name tea) tea
```

**Parameters:**
- `key tea`: Data key
- `value tea`: Data value
- `plugin_name tea`: Plugin requesting access

**Returns:**
- `lit`: Operation success
- `tea`: Retrieved value or key list

**Access Control:**
- Plugins can only access their own data by default
- Shared data requires explicit permission
- Global data accessible with "system" permission

**Examples:**
```cursed
sus msg_sent := send_message("auth", "logger", "{\"event\":\"login_attempt\"}")
sus message := receive_message("logger")         # Returns message or ""
sus data_set := set_shared_data("user_count", "1024", "stats")
sus count := get_shared_data("user_count", "dashboard")
```

## Plugin Configuration

### Configuration Management
```cursed
slay load_plugin_config(plugin_name tea) tea
slay save_plugin_config(plugin_name tea, config tea) lit
slay get_config_value(plugin_name tea, key tea) tea
slay set_config_value(plugin_name tea, key tea, value tea) lit
```

**Parameters:**
- `plugin_name tea`: Name of the plugin
- `config tea`: JSON-encoded configuration
- `key tea`: Configuration key
- `value tea`: Configuration value

**Returns:**
- `tea`: Configuration data or value
- `lit`: Operation success

### Configuration Validation
```cursed
slay validate_plugin_config(plugin_name tea, config tea) lit
slay get_config_schema(plugin_name tea) tea
```

**Examples:**
```cursed
sus config := load_plugin_config("auth")         # Returns JSON config
sus debug := get_config_value("auth", "debug_mode")
sus updated := set_config_value("auth", "timeout", "30")
sus valid := validate_plugin_config("auth", "{\"timeout\":30}")
```

## Plugin Monitoring and Health

### Health Checks
```cursed
slay check_plugin_health(plugin_name tea) tea
slay get_plugin_metrics(plugin_name tea) tea
slay reset_plugin_metrics(plugin_name tea) lit
```

**Health Status:**
- `"healthy"`: Plugin operating normally
- `"warning"`: Plugin has minor issues
- `"critical"`: Plugin has serious problems
- `"unresponsive"`: Plugin not responding

### Performance Monitoring
```cursed
slay get_plugin_performance(plugin_name tea) tea
slay log_plugin_activity(plugin_name tea, activity tea) lit
slay get_plugin_logs(plugin_name tea, limit normie) tea
```

**Metrics Tracked:**
- API call count and response times
- Memory usage
- Event handling performance
- Error rates
- Resource consumption

**Examples:**
```cursed
sus health := check_plugin_health("auth")        # Returns "healthy"
sus metrics := get_plugin_metrics("auth")        # Returns JSON metrics
sus perf := get_plugin_performance("cache")      # Returns performance data
sus logged := log_plugin_activity("auth", "token_validated")
```

## Plugin Metadata and Information

### Plugin Information
```cursed
slay get_plugin_info(plugin_name tea) tea
slay list_loaded_plugins() tea
slay get_plugin_version(plugin_name tea) tea
slay get_plugin_dependencies(plugin_name tea) tea
```

**Plugin Info Fields:**
- `name`: Plugin name
- `version`: Plugin version
- `author`: Plugin author
- `description`: Plugin description
- `dependencies`: Required dependencies
- `permissions`: Required permissions

### Dependency Management
```cursed
slay check_plugin_dependencies(plugin_name tea) lit
slay install_plugin_dependency(plugin_name tea, dependency tea) lit
slay resolve_plugin_conflicts(plugin_list tea) tea
```

**Examples:**
```cursed
sus info := get_plugin_info("auth")              # Returns plugin metadata
sus plugins := list_loaded_plugins()             # Returns "auth,logger,cache"
sus version := get_plugin_version("auth")        # Returns "1.2.3"
sus deps_ok := check_plugin_dependencies("auth") # Returns based
```

## Advanced Plugin Features

### Plugin Hooks and Filters
```cursed
slay register_plugin_hook(hook_name tea, plugin_name tea, callback tea) lit
slay apply_plugin_filters(filter_name tea, data tea) tea
slay get_hook_plugins(hook_name tea) tea
```

**Common Hooks:**
- `"pre_request"`: Before request processing
- `"post_response"`: After response generation
- `"user_authentication"`: During user auth
- `"data_validation"`: During data validation

### Plugin Templates and Scaffolding
```cursed
slay create_plugin_template(plugin_name tea, template_type tea) tea
slay validate_plugin_structure(plugin_path tea) lit
slay package_plugin(plugin_name tea, output_path tea) lit
```

**Template Types:**
- `"basic"`: Basic plugin template
- `"api"`: API extension plugin
- `"middleware"`: Middleware plugin
- `"service"`: Background service plugin

**Examples:**
```cursed
sus hook_reg := register_plugin_hook("pre_request", "auth", "check_auth")
sus filtered := apply_plugin_filters("sanitize_input", user_data)
sus template := create_plugin_template("new_plugin", "api")
sus packaged := package_plugin("auth", "./dist/auth.plugin")
```

## Error Handling and Recovery

### Error Management
```cursed
slay get_plugin_errors(plugin_name tea) tea
slay clear_plugin_errors(plugin_name tea) lit
slay set_error_handler(plugin_name tea, handler tea) lit
```

**Error Types:**
- `"load_error"`: Plugin loading failed
- `"runtime_error"`: Plugin runtime exception
- `"permission_error"`: Security permission denied
- `"dependency_error"`: Missing dependency
- `"config_error"`: Configuration problem

### Recovery Mechanisms
```cursed
slay restart_plugin(plugin_name tea) lit
slay recover_plugin(plugin_name tea, recovery_mode tea) lit
slay quarantine_plugin(plugin_name tea, reason tea) lit
```

**Recovery Modes:**
- `"soft"`: Restart plugin in same state
- `"reset"`: Reset plugin to default state
- `"safe"`: Start plugin in safe mode

**Examples:**
```cursed
sus errors := get_plugin_errors("problematic")   # Returns error list
sus cleared := clear_plugin_errors("problematic")
sus restarted := restart_plugin("crashed")
sus quarantined := quarantine_plugin("malicious", "security_violation")
```

## Performance Characteristics

### Time Complexity
- Plugin loading: O(1) for individual plugins
- Event broadcasting: O(n) where n is number of plugins
- API calls: O(1) for direct calls
- Configuration access: O(1) for cached configs

### Memory Usage
- Plugin isolation prevents memory leaks between plugins
- Shared data store uses efficient key-value storage
- Event queue has configurable size limits
- Metrics collection uses rolling windows

### Security Guarantees
- Plugins run in isolated sandboxes
- Permission system prevents unauthorized access
- Inter-plugin communication is monitored
- All plugin operations are logged

## Usage Patterns

### Basic Plugin System Setup
```cursed
yeet "plugin_vibes"

# Initialize plugin system
slay init_plugin_system() lit {
    sus plugins := discover_plugins("./plugins")
    
    # Load core plugins first
    load_plugin("security")
    load_plugin("logging")
    
    # Load application plugins
    bestie string_contains(plugins, "auth") {
        load_plugin("auth")
    }
    
    damn based
}

# Shutdown plugin system
slay shutdown_plugin_system() lit {
    sus plugins := list_loaded_plugins()
    
    # Unload in reverse order
    bestie string_contains(plugins, "auth") {
        unload_plugin("auth")
    }
    
    unload_plugin("logging")
    unload_plugin("security")
    
    damn based
}
```

### Plugin Event Handling
```cursed
# Set up event system
slay setup_events() lit {
    # Register core event handlers
    register_event_handler("logger", "error", "log_error")
    register_event_handler("auth", "login_attempt", "validate_login")
    register_event_handler("stats", "user_action", "track_action")
    
    damn based
}

# Handle user authentication
slay handle_user_login(username tea, password tea) tea {
    # Broadcast login attempt event
    sus event_data := "{\"username\":\"" + username + "\",\"timestamp\":\"" + get_timestamp() + "\"}"
    broadcast_event("login_attempt", event_data)
    
    # Call auth plugin
    sus auth_result := call_plugin_api("auth", "validate_credentials", 
                                      "{\"username\":\"" + username + "\",\"password\":\"" + password + "\"}")
    
    # Broadcast result
    bestie auth_result == "valid" {
        broadcast_event("user_login", event_data)
        damn "success"
    } else {
        broadcast_event("login_failed", event_data)
        damn "failed"
    }
}
```

### Secure Plugin Loading
```cursed
# Load plugin with security checks
slay load_plugin_secure(plugin_name tea, permissions tea) lit {
    # Create sandbox first
    sus restrictions := "{\"memory_limit\":2048,\"time_limit\":30}"
    sus sandbox := create_plugin_sandbox(plugin_name, restrictions)
    
    ready !sandbox {
        damn nah
    }
    
    # Set permissions
    bestie string_contains(permissions, "file_read") {
        set_plugin_permission(plugin_name, "file_read", based)
    }
    
    bestie string_contains(permissions, "network") {
        set_plugin_permission(plugin_name, "network", based)
    }
    
    # Load plugin
    sus loaded := load_plugin(plugin_name)
    
    ready !loaded {
        destroy_plugin_sandbox(plugin_name)
        damn nah
    }
    
    # Validate security
    sus security_report := validate_plugin_security(plugin_name)
    ready string_contains(security_report, "RISK") {
        quarantine_plugin(plugin_name, "security_risk")
        damn nah
    }
    
    damn based
}
```

## Implementation Notes

### Pure CURSED Implementation
- No external plugin framework dependencies
- Compatible with both interpretation and compilation modes
- Self-contained plugin management system
- Uses CURSED string and data types throughout

### Thread Safety
- Plugin operations are thread-safe
- Event system supports concurrent access
- Shared data store has proper locking
- Plugin sandboxes are isolated

### Error Handling
- All errors returned as strings with "ERROR:" prefix
- Plugin failures don't crash the system
- Comprehensive error logging and recovery
- Graceful degradation on plugin issues

## Testing Strategy

### Unit Tests
```cursed
yeet "testz"
yeet "plugin_vibes"

# Test plugin discovery
test_start("Plugin discovery")
sus plugins := discover_plugins("./test_plugins")
assert_true(string_contains(plugins, "test_plugin"))

# Test plugin loading
test_start("Plugin loading")
sus loaded := load_plugin("test_plugin")
assert_true(loaded)

# Test API registration
test_start("API registration")
sus registered := register_plugin_api("test_plugin", "test_api", "test_function")
assert_true(registered)

# Test event system
test_start("Event system")
sus handler_reg := register_event_handler("test_plugin", "test_event", "test_handler")
assert_true(handler_reg)

sus event_count := broadcast_event("test_event", "{\"test\":true}")
assert_eq_int(event_count, 1)

print_test_summary()
```

### Integration Tests
- Multi-plugin interaction testing
- Security and permission validation
- Performance and memory usage testing
- Error recovery and resilience testing

## Dependencies

- `testz`: Testing framework for module validation
- No external plugin management libraries
- No FFI dependencies for core functionality

## Security Considerations

- Plugin sandboxing prevents system compromise
- Permission system enforces access control
- All plugin communications are logged
- Malicious plugin detection and quarantine
- Secure plugin loading and validation

## Compatibility

### Plugin Standards
- Standard plugin interface specification
- Common plugin metadata format
- Compatible with CURSED module system
- Extensible for custom plugin types

### Platform Support
- Consistent behavior across all platforms
- No platform-specific plugin handling
- Portable pure CURSED implementation
- Compatible with both interpretation and compilation modes
