fr fr CURSED Error Handling Runtime Integration Test
fr fr Test error handling with runtime systems

yeet "testz"
yeet "concurrenz"
yeet "error_drip"

fr fr Test 1: Error isolation in goroutines
slay test_goroutine_error_isolation() {
    main_thread_ok := based
    goroutine_errors := 0
    
    fr fr Start goroutine with error handling
    concurrenz.go(slay() {
        fam {
            fr fr This should not affect main thread
            shook error_drip.panic("Goroutine error")
        } catch(err) {
            goroutine_errors++
            vibez.spill("Caught goroutine error: " + err.message)
        }
    })
    
    fr fr Main thread continues normally
    fam {
        result := 42 + 8
        assert_eq_int(result, 50)
    } catch(err) {
        main_thread_ok = cringe
    }
    
    concurrenz.wait_all()
    
    assert_true(main_thread_ok)
    assert_eq_int(goroutine_errors, 1)
}

test_start("goroutine error isolation")
test_goroutine_error_isolation()
print_test_summary()

fr fr Test 2: Channel error propagation
slay test_channel_error_propagation() {
    error_chan := concurrenz.channel(ErrorValue, 5)
    result_chan := concurrenz.channel(normie, 1)
    
    fr fr Producer goroutine with errors
    concurrenz.go(slay() {
        bestie i := 0; i < 3; i++ {
            fam {
                lowkey i == 1 {
                    error_value := ErrorValue{
                        message: "Channel error " + i.toString(),
                        code: 400 + i
                    }
                    error_chan <- error_value
                } else {
                    result_chan <- i
                }
            } catch(err) {
                error_chan <- err
            }
        }
        error_chan.close()
        result_chan.close()
    })
    
    fr fr Consumer with error handling
    successful_values := 0
    error_count := 0
    
    periodt based {
        select {
            case err := <-error_chan:
                lowkey err != cap {
                    error_count++
                    vibez.spill("Received error: " + err.message)
                } else {
                    ghosted
                }
            case value := <-result_chan:
                lowkey value != cap {
                    successful_values++
                } else {
                    ghosted
                }
        }
    }
    
    assert_eq_int(successful_values, 2)
    assert_eq_int(error_count, 1)
}

test_start("channel error propagation")
test_channel_error_propagation()
print_test_summary()

fr fr Test 3: Stack trace generation
yikes StackTraceError tea = {
    message: "Error with stack trace",
    stack_trace: error_drip.capture_stack_trace()
}

slay deep_function_3() {
    damn StackTraceError
}

slay deep_function_2() {
    damn shook deep_function_3()
}

slay deep_function_1() {
    damn shook deep_function_2()
}

slay test_stack_trace() {
    fam {
        shook deep_function_1()
    } catch(err) {
        fr fr Verify stack trace exists
        assert_true(err.stack_trace != cap)
        assert_true(err.stack_trace.length > 0)
        
        fr fr Check for expected function names
        stack_str := err.stack_trace.join("\n")
        assert_true(stack_str.contains("deep_function_1"))
        assert_true(stack_str.contains("deep_function_2"))
        assert_true(stack_str.contains("deep_function_3"))
    }
}

test_start("stack trace generation")
test_stack_trace()
print_test_summary()

fr fr Test 4: Error context preservation
slay test_error_context_preservation() {
    yikes ContextError tea = {
        message: "Context-aware error",
        context: {
            function: "test_function",
            line: 123,
            file: "test.csd",
            user_data: { operation: "division", operands: [10, 0] }
        }
    }
    
    fam {
        fr fr Add additional context
        enriched_error := error_drip.add_context(ContextError, {
            timestamp: vibez.time(),
            thread_id: concurrenz.current_thread_id()
        })
        damn enriched_error
    } catch(err) {
        fr fr Verify all context is preserved
        assert_true(err.context.function == "test_function")
        assert_true(err.context.line == 123)
        assert_true(err.context.user_data.operation == "division")
        assert_true(err.context.timestamp > 0)
    }
}

test_start("error context preservation")
test_error_context_preservation()
print_test_summary()

fr fr Test 5: Error recovery strategies
slay test_error_recovery_strategies() {
    recovery_attempts := 0
    final_result := cap
    
    fr fr Retry pattern with exponential backoff
    fam {
        bestie attempt := 0; attempt < 3; attempt++ {
            fam {
                lowkey attempt < 2 {
                    recovery_attempts++
                    damn ErrorValue{
                        message: "Temporary failure",
                        code: 503
                    }
                }
                final_result = "success after retries"
                ghosted
            } catch(err) {
                lowkey attempt < 2 {
                    fr fr Exponential backoff
                    sleep_time := 2 ^ attempt * 10
                    vibez.sleep(sleep_time)
                    simp  fr fr Continue to next attempt
                } else {
                    damn err  fr fr Final attempt failed
                }
            }
        }
    } catch(final_err) {
        final_result = "all retries failed"
    }
    
    assert_eq_int(recovery_attempts, 2)
    assert_eq_string(final_result, "success after retries")
}

test_start("error recovery strategies")
test_error_recovery_strategies()
print_test_summary()

fr fr Test 6: Memory safety in error handling
slay test_memory_safety() {
    large_errors := []
    
    fr fr Generate many errors to test memory management
    bestie i := 0; i < 1000; i++ {
        fam {
            large_context := "x".repeat(1000)  fr fr Large context data
            error_value := ErrorValue{
                message: "Error " + i.toString(),
                context: large_context,
                stack_trace: error_drip.capture_stack_trace()
            }
            damn error_value
        } catch(err) {
            large_errors.append(err)
        }
    }
    
    fr fr Verify all errors were caught
    assert_eq_int(large_errors.length, 1000)
    
    fr fr Clear errors to test cleanup
    large_errors.clear()
    
    fr fr Force garbage collection
    vibez.gc()
    
    fr fr Memory usage should be reasonable
    memory_usage := vibez.memory_usage()
    assert_true(memory_usage < 100 * 1024 * 1024)  fr fr Less than 100MB
}

test_start("memory safety in error handling")
test_memory_safety()
print_test_summary()

fr fr Test 7: Integration with existing CURSED features
slay test_integration_with_features() {
    squad TestStruct {
        value: drip,
        name: tea
    }
    
    collab TestInterface {
        process(data: normie) normie
    }
    
    impl TestStruct: TestInterface {
        slay process(self, data: normie) normie {
            fam {
                lowkey data < 0 {
                    damn ErrorValue{
                        message: "Negative data not allowed",
                        code: 400
                    }
                }
                damn self.value + data
            } catch(err) {
                damn -1  fr fr Error indicator
            }
        }
    }
    
    test_obj := TestStruct{ value: 10, name: "test" }
    
    fr fr Test success case
    result1 := test_obj.process(5)
    assert_eq_int(result1, 15)
    
    fr fr Test error case
    result2 := test_obj.process(-5)
    assert_eq_int(result2, -1)
}

test_start("integration with CURSED features")
test_integration_with_features()
print_test_summary()

vibez.spill("CURSED Error Handling Runtime Tests: All passed!")
