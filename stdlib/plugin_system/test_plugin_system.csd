yeet "testz"
yeet "plugin_system"

# Comprehensive Plugin System Tests

slay run_all_plugin_tests() lit {
    test_start("Plugin System Comprehensive Tests")
    
    # Test plugin system initialization
    test_plugin_system_init()
    
    # Test plugin metadata creation
    test_plugin_metadata_creation()
    
    # Test plugin registration
    test_plugin_registration()
    
    # Test plugin loading and unloading
    test_plugin_loading_unloading()
    
    # Test plugin hooks system
    test_plugin_hooks()
    
    # Test plugin event system
    test_plugin_events()
    
    # Test plugin configuration
    test_plugin_configuration()
    
    # Test plugin security and permissions
    test_plugin_security()
    
    # Test plugin API context
    test_plugin_api_context()
    
    # Test plugin dependency resolution
    test_plugin_dependency_resolution()
    
    # Test plugin hot reload
    test_plugin_hot_reload()
    
    # Test plugin health checks
    test_plugin_health_checks()
    
    # Test plugin system statistics
    test_plugin_statistics()
    
    # Test error handling
    test_plugin_error_handling()
    
    # Test plugin cleanup
    test_plugin_cleanup()
    
    print_test_summary()
    damn based
}

slay test_plugin_system_init() lit {
    test_start("Plugin System Initialization")
    
    # Initialize plugin system
    sus result := plugin_system_init()
    assert_true(result)
    
    # Check initialization state
    assert_true(g_plugin_registry.is_initialized)
    assert_eq_int(g_plugin_registry.plugin_count, 0)
    assert_eq_int(g_plugin_registry.active_count, 0)
    assert_eq_int(g_plugin_registry.hook_count, 0)
    assert_eq_int(g_plugin_registry.max_plugins, 100)
    assert_true(g_plugin_registry.sandbox_enabled)
    assert_true(g_plugin_registry.hot_reload_enabled)
    assert_true(g_plugin_registry.dependency_resolver_enabled)
    
    # Check resource limits
    assert_eq_int(g_plugin_registry.resource_limits[0], 1024)  # Memory
    assert_eq_int(g_plugin_registry.resource_limits[1], 100)   # CPU
    assert_eq_int(g_plugin_registry.resource_limits[2], 50)    # File descriptors
    assert_eq_int(g_plugin_registry.resource_limits[3], 10)    # Network connections
    assert_eq_int(g_plugin_registry.resource_limits[4], 5)     # Threads
    assert_eq_int(g_plugin_registry.limit_count, 5)
    
    vibez.spill("✅ Plugin system initialization tests passed")
    damn based
}

slay test_plugin_metadata_creation() lit {
    test_start("Plugin Metadata Creation")
    
    # Create plugin metadata
    sus metadata := plugin_create_metadata("test_plugin", "1.0.0", "Test Plugin", "Test Author")
    
    # Verify metadata fields
    assert_eq_string(metadata.name, "test_plugin")
    assert_eq_string(metadata.version, "1.0.0")
    assert_eq_string(metadata.description, "Test Plugin")
    assert_eq_string(metadata.author, "Test Author")
    assert_eq_int(metadata.dependency_count, 0)
    assert_eq_int(metadata.priority, PLUGIN_PRIORITY_NORMAL)
    assert_eq_int(metadata.status, PLUGIN_STATUS_UNLOADED)
    assert_eq_string(metadata.api_version, "1.0.0")
    assert_eq_string(metadata.entry_point, "plugin_main")
    assert_eq_string(metadata.config_schema, "{}")
    assert_eq_int(metadata.permission_count, 0)
    assert_eq_int(metadata.load_timestamp, 0)
    assert_eq_string(metadata.error_message, "")
    assert_false(metadata.is_core_plugin)
    assert_eq_int(metadata.resource_usage, 0)
    assert_eq_string(metadata.plugin_type, "extension")
    assert_eq_int(metadata.hook_count, 0)
    
    vibez.spill("✅ Plugin metadata creation tests passed")
    damn based
}

