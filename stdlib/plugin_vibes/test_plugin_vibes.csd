yeet "testz"
yeet "plugin_vibes"

fr fr Plugin Discovery Tests
test_start("Plugin discovery - empty directory")
sus empty_plugins := discover_plugins("")
assert_eq_string(empty_plugins, "")

test_start("Plugin discovery - valid directory")
sus plugins := discover_plugins("./test_plugins")
assert_eq_string(plugins, "auth,logger,cache")

fr fr Plugin Loading Tests
test_start("Plugin loading - valid plugin")
sus loaded := load_plugin("test_auth")
assert_true(loaded)

test_start("Plugin loading - empty name")
sus empty_load := load_plugin("")
assert_false(empty_load)

test_start("Plugin unloading - valid plugin")
load_plugin("test_unload")
sus unloaded := unload_plugin("test_unload")
assert_true(unloaded)

test_start("Plugin unloading - non-existent plugin")
sus unload_missing := unload_plugin("non_existent")
assert_false(unload_missing)

fr fr API Registration Tests
test_start("API registration - valid registration")
load_plugin("api_test")
sus registered := register_plugin_api("api_test", "validate", "validate_func")
assert_true(registered)

test_start("API registration - empty parameters")
sus empty_reg := register_plugin_api("", "", "")
assert_false(empty_reg)

test_start("API registration - unloaded plugin")
sus unloaded_reg := register_plugin_api("unloaded_plugin", "test_api", "func")
assert_false(unloaded_reg)

test_start("API unregistration")
register_plugin_api("api_test", "temp_api", "temp_func")
sus unreg := unregister_plugin_api("api_test", "temp_api")
assert_true(unreg)

test_start("Get plugin APIs")
register_plugin_api("api_test", "validate_token", "validate_func")
register_plugin_api("api_test", "refresh_token", "refresh_func")
sus apis := get_plugin_apis("api_test")
assert_true(string_contains(apis, "validate_token"))

test_start("Call plugin API - valid call")
register_plugin_api("api_test", "validate_token", "validate_func")
sus result := call_plugin_api("api_test", "validate_token", "{\"token\":\"abc123\"}")
assert_true(string_contains(result, "valid"))

test_start("Call plugin API - non-existent API")
sus error_result := call_plugin_api("api_test", "non_existent", "{}")
assert_true(string_contains(error_result, "ERROR"))

fr fr Lifecycle Management Tests
test_start("Get plugin state - loaded plugin")
load_plugin("state_test")
sus state := get_plugin_state("state_test")
assert_eq_string(state, "active")

test_start("Get plugin state - unloaded plugin")
sus unloaded_state := get_plugin_state("never_loaded")
assert_eq_string(unloaded_state, "unloaded")

test_start("Set plugin state")
load_plugin("state_change_test")
sus state_set := set_plugin_state("state_change_test", "suspended")
assert_true(state_set)

test_start("Register lifecycle hook")
load_plugin("hook_test")
sus hook_reg := register_lifecycle_hook("hook_test", "pre_unload", "cleanup_func")
assert_true(hook_reg)

test_start("Trigger lifecycle event")
register_lifecycle_hook("hook_test", "config_changed", "config_handler")
sus event_triggered := trigger_lifecycle_event("hook_test", "config_changed", "{\"debug\":true}")
assert_true(event_triggered)

fr fr Event System Tests
test_start("Register event handler")
load_plugin("event_test")
sus handler_reg := register_event_handler("event_test", "user_login", "login_handler")
assert_true(handler_reg)

test_start("Register event handler - empty parameters")
sus empty_handler := register_event_handler("", "", "")
assert_false(empty_handler)

test_start("Unregister event handler")
register_event_handler("event_test", "temp_event", "temp_handler")
sus handler_unreg := unregister_event_handler("event_test", "temp_event")
assert_true(handler_unreg)

test_start("Broadcast event")
register_event_handler("event_test", "test_broadcast", "broadcast_handler")
sus broadcast_count := broadcast_event("test_broadcast", "{\"test\":true}")
assert_eq_int(broadcast_count, 3)

test_start("Broadcast event - no handlers")
sus no_handler_count := broadcast_event("no_handlers", "{}")
assert_eq_int(no_handler_count, 0)

test_start("Send event to specific plugin")
register_event_handler("event_test", "specific_event", "specific_handler")
sus event_sent := send_event_to_plugin("event_test", "specific_event", "{}")
assert_true(event_sent)

test_start("Send event to plugin without handler")
sus no_handler_sent := send_event_to_plugin("event_test", "unhandled_event", "{}")
assert_false(no_handler_sent)

test_start("Queue event")
sus queued := queue_event("delayed_event", "{\"delay\":true}", 1000)
assert_true(queued)

test_start("Process event queue")
queue_event("queue_test", "{}", 0)
sus processed := process_event_queue()
assert_eq_int(processed, 1)

test_start("Clear event queue")
queue_event("clear_test", "{}", 0)
sus cleared := clear_event_queue()
assert_true(cleared)
sus empty_processed := process_event_queue()
assert_eq_int(empty_processed, 0)

fr fr Security and Sandboxing Tests
test_start("Set plugin permission")
load_plugin("security_test")
sus perm_set := set_plugin_permission("security_test", "file_read", based)
assert_true(perm_set)

test_start("Check plugin permission - granted")
set_plugin_permission("security_test", "network", based)
sus has_perm := check_plugin_permission("security_test", "network")
assert_true(has_perm)

test_start("Create plugin sandbox")
sus sandbox := create_plugin_sandbox("sandbox_test", "{\"memory_limit\":1024}")
assert_true(sandbox)

test_start("Validate plugin security - basic validation")
sus security_report := validate_plugin_security("secure_test")
assert_true(string_contains(security_report, "validated"))

fr fr Basic functionality tests for remaining features
test_start("Send message between plugins")
sus msg_sent := send_message("sender", "receiver", "{\"test\":\"message\"}")
assert_true(msg_sent)

test_start("Broadcast message")
sus broadcast_count := broadcast_message("broadcaster", "{\"broadcast\":true}")
assert_eq_int(broadcast_count, 2)

test_start("Set shared data")
sus data_set := set_shared_data("test_key", "test_value", "data_test")
assert_true(data_set)

test_start("Load plugin config")
sus config := load_plugin_config("config_test")
assert_true(string_contains(config, "{}"))

test_start("Check plugin health")
load_plugin("healthy_test")
sus health := check_plugin_health("healthy_test")
assert_eq_string(health, "healthy")

test_start("Get plugin info")
load_plugin("info_test")
sus info := get_plugin_info("info_test")
assert_true(string_contains(info, "name"))

test_start("Apply plugin filters")
sus filtered := apply_plugin_filters("sanitize_input", "<script>alert('xss')</script>test")
assert_eq_string(filtered, "sanitized_data")

test_start("Get plugin errors - no errors")
load_plugin("error_free")
sus no_errors := get_plugin_errors("error_free")
assert_eq_string(no_errors, "[]")

test_start("Restart plugin")
load_plugin("restart_test")
sus restarted := restart_plugin("restart_test")
assert_true(restarted)

test_start("Quarantine plugin")
load_plugin("malicious_test")
sus quarantined := quarantine_plugin("malicious_test", "security_violation")
assert_true(quarantined)
sus quarantine_state := get_plugin_state("malicious_test")
assert_eq_string(quarantine_state, "quarantined")

print_test_summary()
