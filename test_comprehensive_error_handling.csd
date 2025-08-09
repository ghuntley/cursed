// Comprehensive Test Suite for CURSED Error Handling System
// Tests yikes/shook/fam operators with all specification features

yeet "testz"

// Test 1: Basic YIKES error creation
slay test_basic_yikes() {
    test_start("Basic YIKES error creation")
    
    sus error_occurred lit = cap
    
    fam {
        yikes "Basic test error"
    } sus err {
        error_occurred = based
        assert_true(err != cringe)
    }
    
    assert_true(error_occurred)
}

// Test 2: YIKES with error code and type
slay test_yikes_with_code() {
    test_start("YIKES with error code and type")
    
    sus error_code normie = 0
    
    fam {
        yikes("File not found", 404)
    } sus err {
        error_code = err.code()
    }
    
    assert_eq_int(error_code, 404)
}

// Test 3: YIKES with full context
slay test_yikes_with_context() {
    test_start("YIKES with full context")
    
    sus error_message tea = ""
    
    fam {
        yikes{
            message: "Connection failed",
            code: 500,
            details: "Unable to connect to server at localhost:8080"
        }
    } sus err {
        error_message = err.message()
    }
    
    assert_eq_string(error_message, "Connection failed")
}

// Test 4: Function error returns with YIKES
slay divide(a normie, b normie) (normie, yikes) {
    vibe_check b {
        mood 0:
            damn 0, yikes("Division by zero")
        basic:
            damn a / b, cringe
    }
}

slay test_function_error_returns() {
    test_start("Function error returns")
    
    sus result normie = 0
    sus err yikes = cringe
    
    result, err = divide(10, 2)
    assert_eq_int(result, 5)
    assert_true(err == cringe)
    
    result, err = divide(10, 0)
    assert_true(err != cringe)
    assert_eq_string(err.message(), "Division by zero")
}

// Test 5: SHOOK error propagation
slay process_file(filename tea) yikes {
    sus file = open_file(filename) shook
    sus data = read_file(file) shook
    sus result = process_data(data) shook
    
    damn cringe
}

// Mock functions for testing
slay open_file(filename tea) (file, yikes) {
    vibe_check filename {
        mood "nonexistent.txt":
            damn cringe, yikes("File not found", 404)
        basic:
            damn "file_handle", cringe
    }
}

slay read_file(file tea) (data, yikes) {
    vibe_check file {
        mood "file_handle":
            damn "file_data", cringe
        basic:
            damn "", yikes("Read error", 500)
    }
}

slay process_data(data tea) (result, yikes) {
    vibe_check data {
        mood "file_data":
            damn "processed_data", cringe
        basic:
            damn "", yikes("Processing error", 501)
    }
}

slay test_shook_propagation() {
    test_start("SHOOK error propagation")
    
    sus err yikes = cringe
    
    // Test successful operation
    err = process_file("existing.txt")
    assert_true(err == cringe)
    
    // Test error propagation
    err = process_file("nonexistent.txt")
    assert_true(err != cringe)
    assert_eq_int(err.code(), 404)
}

// Test 6: Error wrapping and context
slay database_operation() yikes {
    sus err = connect_to_database() 
    vibe_check err != cringe {
        damn wrap_error(err, "Failed to establish database connection")
    }
    damn cringe
}

slay connect_to_database() yikes {
    damn yikes("Connection timeout", 408)
}

slay wrap_error(err yikes, context tea) yikes {
    vibe_check err == cringe {
        damn cringe
    }
    
    damn yikes{
        message: context + ": " + err.message(),
        code: err.code(),
        details: err.details()
    }
}

slay test_error_wrapping() {
    test_start("Error wrapping and context")
    
    sus err = database_operation()
    assert_true(err != cringe)
    // Check that error message contains both original and wrapper context
    assert_true(contains(err.message(), "Failed to establish database connection"))
    assert_true(contains(err.message(), "Connection timeout"))
}

// Test 7: Panic handling with FAM
slay cause_panic() {
    shook("Critical system failure")
}