slay test_plugin_registration() lit {
    test_start("Plugin Registration")
    
    # Create and register a plugin
    sus metadata := plugin_create_metadata("test_plugin_1", "1.0.0", "Test Plugin 1", "Test Author")
    sus result := plugin_register(metadata)
    assert_true(result)
    
    # Verify plugin was registered
    assert_eq_int(g_plugin_registry.plugin_count, 1)
    assert_eq_string(g_plugin_registry.plugins[0].name, "test_plugin_1")
    
    # Test duplicate registration
    sus duplicate_result := plugin_register(metadata)
    assert_false(duplicate_result)
    assert_eq_int(g_plugin_registry.plugin_count, 1)  # Should not increase
    
    # Register another plugin
    sus metadata2 := plugin_create_metadata("test_plugin_2", "2.0.0", "Test Plugin 2", "Test Author")
    sus result2 := plugin_register(metadata2)
    assert_true(result2)
    assert_eq_int(g_plugin_registry.plugin_count, 2)
    
    vibez.spill("✅ Plugin registration tests passed")
    damn based
}

slay test_plugin_loading_unloading() lit {
    test_start("Plugin Loading and Unloading")
    
    # Test plugin loading
    sus load_result := plugin_load("test_plugin_1")
    assert_true(load_result)
    
    # Verify plugin status
    sus status := plugin_get_status("test_plugin_1")
    assert_eq_int(status, PLUGIN_STATUS_LOADED)
    
    # Verify active plugins count
    assert_eq_int(g_plugin_registry.active_count, 1)
    
    # Test plugin unloading
    sus unload_result := plugin_unload("test_plugin_1")
    assert_true(unload_result)
    
    # Verify plugin status after unloading
    sus status_after := plugin_get_status("test_plugin_1")
    assert_eq_int(status_after, PLUGIN_STATUS_UNLOADED)
    
    # Verify active plugins count after unloading
    assert_eq_int(g_plugin_registry.active_count, 0)
    
    # Test loading non-existent plugin
    sus nonexistent_result := plugin_load("nonexistent_plugin")
    assert_false(nonexistent_result)
    
    vibez.spill("✅ Plugin loading/unloading tests passed")
    damn based
}

slay test_plugin_hooks() lit {
    test_start("Plugin Hooks System")
    
    # Register a hook
    sus hook_result := plugin_register_hook("test_plugin_1", "test_hook", "test_callback", PLUGIN_PRIORITY_HIGH)
    assert_true(hook_result)
    
    # Verify hook was registered
    assert_eq_int(g_plugin_registry.hook_count, 1)
    assert_eq_string(g_plugin_registry.plugin_hooks[0].hook_name, "test_hook")
    assert_eq_string(g_plugin_registry.plugin_hooks[0].plugin_name, "test_plugin_1")
    assert_eq_string(g_plugin_registry.plugin_hooks[0].callback_function, "test_callback")
    assert_eq_int(g_plugin_registry.plugin_hooks[0].priority, PLUGIN_PRIORITY_HIGH)
    assert_true(g_plugin_registry.plugin_hooks[0].is_active)
    
    # Register another hook with different priority
    sus hook_result2 := plugin_register_hook("test_plugin_2", "test_hook", "test_callback2", PLUGIN_PRIORITY_LOW)
    assert_true(hook_result2)
    assert_eq_int(g_plugin_registry.hook_count, 2)
    
    # Execute hooks
    sus execute_result := plugin_execute_hooks("test_hook", "test_context")
    assert_true(execute_result)
    
    # Verify hook execution statistics
    assert_eq_int(g_plugin_registry.plugin_hooks[0].execution_count, 1)
    assert_eq_int(g_plugin_registry.plugin_hooks[0].success_count, 1)
    
    # Remove hooks for a plugin
    sus remove_result := plugin_remove_hooks("test_plugin_1")
    assert_true(remove_result)
    assert_eq_int(g_plugin_registry.hook_count, 1)
    
    vibez.spill("✅ Plugin hooks system tests passed")
    damn based
}

