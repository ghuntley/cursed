yeet "testz"

slay test_basic_yikes_creation() {
    test_start("Basic yikes error creation")
    
    // Test simple error creation
    sus err yikes = "Basic error message", 100
    assert_true(err.isError())
    assert_eq_int(err.getCode(), 100)
    
    vibez.spill("✅ Basic yikes creation works")
}

slay test_yikes_with_context() {
    test_start("Yikes error with context data")
    
    // Test error with additional context
    sus err yikes = "File operation failed", 500 {
        file: "/tmp/test.txt",
        operation: "read",
        user_id: "12345"
    }
    
    assert_true(err.isError())
    assert_eq_int(err.getCode(), 500)
    
    vibez.spill("✅ Yikes with context works")
}

slay test_shook_propagation() {
    test_start("Shook error propagation")
    
    // Function that might fail
    slay risky_operation() shook {
        sus rand_val drip = 42
        ready (rand_val > 50) {
            damn "Success"
        } otherwise {
            yikes "Operation failed", 404
        }
    }
    
    // Propagate error using shook
    sus result = shook risky_operation()
    
    // This should either return a value or propagate error
    vibez.spill("✅ Shook propagation works")
}

slay test_fam_error_recovery() {
    test_start("Fam error recovery blocks")
    
    sus recovery_worked lit = cringe
    
    fam {
        // Try block - code that might fail
        sus dangerous_value drip = 10 / 0  // Division by zero
        vibez.spill("This shouldn't execute")
    } catch(err) {
        // Catch block - handle specific errors
        vibez.spill("Caught error: " + err.getMessage())
        recovery_worked = based
    } finally {
        // Finally block - always executes
        vibez.spill("Cleanup completed")
    }
    
    assert_true(recovery_worked)
    vibez.spill("✅ Fam error recovery works")
}

slay test_stack_trace_capture() {
    test_start("Stack trace capture and formatting")
    
    slay level_three() {
        yikes "Deep error", 300 // This should capture full stack trace
    }
    
    slay level_two() {
        damn level_three()
    }
    
    slay level_one() {
        damn level_two()
    }
    
    fam {
        level_one()
    } catch(err) {
        vibez.spill("Error caught with stack trace:")
        vibez.spill(err.getStackTrace())
        assert_true(err.hasStackTrace())
    }
    
    vibez.spill("✅ Stack trace capture works")
}

slay test_defer_integration() {
    test_start("Defer integration with error handling")
    
    sus cleanup_called lit = cringe
    sus resource_acquired lit = based
    
    {
        // Acquire resource
        vibez.spill("Acquiring resource...")
        
        // Register cleanup with defer
        later {
            vibez.spill("Cleaning up resource...")
            cleanup_called = based
            resource_acquired = cringe
        }
        
        // Simulate error that triggers cleanup
        yikes "Resource error", 500
    }
    
    // Verify cleanup was called
    assert_true(cleanup_called)
    assert_false(resource_acquired)
    
    vibez.spill("✅ Defer integration works")
}

slay test_nested_error_context() {
    test_start("Nested error context preservation")
    
    slay inner_function() {
        yikes "Inner error", 100 {
            context: "inner_function",
            level: "3"
        }
    }
    
    slay middle_function() {
        fam {
            damn shook inner_function()
        } catch(inner_err) {
            // Re-wrap with additional context
            yikes "Middle error", 200 {
                context: "middle_function", 
                level: "2",
                caused_by: inner_err
            }
        }
    }
    
    slay outer_function() {
        fam {
            damn shook middle_function()
        } catch(middle_err) {
            // Final context layer
            yikes "Outer error", 300 {
                context: "outer_function",
                level: "1", 
                root_cause: middle_err
            }
        }
    }
    
    fam {
        outer_function()
    } catch(final_err) {
        vibez.spill("Final error with full context chain:")
        vibez.spill(final_err.getFullContext())
        assert_true(final_err.hasInnerError())
        assert_eq_int(final_err.getInnerError().getCode(), 200)
    }
    
    vibez.spill("✅ Nested error context works")
}