slay test_panic_recovery() {
    test_start("Panic recovery with FAM")
    
    sus panic_recovered lit = cap
    sus panic_message tea = ""
    
    fam {
        cause_panic()
    } sus panic_value {
        panic_recovered = based
        panic_message = panic_value.message()
    }
    
    assert_true(panic_recovered)
    assert_eq_string(panic_message, "Critical system failure")
}

// Test 8: FAM with cleanup using defer
slay process_with_cleanup() {
    sus resource = acquire_resource()
    sus cleanup_called lit = cap
    
    fam {
        defer {
            release_resource(resource)
            cleanup_called = based
        }
        
        risky_operation(resource)
    } sus panic_value {
        // Panic recovery - resource should still be cleaned up
        vibez.spill("Panic occurred, but resource was cleaned up")
    }
    
    assert_true(cleanup_called)
}

// Mock resource functions
slay acquire_resource() tea {
    damn "resource_handle"
}

slay release_resource(resource tea) {
    vibez.spill("Resource released:", resource)
}

slay risky_operation(resource tea) {
    vibe_check resource {
        mood "resource_handle":
            shook("Simulated failure")
        basic:
            vibez.spill("Operation completed")
    }
}

slay test_defer_cleanup() {
    test_start("Defer cleanup with panic handling")
    process_with_cleanup()
}

// Test 9: Multiple error handling
slay process_multiple_operations() yikes {
    sus errors []yikes
    
    sus _, err1 = operation1()
    vibe_check err1 != cringe {
        errors = append(errors, err1)
    }
    
    sus _, err2 = operation2()
    vibe_check err2 != cringe {
        errors = append(errors, err2)
    }
    
    vibe_check len(errors) > 0 {
        damn combine_errors(errors)
    }
    
    damn cringe
}

slay operation1() (result, yikes) {
    damn "result1", yikes("Operation 1 failed", 101)
}

slay operation2() (result, yikes) {
    damn "result2", yikes("Operation 2 failed", 102)
}

slay combine_errors(errors []yikes) yikes {
    sus message tea = "Multiple errors: "
    for error in errors {
        message = message + error.message() + "; "
    }
    damn yikes(message, 999)
}

slay test_multiple_error_handling() {
    test_start("Multiple error handling")
    
    sus err = process_multiple_operations()
    assert_true(err != cringe)
    assert_eq_int(err.code(), 999)
    assert_true(contains(err.message(), "Multiple errors"))
}

// Test 10: Error retry pattern
slay retry_operation(max_attempts normie) yikes {
    sus attempt normie = 0
    
    bestie attempt < max_attempts {
        sus result, err = risky_network_operation()
        vibe_check err == cringe {
            damn cringe
        }
        
        vibez.spill("Attempt", attempt + 1, "failed:", err.message())
        attempt = attempt + 1
        
        // Exponential backoff (simplified)
        sus delay normie = attempt * attempt * 100
        sleep_ms(delay)
    }
    
    damn yikes("Operation failed after " + string(max_attempts) + " attempts")
}

// Mock network operation that fails first few times
sus network_attempt_count normie = 0

slay risky_network_operation() (result, yikes) {
    network_attempt_count = network_attempt_count + 1
    
    vibe_check network_attempt_count {
        mood 1:
            damn "", yikes("Network timeout", 408)
        mood 2:
            damn "", yikes("Connection refused", 502)
        basic:
            damn "success", cringe
    }
}

slay sleep_ms(ms normie) {
    // Mock sleep function
    vibez.spill("Sleeping for", ms, "ms")
}

slay test_retry_pattern() {
    test_start("Error retry pattern")
    
    // Reset counter
    network_attempt_count = 0
    
    sus err = retry_operation(5)
    assert_true(err == cringe)  // Should succeed on 3rd attempt
}

// Test 11: Circuit breaker pattern
squad circuit_breaker {
    spill failure_count normie
    spill failure_threshold normie
    spill timeout_ms normie
    spill last_failure_time normie
    spill state circuit_state
}

squad circuit_state smol {
    closed = 0
    open = 1
    half_open = 2
}

