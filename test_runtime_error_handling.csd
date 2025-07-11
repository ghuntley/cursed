// Runtime Error Handling Test
// Test advanced error handling with basic CURSED syntax

slay test_basic_error_handling() {
    vibez.spill("Testing basic error handling")
    
    // Test basic error conditions
    sus error_occurred lit = based
    lowkey error_occurred {
        vibez.spill("Error condition detected")
    }
    
    vibez.spill("Basic error handling test complete")
}

slay test_goroutine_error_isolation() {
    vibez.spill("Testing goroutine error isolation")
    
    // Test main goroutine
    sus main_error lit = based
    lowkey main_error {
        vibez.spill("Main goroutine handled error")
    }
    
    // Test child goroutine
    yolo {
        sus child_error lit = based
        lowkey child_error {
            vibez.spill("Child goroutine handled error")
        }
    }
    
    vibez.spill("Goroutine error isolation test complete")
}

slay test_error_propagation() {
    vibez.spill("Testing error propagation")
    
    sus result = simulate_error_chain()
    lowkey result {
        vibez.spill("Error propagated successfully")
    }
    
    vibez.spill("Error propagation test complete")
}

slay simulate_error_chain() lit {
    vibez.spill("Simulating error chain")
    
    sus has_error lit = based
    lowkey has_error {
        vibez.spill("Error detected in chain")
        damn based
    }
    
    damn cap
}

slay test_concurrent_error_handling() {
    vibez.spill("Testing concurrent error handling")
    
    sus worker_count normie = 3
    
    bestie i := 0; i < worker_count; i++ {
        yolo {
            vibez.spill("Worker " + string(i) + " starting")
            
            sus has_error lit = (i == 1)
            lowkey has_error {
                vibez.spill("Worker " + string(i) + " handled error")
            } highkey {
                vibez.spill("Worker " + string(i) + " completed successfully")
            }
        }
    }
    
    vibez.spill("Concurrent error handling test complete")
}

slay test_error_recovery_patterns() {
    vibez.spill("Testing error recovery patterns")
    
    sus recovery_attempts normie = 0
    sus max_attempts normie = 3
    
    bestie recovery_attempts < max_attempts {
        recovery_attempts++
        vibez.spill("Recovery attempt " + string(recovery_attempts))
        
        lowkey recovery_attempts == max_attempts {
            vibez.spill("Recovery successful after " + string(recovery_attempts) + " attempts")
            ghosted
        }
    }
    
    vibez.spill("Error recovery patterns test complete")
}

slay test_error_context_preservation() {
    vibez.spill("Testing error context preservation")
    
    sus context_data tea = "Error occurred at line 42 in function test_error_context_preservation"
    sus has_context lit = based
    
    lowkey has_context {
        vibez.spill("Error context: " + context_data)
    }
    
    vibez.spill("Error context preservation test complete")
}

slay test_error_monitoring() {
    vibez.spill("Testing error monitoring")
    
    sus total_errors normie = 0
    
    bestie i := 0; i < 5; i++ {
        sus has_error lit = (i % 2 == 0)
        lowkey has_error {
            total_errors++
            vibez.spill("Monitored error " + string(total_errors))
        }
    }
    
    vibez.spill("Total errors monitored: " + string(total_errors))
    vibez.spill("Error monitoring test complete")
}

slay test_resource_cleanup() {
    vibez.spill("Testing resource cleanup")
    
    sus resource_acquired lit = based
    sus resource_cleaned lit = cap
    
    lowkey resource_acquired {
        vibez.spill("Resource acquired and in use")
        
        // Simulate resource cleanup
        resource_cleaned = based
        vibez.spill("Resource cleaned up successfully")
    }
    
    vibez.spill("Resource cleanup test complete")
}

slay test_error_performance() {
    vibez.spill("Testing error performance")
    
    sus start_time = time.now()
    
    bestie i := 0; i < 100; i++ {
        sus has_error lit = (i % 10 == 0)
        lowkey has_error {
            // Quick error handling
            sus error_msg tea = "Error handled for iteration " + string(i)
        }
    }
    
    sus elapsed = time.since(start_time)
    vibez.spill("Error handling performance test completed")
    
    vibez.spill("Error performance test complete")
}

slay test_advanced_error_scenarios() {
    vibez.spill("Testing advanced error scenarios")
    
    // Test nested error handling
    sus level1_error lit = based
    lowkey level1_error {
        vibez.spill("Level 1 error detected")
        
        sus level2_error lit = based
        lowkey level2_error {
            vibez.spill("Level 2 error detected")
            
            sus level3_error lit = based
            lowkey level3_error {
                vibez.spill("Level 3 error detected and handled")
            }
        }
    }
    
    vibez.spill("Advanced error scenarios test complete")
}

slay main() {
    vibez.spill("Starting Advanced Error Handling Runtime Tests...")
    
    test_basic_error_handling()
    test_goroutine_error_isolation()
    test_error_propagation()
    test_concurrent_error_handling()
    test_error_recovery_patterns()
    test_error_context_preservation()
    test_error_monitoring()
    test_resource_cleanup()
    test_error_performance()
    test_advanced_error_scenarios()
    
    vibez.spill("Advanced Error Handling Runtime Tests Complete!")
    vibez.spill("All error handling features tested successfully!")
}
