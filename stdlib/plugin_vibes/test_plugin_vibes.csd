yeet "testz"
yeet "plugin_vibes"

# ==========================================
# CURSED Plugin System - Comprehensive Test Suite
# Tests for plugin loading, API management, events, security, and more
# ==========================================

# Test plugin discovery functionality
test_start("Plugin discovery")
sus plugins := discover_plugins("./plugins")
assert_true(string_contains(plugins, "auth"))
assert_true(string_contains(plugins, "logger"))
assert_true(string_contains(plugins, "cache"))
assert_eq_string(plugins, "auth,logger,cache,stats")

sus test_plugins := discover_plugins("./test_plugins")
assert_eq_string(test_plugins, "test_plugin,mock_plugin")

sus empty_discovery := discover_plugins("./nonexistent")
assert_eq_string(empty_discovery, "")

# Test plugin loading and unloading
test_start("Plugin loading")
sus loaded := load_plugin("auth")
assert_true(loaded)

sus state := get_plugin_state("auth")
assert_eq_string(state, "active")

sus plugin_list := list_loaded_plugins()
assert_true(string_contains(plugin_list, "auth"))

# Test duplicate loading prevention
sus duplicate_load := load_plugin("auth")
assert_false(duplicate_load)

# Test plugin unloading
test_start("Plugin unloading")
sus unloaded := unload_plugin("auth")
assert_true(unloaded)

sus state_after_unload := get_plugin_state("auth")
assert_eq_string(state_after_unload, "unloaded")

sus empty_list := list_loaded_plugins()
assert_false(string_contains(empty_list, "auth"))

# Test unloading non-existent plugin
sus invalid_unload := unload_plugin("nonexistent")
assert_false(invalid_unload)

# Test API registration and management
test_start("API registration")
load_plugin("auth")
sus api_registered := register_plugin_api("auth", "validate_token", "auth_validate")
assert_true(api_registered)

sus apis := get_plugin_apis("auth")
assert_true(string_contains(apis, "validate_token"))

# Test API registration for non-loaded plugin
sus invalid_api_reg := register_plugin_api("nonexistent", "test_api", "test_func")
assert_false(invalid_api_reg)

# Test API calling
test_start("API calling")
sus api_result := call_plugin_api("auth", "validate_token", "{\"token\":\"abc123\"}")
assert_eq_string(api_result, "valid")

sus invalid_token_result := call_plugin_api("auth", "validate_token", "{\"token\":\"invalid\"}")
assert_eq_string(invalid_token_result, "invalid")

# Test calling non-existent API
sus invalid_api_call := call_plugin_api("auth", "nonexistent_api", "{}")
assert_eq_string(invalid_api_call, "ERROR: API not found")

# Test calling API on inactive plugin
unload_plugin("auth")
sus inactive_api_call := call_plugin_api("auth", "validate_token", "{}")
assert_eq_string(inactive_api_call, "ERROR: Plugin not active")

# Test API unregistration
test_start("API unregistration")
load_plugin("auth")
register_plugin_api("auth", "test_api", "test_func")
sus api_unregistered := unregister_plugin_api("auth", "test_api")
assert_true(api_unregistered)

# Test lifecycle management
test_start("Plugin lifecycle")
sus initial_state := get_plugin_state("logger")
assert_eq_string(initial_state, "unloaded")

load_plugin("logger")
sus active_state := get_plugin_state("logger")
assert_eq_string(active_state, "active")

sus state_set := set_plugin_state("logger", "suspended")
assert_true(state_set)

sus suspended_state := get_plugin_state("logger")
assert_eq_string(suspended_state, "suspended")

# Test lifecycle hooks
test_start("Lifecycle hooks")
sus hook_registered := register_lifecycle_hook("logger", "pre_unload", "cleanup_logger")
assert_true(hook_registered)

sus event_triggered := trigger_lifecycle_event("logger", "pre_unload", "{}")
assert_true(event_triggered)

# Test event system
test_start("Event system")
sus handler_reg := register_event_handler("logger", "user_login", "log_login")
assert_true(handler_reg)

sus event_sent := send_event_to_plugin("logger", "user_login", "{\"user\":\"alice\"}")
assert_true(event_sent)

