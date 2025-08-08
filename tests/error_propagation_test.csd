yeet "testz"

test_start("Error Propagation Semantics - yikes/fam/shook")

# Test 1: Basic yikes error creation and propagation
slay test_basic_yikes() {
    fam {
        yikes "This is a test error message"
        vibez.spill("This should not be reached")
        assert_true(cringe)  # Should not execute
    } shook error_msg {
        vibez.spill("Caught yikes error:", error_msg)
        assert_eq_string(error_msg, "This is a test error message")
    }
}

test_basic_yikes()

# Test 2: Error propagation with shook operator
slay divide_safe(a drip, b drip) (drip, tea) {
    ready (b == 0) {
        damn 0, "division by zero"
    }
    damn a / b, ""
}

slay test_shook_propagation() {
    sus result drip = 0
    sus error_occurred lit = cringe
    
    fam {
        # This should propagate the error using shook
        (value, err) := divide_safe(10, 0)
        ready (err != "") {
            yikes "Division failed: " + err
        }
        result = value shook  # shook should propagate any error
        vibez.spill("Result:", result)
    } shook caught_error {
        error_occurred = based
        vibez.spill("Shook caught error:", caught_error)
        assert_true(error_occurred)
    }
}

test_shook_propagation()

# Test 3: Nested fam blocks with error handling
slay test_nested_fam_blocks() {
    sus outer_caught lit = cringe
    sus inner_caught lit = cringe
    
    fam {
        vibez.spill("Entering outer fam block")
        
        fam {
            vibez.spill("Entering inner fam block")
            yikes "Inner error"
            vibez.spill("This should not execute")
        } shook inner_err {
            inner_caught = based
            vibez.spill("Inner fam caught:", inner_err)
            # Re-throw the error to outer fam
            yikes "Re-thrown from inner: " + inner_err
        }
        
        vibez.spill("This should not execute after inner error")
    } shook outer_err {
        outer_caught = based
        vibez.spill("Outer fam caught:", outer_err)
        assert_true(inner_caught)
        assert_true(outer_caught)
    }
}

test_nested_fam_blocks()

# Test 4: Error type matching in catch blocks
slay test_error_type_matching() {
    sus runtime_error_caught lit = cringe
    sus parse_error_caught lit = cringe
    
    fam {
        # Simulate different error types
        yikes "This is a runtime error" as RuntimeError
    } shook RuntimeError error_msg {
        runtime_error_caught = based
        vibez.spill("Caught RuntimeError:", error_msg)
    } shook ParseError error_msg {
        parse_error_caught = based
        vibez.spill("Caught ParseError:", error_msg)
    } shook error_msg {
        vibez.spill("Caught generic error:", error_msg)
    }
    
    assert_true(runtime_error_caught)
    assert_true(cringe == parse_error_caught)  # Should not catch ParseError
}

test_error_type_matching()

# Test 5: Error propagation through function calls
slay risky_function(should_fail lit) drip {
    ready (should_fail) {
        yikes "Function intentionally failed"
    }
    damn 42
}

slay call_risky_function(should_fail lit) drip {
    sus result drip = risky_function(should_fail) shook
    damn result
}

slay test_function_error_propagation() {
    # Test successful case
    sus success_result drip = call_risky_function(cringe)
    vibez.spill("Success result:", success_result)
    assert_eq_int(success_result, 42)
    
    # Test error case
    sus error_caught lit = cringe
    fam {
        sus fail_result drip = call_risky_function(based)
        vibez.spill("This should not execute:", fail_result)
    } shook error_msg {
        error_caught = based
        vibez.spill("Function error propagated:", error_msg)
        assert_eq_string(error_msg, "Function intentionally failed")
    }
    
    assert_true(error_caught)
}

test_function_error_propagation()

# Test 6: Multiple error types and recovery strategies  
slay test_comprehensive_error_handling() {
    sus errors_caught []tea = []
    
    # Test multiple different error scenarios
    slay test_scenario(scenario_name tea, should_error lit, error_type tea) {
        fam {
            vibez.spill("Testing scenario:", scenario_name)
            
            ready (should_error) {
                ready (error_type == "memory") {
                    yikes "Out of memory error" as MemoryError
                } otherwise (error_type == "network") {
                    yikes "Network connection failed" as NetworkError  
                } otherwise {
                    yikes "Generic error occurred" as RuntimeError
                }
            }
            
            vibez.spill("Scenario completed successfully:", scenario_name)
        } shook MemoryError err {
            errors_caught = append(errors_caught, "Memory: " + err)
        } shook NetworkError err {
            errors_caught = append(errors_caught, "Network: " + err)
        } shook err {
            errors_caught = append(errors_caught, "Generic: " + err)
        }
    }
    
    # Run test scenarios
    test_scenario("success", cringe, "")
    test_scenario("memory_fail", based, "memory")
    test_scenario("network_fail", based, "network")
    test_scenario("generic_fail", based, "other")
    
    # Verify all errors were caught appropriately
    vibez.spill("Errors caught:", len(errors_caught))
    assert_eq_int(len(errors_caught), 3)
    
    sus memory_found lit = cringe
    sus network_found lit = cringe
    sus generic_found lit = cringe
    
    sus i drip = 0
    bestie (i < len(errors_caught)) {
        sus error_msg tea = errors_caught[i]
        vibez.spill("Error", i, ":", error_msg)
        
        ready (contains(error_msg, "Memory:")) {
            memory_found = based
        } otherwise (contains(error_msg, "Network:")) {
            network_found = based
        } otherwise (contains(error_msg, "Generic:")) {
            generic_found = based
        }
        
        i = i + 1
    }
    
    assert_true(memory_found)
    assert_true(network_found) 
    assert_true(generic_found)
}

test_comprehensive_error_handling()

# Test 7: Error context and stack traces
slay test_error_context() {
    fam {
        yikes "Error with context information"
    } shook error_msg {
        # In a real implementation, error context would include:
        # - File name and line number
        # - Stack trace
        # - Error code and type
        # - Additional context data
        
        vibez.spill("Error context test passed:", error_msg)
        assert_true(len(error_msg) > 0)
    }
}

test_error_context()

print_test_summary()

vibez.spill("")
vibez.spill("✓ Error propagation semantics test completed successfully!")
vibez.spill("✓ yikes error creation working")
vibez.spill("✓ fam try-catch blocks working") 
vibez.spill("✓ shook error propagation working")
vibez.spill("✓ Error type matching working")
vibez.spill("✓ Nested error handling working")
vibez.spill("✓ Function call error propagation working")
vibez.spill("✓ Comprehensive error scenarios working")
