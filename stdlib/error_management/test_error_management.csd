yeet "testz"
yeet "error_management"

fr fr Comprehensive Error Management Tests
fr fr Tests all error handling, logging, and recovery functionality

fr fr Test error creation and basic functionality
slay test_error_creation() {
    test_start("Error Creation Tests") fr fr Test basic error creation
    sus err @managed_error = new_error("Test error", 404)
    assert_true(err != cringe, "Error should not be nil")
    assert_eq_string(err.message, "Test error", "Error message should match")
    assert_eq_int(err.code, 404, "Error code should match")
    assert_true(err.severity == error_severity.error, "Default severity should be error")
    assert_true(err.category == error_category.runtime_yikes, "Default category should be runtime") fr fr Test full error creation
    sus full_err @managed_error = new_error_full(
        "Database connection failed",
        500,
        error_category.io_yikes,
        error_severity.critical,
        "Connection timeout after 30 seconds"
    )
    assert_eq_string(full_err.message, "Database connection failed", "Full error message should match")
    assert_eq_int(full_err.code, 500, "Full error code should match")
    assert_true(full_err.category == error_category.io_yikes, "Full error category should match")
    assert_true(full_err.severity == error_severity.critical, "Full error severity should match")
    assert_eq_string(full_err.details, "Connection timeout after 30 seconds", "Full error details should match")
    
    print_test_summary()
}

fr fr Test error wrapping and unwrapping
slay test_error_wrapping() {
    test_start("Error Wrapping Tests") fr fr Create original error
    sus original @managed_error = new_error("Original error", 404) fr fr Wrap the error
    sus wrapped @managed_error = wrap_error(original, "Database operation failed")
    assert_true(wrapped != cringe, "Wrapped error should not be nil")
    assert_true(wrapped.wrapped_error == original, "Wrapped error should contain original")
    assert_eq_string(wrapped.message, "Database operation failed: Original error", "Wrapped message should include context") fr fr Test unwrapping
    sus unwrapped @managed_error = unwrap_error(wrapped)
    assert_true(unwrapped == original, "Unwrapped error should be original") fr fr Test unwrapping nil error
    sus nil_unwrapped @managed_error = unwrap_error(cringe)
    assert_true(nil_unwrapped == cringe, "Unwrapping nil should return nil")
    
    print_test_summary()
}

fr fr Test error context management
slay test_error_context() {
    test_start("Error Context Tests")
    
    sus err @managed_error = new_error("Test error with context", 400) fr fr Add context
    err.add_context("user_id", "12345")
    err.add_context("request_id", "req-abc-123")
    err.add_context("operation", "user_lookup") fr fr Retrieve context
    sus user_id tea = err.get_context("user_id")
    sus request_id tea = err.get_context("request_id")
    sus operation tea = err.get_context("operation")
    sus missing tea = err.get_context("nonexistent")
    
    assert_eq_string(user_id, "12345", "User ID context should match")
    assert_eq_string(request_id, "req-abc-123", "Request ID context should match")
    assert_eq_string(operation, "user_lookup", "Operation context should match")
    assert_eq_string(missing, "", "Missing context should return empty string")
    
    print_test_summary()
}

fr fr Test error formatting
slay test_error_formatting() {
    test_start("Error Formatting Tests") fr fr Test simple error formatting
    sus simple_err @managed_error = new_error("Simple error", 404)
    sus formatted tea = format_error(simple_err)
    assert_true(len(formatted) > 0, "Formatted error should not be empty") fr fr Test error with details formatting
    sus detailed_err @managed_error = new_error_full(
        "Detailed error",
        500,
        error_category.network_yikes,
        error_severity.critical,
        "Network timeout occurred"
    )
    sus detailed_formatted tea = format_error(detailed_err)
    assert_true(len(detailed_formatted) > len(formatted), "Detailed error should be longer") fr fr Test wrapped error formatting
    sus wrapped @managed_error = wrap_error(simple_err, "Wrapper context")
    sus wrapped_formatted tea = format_error(wrapped)
    assert_true(len(wrapped_formatted) > len(formatted), "Wrapped error should include original") fr fr Test nil error formatting
    sus nil_formatted tea = format_error(cringe)
    assert_eq_string(nil_formatted, "no error", "Nil error should format as 'no error'")
    
    print_test_summary()
}