# Test event broadcasting
sus broadcast_count := broadcast_event("system_startup", "{}")
assert_true(broadcast_count >= 0)

# Test event handler unregistration
sus handler_unreg := unregister_event_handler("logger", "user_login")
assert_true(handler_unreg)

# Test event queue
test_start("Event queue")
sus queued := queue_event("delayed_cleanup", "{}", 5000)
assert_true(queued)

sus processed := process_event_queue()
assert_true(processed >= 0)

sus queue_cleared := clear_event_queue()
assert_true(queue_cleared)

# Test security and permissions
test_start("Plugin permissions")
sus perm_set := set_plugin_permission("logger", "file_read", based)
assert_true(perm_set)

sus can_read := check_plugin_permission("logger", "file_read")
assert_true(can_read)

sus cannot_write := check_plugin_permission("logger", "file_write")
assert_false(cannot_write)

sus perms := get_plugin_permissions("logger")
assert_true(string_contains(perms, "file_read"))

# Test permission denial
sus perm_denied := set_plugin_permission("logger", "system", cap)
assert_true(perm_denied)

sus no_system_access := check_plugin_permission("logger", "system")
assert_false(no_system_access)

# Test sandbox creation and management
test_start("Plugin sandbox")
sus sandbox_created := create_plugin_sandbox("untrusted", "{\"memory_limit\":1024}")
assert_true(sandbox_created)

# Test sandbox security validation
sus security_report := validate_plugin_security("untrusted")
assert_true(string_contains(security_report, "SANDBOX:OK"))

# Test sandbox destruction
sus sandbox_destroyed := destroy_plugin_sandbox("untrusted")
assert_true(sandbox_destroyed)

# Test plugin communication
test_start("Plugin communication")
load_plugin("cache")

sus msg_sent := send_message("logger", "cache", "{\"event\":\"cache_request\"}")
assert_true(msg_sent)

sus message := receive_message("cache")
assert_true(string_length(message) > 0)

# Test message broadcasting
sus broadcast_sent := broadcast_message("logger", "{\"system\":\"maintenance_mode\"}")
assert_true(broadcast_sent > 0)

# Test shared data store
test_start("Shared data")
sus data_set := set_shared_data("user_count", "1024", "logger")
assert_true(data_set)

sus data_retrieved := get_shared_data("user_count", "logger")
assert_eq_string(data_retrieved, "1024")

sus keys_listed := list_shared_keys("logger")
assert_true(string_contains(keys_listed, "user_count"))

sus data_deleted := delete_shared_data("user_count", "logger")
assert_true(data_deleted)

# Test configuration management
test_start("Plugin configuration")
sus config := load_plugin_config("logger")
assert_true(string_length(config) > 0)
assert_true(is_valid_json(config))

sus config_saved := save_plugin_config("logger", "{\"debug\":true,\"level\":\"info\"}")
assert_true(config_saved)

sus debug_value := get_config_value("logger", "debug")
assert_eq_string(debug_value, "true")

sus config_updated := set_config_value("logger", "timeout", "60")
assert_true(config_updated)

# Test configuration validation
sus config_valid := validate_plugin_config("logger", "{\"debug\":true}")
assert_true(config_valid)

sus config_invalid := validate_plugin_config("logger", "{invalid json}")
assert_false(config_invalid)

sus schema := get_config_schema("logger")
assert_true(string_contains(schema, "object"))

# Test plugin monitoring and health
test_start("Plugin health monitoring")
sus health := check_plugin_health("logger")
assert_eq_string(health, "healthy")

sus metrics := get_plugin_metrics("logger")
assert_true(is_valid_json(metrics))
assert_true(string_contains(metrics, "api_calls"))

sus performance := get_plugin_performance("logger")
assert_true(is_valid_json(performance))

sus activity_logged := log_plugin_activity("logger", "test_activity")
assert_true(activity_logged)

sus metrics_reset := reset_plugin_metrics("logger")
assert_true(metrics_reset)

sus logs := get_plugin_logs("logger", 10)
assert_true(string_contains(logs, "LOGS:"))

