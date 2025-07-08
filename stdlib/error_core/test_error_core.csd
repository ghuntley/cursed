yeet "testz"
yeet "error_core"

# Test basic error creation
slay test_basic_error_creation() {
    test_start("Basic error creation")
    
    sus err = new_error("Test error", 1000)
    assert_eq_string(err.message(), "Test error")
    assert_eq_int(err.code(), 1000)
    assert_eq_string(err.details(), "")
    
    vibez.spill("✅ Basic error creation test passed")
}

# Test IO error creation
slay test_io_error_creation() {
    test_start("IO error creation")
    
    sus err = new_io_error("File not found", "/path/to/file.txt", "read")
    assert_eq_string(err.base.message(), "File not found")
    assert_eq_int(err.base.code(), 1001)
    assert_eq_string(err.path, "/path/to/file.txt")
    assert_eq_string(err.operation, "read")
    
    vibez.spill("✅ IO error creation test passed")
}

# Test value error creation
slay test_value_error_creation() {
    test_start("Value error creation")
    
    sus err = new_value_error("Invalid value", "abc", "normie")
    assert_eq_string(err.base.message(), "Invalid value")
    assert_eq_int(err.base.code(), 2001)
    assert_eq_string(err.value, "abc")
    assert_eq_string(err.expected_type, "normie")
    
    vibez.spill("✅ Value error creation test passed")
}

# Test type error creation
slay test_type_error_creation() {
    test_start("Type error creation")
    
    sus err = new_type_error("Type mismatch", "tea", "normie")
    assert_eq_string(err.base.message(), "Type mismatch")
    assert_eq_int(err.base.code(), 3001)
    assert_eq_string(err.actual_type, "tea")
    assert_eq_string(err.expected_type, "normie")
    
    vibez.spill("✅ Type error creation test passed")
}

# Test memory error creation
slay test_memory_error_creation() {
    test_start("Memory error creation")
    
    sus err = new_memory_error("Out of memory", 1024, 512)
    assert_eq_string(err.base.message(), "Out of memory")
    assert_eq_int(err.base.code(), 4001)
    assert_eq_int(err.requested_size, 1024)
    assert_eq_int(err.available_size, 512)
    
    vibez.spill("✅ Memory error creation test passed")
}

# Test network error creation
slay test_network_error_creation() {
    test_start("Network error creation")
    
    sus err = new_network_error("Connection timeout", "localhost", 8080, "30s")
    assert_eq_string(err.base.message(), "Connection timeout")
    assert_eq_int(err.base.code(), 5001)
    assert_eq_string(err.host, "localhost")
    assert_eq_int(err.port, 8080)
    assert_eq_string(err.timeout_duration, "30s")
    
    vibez.spill("✅ Network error creation test passed")
}

# Test parse error creation
slay test_parse_error_creation() {
    test_start("Parse error creation")
    
    sus err = new_parse_error("Syntax error", 42, 5, 10)
    assert_eq_string(err.base.message(), "Syntax error")
    assert_eq_int(err.base.code(), 6001)
    assert_eq_int(err.position, 42)
    assert_eq_int(err.line, 5)
    assert_eq_int(err.column, 10)
    
    vibez.spill("✅ Parse error creation test passed")
}

# Test security error creation
slay test_security_error_creation() {
    test_start("Security error creation")
    
    sus err = new_security_error("Access denied", "/etc/passwd", "read")
    assert_eq_string(err.base.message(), "Access denied")
    assert_eq_int(err.base.code(), 7001)
    assert_eq_string(err.resource, "/etc/passwd")
    assert_eq_string(err.required_permission, "read")
    
    vibez.spill("✅ Security error creation test passed")
}

# Test runtime error creation
slay test_runtime_error_creation() {
    test_start("Runtime error creation")
    
    sus err = new_runtime_error("Goroutine panic", 123, "user_function")
    assert_eq_string(err.base.message(), "Goroutine panic")
    assert_eq_int(err.base.code(), 8001)
    assert_eq_int(err.goroutine_id, 123)
    assert_eq_string(err.operation, "user_function")
    
    vibez.spill("✅ Runtime error creation test passed")
}

# Test error type detection
slay test_error_type_detection() {
    test_start("Error type detection")
    
    sus io_err = new_io_error("IO test", "/tmp", "write")
    sus value_err = new_value_error("Value test", "bad", "good")
    sus type_err = new_type_error("Type test", "actual", "expected")
    
    assert_true(is_error_type(yikes{message: "test", code: 1500, details: ""}, "io_error"))
    assert_true(is_error_type(yikes{message: "test", code: 2500, details: ""}, "value_error"))
    assert_true(is_error_type(yikes{message: "test", code: 3500, details: ""}, "type_error"))
    
    assert_false(is_error_type(yikes{message: "test", code: 1500, details: ""}, "value_error"))
    assert_false(is_error_type(yikes{message: "test", code: 2500, details: ""}, "type_error"))
    
    vibez.spill("✅ Error type detection test passed")
}