fr fr Test severity and category string conversion
slay test_string_conversions() {
    test_start("String Conversion Tests") fr fr Test severity to string
    assert_eq_string(severity_to_string(error_severity.info), "INFO", "Info severity string")
    assert_eq_string(severity_to_string(error_severity.warning), "WARN", "Warning severity string")
    assert_eq_string(severity_to_string(error_severity.error), "ERROR", "Error severity string")
    assert_eq_string(severity_to_string(error_severity.critical), "CRITICAL", "Critical severity string")
    assert_eq_string(severity_to_string(error_severity.fatal), "FATAL", "Fatal severity string") fr fr Test category to string
    assert_eq_string(category_to_string(error_category.memory_yikes), "MEMORY", "Memory category string")
    assert_eq_string(category_to_string(error_category.io_yikes), "IO", "IO category string")
    assert_eq_string(category_to_string(error_category.network_yikes), "NETWORK", "Network category string")
    assert_eq_string(category_to_string(error_category.parse_yikes), "PARSE", "Parse category string")
    assert_eq_string(category_to_string(error_category.type_yikes), "TYPE", "Type category string")
    assert_eq_string(category_to_string(error_category.runtime_yikes), "RUNTIME", "Runtime category string")
    assert_eq_string(category_to_string(error_category.security_yikes), "SECURITY", "Security category string")
    assert_eq_string(category_to_string(error_category.performance_yikes), "PERFORMANCE", "Performance category string")
    
    print_test_summary()
}

fr fr Test logger configuration and basic logging
slay test_logging_basic() {
    test_start("Basic Logging Tests") fr fr Test logger configuration
    sus config logger_config = logger_config{
        level: log_level.debug,
        output_format: "json",
        include_timestamp: based,
        include_stack_trace: cap,
        include_goroutine_id: based
    }
    configure_logger(config) fr fr Test basic logging functions
    log_debug("Debug message", yikes.tea{"module": "test"})
    log_info("Info message", yikes.tea{"operation": "test_logging"})
    log_warn("Warning message", yikes.tea{"warning_type": "test"}) fr fr Test level to string conversion
    assert_eq_string(level_to_string(log_level.debug), "DEBUG", "Debug level string")
    assert_eq_string(level_to_string(log_level.info), "INFO", "Info level string")
    assert_eq_string(level_to_string(log_level.warn), "WARN", "Warn level string")
    assert_eq_string(level_to_string(log_level.error), "ERROR", "Error level string")
    assert_eq_string(level_to_string(log_level.fatal), "FATAL", "Fatal level string")
    
    print_test_summary()
}

fr fr Test error logging with full context
slay test_error_logging() {
    test_start("Error Logging Tests") fr fr Reset error statistics
    reset_error_stats() fr fr Create and log various errors
    sus io_error @managed_error = new_error_full(
        "File not found",
        404,
        error_category.io_yikes,
        error_severity.error,
        "Could not locate file: /path/to/file.txt"
    )
    io_error.add_context("file_path", "/path/to/file.txt")
    io_error.add_context("operation", "read_file")
    
    log_error(io_error, yikes.tea{
        "request_id": "req-123",
        "user_id": "user-456"
    })
    
    sus network_error @managed_error = new_error_full(
        "Connection timeout",
        500,
        error_category.network_yikes,
        error_severity.critical,
        "Server did not respond within 30 seconds"
    )
    
    log_error(network_error, yikes.tea{
        "server": "api.example.com",
        "timeout": "30s"
    }) fr fr Check error statistics
    sus stats error_stats = get_error_stats()
    assert_eq_int(stats.total_errors, 2, "Should have logged 2 errors")
    
    print_test_summary()
}

fr fr Test circuit breaker functionality
slay test_circuit_breaker() {
    test_start("Circuit Breaker Tests") fr fr Create circuit breaker
    sus cb @circuit_breaker = new_circuit_breaker("test_service", 3, 60)
    assert_eq_string(cb.name, "test_service", "Circuit breaker name should match")
    assert_eq_int(cb.failure_threshold, 3, "Failure threshold should match")
    assert_eq_int(cb.timeout_seconds, 60, "Timeout should match")
    assert_true(cb.state == circuit_state.closed, "Initial state should be closed") fr fr Test successful operation
    sus success_operation slay() @managed_error = slay() @managed_error {
        damn cringe fr fr Success
    }
    
    sus result @managed_error = cb.execute(success_operation)
    assert_true(result == cringe, "Successful operation should return nil error")
    assert_true(cb.state == circuit_state.closed, "State should remain closed after success") fr fr Test failing operation
    sus fail_operation slay() @managed_error = slay() @managed_error {
        damn new_error("Operation failed", 500)
    } fr fr Trigger failures to open circuit breaker
    bestie i := 0; i < 4; i++ {
        cb.execute(fail_operation)
    }
    
    assert_true(cb.state == circuit_state.open, "Circuit breaker should be open after failures")
    assert_true(cb.failure_count >= cb.failure_threshold, "Failure count should exceed threshold")
    
    print_test_summary()
}