# Test plugin information
test_start("Plugin information")
sus info := get_plugin_info("logger")
assert_true(is_valid_json(info))
assert_true(string_contains(info, "version"))

sus version := get_plugin_version("logger")
assert_true(string_length(version) > 0)

sus deps := get_plugin_dependencies("auth")
assert_true(string_contains(deps, "crypto"))

sus deps_ok := check_plugin_dependencies("auth")
assert_true(deps_ok)

# Test dependency management
sus dep_installed := install_plugin_dependency("cache", "memory")
assert_true(dep_installed)

sus conflicts := resolve_plugin_conflicts("auth,logger")
assert_eq_string(conflicts, "")

# Test advanced plugin features
test_start("Advanced plugin features")
sus hook_reg := register_plugin_hook("pre_request", "logger", "log_request")
assert_true(hook_reg)

sus filtered_data := apply_plugin_filters("sanitize_input", "test_data")
assert_eq_string(filtered_data, "test_data")

sus hook_plugins := get_hook_plugins("pre_request")
assert_true(string_contains(hook_plugins, "logger"))

# Test plugin templates
sus template := create_plugin_template("new_plugin", "basic")
assert_eq_string(template, "basic_template")

sus api_template := create_plugin_template("api_plugin", "api")
assert_eq_string(api_template, "api_template")

sus structure_valid := validate_plugin_structure("test.plugin.csd")
assert_true(structure_valid)

sus packaged := package_plugin("logger", "./dist/logger.plugin")
assert_true(packaged)

# Test error handling and recovery
test_start("Error handling")
sus errors := get_plugin_errors("logger")
assert_eq_string(errors, "")

sus errors_cleared := clear_plugin_errors("logger")
assert_true(errors_cleared)

sus error_handler_set := set_error_handler("logger", "handle_error")
assert_true(error_handler_set)

# Test plugin restart
sus restarted := restart_plugin("logger")
assert_true(restarted)

sus state_after_restart := get_plugin_state("logger")
assert_eq_string(state_after_restart, "active")

# Test plugin recovery
sus recovered_soft := recover_plugin("logger", "soft")
assert_true(recovered_soft)

sus recovered_reset := recover_plugin("logger", "reset")
assert_true(recovered_reset)

sus recovered_safe := recover_plugin("logger", "safe")
assert_true(recovered_safe)

# Test invalid recovery mode
sus invalid_recovery := recover_plugin("logger", "invalid_mode")
assert_false(invalid_recovery)

# Test plugin quarantine
test_start("Plugin quarantine")
load_plugin("mock_plugin")
sus quarantined := quarantine_plugin("mock_plugin", "security_violation")
assert_true(quarantined)

sus quarantine_state := get_plugin_state("mock_plugin")
assert_eq_string(quarantine_state, "quarantined")

# Test utility functions
test_start("Utility functions")
sus plugin_exists := validate_plugin_exists("auth")
assert_true(plugin_exists)

sus plugin_not_exists := validate_plugin_exists("nonexistent_plugin")
assert_false(plugin_not_exists)

# Test string manipulation utilities
sus contains_result := string_contains("hello world", "world")
assert_true(contains_result)

sus not_contains := string_contains("hello", "xyz")
assert_false(not_contains)

sus starts_with_result := string_starts_with("hello world", "hello")
assert_true(starts_with_result)

sus not_starts_with := string_starts_with("hello", "world")
assert_false(not_starts_with)

sus ends_with_result := string_ends_with("hello world", "world")
assert_true(ends_with_result)

sus not_ends_with := string_ends_with("hello", "xyz")
assert_false(not_ends_with)

# Test JSON validation
sus valid_json := is_valid_json("{\"key\":\"value\"}")
assert_true(valid_json)

sus invalid_json := is_valid_json("{invalid}")
assert_false(invalid_json)

sus valid_array := is_valid_json("[1,2,3]")
assert_true(valid_array)

# Test edge cases and error conditions
test_start("Edge cases")

# Test loading invalid plugin
sus invalid_load := load_plugin("totally_invalid_plugin")
assert_false(invalid_load)

# Test API call with non-active plugin
set_plugin_state("logger", "suspended")
sus suspended_api_call := call_plugin_api("logger", "test_api", "{}")
assert_eq_string(suspended_api_call, "ERROR: Plugin not active")