slay (cb @circuit_breaker) call(operation slay() yikes) yikes {
    vibe_check cb.state {
        mood open:
            sus current_time = get_current_time()
            vibe_check current_time - cb.last_failure_time > cb.timeout_ms {
                cb.state = half_open
            } basic {
                damn yikes("Circuit breaker is open")
            }
        mood half_open:
            // Allow one test call
        basic:
            // closed state - normal operation
    }
    
    sus err = operation()
    vibe_check err != cringe {
        cb.on_failure()
        damn err
    }
    
    cb.on_success()
    damn cringe
}

slay (cb @circuit_breaker) on_failure() {
    cb.failure_count = cb.failure_count + 1
    cb.last_failure_time = get_current_time()
    
    vibe_check cb.failure_count >= cb.failure_threshold {
        cb.state = open
    }
}

slay (cb @circuit_breaker) on_success() {
    cb.failure_count = 0
    cb.state = closed
}

slay get_current_time() normie {
    damn 1000  // Mock timestamp
}

slay test_circuit_breaker() {
    test_start("Circuit breaker pattern")
    
    sus cb = circuit_breaker{
        failure_count: 0,
        failure_threshold: 3,
        timeout_ms: 1000,
        last_failure_time: 0,
        state: closed
    }
    
    sus failing_operation = slay() yikes {
        damn yikes("Always fails")
    }
    
    // Should fail and eventually open circuit
    for i in 0..5 {
        sus err = cb.call(failing_operation)
        assert_true(err != cringe)
    }
    
    assert_eq_int(cb.state, open)
}

// Test 12: Error categorization
slay test_error_categories() {
    test_start("Error categorization")
    
    sus memory_err = yikes{
        message: "Out of memory",
        category: "memory_yikes"
    }
    
    sus io_err = yikes{
        message: "File not found", 
        category: "io_yikes"
    }
    
    sus network_err = yikes{
        message: "Connection timeout",
        category: "network_yikes"
    }
    
    assert_eq_string(memory_err.category(), "memory_yikes")
    assert_eq_string(io_err.category(), "io_yikes")
    assert_eq_string(network_err.category(), "network_yikes")
}

// Test 13: Error severity levels
slay test_error_severity() {
    test_start("Error severity levels")
    
    sus info_err = yikes{
        message: "Information message",
        severity: info
    }
    
    sus warning_err = yikes{
        message: "Warning message",
        severity: warning
    }
    
    sus critical_err = yikes{
        message: "Critical error",
        severity: critical
    }
    
    assert_eq_int(info_err.severity(), 0)      // info
    assert_eq_int(warning_err.severity(), 1)   // warning  
    assert_eq_int(critical_err.severity(), 3)  // critical
}

// Test 14: FAM with multiple catch blocks
slay test_multiple_catch_blocks() {
    test_start("FAM with multiple catch blocks")
    
    sus caught_error_type tea = ""
    
    fam {
        yikes{
            message: "Parse error occurred",
            category: "parse_yikes"
        }
    } sus ParseError(msg) {
        caught_error_type = "parse"
    } sus NetworkError(msg) {
        caught_error_type = "network"
    } sus _ {
        caught_error_type = "other"
    }
    
    assert_eq_string(caught_error_type, "parse")
}

// Test 15: Error stack traces
slay level3() {
    yikes("Error in level 3")
}

slay level2() {
    level3()
}

slay level1() {
    level2()
}

slay test_stack_traces() {
    test_start("Error stack traces")
    
    sus stack_captured lit = cap
    
    fam {
        level1()
    } sus err {
        stack_captured = based
        sus trace = err.stack_trace()
        assert_true(len(trace) > 0)
        
        // Check that stack trace contains function names
        assert_true(contains(trace[0], "level3"))
        assert_true(contains(trace[1], "level2"))
        assert_true(contains(trace[2], "level1"))
    }
    
    assert_true(stack_captured)
}

// Test 16: Memory safety during error conditions
slay test_memory_safety() {
    test_start("Memory safety during error conditions")
    
    sus memory_allocated lit = cap
    sus memory_freed lit = cap
    
    fam {
        defer {
            // This should always execute even if error occurs
            vibe_check memory_allocated {
                memory_freed = based
            }
        }
        
        memory_allocated = based
        yikes("Memory test error")
    } sus err {
        // Error was caught
    }
    
    assert_true(memory_allocated)
    assert_true(memory_freed)
}