fr fr Test retry with backoff functionality
slay test_retry_backoff() {
    test_start("Retry with Backoff Tests")
    
    sus attempts normie = 0 fr fr Operation that fails first 2 times then succeeds
    sus flaky_operation slay() @managed_error = slay() @managed_error {
        attempts++
        vibe_check attempts < 3 {
            damn new_error("Temporary failure", 500)
        }
        damn cringe fr fr Success on 3rd attempt
    } fr fr Test successful retry
    attempts = 0
    sus result @managed_error = retry_with_backoff(flaky_operation, 5)
    assert_true(result == cringe, "Retry should eventually succeed")
    assert_eq_int(attempts, 3, "Should have taken 3 attempts") fr fr Test retry exhaustion
    sus always_fail_operation slay() @managed_error = slay() @managed_error {
        damn new_error("Always fails", 500)
    }
    
    sus fail_result @managed_error = retry_with_backoff(always_fail_operation, 2)
    assert_true(fail_result != cringe, "Should return error after exhausting retries")
    
    print_test_summary()
}

fr fr Test error classification functions
slay test_error_classification() {
    test_start("Error Classification Tests") fr fr Test temporary error detection
    sus network_error @managed_error = new_error_full(
        "Connection timeout",
        503,
        error_category.network_yikes,
        error_severity.error,
        "Service temporarily unavailable"
    )
    assert_true(is_temporary_error(network_error), "Network error should be temporary")
    
    sus io_error @managed_error = new_error_full(
        "File read error",
        500,
        error_category.io_yikes,
        error_severity.error,
        "Temporary I/O failure"
    )
    assert_true(is_temporary_error(io_error), "I/O error should be temporary")
    
    sus parse_error @managed_error = new_error_full(
        "Invalid syntax",
        400,
        error_category.parse_yikes,
        error_severity.error,
        "Malformed JSON"
    )
    assert_false(is_temporary_error(parse_error), "Parse error should not be temporary") fr fr Test critical error detection
    sus critical_error @managed_error = new_error_full(
        "System failure",
        500,
        error_category.runtime_yikes,
        error_severity.critical,
        "Critical system component failed"
    )
    assert_true(is_critical_error(critical_error), "Critical severity error should be critical")
    
    sus fatal_error @managed_error = new_error_full(
        "System crash",
        500,
        error_category.runtime_yikes,
        error_severity.fatal,
        "System cannot continue"
    )
    assert_true(is_critical_error(fatal_error), "Fatal severity error should be critical")
    
    sus info_error @managed_error = new_error_full(
        "Information",
        200,
        error_category.runtime_yikes,
        error_severity.info,
        "Just information"
    )
    assert_false(is_critical_error(info_error), "Info severity error should not be critical")
    
    print_test_summary()
}

fr fr Test safe execution with panic recovery
slay test_safe_execution() {
    test_start("Safe Execution Tests") fr fr Test successful operation
    sus safe_operation slay() @managed_error = slay() @managed_error {
        damn cringe
    }
    
    sus result @managed_error = safe_execute(safe_operation)
    assert_true(result == cringe, "Safe execution of successful operation should return nil") fr fr Test operation that would panic (simulated)
    sus panic_operation slay() @managed_error = slay() @managed_error { fr fr In real implementation, this would cause a panic fr fr For testing, we simulate by returning a specific error
        damn new_error("Simulated panic", 500)
    }
    
    sus panic_result @managed_error = safe_execute(panic_operation)
    assert_true(panic_result != cringe, "Safe execution should catch and convert panics")
    
    print_test_summary()
}