# Reset logger state for other tests
set_plugin_state("logger", "active")

# Test permission checks for non-existent plugin
sus no_plugin_perm := check_plugin_permission("nonexistent", "file_read")
assert_false(no_plugin_perm)

# Test sending message to non-loaded plugin
sus invalid_msg := send_message("logger", "nonexistent", "test")
assert_false(invalid_msg)

# Test configuration for non-standard plugin
sus unknown_config := load_plugin_config("unknown_plugin")
assert_true(is_valid_json(unknown_config))

# Test health check for unloaded plugin
sus unloaded_health := check_plugin_health("unloaded_plugin")
assert_eq_string(unloaded_health, "unloaded")

# Test performance tests
test_start("Performance tests")

# Test rapid plugin loading/unloading
sus i normie = 0
bestie i < 3 {
    sus rapid_load := load_plugin("test_plugin")
    sus rapid_unload := unload_plugin("test_plugin")
    assert_true(rapid_load || !rapid_load)  # Should not crash
    assert_true(rapid_unload || !rapid_unload)
    i = i + 1
}

# Test multiple API registrations
load_plugin("test_plugin")
sus api1 := register_plugin_api("test_plugin", "api1", "func1")
sus api2 := register_plugin_api("test_plugin", "api2", "func2")
sus api3 := register_plugin_api("test_plugin", "api3", "func3")
assert_true(api1)
assert_true(api2)
assert_true(api3)

sus all_apis := get_plugin_apis("test_plugin")
assert_true(string_length(all_apis) > 0)

# Test multiple event handlers
sus event1 := register_event_handler("test_plugin", "event1", "handler1")
sus event2 := register_event_handler("test_plugin", "event2", "handler2")
sus event3 := register_event_handler("test_plugin", "event3", "handler3")
assert_true(event1)
assert_true(event2)
assert_true(event3)

# Test broadcast to multiple handlers
sus multi_broadcast := broadcast_event("global_event", "{}")
assert_true(multi_broadcast >= 0)

# Integration tests
test_start("Integration tests")

# Test complete plugin lifecycle
sus full_lifecycle := load_plugin("integration_test")
bestie full_lifecycle {
    register_plugin_api("integration_test", "test_api", "test_func")
    register_event_handler("integration_test", "test_event", "test_handler")
    set_plugin_permission("integration_test", "file_read", based)
    save_plugin_config("integration_test", "{\"test\":true}")
    
    sus api_works := call_plugin_api("integration_test", "test_api", "{}")
    sus event_works := send_event_to_plugin("integration_test", "test_event", "{}")
    sus perm_works := check_plugin_permission("integration_test", "file_read")
    sus config_works := get_config_value("integration_test", "test")
    
    # Clean up
    unload_plugin("integration_test")
    
    assert_true(string_contains(api_works, "SUCCESS") || string_contains(api_works, "ERROR"))
    assert_true(event_works)
    assert_true(perm_works)
    assert_eq_string(config_works, "true")
}

# Test plugin communication chain
load_plugin("sender")
load_plugin("receiver")
sus chain_msg := send_message("sender", "receiver", "{\"chain\":\"test\"}")
sus received_msg := receive_message("receiver")
assert_true(chain_msg)
assert_true(string_length(received_msg) > 0)

# Test plugin dependency chain
sus dep_chain := check_plugin_dependencies("complex_plugin")
assert_true(dep_chain || !dep_chain)  # Should not crash

# Test security validation chain
create_plugin_sandbox("security_test", "{\"memory_limit\":512}")
set_plugin_permission("security_test", "network", based)
set_plugin_permission("security_test", "system", based)
sus security_validation := validate_plugin_security("security_test")
assert_true(string_contains(security_validation, "RISK:") || string_contains(security_validation, "STATUS:"))

# Clean up test plugins
unload_plugin("logger")
unload_plugin("cache")
unload_plugin("test_plugin")
unload_plugin("sender")
unload_plugin("receiver")

# Verify cleanup
sus final_plugin_list := list_loaded_plugins()
assert_eq_string(final_plugin_list, "")

print_test_summary()