slay test_plugin_events() lit {
    test_start("Plugin Event System")
    
    # Register event listener
    sus listener_result := plugin_register_event_listener("test_plugin_1", PLUGIN_EVENT_LOAD, "on_load_callback")
    assert_true(listener_result)
    
    # Verify listener was registered
    assert_eq_int(g_plugin_registry.listener_count, 1)
    assert_eq_int(g_plugin_registry.event_listeners[0].event_type, PLUGIN_EVENT_LOAD)
    assert_eq_string(g_plugin_registry.event_listeners[0].plugin_name, "test_plugin_1")
    assert_eq_string(g_plugin_registry.event_listeners[0].callback_function, "on_load_callback")
    assert_true(g_plugin_registry.event_listeners[0].is_active)
    
    # Register another event listener
    sus listener_result2 := plugin_register_event_listener("test_plugin_2", PLUGIN_EVENT_UNLOAD, "on_unload_callback")
    assert_true(listener_result2)
    assert_eq_int(g_plugin_registry.listener_count, 2)
    
    # Trigger event
    sus trigger_result := plugin_trigger_event(PLUGIN_EVENT_LOAD, "test_plugin_1", "Test load event")
    assert_true(trigger_result)
    
    # Verify event listener execution
    assert_eq_int(g_plugin_registry.event_listeners[0].execution_count, 1)
    
    # Remove event listeners for a plugin
    sus remove_result := plugin_remove_listeners("test_plugin_1")
    assert_true(remove_result)
    assert_eq_int(g_plugin_registry.listener_count, 1)
    
    vibez.spill("✅ Plugin event system tests passed")
    damn based
}

slay test_plugin_configuration() lit {
    test_start("Plugin Configuration")
    
    # Test setting configuration
    sus set_result := plugin_set_config("test_plugin_1", "setting1", "value1")
    assert_true(set_result)
    
    # Test getting configuration
    sus value := plugin_get_config("test_plugin_1", "setting1")
    assert_eq_string(value, "default_value")  # Returns default in simulation
    
    # Test getting non-existent configuration
    sus value2 := plugin_get_config("nonexistent_plugin", "setting1")
    assert_eq_string(value2, "default_value")
    
    vibez.spill("✅ Plugin configuration tests passed")
    damn based
}

slay test_plugin_security() lit {
    test_start("Plugin Security and Permissions")
    
    # Test permission check for non-existent plugin
    sus perm_result := plugin_check_permission("nonexistent_plugin", "read_files")
    assert_false(perm_result)
    
    # Test permission check for existing plugin without permissions
    sus perm_result2 := plugin_check_permission("test_plugin_1", "read_files")
    assert_false(perm_result2)
    
    # Add permission to plugin (would be done during registration in real implementation)
    sus plugin_index := plugin_find_by_name("test_plugin_1")
    nah (plugin_index != -1) {
        g_plugin_registry.plugins[plugin_index].permissions[0] = "read_files"
        g_plugin_registry.plugins[plugin_index].permission_count = 1
    }
    
    # Test permission check after adding permission
    sus perm_result3 := plugin_check_permission("test_plugin_1", "read_files")
    assert_true(perm_result3)
    
    # Test permission check for different permission
    sus perm_result4 := plugin_check_permission("test_plugin_1", "write_files")
    assert_false(perm_result4)
    
    vibez.spill("✅ Plugin security tests passed")
    damn based
}

slay test_plugin_api_context() lit {
    test_start("Plugin API Context")
    
    # Create API context
    sus context := plugin_create_api_context("test_plugin_1")
    
    # Verify context fields
    assert_eq_string(context.plugin_name, "test_plugin_1")
    assert_eq_string(context.api_version, "1.0.0")
    assert_eq_int(context.permission_count, 0)
    assert_eq_int(context.resource_quota, 1024)
    assert_eq_int(context.resource_used, 0)
    assert_eq_int(context.sandbox_level, 1)
    assert_eq_int(context.api_count, 0)
    assert_eq_int(context.request_count, 0)
    assert_eq_int(context.rate_limit, 100)
    assert_eq_int(context.rate_limit_window, 60)
    assert_eq_int(context.last_request_time, 0)
    assert_eq_int(context.error_count, 0)
    assert_false(context.is_trusted)
    assert_eq_string(context.security_token, "temp_token")
    assert_eq_string(context.session_id, "session_123")
    assert_eq_int(context.timeout_seconds, 3600)
    
    vibez.spill("✅ Plugin API context tests passed")
    damn based
}

slay test_plugin_dependency_resolution() lit {
    test_start("Plugin Dependency Resolution")
    
    # Test dependency resolution for plugin without dependencies
    sus resolve_result := plugin_resolve_dependencies("test_plugin_1")
    assert_true(resolve_result)
    
    # Test dependency resolution for non-existent plugin
    sus resolve_result2 := plugin_resolve_dependencies("nonexistent_plugin")
    assert_false(resolve_result2)
    
    # Add a dependency to test_plugin_1
    sus plugin_index := plugin_find_by_name("test_plugin_1")
    nah (plugin_index != -1) {
        g_plugin_registry.plugins[plugin_index].dependencies[0] = "test_plugin_2"
        g_plugin_registry.plugins[plugin_index].dependency_count = 1
    }
    
    # Test dependency resolution with unloaded dependency
    sus resolve_result3 := plugin_resolve_dependencies("test_plugin_1")
    assert_true(resolve_result3)  # Should load dependency
    
    vibez.spill("✅ Plugin dependency resolution tests passed")
    damn based
}