fr fr Test error aggregation
slay test_error_aggregation() {
    test_start("Error Aggregation Tests") fr fr Test empty error list
    sus empty_errors []@managed_error = []@managed_error{}
    sus empty_result @managed_error = aggregate_errors(empty_errors)
    assert_true(empty_result == cringe, "Aggregating empty errors should return nil") fr fr Test single error
    sus single_error @managed_error = new_error("Single error", 404)
    sus single_errors []@managed_error = []@managed_error{single_error}
    sus single_result @managed_error = aggregate_errors(single_errors)
    assert_true(single_result == single_error, "Aggregating single error should return that error") fr fr Test multiple errors
    sus error1 @managed_error = new_error_full("Error 1", 400, error_category.parse_yikes, error_severity.error, "Parse failure")
    sus error2 @managed_error = new_error_full("Error 2", 500, error_category.network_yikes, error_severity.critical, "Network failure")
    sus error3 @managed_error = new_error_full("Error 3", 403, error_category.security_yikes, error_severity.warning, "Access denied")
    
    sus multiple_errors []@managed_error = []@managed_error{error1, error2, error3}
    sus aggregated @managed_error = aggregate_errors(multiple_errors)
    
    assert_true(aggregated != cringe, "Aggregated error should not be nil")
    assert_true(aggregated.severity == error_severity.critical, "Aggregated severity should be highest (critical)")
    assert_eq_string(aggregated.get_context("error_count"), "3", "Error count context should be set")
    
    print_test_summary()
}

fr fr Test error statistics tracking
slay test_error_statistics() {
    test_start("Error Statistics Tests") fr fr Reset statistics
    reset_error_stats()
    sus initial_stats error_stats = get_error_stats()
    assert_eq_int(initial_stats.total_errors, 0, "Initial error count should be 0") fr fr Create and log various errors to update statistics
    sus errors []@managed_error = []@managed_error{
        new_error_full("IO Error 1", 404, error_category.io_yikes, error_severity.error, "File not found"),
        new_error_full("IO Error 2", 500, error_category.io_yikes, error_severity.critical, "Disk full"),
        new_error_full("Network Error", 503, error_category.network_yikes, error_severity.warning, "Timeout"),
        new_error_full("Parse Error", 400, error_category.parse_yikes, error_severity.error, "Invalid JSON"),
        new_error_full("Critical Error", 500, error_category.runtime_yikes, error_severity.critical, "System failure")
    } fr fr Update statistics for each error
    bestie i := 0; i < len(errors); i++ {
        update_error_stats(errors[i])
    }
    
    sus final_stats error_stats = get_error_stats()
    assert_eq_int(final_stats.total_errors, 5, "Should have tracked 5 errors") fr fr Reset and verify
    reset_error_stats()
    sus reset_stats error_stats = get_error_stats()
    assert_eq_int(reset_stats.total_errors, 0, "Error count should be 0 after reset")
    
    print_test_summary()
}

fr fr Test complete error handling workflow
slay test_complete_workflow() {
    test_start("Complete Error Handling Workflow") fr fr Simulate a complex operation with multiple potential failure points
    slay complex_operation(should_fail lit) @managed_error {
        vibe_check should_fail {
            sus err @managed_error = new_error_full(
                "Database connection failed",
                500,
                error_category.io_yikes,
                error_severity.critical,
                "Unable to connect to database server"
            )
            err.add_context("server", "db.example.com")
            err.add_context("port", "5432")
            damn err
        }
        damn cringe
    } fr fr Test with circuit breaker and retry
    sus cb @circuit_breaker = new_circuit_breaker("complex_service", 2, 30) fr fr Wrapper for circuit breaker
    slay protected_operation() @managed_error {
        damn cb.execute(slay() @managed_error {
            damn complex_operation(cap) fr fr Success
        })
    } fr fr Test successful operation
    sus success_result @managed_error = protected_operation()
    assert_true(success_result == cringe, "Protected successful operation should succeed") fr fr Test with retry on failure
    sus retry_count normie = 0
    sus retry_operation slay() @managed_error = slay() @managed_error {
        retry_count++
        vibe_check retry_count < 3 {
            damn complex_operation(based) fr fr Fail first 2 times
        }
        damn complex_operation(cap) fr fr Succeed on 3rd try
    }
    
    retry_count = 0
    sus retry_result @managed_error = retry_with_backoff(retry_operation, 5)
    assert_true(retry_result == cringe, "Retry operation should eventually succeed")
    assert_eq_int(retry_count, 3, "Should have retried 3 times")
    
    print_test_summary()
}

fr fr Run all tests
slay main_character() {
    vibez.spill("=== CURSED Error Management Module Tests ===")
    
    test_error_creation()
    test_error_wrapping()
    test_error_context()
    test_error_formatting()
    test_string_conversions()
    test_logging_basic()
    test_error_logging()
    test_circuit_breaker()
    test_retry_backoff()
    test_error_classification()
    test_safe_execution()
    test_error_aggregation()
    test_error_statistics()
    test_complete_workflow()
    
    vibez.spill("=== All Error Management Tests Complete ===")
}