slay test_concurrent_error_handling() {
    test_start("Concurrent error handling with goroutines")
    
    sus error_channel dm[yikes] = make(dm[yikes], 2)
    
    // Goroutine 1: might succeed
    stan {
        fam {
            vibez.spill("Goroutine 1: working...")
            // Simulate success
            vibez.spill("Goroutine 1: completed successfully")
        } catch(err) {
            error_channel <- err
        }
    }
    
    // Goroutine 2: will fail
    stan {
        fam {
            vibez.spill("Goroutine 2: working...")
            yikes "Goroutine 2 failed", 500
        } catch(err) {
            error_channel <- err
        }
    }
    
    // Collect errors
    sus errors_received drip = 0
    bestie (errors_received < 1) {  // Expecting 1 error
        select {
            vibe err := <-error_channel:
                vibez.spill("Received error from goroutine: " + err.getMessage())
                errors_received = errors_received + 1
            later 5000ms:
                vibez.spill("Timeout waiting for errors")
                break
        }
    }
    
    assert_eq_int(errors_received, 1)
    vibez.spill("✅ Concurrent error handling works")
}

slay test_resource_cleanup_patterns() {
    test_start("Resource cleanup patterns with errors")
    
    sus file_closed lit = cringe
    sus connection_closed lit = cringe
    sus memory_freed lit = cringe
    
    fam {
        // Simulate resource acquisition
        vibez.spill("Opening file...")
        later { file_closed = based }
        
        vibez.spill("Opening connection...")
        later { connection_closed = based }
        
        vibez.spill("Allocating memory...")
        later { memory_freed = based }
        
        // Simulate error after resource acquisition
        yikes "Processing error", 500
        
    } catch(err) {
        vibez.spill("Error occurred, but resources should still be cleaned up")
        vibez.spill("Error: " + err.getMessage())
    }
    
    // Verify all resources were cleaned up despite error
    assert_true(file_closed)
    assert_true(connection_closed)
    assert_true(memory_freed)
    
    vibez.spill("✅ Resource cleanup patterns work")
}

slay test_error_recovery_strategies() {
    test_start("Error recovery strategies")
    
    sus attempt_count drip = 0
    sus max_attempts drip = 3
    sus final_result tea = ""
    
    bestie (attempt_count < max_attempts) {
        attempt_count = attempt_count + 1
        
        fam {
            ready (attempt_count < 3) {
                yikes "Temporary failure", 503 {
                    attempt: attempt_count
                }
            } otherwise {
                final_result = "Success on attempt " + attempt_count
                damn final_result
            }
        } catch(err) {
            vibez.spill("Attempt " + attempt_count + " failed: " + err.getMessage())
            ready (attempt_count >= max_attempts) {
                vibez.spill("Max attempts reached, giving up")
                break
            }
        }
    }
    
    assert_eq_string(final_result, "Success on attempt 3")
    vibez.spill("✅ Error recovery strategies work")
}

slay test_performance_error_handling() {
    test_start("Performance impact of error handling")
    
    sus start_time drip = vibez.time_ms()
    sus iterations drip = 10000
    sus errors_handled drip = 0
    
    bestie i drip := 0; i < iterations; i = i + 1 {
        fam {
            ready (i % 100 == 0) {
                yikes "Periodic error", i
            } otherwise {
                // Normal operation
                sus dummy drip = i * 2
            }
        } catch(err) {
            errors_handled = errors_handled + 1
        }
    }
    
    sus end_time drip = vibez.time_ms()
    sus duration drip = end_time - start_time
    
    vibez.spill("Processed " + iterations + " operations in " + duration + "ms")
    vibez.spill("Handled " + errors_handled + " errors")
    assert_eq_int(errors_handled, 100)  // Should be iterations / 100
    
    vibez.spill("✅ Performance error handling works")
}

// Main test runner
slay main() {
    vibez.spill("🚀 Starting comprehensive error handling tests...")
    
    test_basic_yikes_creation()
    test_yikes_with_context()
    test_shook_propagation()
    test_fam_error_recovery()
    test_stack_trace_capture()
    test_defer_integration()
    test_nested_error_context()
    test_concurrent_error_handling()
    test_resource_cleanup_patterns()
    test_error_recovery_strategies()
    test_performance_error_handling()
    
    print_test_summary()
    vibez.spill("🎉 All error handling tests completed!")
}
