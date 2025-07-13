yeet "testz"
yeet "signal_boost"

# Test signal handler registration
test_start("Signal handler registration")
sus result lit = register_signal_handler(SIGTERM, HANDLER_DEFAULT, "")
assert_true(result)
assert_true(has_signal_handler(SIGTERM))
sus handler map = get_signal_handler(SIGTERM)
assert_eq_string(handler.get("type"), HANDLER_DEFAULT)
print_test_summary()

# Test signal handler unregistration
test_start("Signal handler unregistration")
sus unreg_result lit = unregister_signal_handler(SIGTERM)
assert_true(unreg_result)
assert_false(has_signal_handler(SIGTERM))
print_test_summary()

# Test custom signal handler
test_start("Custom signal handler")
sus custom_result lit = register_signal_handler(SIGUSR1, HANDLER_CUSTOM, "log_only")
assert_true(custom_result)
sus custom_handler map = get_signal_handler(SIGUSR1)
assert_eq_string(custom_handler.get("type"), HANDLER_CUSTOM)
assert_eq_string(custom_handler.get("action"), "log_only")
print_test_summary()

# Test signal notification
test_start("Signal notification")
sus notify_result lit = notify(SIGUSR1)
assert_true(notify_result)
sus updated_handler map = get_signal_handler(SIGUSR1)
sus count normie = updated_handler.get("count")
assert_eq_int(count, 1)
print_test_summary()

# Test signal names
test_start("Signal name resolution")
assert_eq_string(get_signal_name(SIGTERM), "SIGTERM")
assert_eq_string(get_signal_name(SIGINT), "SIGINT")
assert_eq_string(get_signal_name(SIGUSR1), "SIGUSR1")
assert_eq_string(get_signal_name(SIGUSR2), "SIGUSR2")
assert_eq_string(get_signal_name(SIGHUP), "SIGHUP")
assert_eq_string(get_signal_name(999), "UNKNOWN")
print_test_summary()

# Test shutdown task management
test_start("Shutdown task management")
sus task_result lit = add_shutdown_task("custom_cleanup")
assert_true(task_result)
sus remove_result lit = remove_shutdown_task("custom_cleanup")
assert_true(remove_result)
sus remove_fail lit = remove_shutdown_task("nonexistent_task")
assert_false(remove_fail)
print_test_summary()

# Test shutdown state
test_start("Shutdown state management")
assert_false(is_shutdown_requested())
initiate_graceful_shutdown()
assert_true(is_shutdown_requested())
sus cancel_result lit = cancel_shutdown()
assert_true(cancel_result)
assert_false(is_shutdown_requested())
print_test_summary()

# Test signal throttling
test_start("Signal throttling")
sus throttle_result lit = set_signal_throttle(SIGTERM, 1000)
assert_true(throttle_result)
# First signal should not be throttled
assert_false(is_signal_throttled(SIGTERM))
# Immediate second signal should be throttled
assert_true(is_signal_throttled(SIGTERM))
print_test_summary()

# Test signal subscription
test_start("Signal subscription")
sus sub_result lit = subscribe_to_signal(SIGINT, "test_subscriber")
assert_true(sub_result)
sus unsub_result lit = unsubscribe_from_signal(SIGINT, "test_subscriber")
assert_true(unsub_result)
sus unsub_fail lit = unsubscribe_from_signal(SIGINT, "nonexistent_subscriber")
assert_false(unsub_fail)
print_test_summary()

# Test signal list functionality
test_start("Signal list functionality")
register_signal_handler(SIGTERM, HANDLER_DEFAULT, "")
register_signal_handler(SIGINT, HANDLER_IGNORE, "")
register_signal_handler(SIGUSR1, HANDLER_CUSTOM, "test_action")
sus signals [normie] = list_signal_handlers()
assert_true(signals.length() >= 3)
print_test_summary()

# Test ignore signal handler
test_start("Ignore signal handler")
register_signal_handler(SIGPIPE, HANDLER_IGNORE, "")
sus ignore_result lit = notify(SIGPIPE)
assert_true(ignore_result)
sus ignore_handler map = get_signal_handler(SIGPIPE)
sus ignore_count normie = ignore_handler.get("count")
assert_eq_int(ignore_count, 1)
print_test_summary()

# Test multiple signal notifications
test_start("Multiple signal notifications")
register_signal_handler(SIGALRM, HANDLER_DEFAULT, "")
notify(SIGALRM)
notify(SIGALRM)
notify(SIGALRM)
sus multi_handler map = get_signal_handler(SIGALRM)
sus multi_count normie = multi_handler.get("count")
assert_eq_int(multi_count, 3)
print_test_summary()

# Test signal notification without handler
test_start("Signal notification without handler")
sus no_handler_result lit = notify(999)  # Non-existent signal
assert_false(no_handler_result)
print_test_summary()

# Test initialization
test_start("Module initialization")
reset()  # Clear state first
init_signal_boost()
# Check that default handlers are registered
assert_true(has_signal_handler(SIGTERM))
assert_true(has_signal_handler(SIGINT))
assert_true(has_signal_handler(SIGUSR1))
assert_true(has_signal_handler(SIGUSR2))
assert_true(has_signal_handler(SIGHUP))
print_test_summary()

# Test module info
test_start("Module information")
sus info tea = get_module_info()
assert_true(info.contains("signal_boost"))
assert_true(info.contains("v1.0"))
print_test_summary()

# Test signal constants
test_start("Signal constants")
assert_eq_int(SIGTERM, 15)
assert_eq_int(SIGINT, 2)
assert_eq_int(SIGUSR1, 10)
assert_eq_int(SIGUSR2, 12)
assert_eq_int(SIGHUP, 1)
assert_eq_int(SIGQUIT, 3)
assert_eq_int(SIGPIPE, 13)
assert_eq_int(SIGALRM, 14)
print_test_summary()

# Test handler types
test_start("Handler type constants")
assert_eq_string(HANDLER_IGNORE, "ignore")
assert_eq_string(HANDLER_DEFAULT, "default")
assert_eq_string(HANDLER_CUSTOM, "custom")
print_test_summary()

# Test comprehensive signal processing
test_start("Comprehensive signal processing")
reset()
init_signal_boost()

# Test SIGTERM default behavior
notify(SIGTERM)
assert_true(is_shutdown_requested())
cancel_shutdown()

# Test SIGUSR1 default behavior (should call reload_configuration)
notify(SIGUSR1)

# Test SIGUSR2 default behavior (should call dump_statistics)
notify(SIGUSR2)

# Test custom action
register_signal_handler(SIGQUIT, HANDLER_CUSTOM, "graceful_shutdown")
notify(SIGQUIT)
assert_true(is_shutdown_requested())

print_test_summary()

# Test edge cases
test_start("Edge cases")
# Test removing non-existent handler
sus remove_nonexistent lit = unregister_signal_handler(999)
assert_false(remove_nonexistent)

# Test getting handler for non-existent signal
sus empty_handler map = get_signal_handler(999)
assert_eq_int(empty_handler.size(), 0)

# Test throttling non-configured signal
assert_false(is_signal_throttled(999))

print_test_summary()

vibez.spill("All signal_boost module tests completed successfully!")
