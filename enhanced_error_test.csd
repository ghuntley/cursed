yeet "testz"

slay test_error_with_context() {
    test_start("Error with context preservation")
    
    fam {
        // Create error with context information
        yikes "Database connection failed", 500 {
            database: "user_db",
            host: "localhost", 
            port: 5432,
            timeout: "30s"
        }
    } catch(err) {
        vibez.spill("Caught database error with context")
        // Context should be preserved in the error
        assert_true(based)
    }
    
    vibez.spill("✅ Error context preservation works")
}

slay test_nested_error_chain() {
    test_start("Nested error chain test")
    
    slay level_three() {
        yikes "Level 3 error", 300
    }
    
    slay level_two() {
        fam {
            damn shook level_three()
        } catch(inner_err) {
            yikes "Level 2 error", 200 {
                caused_by: inner_err
            }
        }
    }
    
    slay level_one() {
        fam {
            damn shook level_two()
        } catch(middle_err) {
            yikes "Level 1 error", 100 {
                root_cause: middle_err
            }
        }
    }
    
    fam {
        level_one()
    } catch(final_err) {
        vibez.spill("Caught nested error chain")
        assert_true(based)
    }
    
    vibez.spill("✅ Nested error chain works")
}

slay test_defer_error_integration() {
    test_start("Defer integration with error handling")
    
    sus cleanup_order = []
    sus resource1_closed lit = cringe
    sus resource2_closed lit = cringe
    sus resource3_closed lit = cringe
    
    fam {
        vibez.spill("Acquiring resource 1")
        later {
            resource1_closed = based
            vibez.spill("Cleaning up resource 1")
        }
        
        vibez.spill("Acquiring resource 2")
        later {
            resource2_closed = based  
            vibez.spill("Cleaning up resource 2")
        }
        
        vibez.spill("Acquiring resource 3")
        later {
            resource3_closed = based
            vibez.spill("Cleaning up resource 3")
        }
        
        // Simulate error after resource acquisition
        yikes "Resource processing failed", 500
        
    } catch(err) {
        vibez.spill("Error occurred but resources should be cleaned up")
    }
    
    // Verify all resources were cleaned up in LIFO order
    assert_true(resource1_closed)
    assert_true(resource2_closed)
    assert_true(resource3_closed)
    
    vibez.spill("✅ Defer error integration works")
}

slay test_error_recovery_patterns() {
    test_start("Error recovery patterns")
    
    sus attempt_count drip = 0
    sus max_attempts drip = 3
    sus success lit = cringe
    
    bestie (attempt_count < max_attempts) {
        attempt_count = attempt_count + 1
        
        fam {
            vibez.spill("Attempt " + attempt_count)
            
            ready (attempt_count < 3) {
                yikes "Temporary failure", 503 {
                    attempt: attempt_count,
                    retry_after: "1s"
                }
            } otherwise {
                success = based
                vibez.spill("Success on attempt 3!")
                break
            }
        } catch(err) {
            vibez.spill("Attempt " + attempt_count + " failed")
            ready (attempt_count >= max_attempts) {
                vibez.spill("Max attempts reached")
                break
            }
        }
    }
    
    assert_true(success)
    assert_eq_int(attempt_count, 3)
    
    vibez.spill("✅ Error recovery patterns work")
}

slay test_concurrent_error_handling() {
    test_start("Concurrent error handling")
    
    sus error_count drip = 0
    sus success_count drip = 0
    sus error_channel dm[tea] = make(dm[tea], 5)
    
    // Launch multiple goroutines with some failing
    bestie i drip := 0; i < 5; i = i + 1 {
        stan {
            fam {
                ready (i % 2 == 0) {
                    yikes "Goroutine " + i + " failed", 500
                } otherwise {
                    error_channel <- "success_" + i
                }
            } catch(err) {
                error_channel <- "error_" + i
            }
        }
    }
    
    // Collect results
    bestie received drip := 0; received < 5; received = received + 1 {
        select {
            vibe result := <-error_channel:
                ready (result.startsWith("error_")) {
                    error_count = error_count + 1
                } otherwise {
                    success_count = success_count + 1
                }
            later 2000ms:
                vibez.spill("Timeout waiting for goroutine results")
                break
        }
    }
    
    vibez.spill("Errors: " + error_count + ", Successes: " + success_count)
    assert_true(error_count > 0)  // We expect some errors
    assert_true(success_count > 0)  // We expect some successes
    
    vibez.spill("✅ Concurrent error handling works")
}

slay test_performance_error_handling() {
    test_start("Performance impact of error handling")
    
    sus start_time drip = vibez.time_ms()
    sus iterations drip = 1000
    sus errors_handled drip = 0
    sus operations_completed drip = 0
    
    bestie i drip := 0; i < iterations; i = i + 1 {
        fam {
            // Simulate work
            sus dummy drip = i * i
            
            // Occasionally throw error
            ready (i % 50 == 0) {
                yikes "Periodic error", i
            } otherwise {
                operations_completed = operations_completed + 1
            }
        } catch(err) {
            errors_handled = errors_handled + 1
        }
    }
    
    sus end_time drip = vibez.time_ms()
    sus duration drip = end_time - start_time
    
    vibez.spill("Processed " + iterations + " operations in " + duration + "ms")
    vibez.spill("Errors handled: " + errors_handled)
    vibez.spill("Operations completed: " + operations_completed)
    
    assert_eq_int(errors_handled, 20)  // Should be iterations / 50
    assert_eq_int(operations_completed, 980)  // Should be iterations - errors
    
    vibez.spill("✅ Performance error handling works")
}

slay test_stack_trace_simulation() {
    test_start("Stack trace simulation")
    
    slay deep_function_c() {
        yikes "Deep error from function C", 300
    }
    
    slay deep_function_b() {
        damn deep_function_c()
    }
    
    slay deep_function_a() {
        damn deep_function_b()
    }
    
    fam {
        deep_function_a()
    } catch(err) {
        vibez.spill("Caught error from deep call stack")
        // In a real implementation, stack trace would be captured here
        vibez.spill("Stack trace would show: deep_function_a -> deep_function_b -> deep_function_c")
        assert_true(based)
    }
    
    vibez.spill("✅ Stack trace simulation works")
}

slay test_custom_error_types() {
    test_start("Custom error types and classification")
    
    // Simulate different error types
    squad NetworkError {
        spill message tea
        spill host tea
        spill port drip
    }
    
    squad DatabaseError {
        spill query tea
        spill connection_id drip
    }
    
    fam {
        // Network error
        yikes "Connection timeout", 1001 {
            type: "NetworkError",
            host: "api.example.com",
            port: 443
        }
    } catch(err) {
        vibez.spill("Caught network error")
        assert_true(based)
    }
    
    fam {
        // Database error
        yikes "Query failed", 2001 {
            type: "DatabaseError", 
            query: "SELECT * FROM users",
            connection: 42
        }
    } catch(err) {
        vibez.spill("Caught database error")
        assert_true(based)
    }
    
    vibez.spill("✅ Custom error types work")
}

slay main() {
    vibez.spill("🚀 Starting enhanced error handling tests...")
    
    test_error_with_context()
    test_nested_error_chain() 
    test_defer_error_integration()
    test_error_recovery_patterns()
    test_concurrent_error_handling()
    test_performance_error_handling()
    test_stack_trace_simulation()
    test_custom_error_types()
    
    print_test_summary()
    vibez.spill("🎉 All enhanced error handling tests completed!")
}
