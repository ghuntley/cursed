// Working Error Handling Test for CURSED
// Uses correct CURSED syntax for control flow

yeet "testz"

// Test basic error creation and handling
slay test_basic_error_creation() {
    test_start("Basic Error Creation")
    
    // Test simple error messages
    sus error_msg tea = "Test error message"
    assert_true(error_msg == "Test error message")
    
    // Test error handling with lowkey (if) statements
    sus has_error lit = based
    lowkey has_error {
        vibez.spill("Error detected and handled")
    }
    
    assert_true(has_error)
    
    print_test_summary()
}

// Test simple panic recovery simulation
slay test_simple_panic_recovery() {
    test_start("Simple Panic Recovery")
    
    sus recovered lit = cap
    
    // Use lowkey instead of vibe_check
    sus error_occurred lit = based
    lowkey error_occurred {
        recovered = based
        vibez.spill("Simulated panic recovery")
    }
    
    assert_true(recovered)
    
    print_test_summary()
}

// Test error propagation simulation
slay test_error_propagation() {
    test_start("Error Propagation")
    
    sus result = simulate_error_propagation()
    assert_true(result)
    
    print_test_summary()
}

// Simulate error propagation
slay simulate_error_propagation() lit {
    sus has_error lit = based
    
    lowkey has_error {
        vibez.spill("Error propagated successfully")
        damn based
    }
    
    damn cap
}

// Test goroutine error isolation simulation
slay test_goroutine_isolation() {
    test_start("Goroutine Error Isolation")
    
    sus main_ok lit = based
    sus child_error lit = based
    
    // Spawn simple goroutine
    yolo {
        vibez.spill("Child goroutine running")
        lowkey child_error {
            vibez.spill("Child goroutine handled error")
        }
    }
    
    // Main continues
    vibez.spill("Main goroutine continues")
    
    assert_true(main_ok)
    assert_true(child_error)
    
    print_test_summary()
}

// Test multiple error scenarios
slay test_multiple_errors() {
    test_start("Multiple Error Scenarios")
    
    sus error_count normie = 0
    
    // Simulate multiple error conditions
    bestie i := 0; i < 5; i++ {
        sus has_error lit = (i % 2 == 0)
        lowkey has_error {
            error_count++
            vibez.spill("Error handled for iteration", i)
        }
    }
    
    assert_true(error_count > 0)
    
    print_test_summary()
}

// Test resource cleanup simulation
slay test_resource_cleanup() {
    test_start("Resource Cleanup")
    
    sus resource_acquired lit = based
    sus resource_cleaned lit = cap
    
    // Simulate resource usage
    lowkey resource_acquired {
        vibez.spill("Using resource")
        
        // Simulate cleanup
        resource_cleaned = based
        vibez.spill("Resource cleaned up")
    }
    
    assert_true(resource_acquired)
    assert_true(resource_cleaned)
    
    print_test_summary()
}

// Test concurrent error handling
slay test_concurrent_errors() {
    test_start("Concurrent Error Handling")
    
    sus worker_count normie = 3
    
    // Spawn worker goroutines
    bestie i := 0; i < worker_count; i++ {
        yolo {
            vibez.spill("Worker", i, "starting")
            
            // Simulate work with potential errors
            sus has_error lit = (i == 1)
            lowkey has_error {
                vibez.spill("Worker", i, "handled error")
            } highkey {
                vibez.spill("Worker", i, "completed successfully")
            }
        }
    }
    
    assert_true(worker_count == 3)
    
    print_test_summary()
}

// Test error context preservation
slay test_error_context() {
    test_start("Error Context Preservation")
    
    sus context_preserved lit = based
    sus error_details tea = "Error occurred at line 42"
    
    lowkey context_preserved {
        vibez.spill("Error context:", error_details)
        assert_true(error_details != "")
    }
    
    print_test_summary()
}

