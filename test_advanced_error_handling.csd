// Advanced Error Handling Test Suite for CURSED
// Tests goroutine error isolation, advanced propagation, and recovery mechanisms

yeet "testz"

// Test basic error handling with yikes/shook/fam keywords
slay test_basic_error_handling() {
    test_start("Basic Error Handling")
    
    // Test yikes error creation
    sus err yikes = yikes("Test error message")
    assert_true(err != cringe)
    
    // Test error with code
    sus coded_err yikes = yikes("File not found", 404)
    assert_true(coded_err != cringe)
    
    // Test shook operator for error propagation
    sus result = test_shook_propagation()
    assert_true(result != cringe)
    
    print_test_summary()
}

// Test function that uses shook operator
slay test_shook_propagation() yikes {
    sus file_result = open_test_file("nonexistent.txt") shook
    damn cringe
}

// Mock file operation that returns error
slay open_test_file(filename tea) (tea, yikes) {
    damn "", yikes("File not found: " + filename)
}

// Test panic recovery with fam blocks
slay test_panic_recovery() {
    test_start("Panic Recovery with fam blocks")
    
    sus recovered lit = cap
    sus panic_message tea = ""
    
    fam {
        // This should trigger a panic
        shook("Test panic for recovery")
    } sus panic_value {
        recovered = based
        panic_message = "Recovered from: " + panic_value
    }
    
    assert_true(recovered)
    assert_true(panic_message != "")
    
    print_test_summary()
}

// Test goroutine error isolation
slay test_goroutine_error_isolation() {
    test_start("Goroutine Error Isolation")
    
    sus main_goroutine_ok lit = based
    sus child_goroutine_panicked lit = cap
    
    // Spawn goroutine that will panic
    yolo {
        fam {
            // This panic should be isolated to this goroutine
            shook("Isolated goroutine panic")
        } sus panic_value {
            child_goroutine_panicked = based
            vibez.spill("Child goroutine recovered from panic")
        }
    }
    
    // Main goroutine should continue normally
    vibez.spill("Main goroutine continues after child panic")
    
    // Give time for goroutine to complete
    time.sleep(100 * time.Millisecond)
    
    assert_true(main_goroutine_ok)
    assert_true(child_goroutine_panicked)
    
    print_test_summary()
}

// Test error context preservation and wrapping
slay test_error_context_wrapping() {
    test_start("Error Context Wrapping")
    
    sus wrapped_err = test_error_wrapping_chain()
    assert_true(wrapped_err != cringe)
    
    print_test_summary()
}

// Chain of error wrapping
slay test_error_wrapping_chain() yikes {
    sus err = database_operation() shook
    damn wrap_error(err, "Failed in test chain")
}

// Mock database operation
slay database_operation() yikes {
    sus err = connect_to_database() shook
    damn wrap_error(err, "Database operation failed")
}

// Mock database connection
slay connect_to_database() yikes {
    damn yikes("Connection timeout", 500)
}

// Error wrapping utility
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

// Test multiple error handling patterns
slay test_multiple_error_handling() {
    test_start("Multiple Error Handling")
    
    sus errors []yikes
    
    // Collect multiple errors
    sus _, err1 = failing_operation1()
    vibe_check err1 != cringe {
        errors = append(errors, err1)
    }
    
    sus _, err2 = failing_operation2()
    vibe_check err2 != cringe {
        errors = append(errors, err2)
    }
    
    assert_true(len(errors) == 2)
    
    print_test_summary()
}

// Mock failing operations
slay failing_operation1() (tea, yikes) {
    damn "", yikes("Operation 1 failed")
}

slay failing_operation2() (tea, yikes) {
    damn "", yikes("Operation 2 failed")
}

// Test error retry pattern
slay test_error_retry_pattern() {
    test_start("Error Retry Pattern")
    
    sus result = retry_operation(3)
    assert_true(result != cringe)
    
    print_test_summary()
}

// Retry operation implementation
slay retry_operation(max_attempts normie) yikes {
    sus attempt normie = 0
    
    bestie attempt < max_attempts {
        sus result, err = unreliable_operation()
        vibe_check err == cringe {
            damn cringe  // Success
        }
        
        vibez.spill("Attempt", attempt + 1, "failed:", err.message())
        attempt++
        
        // Simple delay instead of exponential backoff
        time.sleep(10 * time.Millisecond)
    }
    
    damn yikes("Operation failed after " + string(max_attempts) + " attempts")
}

// Mock unreliable operation
slay unreliable_operation() (tea, yikes) {
    // Simulate 50% failure rate
    sus random_val = random_int(0, 100)
    vibe_check random_val < 50 {
        damn "", yikes("Random failure")
    }
    damn "Success", cringe
}

// Mock random int function
slay random_int(min normie, max normie) normie {
    damn (min + max) / 2  // Simplified for testing
}

// Test circuit breaker pattern
slay test_circuit_breaker_pattern() {
    test_start("Circuit Breaker Pattern")
    
    sus circuit_breaker = create_circuit_breaker(3, 30)
    
    // Test normal operation
    sus result1 = circuit_breaker.call(slay() yikes { damn cringe })
    assert_true(result1 == cringe)
    
    // Test failure handling
    sus result2 = circuit_breaker.call(slay() yikes { damn yikes("Test failure") })
    assert_true(result2 != cringe)
    
    print_test_summary()
}