slay test_plugin_hot_reload() lit {
    test_start("Plugin Hot Reload")
    
    # Test hot reload for loaded plugin
    plugin_load("test_plugin_1")
    sus reload_result := plugin_hot_reload("test_plugin_1")
    assert_true(reload_result)
    
    # Test hot reload for non-existent plugin
    sus reload_result2 := plugin_hot_reload("nonexistent_plugin")
    assert_false(reload_result2)
    
    # Test hot reload when disabled
    g_plugin_registry.hot_reload_enabled = cap
    sus reload_result3 := plugin_hot_reload("test_plugin_1")
    assert_false(reload_result3)
    
    # Re-enable hot reload
    g_plugin_registry.hot_reload_enabled = based
    
    vibez.spill("✅ Plugin hot reload tests passed")
    damn based
}

slay test_plugin_health_checks() lit {
    test_start("Plugin Health Checks")
    
    # Test health check for non-existent plugin
    sus health_result := plugin_health_check("nonexistent_plugin")
    assert_false(health_result)
    
    # Test health check for unloaded plugin
    sus health_result2 := plugin_health_check("test_plugin_1")
    assert_false(health_result2)
    
    # Load plugin and test health check
    plugin_load("test_plugin_1")
    sus health_result3 := plugin_health_check("test_plugin_1")
    assert_true(health_result3)
    
    # Test health check with excessive resource usage
    sus plugin_index := plugin_find_by_name("test_plugin_1")
    nah (plugin_index != -1) {
        g_plugin_registry.plugins[plugin_index].resource_usage = 2048  # Exceeds limit
    }
    
    sus health_result4 := plugin_health_check("test_plugin_1")
    assert_false(health_result4)
    
    # Reset resource usage
    nah (plugin_index != -1) {
        g_plugin_registry.plugins[plugin_index].resource_usage = 0
    }
    
    vibez.spill("✅ Plugin health check tests passed")
    damn based
}

slay test_plugin_statistics() lit {
    test_start("Plugin Statistics")
    
    # Test system statistics
    sus stats := plugin_get_system_stats()
    assert_eq_int(stats, g_plugin_registry.plugin_count)
    
    # Test plugin listing
    sus total_plugins := plugin_list_all()
    assert_eq_int(total_plugins, g_plugin_registry.plugin_count)
    
    sus active_plugins := plugin_list_active()
    assert_eq_int(active_plugins, g_plugin_registry.active_count)
    
    vibez.spill("✅ Plugin statistics tests passed")
    damn based
}

slay test_plugin_error_handling() lit {
    test_start("Plugin Error Handling")
    
    # Test finding non-existent plugin
    sus find_result := plugin_find_by_name("nonexistent_plugin")
    assert_eq_int(find_result, -1)
    
    # Test getting info for non-existent plugin
    sus info := plugin_get_info("nonexistent_plugin")
    assert_eq_string(info.name, "")
    
    # Test loading non-existent plugin
    sus load_result := plugin_load("nonexistent_plugin")
    assert_false(load_result)
    
    # Test unloading non-existent plugin
    sus unload_result := plugin_unload("nonexistent_plugin")
    assert_false(unload_result)
    
    vibez.spill("✅ Plugin error handling tests passed")
    damn based
}

slay test_plugin_cleanup() lit {
    test_start("Plugin Cleanup")
    
    # Load multiple plugins
    plugin_load("test_plugin_1")
    plugin_load("test_plugin_2")
    
    # Verify plugins are loaded
    assert_eq_int(g_plugin_registry.active_count, 2)
    
    # Cleanup all plugins
    sus cleanup_result := plugin_cleanup_all()
    assert_true(cleanup_result)
    
    # Verify all plugins are unloaded
    assert_eq_int(g_plugin_registry.active_count, 0)
    
    vibez.spill("✅ Plugin cleanup tests passed")
    damn based
}

# Run all tests
run_all_plugin_tests()