# Test error wrapping
slay test_error_wrapping() {
    test_start("Error wrapping")
    
    sus original_err = yikes{message: "Original error", code: 1000, details: "Original details"}
    sus wrapped_err = wrap_error(original_err, "Database operation")
    
    assert_eq_string(wrapped_err.message(), "Database operation: Original error")
    assert_eq_int(wrapped_err.code(), 1000)
    assert_eq_string(wrapped_err.details(), "Original details")
    
    # Test wrapping nil error
    sus nil_wrapped = wrap_error(cringe, "Should return nil")
    assert_eq(nil_wrapped, cringe)
    
    vibez.spill("✅ Error wrapping test passed")
}

# Test error chaining
slay test_error_chaining() {
    test_start("Error chaining")
    
    sus base_err = yikes{message: "Base error", code: 1000, details: "Base details"}
    sus new_err = yikes{message: "New error", code: 2000, details: "New details"}
    sus chained_err = chain_error(base_err, new_err)
    
    assert_eq_string(chained_err.message(), "New error (caused by: Base error)")
    assert_eq_int(chained_err.code(), 2000)
    assert_eq_string(chained_err.details(), "New details | Previous: Base details")
    
    vibez.spill("✅ Error chaining test passed")
}

# Test error combining
slay test_error_combining() {
    test_start("Error combining")
    
    sus errors []yikes = []yikes{
        yikes{message: "Error 1", code: 1000, details: "Details 1"},
        yikes{message: "Error 2", code: 2000, details: "Details 2"},
        yikes{message: "Error 3", code: 3000, details: "Details 3"}
    }
    
    sus combined = combine_errors(errors)
    assert_eq_string(combined.message(), "Multiple errors occurred: Error 1; Error 2; Error 3")
    assert_eq_int(combined.code(), 9001)
    assert_eq_string(combined.details(), "Details 1 | Details 2 | Details 3")
    
    # Test single error
    sus single_errors []yikes = []yikes{
        yikes{message: "Single error", code: 1000, details: "Single details"}
    }
    sus single_combined = combine_errors(single_errors)
    assert_eq_string(single_combined.message(), "Single error")
    
    # Test empty errors
    sus empty_errors []yikes = []yikes{}
    sus empty_combined = combine_errors(empty_errors)
    assert_eq(empty_combined, cringe)
    
    vibez.spill("✅ Error combining test passed")
}

# Test temporary error detection
slay test_temporary_error_detection() {
    test_start("Temporary error detection")
    
    sus network_err = yikes{message: "Network error", code: 5500, details: "Network details"}
    sus io_err = yikes{message: "IO error", code: 1500, details: "IO details"}
    sus security_err = yikes{message: "Security error", code: 7500, details: "Security details"}
    
    assert_true(is_temporary_error(network_err))
    assert_true(is_temporary_error(io_err))
    assert_false(is_temporary_error(security_err))
    
    vibez.spill("✅ Temporary error detection test passed")
}

# Test critical error detection
slay test_critical_error_detection() {
    test_start("Critical error detection")
    
    sus memory_err = yikes{message: "Memory error", code: 4500, details: "Memory details"}
    sus security_err = yikes{message: "Security error", code: 7500, details: "Security details"}
    sus io_err = yikes{message: "IO error", code: 1500, details: "IO details"}
    
    assert_true(is_critical_error(memory_err))
    assert_true(is_critical_error(security_err))
    assert_false(is_critical_error(io_err))
    
    vibez.spill("✅ Critical error detection test passed")
}

# Test error formatting
slay test_error_formatting() {
    test_start("Error formatting")
    
    sus err = yikes{message: "Test error", code: 1000, details: "Test details"}
    sus formatted = format_error(err)
    sus expected = "[Error 1000] Test error | Details: Test details"
    
    assert_eq_string(formatted, expected)
    
    # Test nil error formatting
    sus nil_formatted = format_error(cringe)
    assert_eq_string(nil_formatted, "no error")
    
    vibez.spill("✅ Error formatting test passed")
}

# Test JSON error formatting
slay test_json_error_formatting() {
    test_start("JSON error formatting")
    
    sus err = yikes{message: "Test error", code: 1000, details: "Test details"}
    sus json_formatted = format_error_json(err)
    sus expected = "{\"error\": {\"code\": 1000, \"message\": \"Test error\", \"details\": \"Test details\"}}"
    
    assert_eq_string(json_formatted, expected)
    
    # Test nil error JSON formatting
    sus nil_json = format_error_json(cringe)
    assert_eq_string(nil_json, "{\"error\": null}")
    
    vibez.spill("✅ JSON error formatting test passed")
}

# Test circuit breaker
slay test_circuit_breaker() {
    test_start("Circuit breaker")
    
    sus cb = new_circuit_breaker(3, 2)
    assert_eq_int(cb.failure_count, 0)
    assert_eq_int(cb.state, 0)  # closed
    
    # Simulate failures
    cb.on_failure()
    cb.on_failure()
    cb.on_failure()
    
    assert_eq_int(cb.failure_count, 3)
    assert_eq_int(cb.state, 1)  # open
    
    # Test success resets
    cb.on_success()
    assert_eq_int(cb.failure_count, 0)
    assert_eq_int(cb.state, 0)  # closed
    
    vibez.spill("✅ Circuit breaker test passed")
}

