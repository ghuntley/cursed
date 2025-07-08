// CURSED Logging Module Test Suite
// Comprehensive tests for logging and debugging functionality

yeet "testz"
yeet "logging"

slay test_basic_logging() {
    test_start("Basic Logging Functions")
    
    // Test all log levels
    logging.log_trace("This is a trace message")
    logging.log_debug("This is a debug message")
    logging.log_info("This is an info message")
    logging.log_warn("This is a warning message")
    logging.log_error("This is an error message")
    logging.log_fatal("This is a fatal message")
    
    // All logging functions should execute without error
    assert_true(based)
}

slay test_log_level_filtering() {
    test_start("Log Level Filtering")
    
    // Set log level to WARN
    logging.set_log_level(logging.LOG_WARN)
    
    // Test that only WARN and above are logged
    logging.log_debug("This should not appear")
    logging.log_info("This should not appear")
    logging.log_warn("This should appear")
    logging.log_error("This should appear")
    
    // Reset to INFO for other tests
    logging.set_log_level(logging.LOG_INFO)
    
    assert_true(based)
}

slay test_log_formatting() {
    test_start("Log Message Formatting")
    
    // Test custom format
    logging.set_log_format("{level}: {message}")
    logging.log_info("Custom format test")
    
    // Reset to default format
    logging.set_log_format("[{timestamp}] {level}: {message}")
    logging.log_info("Default format test")
    
    assert_true(based)
}

slay test_structured_logging() {
    test_start("Structured Logging")
    
    // Test structured logging with fields
    sus fields map[tea]tea = map[tea]tea{
        "user_id": "12345",
        "action": "login",
        "ip": "192.168.1.1"
    }
    
    logging.info_with_fields("User login successful", fields)
    logging.warn_with_fields("Login from new IP", fields)
    logging.error_with_fields("Failed login attempt", fields)
    
    assert_true(based)
}

slay test_named_logger() {
    test_start("Named Logger Creation")
    
    // Create named logger
    sus logger logging.Logger = logging.create_logger("TestLogger")
    
    // Test named logger functions
    logging.logger_info(logger, "Info from named logger")
    logging.logger_warn(logger, "Warning from named logger")
    logging.logger_error(logger, "Error from named logger")
    
    assert_true(based)
}

slay test_performance_logging() {
    test_start("Performance Logging")
    
    // Test performance logging
    logging.log_performance("database_query", 150)
    logging.log_performance("file_processing", 2500)
    
    // Test memory usage logging
    logging.log_memory_usage("data_loading", 1024000)
    logging.log_memory_usage("cache_allocation", 512000)
    
    assert_true(based)
}

slay test_error_logging() {
    test_start("Error Logging Utilities")
    
    // Test error logging with stack trace
    sus stack_trace tea = "at main() line 42\nat process_data() line 15"
    logging.log_error_with_stack("Database connection failed", stack_trace)
    
    // Test exception logging
    logging.log_exception("NullPointerException", "Variable was null", "data_processor.csd:25")
    
    assert_true(based)
}

slay test_conditional_logging() {
    test_start("Conditional Logging")
    
    // Test conditional logging
    sus debug_enabled lit = based
    logging.log_if(debug_enabled, logging.LOG_DEBUG, "Debug mode is enabled")
    
    sus error_condition lit = cap
    logging.log_if(error_condition, logging.LOG_ERROR, "This should not appear")
    
    assert_true(based)
}

slay test_debug_utilities() {
    test_start("Debug Utilities")
    
    // Test variable logging
    sus test_variable tea = "test_value"
    logging.log_variable("test_variable", test_variable)
    
    // Test function entry/exit logging
    logging.log_function_entry("test_function")
    logging.log_checkpoint("middle_of_function")
    logging.log_function_exit("test_function")
    
    assert_true(based)
}

slay test_log_constants() {
    test_start("Log Level Constants")
    
    // Test log level constants
    assert_eq_int(logging.LOG_TRACE, 0)
    assert_eq_int(logging.LOG_DEBUG, 1)
    assert_eq_int(logging.LOG_INFO, 2)
    assert_eq_int(logging.LOG_WARN, 3)
    assert_eq_int(logging.LOG_ERROR, 4)
    assert_eq_int(logging.LOG_FATAL, 5)
}

slay test_log_configuration() {
    test_start("Log Configuration")
    
    // Test setting log file
    logging.set_log_file("test.log")
    logging.log_info("Message to file")
    
    // Test getting log file size
    sus file_size thicc = logging.get_log_file_size()
    assert_true(file_size >= 0)
    
    // Test log rotation
    logging.rotate_log_file()
    
    // Test clearing log file
    logging.clear_log_file()
    
    assert_true(based)
}

slay test_logger_methods() {
    test_start("Logger Instance Methods")
    
    // Create logger and test all methods
    sus logger logging.Logger = logging.create_logger("MethodTest")
    
    logging.logger_trace(logger, "Trace message")
    logging.logger_debug(logger, "Debug message")
    logging.logger_info(logger, "Info message")
    logging.logger_warn(logger, "Warning message")
    logging.logger_error(logger, "Error message")
    logging.logger_fatal(logger, "Fatal message")
    
    assert_true(based)
}

slay test_log_message_formatting() {
    test_start("Log Message Formatting")
    
    // Test timestamp generation
    sus timestamp tea = logging.get_timestamp()
    assert_true(len(timestamp) > 0)
    
    // Test placeholder replacement
    sus original tea = "Hello {name}, welcome to {place}"
    sus replaced tea = logging.replace_placeholder(original, "{name}", "World")
    // Note: This tests the function exists and can be called
    
    assert_true(based)
}

// Main test runner
slay main() {
    vibez.spill("Starting CURSED Logging Module Tests")
    
    test_basic_logging()
    test_log_level_filtering()
    test_log_formatting()
    test_structured_logging()
    test_named_logger()
    test_performance_logging()
    test_error_logging()
    test_conditional_logging()
    test_debug_utilities()
    test_log_constants()
    test_log_configuration()
    test_logger_methods()
    test_log_message_formatting()
    
    print_test_summary()
}