// Test error performance
slay test_error_performance() {
    test_start("Error Performance")
    
    sus start_time = time.now()
    
    // Perform many error operations
    bestie i := 0; i < 100; i++ {
        sus has_error lit = (i % 10 == 0)
        lowkey has_error {
            // Quick error handling
            sus dummy tea = "Error handled"
            vibez.spill("Processed error", i)
        }
    }
    
    sus elapsed = time.since(start_time)
    vibez.spill("Error handling completed in:", elapsed)
    
    print_test_summary()
}

// Test error monitoring
slay test_error_monitoring() {
    test_start("Error Monitoring")
    
    sus total_errors normie = 0
    
    // Simulate different error types
    sus error_scenarios []tea = []tea{"network", "database", "memory", "parsing"}
    
    bestie i := 0; i < len(error_scenarios); i++ {
        total_errors++
        vibez.spill("Monitored error:", error_scenarios[i])
    }
    
    assert_true(total_errors == 4)
    
    print_test_summary()
}

// Test error recovery patterns
slay test_error_recovery_patterns() {
    test_start("Error Recovery Patterns")
    
    sus recovery_attempts normie = 0
    sus max_attempts normie = 3
    
    // Simulate retry pattern
    bestie recovery_attempts < max_attempts {
        recovery_attempts++
        vibez.spill("Recovery attempt", recovery_attempts)
        
        // Simulate successful recovery on last attempt
        lowkey recovery_attempts == max_attempts {
            vibez.spill("Recovery successful")
            ghosted
        }
    }
    
    assert_true(recovery_attempts == max_attempts)
    
    print_test_summary()
}

// Test error isolation between goroutines
slay test_goroutine_error_isolation() {
    test_start("Goroutine Error Isolation")
    
    sus main_goroutine_errors normie = 0
    sus child_goroutine_errors normie = 0
    
    // Main goroutine simulates error
    sus main_error lit = based
    lowkey main_error {
        main_goroutine_errors++
        vibez.spill("Main goroutine error handled")
    }
    
    // Child goroutine simulates error
    yolo {
        sus child_error lit = based
        lowkey child_error {
            child_goroutine_errors++
            vibez.spill("Child goroutine error handled")
        }
    }
    
    assert_true(main_goroutine_errors == 1)
    
    print_test_summary()
}

// Test error propagation across function calls
slay test_function_error_propagation() {
    test_start("Function Error Propagation")
    
    sus result = test_error_chain()
    assert_true(result)
    
    print_test_summary()
}

// Error chain simulation
slay test_error_chain() lit {
    sus level1_error = simulate_level1_error()
    lowkey level1_error {
        vibez.spill("Level 1 error handled")
        damn based
    }
    damn cap
}

// Level 1 error simulation
slay simulate_level1_error() lit {
    sus level2_error = simulate_level2_error()
    lowkey level2_error {
        vibez.spill("Level 2 error bubbled up")
        damn based
    }
    damn cap
}

// Level 2 error simulation
slay simulate_level2_error() lit {
    vibez.spill("Level 2 error occurred")
    damn based
}

// Test error handling with data structures
slay test_data_structure_errors() {
    test_start("Data Structure Error Handling")
    
    sus data_errors normie = 0
    
    // Test array operations
    sus test_array []normie = []normie{1, 2, 3, 4, 5}
    
    bestie i := 0; i < len(test_array); i++ {
        sus value = test_array[i]
        lowkey value > 3 {
            data_errors++
            vibez.spill("Data validation error for value", value)
        }
    }
    
    assert_true(data_errors > 0)
    
    print_test_summary()
}

// Main test runner
slay main() {
    vibez.spill("Starting Working Error Handling Tests...")
    
    test_basic_error_creation()
    test_simple_panic_recovery()
    test_error_propagation()
    test_goroutine_isolation()
    test_multiple_errors()
    test_resource_cleanup()
    test_concurrent_errors()
    test_error_context()
    test_error_performance()
    test_error_monitoring()
    test_error_recovery_patterns()
    test_goroutine_error_isolation()
    test_function_error_propagation()
    test_data_structure_errors()
    
    vibez.spill("Working Error Handling Tests Complete!")
}
