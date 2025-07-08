// Test suite for chadlogging module
yeet "testz"
yeet "chadlogging"

// Test log level functions
test_start("Log level names")
assert_eq_string(get_log_level_name(LOG_DEBUG), "DEBUG")
assert_eq_string(get_log_level_name(LOG_INFO), "INFO")
assert_eq_string(get_log_level_name(LOG_WARN), "WARN")
assert_eq_string(get_log_level_name(LOG_ERROR), "ERROR")

// Test log level setting
test_start("Set log level")
assert_true(set_log_level(LOG_DEBUG))
assert_true(set_log_level(LOG_ERROR))
assert_false(set_log_level(-1))  // Invalid level
assert_false(set_log_level(10))  // Invalid level

// Test log level filtering
test_start("Log level filtering")
set_log_level(LOG_WARN)
assert_false(should_log(LOG_DEBUG))
assert_false(should_log(LOG_INFO))
assert_true(should_log(LOG_WARN))
assert_true(should_log(LOG_ERROR))

// Test basic logging functions
test_start("Basic logging functions")
set_log_level(LOG_DEBUG)
assert_true(debug("Debug message"))
assert_true(info("Info message"))
assert_true(warn("Warning message"))
assert_true(error("Error message"))

// Test log formatting
test_start("Log formatting")
sus formatted tea = format_log_message(LOG_INFO, "Test message")
// Check that formatted message contains expected components
assert_true(formatted != "")

// Test structured logging
test_start("Structured logging")
assert_true(log_with_fields(LOG_INFO, "User action", "user_id=123"))
assert_true(log_with_context(LOG_INFO, "Database query", "db_service"))

// Test configuration functions
test_start("Configuration functions")
assert_true(set_log_file("test.log"))
assert_true(set_log_format("[%level%] %message%"))
assert_true(set_max_log_size(2097152))  // 2MB
assert_true(set_max_log_files(10))

// Test logger creation
test_start("Logger creation")
sus logger_name tea = create_logger("test_logger", LOG_INFO)
assert_eq_string(logger_name, "test_logger")

// Test logger-specific logging
test_start("Logger-specific logging")
assert_true(log_with_logger("db_logger", LOG_ERROR, "Connection failed"))
assert_true(log_with_logger("auth_logger", LOG_WARN, "Invalid token"))

// Test performance logging
test_start("Performance logging")
sus start_time normie = perf_start("database_query")
assert_true(perf_end("database_query", start_time))

// Test log rotation
test_start("Log rotation")
assert_false(should_rotate_log())  // Should not rotate by default
assert_true(rotate_logs())

// Test log statistics
test_start("Log statistics")
sus stats tea = get_log_stats()
assert_true(stats != "")

// Test system initialization and cleanup
test_start("System initialization")
assert_true(init_logging())
assert_true(cleanup_logging())

// Test edge cases
test_start("Edge cases")
assert_true(log_message(LOG_INFO, ""))  // Empty message
assert_true(log_message(LOG_DEBUG, "Very long message that might test the limits of the logging system and see how it handles extended content"))

// Test log level constants
test_start("Log level constants")
assert_eq_int(LOG_DEBUG, 0)
assert_eq_int(LOG_INFO, 1)
assert_eq_int(LOG_WARN, 2)
assert_eq_int(LOG_ERROR, 3)

// Test timestamp function
test_start("Timestamp function")
sus timestamp tea = get_timestamp()
assert_true(timestamp != "")

// Test log filtering with different levels
test_start("Log filtering comprehensive")
set_log_level(LOG_ERROR)
assert_false(should_log(LOG_DEBUG))
assert_false(should_log(LOG_INFO))
assert_false(should_log(LOG_WARN))
assert_true(should_log(LOG_ERROR))

set_log_level(LOG_DEBUG)
assert_true(should_log(LOG_DEBUG))
assert_true(should_log(LOG_INFO))
assert_true(should_log(LOG_WARN))
assert_true(should_log(LOG_ERROR))

// Test complex structured logging
test_start("Complex structured logging")
assert_true(log_with_fields(LOG_ERROR, "Payment failed", "user_id=456 amount=99.99 currency=USD"))
assert_true(log_with_context(LOG_WARN, "Slow query detected", "database_monitor"))

// Test performance measurement
test_start("Performance measurement")
sus perf_start_time normie = perf_start("api_request")
assert_true(perf_start_time > 0)
assert_true(perf_end("api_request", perf_start_time))

// Test configuration edge cases
test_start("Configuration edge cases")
assert_true(set_log_file(""))  // Empty path
assert_true(set_log_format(""))  // Empty format
assert_true(set_max_log_size(0))  // Zero size
assert_true(set_max_log_files(0))  // Zero files

// Test multiple logger instances
test_start("Multiple logger instances")
sus logger1 tea = create_logger("auth", LOG_WARN)
sus logger2 tea = create_logger("db", LOG_ERROR)
assert_eq_string(logger1, "auth")
assert_eq_string(logger2, "db")

assert_true(log_with_logger("auth", LOG_WARN, "Authentication warning"))
assert_true(log_with_logger("db", LOG_ERROR, "Database error"))

// Test log message formatting with different levels
test_start("Log message formatting")
sus debug_msg tea = format_log_message(LOG_DEBUG, "Debug test")
sus info_msg tea = format_log_message(LOG_INFO, "Info test")
sus warn_msg tea = format_log_message(LOG_WARN, "Warn test")
sus error_msg tea = format_log_message(LOG_ERROR, "Error test")

assert_true(debug_msg != "")
assert_true(info_msg != "")
assert_true(warn_msg != "")
assert_true(error_msg != "")

// Test system state after cleanup
test_start("System state after cleanup")
assert_true(init_logging())
assert_true(info("System initialized"))
assert_true(cleanup_logging())

// Test buffer flushing
test_start("Buffer flushing")
assert_true(flush_logs())

print_test_summary()