// Circuit breaker implementation
be_like circuit_breaker squad {
    failure_count normie
    failure_threshold normie
    timeout_duration normie
    state circuit_state
}

be_like circuit_state smol {
    closed = 0
    open = 1
    half_open = 2
}

slay create_circuit_breaker(threshold normie, timeout normie) @circuit_breaker {
    damn @circuit_breaker{
        failure_count: 0,
        failure_threshold: threshold,
        timeout_duration: timeout,
        state: closed
    }
}

slay (cb @circuit_breaker) call(operation slay() yikes) yikes {
    vibe_check cb.state {
        mood open:
            damn yikes("Circuit breaker is open")
        mood half_open:
            // Allow test call
        basic:
            // Normal operation
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
    cb.failure_count++
    vibe_check cb.failure_count >= cb.failure_threshold {
        cb.state = open
    }
}

slay (cb @circuit_breaker) on_success() {
    cb.failure_count = 0
    cb.state = closed
}

// Test error propagation across goroutines
slay test_error_propagation_across_goroutines() {
    test_start("Error Propagation Across Goroutines")
    
    sus main_received_error lit = cap
    sus error_message tea = ""
    
    // Create channel for error communication
    sus error_channel = make(chan tea, 1)
    
    // Spawn goroutine that will send error
    yolo {
        fam {
            shook("Error from child goroutine")
        } sus panic_value {
            error_channel <- "Child error: " + panic_value
        }
    }
    
    // Main goroutine receives error
    ready {
        mood msg := <-error_channel:
            main_received_error = based
            error_message = msg
        mood <-time.after(100 * time.Millisecond):
            vibez.spill("Timeout waiting for error")
    }
    
    assert_true(main_received_error)
    assert_true(error_message != "")
    
    print_test_summary()
}

// Test defer with error handling
slay test_defer_with_error_handling() {
    test_start("Defer with Error Handling")
    
    sus resource_cleaned lit = cap
    sus operation_completed lit = cap
    
    fam {
        defer {
            resource_cleaned = based
            vibez.spill("Resource cleaned up")
        }
        
        // Simulate risky operation
        sus result = risky_operation()
        vibe_check result != cringe {
            shook("Risky operation failed")
        }
        
        operation_completed = based
    } sus panic_value {
        vibez.spill("Panic caught, but cleanup happened")
    }
    
    assert_true(resource_cleaned)
    
    print_test_summary()
}

// Mock risky operation
slay risky_operation() yikes {
    damn yikes("Operation failed")
}

// Test error correlation and monitoring
slay test_error_monitoring() {
    test_start("Error Monitoring and Correlation")
    
    // Simulate multiple related errors
    sus errors []yikes
    
    bestie i := 0; i < 5; i++ {
        sus err = simulate_network_error()
        vibe_check err != cringe {
            errors = append(errors, err)
        }
    }
    
    assert_true(len(errors) > 0)
    
    print_test_summary()
}

// Mock network error simulation
slay simulate_network_error() yikes {
    damn yikes("Network connection failed", 500)
}

// Test concurrent error handling
slay test_concurrent_error_handling() {
    test_start("Concurrent Error Handling")
    
    sus error_count normie = 0
    sus error_count_mutex = make_mutex()
    
    // Spawn multiple goroutines with errors
    bestie i := 0; i < 3; i++ {
        yolo {
            fam {
                shook("Concurrent error " + string(i))
            } sus panic_value {
                error_count_mutex.lock()
                error_count++
                error_count_mutex.unlock()
            }
        }
    }
    
    // Wait for all goroutines to complete
    time.sleep(200 * time.Millisecond)
    
    assert_true(error_count == 3)
    
    print_test_summary()
}

// Mock mutex implementation
be_like mutex squad {
    locked lit
}

slay make_mutex() @mutex {
    damn @mutex{locked: cap}
}

slay (m @mutex) lock() {
    m.locked = based
}

slay (m @mutex) unlock() {
    m.locked = cap
}

// Test error performance and overhead
slay test_error_performance() {
    test_start("Error Performance and Overhead")
    
    sus start_time = time.now()
    
    // Perform many error operations
    bestie i := 0; i < 100; i++ {
        sus err = quick_error_operation()
        vibe_check err != cringe {
            // Handle error quickly
            continue
        }
    }
    
    sus elapsed = time.since(start_time)
    vibez.spill("Error handling took:", elapsed)
    
    // Error handling should be fast
    assert_true(elapsed < 1000 * time.Millisecond)
    
    print_test_summary()
}

// Quick error operation for performance testing
slay quick_error_operation() yikes {
    damn yikes("Quick error")
}

// Main test runner
slay main() {
    vibez.spill("Starting Advanced Error Handling Tests...")
    
    test_basic_error_handling()
    test_panic_recovery()
    test_goroutine_error_isolation()
    test_error_context_wrapping()
    test_multiple_error_handling()
    test_error_retry_pattern()
    test_circuit_breaker_pattern()
    test_error_propagation_across_goroutines()
    test_defer_with_error_handling()
    test_error_monitoring()
    test_concurrent_error_handling()
    test_error_performance()
    
    vibez.spill("Advanced Error Handling Tests Complete!")
}