# Test error statistics
slay test_error_statistics() {
    test_start("Error statistics")
    
    sus stats = get_error_stats()
    sus initial_count = stats.total_errors
    
    # Record some errors
    sus io_err = yikes{message: "IO error", code: 1500, details: "IO details"}
    sus value_err = yikes{message: "Value error", code: 2500, details: "Value details"}
    
    record_error(io_err)
    record_error(value_err)
    
    sus updated_stats = get_error_stats()
    assert_eq_int(updated_stats.total_errors, initial_count + 2)
    
    vibez.spill("✅ Error statistics test passed")
}

# Test error context capture
slay test_error_context_capture() {
    test_start("Error context capture")
    
    sus context = capture_error_context("Test context")
    assert_eq_string(context.user_context, "Test context")
    assert_eq_int(context.goroutine_id, 1)
    assert_eq_string(context.function_name, "current_function")
    assert_eq_string(context.file_name, "current_file.csd")
    assert_eq_int(context.line_number, 1)
    
    vibez.spill("✅ Error context capture test passed")
}

# Test error severity levels
slay test_error_severity_levels() {
    test_start("Error severity levels")
    
    sus info_err = base_error{error_severity: info}
    sus warning_err = base_error{error_severity: warning}
    sus error_err = base_error{error_severity: error}
    sus critical_err = base_error{error_severity: critical}
    sus fatal_err = base_error{error_severity: fatal}
    
    assert_eq_int(info_err.severity(), 0)
    assert_eq_int(warning_err.severity(), 1)
    assert_eq_int(error_err.severity(), 2)
    assert_eq_int(critical_err.severity(), 3)
    assert_eq_int(fatal_err.severity(), 4)
    
    vibez.spill("✅ Error severity levels test passed")
}

# Test advanced error propagation with yikes/shook/fam
slay test_advanced_error_propagation() {
    test_start("Advanced error propagation")
    
    # Test function that might fail
    slay might_fail(should_fail lit) yikes {
        vibe_check should_fail {
            damn yikes{message: "Planned failure", code: 9999, details: "Test failure"}
        }
        damn cringe
    }
    
    # Test function with error propagation
    slay process_with_propagation(should_fail lit) yikes {
        sus err = might_fail(should_fail)
        vibe_check err != cringe {
            damn wrap_error(err, "Process failed")
        }
        damn cringe
    }
    
    # Test success case
    sus success_err = process_with_propagation(cap)
    assert_eq(success_err, cringe)
    
    # Test failure case
    sus failure_err = process_with_propagation(based)
    assert_eq_string(failure_err.message(), "Process failed: Planned failure")
    
    vibez.spill("✅ Advanced error propagation test passed")
}

# Test retry mechanism
slay test_retry_mechanism() {
    test_start("Retry mechanism")
    
    sus attempt_count normie = 0
    
    # Function that fails first few times
    slay flaky_operation() yikes {
        attempt_count++
        vibe_check attempt_count < 3 {
            damn yikes{message: "Temporary failure", code: 5500, details: "Network timeout"}
        }
        damn cringe
    }
    
    sus result = retry_with_backoff(flaky_operation, 5, 100)
    assert_eq(result, cringe)
    assert_eq_int(attempt_count, 3)
    
    vibez.spill("✅ Retry mechanism test passed")
}

# Test error hierarchy interface compliance
slay test_error_interface_compliance() {
    test_start("Error interface compliance")
    
    sus io_err = new_io_error("IO test", "/tmp", "read")
    sus value_err = new_value_error("Value test", "bad", "good")
    sus type_err = new_type_error("Type test", "actual", "expected")
    sus memory_err = new_memory_error("Memory test", 1024, 512)
    
    # Test that all error types implement the interface
    assert_eq_string(io_err.base.message(), "IO test")
    assert_eq_string(value_err.base.message(), "Value test")
    assert_eq_string(type_err.base.message(), "Type test")
    assert_eq_string(memory_err.base.message(), "Memory test")
    
    # Test that all have proper codes
    assert_eq_int(io_err.base.code(), 1001)
    assert_eq_int(value_err.base.code(), 2001)
    assert_eq_int(type_err.base.code(), 3001)
    assert_eq_int(memory_err.base.code(), 4001)
    
    vibez.spill("✅ Error interface compliance test passed")
}

# Main test runner
test_basic_error_creation()
test_io_error_creation()
test_value_error_creation()
test_type_error_creation()
test_memory_error_creation()
test_network_error_creation()
test_parse_error_creation()
test_security_error_creation()
test_runtime_error_creation()
test_error_type_detection()
test_error_wrapping()
test_error_chaining()
test_error_combining()
test_temporary_error_detection()
test_critical_error_detection()
test_error_formatting()
test_json_error_formatting()
test_circuit_breaker()
test_error_statistics()
test_error_context_capture()
test_error_severity_levels()
test_advanced_error_propagation()
test_retry_mechanism()
test_error_interface_compliance()

print_test_summary()