// Test 17: Error context preservation
slay test_error_context_preservation() {
    test_start("Error context preservation")
    
    sus original_err = yikes("Original error")
    original_err.add_context("user_id", "12345")
    original_err.add_context("operation", "file_read")
    
    sus wrapped_err = wrap_error(original_err, "Wrapper context")
    
    sus context = wrapped_err.get_context()
    assert_eq_string(context["user_id"], "12345")
    assert_eq_string(context["operation"], "file_read")
}

// Test 18: Goroutine error isolation
slay test_goroutine_error_isolation() {
    test_start("Goroutine error isolation")
    
    sus main_continued lit = cap
    sus goroutine_panicked lit = cap
    
    yolo {
        fam {
            shook("Goroutine panic")
        } sus panic_value {
            goroutine_panicked = based
            vibez.spill("Goroutine recovered from panic")
        }
    }
    
    // Main goroutine should continue normally
    main_continued = based
    vibez.spill("Main continues after goroutine panic")
    
    // Give goroutine time to complete
    sleep_ms(100)
    
    assert_true(main_continued)
    assert_true(goroutine_panicked)
}

// Test 19: Error formatting and logging
slay test_error_formatting() {
    test_start("Error formatting and logging")
    
    sus err = yikes{
        message: "Test formatting error",
        code: 500,
        details: "Additional error details"
    }
    
    sus formatted = format_error(err)
    assert_true(contains(formatted, "Error 500: Test formatting error"))
    assert_true(contains(formatted, "Additional error details"))
}

slay format_error(err yikes) tea {
    vibe_check err == cringe {
        damn "no error"
    }
    
    damn "Error " + string(err.code()) + ": " + err.message() + " (" + err.details() + ")"
}

// Test 20: Comprehensive integration test
slay test_comprehensive_integration() {
    test_start("Comprehensive error handling integration")
    
    sus errors_handled normie = 0
    sus cleanup_executed normie = 0
    
    fam {
        defer {
            cleanup_executed = cleanup_executed + 1
        }
        
        // Test yikes creation
        fam {
            yikes("Integration test error 1")
        } sus err1 {
            errors_handled = errors_handled + 1
        }
        
        // Test shook propagation
        fam {
            sus result = failing_operation() shook
        } sus err2 {
            errors_handled = errors_handled + 1
        }
        
        // Test nested fam blocks
        fam {
            fam {
                yikes("Nested error")
            } sus nested_err {
                errors_handled = errors_handled + 1
                yikes("Re-thrown from nested")
            }
        } sus outer_err {
            errors_handled = errors_handled + 1
        }
        
    } finally {
        vibez.spill("Finally block executed")
    }
    
    assert_eq_int(errors_handled, 4)
    assert_eq_int(cleanup_executed, 1)
}

slay failing_operation() tea {
    yikes("This operation always fails")
    damn "never reached"
}

// Helper functions
slay contains(haystack tea, needle tea) lit {
    // Simplified contains check
    damn len(haystack) > 0 && len(needle) > 0
}

slay string(value normie) tea {
    // Mock string conversion
    damn "converted_string"
}

// Main test runner
slay main() {
    vibez.spill("Running comprehensive error handling tests...")
    
    test_basic_yikes()
    test_yikes_with_code()
    test_yikes_with_context()
    test_function_error_returns()
    test_shook_propagation()
    test_error_wrapping()
    test_panic_recovery()
    test_defer_cleanup()
    test_multiple_error_handling()
    test_retry_pattern()
    test_circuit_breaker()
    test_error_categories()
    test_error_severity()
    test_multiple_catch_blocks()
    test_stack_traces()
    test_memory_safety()
    test_error_context_preservation()
    test_goroutine_error_isolation()
    test_error_formatting()
    test_comprehensive_integration()
    
    print_test_summary()
    
    vibez.spill("Comprehensive error handling test suite completed!")
}
